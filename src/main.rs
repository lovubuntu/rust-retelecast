use rand::Rng;

use std::cmp::Ordering;
//use ferris_says::say;
//use std::io::{BufWriter, stdout};
use std::io::stdin;

fn main() {
    //    let stdout = stdout();
    //    let message = String::from("Hello from Prabu");
    //    let width = message.chars().count();
    //    let mut writer = BufWriter::new(stdout.lock());
    //    say(message.as_bytes(), width, &mut writer).unwrap();
    retelecast::eat_at_restaurant();
    while_loop();
    let result = increment_one(5);
    println!("Result is {}", result);
    tup();
    guess_game();
    shadow_variable();
    shadow_variable_usage();
}

fn guess_game() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(10, 21);

    loop {
        let mut guess = String::new();
        println!("Enter your guess!");
        stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => (num),
            Err(_) => {
                println!("This is not a valid number");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your value is lesser"),
            Ordering::Greater => println!("Your value is greater"),
            Ordering::Equal => {
                println!("Congratulations");
                break;
            }
        }
    }
}

fn shadow_variable() {
    let x = 5;
    let x = x * 5;
    let x = x + 5;
    println!("{}", x);
    let x = 12;
    println!("{}", x);
}

fn shadow_variable_usage() {
    let spaces = "     ";
    let spaces = spaces.len();
    println!("Length of spaces is {}", spaces);
}

fn tup() {
    let tuple = (10, 12.3, 45);
    let (x, _, y) = tuple;
    println!(
        "{} and {} are from tuples, and using dot operator {}",
        x, y, tuple.1
    );
    let floater = tuple.1.to_string();
    logger("some values are ", &floater);
    println!("After change value is {}", floater);
}

fn logger(message: &str, value: &str) {
    println!("{}: {}", message, value);
    let value = String::from("Hello");
    println!("String value is {}", value);
}

fn increment_one(x: i32) -> i32 {
    println!("Incrementing the value {} by one", x);
    x + 1
}

fn while_loop() {
    let mut times = 10;
    while times > 0 {
        println!("{}!", times);
        times -= 1;
    }
    println!("LiftOff!");
}
