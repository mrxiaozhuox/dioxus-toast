use dioxus::prelude::*;
use dioxus_toast::{ToastManager, ToastInfo};

fn main() {
    dioxus::desktop::launch(app)
}

fn app(cx: Scope) -> Element {

    std::panic::set_hook(Box::new(|info| {
        println!("Panic: {}", info);
    }));

    let toast = use_ref(&cx, ToastManager::default);

    cx.render(rsx! {
        dioxus_toast::Toast { }
        div {
            button {
                onclick: move |_| {
                    toast.write().popup(ToastInfo {
                        
                    })
                },
                "弹出"
            }
        }
    })
}