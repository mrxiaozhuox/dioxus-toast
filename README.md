# dioxus-toast

> Add toast support in your dioxus project.

```rust
let toast = use_ref(&cx, ToastManager::default);

cx.render(rsx! {
    dioxus_toast::ToastFrame {
        manager: toast
    }
    div {
        button {
            onclick: move |_| {
                let _id = toast.write().popup(ToastInfo::simple("Hello Dioxus"));
            },
            "Normal Toast"
        }
        button {
            onclick: move |_| {
                let _id = toast.write().popup(ToastInfo {
                    heading:Some("Success!".into()),
                    context:"Dioxus Toast".into(),
                    allow_toast_close:true,
                    position:dioxus_toast::Position::BottomLeft, 
                    icon: Some(Icon::Success), 
                    hide_after: Some(5), 
                });
            },
            "Success Toast"
        }
    }
})
```