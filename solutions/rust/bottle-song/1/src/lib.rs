use std::collections::HashMap;
use convert_case::{Case, Casing};

pub fn recite(start_bottles: u32, take_down: u32) -> String {

    let mut numbers = HashMap::new();
    numbers.insert(0, "no");
    numbers.insert(1, "one");
    numbers.insert(2, "two");
    numbers.insert(3, "three");
    numbers.insert(4, "four");
    numbers.insert(5, "five");
    numbers.insert(6, "six");
    numbers.insert(7, "seven");
    numbers.insert(8, "eight");
    numbers.insert(9, "nine");
    numbers.insert(10, "ten");

    let mut result = String::new();
    for bottles in (start_bottles-take_down+1..=start_bottles).rev() {
        if !result.is_empty() {
            result += "\n";
        }
        result += &get_verse(&numbers, bottles as i32)
    }
    result
}

fn get_verse(numbers: &HashMap<i32, &str>, bottles: i32) -> String {
    let mut bottle = if bottles == 1 { "bottle" } else { "bottles" };
    let mut str = format!("{} green {} hanging on the wall,\n", numbers[&bottles].to_case(Case::Title), bottle);
    str += &str.clone();
    str += "And if one green bottle should accidentally fall,\n";
    bottle = if bottles - 1 == 1 { "bottle" } else { "bottles" };
    str += &format!("There'll be {} green {} hanging on the wall.\n", numbers[&(bottles - 1)], bottle);

    str
}