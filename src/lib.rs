#![allow(non_snake_case)]

mod style;

use dioxus::prelude::*;
use dioxus_heroicons::{Icon, solid::Shape};

#[derive(Debug, Props)]
pub struct ToastProps<'a> {
    text: &'a str,
    state: UseState<bool>,

    #[props(optional)]
    heading: Option<&'a str>,

    #[props(default = true)]
    close_button: bool,
}

pub fn Toast<'a>(cx: Scope<'a, ToastProps<'a>>) -> Element<'a> {

    if !**cx.props.state.get() {
        return None;
    }

    let heading = cx.props.heading.unwrap_or_default();

    cx.render(rsx! {
        div {
            class: "toast-controller",
            div {
                style: format_args!("{} {}", style::TOAST, style::TOAST_BOTTOM_LEFT),
                div {
                    style: format_args!("{}", style::TOAST_SINGLE),
                    if !heading.is_empty() {
                        cx.render(
                            rsx! {
                                h2 {
                                    style: format_args!("{}", style::TOAST_SINGLE__H2),
                                    "{heading}"
                                }
                            }
                        )
                    } else { None }
                    if cx.props.close_button {
                        cx.render(
                            rsx! {
                                a {
                                    style: format_args!("{}", style::TOAST_SINGLE_CLOSE),
                                    onclick: move |_| {
                                        cx.props.state.setter()(false);
                                    },
                                    Icon {
                                        icon: Shape::X,
                                        size: 15,
                                    }
                                }
                            }
                        )
                    } else {
                        None
                    }
                    "{cx.props.text}"
                }
            }
        }
    })
}