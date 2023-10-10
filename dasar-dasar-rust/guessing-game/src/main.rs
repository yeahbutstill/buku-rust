use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    println!("The secret_number: {}", secret_number);
    println!("Please input your guess: ");
    
    //let guess_defualt_immutable = String::new(); // immutable
    let mut guess_mutable = String::new(); // mutable
    io::stdin()
        .read_line(&mut guess_mutable)
        .expect("Failed to readline");
    
    let guess_mutable: u32 = guess_mutable.trim().parse().expect("Please type using number");
    println!("You guessed: {}", guess_mutable);

    match guess_mutable.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("To big"),
        Ordering::Equal => println!("You win"),
    }
}
