/*
DE: Lasse die Nutzer*in eine zweistellige Zahl eingeben und gib dann die Quersumme aus.
EN: Let the user enter a two-digit number and then output the cross sum.
*/

use std::io;

fn main() {
    let stdin = io::stdin();
    
    loop {
        let mut raw_input = String::new();
        match stdin.read_line(&mut raw_input) {
            Ok(_) => {
                raw_input = raw_input.trim().to_string();
            }
            Err(_) => {
                println!("Enter something you peasant!");
                continue;
            }
        }


        if raw_input.len() == 2 {
            match raw_input.parse::<i32>() {
                Ok(n) => {
                    // checked before if real number
                    let i = raw_input[0..1].parse::<i32>().unwrap();
                    let j = raw_input[1..2].parse::<i32>().unwrap();

                    println!("Cross Sum of {n} is {}", i + j);
                    break;
                }
                Err(_) => {
                    println!("You  didn't enter a number!")
                }
            }
        } else {
            println!("Enter a two digit number, you peasant!");
        }
    }


}
