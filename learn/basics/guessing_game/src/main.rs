use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    println!("Welcome to the guessing game!");

    let secret_num = rand::thread_rng().gen_range(1..101);
    println!("Shh! Secret number is: {}", secret_num);

    let mut guess = String::new();
    println!("Guess a number:");

    io::stdin().read_line(&mut guess).expect(
        "Failed to read line!");
    println!("You guessed: {}", guess);

    let guess: i32 = guess.trim().parse().expect(
        "Please enter a valid integer.");

    match guess.cmp(&secret_num){
        Ordering::Equal => println!("You guessed correctly!"),
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
    };

}
