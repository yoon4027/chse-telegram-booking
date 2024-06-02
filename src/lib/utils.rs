use regex::Regex;

pub fn is_valid_number(number: &str) -> bool {
    let re = Regex::new(r"^(7|9)\d{6}$").unwrap();
    re.is_match(number)
}
