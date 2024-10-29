use contest_lib_rs::io::prelude::*;
use contest_lib_rs::runner::prelude::*;

fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let _ = (read, write);
}

make_multi_solver!(solve(solve_case));

fn main() {
    solver_main(solve);
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
