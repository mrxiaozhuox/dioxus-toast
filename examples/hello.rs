use dioxus::prelude::*;

fn main() {
    dioxus::desktop::launch(app)
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        dioxus_toast::Toast {
            text: "hello world",
        }
    })
}