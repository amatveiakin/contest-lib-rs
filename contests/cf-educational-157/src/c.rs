use contest_lib_rs::counting_set::CountingSet;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_basic::IterutilsBasic;

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    const L: usize = 5;
    let n = read.usize();
    let ws = read.vec_word(n).into_iter()
        .map(|w| w.chars().map(|ch| ch.to_digit(10).unwrap() as u32).collect_vec())
        .collect_vec();
    let wis = ws.iter().map(|w| { let mut w = w.clone(); w.reverse(); w }).collect_vec();
    let mut lc = CountingSet::new();
    for w in &ws {
        let s: u32 = w.iter().sum();
        lc.push((w.len(), s));
    }
    let mut ans: u64 = 0;
    for hl in 1..=L {
        let l = hl * 2;
        for w in &ws {
            let wl = w.len();
            if hl <= wl && wl < l {
                let s: u32 = w[0..hl].iter().sum();
                let sp: u32 = w[hl..].iter().sum();
                ans += lc.count(&(l - wl, s - sp)) as u64;
            }
        }
        for w in &wis {
            let wl = w.len();
            if hl < wl && wl < l {
                let s: u32 = w[0..hl].iter().sum();
                let sp: u32 = w[hl..].iter().sum();
                ans += lc.count(&(l - wl, s - sp)) as u64;
            }
        }
    }
    emitln!(write, ans);
}

fn main() {
    let mut read = Reader::new(std::io::stdin().lock());
    let mut write = std::io::BufWriter::new(std::io::stdout().lock());
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
        assert_trimmed_eq!(&run_solver(solve, "10  5 93746 59 3746 593 746 5937 46 59374 6"), "20");
        assert_trimmed_eq!(&run_solver(solve, "5  2 22 222 2222 22222"), "13");
        assert_trimmed_eq!(&run_solver(solve, "3  1 1 1"), "9");
        // assert_trimmed_eq!(&run_solver(solve, ""), "");
    }
}
