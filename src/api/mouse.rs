use serde::Serialize;
use serde_wasm_bindgen;
use wasm_bindgen::JsValue;

use crate::tauri::invoke;

#[derive(Serialize)]
struct ClickFixedArgs {
    x: i32,
    y: i32,
}

pub async fn click_fixed(x: i32, y: i32) -> Result<(), JsValue> {
    let args = ClickFixedArgs { x, y };

    let js_args = serde_wasm_bindgen::to_value(&args)?;
    let _ = invoke("click_fixed", js_args).await;

    Ok(())
}

pub async fn start_mouse_hook() -> Result<(), JsValue> {
    let _ = invoke("start_mouse_hook_command", JsValue::NULL).await;
    Ok(())
}

pub async fn stop_mouse_hook() -> Result<(), JsValue> {
    let _ = invoke("stop_mouse_hook_command", JsValue::NULL).await;
    Ok(())
}

// pub async fn get_cursor_pos() -> Result<(i32, i32), JsValue> {
//     invoke("handle_get_pos", JsValue::NULL).await
// }
