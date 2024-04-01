// Enter a number and have the program generate
// PI up to that many decimal places.
// Keep a limit to how far the program will go.
//

use rug::{Float};
use rug::ops::Pow;
use std::ops::Div;

fn find_pi(n_decimals: u32) {

    let mut sum: Float = Float::with_val(n_decimals, 0.0);
    let max_elements: u32 = 10;

    for n in 0..max_elements {
        let first_fraction_num: Float = Float::with_val(n_decimals, Float::factorial(4 * n));
        let first_fraction_denom: Float = Float::with_val(n_decimals, Float::with_val(n_decimals, 4).pow(n) * Float::with_val(n_decimals, Float::factorial(n))).pow(4);
        
    }

}
fn main () {

    find_pi(5);

}
