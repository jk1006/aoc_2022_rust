#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

pub fn run() {
    let input = split_input(include_str!("puzzle_inputs/day2.txt"), true);
    println!("Solution day2_part1: {}", solve(&input));
    let input = split_input(include_str!("puzzle_inputs/day2.txt"), false);
    println!("Solution day2_part2: {}", solve(&input));
}

fn solve(input: &Vec<( Move, Move )>) -> usize {
    return input.into_iter().map(|(m1, m2)| {
        calculate_score((m1, m2)) + get_basic_score(m2)
    } ).sum();
    
}

fn calculate_score(mvs: (&Move, &Move)) -> usize {
    match mvs {
        (Move::Rock, Move::Rock) => 3,
        (Move::Rock, Move::Paper) => 6,
        (Move::Rock, Move::Scissors) => 0,
        (Move::Paper, Move::Rock) => 0,
        (Move::Paper, Move::Paper) => 3,
        (Move::Paper, Move::Scissors) => 6,
        (Move::Scissors, Move::Rock) => 6,
        (Move::Scissors, Move::Scissors) => 3,
        (Move::Scissors, Move::Paper) => 0,
    }
}

fn get_basic_score(mv: &Move) -> usize {
    match mv {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    }
}

fn split_input(input: &str, part1: bool) -> Vec<( Move, Move)> {
    let mut result: Vec<(Move,Move)> = Vec::new();
    let lines: Vec <&str> = input.split("\n").filter(|x| x != &"").collect();
    for l in lines {
        let moves: Vec<String> = l.split(" ").map(|x| x.to_string()).collect();
        result.push((return_move_for_input(&moves[0]), if part1 {return_move_for_input(&moves[1])} else {return_winning_move(&moves[0], &moves[1])})) ;
    }
    result
}

fn return_winning_move(moves_1: &str, moves_2: &str) -> Move {
    match moves_2 {
        "X" => get_losing_move(moves_1),
        "Y" => return_move_for_input(moves_1),
        "Z" => get_winning_move(moves_1),
        _ => panic!("invalid move"),
    }
}

fn get_losing_move(mv: &str) -> Move {
    match mv {
        "A" => Move::Scissors,
        "B" => Move::Rock,
        "C" => Move::Paper,
        _ => panic!("invalid move"),
    }
}

fn get_winning_move(mv: &str) -> Move {
    match mv {
        "A" => Move::Paper,
        "B" => Move::Scissors,
        "C" => Move::Rock,
        _ => panic!("invalid move"),
    }
}

fn return_move_for_input(move_input: &str) -> Move {
    match move_input {
        "A" | "X" => Move::Rock,
        "B" | "Y" => Move::Paper,
        "C" | "Z" => Move::Scissors,
        _=> { println!("move {:?}", move_input); panic!("Invalid move!")},
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
