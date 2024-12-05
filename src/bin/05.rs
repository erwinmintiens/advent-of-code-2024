advent_of_code::solution!(5);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let input_vec: Vec<&str> = input.split_terminator("\n").collect();
    let (ordering_rules, updates) = get_ordering_rules_and_updates(&input_vec);
    Some(check_updates(&updates, &ordering_rules))
}

fn get_ordering_rules_and_updates(vec: &Vec<&str>) -> (Vec<(u8, u8)>, Vec<Vec<u8>>) {
    let mut ordering_rules = Vec::new();
    let mut updates = Vec::new();
    let mut fetching_rules = true;
    for record in vec {
        if record.is_empty() {
            fetching_rules = false;
            continue;
        }
        if fetching_rules {
            let mut iter = record.split("|");
            ordering_rules.push((
                iter.next().unwrap().parse::<u8>().unwrap(),
                iter.next().unwrap().parse::<u8>().unwrap(),
            ));
        } else {
            updates.push(
                record
                    .split(",")
                    .map(|i| i.parse::<u8>().unwrap())
                    .collect(),
            );
        }
    }
    (ordering_rules, updates)
}

fn check_updates(updates: &Vec<Vec<u8>>, ordering_rules: &Vec<(u8, u8)>) -> u32 {
    let mut result: u32 = 0;
    for update in updates {
        let mut processed_pages: Vec<u8> = Vec::new();
        let mut valid_update = true;
        for page_number in update {
            // If second number matches page number AND processed_pages does not contain first
            // number AND the update contains the first number
            if ordering_rules.iter().any(|&(first, second)| {
                second == *page_number
                    && (!processed_pages.contains(&first) && update.contains(&first))
            }) {
                valid_update = false;
                break;
            } else {
                processed_pages.push(*page_number);
            }
        }
        if valid_update {
            result += get_middle_page(update) as u32;
        }
    }
    result
}

fn get_middle_page(update_record: &Vec<u8>) -> u8 {
    let index = (update_record.len() as f32 / 2.0).floor();
    return update_record[index as usize];
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
