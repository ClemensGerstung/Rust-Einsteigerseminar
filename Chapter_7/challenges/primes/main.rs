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

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }

    for a in 2..n {
        if n % a == 0 {
            return false;
        }
    }
    true
}

#[tokio::main]
async fn main() {
    // 1999999927, 2000000872, 2000000087, 2000000084

    let mut vector = Vec::new();
    for i in 0..5 {
        let input = user_input!(format!("Enter {} number: ", i + 1));
        vector.push(input);
    }

    let tasks = vector.iter().map(|i| i.to_owned()).map(|i| {
        tokio::spawn(async move {
            match i.parse::<u32>() {
                Ok(value) => {
                    let res = if is_prime(value) { "is" } else { "is not" };
                    println!("{} {} prime number", value, res);
                }
                Err(_) => println!("{} is not a number thus not a prime!", i),
            }
        })
    });

    for handle in tasks {
        handle.await.unwrap();
    }
}
