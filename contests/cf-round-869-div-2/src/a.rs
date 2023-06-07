use contest_lib_rs::{io, emitln};

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let t = read.u32();
    for _ in 0..t {
        let n = read.usize();
        let k = read.usize();
        let opinions = (0..n).map(|_| read.word().chars().map(|ch| {
            match ch {
                '+' => true,
                '-' => false,
                _ => unreachable!(),
            }
        }).collect::<Vec<_>>()).collect::<Vec<_>>();
        let mut keep = vec![true; n];
        for i in 0..k {
            for j in 1..n {
                if opinions[j][i] != opinions[0][i] {
                    keep[j] = false;
                }
            }
        }
        let answer = keep.iter().filter(|&&x| x).count();
        emitln!(write, answer);
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
        assert_trimmed_eq!(&run_solver(solve,
"
        5
        2 2
        ++
        +-
        1 3
        +-+
        4 1
        +
        -
        -
        +
        5 4
        ++++
        +--+
        ++-+
        +-++
        ++++
        4 2
        ++
        --
        --
        -+
"),
"\
        1
        1
        2
        2
        1
");
    }
}
