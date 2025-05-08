#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{error::Error, rc::Rc};
use std::time::Duration;
use slint::{Model, VecModel};

use std::sync::{Arc, Mutex};

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
    average_score: u8,
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

    control_ui.on_switch_tab({
        let score_ui_weak = score_ui_weak.clone();
        move |new_index| {
            if let Some(w2) = score_ui_weak.upgrade() {
                w2.global::<vars>().set_current_menu(new_index);
            }
        }
    });

    let team_data = Rc::new(VecModel::from(vec![
        Team { number: 1, name: "Deuce 🎾".into(), members: "Ian - JT".into(), rank: 0, wins: 0, losses: 0, ties: 0, average_score: 0, rp: 0 },
        Team { number: 2, name: "AK-5000".into(), members: "Logan - Bennett".into(), rank: 0, wins: 0, losses: 0, ties: 0, average_score: 0, rp: 0},
        Team { number: 3, name: "JPC".to_string(), members: "Sam - Xavier".to_string(), rank: 0, wins: 0, losses: 0, ties: 0, average_score: 0, rp: 0},
        Team { number: 4, name: "Cooper Bot".to_string(), members: "John - Scott".to_string(), rank: 0, wins: 0, losses: 0, ties: 0, average_score: 0, rp: 0},
        Team { number: 5, name: "Fat Man".to_string(), members: "Austin - Evan".to_string(), rank: 0, wins: 0, losses: 0, ties: 0, average_score: 0, rp: 0},
        Team { number: 6, name: "Steve".to_string(), members: "Fabian - Tanav - Cru".to_string(), rank: 0, wins: 0, losses: 0, ties: 0, average_score: 0, rp: 0},
        Team { number: 7, name: "'Might' Blow Up".to_string(), members: "Andrew - Jack - Jackson L".to_string(), rank: 0, wins: 0, losses: 0, ties: 0, average_score: 0, rp: 0},
        Team { number: 8, name: "Sauron".to_string(), members: "Jesse - Zach - Drew".to_string(), rank: 0, wins: 0, losses: 0, ties: 0, average_score: 0, rp: 0},
    ]));

    let alliance_1: [i32; 2] = [0, 0];
    let alliance_2: [i32; 2] = [0, 0];
    let alliance_3: [i32; 2] = [0, 0];
    let alliance_4: [i32; 2] = [0, 0];

    let mut match_schedule = vec![
        Match { red_alliance: (2, 7), blue_alliance: (3, 6)},
        Match { red_alliance: (1, 3), blue_alliance: (0, 2)},
        Match { red_alliance: (6, 7), blue_alliance: (4, 5)},
        Match { red_alliance: (4, 6), blue_alliance: (5, 7)},
        Match { red_alliance: (1, 2), blue_alliance: (0, 3)},
        Match { red_alliance: (5, 6), blue_alliance: (4, 7)},
        Match { red_alliance: (0, 4), blue_alliance: (1, 5)},
        Match { red_alliance: (2, 6), blue_alliance: (3, 7)},
        Match { red_alliance: (0, 7), blue_alliance: (1, 6)},
        Match { red_alliance: (3, 4), blue_alliance: (2, 5)},
        Match { red_alliance: (0, 5), blue_alliance: (1, 4)},
        Match { red_alliance: (2, 3), blue_alliance: (0, 1)},
        Match { red_alliance: (3, 5), blue_alliance: (2, 4)},
        Match { red_alliance: (0, 6), blue_alliance: (1, 7)},
    ];

    let mut blue_auto: [i32; 30] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    let mut red_auto: [i32; 30] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    let mut blue_grid: [i32; 30] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    let mut red_grid: [i32; 30] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    let mut blue_array: [i32; 30] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    let mut red_array: [i32; 30] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    let mut blue_endgame: [i32; 30] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    let mut red_endgame: [i32; 30] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    let mut blue_bonus: [i32; 30] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    let mut red_bonus: [i32; 30] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    let mut blue_score: [i32; 30] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    let mut red_score: [i32; 30] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];

    let mut add_match: [usize; 5] = [1,0,0,0,0];

    // Initialize the shared state
    let add_match_state: Arc<Mutex<[usize; 5]>> = Arc::new(Mutex::new([0usize; 5]));

    // Clone for each closure
    let add_match_for_callback = add_match_state.clone();
    let add_match_for_timer = add_match_state.clone();

    // Set up a timer to check and sync matchNumber between windows
    let timer = slint::Timer::default();
    let previous_value = std::rc::Rc::new(std::cell::RefCell::new(1));

    let schedule = Rc::new(std::cell::RefCell::new(match_schedule.clone())); // Wrap in Rc<RefCell> for interior mutability
    let teams: Rc<Rc<VecModel<Team>>> = Rc::new(team_data.clone()); // Wrap in Rc for shared ownership

    let mut old_match_number = 1;

    control_ui.on_add_match(move |red1, red2, blue1, blue2| {
        // Add the new match to the vector
        // We need to determine the correct way to update match_schedule based on its type
        
        // If match_schedule is a Vec<Match>:
        match_schedule.push(Match {
            red_alliance: (red1 as usize, red2 as usize),
            blue_alliance: (blue1 as usize, blue2 as usize),
        });
        
        /* 
        // If match_schedule is a Rc<RefCell<Vec<Match>>>:
        match_schedule.borrow_mut().push(Match {
            red_alliance: (red1 as usize, red2 as usize),
            blue_alliance: (blue1 as usize, blue2 as usize),
        });
        */
        
        // Lock the mutex and update the shared state
        let mut add_match = add_match_for_callback.lock().unwrap();
        add_match[0] = 1;  // Signal that a match was added
        add_match[1] = red1 as usize;
        add_match[2] = red2 as usize;
        add_match[3] = blue1 as usize;
        add_match[4] = blue2 as usize;
        
        // For debugging - print the newly added match
        println!("Added match: Red ({}, {}) vs Blue ({}, {})", red1, red2, blue1, blue2);
        println!("Set flag to: {}", add_match[0]);
    });

    // Correct parameter order: (mode, duration, callback)
    timer.start(
        slint::TimerMode::Repeated,
        Duration::from_millis(100),
        move || {
            // Check if the control window's matchNumber has changed
            if let (Some(control), Some(score)) = (control_ui_weak.upgrade(), score_ui_weak.upgrade()) {
                let teams = teams.clone();
                let mut schedule = schedule.clone();
                
                if control.global::<vars>().get_matchNumber() != old_match_number { 
                    let teams_borrowed: Rc<Rc<VecModel<Team>>> = teams.clone();
                    let match_index = old_match_number as usize - 1;

                   // Try to lock the shared state
                   if let Ok(mut add_match) = add_match_for_timer.try_lock() {
                       // Check if there's a new match to add
                       if add_match[0] == 1 {
                           println!("Timer detected new match - Flag value: {}", add_match[0]);

                           // Add the match to the schedule with explicit typing
                           let mut schedule_vec: std::cell::RefMut<'_, Vec<Match>> = schedule.borrow_mut();
                           schedule_vec.push(
                               Match { 
                                   red_alliance: (add_match[1], add_match[2]), 
                                   blue_alliance: (add_match[3], add_match[4])
                               }
                           );

                           // Reset the flag after adding
                           add_match[0] = 0;
                           println!("Timer reset flag to: {}", add_match[0]);
                       }
                   }

                    let current_match = &schedule.borrow()[match_index];
            
                    if let (Some(mut red1), Some(mut red2), Some(mut blue1), Some(mut blue2)) = (
                        teams_borrowed.row_data(current_match.red_alliance.0),
                        teams_borrowed.row_data(current_match.red_alliance.1),
                        teams_borrowed.row_data(current_match.blue_alliance.0),
                        teams_borrowed.row_data(current_match.blue_alliance.1),
                    ) {
                        let red_total = red_score[match_index];
                        let blue_total = blue_score[match_index];
                        let red_end = red_endgame[match_index];
                        let blue_end = blue_endgame[match_index];
            
                        if blue_total > red_total {
                            blue1.wins += 1;
                            blue2.wins += 1;
                            blue1.rp += 3;
                            blue2.rp += 3;
                            if blue_end > red_end {
                                blue1.rp += 1;
                                blue2.rp += 1;
                            }
                            red1.losses += 1;
                            red2.losses += 1;
                        } else if red_total > blue_total {
                            red1.wins += 1;
                            red2.wins += 1;
                            red1.rp += 3;
                            red2.rp += 3;
                            if red_end > blue_end {
                                red1.rp += 1;
                                red2.rp += 1;
                            }
                            blue1.losses += 1;
                            blue2.losses += 1;
                        } else {
                            // It's a tie
                            red1.ties += 1;
                            red2.ties += 1;
                            blue1.ties += 1;
                            blue2.ties += 1;
            
                            red1.rp += 1;
                            red2.rp += 1;
                            blue1.rp += 1;
                            blue2.rp += 1;
                        }
            
                        // Optionally: Update average score
                        red1.average_score = (red1.average_score + red_total as u8) / 2;
                        red2.average_score = (red2.average_score + red_total as u8) / 2;
                        blue1.average_score = (blue1.average_score + blue_total as u8) / 2;
                        blue2.average_score = (blue2.average_score + blue_total as u8) / 2;
            
                        // Push back updates to VecModel
                        teams_borrowed.set_row_data(current_match.red_alliance.0, red1);
                        teams_borrowed.set_row_data(current_match.red_alliance.1, red2);
                        teams_borrowed.set_row_data(current_match.blue_alliance.0, blue1);
                        teams_borrowed.set_row_data(current_match.blue_alliance.1, blue2);
                    }
                    let team_list = &teams; // Rc<VecModel<Team>>

                    // Collect teams into a vector for sorting
                    let mut all_teams: Vec<_> = (0..team_list.row_count())
                        .map(|i| team_list.row_data(i).unwrap())
                        .collect();
                    
                    // Sort by RP, then Wins
                    all_teams.sort_by(|a, b| {
                        b.rp.cmp(&a.rp)
                            .then_with(|| b.average_score.partial_cmp(&a.average_score).unwrap_or(std::cmp::Ordering::Equal))
                    });
                    
                    // 🔹 Print the sorted list
                    println!("--- Team Rankings ---");
/*                     for (i, team) in all_teams.iter().enumerate() {
                        println!(
                            "#{:2} - Team {} | Wins: {} | RP: {} | Ties: {} | Losses: {} | Name: {}",
                            i + 1,
                            team.number,
                            team.wins,
                            team.rp,
                            team.ties,
                            team.losses,
                            team.name
                        );
                    }
                     */
                    // 🔹 Update the ranks in the model
                    for (i, team) in all_teams.iter().enumerate() {
                        let mut updated_team = team.clone();
                        updated_team.rank = (i + 1) as u8;
                    
                        // Find the index of this team in the original model
                        if let Some(index) = (0..team_list.row_count()).find(|&j| {
                            let t = team_list.row_data(j).unwrap();
                            t.number == updated_team.number
                        }) {
                            team_list.set_row_data(index, updated_team);
                        }
                    }

                    score.global::<team_rank>().set_rank1_n(slint::SharedString::from(all_teams[0].name.clone()));
                    score.global::<team_rank>().set_rank2_n(slint::SharedString::from(all_teams[1].name.clone()));
                    score.global::<team_rank>().set_rank3_n(slint::SharedString::from(all_teams[2].name.clone()));
                    score.global::<team_rank>().set_rank4_n(slint::SharedString::from(all_teams[3].name.clone()));
                    score.global::<team_rank>().set_rank5_n(slint::SharedString::from(all_teams[4].name.clone()));
                    score.global::<team_rank>().set_rank6_n(slint::SharedString::from(all_teams[5].name.clone()));
                    score.global::<team_rank>().set_rank7_n(slint::SharedString::from(all_teams[6].name.clone()));
                    score.global::<team_rank>().set_rank8_n(slint::SharedString::from(all_teams[7].name.clone()));

                    score.global::<team_rank>().set_rank1_rp(slint::SharedString::from(all_teams[0].rp.to_string() + " RP"));
                    score.global::<team_rank>().set_rank2_rp(slint::SharedString::from(all_teams[1].rp.to_string() + " RP"));
                    score.global::<team_rank>().set_rank3_rp(slint::SharedString::from(all_teams[2].rp.to_string() + " RP"));
                    score.global::<team_rank>().set_rank4_rp(slint::SharedString::from(all_teams[3].rp.to_string() + " RP"));
                    score.global::<team_rank>().set_rank5_rp(slint::SharedString::from(all_teams[4].rp.to_string() + " RP"));
                    score.global::<team_rank>().set_rank6_rp(slint::SharedString::from(all_teams[5].rp.to_string() + " RP"));
                    score.global::<team_rank>().set_rank7_rp(slint::SharedString::from(all_teams[6].rp.to_string() + " RP"));
                    score.global::<team_rank>().set_rank8_rp(slint::SharedString::from(all_teams[7].rp.to_string() + " RP"));

                    score.global::<team_rank>().set_rank1_as(slint::SharedString::from(all_teams[0].average_score.to_string() + " Avg Score"));
                    score.global::<team_rank>().set_rank2_as(slint::SharedString::from(all_teams[1].average_score.to_string() + " Avg Score"));
                    score.global::<team_rank>().set_rank3_as(slint::SharedString::from(all_teams[2].average_score.to_string() + " Avg Score"));
                    score.global::<team_rank>().set_rank4_as(slint::SharedString::from(all_teams[3].average_score.to_string() + " Avg Score"));
                    score.global::<team_rank>().set_rank5_as(slint::SharedString::from(all_teams[4].average_score.to_string() + " Avg Score"));
                    score.global::<team_rank>().set_rank6_as(slint::SharedString::from(all_teams[5].average_score.to_string() + " Avg Score"));
                    score.global::<team_rank>().set_rank7_as(slint::SharedString::from(all_teams[6].average_score.to_string() + " Avg Score"));
                    score.global::<team_rank>().set_rank8_as(slint::SharedString::from(all_teams[7].average_score.to_string() + " Avg Score"));
                    
                    reset_all_scores(&control);
                    old_match_number = control.global::<vars>().get_matchNumber().clone();
                }

                let match_num = control.global::<vars>().get_matchNumber();
                if match_num > 0 && (match_num as usize) <= schedule.borrow().len() {

                    let current_match = &schedule.borrow()[(match_num - 1) as usize];

                    let teams_borrowed = teams.clone(); // optional if already in scope

                    blue_auto[match_num as usize - 1] = control.global::<score_blue>().get_park() + control.global::<score_blue>().get_auto_ground() + control.global::<score_blue>().get_auto_beacon() + control.global::<score_blue>().get_auto_center() + control.global::<score_blue>().get_auto_L1() + control.global::<score_blue>().get_auto_L2() + control.global::<score_blue>().get_auto_L3();
                    red_auto[match_num as usize - 1] = control.global::<score_red>().get_park() + control.global::<score_red>().get_auto_ground() + control.global::<score_red>().get_auto_beacon() + control.global::<score_red>().get_auto_center() + control.global::<score_red>().get_auto_L1() + control.global::<score_red>().get_auto_L2() + control.global::<score_red>().get_auto_L3();

                    blue_grid[match_num as usize - 1] = control.global::<score_blue>().get_L1() + control.global::<score_blue>().get_L2() + control.global::<score_blue>().get_L3() + control.global::<score_blue>().get_ground();
                    red_grid[match_num as usize - 1] = control.global::<score_red>().get_L1() + control.global::<score_red>().get_L2() + control.global::<score_red>().get_L3() + control.global::<score_red>().get_ground();

                    blue_array[match_num as usize - 1] = control.global::<score_blue>().get_ground() + control.global::<score_blue>().get_beacon() + control.global::<score_blue>().get_center();
                    red_array[match_num as usize - 1] = control.global::<score_red>().get_ground() + control.global::<score_red>().get_beacon() + control.global::<score_red>().get_center();

                    blue_bonus[match_num as usize - 1] = control.global::<score_blue>().get_primary() + control.global::<score_blue>().get_power_up() + control.global::<score_blue>().get_redundant();
                    red_bonus[match_num as usize - 1] = control.global::<score_red>().get_primary() + control.global::<score_red>().get_power_up() + control.global::<score_red>().get_redundant();

                    blue_endgame[match_num as usize - 1] = control.global::<score_blue>().get_end_park() + control.global::<score_blue>().get_climb();
                    red_endgame[match_num as usize - 1] = control.global::<score_red>().get_end_park() + control.global::<score_red>().get_climb();

                    blue_score[match_num as usize - 1] = blue_auto[match_num as usize - 1] + blue_grid[match_num as usize - 1] + blue_array[match_num as usize - 1] + blue_endgame[match_num as usize - 1] + blue_bonus[match_num as usize - 1];
                    red_score[match_num as usize - 1] = red_auto[match_num as usize - 1] + red_grid[match_num as usize - 1] + red_array[match_num as usize - 1] + red_endgame[match_num as usize - 1] + red_bonus[match_num as usize - 1];


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

                        score.global::<display_score>().set_blue_auto(blue_auto[match_num as usize - 1]);
                        score.global::<display_score>().set_red_auto(red_auto[match_num as usize - 1]);

                        score.global::<vars>().set_blue_score(blue_score[match_num as usize - 1]);
                        score.global::<vars>().set_red_score(red_score[match_num as usize - 1]);
                    }
                }

                /*
                control.on_submit_score({
                    let teams = teams.clone();
                    let schedule = schedule.clone();
                    move || {
                        let teams_borrowed: Rc<Rc<VecModel<Team>>> = teams.clone();
                        let match_index = match_num as usize - 1;
                        let current_match = &schedule[match_index];
                
                        if let (Some(mut red1), Some(mut red2), Some(mut blue1), Some(mut blue2)) = (
                            teams_borrowed.row_data(current_match.red_alliance.0),
                            teams_borrowed.row_data(current_match.red_alliance.1),
                            teams_borrowed.row_data(current_match.blue_alliance.0),
                            teams_borrowed.row_data(current_match.blue_alliance.1),
                        ) {
                            let red_total = red_score[match_index];
                            let blue_total = blue_score[match_index];
                            let red_end = red_endgame[match_index];
                            let blue_end = blue_endgame[match_index];
                
                            if blue_total > red_total {
                                blue1.wins += 1;
                                blue2.wins += 1;
                                blue1.rp += 3;
                                blue2.rp += 3;
                                if blue_end > red_end {
                                    blue1.rp += 1;
                                    blue2.rp += 1;
                                }
                                red1.losses += 1;
                                red2.losses += 1;
                            } else if red_total > blue_total {
                                red1.wins += 1;
                                red2.wins += 1;
                                red1.rp += 3;
                                red2.rp += 3;
                                if red_end > blue_end {
                                    red1.rp += 1;
                                    red2.rp += 1;
                                }
                                blue1.losses += 1;
                                blue2.losses += 1;
                            } else {
                                // It's a tie
                                red1.ties += 1;
                                red2.ties += 1;
                                blue1.ties += 1;
                                blue2.ties += 1;
                
                                red1.rp += 1;
                                red2.rp += 1;
                                blue1.rp += 1;
                                blue2.rp += 1;
                            }
                
                            // Optionally: Update average score
                            red1.average_score = (red1.average_score + red_total as u8) / 2;
                            red2.average_score = (red2.average_score + red_total as u8) / 2;
                            blue1.average_score = (blue1.average_score + blue_total as u8) / 2;
                            blue2.average_score = (blue2.average_score + blue_total as u8) / 2;
                
                            // Push back updates to VecModel
                            teams_borrowed.set_row_data(current_match.red_alliance.0, red1);
                            teams_borrowed.set_row_data(current_match.red_alliance.1, red2);
                            teams_borrowed.set_row_data(current_match.blue_alliance.0, blue1);
                            teams_borrowed.set_row_data(current_match.blue_alliance.1, blue2);
                        }
                    }
                }); */
                
                let control_value = control.global::<vars>().get_matchNumber();
                let score_value = score.global::<vars>().get_matchNumber();

                let mut prev = previous_value.borrow_mut();

                // If the control value has changed from previous, update the score window
                if control_value != *prev {
                    score.global::<vars>().set_matchNumber(control_value);
                    *prev = control_value as i32;
                }
                // If the score value has changed and is different from control, update the control window
                else if score_value != control_value {
                    control.global::<vars>().set_matchNumber(score_value);
                    *prev = score_value as i32;
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

fn reset_all_scores(ui: &ControlWindow) {
    // Reset score_blue properties
    let score_blue = ui.global::<score_blue>();
    score_blue.set_park(0);
    score_blue.set_auto_ground(0);
    score_blue.set_auto_beacon(0);
    score_blue.set_auto_center(0);
    score_blue.set_auto_L1(0);
    score_blue.set_auto_L2(0);
    score_blue.set_auto_L3(0);
    score_blue.set_ground(0);
    score_blue.set_beacon(0);
    score_blue.set_center(0);
    score_blue.set_L1(0);
    score_blue.set_L2(0);
    score_blue.set_L3(0);
    score_blue.set_end_park(0);
    score_blue.set_climb(0);
    score_blue.set_primary(0);
    score_blue.set_power_up(0);
    score_blue.set_redundant(0);

    // Reset score_red properties
    let score_red = ui.global::<score_red>();
    score_red.set_park(0);
    score_red.set_auto_ground(0);
    score_red.set_auto_beacon(0);
    score_red.set_auto_center(0);
    score_red.set_auto_L1(0);
    score_red.set_auto_L2(0);
    score_red.set_auto_L3(0);
    score_red.set_ground(0);
    score_red.set_beacon(0);
    score_red.set_center(0);
    score_red.set_L1(0);
    score_red.set_L2(0);
    score_red.set_L3(0);
    score_red.set_end_park(0);
    score_red.set_climb(0);
    score_red.set_primary(0);
    score_red.set_power_up(0);
    score_red.set_redundant(0);

    // Reset display_score properties
    let display_score = ui.global::<display_score>();
    display_score.set_blue_auto(0);
    display_score.set_red_auto(0);
    display_score.set_blue_score(0);
    display_score.set_red_score(0);
    display_score.set_blue_array(0);
    display_score.set_red_array(0);
    display_score.set_blue_grid(0);
    display_score.set_red_grid(0);
    display_score.set_blue_endgame(0);
    display_score.set_red_endgame(0);
    display_score.set_blue_bonus(0);
    display_score.set_red_bonus(0);
}