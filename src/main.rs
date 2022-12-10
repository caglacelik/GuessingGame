use std::{io, cmp::Ordering};
use rand::Rng;
use colored::*;

extern crate colored;

pub struct Guess {
    val: i32,
}

impl Guess {
    pub fn new(val: i32) -> Guess {
        if val < 1 || val > 100 {
            panic!("Your guess value must be between 1 & 100 but got {}", val);
        }
        Guess {val}
    }
    
    pub fn val(&self) -> i32 {
        self.val
    }
}

fn main() {
    println!("WELCOME TO THE GAME!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop { 

        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read number");

        println!("Your guess: {guess}");

        let guess: i32 = Guess::new(guess.trim().parse().unwrap()).val;

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("{}", "Too small!".red()),
                Ordering::Greater => println!("{}", "Too big!".red()),
                Ordering::Equal => {
                    println!("{}", "You win!".green().bold().italic());
                    break;
                }       

            }
    }

}
