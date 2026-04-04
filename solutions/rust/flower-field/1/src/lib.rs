pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() {
        return vec![];
    }

    let field: Vec<&[u8]> = garden.iter().map(|row| row.as_bytes()).collect();
    let width = field[0].len();
    let height = field.len();
    let mut new_garden = Vec::with_capacity(height);
    for y in 0..height {
        new_garden.push(Vec::with_capacity(width));
        for x in 0..width {
            if field[y][x] == '*' as u8 {
                new_garden[y].push('*' as u8);
                continue;
            }
            let count = neighbors(&field, x, y).iter().filter(|c| **c == '*' as u8).count() as u32;
            if count == 0 {
                new_garden[y].push(' ' as u8);
                continue;
            }
            new_garden[y].push(char::from_digit(count, 10).unwrap() as u8);
        }
    }

    new_garden.iter().map(|row| String::from_utf8(row.to_vec()).unwrap()).collect()
}

fn neighbors(garden: &Vec<&[u8]>, x: usize, y: usize) -> Vec<u8> {
    let mut neighbors = Vec::new();
    if x > 0 {
        if y > 0 {
            neighbors.push(garden[y-1][x-1]);
        }
        neighbors.push(garden[y][x-1]);
        if y < garden.len()-1 {
            neighbors.push(garden[y+1][x-1]);
        }
    }

    if y > 0 {
        neighbors.push(garden[y-1][x]);
    }
    neighbors.push(garden[y][x]);
    if y < garden.len()-1 {
        neighbors.push(garden[y+1][x]);
    }

    if x < garden[y].len()-1 {
        if y > 0 {
            neighbors.push(garden[y-1][x+1]);
        }
        neighbors.push(garden[y][x+1]);
        if y < garden.len()-1 {
            neighbors.push(garden[y+1][x+1]);
        }
    }

    neighbors
}