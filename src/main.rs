use std::io::{self};
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {} ", secret_number);

    loop {

        println!("Please input your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess:u32 =match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        print!("You guessed: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("{}", " Too small!".truecolor(255, 255, 0)),
            Ordering::Greater => println!("{}", " Too big!".truecolor(255, 0, 0)),
            Ordering::Equal => {
                println!("{}", " You win!".truecolor(0, 255, 0));
                break;
            }
        }
    }
}