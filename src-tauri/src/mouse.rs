use windows::{
    core::Result,
    Win32::{
        Foundation::POINT,
        UI::{
            Input::KeyboardAndMouse::{
                SendInput, INPUT, INPUT_0, INPUT_MOUSE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP,
                MOUSEINPUT,
            },
            WindowsAndMessaging::{GetCursorPos, SetCursorPos},
        },
    },
};

pub fn get_cursor_pos() -> Result<POINT> {
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
