<div align="center">
  <h1>Dioxus Toast</h1>
  <p></p>
    <div>
    <img src="https://img.shields.io/badge/Dioxus%20Support-0.3.X-green?style=flat-square&logo=Rust"></img>
  	<img src="https://img.shields.io/github/actions/workflow/status/mrxiaozhuox/dioxus-toast/rust.yml?label=Example%20Build&style=flat-square&logo=Github"></img>
  </div>	
  <p></p>
  <strong>Add toast support for your dioxus project.</strong>
  <p></p>
</div>

```rust
use dioxus::prelude::*;
use fermi::{AtomRef, use_atom_ref, use_init_atom_root};
use dioxus_toast::{ToastInfo, ToastManager};

fn main() {
    dioxus_desktop::launch(app)
}

static TOAST_MANAGER: AtomRef<ToastManager> = |_| ToastManager::default();

fn app(cx: Scope) -> Element {

    use_init_atom_root(&cx);

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

```