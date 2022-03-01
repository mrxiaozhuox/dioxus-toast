#![allow(non_snake_case)]

mod style;

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

#[derive(Debug)]
pub struct ToastInfo {
    text: String
}

#[inline_props]
pub fn ToastFrame<'a>(cx: Scope, manager: &'a UseRef<ToastManager>) -> Element {

    println!("{:?}", manager.read());

    cx.render(rsx! {
        div {
            class: "toast-scope",
            style { [ include_str!("./assets/toast.css") ] },
            div {
                class: "toast-wrap",
                div {
                    class: "toast-single",
                    "123"
                }
            }
        }
    })
}