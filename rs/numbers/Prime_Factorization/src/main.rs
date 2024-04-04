// Prime Factorization - Have the user enter a number and find all Prime Factors (if there are any) and display them.

use std::io::stdin;

fn main() {
    
    let mut input: String = String::new();

    stdin().read_line(&mut input)
        .ok()
        .expect("Failed to read");

    let number: i64 = input.trim().parse().expect("Error converting to INT");

    get_prime_factors(number);
        
}

fn get_prime_factors(n: i64) {
    for number in 2..n {
        if n % number == 0 {
                println!("{}", number);
        }

    }
}
