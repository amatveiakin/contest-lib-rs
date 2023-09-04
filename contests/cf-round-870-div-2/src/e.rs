use contest_lib_rs::{io, emitln, bitset::Bitset};

fn dfs(edges_in: &[Bitset], individual_profits: &[i64], total_profits: &mut [Option<i64>], v: usize) {
    if total_profits[v].is_some() {
        return;
    }
    let mut max_parent_profit = 0;
    for (u, edge_in) in edges_in[v].iter().enumerate() {
        if edge_in {
            dfs(edges_in, individual_profits, total_profits, u);
            max_parent_profit = max_parent_profit.max(total_profits[u].unwrap());
        }
    }
    total_profits[v] = Some(individual_profits[v] + max_parent_profit);
}

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let m = read.usize();
    let n = read.usize();
    let p = read.vec_i64(n);
    let mut full = Bitset::new(n);
    full.fill(true);
    let mut edges_in = vec![full; n];
    for _ in 0..m {
        let mut r = read.vec_i32(n).into_iter().enumerate().map(|(i, v)| (v, i)).collect::<Vec<_>>();
        r.sort();
        let mut group_rating = -1;
        let mut group_models = vec![];
        let mut mask = Bitset::new(n);
        for &(rating, i) in r.iter() {
            if rating != group_rating {
                for j in group_models.drain(..) {
                    mask.set(j, true);
                }
                group_rating = rating;
            }
            group_models.push(i);
            edges_in[i] &= &mask;
        }
    }
    let mut total_profits = vec![None; n];
    let mut max_total_profit = 0;
    // eprintln!("edges_in =\n{}", edges_in.iter().map(|bs| format!("{bs:?}")).join("\n"));
    for i in 0..n {
        dfs(&edges_in, &p, &mut total_profits, i);
        max_total_profit = max_total_profit.max(total_profits[i].unwrap());
    }
    emitln!(write, max_total_profit);
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
        assert_trimmed_eq!(&run_solver(solve, "\
        3 5
        10 10 10 10 10
        1 2 3 4 5
        1 5 2 3 4
        2 3 4 5 1
        "), "30");
        assert_trimmed_eq!(&run_solver(solve, "\
        3 5
        10 10 10 10 50
        1 2 3 4 5
        1 5 2 3 4
        2 3 4 5 1
        "), "50");
        assert_trimmed_eq!(&run_solver(solve, "\
        1 1
        1000000000
        1
        "), "1000000000");
        assert_trimmed_eq!(&run_solver(solve, "\
        5 5
        1000000000 1000000000 1000000000 1000000000 1000000000
        5 4 3 2 1
        5 4 3 2 1
        5 4 3 2 1
        5 4 3 2 1
        5 4 3 2 1
        "), "5000000000");
        assert_trimmed_eq!(&run_solver(solve, "\
        1 3
        1 2 3
        3 3 3
        "), "3");
    }
}
