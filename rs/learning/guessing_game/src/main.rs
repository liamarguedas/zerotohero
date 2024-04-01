
use colored::*;
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    
    println!("Give me a number between 1 and 10: ");

    let random_number: i32 = rand::thread_rng().gen_range(1, 10);
    
    loop {

    let mut guess: String = String::new();
        
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read number");
   
    let guess: i32 = guess.trim().parse().expect("value is not a number");

    match guess.cmp(&random_number) {
        Ordering::Less => println!("{}","Number is bigger".red()),
        Ordering::Greater => println!("{}","Number is smaller".red()),
        Ordering::Equal => { println!("{}","You win".green()); break;},
}}
}
