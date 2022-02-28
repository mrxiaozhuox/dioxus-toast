#![allow(non_snake_case)]

mod style;

use std::collections::HashMap;

use dioxus::prelude::*;
use uuid::Uuid;

#[derive(Default, Debug)]
pub struct ToastManager<'a> {
    list: HashMap<Uuid, Element<'a>>,
}

impl<'a> ToastManager<'a> {
    pub fn popup(&mut self, option: Element<'a>) -> Uuid {
        let uuid = Uuid::new_v4();
        self.list.insert(uuid, option);
        uuid
    }
}

#[derive(Props)]
pub struct ToastProps<'a> {
    text: &'a str
}

pub fn Toast<'a>(cx: Scope<'a, ToastProps<'a>>) -> Element {
    cx.render(rsx! {
        div {}
    })
}

#[inline_props]
pub fn ToastFrame<'a>(cx: Scope, manager: &'a UseRef<ToastManager<'a>>) -> Element {

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