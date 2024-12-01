advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let list = input.split("\n");
    let mut vec1: Vec<u32> = Vec::new();
    let mut vec2: Vec<u32> = Vec::new();
    for record in list {
        let mut parts = record.split_whitespace();
        let part1 = parts.next();
        let part2 = parts.next();
        let part1 = part1.unwrap().parse::<u32>().unwrap();
        let part2 = part2.unwrap().parse::<u32>().unwrap();
        vec1.push(part1);
        vec2.push(part2);
    }
    vec1.sort();
    vec2.sort();

    let mut result: u32 = 0;
    for (index, value) in vec1.iter().enumerate() {
        let difference: i32 = (*value as i32) - (vec2[index] as i32);
        result += difference.abs() as u32;
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let list = input.split("\n");
    let mut vec1: Vec<u32> = Vec::new();
    let mut vec2: Vec<u32> = Vec::new();
    for record in list {
        let mut parts = record.split_whitespace();
        let part1 = parts.next();
        let part2 = parts.next();
        vec1.push(part1.unwrap().parse::<u32>().unwrap());
        vec2.push(part2.unwrap().parse::<u32>().unwrap());
    }
    let mut result: u32 = 0;
    for value in vec1 {
        let count = vec2.iter().filter(|x| **x == value).count() as u32;
        result += value * count;
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
