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
    } else if is_sublist(first_list, second_list) {
        Comparison::Sublist
    } else if is_sublist(second_list, first_list) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}

fn is_sublist(sublist: &[i32], mainlist: &[i32]) -> bool {
    let mut j;
    for offset in 0 .. mainlist.len() {
        j = 0;
        for n in mainlist.iter().skip(offset) {
            if j == sublist.len() {
                return true;
            }
            if *n == sublist[j] {
                j += 1;
            } else {
                break;
            }
        }
        if j == sublist.len() {
            return true;
        }
    }

    false
}
