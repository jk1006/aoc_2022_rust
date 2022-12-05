#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn value(&self) -> i32 {
        match *self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

pub fn run() {
    let input = split_input(include_str!("puzzle_inputs/day2.txt"), true);
    println!("Solution day2_part1: {}", solve(&input));
    let input = split_input(include_str!("puzzle_inputs/day2.txt"), false);
    println!("Solution day2_part2: {}", solve(&input));
}

fn solve(input: &Vec<(Move, Move)>) -> usize {
    return input
        .into_iter()
        .map(|(m1, m2)| calculate_score(m1, m2) + m2.value() as usize)
        .sum();
}

fn calculate_score(enemy_move: &Move, my_move: &Move) -> usize {
    let result: i32 = my_move.value() - enemy_move.value();
    match result {
        0 => 3,
        1 | -2 => 6,
        _ => 0,
    }
}

fn split_input(input: &str, part1: bool) -> Vec<(Move, Move)> {
    let mut result: Vec<(Move, Move)> = Vec::new();
    let lines: Vec<&str> = input.split("\n").filter(|x| x != &"").collect();
    for l in lines {
        let moves: Vec<String> = l.split(" ").map(|x| x.to_string()).collect();
        result.push((
            return_move_for_input(&moves[0]),
            if part1 {
                return_move_for_input(&moves[1])
            } else {
                return_requested_move(&moves[0], &moves[1])
            },
        ));
    }
    result
}

fn return_requested_move(moves_1: &str, moves_2: &str) -> Move {
    match moves_2 {
        "X" => get_losing_move(moves_1),
        "Y" => return_move_for_input(moves_1),
        "Z" => get_winning_move(moves_1),
        _ => unreachable!(),
    }
}

fn get_losing_move(mv: &str) -> Move {
    match mv {
        "A" => Move::Scissors,
        "B" => Move::Rock,
        "C" => Move::Paper,
        _ => unreachable!(),
    }
}

fn get_winning_move(mv: &str) -> Move {
    match mv {
        "A" => Move::Paper,
        "B" => Move::Scissors,
        "C" => Move::Rock,
        _ => unreachable!(),
    }
}

fn return_move_for_input(move_input: &str) -> Move {
    match move_input {
        "A" | "X" => Move::Rock,
        "B" | "Y" => Move::Paper,
        "C" | "Z" => Move::Scissors,
        _ => {
            println!("move {:?}", move_input);
            panic!("Invalid move!")
        }
    }
}

mod tests {
    use crate::day2::*;
    const TEST_INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn day2_part1() {
        let input = split_input(TEST_INPUT, true);
        println!("{:?}", input);
        assert_eq!(solve(&input), 15);
    }

    #[test]
    fn day2_part2() {
        let input = split_input(TEST_INPUT, false);
        println!("{:?}", input);
        assert_eq!(solve(&input), 12);
    }
}
