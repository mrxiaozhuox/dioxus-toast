use dioxus::prelude::*;
use dioxus_toast::{ToastInfo, ToastManager};

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
                    toast.write().popup(ToastInfo {
                        heading:Some("Hello Dioxus".into()),
                        context:"hello world: <a href=\"#\">Dioxus</a>".into(),
                        allow_toast_close:true,
                        position:dioxus_toast::Position::BottomLeft, 
                        icon: None, 
                        hide_after: None, 
                    });
                },
                "弹出"
            }
        }
    })
}
