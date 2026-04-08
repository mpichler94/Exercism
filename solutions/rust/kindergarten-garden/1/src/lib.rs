pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let students = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ];
    let offset = students.iter().position(|s| *s == student).unwrap() * 2;

    diagram
        .lines()
        .flat_map(|line| line[offset..offset + 2].chars())
        .map(|c| get_flower(c))
        .collect()
}

fn get_flower(c: char) -> &'static str {
    match c {
        'G' => "grass",
        'C' => "clover",
        'R' => "radishes",
        'V' => "violets",
        _ => panic!(),
    }
}
