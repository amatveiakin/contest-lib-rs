use contest_lib_rs::common_divisors::gcd;
use contest_lib_rs::counting_set::CountingSet;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::primes::primes;
use contest_lib_rs::relax::Relax;

// Always irreducible
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rational {
    num: i32,
    den: i32,
}

impl Rational {
    fn fraq(num: i32, den: i32) -> Self {
        assert!(num >= 0 && den > 0);
        let g = gcd(num, den);
        Self { num: num / g, den: den / g }
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Rational {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.num as i64 * other.den as i64).cmp(&(other.num as i64 * self.den as i64))
    }
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let n = read.usize();
    let a1 = read.i32();
    let a2n = read.vec_i32(n - 1);

    let mut best = Rational::fraq(i32::MAX, 1);
    for k in primes() {
        let k = k as usize;
        if k > n {
            break;
        }
        if n % k != 0 {
            continue;
        }

        let mut grsize = vec![0; k];
        for (i, &v) in [a1].iter().chain(a2n.iter()).enumerate() {
            grsize[i % k] += v;
        }
        let mut sizes = CountingSet::from_item_iter(grsize.iter().copied());

        best.relax_min(Rational::fraq(*sizes.last().unwrap(), *sizes.first().unwrap()));
        for i in 0..(n - 1) {
            let left = i % k;
            let right = (i + 1) % k;
            assert!(sizes.remove(grsize[left]));
            assert!(sizes.remove(grsize[right]));
            grsize[left] += -a1 + a2n[i];
            grsize[right] += a1 - a2n[i];
            sizes.push(grsize[left]);
            sizes.push(grsize[right]);
            best.relax_min(Rational::fraq(*sizes.last().unwrap(), *sizes.first().unwrap()));
        }
    }

    emitln!(write, best.num, best.den);
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
        2
        4 1
        2 1 2
        3 10
        4 3"), "\
        1 1
        10 3");
        // assert_trimmed_eq!(&run_solver(solve, ""), "");
    }
}
