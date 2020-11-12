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
    guess_game()
}

fn guess_game() {
    println!("Enter your guess!");
    let secret_number = rand::thread_rng().gen_range(10, 21);
    loop {
        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = guess.trim().parse().expect("Number should be a string");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your value is lesser"),
            Ordering::Greater => println!("Your value is greater"),
            Ordering::Equal => {
                println!("Congratulations");
                break;
            },
        }
    }
}
