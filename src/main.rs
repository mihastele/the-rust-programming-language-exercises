use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {

    let mut guesses_left = 10;
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        println!("Guess the number!");
    
        println!("Please input your guess.");
    
    
        let mut guess = String::new();
    
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("Bullshit\n");
                continue;
            },
        };

        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }

        guesses_left -= 1;
        if guesses_left <= 0 {
            println!("You lose!");
            println!("The number was {secret_number}");
            break;
        }

        print!("you have {guesses_left} guesses left\n")
    }
}