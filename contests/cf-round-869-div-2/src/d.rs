use std::collections::{HashMap, HashSet};

use contest_lib_rs::{io, emitln};


fn find_cycle(out_edges: &HashMap<usize, Vec<usize>>, start: usize) -> Option<Vec<usize>> {
    let mut visited = HashSet::new();
    visited.insert(start);
    let mut stack = vec![start];
    loop {
        let neighbours = out_edges.get(stack.last().unwrap()).unwrap();
        if neighbours.contains(&start) && stack.len() > 2 {
            return Some(stack);
        }
        if let Some(next) = neighbours.iter().find(|&&x| !visited.contains(&x)) {
            visited.insert(*next);
            stack.push(*next);
        } else {
            stack.pop().unwrap();
            if stack.is_empty() {
                return None;
            }
        }
    }
}

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let t = read.usize();
    'testcase: for _ in 0..t {
        let n = read.usize();
        let m = read.usize();
        let edges = (0..m).map(|_| (read.usize(), read.usize())).collect::<Vec<_>>();
        let mut out_edges = HashMap::new();
        for (a, b) in edges {
            out_edges.entry(a).or_insert(vec![]).push(b);
            out_edges.entry(b).or_insert(vec![]).push(a);
        }
        for a in 1..=n {
            let Some(neighbours) = out_edges.get(&a) else {
                continue;
            };
            if neighbours.len() >= 4 {
                if let Some(cycle) = find_cycle(&out_edges, a) {
                    assert_eq!(cycle[0], a);
                    let special_neighbours: Vec<_> = neighbours
                        .iter()
                        .filter(|&&b| {
                            b != cycle[1] && b != *cycle.last().unwrap()
                        })
                        .take(2)
                        .collect();
                    emitln!(write, "YES");
                    let cl = cycle.len();
                    emitln!(write, cl + 2);
                    for i in 0..cl {
                        emitln!(write, cycle[i], cycle[(i + 1) % cl]);
                    }
                    emitln!(write, a, special_neighbours[0]);
                    emitln!(write, a, special_neighbours[1]);
                    continue 'testcase;
                }
            }
        }
        emitln!(write, "NO");
    }
}

fn main() {
    let mut read = io::Reader::new(std::io::stdin().lock());
    let mut write = std::io::stdout().lock();
    solve(&mut read, &mut write);
}


#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use pretty_assertions::assert_eq;
    use contest_lib_rs::testing::solution_testing::prelude::*;

    #[test]
    fn test() {
//         assert_trimmed_eq!(&run_solver(solve, "\
//         3
//         7 8
//         1 2
//         2 3
//         3 4
//         4 1
//         4 5
//         4 6
//         4 2
//         6 7
//         7 7
//         6 7
//         1 2
//         2 3
//         3 4
//         4 1
//         1 3
//         3 5
//         4 4
//         1 3
//         3 4
//         4 1
//         1 2
// "), "");
    }
}
