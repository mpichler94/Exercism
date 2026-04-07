use std::iter::once;

pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }
    
    list
        .windows(2)
        .map(|pairs| format!("For want of a {} the {} was lost.\n", pairs[0], pairs[1]))
        .chain(once(format!("And all for the want of a {}.", list[0])))
        .fold(String::new(), |acc, line| acc + line.as_str())
}
