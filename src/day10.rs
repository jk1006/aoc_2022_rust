#[derive(Clone, Debug)]
enum Instruction {
    Noop,
    Add(i32),
}

pub fn run() {
    let input = split_input(include_str!("puzzle_inputs/day10.txt"));
    println!("Solution day10_part1: {}", solve_part1(input.clone()));
    println!("Solution day10_part2:\n{}", solve_part2(input));
}

fn solve_part1(input: Vec<Instruction>) -> i32 {
    let mut cycle = 0;
    let mut register: i32 = 1;
    let mut result: i32 = 0;
    let relevant_cycles = [20, 60, 100, 140, 180, 220];
    for instruction in input {
        println!(
            "INSTRUCTION: {:?}, CYCLE: {}, REGISTER: {}",
            instruction, cycle, register
        );
        match instruction {
            Instruction::Add(value) => {
                cycle += 1;
                if relevant_cycles.iter().any(|&x| x == cycle) {
                    result += cycle * register;
                }
                cycle += 1;
                if relevant_cycles.iter().any(|&x| x == cycle) {
                    result += cycle * register;
                }
                register += value;
            }
            Instruction::Noop => {
                cycle += 1;
                if relevant_cycles.iter().any(|&x| x == cycle) {
                    result += cycle * register;
                }
            }
        }
    }
    result
}

fn solve_part2(mut input: Vec<Instruction>) -> String {
    let mut board = vec![vec!['.'; 40]; 6];
    let mut sprite_middle = 1;
    let mut no_current_instruction = true;
    let mut current_instruction: (Instruction, usize) = (Instruction::Noop, 0);
    for cycle in 0..240 {
        if no_current_instruction {
            let instr = input.remove(0);
            current_instruction = (instr, 2);
            no_current_instruction = false;
        }
        println!("CYCLE: {}", cycle);
        println!("Currenct instruction: {:?}", current_instruction);
        println!("sprite middle: {}", sprite_middle);
        if (sprite_middle - 1..=sprite_middle + 1).contains(&(cycle % 40)) {
            board[(cycle / 40) as usize][(cycle % 40) as usize] = '#'
        }
        match current_instruction.0 {
            Instruction::Add(value) => {
                current_instruction.1 -= 1;
                if current_instruction.1 == 0 {
                    sprite_middle += value;
                    no_current_instruction = true;
                }
            }
            Instruction::Noop => {
                no_current_instruction = true;
            }
        }
    }
    let mut result = "".to_string();
    for line in board {
        result.push_str(&line.iter().cloned().collect::<String>());
        result.push('\n');
    }
    result
}

fn split_input(input: &str) -> Vec<Instruction> {
    input
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(create_instcution_from_str)
        .collect()
}

fn create_instcution_from_str(line: &str) -> Instruction {
    let mut split = line.split(' ');
    if split.next().unwrap() == "noop" {
        Instruction::Noop
    } else {
        let val: i32 = split.next().unwrap().to_string().parse::<i32>().unwrap();
        Instruction::Add(val)
    }
}

mod tests {
    #[cfg(test)]
    use crate::day10::*;
    #[cfg(test)]
    const TEST_INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn day10_part1() {
        let input = split_input(TEST_INPUT);
        assert_eq!(solve_part1(input), 13140);
    }

    #[test]
    fn day10_part2() {
        let input = split_input(TEST_INPUT);
        let expected = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";
        assert_eq!(solve_part2(input), expected.to_string());
    }
}
