use contest_lib_rs::base_one::{Base, BaseOneConversion};
use contest_lib_rs::counting_set::CountingSet;
use contest_lib_rs::genealogy::{lowest_common_ancestor, DfsNumbering, BinaryLifting};
use contest_lib_rs::graph::VertexId;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_basic::IterutilsBasic;
use contest_lib_rs::iterutils_zip_eq::IterutilsZipEq;
use contest_lib_rs::relax::Relax;
use contest_lib_rs::tree::Tree;

const N_BITS: usize = 30;

fn fill_parent_with_bits(v: VertexId, t: &Tree<(), ()>, a: &[u32], p1: &mut Vec<[Option<u32>; N_BITS]>) {
    let pbits = t.parent(v).map_or([None; N_BITS], |p| p1[p]);
    for i in 0..N_BITS {
        p1[v][i] = if a[v] & (1 << i) != 0 {
            Some(v as u32)
        } else {
            pbits[i]
        };
    }
    for ch in t.children(v) {
        fill_parent_with_bits(ch, t, a, p1);
    }
}

fn farthest_parent_with_bit(
    mut v: VertexId, root: VertexId, dn: &DfsNumbering, bl: &BinaryLifting,
    b: usize, p1: &Vec<[Option<u32>; N_BITS]>
) -> VertexId {
    assert!(dn.is_ancestor(root, v));
    assert!(dn.is_ancestor(root, p1[v][b].unwrap() as VertexId));
    let mut jump = usize::MAX;
    'outer: loop {
        jump.relax_min(bl[v].len());
        while jump > 0 {
            jump -= 1;
            let p = bl[v][jump] as VertexId;
            if !dn.is_ancestor(root, p) {
                continue;
            }
            let Some(pp1) = p1[p][b] else {
                continue;
            };
            if !dn.is_ancestor(root, pp1 as VertexId) {
                continue;
            }
            v = p;
            continue 'outer;
        }
        break;
    }
    assert!(p1[v][b] == Some(v as u32));
    v
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let a = read.vec_u32(n);
    let t = Tree::from_read_edges(n, Base::ONE, read).unwrap();

    let mut p1 = vec![[None; N_BITS]; n];
    let dn = DfsNumbering::new(&t);
    let bl = BinaryLifting::new(&t);
    fill_parent_with_bits(0, &t, &a, &mut p1);

    let q = read.usize();
    for _ in 0..q {
        let [mut u, mut v] = read.usizes().from1b();
        if dn[v].0 < dn[u].0 {
            std::mem::swap(&mut u, &mut v);
        }
        let cp = lowest_common_ancestor(&dn, &bl, u, v);
        let mut s_ends = CountingSet::new();
        for b in 0..N_BITS {
            let up1 = p1[u][b].map(|p| p as VertexId).filter(|&p| dn.is_ancestor(cp, p));
            let vp1 = p1[v][b].map(|p| p as VertexId).filter(|&p| dn.is_ancestor(cp, p));
            let (from, to) = match (up1, vp1) {
                (Some(up1), Some(vp1)) => (
                    (-1, up1),
                    ( 1, vp1),
                ),
                (Some(up1), None) => (
                    (-1, up1),
                    (-1, farthest_parent_with_bit(up1, cp, &dn, &bl, b, &p1)),
                ),
                (None, Some(vp1)) => (
                    ( 1, farthest_parent_with_bit(vp1, cp, &dn, &bl, b, &p1)),
                    ( 1, vp1),
                ),
                (None, None) => continue,
            };
            let finish_vertex = |(subtree, w) : (i32, VertexId)| {
                if w == cp {
                    (0, 0)
                } else {
                    match subtree {
                        -1 => (-1, dn[w].1),
                        1  => (1,  dn[w].0),
                        _ => unreachable!(),
                    }
                }
            };
            let from = finish_vertex(from);
            let to = finish_vertex(to);
            s_ends.push((from, false));
            s_ends.push((to, true));
        }
        let s_ends = s_ends.item_iter().copied().collect_vec();

        let mut left = vec![];
        let mut right = vec![];
        {
            let mut cnt = 0;
            for &(_, is_end) in s_ends.iter() {
                if !is_end {
                    cnt += 1;
                }
                left.push(cnt);
            }
        }
        {
            let mut cnt = 0;
            for &(_, is_end) in s_ends.iter().rev() {
                if is_end {
                    cnt += 1;
                }
                right.push(cnt);
            }
            right.reverse();
        }
        assert_eq!(left.len(), right.len());

        let mut ans = 0;
        for (l, r) in left.iter().zip_eq(right.iter()) {
            ans.relax_max(l + r);
        }

        emit!(write, ans);
    }
    emitln!(write);
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
        3
        4
        1 2 3 4
        1 3
        1 2
        1 4
        3
        1 1
        1 3
        1 4
        3
        7 6 3
        3 1
        2 1
        4
        1 1
        1 2
        1 3
        2 3
        1
        4
        1
        1 1
        "), "\
        2 4 3
        6 6 6 6
        2 ");

        assert_trimmed_eq!(&run_solver(solve, "\
        3
        7
        4 7 7 4 10 8 10
        6 1
        3 1
        2 1
        7 4
        1 5
        4 2
        4
        7 5
        2 3
        4 5
        2 5
        6
        9 5 6 2 4 6
        5 1
        2 1
        1 6
        4 3
        1 3
        4
        6 1
        1 4
        4 3
        3 5
        7
        5 1 3 7 5 1 6
        2 1
        5 4
        2 3
        3 4
        7 6
        6 3
        2
        4 2
        7 7
        "), "\
        8 6 7 7
        6 6 4 7
        6 4 ");

        assert_trimmed_eq!(&run_solver(solve, "\
        1
        7
        6 8 7 2 5 8 7
        2 1
        3 2
        4 3
        4 6
        4 5
        6 7
        4
        1 5
        6 7
        4 5
        1 4
        "), "\
        7 7 5 7 ");
    }
}
