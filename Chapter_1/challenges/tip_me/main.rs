/*
DE:     Schreiben Sie ein Programm welche die Rechnung eines Tisches gleichmäßig auf alle Gäste aufteilt.
        Geben Sie das Ergebnis auf zwei Nachkommastellen gerundet aus.
        Berechnen Sie dann die Höhe des Trinkgeldes und geben Sie dann
        - den Gesamtbetrag pro Gast
        - den Rechnungsanteil ohne Trinkgeld pro Gast
        - das Trinkgeld pro Gast
        - das Gesamt-Trinkgeld
        in einer print()-Anweisung aus.

EN:     Write a program that evenly distributes the bill of a table to all guests.
        Round the result to two decimal places.
        Then calculate the amount of the tip and then give
        - the total amount per guest
        - the bill share without tip per guest
        - the tip per guest
        - the total tip
        in a print() statement.
*/

use std::io; // Import the io library

fn main() -> io::Result<()> {
    let person_count = read_value("How many people?", 1);
    let bill = read_value("Total bill?", 1);
    let tip_percentage = read_value("Tip (in %)?", 1);

    let tip = (bill * tip_percentage) as f32 / 100.0;
    let total = bill as f32 + tip;
    let total_per_person = total / person_count as f32;

    println!("Bill: {:.2}, Tip: {:.2}, Total: {:.2}, Bill per Person: {:.2}, Tip per Person: {:.2}, Total per Person: {:.2}", 
                bill, tip, total, bill / person_count, tip / person_count as f32, total_per_person);

    Ok(())
}

fn read_value(msg: &str, def: i32) -> i32 {
    let stdin = io::stdin();

    print!("{}\n", msg);
    let mut raw_input = String::new();
    match stdin.read_line(&mut raw_input) {
        Ok(_) => {
            raw_input = raw_input.trim().to_string();
    
            match raw_input.parse::<i32>() {
                Ok(i) => i,
                Err(_) => def
            }
        }
        Err(_) => def
    }
}