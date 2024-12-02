use std::collections::HashSet;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let list = input.split("\n");
    let mut result: u32 = 0;
    for record in list {
        let values_list: Vec<u8> = record
            .split_whitespace()
            .filter_map(|s| s.parse::<u8>().ok())
            .collect();
        let (is_ascending, is_descending) = check_order(&values_list);
        if (!is_ascending && !is_descending) || values_list.is_empty() {
            continue;
        }
        if check_gap(is_descending, &values_list) {
            result += 1;
        }
    }
    Some(result)
}

fn check_order(vec: &Vec<u8>) -> (bool, bool) {
    let mut is_ascending = true;
    let mut is_descending = true;
    for i in 1..vec.len() {
        if vec[i] <= vec[i - 1] {
            is_ascending = false;
        }
        if vec[i] >= vec[i - 1] {
            is_descending = false;
        }
    }
    (is_ascending, is_descending)
}

fn get_stats(vec: &Vec<u8>) -> (u8, u8) {
    let mut asc = 0;
    let mut desc = 0;
    for i in 1..vec.len() {
        if vec[i] < vec[i - 1] {
            desc += 1;
        }
        if vec[i] > vec[i - 1] {
            asc += 1;
        }
    }
    return (asc, desc);
}

fn check_gap(is_descending: bool, vec: &Vec<u8>) -> bool {
    if is_descending {
        for i in 1..vec.len() {
            if (vec[i - 1] - vec[i]) > 3 {
                return false;
            }
        }
        println!("VALID");
        return true;
    } else {
        for i in 1..vec.len() {
            if vec[i] - vec[i - 1] > 3 {
                return false;
            }
        }
        println!("VALID");
        return true;
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let list = input.split("\n");
    let mut result: u32 = 0;
    for record in list {
        let mut values_list: Vec<u8> = record
            .split_whitespace()
            .filter_map(|s| s.parse::<u8>().ok())
            .collect();
        if values_list.is_empty() {
            continue;
        }
        let ln = values_list.len();
        values_list = remove_duplicate(values_list);
        if ln - values_list.len() > 1 {
            continue;
        }
        let (asc, desc) = get_stats(&values_list);
        println!("Handling: {:?}", values_list);
        match (asc, desc) {
            (0, val) => {
                if val < (values_list.len() - 2) as u8 {
                    continue;
                }
                if check_gap(true, &values_list) {
                    result += 1;
                }
            }
            (val, 0) => {
                if val < (values_list.len() - 2) as u8 {
                    continue;
                }
                if check_gap(false, &values_list) {
                    result += 1;
                }
            }
            (1, val) => {
                if val != (values_list.len() - 2) as u8 {
                    continue;
                }
                for i in 0..values_list.len() - 1 {
                    if values_list[i] < values_list[i + 1] {
                        values_list.remove(i);
                        break;
                    }
                }
                let (asc, _) = get_stats(&values_list);
                if asc != 0 {
                    continue;
                }
                // Remove faulty
                if check_gap(true, &values_list) {
                    result += 1;
                }
            }
            (val, 1) => {
                if val != (values_list.len() - 2) as u8 {
                    continue;
                }
                for i in 0..values_list.len() - 1 {
                    if values_list[i] > values_list[i + 1] {
                        values_list.remove(i);
                        break;
                    }
                }
                let (_, desc) = get_stats(&values_list);
                if desc != 0 {
                    continue;
                }
                // Remove faulty
                if check_gap(false, &values_list) {
                    result += 1;
                }
            }
            (_, _) => continue,
        }
    }
    Some(result)
}

fn remove_duplicate(vec: Vec<u8>) -> Vec<u8> {
    let mut seen = HashSet::new();
    vec.into_iter().filter(|x| seen.insert(*x)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
