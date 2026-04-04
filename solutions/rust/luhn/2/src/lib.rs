/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut sum = 0;
    let mut count = 0;
    for c in code.chars().rev() {
        if c.is_whitespace() {
            continue;
        }

        let num = c.to_digit(10)
            .map(|num| if count % 2 == 0 { num } else { num * 2 })
            .map(|num| if num > 9 { num - 9 } else { num });

        if num.is_none() {
            return false;
        }

        sum += num.unwrap();
        count += 1;
    }

    sum.is_multiple_of(10) && count > 1
}
