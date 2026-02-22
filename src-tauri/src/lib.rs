mod modes;
mod mouse;
use std::sync::OnceLock;
use tauri::AppHandle;

static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();

/**
 * Khai báo hàm
 */
#[tauri::command]
fn click_fixed(x: i32, y: i32) -> Result<(), String> {
    modes::run(modes::ClickMode::FixedWithRestore { x, y }).map_err(|e| e.to_string())
}

#[tauri::command]
fn start_mouse_hook_command() {
    mouse::start_mouse_hook();
}

#[tauri::command]
fn stop_mouse_hook_command() {
    mouse::stop_mouse_hook();
}

/**
 * gán các hàm vào trong array invoke_handler để dùng ở FE
 * ví dụ: await invoke("click_fixed", { x: 1486, y: 1074 });
 */
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            APP_HANDLE.set(app.handle().clone()).ok();

            mouse::setup_mouse_worker();

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            click_fixed,
            start_mouse_hook_command,
            stop_mouse_hook_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
