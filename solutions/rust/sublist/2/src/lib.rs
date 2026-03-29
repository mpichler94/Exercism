#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list == second_list {
        Comparison::Equal
    } else if second_list.includes(first_list) {
        Comparison::Sublist
    } else if first_list.includes(second_list) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}

trait Includes: PartialEq {
    fn includes(&self, other: Self) -> bool;
}

impl<T: PartialEq> Includes for &[T] {
    fn includes(&self, other: Self) -> bool {
        other.is_empty() || self.windows(other.len()).any(|w| w == other)
    }
}
