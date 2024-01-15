/*
DE:
Schreibe eine App, in welche die User*in ihre Schrittzahlen der letzten n Tage eingeben kann
Die App berechnet daraus das arithmetische Mittel https://de.wikipedia.org/wiki/Mittelwert#Arithmetischer_Mittelwert
Nutze hierbei die sum() Funktion um die Werte aufzusummieren.
Beachte, dass du Integer und keine String aufsummieren willst
Gib abschließend das Ergebnis gerundet und ohne Nachkommastellen aus

EN:
Write an app in which the user can enter their step counts for the last n days
The app calculates the arithmetic mean from this https://en.wikipedia.org/wiki/Average#Arithmetic_mean
Use the sum() function to add up the values.
Note that you want to add up integers and not strings
Finally, output the result rounded and without decimal places
*/

use chrono;
use chrono::prelude::*;
use chrono::Duration;
use std::io;

fn main() {
    let local: DateTime<Local> = Local::now();
    let raw_total_days = input("How many days?");
    let total_days = match raw_total_days.parse::<i32>() {
        Ok(i) => i,
        Err(_) => 1,
    };

    let mut vec: Vec<i32> = Vec::new();
    for day in (0..total_days).rev() {
        let date_to_enter = local - Duration::days(day as i64);
        let raw_steps = input(
            format!(
                "Steps from {:02}.{:02}.{}",
                date_to_enter.day(),
                date_to_enter.month(),
                date_to_enter.year()
            )
            .as_str(),
        );
        let steps = match raw_steps.parse::<i32>() {
            Ok(i) => i,
            Err(_) => 0,
        };
        vec.push(steps)
    }

    let total: i32 = vec.iter().sum();
    println!("total steps {}, average {}", total, total / total_days);
}

fn input(prompt: &str) -> String {
    let stdin = io::stdin();

    print!("{}\n", prompt);
    let mut raw_input = String::new();
    match stdin.read_line(&mut raw_input) {
        Ok(_) => {
            raw_input = raw_input.trim().to_string();

            raw_input
        }
        Err(_) => String::new(),
    }
}
