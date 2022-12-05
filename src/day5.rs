pub fn run() {
    let input = split_input(include_str!("puzzle_inputs/day1.txt"));
    println!("Solution day1_part1: {}", solve_part1(input.clone()));
    println!("Solution day1_part2: {}", solve_part2(input.clone()));
}

#[derive(Clone)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

#[derive(Clone)]
struct Stack {
    crates: Vec<char>,
    moves: Vec<Move>,
}

impl Stack {
    fn add_move(&mut self, mv: Move) {
        self.moves.push(mv);
    }
}

fn solve_part1(input: Vec<usize>) -> String {

    "".to_string()
}

fn solve_part2(mut input: Vec<usize>) -> usize {
    input.sort();
    return input.iter().rev().take(3).sum();
}

fn split_input(input: &str) -> Vec<Stack> {
    let input_iter = input.split("\n\n");
    let splitted: &str = input_iter.next().unwrap();
    let moves = input_iter.next().unwrap().split("\n")
        .filter(|l| !l.is_empty())
        .map(create_move);

    Stack {
        moves,
    }
}

fn create_move(input: &str) -> Move {
    let move_iter = input.split(" from ");
    let first_part = move_iter.next().unwrap();
    let amount: usize = first_part[5..first_part.len() +1].parse::<usize>().unwrap();
    let second_part = move_iter.next().unwrap();
    let from = second_part[0..1].parse::<usize>().unwrap();
    let to = second_part[6..7].parse::<usize>().unwrap();

    Move {
        amount,
        from,
        to,
    }
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
    fn day1_part1() {
        let input = split_input(TEST_INPUT);
        assert_eq!(solve_part1(input), "CMZ".to_string());
    }

    #[test]
    fn day1_part2() {
        let input = split_input(TEST_INPUT);
        //assert_eq!(solve_part2(input), 45000);
    }
}
