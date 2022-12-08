pub fn run() {
    let input = split_input(include_str!("puzzle_inputs/day3.txt"));
    println!("Solution day3_part1: {}", solve_part1(input.clone()));
    println!("Solution day3_part2: {}", solve_part2(input));
}

fn solve_part1(input: Vec<Vec<char>>) -> usize {
    let mut result = 0;

    for line in input {
        let mut chunks = line.chunks(line.len() / 2);
        let mut double_chars: Vec<char> =
            find_double_chars(chunks.next().unwrap(), chunks.next().unwrap());
        double_chars.dedup();
        for c in double_chars {
            if c as usize > 96 {
                result += return_prio_for_lowercase_char(c);
            } else {
                result += return_prio_for_uppercase_char(c);
            }
        }
    }
    result
}

fn find_double_chars(next_1: &[char], next_2: &[char]) -> Vec<char> {
    let mut result: Vec<char> = Vec::new();
    for c in next_1 {
        if next_2.contains(c) {
            result.push(*c);
        }
    }
    result
}

fn solve_part2(input: Vec<Vec<char>>) -> usize {
    let mut result = 0;
    let mut input_iter = input.iter();
    while let Some(line) = input_iter.next() {
        let second_line = input_iter.next().unwrap();
        let third_line = input_iter.next().unwrap();
        let mut line_dedup: Vec<char> = line.clone();
        line_dedup.sort();
        line_dedup.dedup();
        for c in line_dedup {
            if second_line.contains(&c) && third_line.contains(&c) {
                if c as usize > 96 {
                    result += return_prio_for_lowercase_char(c);
                } else {
                    result += return_prio_for_uppercase_char(c);
                }
            }
        }
    }
    result
}

fn return_prio_for_lowercase_char(input: char) -> usize {
    input as usize - 96
}

fn return_prio_for_uppercase_char(input: char) -> usize {
    input as usize - 38
}

fn split_input(input: &str) -> Vec<Vec<char>> {
    let lines = input.split('\n');
    lines
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect()
}

mod tests {
    #[cfg(test)]
    use crate::day3::*;
    #[cfg(test)]
    const TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn day1_part1() {
        let input = split_input(TEST_INPUT);
        assert_eq!(solve_part1(input), 157);
    }

    #[test]
    fn day1_part2() {
        let input = split_input(TEST_INPUT);
        assert_eq!(solve_part2(input), 70);
    }
}
