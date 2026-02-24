use crate::hooks::mouse::use_mouse_listener;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;

/**
 * Để dùng dc log cần phần này
 *
 * Cần import use wasm_bindgen::prelude::*;
 *
 * Cách dùng:
 * - log(variant)
 */
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &JsValue);
}

#[component]
pub fn App() -> impl IntoView {
    let position = RwSignal::new((0, 0));

    use_mouse_listener(position);
    Effect::new(move |_| {
        // Truy cập trực tiếp signal trong closure
        let (x, y) = position.get();
        log(&format!("App position: x={}, y={}", x, y).into());
    });

    view! {
        <div class="p-4">
            <h1>"Mouse Position"</h1>
            <p>
                {move || {
                    let (x, y) = position.get();
                    format!("X: {}, Y: {}", x, y)
                }}
            </p>
        </div>
    }
}
