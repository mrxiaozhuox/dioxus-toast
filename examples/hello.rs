use dioxus::prelude::*;
use dioxus_toast::ToastManager;

fn main() {
    dioxus::desktop::launch(app)
}

fn app(cx: Scope) -> Element {

    std::panic::set_hook(Box::new(|info| {
        println!("Panic: {}", info);
    }));

    let toast = use_ref(&cx, ToastManager::default);

    cx.render(rsx! {
        dioxus_toast::ToastFrame {
            manager: toast
        }
        div {
            button {
                onclick: move |_| {
                    toast.write().popup(cx.render(rsx! {
                        div { "123" }
                    }));
                },
                "弹出"
            }
        }
    })
}