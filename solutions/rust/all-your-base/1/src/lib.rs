#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }
    let invalid = number.iter().find(|d| **d >= from_base);
    if invalid.is_some() {
        return Err(Error::InvalidDigit(*invalid.unwrap()));
    }

    let mut result = number
        .iter()
        .enumerate()
        .map(|(i, d)| *d as u64 * (from_base.pow(number.len() as u32 - 1 - i as u32)) as u64)
        .reduce(|a, b| a + b)
        .unwrap_or(0);

    let mut digits = Vec::new();

    while result > 0 {
        let next = result / to_base as u64;
        digits.push((result % to_base as u64) as u32);
        result = next;
    }
    if digits.is_empty() {
        digits.push(0)
    }

    digits.reverse();
    Ok(digits)
}
