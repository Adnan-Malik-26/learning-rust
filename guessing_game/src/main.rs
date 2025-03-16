use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("**************Guess the Number**************");
    
    let secret_number = rand::rng().random_range(1..=100);
    loop{    
        println!("Input Your Guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
            };
        
        println!("You guessed: {guess}");
       
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You WIN!!");
                break;
            }
        }    
    }
}
