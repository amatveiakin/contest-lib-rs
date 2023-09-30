// UNFINISHED

use std::collections::HashSet;

use contest_lib_rs::io::prelude::*;

fn solve_case_impl(a: u32, b: u32, c: u32, d: u32, m: u32) -> i32 {
    if (a, b) == (c, d) {
        return 0;
    }
    let mut old_gen = HashSet::new();
    let mut gen = vec![];
    gen.push((a, b));
    let mut op = 1;
    while !gen.is_empty() {
        let mut next_gen = vec![];
        for (x, y) in gen {
            for (nx, ny) in [
                (x & y, y),
                (x | y, y),
                (x, x ^ y),
                (x, y ^ m),
            ] {
                if (nx, ny) == (c, d) {
                    return op;
                }
                if old_gen.insert((nx, ny)) {
                    next_gen.push((nx, ny));
                }
            }
        }
        gen = next_gen;
        op += 1;
    }
    return -1;
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [a, b, c, d, m] = read.u32s();
    emitln!(write, solve_case_impl(a, b, c, d, m));
}

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let t = read.usize();
    for _ in 0..t {
        solve_case(read, write);
    }
}

fn main() {
    let mut read = Reader::new(std::io::stdin().lock());
    let mut write = std::io::stdout().lock();
    solve(&mut read, &mut write);
}


#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use contest_lib_rs::iterutils_basic::IterutilsBasic;
    use contest_lib_rs::rand::{self, random};
    use pretty_assertions::assert_eq;
    use contest_lib_rs::testing::solution_testing::prelude::*;

    // #[test]
    // fn log_test() {
    //     let [a, b, c, d, m] = [5784, 2243, 9744, 8192, 9873];
    //     let bit_ret = (0..30).map(|bit| {
    //         let mask = 1 << bit;
    //         let a = a & mask;
    //         let b = b & mask;
    //         let c = c & mask;
    //         let d = d & mask;
    //         let m = m & mask;
    //         solve_case_impl(a, b, c, d, m)
    //     }).collect_vec();
    //     println!("{}", solve_case_impl(a, b, c, d, m));
    //     println!("{:?}", bit_ret);
    //     panic!();
    // }

    // #[test]
    // fn gen_test() {
    //     for x in [5784, 2243, 9744, 8192, 9873] {
    //         println!("{x:#016b}");
    //     }
    //     panic!();
    //     loop {
    //         let a = random::<u32>() % 10000;
    //         let b = random::<u32>() % 10000;
    //         let c = random::<u32>() % 10000;
    //         let d = random::<u32>() % 10000;
    //         let m = random::<u32>() % 10000;
    //         let ret = solve_case_impl(a, b, c, d, m);
    //         if ret > 5 {
    //             panic!("{} {} {} {} {} => {}", a, b, c, d, m, ret);
    //         }
    //     }
    // }

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve_case, "1 1 1 1 1"), "0");
        assert_trimmed_eq!(&run_solver(solve, "\
        10
        1 0 1 1 1
        3 3 1 2 1
        1 6 0 7 1
        2 4 4 9 8
        21 4 0 17 28
        50 50 0 0 39
        95 33 1 33 110
        138 202 174 64 108
        78 340 68 340 461
        457 291 491 566 766
        "), "\
        1
        -1
        2
        -1
        -1
        2
        1
        4
        1
        3");
    }
}



// use std::collections::HashMap;
//
// use contest_lib_rs::directed_graph::DirectedGraph;
// use contest_lib_rs::testing::graph_output::save_graph;
//
// fn main() {
//     let mut graph = DirectedGraph::new();
//
//     let (a, b, c, d, _m) = (457, 291, 491, 566, 766);
//     let mut old_gen = HashMap::new();
//     let mut gen = vec![];
//     let fmt = |x, y| format!("{:#012b}\n{:#012b}", x, y);
//     gen.push((graph.add_vertex_p(fmt(a, b)), (a, b)));
//     let mut op = 1;
//     while !gen.is_empty() {
//         let mut next_gen = vec![];
//         for (u, (x, y)) in gen {
//             for (nx, ny) in [
//                 (x & y, y),
//                 (x | y, y),
//                 (x, x ^ y),
//                 // (x, y ^ m),
//             ] {
//                 if (nx, ny) == (c, d) {
//                     break;
//                 }
//                 if let Some(v) = old_gen.get(&(nx, ny)) {
//                     graph.add_edge(u, *v);
//                 } else {
//                     let v = graph.add_vertex_p(fmt(nx, ny));
//                     old_gen.insert((nx, ny), v);
//                     graph.add_edge(u, v);
//                     next_gen.push((v, (nx, ny)));
//                 }
//             }
//         }
//         gen = next_gen;
//         op += 1;
//     }
//     _ = op;
//
//     save_graph(&graph);
// }
