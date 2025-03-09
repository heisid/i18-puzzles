use std::fs;

fn is_valid_password(password: &str) -> bool {
    let mut char_count = 0;
    let mut digits_count = 0;
    let mut upper_count = 0;
    let mut lower_count = 0;
    let mut non_ascii_count = 0;
    password.chars().for_each(|c| {
        char_count += 1;
        if c.is_ascii_digit() { digits_count += 1; }
        if c.is_uppercase() { upper_count += 1; }
        if c.is_lowercase() { lower_count += 1; }
        if !c.is_ascii() { non_ascii_count += 1; }
    });

    char_count >= 4 && char_count <= 12 &&
        digits_count > 0 &&
        upper_count > 0 &&
        lower_count > 0 &&
        non_ascii_count > 0
}

pub fn count_valid_passwords() {
    let file_path = "input3.txt";
    let mut count: u32 = 0;
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    contents.lines().for_each(|line| {
        if is_valid_password(line) { count += 1; }
    });
    println!("Result Puzzle 3: {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_password() {
        assert_eq!(is_valid_password("d9Ō"), false);
        assert_eq!(is_valid_password("uwI.E9GvrnWļbzO"), false);
        assert_eq!(is_valid_password("ž-2á"), false);
        assert_eq!(is_valid_password("Ģ952W*F4"), false);
        assert_eq!(is_valid_password("?O6JQf"), false);
        assert_eq!(is_valid_password("xi~Rťfsa"), false);
        assert_eq!(is_valid_password("r_j4XcHŔB"), true);
        assert_eq!(is_valid_password("71äĜ3"), true);
    }
}