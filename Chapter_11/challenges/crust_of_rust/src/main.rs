use game::Game;
use player::Player;
use statement::Statement;

mod game;
mod player;
mod statement;

use std::io;

#[macro_export]
macro_rules! user_input {
    () => {{
        let stdio = io::stdin();

        let mut raw_input = String::new();
        match stdio.read_line(&mut raw_input) {
            Ok(_) => raw_input.trim().to_string(),
            Err(_) => String::new(),
        }
    }};
}

fn main() {
    let mut game = Game::new(vec![
        Statement::new("Rust is primarily a functional programming language.".to_string(), false),
        Statement::new("Rust was the 'most loved programming language' in the Stack Overflow Developer Survey from 2016 to 2020.".to_string(), true),
        Statement::new("Rust does not have a garbage collector.".to_string(), true),
        Statement::new("Rust was originally designed by Graydon Hoare.".to_string(), true),
        Statement::new("Rust allows null pointers for memory safety.".to_string(), false),
        Statement::new("In Rust, variables are immutable by default.".to_string(), true),
        Statement::new("Rust is developed and sponsored by Microsoft.".to_string(), false),
        Statement::new("Rust uses a borrowing and ownership system for memory management.".to_string(), true),
        Statement::new("Rust does not support asynchronous programming.".to_string(), false),
        Statement::new("The Rust compiler is named 'rustc'.".to_string(), true),
    ]);

    let mut exit_game = false;
    let mut current_statement: Option<Statement> = None;

    while !exit_game {
        match game.current_state() {
            game::GameState::IDLE => {
                println!("What you gonna do?\n\t1. Play Game\n\t2. Display Hall Of Fame\n\t3. Exit");
                let input = user_input!();

                match input.parse::<i32>() {
                    Ok(selection) => {
                        match selection {
                            1 => {
                                match game.unidle() {
                                    Ok(_) => {},
                                    Err(_) => println!("Could not start game!"),
                                }
                            },
                            2 => {
                                println!("\nHall of Fame:");
                                let hof = game.get_hall_of_fame();
                                for player in hof {
                                    println!("{}", player);
                                }

                                println!("");
                            },
                            3 => {
                                exit_game = true;
                                println!("\nBye!");
                            }
                            _ => println!("unknown selection"),
                        }
                    },
                    Err(_) => println!("unknown selection"),
                }
            },
            game::GameState::NEW_GAME => match game.start(Player::new()) {
                Ok(statement) => {
                    println!(
                        "\nIs the following statement correct?\n{}",
                        statement.question
                    );
                    current_statement = Some(statement);
                }
                Err(_) => println!("Could not start game!"),
            },
            game::GameState::PLAYING => {
                match &current_statement {
                    Some(statement) => {
                        let input = user_input!();
                        match game.play(statement, input) {
                            Ok(statement) => {
                                println!("The answer was correct!\n");

                                println!(
                                    "\nIs the following statement correct?\n{}",
                                    statement.question
                                );

                                current_statement = Some(statement);
                            },
                            Err(state) => {
                                match state {
                                    game::PlayResult::WRONG_ANSWER => {
                                        println!("The answer was NOT correct!");
                                    },
                                    game::PlayResult::NO_MORE_ANSWERS => {
                                        println!("The answer was correct!\n");
                                        println!("These were all questions, good job answering them!");
                                    },
                                    _ => println!("Error 37"),
                                }
                            },
                        }
                    },
                    None => println!("No active statement!"),
                }
            },
            game::GameState::END_GAME => {
                println!("Please enter your name for the scoreboard: ");
                let input = user_input!();
                match game.end_game(input.to_string()) {
                    Ok(score) => println!("GG {} you scored {}", input, score),
                    Err(_) => println!("Uh, Oh, something went wrong saving your result"),
                }
            },
        }
    }
}
