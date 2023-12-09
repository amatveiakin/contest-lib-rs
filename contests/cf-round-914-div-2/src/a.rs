use contest_lib_rs::io::prelude::*;
use contest_lib_rs::point_2d::{PointReading, Point2D};

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [a, b] = read.i32s();
    let k = read.p2_i32();
    let q = read.p2_i32();
    let mut moves = vec![
        Point2D::new( a,  b),
        Point2D::new( a, -b),
        Point2D::new(-a,  b),
        Point2D::new(-a, -b),
    ];
    if a != b {
        moves.extend_from_slice(&[
            Point2D::new( b,  a),
            Point2D::new( b, -a),
            Point2D::new(-b,  a),
            Point2D::new(-b, -a),
        ]);
    }
    let mut ans = 0;
    'outer: for &m1 in &moves {
        for &m2 in &moves {
            let dest = k + m1 + m2;
            if dest != k && dest == q {
                ans += 1;
                continue 'outer;
            }
        }
    }
    emitln!(write, ans);
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
        4
        2 1
        0 0
        3 3
        1 1
        3 1
        1 3
        4 4
        0 0
        8 0
        4 2
        1 4
        3 4
        "), "\
        2
        1
        2
        0");
    }
}
