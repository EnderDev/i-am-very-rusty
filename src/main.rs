use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("\nwelcome to the shitty number guessing game thing\n\nive generated a random number guess it bro i bet u cant\n");

    let num = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("make a guess!");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let guess: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if(guess < num) {
            println!("\ntoo small idiot\n")
        } else if(guess > num) {
            println!("\ntoo large idiot\n")
        } else if(guess == num) {
            println!("\nwow u did it\n");
            break;
        }
    }
}