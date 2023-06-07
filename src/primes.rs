thread_local! {
    static PRIMES: std::cell::RefCell<Primes> = std::cell::RefCell::new(Primes::new());
}

pub fn primes() -> PrimesIter {
    PrimesIter { index: 0 }
}

struct Primes {
    values: Vec<u32>,
}

impl Primes {
    fn new() -> Self {
        Primes {
            values: vec![2, 3],
        }
    }
}

pub struct PrimesIter {
    index: usize,
}

impl Iterator for PrimesIter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // Performance note: on my machine the overhead on `thread_local` and `ref_cell` is about 1%
        // when generating primes up to 10'000'000.
        PRIMES.with(|p| {
            let values = &mut p.borrow_mut().values;
            if self.index < values.len() {
                let ret = values[self.index];
                self.index += 1;
                Some(ret)
            } else {
                assert_eq!(self.index, values.len());
                let mut x = values.last().unwrap() + 2;
                'outer: loop {
                    for &p in values.iter() {
                        if x % p == 0 {
                            x += 2;
                            continue 'outer;
                        }
                        if p * p > x {
                            break;
                        }
                    }
                    values.push(x);
                    self.index += 1;
                    return Some(x);
                }
            }
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_primes() {
        let v = primes().take_while(|&x| x < 100).collect::<Vec<_>>();
        assert_eq!(v, vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47,
            53, 59, 61, 67, 71, 73, 79, 83, 89, 97
        ]);
    }
}
