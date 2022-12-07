use std::collections::HashMap;

pub fn run() {
    let input = include_str!("puzzle_inputs/day7.txt");
    println!("Solution day1_part1: {}", solve_part1(input));
    println!("Solution day1_part2: {}", solve_part2(input.clone()));
}

fn solve_part1(input: &str) -> u64 {
    let mut cwd: Vec<String> = vec![];
    let mut sizes: HashMap<String, u64> = HashMap::new();

    for line in input.lines() {
        if line.starts_with("$ cd") {
            match &line[5..] {
                ".." => {cwd.pop();},
                "/" => {cwd = vec!["".to_string()]}
                new_dir => cwd.push(new_dir.to_string()),
            }
        } else if line.starts_with("$ ls") || line.starts_with("dir"){
            continue;
        } else {
            let (size, _file_name) = line.split_once(' ').unwrap();
            println!("{}", line);
            println!("size: {}", size);
            let size = size.parse::<u64>().unwrap();
            for i in 0..cwd.len() {
                let key = cwd[0..=i].join("/").to_string();

                sizes.entry(key).and_modify(|a| *a += size).or_insert(size);
            }
        }

    }


    return sizes.iter().filter_map(|(_, &size)| if size <= 100000 {Some(size)} else { None }).sum();
}

fn solve_part2(input: &str) -> u64 {
    let mut cwd: Vec<String> = vec![];
    let mut sizes: HashMap<String, u64> = HashMap::new();

    for line in input.lines() {
        if line.starts_with("$ cd") {
            match &line[5..] {
                ".." => {cwd.pop();},
                "/" => {cwd = vec!["".to_string()]}
                new_dir => cwd.push(new_dir.to_string()),
            }
        } else if line.starts_with("$ ls") || line.starts_with("dir"){
            continue;
        } else {
            let (size, _file_name) = line.split_once(' ').unwrap();
            println!("{}", line);
            println!("size: {}", size);
            let size = size.parse::<u64>().unwrap();
            for i in 0..cwd.len() {
                let key = cwd[0..=i].join("/").to_string();

                sizes.entry(key).and_modify(|a| *a += size).or_insert(size);
            }
        }

    }
    let file_system_size = sizes.get("").unwrap();
    println!("{:?}", file_system_size);
    let to_be_freed = 30000000 - (70000000 - file_system_size);
    return sizes.iter().map(|(_, &size)| size).filter(|size| size >= &to_be_freed).min_by(|a, b| a.cmp(b)).unwrap();
}

mod tests {
    use crate::day7::*;
    const TEST_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn day1_part1() {
        assert_eq!(solve_part1(TEST_INPUT), 95437);
    }

    #[test]
    fn day1_part2() {
        assert_eq!(solve_part2(TEST_INPUT), 24933642);
    }
}
