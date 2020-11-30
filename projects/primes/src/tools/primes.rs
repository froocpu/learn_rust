pub fn is_prime_loop(candidate: &str) -> bool {

    if candidate == "1" {
        return false;
    }

    let mut flag = 0;
    let upper_limit = candidate.parse::<u64>().unwrap();

    for n in 2..upper_limit {
        if upper_limit % n == 0 {
            flag += 1;
            break;
        }
    }

    return flag == 0;

}

pub fn is_multiple_of_3(candidate: &str) -> bool {

    let mut sum = 0;
    for y in candidate.chars() {
        let z = (y.to_string()).parse::<i32>().unwrap();
        sum += z;
    }

    return sum % 3 == 0;
}