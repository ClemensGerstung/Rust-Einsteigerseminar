/*
DE: Lasse die Nutzer*in Die Farbe des Lieblingstiers,
    sowie ein Schiffsteil eingeben und gib dann den Piratennamen aus.
EN: Let the user enter the color of their favorite animal
    and a part of a ship and then output the pirate name.
*/

use std::io;

fn main() -> io::Result<()>{
    let stdin = io::stdin();
    let mut color = String::new();
    let mut part = String::new();

    println!("Ayeeee Mate, so you are the new Cap'n. How should we call ya?🏴‍☠️\n");
    print!("Your favorite animal's color? \n");
    stdin.read_line(&mut color)?;
    color = color.trim().to_string();

    print!("Your favorite part of your ship? \n");
    stdin.read_line(&mut part)?;
    part = part.trim().to_string();

    println!("Aye, Cap'n {} {}", color, part);

    Ok(())
}
