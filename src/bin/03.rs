advent_of_code::solution!(3);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    Some(get_result_of_string(input))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result: u32 = 0;
    let vec: Vec<&str> = input.split("don't()").collect();

    for (i, item) in vec.iter().enumerate() {
        println!("{}, {}", i, item);
        if i == 0 {
            result += get_result_of_string(item);
        }
        let s: Vec<&str> = item.split("do()").collect();
        if s.is_empty() {
            continue;
        }
        for (index, value) in s.iter().enumerate() {
            if index == 0 {
                continue;
            } else {
                result += get_result_of_string(value);
            }
        }
    }
    Some(result)
}

fn get_result_of_string(input: &str) -> u32 {
    let mut result: u32 = 0;
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    for cap in re.captures_iter(input) {
        let first_number_str = cap.get(1).map_or("", |m| m.as_str());
        let second_number_str = cap.get(2).map_or("", |m| m.as_str());
        if let (Ok(first_number), Ok(second_number)) = (
            first_number_str.parse::<u32>(),
            second_number_str.parse::<u32>(),
        ) {
            result += first_number * second_number;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
