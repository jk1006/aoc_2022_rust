pub fn run() {
    let input = split_input(include_str!("puzzle_inputs/day6.txt"));
    println!("Solution day6_part1: {}", solve_part1(input.clone()));
    println!("Solution day6_part2: {}", solve_part2(input));
}

fn solve_part1(input: Vec<char>) -> usize {
    let mut last_4 = Vec::new();
    for (i, c) in input.into_iter().enumerate() {
        last_4.push(c);
        if last_4.len() == 5 {
            last_4.remove(0);
            let mut tmp = last_4.clone();
            tmp.sort();
            tmp.dedup();
            if tmp.len() == 4 {
                return i + 1;
            }
        }
    }
    0
}

fn solve_part2(input: Vec<char>) -> usize {
    let mut last_14 = Vec::new();
    for (i, c) in input.into_iter().enumerate() {
        last_14.push(c);
        if last_14.len() == 15 {
            last_14.remove(0);
            let mut tmp = last_14.clone();
            tmp.sort();
            tmp.dedup();
            if tmp.len() == 14 {
                return i + 1;
            }
        }
    }
    0
}

fn split_input(input: &str) -> Vec<char> {
    return input.chars().collect();
}

#[cfg(test)]
mod tests {
    use crate::day6::*;
    const TEST_INPUT1: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const TEST_INPUT2: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const TEST_INPUT3: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const TEST_INPUT4: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn day6_part1_1() {
        let input = split_input(TEST_INPUT1);
        assert_eq!(solve_part1(input), 5);
    }

    #[test]
    fn day6_part1_2() {
        let input = split_input(TEST_INPUT2);
        assert_eq!(solve_part1(input), 6);
    }
    #[test]
    fn day6_part1_3() {
        let input = split_input(TEST_INPUT3);
        assert_eq!(solve_part1(input), 10);
    }
    #[test]
    fn day6_part1_4() {
        let input = split_input(TEST_INPUT4);
        assert_eq!(solve_part1(input), 11);
    }
    #[test]
    fn day6_part2_1() {
        let input = split_input("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(solve_part2(input), 19);
    }

    #[test]
    fn day6_part2_2() {
        let input = split_input("bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(solve_part2(input), 23);
    }
    #[test]
    fn day6_part2_3() {
        let input = split_input("nppdvjthqldpwncqszvftbrmjlhg");
        assert_eq!(solve_part2(input), 23);
    }
    #[test]
    fn day6_part2_4() {
        let input = split_input("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        assert_eq!(solve_part2(input), 29);
    }
    #[test]
    fn day6_part2_5() {
        let input = split_input("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        assert_eq!(solve_part2(input), 26);
    }
}
