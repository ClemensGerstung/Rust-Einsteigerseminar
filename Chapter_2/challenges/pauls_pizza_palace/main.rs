use std::io;
/*
DE:
    Schreibe ein Programm welches, den Preis der bestellten Pizza ermittelt
    Kosten:
    Pizza S = 3,50 €
    Pizza m = 5,50 €
    Pizza L = 7,00 €
    Pepperoni auf Pizza S = 2 €
    Pepperoni auf Pizza M, L = 3 €
    Extra Käse auf allen Pizzen = 1 €

EN:
    Write a program that determines the price of the ordered pizza
    Costs:
    Pizza S = 3,50 €
    Pizza m = 5,50 €
    Pizza L = 7,00 €
    Pepperoni on Pizza S = 2 €
    Pepperoni on Pizza M, L = 3 €
    Extra cheese on all pizzas = 1 €
*/

const PIZZA_PRICE_S: f32 = 3.5;
const PIZZA_PRICE_M: f32 = 5.5;
const PIZZA_PRICE_L: f32 = 7.0;
const PEPPERONI_PRICE_S: f32 = 2.0;
const PEPPERONI_PRICE_M: f32 = 3.0;
const PEPPERONI_PRICE_L: f32 = 3.0;
const CHEESE_PRICE: f32 = 1.0;
const SIZE_S: &str = "s";
const SIZE_M: &str = "m";
const SIZE_L: &str = "l";

fn main() {
    println!("Welcome to Paul's Pizza Palace!");
    let size = read_input("Which size do you want? (S/M/L):");
    let peperoni = read_input("Do you want extra Peperoni? (yes|no): ");
    let cheese = read_input("Do you want extra cheese? (yes|no): ");

    if !vec![SIZE_S, SIZE_M, SIZE_L].contains(&size.to_lowercase().as_str()) {
        println!("unknown size, rtfm!");
    }

    let price = calculate_price(
        size.to_string(), // copy
        peperoni.to_lowercase().eq("yes"),
        cheese.to_lowercase().eq("yes"),
    );

    println!(
        "your pizza (size: {}, pepperoni: {}, extra cheese: {}) costs {}",
        size, peperoni, cheese, price
    )
}

fn calculate_price(size: String, pepperoni: bool, cheese: bool) -> f32 {
    let base_price = match size.as_str() {
        SIZE_S => PIZZA_PRICE_S,
        SIZE_M => PIZZA_PRICE_M,
        SIZE_L => PIZZA_PRICE_L,
        _ => 0.0
    };

    let pepperoni_extra = match (size.as_str(), pepperoni) {
        (SIZE_S, true) => PEPPERONI_PRICE_S,
        (SIZE_M, true) => PEPPERONI_PRICE_M,
        (SIZE_L, true) => PEPPERONI_PRICE_L,
        (_, _) => 0.0,
    };

    let cheese_extra = if cheese { CHEESE_PRICE } else { 0.0 };

    return base_price + pepperoni_extra + cheese_extra;
}

fn read_input(prompt: &str) -> String {
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
