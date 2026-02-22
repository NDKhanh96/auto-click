use crate::APP_HANDLE;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, OnceLock};
use std::thread;
use tauri::Emitter;
use windows::{
    core::Result,
    Win32::{
        Foundation::{LPARAM, LRESULT, POINT, WPARAM},
        UI::{
            Input::KeyboardAndMouse::{
                SendInput, INPUT, INPUT_0, INPUT_MOUSE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP,
                MOUSEINPUT,
            },
            WindowsAndMessaging::{
                CallNextHookEx, DispatchMessageW, GetMessageW, SetCursorPos, SetWindowsHookExW,
                TranslateMessage, UnhookWindowsHookEx, MSG, MSLLHOOKSTRUCT, WH_MOUSE_LL,
                WM_MOUSEMOVE,
            },
        },
    },
};

static HOOK_SENDER: OnceLock<mpsc::Sender<(i32, i32)>> = OnceLock::new();

pub fn get_cursor_pos() -> Result<POINT> {
    use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;
    unsafe {
        let mut point = POINT::default();
        GetCursorPos(&mut point)?;
        Ok(point)
    }
}

pub fn set_cursor_pos(x: i32, y: i32) -> Result<()> {
    unsafe {
        SetCursorPos(x, y)?;
        Ok(())
    }
}

pub fn left_click() {
    unsafe {
        let inputs = [
            INPUT {
                r#type: INPUT_MOUSE,
                Anonymous: INPUT_0 {
                    mi: MOUSEINPUT {
                        dx: 0,
                        dy: 0,
                        mouseData: 0,
                        dwFlags: MOUSEEVENTF_LEFTDOWN,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            },
            INPUT {
                r#type: INPUT_MOUSE,
                Anonymous: INPUT_0 {
                    mi: MOUSEINPUT {
                        dx: 0,
                        dy: 0,
                        mouseData: 0,
                        dwFlags: MOUSEEVENTF_LEFTUP,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            },
        ];

        let size = std::mem::size_of::<INPUT>() as i32;
        SendInput(&inputs, size);
    }
}

pub fn click_at(x: i32, y: i32) -> Result<()> {
    set_cursor_pos(x, y)?;
    left_click();
    Ok(())
}

static RUNNING: AtomicBool = AtomicBool::new(false);

unsafe extern "system" fn mouse_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code >= 0 && RUNNING.load(Ordering::Relaxed) {
        if wparam.0 as u32 == WM_MOUSEMOVE {
            let info = *(lparam.0 as *const MSLLHOOKSTRUCT);

            if let Some(sender) = HOOK_SENDER.get() {
                let _ = sender.send((info.pt.x, info.pt.y));
            }
        }
    }

    CallNextHookEx(None, code, wparam, lparam)
}

pub fn start_mouse_hook() {
    RUNNING.store(true, Ordering::Relaxed);

    thread::spawn(|| unsafe {
        let hook =
            SetWindowsHookExW(WH_MOUSE_LL, Some(mouse_proc), None, 0).expect("Failed to set hook");

        let mut msg = MSG::default();

        while RUNNING.load(Ordering::Relaxed) && GetMessageW(&mut msg, None, 0, 0).into() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        let _ = UnhookWindowsHookEx(hook);
    });
}

pub fn stop_mouse_hook() {
    RUNNING.store(false, Ordering::Relaxed);
}

pub fn setup_mouse_worker() {
    let (tx, rx) = mpsc::channel::<(i32, i32)>();
    HOOK_SENDER.set(tx).ok();

    std::thread::spawn(move || {
        use std::time::{Duration, Instant};

        let throttle = Duration::from_millis(16);
        let mut last_emit = Instant::now();
        let mut last_pos = (0, 0);

        while let Ok((x, y)) = rx.recv() {
            let now = Instant::now();

            if now.duration_since(last_emit) >= throttle && (x, y) != last_pos {
                if let Some(app) = APP_HANDLE.get() {
                    let _ = app.emit("mouse-move", (x, y));
                }

                last_emit = now;
                last_pos = (x, y);
            }
        }
    });
}
