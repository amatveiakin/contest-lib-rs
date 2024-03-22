use contest_lib_rs::base_one::BaseOneConversion;
use contest_lib_rs::combinatorics_mod::num_combinations_mod;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::mod_ring::ModNumber;
use contest_lib_rs::num::RingNumber;
use contest_lib_rs::prefix_accumulate::PrefixSum;

type ModNum = ModNumber<998244353>;

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, q] = read.usizes();
    let a = read.vec_i32(n);
    let b = read.vec_i32(n);
    let a = PrefixSum::from_iter(a);
    let b = PrefixSum::from_iter(b);
    let mut sums = vec![ModNum::zero()];
    let bs = b.get(..);
    for i in 0..=(a.get(..) + bs) {
        sums.push(*sums.last().unwrap() + num_combinations_mod(bs, i));
    }
    for _ in 0..q {
        let [l, r] = read.u32s().from1b();
        let m = a.get(l..=r) + b.get(l..=r) - (a.get(..) - a.get(l..=r));
        if m < 0 {
            emit!(write, ModNum::zero());
        } else {
            emit!(write, sums[m as usize] / ModNum::from(2).pow(bs as u32));
        }
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
        2 2
        1 0
        0 2
        2 2
        1 1
        "), "748683265 748683265");
        assert_trimmed_eq!(&run_solver(solve, "\
        4 3
        2 3 4 5
        1 0 7 3
        3 3
        2 3
        1 4
        "), "997756929 273932289 1 ");
        // assert_trimmed_eq!(&run_solver(solve, ""), "");
    }
}
