#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::error::Error;
use std::time::Duration;
slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let score_ui = ScoreWindow::new()?;
    let control_ui = ControlWindow::new()?;

    // Create shared handles to both UIs for synchronization
    let score_ui_weak = score_ui.as_weak();
    let control_ui_weak = control_ui.as_weak();

    // Set up a timer to check and sync matchNumber between windows
    let timer = slint::Timer::default();
    let previous_value = std::rc::Rc::new(std::cell::RefCell::new(1));

    // Correct parameter order: (mode, duration, callback)
    timer.start(
        slint::TimerMode::Repeated,
        Duration::from_millis(100),
        move || {
            // Check if control window's matchNumber has changed
            if let (Some(control), Some(score)) = (control_ui_weak.upgrade(), score_ui_weak.upgrade()) {
                let control_value = control.global::<vars>().get_matchNumber();
                let score_value = score.global::<vars>().get_matchNumber();

                let mut prev = previous_value.borrow_mut();

                // If control value has changed from previous, update score window
                if control_value != *prev {
                    score.global::<vars>().set_matchNumber(control_value);
                    *prev = control_value;
                }
                // If score value has changed and is different from control, update control window
                else if score_value != control_value {
                    control.global::<vars>().set_matchNumber(score_value);
                    *prev = score_value;
                }
            }
        },
    );

    // Handle the counter increase callback
    score_ui.on_request_increase_value({
        let ui_handle = score_ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
        }
    });

    // Show both windows
    control_ui.show()?;
    score_ui.run()?; // This will block until the score window is closed

    Ok(())
}