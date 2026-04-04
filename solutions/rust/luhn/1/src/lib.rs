/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let new_code = code.replace(' ', "");
    if new_code.len() < 2 {
        return false;
    }

    if new_code.bytes().any(|c| !c.is_ascii_digit()) {
        return false;
    }

    let sum: u32 = (0..new_code.len())
        .map(|i| convert(&new_code, i))
        .sum();

    sum % 10 == 0
}

fn convert(code: &String, index: usize) -> u32 {
    let digit = code.chars().nth(index).unwrap().to_digit(10).unwrap();

    if code.len() % 2 == index % 2 {
        if digit > 4 {
            digit * 2 - 9
        } else {
            digit * 2
        }
    } else {
        digit
    }
}