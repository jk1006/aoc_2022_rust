
pub fn run() {
    let input = split_input(include_str!("puzzle_inputs/day4.txt"));
    println!("Solution day1_part1: {}", solve_part1(input.clone()));
    println!("Solution day1_part2: {}", solve_part2(input.clone()));
}

fn solve_part1(input: Vec<((usize,usize),(usize,usize))>) -> usize {
    let mut result: usize = 0;
    for line in input {
        if (line.0.0 <= line.1.0 && line.0.1 >= line.1.1)
            || (line.1.0 <= line.0.0 && line.1.1 >= line.0.1) {
                result += 1;
        }
    }

    result
}

fn solve_part2(input: Vec<((usize,usize),(usize,usize))>) -> usize {
    let mut result: usize = 0;

    for line in input {
        for i in line.0.0..line.0.1 + 1 {
            if (line.1.0..line.1.1 + 1).contains(&i) {
                result += 1;
                break;
            }
        } 
    }

    result
}

fn split_input(input: &str) -> Vec<((usize,usize),(usize,usize))> {
    let mut result: Vec<((usize, usize),(usize,usize))> = Vec::new();
    let tmp: Vec<Vec<&str>> =input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.split(",").collect())
        .collect();
    for line in tmp {

        let test1: Vec<usize> = line[0].split("-").map(|st| st.parse::<usize>().unwrap()).collect();
        let tuple1 = (test1[0], test1[1]);

        let test2: Vec<usize> = line[1].split("-").map(|st| st.parse::<usize>().unwrap()).collect();
        let tuple2 = (test2[0], test2[1]);
        result.push((tuple1, tuple2));
    }
    result
}

mod tests {
    use crate::day4::*;
    const TEST_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn day1_part1() {
        let input = split_input(TEST_INPUT);
        assert_eq!(solve_part1(input), 2);
    }

    #[test]
    fn day1_part2() {
        let input = split_input(TEST_INPUT);
        assert_eq!(solve_part2(input), 4);
    }
}
