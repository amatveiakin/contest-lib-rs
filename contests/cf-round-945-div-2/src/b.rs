use contest_lib_rs::io::prelude::*;

fn add_bits(b: &mut [i32; 20], sign: i32, mut x: u32) {
    for i in 0..20 {
        if x & 1 == 1 {
            b[i] += sign;
        }
        x >>= 1;
    }
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let a = read.vec_u32(n);
    let mut l = 1;
    let mut r = n;
    while l < r {
        let k = (l + r) / 2;
        let mut ok = true;
        let mut b = [0; 20];
        for i in 0..k {
            add_bits(&mut b, 1, a[i]);
        }
        let or = b.map(|x| x > 0);
        for i in 0..(n - k) {
            add_bits(&mut b, -1, a[i]);
            add_bits(&mut b, 1, a[i + k]);
            if b.map(|x| x > 0) != or {
                ok = false;
                break;
            }
        }
        if ok {
            r = k;
        } else {
            l = k + 1;
        }
    }
    emitln!(write, l);
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
        assert_trimmed_eq!(&run_solver(solve, "\
        7
        1
        0
        3
        2 2 2
        3
        1 0 2
        5
        3 0 1 4 2
        5
        2 0 4 0 2
        7
        0 0 0 0 1 2 4
        8
        0 1 3 2 2 1 0 3
        "), "\
        1
        1
        3
        4
        4
        7
        3");
    }
}
