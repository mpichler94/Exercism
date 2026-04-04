// using Faulhaber's formula
pub fn square_of_sum(n: u32) -> u32 {
    let sum = n * (n + 1) / 2;
    sum.pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    let f = n as f64;
    ((f.powf(3.0) + 1.5 * f.powf(2.0) + 0.5 * f) / 3.0).round() as u32
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
