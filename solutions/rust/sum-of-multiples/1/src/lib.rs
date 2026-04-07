use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut result = HashSet::new();

    for factor in factors {
        result.extend(multiples_of(*factor, limit));
    }

    result.iter().sum()
}

fn multiples_of(factor: u32, limit: u32) -> HashSet<u32> {
    let mut set = HashSet::new();
    for i in 1..limit {
        let number = factor * i;
        if number >= limit {
            break;
        }
        set.insert(number);
    }

    set
}
