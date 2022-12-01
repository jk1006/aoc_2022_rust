
pub fn run() {
    let input = split_input(include_str!("puzzle_inputs/day1.txt"));
    println!("Solution day1_part1: {}", solve_part1(&input));
    println!("Solution day1_part2: {}", solve_part2(&input));

}

fn solve_part1(input: &Vec<Vec<&str>>) -> usize {
    return sum_up_calories(input).into_iter().max().unwrap_or(0);
}

fn solve_part2(input: &Vec<Vec<&str>>) -> usize {
    let mut calories: Vec<usize> = sum_up_calories(input);
    calories.sort_by(|a,b| b.cmp(a));
    return calories[0..3].into_iter().sum();
}

fn sum_up_calories(input: &Vec<Vec<&str>>) -> Vec<usize> {
    return input.into_iter().map(|c| c.iter().map(|x| x.parse::<usize>().unwrap()).sum()).collect();
}

fn split_input(input: &str) -> Vec<Vec<&str>> {
    let input: Vec<&str> = input.split("\n\n").collect();
    return input.into_iter().map(|x| x.split("\n").filter(|&x| x != "").collect()).collect();
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
        assert_eq!(solve_part1(&input), 24000);    
    }

    #[test]
    fn day1_part2() {
        let input = split_input(TEST_INPUT);
        assert_eq!(solve_part2(&input), 45000);
    }
}
