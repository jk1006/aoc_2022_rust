
pub fn run() {
    let input = split_input(include_str!("puzzle_inputs/day1.txt"));
    println!("Solution day1_part1: {}", solve_part1(input.clone()));
    println!("Solution day1_part2: {}", solve_part2(input.clone()));

}

fn solve_part1(input: Vec<usize>) -> usize {
    return input.into_iter().max().unwrap_or(0);
}

fn solve_part2(mut input: Vec<usize> ) -> usize {
    input.sort();
    return input.iter().rev().take(3).sum();
}

fn split_input(input: &str) -> Vec<usize> {
    return input
        .split("\n\n")
        .map(|x| x.lines().map(|cals| cals.parse::<usize>().unwrap()).sum())
        .collect();
}





mod tests {
    use crate::day1::*;
    const TEST_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";


    #[test]
    fn day1_part1() {
        let input = split_input(TEST_INPUT);
        assert_eq!(solve_part1(input), 24000);    
    }

    #[test]
    fn day1_part2() {
        let input = split_input(TEST_INPUT);
        assert_eq!(solve_part2(input), 45000);
    }
}
