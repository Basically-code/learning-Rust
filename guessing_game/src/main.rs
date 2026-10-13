use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    //------Start----------------------------------------
    println!("Hello Welcome to the guessing game!!");

    let secret = rand::thread_rng().gen_range(1..=100);

    //----------------------------------------------------

    loop{
        println!("Please enter your guess: ");

        let mut guess = String::new(); //Creates a mutable(Changeable) empty string ""


        io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line!!");

        let guess: u32 = match guess.trim().parse() {
            Ok(chill) => chill,
            Err(_)  => continue, 
        };

        println!("You guessed: {guess}");
        match guess.cmp(&secret){
            Ordering::Less    => println!("You guessed less"),
            Ordering::Greater => println!("You guessed greater"),
            Ordering::Equal   => {
                println!("You guessed right, You win!!");
                break;
            }
        }
    }
    
}