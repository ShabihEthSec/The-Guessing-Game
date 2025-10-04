use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("The Guessing Game");

    let secret_number = rand::rng().random_range(1..=100);

    println!("Guess The Number...");
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Unable to read");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match &guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Low!"),
            Ordering:: Greater => println!("Too High!"),
            Ordering:: Equal => {
                println!("You Win!");
                break;
            }
        }
    }
    
}