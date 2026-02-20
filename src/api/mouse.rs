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
