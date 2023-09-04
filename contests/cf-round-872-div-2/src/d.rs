use contest_lib_rs::{io, emitln};

const M: u32 = 1_000_000_007;


fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn modinverse(a: i64, m: i64) -> Option<i64> {
    let (g, x, _) = egcd(a, m);
    if g != 1 {
        None
    } else {
        Some((x % m + m) % m)
    }
}


fn m_prod(a: u32, b: u32) -> u32 {
    ((a as u64) * (b as u64)).rem_euclid(M as u64) as u32
}

fn m_inverse(a: u32) -> u32 {
    modinverse(a as i64, M as i64).unwrap().try_into().unwrap()
}

fn num_combinations(n: u32, k: u32) -> u32 {
    if k > n {
        return 0;
    }
    (n - k + 1 ..= n).fold(1, m_prod) / (1 ..= k).fold(1, m_prod)
}

// fn dfs(
//     nbrs: &Vec<Vec<usize>>,
//     parent: Option<usize>,
//     v: usize
// ) -> (u32, u32, u32) {
//     let mut internal_paths = 0;
//     let mut parent_child_paths = 0;
//     let mut our_subtree_size = 0;
//     let children_dfs: Vec<_> =
//         nbrs[v]
//         .iter()
//         .filter(|&&u| Some(u) != parent)
//         .map(|&u| dfs(nbrs, Some(v), u))
//         .collect();
//     for &(sub_internal, sub_parent_child, sub_size) in children_dfs.iter() {
//         our_subtree_size += sub_size;
//         internal_paths = (internal_paths + sub_internal + sub_parent_child) % M;
//         parent_child_paths = (parent_child_paths + sub_parent_child + sub_size + 1) % M;
//     }
//     for ch1 in 0..children_dfs.len() {
//         for ch2 in 0..ch1 {
//             let (_, _, subtree_size1) = children_dfs[ch1];
//             let (_, _, subtree_size2) = children_dfs[ch2];
//             internal_paths = (internal_paths + m_prod(subtree_size1, subtree_size2)) % M;  // BAD
//         }
//     }
//     our_subtree_size = our_subtree_size + 1;
//     (internal_paths, parent_child_paths, our_subtree_size)
// }

fn dfs(
    nbrs: &Vec<Vec<usize>>,
    n: u32,
    parent: Option<usize>,
    v: usize
) -> (u32, u32) {
    let children_dfs: Vec<_> =
        nbrs[v]
        .iter()
        .filter(|&&u| Some(u) != parent)
        .map(|&u| dfs(nbrs, n, Some(v), u))
        .collect();
    let children_len: u32 = children_dfs.iter().map(|&(len, _)| len).sum();
    let children_sizes: Vec<_> = children_dfs.iter().map(|&(_, size)| size).collect();
    let total_children_size: u32 = children_sizes.iter().sum();

    let parent_size = n - total_children_size - 1;
    let mut all_sizes = children_sizes;
    all_sizes.push(parent_size);
    let mut my_len = 0;
    for i in 0..all_sizes.len() {
        my_len = (my_len + all_sizes[i]) % M;
        for j in 0..i {
            my_len = (my_len + m_prod(all_sizes[i], all_sizes[j])) % M;
        }
    }

    let total_len = (children_len + my_len) % M;
    let total_size = total_children_size + 1;
    (total_len, total_size)
}


#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let n = read.u32();
    let k = read.u32();
    let mut nbrs = vec![vec![]; n as usize];
    for _ in 0..(n - 1) {
        let u = read.usize();
        let v = read.usize();
        nbrs[u - 1].push(v - 1);
        nbrs[v - 1].push(u - 1);
    }

    let answer = match k {
        1 => 1,
        2 => {
            let (paths, _) = dfs(&nbrs, n, None, 0);
            let total = num_combinations(n, k);
            m_prod(paths, m_inverse(total))
        },
        3 => 1,
        _ => panic!("Not fair!")
    };
    emitln!(write, answer);
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
    use contest_lib_rs::testing::solution_testing::prelude::*;

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve, "\
        4 2
        1 2
        2 3
        3 4
        "), "666666674");
        assert_trimmed_eq!(&run_solver(solve, "\
        5 1
        1 2
        2 3
        3 4
        3 5
        "), "1");
    }
}
