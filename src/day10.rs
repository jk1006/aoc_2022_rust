#[derive(Clone, Debug)]
enum Instruction {
    Noop,
    Add(i32),
}

impl Instruction {
    fn get_value(&self) -> i32 {
        match self {
            Instruction::Add(value) => *value,
            _ => 0,
        }
    }
}

pub fn run() {
    let input = split_input(include_str!("puzzle_inputs/day10.txt"));
    println!("Solution day10_part1: {}", solve_part1(input.clone()));
    println!("Solution day10_part2: {}", solve_part2(input));
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
   let board = vec![vec!['.';40];6]; 
   let mut current_cycle = 1;
   let mut cycles_instruction_finished = 0;
   let mut sprite_middle = 1;
   let mut value_to_add = 0;
   for instruction in input {
       match instruction {
        Instruction::Add(value) => {
            value_to_add = value;
            cycles_instruction_finished = 2;
            current_cycle += 1;
        },
        Instruction::Noop => {
           if cycles_instruction_finished > 0 {
            cycles_instruction_finished -= 1;
           }
        }
       } 
       if cycles_instruction_finished == 0 {
        
       }
   }

    "".to_string()
}

fn split_input(input: &str) -> Vec<Instruction> {
    input
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| create_instcution_from_str(x))
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
        #######.......#######.......#######.....";
        assert_eq!(solve_part2(input), expected.to_string());
    }
}
