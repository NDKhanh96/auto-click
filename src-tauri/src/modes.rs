use crate::mouse;

pub enum ClickMode {
    Fixed { x: i32, y: i32 },
    FixedWithRestore { x: i32, y: i32 },
}

pub fn run(mode: ClickMode) -> windows::core::Result<()> {
    match mode {
        ClickMode::Fixed { x, y } => {
            mouse::click_at(x, y)?;
        }

        ClickMode::FixedWithRestore { x, y } => {
            let old = mouse::get_cursor_pos()?;
            mouse::click_at(x, y)?;
            mouse::set_cursor_pos(old.x, old.y)?;
        }
    }

    Ok(())
}
