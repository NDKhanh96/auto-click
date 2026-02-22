use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{spawn_local, JsFuture};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["__TAURI__", "core"], js_name = invoke)]
    fn tauri_invoke(cmd: &str, args: JsValue) -> js_sys::Promise;

    #[wasm_bindgen(js_namespace = ["__TAURI__", "event"], js_name = listen)]
    fn tauri_listen(event: &str, handler: &js_sys::Function) -> js_sys::Promise;
}

pub fn use_mouse_listener(position: RwSignal<(i32, i32)>) {
    Effect::new(move |_| {
        // Gọi backend để start mouse hook
        spawn_local(async move {
            let promise = tauri_invoke("start_mouse_hook_command", JsValue::NULL);
            let _ = JsFuture::from(promise).await;
        });

        // Đăng ký listener
        let handler = Closure::wrap(Box::new(move |event: JsValue| {
            // Lấy payload
            let payload = js_sys::Reflect::get(&event, &JsValue::from_str("payload"))
                .unwrap_or(JsValue::NULL);
            let arr = js_sys::Array::from(&payload);

            if arr.length() == 2 {
                let x = arr.get(0).as_f64().unwrap_or(0.0) as i32;
                let y = arr.get(1).as_f64().unwrap_or(0.0) as i32;
                position.set((x, y));
            }
        }) as Box<dyn FnMut(JsValue)>);

        let _ = tauri_listen("mouse-move", handler.as_ref().unchecked_ref());

        handler.forget();
    });
}
