#![allow(non_snake_case)]

mod style;

use dioxus::prelude::*;
use dioxus_heroicons::{Icon, solid::Shape};

#[derive(Debug, Props)]
pub struct ToastProps<'a> {
    text: &'a str
}

pub fn Toast<'a>(cx: Scope<'a, ToastProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "toast-controller",
            div {
                style: format_args!("{} {}", style::TOAST, style::TOAST_BOTTOM_LEFT),
                div {
                    style: format_args!("{}", style::TOAST_SINGLE),
                    span {
                        style: format_args!("{}", style::TOAST_SINGLE_CLOSE),
                        Icon {
                            icon: Shape::X,
                            size: 15,
                        }
                    }
                    "{cx.props.text}"
                }
            }
        }
    })
}