use std::process;

pub fn validate(input: &str) {
    if !is_integer(input) {
        println!("Bad input: {} is not a whole number greater than 0.", input);
        process::exit(1);
    }
}

fn is_integer(input: &str) -> bool {
    match input.parse::<u64>() {
        Ok(..) => true,
        Err(..) => false,
    }
}

#[test]
fn good_input_1_to_1000_returns_true() {

    for n in 1..1000 {
        let n_str = n.to_string();
        assert!(is_integer(&n_str));
    }

}

#[test]
fn bad_input_returns_false() {

    assert!(!is_integer("123.45"));
    assert!(!is_integer("-1"));
    assert!(!is_integer("abcdefg"));
    assert!(!is_integer("!"));

}