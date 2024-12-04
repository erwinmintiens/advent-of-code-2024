advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut result: u32 = 0;
    let input_vec: Vec<&str> = input.split_terminator("\n").collect();
    let mut positions = Vec::new();
    for (y, s) in input_vec.iter().enumerate() {
        for (x, v) in s.chars().enumerate() {
            if v == 'X' {
                positions.push((x, y));
            }
        }
    }
    for (x, y) in positions {
        result += check_x(x, y, &input_vec) as u32;
    }

    Some(result)
}

fn check_x(x: usize, y: usize, vec: &Vec<&str>) -> u8 {
    let mut result: u8 = 0;
    if vec[y].chars().nth(x).unwrap() != 'X' {
        eprintln!("The given coordinates do not point to an X");
        eprintln!("{}", vec[x].chars().nth(y).unwrap());
    }
    // Horizontal right
    if x + 3 < vec[y].len() && vec[y][x..=x + 3] == *"XMAS" {
        result += 1;
    }
    // Horizontal left
    if !(0..3).contains(&x) && vec[y][x - 3..=x] == *"SAMX" {
        result += 1;
    }
    // Vertical down
    if y + 4 <= vec.len()
        && &(y..y + 4)
            .map(|i| vec[i].chars().nth(x).unwrap())
            .collect::<String>()
            == "XMAS"
    {
        result += 1;
    }
    // Vertical up
    if !(0..3).contains(&y)
        && &(y - 3..=y)
            .map(|i| vec[i].chars().nth(x).unwrap())
            .collect::<String>()
            == "SAMX"
    {
        result += 1;
    }
    // Left upper
    if !(0..3).contains(&y) && !(0..3).contains(&x) {
        let max_y = y - 3;
        let mut string = String::new();
        let mut y2 = y;
        let mut x2 = x;
        while y2 >= max_y {
            string.push(vec[y2].chars().nth(x2).unwrap());
            if y2 == 0 || x2 == 0 {
                break;
            }
            y2 -= 1;
            x2 -= 1;
        }
        if &string == "XMAS" {
            result += 1;
        }
    }
    // Right upper
    if !(0..3).contains(&y) && x + 3 < vec[0].len() {
        let max_y = y - 3;
        let mut string = String::new();
        let mut y2 = y;
        let mut x2 = x;
        while y2 >= max_y {
            string.push(vec[y2].chars().nth(x2).unwrap());
            if y2 == 0 || x2 == vec[0].len() {
                break;
            }
            y2 -= 1;
            x2 += 1;
        }
        if &string == "XMAS" {
            result += 1;
        }
    }
    // Left lower
    if y + 3 < vec.len() && !(0..3).contains(&x) {
        let max_y = y + 3;
        let mut string = String::new();
        let mut y2 = y;
        let mut x2 = x;
        while y2 <= max_y {
            string.push(vec[y2].chars().nth(x2).unwrap());
            if y2 == vec.len() || x2 == 0 {
                break;
            }
            y2 += 1;
            x2 -= 1;
        }
        if &string == "XMAS" {
            result += 1;
        }
    }
    // Right lower
    if y + 3 < vec.len() && x + 3 < vec[0].len() {
        let max_y = y + 3;
        let mut string = String::new();
        let mut y2 = y;
        let mut x2 = x;
        while y2 <= max_y {
            string.push(vec[y2].chars().nth(x2).unwrap());
            if y2 == vec.len() || x2 == vec[0].len() {
                break;
            }
            y2 += 1;
            x2 += 1;
        }
        if &string == "XMAS" {
            result += 1;
        }
    }
    result
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
