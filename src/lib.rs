#![allow(non_snake_case)]

// mod style;

use std::collections::HashMap;

use dioxus::prelude::*;

#[derive(Default, Debug)]
pub struct ToastManager {
    list: HashMap<u8, ToastInfo>,
    timer: HashMap<u8, (i64, usize)>,
    id_index: u8,
}

impl ToastManager {
    pub fn popup(&mut self, option: ToastInfo) -> u8 {
        
        self.id_index += 1;
        let toast_id = self.id_index;

        self.list.insert(toast_id, option.clone());

        let hide_after = option.hide_after.unwrap_or(0);
        let timestamp = chrono::Local::now().timestamp();
        self.timer.insert(toast_id, (timestamp, hide_after));

        toast_id
    }

    pub fn clear(&mut self) {
        self.list.clear();
        self.timer.clear();
        self.id_index = 0;
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Position {
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Icon {
    Success,
    Warning,
    Error,
    Info,
}

#[derive(Debug, Clone)]
pub struct ToastInfo {
    pub heading: Option<String>,
    pub context: String,
    pub allow_toast_close: bool,
    pub position: Position,
    pub icon: Option<Icon>,
    pub hide_after: Option<usize>,
}

#[inline_props]
pub fn ToastFrame<'a>(cx: Scope, manager: &'a UseRef<ToastManager>) -> Element {
    // println!("{:?}", manager.read());

    let toast_list = &manager.read().list;

    let mut bottom_left_ele: Vec<LazyNodes> = vec![];
    let mut bottom_right_ele: Vec<LazyNodes> = vec![];
    let mut top_left_ele: Vec<LazyNodes> = vec![];
    let mut top_right_ele: Vec<LazyNodes> = vec![];

    for (id, info) in toast_list {
        let element = rsx! {
            div {
                class: "toast-single",
                id: "{id}",
                dangerous_inner_html: "{info.context}",
                if info.allow_toast_close {
                    cx.render(rsx! {
                        div {
                            class: "close-toast-single",
                            onclick: |_| {
                                // let curr_id = id.clone();
                            },
                            "×",
                        }
                    })
                } else {
                    None
                }
            }
        };

        if info.position == Position::BottomLeft {
            bottom_left_ele.push(element);
        } else if info.position == Position::BottomRight {
            bottom_right_ele.push(element);
        } else if info.position == Position::TopLeft {
            top_left_ele.push(element);
        } else if info.position == Position::TopRight {
            top_right_ele.push(element);
        }
    }

    use_future(&cx, || {
        let toast_manager = manager.to_owned().clone();
        async move {
            loop {
                let timer_list = toast_manager.read().timer.clone();
                for (id, time) in &timer_list {
                    let current_time = chrono::Local::now().timestamp();
                    let expire_time = time.0 + time.1 as i64;
                    // println!("{:?} -> {:?}", current_time, expire_time);
                    if current_time >= expire_time && time.1 != 0_usize {
                        toast_manager.write().list.remove(id);
                        toast_manager.write().timer.remove(id);
                    }
                }
                if toast_manager.read().list.is_empty() {
                    toast_manager.write().id_index = 0;
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        }
    });

    cx.render(rsx! {
        div {
            class: "toast-scope",
            style { [ include_str!("./assets/toast.css") ] },
            div {
                class: "toast-wrap bottom-left",
                id: "wrap-bottom-left",
                bottom_left_ele
            }
            div {
                class: "toast-wrap bottom-right",
                id: "wrap-bottom-right",
                bottom_right_ele
            }
            div {
                class: "toast-wrap top-left",
                id: "wrap-top-left",
                top_left_ele
            }
            div {
                class: "toast-wrap top-right",
                id: "wrap-top-right",
                top_right_ele
            }
        }
    })
}
