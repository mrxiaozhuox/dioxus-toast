use dioxus::prelude::*;
use dioxus_toast::{ToastFrame, ToastInfo, ToastManager};

fn main() {
    launch(app);
}

fn app() -> Element {
    let mut toast = use_signal(|| ToastManager::default());

    rsx! {
        ToastFrame { manager: toast }
        button {
            onclick: move |_| {
                toast.write().popup(ToastInfo::simple("123"));
            },
            "T"
        }
    }
}
