use std::collections::HashMap;

use contest_lib_rs::iterutils::Iterutils;
use contest_lib_rs::{io, emitln};

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let n = read.usize();
    let m = read.usize();
    let mut time_diffs = HashMap::new();
    let mut max_t = vec![None; n];
    let mut playing = vec![true; n];
    playing[n - 1] = false;
    for _ in 0..m {
        let u = read.usize() - 1;
        let v = read.usize() - 1;
        let y = read.i32();
        if v == n - 1 {
            max_t[u] = Some(y);
        } else {
            time_diffs.insert((u, v), y);
        }
    }
    let mut total_t: i64 = 0;
    let mut games = vec![];
    loop {
        if max_t[0] == Some(0) {
            break;
        }
        assert!(playing[0]);
        let players = max_t.iter().copied().enumerate().filter(|&(i, _)| playing[i]).collect::<Vec<_>>();
        let player_limits = players.iter().filter_map(|&(_, x)| x).collect::<Vec<_>>();
        if players.is_empty() {
            break;
        }
        if player_limits.is_empty() {
            emitln!(write, "inf");
            return;
        }
        let play_t = *player_limits.iter().min().unwrap();
        total_t += play_t as i64;
        if play_t > 0 {
            games.push((playing.iter().map(|&v| if v { '1' } else { '0' }).join(""), play_t));
        }
        let mut outsider = None;
        for (p, x) in max_t.iter_mut().enumerate() {
            if playing[p] {
                *x = x.map(|y| y - play_t);
                if *x == Some(0) {
                    outsider = Some(p);
                }
            }
        }
        let outsider = outsider.unwrap();
        playing[outsider] = false;
        for p in 0..n {
            if playing[p] {
                let pair = if p < outsider { (p, outsider) } else { (outsider, p) };
                if let Some(&y) = time_diffs.get(&pair) {
                    if let Some(old_max_t) = max_t[p] {
                        max_t[p] = Some(old_max_t.min(y));
                    } else {
                        max_t[p] = Some(y);
                    }
                }
            }
        }
    }
    emitln!(write, total_t, games.len());
    for (s, t) in games {
        emitln!(write, s, t);
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
    use contest_lib_rs::{solution_testing::run_solver, assert_trimmed_eq};

    #[test]
    fn test() {
        // assert_trimmed_eq!(&run_solver(solve, "\
        // 5 4
        // 1 3 2
        // 1 4 2
        // 2 3 1
        // 2 5 1
        // "), "\
        // 4 4
        // 10000 1
        // 10010 1
        // 10100 1
        // 11110 1");
        assert_trimmed_eq!(&run_solver(solve, "\
        5 5
        2 3 10
        3 4 10
        2 4 10
        2 5 100
        1 4 100"), "\
        210 3
        11110 100
        10110 10
        10000 100");
        // assert_trimmed_eq!(&run_solver(solve, ""), "");
    }
}
