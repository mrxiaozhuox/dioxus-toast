#![allow(non_snake_case)]

// mod style;

use std::collections::HashMap;

use dioxus::prelude::*;
use uuid::Uuid;

#[derive(Default, Debug)]
pub struct ToastManager {
    list: HashMap<Uuid, ToastInfo>,
}

impl ToastManager {
    pub fn popup(&mut self, option: ToastInfo) -> Uuid {
        let uuid = Uuid::new_v4();
        self.list.insert(uuid, option);
        uuid
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Position {
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Icon {
    Success,
    Warning,
    Error,
    Info,
}

#[derive(Debug)]
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
                dangerous_inner_html: "{info.context}"
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