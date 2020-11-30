use crate::tools::primes::*;

#[test]
fn prime_1_is_not_prime() {

    let result = is_prime_loop("1");
    assert!(!result);

}

#[test]
fn prime_2_is_prime() {

    let result = is_prime_loop("2");
    assert!(result);

}

#[test]
fn prime_3_is_prime() {

    let result = is_prime_loop("3");
    assert!(result);

}

#[test]
fn prime_4_is_not_prime() {

    let result = is_prime_loop("4");
    assert!(!result);

}

#[test]
fn char_string_digits_multiple_of_3() {

    let result = is_multiple_of_3("123");
    assert!(result);

}

#[test]
fn char_string_digits_not_multiple_of_3() {

    let result = is_multiple_of_3("1235");
    assert!(!result);

}