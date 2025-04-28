#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::error::Error;
use std::rc::Rc;
use std::time::Duration;
use slint::{Model, VecModel};

slint::include_modules!();

#[derive(Debug, Clone)]
struct Team {
    number: u8,
    name: String,
    members: String,
    rank: u8,
    wins: u8,
    losses: u8,
    ties: u8,
    rp: u8,
}

#[derive(Debug, Clone)]
struct Match {
    red_alliance: (usize, usize),   // use usize to index into Vec
    blue_alliance: (usize, usize),
}

fn main() -> Result<(), Box<dyn Error>> {
    let score_ui = ScoreWindow::new()?;
    let control_ui = ControlWindow::new()?;

    // Create shared handles to both UIs for synchronization
    let score_ui_weak = score_ui.as_weak();
    let control_ui_weak = control_ui.as_weak();

    let team_data = Rc::new(VecModel::from(vec![
        Team { number: 1, name: "Deuce".into(), members: "Ian - JT".into(), rank: 0, wins: 0, losses: 0, ties: 0, rp: 0 },
        Team { number: 2, name: "FreakBot 9000".into(), members: "Logan - Bennett".into(), rank: 0, wins: 0, losses: 0, ties: 0, rp: 0},
        Team { number: 3, name: "JPC".to_string(), members: "Sam - Xavier".to_string(), rank: 0, wins: 0, losses: 0, ties: 0, rp: 0},
        Team { number: 4, name: "Cooper Bot".to_string(), members: "John - Scott".to_string(), rank: 0, wins: 0, losses: 0, ties: 0, rp: 0},
        Team { number: 5, name: "Fat Man".to_string(), members: "Austin - Evan".to_string(), rank: 0, wins: 0, losses: 0, ties: 0, rp: 0},
        Team { number: 6, name: "Tau".to_string(), members: "Fabian - Tanav - Cru".to_string(), rank: 0, wins: 0, losses: 0, ties: 0, rp: 0},
        Team { number: 7, name: "'Might' Blow Up".to_string(), members: "Andrew - Jack - Jackson L".to_string(), rank: 0, wins: 0, losses: 0, ties: 0, rp: 0},
        Team { number: 8, name: "Phi".to_string(), members: "Jesse - Zach - Drew".to_string(), rank: 0, wins: 0, losses: 0, ties: 0, rp: 0},
    ]));

    let match_schedule = vec![
        Match { red_alliance: (0, 1), blue_alliance: (2, 3) },
        Match { red_alliance: (4, 5), blue_alliance: (6, 7) },
        Match { red_alliance: (0, 2), blue_alliance: (4, 6) },
        Match { red_alliance: (1, 3), blue_alliance: (5, 7) },
        Match { red_alliance: (0, 4), blue_alliance: (1, 5) },
        Match { red_alliance: (2, 6), blue_alliance: (3, 7) },
        Match { red_alliance: (0, 5), blue_alliance: (2, 7) },
        Match { red_alliance: (1, 6), blue_alliance: (3, 4) },
        Match { red_alliance: (0, 6), blue_alliance: (3, 5) },
        Match { red_alliance: (1, 7), blue_alliance: (2, 4) },
    ];

    // Set up a timer to check and sync matchNumber between windows
    let timer = slint::Timer::default();
    let previous_value = std::rc::Rc::new(std::cell::RefCell::new(1));

    let schedule = match_schedule.clone(); // put this in Rc if needed
    let teams = team_data.clone(); // your VecModel or plain Vec<Team>

    // Correct parameter order: (mode, duration, callback)
    timer.start(
        slint::TimerMode::Repeated,
        Duration::from_millis(100),
        move || {
            // Check if the control window's matchNumber has changed
            if let (Some(control), Some(score)) = (control_ui_weak.upgrade(), score_ui_weak.upgrade()) {

                let match_num = control.global::<vars>().get_matchNumber();
                if match_num > 0 && (match_num as usize) <= schedule.len() {

                    let current_match = &schedule[(match_num - 1) as usize];

                    let teams_borrowed = teams.clone(); // optional if already in scope

                    if let (Some(red1), Some(red2), Some(blue1), Some(blue2)) = (
                        teams_borrowed.row_data(current_match.red_alliance.0),
                        teams_borrowed.row_data(current_match.red_alliance.1),
                        teams_borrowed.row_data(current_match.blue_alliance.0),
                        teams_borrowed.row_data(current_match.blue_alliance.1),
                    ) {
                        score.global::<vars>().set_red_team1(red1.name.clone().into());
                        score.global::<vars>().set_red_team2(red2.name.clone().into());
                        score.global::<vars>().set_blue_team1(blue1.name.clone().into());
                        score.global::<vars>().set_blue_team2(blue2.name.clone().into());
                        
                        score.global::<vars>().set_red_team1_members(red1.members.clone().into());
                        score.global::<vars>().set_red_team2_members(red2.members.clone().into());
                        score.global::<vars>().set_blue_team1_members(blue1.members.clone().into());
                        score.global::<vars>().set_blue_team2_members(blue2.members.clone().into());
                    }
                }

                let control_value = control.global::<vars>().get_matchNumber();
                let score_value = score.global::<vars>().get_matchNumber();

                let mut prev = previous_value.borrow_mut();

                // If the control value has changed from previous, update the score window
                if control_value != *prev {
                    score.global::<vars>().set_matchNumber(control_value);
                    *prev = control_value;
                }
                // If the score value has changed and is different from control, update the control window
                else if score_value != control_value {
                    control.global::<vars>().set_matchNumber(score_value);
                    *prev = score_value;
                }
            }
        },
    );

    // Handle the counter increase callback
    /*score_ui.on_request_increase_value({
        let ui_handle = score_ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
        }
    });*/

    // Show both windows
    control_ui.show()?;
    score_ui.run()?; // This will block until the score window is closed

    Ok(())
}