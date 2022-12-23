type Op = Box<dyn Fn(&u64) -> u64>;
type Throw = Box<dyn Fn(&u64) -> usize>;
struct Monkey {
    id: i32,
    items: Vec<u64>,
    operation: Op,
    throw: Throw,
    items_inspected: u64,
    div_by: usize,
}

pub fn run() {
    let input = split_input(include_str!("puzzle_inputs/day11.txt"));
    println!("Solution day11_part1: {}", solve_part1(input));
    let input = split_input(include_str!("puzzle_inputs/day11.txt"));
    println!("Solution day11_part2: {}", solve_part2(input));
}

fn solve_part1(mut input: Vec<Monkey>) -> u64 {
    let round = 20;
    for _ in 0..round {
        for i in 0..input.len() {
            let monkey = &mut input[i];
            let mut items_to_be_deleted: Vec<u64> = Vec::new();
            let mut items_to_throw: Vec<(i32, u64)> = Vec::new();
            for item in &monkey.items {
                monkey.items_inspected += 1;
                items_to_be_deleted.push(item.clone());
                let mut new_item_value = (monkey.operation)(&item);
                new_item_value = new_item_value / 3;
                items_to_throw.push(((monkey.throw)(&new_item_value) as i32, new_item_value));
            }
            for i in items_to_be_deleted {
                monkey
                    .items
                    .remove(monkey.items.iter().position(|it| it == &i).unwrap());
            }
            for (id, item_val) in items_to_throw {
                let index_target_monkey = input.iter().position(|m| m.id == id).unwrap();
                let mon = input.iter_mut().nth(index_target_monkey).unwrap();
                mon.items.push(item_val);
            }
        }
    }

    let mut inspected: Vec<u64> = input.iter().map(|m| m.items_inspected).collect();
    inspected.sort();
    inspected
        .into_iter()
        .rev()
        .take(2)
        .fold(1, |acc, x| acc * x)
}

fn solve_part2(mut input: Vec<Monkey>) -> u64 {
    let round = 10000;
    let kgv = input.iter().map(|m| m.div_by).fold(1, |acc, x| acc * x) as u64;
    for r in 0..round {
        for i in 0..input.len() {
            let monkey = &mut input[i];
            let mut items_to_be_deleted: Vec<u64> = Vec::new();
            let mut items_to_throw: Vec<(usize, u64)> = Vec::new();
            for item in &monkey.items {
                monkey.items_inspected += 1;
                items_to_be_deleted.push(item.clone());
                let new_item_value = (monkey.operation)(&item) % kgv;
                items_to_throw.push(((monkey.throw)(&new_item_value), new_item_value));
            }
            for i in items_to_be_deleted {
                monkey
                    .items
                    .remove(monkey.items.iter().position(|it| it == &i).unwrap());
            }
            for (id, item_val) in items_to_throw {
                let index_target_monkey = input.iter().position(|m| m.id == id as i32).unwrap();
                let mon = input.iter_mut().nth(index_target_monkey).unwrap();
                mon.items.push(item_val);
            }
        }
    }

    let mut inspected: Vec<u64> = input.iter().map(|m| m.items_inspected).collect();
    inspected.sort();
    inspected
        .into_iter()
        .rev()
        .take(2)
        .fold(1, |acc, x| acc * x)
}

fn split_input(input: &str) -> Vec<Monkey> {
    let paragraphs = input.split("\n\n");
    let mut result: Vec<Monkey> = Vec::new();
    for par in paragraphs {
        let mut lines = par.split('\n');
        let id = lines
            .next()
            .unwrap()
            .chars()
            .nth(7)
            .unwrap()
            .to_digit(10)
            .unwrap() as i32;
        let line = lines.next().unwrap();
        let items: Vec<u64> = line[18..]
            .split(", ")
            .map(|i| i.parse::<u64>().unwrap())
            .collect();
        let operation = parse_operation(lines.next().unwrap());
        let line_throwing = lines.next().unwrap();
        let div_by: usize = line_throwing[21..].parse::<usize>().unwrap();

        let throw = parse_throwing(line_throwing, lines.next().unwrap(), lines.next().unwrap());
        result.push(Monkey {
            id,
            items,
            operation,
            throw,
            items_inspected: 0,
            div_by,
        });
    }
    result
}

fn parse_throwing(div_line: &str, true_line: &str, false_line: &str) -> Throw {
    let div_by = div_line[21..].parse::<u64>().unwrap();
    let true_monkey = true_line[29..].parse::<u64>().unwrap();
    let false_monkey = false_line[30..].parse::<u64>().unwrap();
    Box::new(move |worry_level| {
        if worry_level % div_by == 0 {
            true_monkey as usize
        } else {
            false_monkey as usize
        }
    })
}

fn parse_operation(line: &str) -> Op {
    let operator: char = line.chars().nth(23).unwrap();
    let second_param: &str = &line[25..];
    match second_param {
        "old" => {
            if operator == '*' {
                return Box::new(|x| x.clone() * x.clone());
            } else {
                return Box::new(|x| x.clone() + x.clone());
            }
        }
        v => {
            let value = v.parse::<u64>().unwrap();
            if operator == '*' {
                return Box::new(move |x| x * value.clone());
            } else {
                return Box::new(move |x| x + value);
            }
        }
    }
}

mod tests {
    #[cfg(test)]
    use crate::day11::*;
    #[cfg(test)]
    const TEST_INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn day1_part1() {
        let input = split_input(TEST_INPUT);
        assert_eq!(solve_part1(input), 10605);
    }

    #[test]
    fn day1_part2() {
        let input = split_input(TEST_INPUT);
        assert_eq!(solve_part2(input), 2713310158);
    }
}
