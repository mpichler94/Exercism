use convert_case::{Case, Casing};

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let numbers = ["no", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten"];
    let mut result = String::new();
    for i in 0..take_down {
        if !result.is_empty() {
            result += "\n";
        }
        result += &get_verse(&numbers, start_bottles - i)
    }
    result
}

fn get_verse(numbers: &[&str], num_bottles: u32) -> String {
    let mut bottles = if num_bottles == 1 { "bottle" } else { "bottles" };
    let mut str = format!("{} green {} hanging on the wall,\n", numbers[num_bottles as usize].to_case(Case::Title), bottles);
    str += &str.clone();
    str += "And if one green bottle should accidentally fall,\n";
    bottles = if num_bottles - 1 == 1 { "bottle" } else { "bottles" };
    str += &format!("There'll be {} green {} hanging on the wall.\n", numbers[(num_bottles - 1) as usize], bottles);
    str
}