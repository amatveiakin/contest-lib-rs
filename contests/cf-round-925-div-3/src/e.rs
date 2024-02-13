use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let m = read.u32();
    let a = read.vec_u32(n);

    let mut nz = 0;
    let mut z = vec![];
    for mut x in a {
        let mut cz = 0;
        while x % 10 == 0 {
            cz += 1;
            x /= 10;
        }
        z.push(cz);
        while x > 0 {
            nz += 1;
            x /= 10;
        }
    }

    z.sort();
    z.reverse();
    let max = nz + z.iter().skip(1).step_by(2).sum::<u32>();
    if max > m {
        emitln!(write, "Sasha");
    } else {
        emitln!(write, "Anna");
    }
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
        9
        2 2
        14 2
        3 5
        9 56 1
        4 10
        1 2007 800 1580
        4 5
        5000 123 30 4
        10 10
        6 4 6 2 3 1 10 9 10 7
        1 1
        6
        1 1
        10
        8 9
        1 2 9 10 10 2 10 2
        4 5
        10 10 10 10
        "), "\
        Sasha
        Anna
        Anna
        Sasha
        Sasha
        Anna
        Anna
        Anna
        Sasha");
    }
}
