use petgraph::prelude::*;
use petgraph::{
    algo::dijkstra,
    dot::{Config, Dot},
};
type Edge = Vec<((usize,usize,char),(usize,usize,char))>;
#[derive(Debug, Clone)]
struct Node {
    pos: (usize, usize),
    c: char,
}
#[derive(Clone)]
pub struct Board {
    start: (usize, usize),
    finish: (usize, usize),
    board: Vec<Vec<Node>>,
}

impl Board {
    fn get_neighbors_for_node(&self, pos: (usize, usize)) -> Vec<&Node> {
        let mut result: Vec<&Node> = Vec::new();
        if pos.0 > 0 {
            result.push(&self.board[pos.1][pos.0 - 1]);
        }
        if pos.0 < self.board[0].len() - 1 {
            result.push(&self.board[pos.1][pos.0 + 1]);
        }

        if pos.1 > 0 {
            result.push(&self.board[pos.1 - 1][pos.0]);
        }
        if pos.1 < self.board.len() - 1 {
            result.push(&self.board[pos.1 + 1][pos.0]);
        }
        result
    }
}
pub fn run() {
    let input = split_input(include_str!("puzzle_inputs/day12.txt"));
    println!("Solution day12_part1: {}", solve_part1(input.clone()));
    println!("Solution day12_part2: {}", solve_part2(input));
}

fn solve_part1(input: Board) -> usize {
    let start = (input.start.1, input.start.0);
    ///let start = (4, 0);
    let edges = create_edges(&input);
    let graph = DiGraphMap::<(usize, usize, char), ()>::from_edges(edges);
    calculate_shortest_path(input, start, &graph).unwrap()
}

fn solve_part2(mut input: Board) -> usize {
    input.board[input.start.1][input.start.0].c = 'a';
    let starting_positions: Vec<(usize, usize)> = input
        .board
        .iter()
        .flatten()
        .filter(|node| node.c == 'a')
        .map(|node| node.pos)
        .collect();
    
    let edges = create_edges(&input);
    let graph = DiGraphMap::<(usize, usize, char), ()>::from_edges(edges);
    let x: Vec<usize> = starting_positions
        .iter()
        .filter_map(|pos| calculate_shortest_path(input.clone(), (pos.1, pos.0), &graph))
        .collect();
    *x.iter().min().unwrap()
}

fn create_edges(input: &Board) -> Edge {

    let mut edges: Edge = Vec::new();
    for i in 0..input.board.len() {
        for j in 0..input.board[0].len() {
            let mut current_edges: Vec<((usize, usize, char), (usize, usize, char))> = Vec::new();
            let current_node = &input.board[i][j];
            let neighbors = input.get_neighbors_for_node((j, i));
            let paths: Vec<&Node> = neighbors
                .iter()
                .filter(|n| n.c as i32 <= current_node.c as i32 + 1)
                .map(|n| *n)
                .collect();

            for path in &paths {
                current_edges.push((
                    (current_node.pos.1, current_node.pos.0, current_node.c),
                    (path.pos.1, path.pos.0, path.c),
                ));
            }
            edges.append(&mut current_edges.clone());
        }
    }
    edges
}


fn calculate_shortest_path(input: Board, start: (usize, usize), graph: &GraphMap<(usize, usize, char), (), Directed>) -> Option<usize> {
    let res = dijkstra(
        &graph,
        (start.0, start.1, 'a'),
        Some((input.finish.1, input.finish.0, '{')),
        |_| 1,
    );
    if res.contains_key(&(input.finish.1, input.finish.0, '{')) {
        Some(res[&(input.finish.1, input.finish.0, '{')])
    } else {
        None
    }
}

fn split_input(input: &str) -> Board {
    let mut board: Vec<Vec<char>> = input
        .split("\n")
        .filter(|l| l != &"")
        .map(|x| x.chars().collect())
        .collect();
    let mut start = (0, 0);
    let mut finish = (0, 0);
    let mut nodes: Vec<Vec<Node>> = Vec::new();
    for i in 0..board.len() {
        nodes.push(Vec::new());
        for j in 0..board[0].len() {
            match board[i][j] {
                'S' => {
                    start = (j, i);
                    board[i][j] = 'a';
                }
                'E' => {
                    finish = (j, i);
                    board[i][j] = '{';
                }
                _ => {}
            }
            nodes[i].push(Node {
                pos: (j, i),
                c: board[i][j],
            })
        }
    }
    Board {
        start,
        finish,
        board: nodes,
    }
}

mod tests {
    #[cfg(test)]
    use crate::day12::*;
    #[cfg(test)]
    const TEST_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn day12_part1() {
        let input = split_input(TEST_INPUT);
        assert_eq!(solve_part1(input), 31);
    }

    #[test]
    fn day12_part2() {
        let input = split_input(TEST_INPUT);
        assert_eq!(solve_part2(input), 29);
    }
}
