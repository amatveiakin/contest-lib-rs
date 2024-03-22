use contest_lib_rs::array_2d::CharArray2DReading;
use contest_lib_rs::bfs::bfs_path;
use contest_lib_rs::bool_ext::BoolExtension;
use contest_lib_rs::directed_graph::DirectedGraph;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::point_2d::Point2D;

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let a = read.char_array2d(2, n);
    let mut g = DirectedGraph::new();
    g.add_vertices(2 * n);
    let vid = |p: Point2D<i32>| (p.y * n as i32 + p.x) as usize;
    for row in 0..2 {
        for col in 0..n {
            let p = Point2D::new(col as i32, row as i32);
            for sh in [[-1, 0], [0, -1], [1, 0], [0, 1]] {
                let sh = Point2D::from_array(sh);
                let q = p + sh;
                if q.y >= 0 && q.y < 2 && q.x >= 0 && q.x < n as i32 {
                    let q = match a[[q.y as usize, q.x as usize]] {
                        '<' => q + Point2D::new(-1, 0),
                        '>' => q + Point2D::new( 1, 0),
                        _ => unreachable!(),
                    };
                    g.add_edge(vid(p), vid(q));
                }
            }
        }
    }
    let path = bfs_path(&g, vid(Point2D::new(0, 0)), vid(Point2D::new(n as i32 - 1, 1)));
    emitln!(write, path.is_some().yesno());
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
        4
        >><<
        >>><
        2
        ><
        ><
        4
        >>><
        >><<
        6
        >><<><
        ><>>><
        "), "\
        YES
        YES
        NO
        YES");
    }
}
