const CHILDREN: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let offset = CHILDREN.iter().position(|c| *c == student).unwrap() * 2;
    diagram
        .lines()
        .flat_map(|line| line[offset..offset + 2].chars())
        .map(|c| match c {
            'G' => "grass",
            'C' => "clover",
            'R' => "radishes",
            'V' => "violets",
            _ => panic!(),
        })
        .collect()
}
