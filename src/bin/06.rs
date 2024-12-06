advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let input_vec: Vec<&str> = input.split_terminator("\n").collect();
    let (x, y) = get_starting_position(&input_vec);
    let coordinates: Coordinates = Coordinates::new(x, y);
    Some(handle_part_1(input_vec, coordinates))
}

pub fn part_two(input: &str) -> Option<u32> {
    let input_vec: Vec<&str> = input.split_terminator("\n").collect();
    let (x, y) = get_starting_position(&input_vec);
    let coordinates: Coordinates = Coordinates::new(x, y);
    Some(handle_part_2(input_vec, coordinates))
}

fn handle_part_1(vec: Vec<&str>, starting_position: Coordinates) -> u32 {
    let mut guard = Guard::new(Direction::Up, starting_position);
    guard.walk(&vec);
    guard.steps_taken.len() as u32
}

fn handle_part_2(vec: Vec<&str>, starting_position: Coordinates) -> u32 {
    let mut guard = Guard::new(Direction::Up, starting_position);
    guard.walk(&vec);
    guard.number_of_obstacles
}

fn get_starting_position(vec: &Vec<&str>) -> (usize, usize) {
    for (y, record) in vec.iter().enumerate() {
        if let Some(x) = record.find('^') {
            return (x, y);
        }
    }
    panic!("No '^' found!")
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Coordinates {
    x: usize,
    y: usize,
}

impl Coordinates {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct Guard {
    walking_direction: Direction,
    steps_taken: Vec<Coordinates>,
    position: Coordinates,
    turn_list: Vec<(Coordinates, Direction)>,
    number_of_obstacles: u32,
}

impl Guard {
    fn new(initial_direction: Direction, starting_position: Coordinates) -> Self {
        Self {
            walking_direction: initial_direction,
            steps_taken: Vec::new(),
            position: starting_position,
            turn_list: Vec::new(),
            number_of_obstacles: 0,
        }
    }

    fn walk(&mut self, input: &Vec<&str>) {
        self.steps_taken.push(self.position.clone());
        self.turn_list.push((self.position, self.walking_direction));
        while !self.check_direction(input) {
            self.check_right_direction(input);
            self.take_step();
        }
    }

    fn check_right_direction(&mut self, input: &Vec<&str>) {
        println!(
            "current position: {:?}, direction: {:?}",
            self.position, self.walking_direction
        );
        match self.walking_direction {
            Direction::Up => {
                let dirs: Vec<&(Coordinates, Direction)> = self
                    .turn_list
                    .iter()
                    .filter(|(coord, dir)| {
                        coord.y == self.position.y
                            && coord.x >= self.position.x
                            && *dir == Direction::Down
                    })
                    .collect();
                println!("DIRS: {:?}", dirs);
                if let Some(&(coord, _)) = dirs.iter().min_by_key(|&(coord, _)| coord.x) {
                    if !input[self.position.y][self.position.x..coord.x].contains('#') {
                        println!("Increased");
                        self.number_of_obstacles += 1;
                    }
                }
            }
            Direction::Down => {
                let dirs: Vec<&(Coordinates, Direction)> = self
                    .turn_list
                    .iter()
                    .filter(|(coord, dir)| {
                        coord.y == self.position.y
                            && coord.x <= self.position.x
                            && *dir == Direction::Up
                    })
                    .collect();
                println!("DIRS: {:?}", dirs);
                if let Some(&(coord, _)) = dirs.iter().max_by_key(|&(coord, _)| coord.x) {
                    if !input[self.position.y][coord.x..self.position.x].contains('#') {
                        println!("Increased");
                        self.number_of_obstacles += 1;
                    }
                }
            }
            Direction::Right => {
                let dirs: Vec<&(Coordinates, Direction)> = self
                    .turn_list
                    .iter()
                    .filter(|(coord, dir)| {
                        coord.x == self.position.x
                            && coord.y >= self.position.y
                            && *dir == Direction::Left
                    })
                    .collect();
                println!("DIRS: {:?}", dirs);
                if let Some(&(coord, _)) = dirs.iter().min_by_key(|&(coord, _)| coord.y) {
                    let mut found_hash = false;
                    for i in self.position.y..coord.y {
                        if let Some(s) = input.get(i) {
                            if s.chars().nth(self.position.x) == Some('#') {
                                found_hash = true;
                            }
                        }
                    }
                    if !found_hash {
                        println!("Increased");
                        self.number_of_obstacles += 1;
                    }
                }
            }
            Direction::Left => {
                let dirs: Vec<&(Coordinates, Direction)> = self
                    .turn_list
                    .iter()
                    .filter(|(coord, dir)| {
                        coord.x == self.position.x
                            && coord.y <= self.position.y
                            && *dir == Direction::Right
                    })
                    .collect();
                println!("DIRS: {:?}", dirs);
                if let Some(&(coord, _)) = dirs.iter().max_by_key(|&(coord, _)| coord.y) {
                    let mut found_hash = false;
                    for i in coord.y..self.position.y {
                        if let Some(s) = input.get(i) {
                            if s.chars().nth(self.position.x) == Some('#') {
                                found_hash = true;
                            }
                        }
                    }
                    if !found_hash {
                        println!("Increased");
                        self.number_of_obstacles += 1;
                    }
                }
            }
        }
    }

    fn take_step(&mut self) {
        match self.walking_direction {
            Direction::Up => self.position.y -= 1,
            Direction::Down => self.position.y += 1,
            Direction::Right => self.position.x += 1,
            Direction::Left => self.position.x -= 1,
        }
        if !self.steps_taken.contains(&self.position) {
            self.steps_taken.push(self.position.clone());
        }
    }

    fn check_direction(&mut self, input: &Vec<&str>) -> bool {
        let mut changed_direction = false;
        match self.walking_direction {
            Direction::Up => {
                if self.position.y == 0 {
                    return true;
                }
                if input[self.position.y - 1]
                    .chars()
                    .nth(self.position.x)
                    .unwrap()
                    == '#'
                {
                    self.walking_direction = Direction::Right;
                    changed_direction = true;
                }
            }
            Direction::Down => {
                if self.position.y + 1 == input.len() {
                    return true;
                }
                if input[self.position.y + 1]
                    .chars()
                    .nth(self.position.x)
                    .unwrap()
                    == '#'
                {
                    self.walking_direction = Direction::Left;
                    changed_direction = true;
                }
            }
            Direction::Left => {
                if self.position.x == 0 {
                    return true;
                }
                if input[self.position.y]
                    .chars()
                    .nth(self.position.x - 1)
                    .unwrap()
                    == '#'
                {
                    self.walking_direction = Direction::Up;
                    changed_direction = true;
                }
            }
            Direction::Right => {
                if self.position.x + 1 == input[0].len() {
                    return true;
                }
                if input[self.position.y]
                    .chars()
                    .nth(self.position.x + 1)
                    .unwrap()
                    == '#'
                {
                    self.walking_direction = Direction::Down;
                    changed_direction = true;
                }
            }
        }
        if changed_direction {
            self.turn_list.push((self.position, self.walking_direction));
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
