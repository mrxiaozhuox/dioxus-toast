use dioxus::prelude::*;
use dioxus_toast::{ToastInfo, ToastManager};

fn main() {
    dioxus::desktop::launch(app)
}

static TOAST_MANAGER: AtomRef<ToastManager> = |_| ToastManager::default();

fn app(cx: Scope) -> Element {
    std::panic::set_hook(Box::new(|info| {
        println!("Panic: {}", info);
    }));

    let toast = use_atom_ref(&cx, TOAST_MANAGER);

    cx.render(rsx! {
        dioxus_toast::ToastFrame {
            manager: toast
        }
        div {
            button {
                onclick: move |_| {
                    // let _id = toast.write().popup(ToastInfo {
                    //     heading:Some("Hello Dioxus".into()),
                    //     context:"hello world: <a href=\"https://dioxuslabs.com/\">Dioxus</a>".into(),
                    //     allow_toast_close:true,
                    //     position:dioxus_toast::Position::BottomLeft, 
                    //     icon: None, 
                    //     hide_after: Some(5), 
                    // });
                    let _id = toast.write().popup(ToastInfo::simple("hello world"));
                    println!("New Toast ID: {}", _id);
                },
                "Normal Toast"
            }
            button {
                onclick: move |_| {
                    let _id = toast.write().popup(ToastInfo::success("Hello World!", "Success"));
                    println!("New Toast ID: {}", _id);  
                },
                "Success Toast"
            }
            button {
                onclick: move |_| {
                    let _id = toast.write().popup(ToastInfo {
                        heading: Some("top-right".into()),
                        context: "Top Right Toast".into(),
                        allow_toast_close: true,
                        position: dioxus_toast::Position::TopRight,
                        icon: None,
                        hide_after: None
                    });
                },
                "Top Right"
            }
        }
    })
}
