// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

slint::include_modules!();
 
fn main() -> Result<(), Box<dyn Error>> {
    let display_ui = PublicWindow::new()?;
    let input_ui = ControlWindow::new()?;

    display_ui.on_request_increase_value({
        let ui_handle = display_ui.as_weak();
        move || { 
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
        }
    });

    input_ui.show()?;
    display_ui.run()?;

    Ok(())
}
