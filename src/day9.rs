#[derive(Clone)]
enum Direction {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

#[derive(Debug)]
struct Grid {
    position_head: (usize, usize),
    positions_tails: Vec<(usize, usize)>,
    positions_visited_by_tail: Vec<(usize, usize)>,
}

impl Grid {
    fn new(tails: usize) -> Self {
        let mut tails_tmp: Vec<(usize, usize)> = Vec::new();
        for _ in 0..tails {
            tails_tmp.push((5000, 5000))
        }
        Self {
            position_head: (5000, 5000),
            positions_tails: tails_tmp,
            positions_visited_by_tail: vec![(5000, 5000)],
        }
    }

    fn to_string(&self) -> String {
        let line = vec!['.'; 26];
        let mut lines = vec![line; 21];
        lines[self.position_head.1][self.position_head.0] = 'H';
        for (i, tail) in self.positions_tails.iter().enumerate() {
            if lines[tail.1][tail.0] == '.' {
                lines[tail.1][tail.0] = char::from_digit((i + 1) as u32, 10).unwrap();
            }
        }
        let mut result = "".to_string();
        for line in lines {
            result.push_str(&(line.iter().cloned().collect::<String>() + "\n"));
        }
        result
    }
}

pub fn run() {
    let input = split_input(include_str!("puzzle_inputs/day9.txt"));
    println!("Solution day9_part1: {}", solve_part1(input.clone()));
    println!("Solution day9_part2: {}", solve_part2(input));
}

fn solve_part1(input: Vec<Direction>) -> usize {
    let mut grid = Grid::new(1);
    for direction in input {
        match direction {
            Direction::Up(amount) => move_up(&mut grid, amount),
            Direction::Down(amount) => move_down(&mut grid, amount),
            Direction::Left(amount) => move_left(&mut grid, amount),
            Direction::Right(amount) => move_right(&mut grid, amount),
        }
    }

    grid.positions_visited_by_tail.len()
}

fn solve_part2(input: Vec<Direction>) -> usize {
    let mut grid = Grid::new(9);
    for direction in input {
        match direction {
            Direction::Up(amount) => move_head_up(&mut grid, amount),
            Direction::Down(amount) => move_head_down(&mut grid, amount),
            Direction::Left(amount) => move_head_left(&mut grid, amount),
            Direction::Right(amount) => move_head_right(&mut grid, amount),
        }
    }

    grid.positions_visited_by_tail.len()
}

fn move_head_right(grid: &mut Grid, amount: usize) {
    for _ in 0..amount {
        grid.position_head.0 += 1;
        for i in 0..grid.positions_tails.len() {
            let mut position_head = (0, 0);
            if i == 0 {
                position_head = grid.position_head;
            } else {
                position_head = grid.positions_tails[i - 1];
            }
            let position_tail = grid.positions_tails[i];
            grid.positions_tails[i] = calculate_new_position(position_head, position_tail);
        }
        update_visited_positions(grid);
    }
}

fn move_head_left(grid: &mut Grid, amount: usize) {
    for _ in 0..amount {
        grid.position_head.0 -= 1;
        for i in 0..grid.positions_tails.len() {
            let mut position_head = (0, 0);
            if i == 0 {
                position_head = grid.position_head;
            } else {
                position_head = grid.positions_tails[i - 1];
            }
            let position_tail = grid.positions_tails[i];
            grid.positions_tails[i] = calculate_new_position(position_head, position_tail);
        }
        update_visited_positions(grid);
    }
}

fn move_head_down(grid: &mut Grid, amount: usize) {
    for _ in 0..amount {
        grid.position_head.1 += 1;
        for i in 0..grid.positions_tails.len() {
            let mut position_head = (0, 0);
            if i == 0 {
                position_head = grid.position_head;
            } else {
                position_head = grid.positions_tails[i - 1];
            }
            let position_tail = grid.positions_tails[i];
            grid.positions_tails[i] = calculate_new_position(position_head, position_tail);
        }
        update_visited_positions(grid);
    }
}

fn move_head_up(grid: &mut Grid, amount: usize) {
    for _ in 0..amount {
        grid.position_head.1 -= 1;
        for i in 0..grid.positions_tails.len() {
            let mut position_head = (0, 0);
            if i == 0 {
                position_head = grid.position_head;
            } else {
                position_head = grid.positions_tails[i - 1];
            }
            let position_tail = grid.positions_tails[i];
            grid.positions_tails[i] = calculate_new_position(position_head, position_tail);
        }
        update_visited_positions(grid);
    }
}

fn calculate_new_position(
    position_head: (usize, usize),
    position_tail: (usize, usize),
) -> (usize, usize) {
    let difference_x: i32 = position_head.0 as i32 - position_tail.0 as i32;
    let difference_y: i32 = position_head.1 as i32 - position_tail.1 as i32;
    match (difference_x, difference_y) {
        (0, 2) => (position_head.0, position_head.1 - 1),
        (0, -2) => (position_head.0, position_head.1 + 1),
        (2, 0) => (position_head.0 - 1, position_tail.1),
        (-2, 0) => (position_head.0 + 1, position_tail.1),
        (1, 2) => (position_head.0, position_head.1 - 1),
        (-1, 2) => (position_head.0, position_head.1 - 1),
        (1, -2) => (position_head.0, position_head.1 + 1),
        (-1, -2) => (position_head.0, position_head.1 + 1),
        (2, 1) => (position_head.0 - 1, position_head.1),
        (-2, 1) => (position_head.0 + 1, position_head.1),
        (2, -1) => (position_head.0 - 1, position_head.1),
        (-2, -1) => (position_head.0 + 1, position_head.1),
        (-2, -2) => (position_head.0 + 1, position_head.1 + 1),
        (-2, 2) => (position_head.0 + 1, position_head.1 - 1),
        (2, 2) => (position_head.0 - 1, position_head.1 - 1),
        (2, -2) => (position_head.0 - 1, position_head.1 + 1),
        _ => position_tail,
    }
}

fn move_up(grid: &mut Grid, amount: usize) {
    for _ in 0..amount {
        for i in 0..grid.positions_tails.len() {
            let mut head = (0, 0);
            if i == 0 {
                grid.position_head.1 -= 1;
                head = grid.position_head;
            } else {
                head = grid.positions_tails[i - 1];
            }
            let mut tail = grid.positions_tails[i];
            if i == 0 {
                grid.position_head = head;
            } else {
                grid.positions_tails[i - 1] = head;
            }
            if head.1 < tail.1 - 1 {
                tail.1 -= 1;
                tail.0 = head.0;
                grid.positions_tails[i] = tail;
                update_visited_positions(grid);
            }
            grid.positions_tails[i] = tail;
        }
    }
}
fn move_down(grid: &mut Grid, amount: usize) {
    for _ in 0..amount {
        for i in 0..grid.positions_tails.len() {
            let mut head = (0, 0);
            if i == 0 {
                grid.position_head.1 += 1;
                head = grid.position_head;
            } else {
                head = grid.positions_tails[i - 1];
            }
            let mut tail = grid.positions_tails[i];
            if i == 0 {
                grid.position_head = head;
            } else {
                grid.positions_tails[i - 1] = head;
            }
            if head.1 > tail.1 + 1 {
                tail.1 += 1;
                tail.0 = head.0;
                grid.positions_tails[i] = tail;
                update_visited_positions(grid);
            }
            grid.positions_tails[i] = tail;
        }
    }
}
fn move_left(grid: &mut Grid, amount: usize) {
    for _ in 0..amount {
        for i in 0..grid.positions_tails.len() {
            let mut head = (0, 0);
            if i == 0 {
                grid.position_head.0 -= 1;
                head = grid.position_head;
            } else {
                head = grid.positions_tails[i - 1];
            }
            let mut tail = grid.positions_tails[i];
            if i == 0 {
                grid.position_head = head;
            } else {
                grid.positions_tails[i - 1] = head;
            }
            grid.positions_tails[i] = tail;
            if head.0 < tail.0 - 1 {
                tail.0 -= 1;
                tail.1 = head.1;
                grid.positions_tails[i] = tail;
                update_visited_positions(grid);
            }
            grid.positions_tails[i] = tail;
        }
    }
}
fn move_right(grid: &mut Grid, amount: usize) {
    for _ in 0..amount {
        for i in 0..grid.positions_tails.len() {
            let mut head = (0, 0);
            if i == 0 {
                grid.position_head.0 += 1;
                head = grid.position_head;
            } else {
                head = grid.positions_tails[i - 1];
            }
            let mut tail = grid.positions_tails[i];
            if i == 0 {
                grid.position_head = head;
            } else {
                grid.positions_tails[i - 1] = head;
            }
            if head.0 > tail.0 + 1 {
                tail.0 += 1;
                tail.1 = head.1;
                grid.positions_tails[i] = tail;
                update_visited_positions(grid);
            }
            grid.positions_tails[i] = tail;
        }
    }
}

fn update_visited_positions(grid: &mut Grid) {
    if !grid
        .positions_visited_by_tail
        .contains(&grid.positions_tails[&grid.positions_tails.len() - 1])
    {
        grid.positions_visited_by_tail
            .push(grid.positions_tails[&grid.positions_tails.len() - 1]);
    }
}

fn split_input(input: &str) -> Vec<Direction> {
    let mut result: Vec<Direction> = Vec::new();
    for line in input.split('\n') {
        if !line.is_empty() {
            let mut split = line.split(' ');
            let direction = match split.next().unwrap() {
                "R" => Direction::Right(split.next().unwrap().parse::<usize>().unwrap()),
                "L" => Direction::Left(split.next().unwrap().parse::<usize>().unwrap()),
                "U" => Direction::Up(split.next().unwrap().parse::<usize>().unwrap()),
                "D" => Direction::Down(split.next().unwrap().parse::<usize>().unwrap()),
                _ => unreachable!(),
            };
            result.push(direction);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::day9::*;
    const TEST_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const TEST_INPUT_2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn day9_part1() {
        let input = split_input(TEST_INPUT);
        assert_eq!(solve_part1(input), 13);
    }

    #[test]
    fn day9_part2() {
        let input = split_input(TEST_INPUT_2);
        assert_eq!(solve_part2(input), 36);
    }
}
