use contest_lib_rs::io::prelude::*;
use contest_lib_rs::mod_ring::ModNumber;
use contest_lib_rs::ivec;

type M = ModNumber<998244353>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Z {
    num0: M,
    num1: M,
    inv: M,
}

impl Z {
    fn new() -> Self {
        Self {
            num0: M::from(0),
            num1: M::from(0),
            inv: M::from(0),
        }
    }
}

fn dfs(l: &Vec<Vec<u32>>, w: &Vec<Vec<u32>>, i: usize, z: &mut Z, cache: &mut Vec<Option<Z>>) {
    // for(int j = 1; j <= S[i]; j++) {
    //     Z.push_back(W[i][j]); // add the integer in the edge to the end of Z
    //     dfs(L[i][j]); // recurse to the next vertex
    // }
    if let Some(iz) = cache[i] {
        z.inv += iz.inv + z.num1 * iz.num0;
        z.num0 += iz.num0;
        z.num1 += iz.num1;
        return;
    }
    let zold = *z;
    for j in 0..l[i].len() {
        if w[i][j] == 1 {
            z.num1 += M::from(1);
        } else {
            z.num0 += M::from(1);
            z.inv += z.num1;
        }
        dfs(l, w, l[i][j] as usize, z, cache);
    }
    cache[i] = Some(Z {
        num0: z.num0 - zold.num0,
        num1: z.num1 - zold.num1,
        inv: z.inv - zold.inv - (z.num0 - zold.num0) * zold.num1,
    });
}

fn solve_impl(l: &Vec<Vec<u32>>, w: &Vec<Vec<u32>>) -> M {
    let mut cache = ivec![None; l.len()];
    let mut z = Z::new();
    dfs(&l, &w, 0, &mut z, &mut cache);
    z.inv
}

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let mut l = vec![vec![]; n];
    let mut w = vec![vec![]; n];
    for i in 0..n {
        let s = read.usize();
        for _ in 0..s {
            l[i].push(read.u32() - 1);
            w[i].push(read.u32());
        }
    }

    let answer = solve_impl(&l, &w);
    emitln!(write, answer);
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
    use contest_lib_rs::rand;
    use pretty_assertions::assert_eq;
    use contest_lib_rs::testing::solution_testing::prelude::*;

    fn dfs_naive(l: &Vec<Vec<u32>>, w: &Vec<Vec<u32>>, i: usize, z: &mut Vec<u32>) {
        for j in 0..l[i].len() {
            z.push(w[i][j]);
            dfs_naive(l, w, l[i][j] as usize, z);
        }
    }
    fn solve_naive(l: &Vec<Vec<u32>>, w: &Vec<Vec<u32>>) -> M {
        let mut z = vec![];
        dfs_naive(&l, &w, 0, &mut z);
        let mut inv = M::from(0);
        let mut num1 = 0;
        for v in z {
            if v == 1 {
                num1 += 1;
            } else {
                inv += M::from(num1);
            }
        }
        inv
    }

    #[test]
    fn test_problem_statement() {
        assert_trimmed_eq!(&run_solver(solve, "\
        5
        2
        4 0
        3 1
        0
        1
        2 0
        2
        3 1
        5 1
        0"), "4");
    }

    #[test]
    fn test_my() {
        // let l = vec![
        //     vec![1, 2, 3, 4],
        //     vec![2, 3, 4],
        //     vec![3, 4],
        //     vec![4],
        //     vec![],
        // ];

        // let l = vec![
        //     vec![1, 2],
        //     vec![3, 4],
        //     vec![3, 4],
        //     vec![5],
        //     vec![5],
        //     vec![],
        // ];

        // let mut l = vec![vec![]; 200_000];
        let mut l = vec![vec![]; 10];
        let n = l.len();

        // let w = vec![
        //     vec![0, 1, 0, 1],
        //     vec![0, 1, 0],
        //     vec![0, 1],
        //     vec![0],
        //     vec![],
        // ];

        for i in 0..n {
            for j in (i + 1)..n {
                l[i].push(j as u32);
            }
        }

        for _ in 0..1 {
            // let n = rand::random::<usize>() % 5 + 5;
            // let mut l = vec![vec![]; n];
            // let mut w = vec![vec![]; n];
            // for i in 0..n {
            //     let s = rand::random::<usize>() % 5;
            //     for _ in 0..s {
            //         l[i].push(rand::random::<u32>() % (i as u32));  ???
            //     }
            //     for _ in 0..l[i].len() {
            //         w[i].push(rand::random::<u32>() % 2);
            //     }
            // }

            let mut w = vec![vec![]; n];
            for i in 0..n {
                for _ in 0..l[i].len() {
                    w[i].push(rand::random::<u32>() % 2);
                }
            }
            assert_eq!(solve_impl(&l, &w), solve_naive(&l, &w));
        }
    }
}
