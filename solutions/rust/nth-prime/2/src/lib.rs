pub fn nth(n: u32) -> u32 {
    let mut count = 0;
    let mut p = 2;
    loop {
        if is_prime(p) {
            if count == n {
                return p;
            }
            count += 1;
        }
        p += 1;
    }
}

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }

    for i in 2..=(n as f64).sqrt() as u32 {
        if n.is_multiple_of(i) {
            return false;
        }
    }

    true
}