mod tests;
mod tools;

use std::env;
use tools::primes::*;

fn main() {

    let args: Vec<String> = env::args().collect();
    let candidate_str = &args[1];

    if is_multiple_of_3(candidate_str) {
        println!("{} is not a prime.", candidate_str);
    }
    else {
        let result = is_prime_loop(candidate_str);

        if result {
            println!("{} is a prime.", candidate_str);
        }
        else {
            println!("{} is not a prime.", candidate_str);
        }
    }

}