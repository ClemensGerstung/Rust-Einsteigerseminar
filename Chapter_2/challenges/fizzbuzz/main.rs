/*
DE:
    Schreibe eine App welche das Gruppenwortspiel Fizz Buzz, bis zur Zahl 100 durchspielt:
    https://de.wikipedia.org/wiki/Fizz_buzz// Die App gibt Fizz aus, wenn die Zahl durch drei teilbar ist oder Buzz, wenn sie durch fünf teilbar ist.
    Ist die Zahl zur fünf und drei teilbar, gibt die App Fizz Buzz aus. Ist die Zahl weder durch fünf, noch durch
    drei teilbar, gibt die App die Zahl selbst aus.
    Gib zuletzt aus, wie viel Fizz, Buzz und Fizz Buzz gefunden wurden

EN:
    Write an app that plays the group word game Fizz Buzz up to the number 100:
    https://en.wikipedia.org/wiki/Fizz_buzz
    The app outputs Fizz if the number is divisible by three or Buzz if it is divisible by five.
    If the number is divisible by five and three, the app outputs Fizz Buzz. If the number is neither divisible by five nor by
    three, the app outputs the number itself.
    Finally, output how much Fizz, Buzz and Fizz Buzz were found
*/

fn main() {
    fizz_buzz()
}

fn fizz_buzz() {
    for index in 1..=100 {
        let out = if index % 15 == 0 {
            "fizz buzz".to_string()
        } else if index % 5 == 0 {
            "buzz".to_string()
        } else if index % 3 == 0 {
            "fizz".to_string()
        } else {
            index.to_string()
        };

        print!("{}, ", out);
    }
    println!("");
}
