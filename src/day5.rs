pub fn run() {
    let input = split_input(include_str!("puzzle_inputs/day5.txt"));
    println!("Solution day5_part1: {}", solve_part1(input.clone()));
    println!("Solution day5_part2: {}", solve_part2(input.clone()));
}

#[derive(Clone, Debug)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

#[derive(Clone, Debug)]
struct Stack {
    crates: Vec<Vec<char>>,
    moves: Vec<Move>,
}

impl Stack {
    fn add_move(&mut self, mv: Move) {
        self.moves.push(mv);
    }
}

fn solve_part1(mut input: Stack) -> String {
    for mv in input.moves {
        let vec_to_process = input.crates[mv.from - 1].clone();
        input.crates[mv.to - 1].append(
            &mut vec_to_process[vec_to_process.len() - mv.amount..]
                .into_iter()
                .rev()
                .map(|c| *c)
                .collect(),
        );
        if mv.amount == input.crates[mv.from - 1].len() {
            input.crates[mv.from - 1].clear();
        } else {
            input.crates[mv.from - 1] = vec_to_process[..vec_to_process.len() - mv.amount].to_vec();
        }
    }
    let mut result: String = "".to_string();
    for cr in input.crates {
        result.push(cr.into_iter().last().unwrap());
    }
    result
}

fn solve_part2(mut input: Stack) -> String {
    for mv in input.moves {
        let vec_to_process = input.crates[mv.from - 1].clone();
        input.crates[mv.to - 1].append(
            &mut vec_to_process[vec_to_process.len() - mv.amount..]
                .into_iter()
                .map(|c| *c)
                .collect(),
        );
        if mv.amount == input.crates[mv.from - 1].len() {
            input.crates[mv.from - 1].clear();
        } else {
            input.crates[mv.from - 1] = vec_to_process[..vec_to_process.len() - mv.amount].to_vec();
        }
    }
    let mut result: String = "".to_string();
    for cr in input.crates {
        result.push(cr.into_iter().last().unwrap());
    }
    result
}

fn split_input(input: &str) -> Stack {
    let mut input_iter = input.split("\n\n");
    let splitted: &str = input_iter.next().unwrap();
    let moves = input_iter
        .next()
        .unwrap()
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(create_move)
        .collect();
    let mut lines: Vec<&str> = splitted.split("\n").collect();
    let last_line = lines.remove(lines.len() - 1);
    let number_of_cols: usize = last_line
        .split("   ")
        .last()
        .unwrap()
        .trim()
        .parse::<usize>()
        .unwrap();
    let mut crates = vec![];
    for _ in 0..number_of_cols {
        crates.push(Vec::new());
    }
    for line in lines {
        for (i, c) in line.chars().enumerate() {
            if c.is_ascii_alphabetic() && c != ' ' {
                match i {
                    1 => crates[0].insert(0, c),
                    5 => crates[1].insert(0, c),
                    9 => crates[2].insert(0, c),
                    13 => crates[3].insert(0, c),
                    17 => crates[4].insert(0, c),
                    21 => crates[5].insert(0, c),
                    25 => crates[6].insert(0, c),
                    29 => crates[7].insert(0, c),
                    33 => crates[8].insert(0, c),
                    _ => println!("{}, {}", c, i),
                }
            }
        }
    }
    Stack { crates, moves }
}

fn create_move(input: &str) -> Move {
    let mut move_iter = input.split(" from ");
    let first_part = move_iter.next().unwrap();
    let amount: usize = first_part[5..first_part.len()].parse::<usize>().unwrap();
    let second_part = move_iter.next().unwrap();
    let from = second_part[0..1].parse::<usize>().unwrap();
    let to = second_part[5..6].parse::<usize>().unwrap();

    Move { amount, from, to }
}

mod tests {
    use crate::day5::*;
    const TEST_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn day5_part1() {
        let input = split_input(TEST_INPUT);
        assert_eq!(solve_part1(input), "CMZ".to_string());
    }

    #[test]
    fn day5_part2() {
        let input = split_input(TEST_INPUT);
        assert_eq!(solve_part2(input), "MCD".to_string());
    }
}
