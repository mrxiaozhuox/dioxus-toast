use dioxus::prelude::*;
use dioxus_web::launch;
use dioxus_toast::{ToastFrame, ToastManager, ToastInfo};

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {

    let toast = use_ref(&cx, ToastManager::default);

    cx.render(rsx! {
        ToastFrame {
            manager: toast
        },
        button {
            onclick: move |_| {
                toast.write().popup(ToastInfo::simple("123"));
            },
            "T"
        }
    })
}
