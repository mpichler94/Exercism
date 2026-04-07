pub fn build_proverb(list: &[&str]) -> String {
    let sentences = list
        .windows(2)
        .map(|pairs| format!("For want of a {} the {} was lost.\n", pairs[0], pairs[1]))
        .fold(String::new(), |acc, line| acc + line.as_str());

    if !list.is_empty() {
        sentences + &format!("And all for the want of a {}.", list[0])
    } else {
        sentences
    }
}
