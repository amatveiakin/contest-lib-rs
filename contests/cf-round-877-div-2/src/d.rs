use contest_lib_rs::{io, emitln, bitset::Bitset};

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let n = read.u32();
    let q = read.u32();
    let s = read.word();

    if n % 2 == 1 {
        for _ in 0..q {
            emitln!(write, "NO");
        }
        return;
    }

    let mut b = Bitset::new(n as usize);
    for (i, c) in s.chars().enumerate() {
        if (c == ')') == (i % 2 == 0) {
            b.set(i, true);
        }
    }
    // println!("b = {b:?}");
    for _ in 0..q {
        let k = read.usize() - 1;
        b.set(k, !b.get(k));
        // println!("k = {k}, b = {b:?}");
        if b.none() {
            emitln!(write, "YES");
            continue;
        }
        let mut prefix = 0;
        while prefix < b.words().len() - 1 && b.words()[prefix] == 0 {
            prefix += 1;
        }
        let mut suffix = b.words().len() - 1;
        while suffix > 1 && b.words()[suffix] == 0 {
            suffix -= 1;
        }
        prefix *= 64;
        suffix *= 64;
        suffix += 64;
        while prefix < suffix && !b.get(prefix) && !b.get(prefix + 1) {
            prefix += 2;
        }
        while suffix > prefix && (suffix > b.len() || (!b.get(suffix - 1) && !b.get(suffix - 2))) {
            suffix -= 2;
        }
        if suffix <= prefix {
            emitln!(write, "YES");
            continue;
        }
        if !b.get(prefix) && b.get(prefix + 1) && !b.get(suffix - 1) && b.get(suffix - 2) {
            emitln!(write, "YES");
        } else {
            emitln!(write, "NO");
        }
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
    // use pretty_assertions::assert_eq;
    use contest_lib_rs::testing::solution_testing::prelude::*;

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve, "\
        10 9
        (())()()))
        9
        7
        2
        6
        3
        6
        7
        4
        8
        "), "\
        YES
        YES
        NO
        NO
        YES
        NO
        YES
        NO
        NO");
        assert_trimmed_eq!(&run_solver(solve, "\
        3 2
        (()
        2
        3"), "\
        NO
        NO");
    }
}
