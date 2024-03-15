//  this is a  rock paper scissors
use colored::Colorize;
use rand::Rng;
use std::io;

fn main() {
    const STATES: [&str; 3] = ["rock", "paper", "scissors"];
    let mut system_score: u8 = 0;
    let mut player_score: u8 = 0;
    let mut round_played: u32 = 1;

    loop {
        if round_played == 1 {
            println!(
                "{}",
                "welcome to game this is a rock paper scissors game".yellow()
            );
            println!("{}", "for playing game please write rock or paper or scissors and for quiting game write q or quite , good luck game round".yellow());
        }

        println!("{} round {}", "input rock paper scissors".blue() , round_played);
        let system_play_index = rand::thread_rng().gen_range(1..STATES.len());
        let system_play_state = STATES[system_play_index];

        let mut player_state = String::new();
        io::stdin()
            .read_line(&mut player_state)
            .expect("failed to read the line");

        let player_state = player_state.trim();
        if player_state == "q" {
            break;
        }

        if !STATES.contains(&player_state) {
            println!(
                "{}",
                "its not a part of game please input rock paper or scissors".yellow()
            );
            continue;
        }

        match (system_play_state, player_state) {
            ("rock", "scissors") | ("paper", "rock") | ("scissors", "paper") => {
                println!("{}", "You have lost".red());
                system_score += 1;
            }
            ("rock", "paper") | ("paper", "scissors") | ("scissors", "rock") => {
                println!("{}", "You have won".green());
                player_score += 1;
            }
            _ => println!("It's a draw!"),
        }

        println!(
            "the system played {} and you played {}",
            system_play_state.blue(),
            player_state.blue()
        );

        println!(
            "System score: {}, Your score: {}",
            system_score, player_score
        );
        round_played += 1;
        if round_played > 10 {
            println!(
                "the game is over the scors are system : {} and you {}",
                system_score, player_score
            );
            if system_score > player_score {
                println!(
                    "{}",
                    "you have lost the match maybe next time you could win".red()
                );
            } else if player_score > system_score {
                println!("{}", "you have won the match you look like pro".green());
            } else {
                println!("the game was a draw");
            }
            break;
        }
    }
}
