use contest_lib_rs::io::prelude::*;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();

    let mut a = 0;
    for i in 1..n {
        emitln!(write, "?", i, i, a, a);
        write.flush().unwrap();
        match read.word().as_str() {
            "<" => {}
            "=" => unreachable!(),
            ">" => a = i,
            _ => unreachable!(),
        }
    }

    let mut b = 0;
    for i in 1..n {
        emitln!(write, "?", a, i, a, b);
        write.flush().unwrap();
        match read.word().as_str() {
            "<" => {}
            "=" => {
                emitln!(write, "?", i, i, b, b);
                write.flush().unwrap();
                match read.word().as_str() {
                    "<" => b = i,
                    "=" => unreachable!(),
                    ">" => {},
                    _ => unreachable!(),
                }
            }
            ">" => b = i,
            _ => unreachable!(),
        }
    }

    emitln!(write, "!", a, b);
    write.flush().unwrap();
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
        // assert_trimmed_eq!(&run_solver(solve, ""), "");
    }
}
