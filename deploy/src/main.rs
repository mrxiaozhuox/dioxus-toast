use dioxus::prelude::*;
use dioxus_toast::*;

fn main() {
    // init debug tool for WebAssembly
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    dioxus::web::launch(app); 
}

fn app(cx: Scope) -> Element {

    let manager = use_ref(&cx, ToastManager::default);

    cx.render(rsx! ( 
        ToastFrame {
            manager: manager,
        }
    ))
}
