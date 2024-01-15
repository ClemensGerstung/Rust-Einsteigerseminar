/*
DE:
Schreibe ein Programm welches prüft, ob ein Jahr ein Schaltjahr ist
Die gregorianische Schalttagsregelung besteht aus folgenden drei einzelnen Regeln:
1. Die durch 4 ganzzahlig teilbaren Jahre sind, abgesehen von den folgenden Ausnahmen, Schaltjahre.
2. Säkularjahre, also die Jahre, die ein Jahrhundert abschließen (z. B. 1800, 1900, 2100 und 2200),
   sind, abgesehen von der folgenden Ausnahme, keine Schaltjahre.
3. Die durch 400 ganzzahlig teilbaren Säkularjahre, zum Beispiel das Jahr 2000, sind jedoch Schaltjahre.
Quelle: https://de.wikipedia.org/wiki/Schaltjahr

EN:
Write a program that checks whether a year is a leap year
The Gregorian leap year rule consists of the following three individual rules:
1. The years that are divisible by 4 are leap years, except for the following exceptions.
2. Secular years, i.e. the years that end a century (e.g. 1800, 1900, 2100 and 2200),
   are not leap years, except for the following exception.
3. The secular years that are divisible by 400, for example the year 2000, are leap years.
Source: https://en.wikipedia.org/wiki/Leap_year
*/

use std::io;

fn main() {
    let input = read_userinput("Enter a year (yyyy): ");
    let year = match input.parse::<u32>() {
        Ok(year) => year,
        Err(_) => {
            println!("Invalid year");
            return;
        }
    };

    println!("{} is a leap year {}", year, is_leap_year(year));
}

fn is_leap_year(year: u32) -> bool {
    year % 4 == 0 && year % 100 == 0 && year % 400 == 0 || year % 4 == 0 && year % 100 != 0
}

fn read_userinput(prompt: &str) -> String {
    let stdin = io::stdin();

    print!("{}\n", prompt);
    let mut raw_input = String::new();
    match stdin.read_line(&mut raw_input) {
        Ok(_) => {
            raw_input = raw_input.trim().to_string();
    
            raw_input
        }
        Err(_) => String::new()
    }
}
