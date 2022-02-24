use dioxus::prelude::*;

fn main() {
    dioxus::desktop::launch(app)
}

fn app(cx: Scope) -> Element {

    std::panic::set_hook(Box::new(|info| {
        println!("Panic: {}", info);
    }));

    let (_, toast) = use_state(&cx, || false);

    cx.render(rsx! {
        dioxus_toast::Toast {
            heading: "Information",
            text: "hello world",
            state: toast.clone(),
        }
        div {
            button {
                onclick: move |_| {
                    toast.setter()(true);
                },
                "弹出"
            }
        }
    })
}