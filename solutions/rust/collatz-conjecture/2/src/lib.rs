pub fn collatz(mut n: u64) -> Option<u64> {
    for count in 0.. {
        match n {
            0 => break,
            1 => return Some(count),
            even if even.is_multiple_of(2) => n /= 2,
            _ => n = n.strict_mul(3).strict_add(1),
        }
    }

    None
}
