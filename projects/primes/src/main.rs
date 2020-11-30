mod tests;
mod tools;

use std::env;
use tools::primes::*;
use tools::validation::validate;

fn main() {

    let args: Vec<String> = env::args().collect();
    let candidate_str = &args[1];

    validate(candidate_str);
    check_prime(candidate_str);

}

fn check_prime(candidate: &str) {

    if is_multiple_of_3(candidate) {
        println!("{} is not a prime - the digits add up to a multiple of 3.", candidate);
    }
    else {

        let result = prime_loop(candidate);

        if result {
            println!("{} is a prime.", candidate);
        }
        else {
            println!("{} is not a prime.", candidate);
        }
    }
}