pub fn factors(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();
    let mut p = n;

    while p > 1 {
        for i in 2..=p {
            if !is_prime(i) {
                continue;
            }

            if p % i == 0 {
                p /= i;
                factors.push(i);
                break;
            }
        }
    }

    factors
}

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }

    for i in 2..=(n as f64).sqrt() as u64 {
        if n.is_multiple_of(i) {
            return false;
        }
    }

    true
}
