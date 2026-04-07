use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors
        .iter()
        .filter(|&f| *f > 0)
        .flat_map(|&f| multiples_of(f, limit))
        .collect::<HashSet<u32>>()
        .iter()
        .sum()
}

fn multiples_of(factor: u32, limit: u32) -> HashSet<u32> {
    (1..limit.div_ceil(factor))
        .map(|n| factor * n)
        .filter(|n| *n < limit)
        .collect::<HashSet<u32>>()
}
