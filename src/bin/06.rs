advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let input_vec: Vec<&str> = input.split_terminator("\n").collect();
    let map = Map::new(input_vec);
    let mut starting_position = Coordinates::new(0, 0);
    if let Some((x, y)) = map.get_starting_position() {
        starting_position = Coordinates::new(x, y);
    }
    let mut guard = Guard::new(Direction::Up, starting_position);
    guard.walk_with_map(&map);
    Some(guard.steps_taken.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input_vec: Vec<&str> = input.split_terminator("\n").collect();
    let map = Map::new(input_vec);
    let mut starting_position = Coordinates::new(0, 0);
    if let Some((x, y)) = map.get_starting_position() {
        starting_position = Coordinates::new(x, y);
    }
    let mut guard = Guard::new(Direction::Up, starting_position);
    guard.walk_with_map(&map);
    Some(guard.number_of_obstacles)
}

struct Map<'a> {
    layout: Vec<&'a str>,
}

impl<'a> Map<'a> {
    fn new(layout: Vec<&'a str>) -> Self {
        Self { layout }
    }

    fn get_starting_position(&self) -> Option<(usize, usize)> {
        for (y, record) in self.layout.iter().enumerate() {
            if let Some(x) = record.find('^') {
                return Some((x, y));
            }
        }
        None
    }

    fn has_hash_between_coordinates(&self, coord1: Coordinates, coord2: Coordinates) -> bool {
        if coord1.x != coord2.x && coord1.y != coord2.y {
            panic!("Checking unrelated coordinates: {:?}, {:?}", coord1, coord2);
        } else if coord1.x == coord2.x {
            for i in coord1.y..coord2.y {
                if let Some(s) = self.layout.get(i) {
                    if s.chars().nth(coord1.x) == Some('#') {
                        return true;
                    }
                }
            }
            return false;
        } else {
            return self.layout[coord1.y][coord1.x..coord2.x].contains('#');
        }
    }

    fn get_next_char(
        &self,
        current_position: Coordinates,
        moving_direction: Direction,
    ) -> Option<char> {
        match moving_direction {
            Direction::Up => {
                if current_position.y == 0 {
                    return None;
                }
                return self.layout[current_position.y - 1]
                    .chars()
                    .nth(current_position.x);
            }
            Direction::Down => {
                if current_position.y == self.layout.len() - 1 {
                    return None;
                }
                return self.layout[current_position.y + 1]
                    .chars()
                    .nth(current_position.x);
            }
            Direction::Right => {
                if current_position.x == self.layout[0].len() - 1 {
                    return None;
                }
                return self.layout[current_position.y]
                    .chars()
                    .nth(current_position.x + 1);
            }
            Direction::Left => {
                if current_position.x == 0 {
                    return None;
                }
                return self.layout[current_position.y]
                    .chars()
                    .nth(current_position.x - 1);
            }
        }
    }
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

    fn turn_right(&mut self) {
        match self.walking_direction {
            Direction::Up => self.walking_direction = Direction::Right,
            Direction::Down => self.walking_direction = Direction::Left,
            Direction::Right => self.walking_direction = Direction::Down,
            Direction::Left => self.walking_direction = Direction::Up,
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

    fn walk_with_map(&mut self, map: &Map) {
        self.steps_taken.push(self.position);
        self.turn_list.push((self.position, self.walking_direction));
        while let Some(c) = map.get_next_char(self.position, self.walking_direction) {
            if c == '#' {
                self.turn_right();
                self.turn_list.push((self.position, self.walking_direction));
            }
            self.check_path_to_the_right(map);
            self.take_step();
        }
    }

    fn check_path_to_the_right(&mut self, map: &Map) {
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
                    if !map.has_hash_between_coordinates(self.position, *coord) {
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
                    if !map.has_hash_between_coordinates(*coord, self.position) {
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
                    if !map.has_hash_between_coordinates(self.position, *coord) {
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
                    if !map.has_hash_between_coordinates(*coord, self.position) {
                        println!("Increased");
                        self.number_of_obstacles += 1;
                    }
                }
            }
        }
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
