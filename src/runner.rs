use crate::io::Reader;

pub mod prelude {
    pub use crate::make_multi_solver;
    pub use super::solver_main;
}


pub fn solver_main<'c, 'd>(
    solve: impl for<'a, 'b> Fn(
        &'a mut Reader<std::io::StdinLock<'c>>,
        &'b mut std::io::BufWriter<std::io::StdoutLock<'d>>),
) {
    let mut read = Reader::new(std::io::stdin().lock());
    let mut write = std::io::BufWriter::new(std::io::stdout().lock());
    solve(&mut read, &mut write);
}

#[macro_export]
macro_rules! make_multi_solver {
    ($solve:ident($solve_case:ident)) => {
        fn $solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
            let t = read.usize();
            for _ in 0..t {
                $solve_case(read, write);
            }
        }
    };
}
