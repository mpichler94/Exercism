pub fn is_armstrong_number(num: u32) -> bool {
    let num_digits = num.to_string().len() as u32;
    if num_digits < 2 {
        return true;
    }

    let sum = num.to_string().chars()
        .map(|c| c.to_digit(10).unwrap().pow(num_digits))
        .sum::<u32>();

    sum == num
}
