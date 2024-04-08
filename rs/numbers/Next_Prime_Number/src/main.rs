// Next Prime Number - Have the program find prime numbers until the user chooses to stop asking for the next one.

use std::io::stdin;
  
fn main() {

    let mut n = String::new();  
       
    stdin().read_line(&mut n)  
        .ok()  
        .expect("Error reading");  
    
    let number: i32 = n
	.trim()
    .parse()
    .expect("This string cannot be converted to a number");


} 
 
fn generate_next_prime(n: i32, last_prime: i32 ) -> i32{

    let mut return_value: i32 = 0;

    for number in last_prime..n {
        
        if number % n == 0 {
    
            println!("{}", number);

            return_value = number;
            break;
        }
    }
   return_value
}  
 
fn generate_next() -> bool { 
   
    println!("Print next? (Y/N)");  
       
    let mut input = String::new();  
       
    stdin().read_line(&mut input)  
        .ok()  
        .expect("Error reading");  
 
    input.to_lowercase() == "y"     
  
}  
