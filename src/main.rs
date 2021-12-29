use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..10);

    loop {
        println!("Please input your guess.");
    
        // new is an associated function of the String type.
        let mut guess = String::new();
    
        // & indicates that this argument is a reference
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        // shadow the previous value of guess 
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
    
        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        println!("Secret number is {}", secret_number);
    }
}
