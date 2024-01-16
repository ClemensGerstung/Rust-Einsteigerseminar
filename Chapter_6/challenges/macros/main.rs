use std::io;

#[macro_export]
macro_rules! user_input {
    ($msg:expr) => {{
        print!("{}\n", $msg);
        let stdio = io::stdin();

        let mut raw_input = String::new();
        match stdio.read_line(&mut raw_input) {
            Ok(_) => raw_input.trim().to_string(),
            Err(_) => String::new(),
        }
    }};
}

fn main() {
    let msg = user_input!("Please enter your CC number: ");
    println!("{}", msg);
}
