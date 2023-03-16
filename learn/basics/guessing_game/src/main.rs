use std::{io, cmp::Ordering};
use rand::Rng;
use colored::{self, Colorize};

fn main() {
    println!("Welcome to the guessing game!");

    let secret_num = rand::thread_rng().gen_range(1..101);
    println!("Shh! Secret number is: {}", secret_num);

    println!("Guess a number:");

    loop{
        let mut guess = String::new();

        match io::stdin().read_line(&mut guess){
            Ok(n) => n,
            Err(_) => {
                println!("Failed to read line!");
                break;
            }
        };

        println!("You guessed: {}", guess);

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Please enter a valid integer.".red());
                continue;
            }
        };

        match guess.cmp(&secret_num){
            Ordering::Greater => println!("{}", "Too big! Guess again:".truecolor(240,190,140)),
            Ordering::Less => println!("{}", "Too small! Guess again:".truecolor(240,190,140)),
            Ordering::Equal => {
                println!("{}", "You guessed correctly!".bright_green().bold());
                break;
            },
        };
    }

}
