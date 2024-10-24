// UNFINISHED
#![allow(dead_code)]

use std::ops::Index;

use contest_lib_rs::io::prelude::*;
use contest_lib_rs::iterutils_basic::IterutilsBasic;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Letter {
    A, P,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Del,
    Plus,
}

struct Context {
    a: usize,
    p: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Op {
    letter: Letter,
    kind: Kind,
    idx: usize,
}

impl Letter {
    fn from_ch(ch: char) -> Self {
        match ch {
            'A' => Self::A,
            'P' => Self::P,
            _ => panic!(),
        }
    }

    fn ch(self) -> char {
        match self {
            Self::A => 'A',
            Self::P => 'P',
        }
    }

    fn other(self) -> Self {
        match self {
            Self::A => Self::P,
            Self::P => Self::A,
        }
    }
}

impl Index<Letter> for Context {
    type Output = usize;

    fn index(&self, index: Letter) -> &Self::Output {
        match index {
            Letter::A => &self.a,
            Letter::P => &self.p,
        }
    }
}

impl Op {
    fn new(letter: Letter, kind: Kind, idx: usize) -> Self {
        Self { letter, kind, idx }
    }
}

fn letters_to_string(ops: &[Letter]) -> String {
    ops.iter().map(|op| op.ch()).join("")
}
fn string_to_letters(s: &str) -> Vec<Letter> {
    s.chars().map(Letter::from_ch).collect()
}

fn apply_one(ctx: &Context, op: Op, s: &[Letter]) -> Vec<Letter> {
    let x = op.letter;
    let y = op.letter.other();
    match op.kind {
        Kind::Del => {
            for i in 0..ctx[x] {
                assert_eq!(s[op.idx + i], x);
            }
            [&s[..op.idx], &s[op.idx + ctx[x]..]].concat()
        },
        Kind::Plus => {
            assert_eq!(s[op.idx], y);
            [&s[..op.idx], &[x, y, x], &s[op.idx + 1..]].concat()
        }
    }
}

fn apply(ctx: &Context, op: &[Op], s: &[Letter]) -> Vec<Letter> {
    let mut s = s.to_vec();
    // println!("START");
    // println!("{}", letters_to_string(&s));
    for op in op {
        // println!("{op:?}");
        s = apply_one(ctx, *op, &s);
        // println!("{}", letters_to_string(&s));
    }
    s
}

fn plus(letter: Letter, i: usize) -> Op { Op::new(letter, Kind::Plus, i) }
fn del(letter: Letter, i: usize) -> Op { Op::new(letter, Kind::Del, i) }

// fn plus_a(i: usize) -> Op { Op::new(Letter::A, Kind::Plus, i) }
// fn del_a(i: usize) -> Op { Op::new(Letter::A, Kind::Del, i) }
// fn plus_p(i: usize) -> Op { Op::new(Letter::P, Kind::Plus, i) }
// fn del_p(i: usize) -> Op { Op::new(Letter::P, Kind::Del, i) }

fn plus_inv(ctx: &Context, x: Letter, i: usize) -> Vec<Op> {
    plus_inv_s(ctx, x, i, 1)
}

fn del_inv(ctx: &Context, str: &[Letter], x: Letter, i: usize) -> Vec<Op> {
    let y = x.other();
    let mut v = vec![];
    if i > 0 {
        let i = i - 1;
        if str[i] == x {
            v.push(plus(y, i));
            for j in 0..ctx[x] {
                v.push(plus(x, i + 2 + j));
            }
            v.push(del(x, i + 1));
            v.extend(plus_inv(ctx, y, i));
        } else {
            for j in 0..ctx[x] {
                v.push(plus(x, i + j));
            }
            v.push(del(x, i));
        }
    } else {
        if str[0] == x {
            v.push(plus(y, 0));
            for j in 0..ctx[x] {
                v.push(plus(x, j));
            }
            v.push(del(x, ctx[x] + 1));
            v.extend(plus_inv(ctx, y, ctx[x]));
        } else {
            for j in 0..ctx[x] {
                v.push(plus(x, j));
            }
            v.push(del(x, ctx[x] + 1));
        }
    }
    v
}

fn plus_inv_s(ctx: &Context, letter: Letter, i: usize, s: usize) -> Vec<Op> {
    let mut v = vec![];
    for j in s..ctx[letter] {
        v.push(plus(letter, i + j));
    }
    v.push(del(letter, i));
    v.push(del(letter, i + 1));
    v
}

fn inverse(ctx: &Context, ops: &[Op], str: &[Letter]) -> Vec<Op> {
    // println!("INVERTING {}", letters_to_string(str));
    let mut str = str.to_vec();
    let mut v = vec![];
    for op in ops.iter().rev() {
        let op_inv = match op.kind {
            Kind::Del => del_inv(ctx, &str, op.letter, op.idx),
            Kind::Plus => plus_inv(ctx, op.letter, op.idx),
        };
        // println!("{op:?} => {op_inv:?}");
        str = apply(ctx, &op_inv, &str);
        v.extend(op_inv);
    }
    v
}

fn x2_to_y2(ctx: &Context, first: Letter, i: usize) -> Vec<Op> {
    let mut v = vec![];
    v.push(plus(first.other(), i + 1));
    v.extend(plus_inv(ctx, first, i));
    v
}

fn xsy_to_yxxs(ctx: &Context, x: Letter, i: usize, s: usize) -> Vec<Op> {
    let mut v = vec![];
    for j in s..ctx[x] {
        v.push(plus(x, i + j));
    }
    v.push(del(x, i));
    v
}

fn xys_to_yysx(ctx: &Context, x: Letter, i: usize, s: usize) -> Vec<Op> {
    let y = x.other();
    let mut v = vec![];
    for j in 0..ctx[y] - s {
        v.push(plus(y, i + j));
    }
    v.push(del(y, i + ctx[y] - s + 1));
    v
}

fn x4_to_null(ctx: &Context, x: Letter, i: usize) -> Vec<Op> {
    let y = x.other();
    let mut v = x4_to_yy(ctx, x, i);
    v.push(del(y, i));
    v
}

fn x4_to_yy(ctx: &Context, x: Letter, i: usize) -> Vec<Op> {
    let y = x.other();
    let mut v = vec![];
    v.push(plus(y, i));
    v.extend(xsy_to_yxxs(ctx, y, i + 2, 1));
    v.extend(xsy_to_yxxs(ctx, y, i + 3, ctx[y] - 1));
    v.extend(xsy_to_yxxs(ctx, y, i + 4, 1));
    v.push(plus(y, i + 1));
    v.push(plus(x, i + 1));
    v.extend(plus_inv(ctx, y, i));
    v.extend(plus_inv_s(ctx, x, i, 3));
    v
}

fn yy_to_x4(ctx: &Context, y: Letter, i: usize) -> Vec<Op> {
    let x = y.other();
    let mut v = vec![];
    v.push(plus(x, i));
    v.push(plus(x, i + 1));
    v.push(plus(x, i + 2));
    v.push(plus(y, i));
    v.extend(plus_inv(ctx, x, i + 1));
    v.extend(plus_inv(ctx, y, i + 1));
    v.extend(xys_to_yysx(ctx, x, i + 4, ctx[y] - 1));
    v.extend(xys_to_yysx(ctx, x, i + 3, 1));
    v.extend(xys_to_yysx(ctx, x, i + 2, ctx[y] - 1));
    v.extend(plus_inv(ctx, y, i));
    v
}

fn x_to_xy4(ctx: &Context, x: Letter, i: usize) -> Vec<Op> {
    let mut v = x_to_x5(ctx, x, i);
    v.extend(yy_to_x4(ctx, x, i + 1));
    v.extend(x2_to_y2(ctx, x, i + 1));
    v.extend(x2_to_y2(ctx, x, i + 3));
    v
}

fn x_to_x5(ctx: &Context, x: Letter, i: usize) -> Vec<Op> {
    let y = x.other();
    let mut v = vec![];
    for j in 0..ctx[y] {
        v.push(plus(y, i + j));
    }
    v.push(del(y, i));
    v
}

fn xmod4is1_x_to_null(ctx: &Context, x: Letter, i: usize) -> Vec<Op> {
    assert_eq!(ctx[x] % 4, 1);
    let y = x.other();
    let k = (ctx[x] - 1) / 4;
    let mut v = vec![];
    for j in 0..2*k {
        v.push(plus(y, i + j));
    }
    for j in 0..k {
        v.extend(x2_to_y2(ctx, y, i + 2*j));
        v.extend(x2_to_y2(ctx, y, i + 2*k + 1 + 2*j));
    }
    v.push(del(x, i));
    v
}

fn xmod4is1_y_to_yx(ctx: &Context, x: Letter, i: usize) -> Vec<Op> {
    assert_eq!(ctx[x] % 4, 1);
    let mut v = vec![];
    v.push(plus(x, i));
    v.extend(xmod4is1_x_to_null(ctx, x, i));
    v
}

fn xmod4is1_x_to_x2(ctx: &Context, x: Letter, i: usize) -> Vec<Op> {
    assert_eq!(ctx[x] % 4, 1);
    let y = x.other();
    let mut v = vec![];
    v.push(plus(y, i));
    v.extend(xmod4is1_y_to_yx(ctx, x, i + 2));
    v.extend(plus_inv(ctx, y, i));
    v
}

fn xmod4is1_remove_xs(ctx: &Context, x: Letter, str: &[Letter]) -> (bool, Vec<Op>) {
    assert_eq!(ctx[x] % 4, 1);
    let n = str.len();
    let mut v = vec![];
    let mut sh = 0;
    for i in 0..n {
        if str[i] == x {
            if i == n - 1 && sh == i {
                return (true, v);
            }
            v.extend(xmod4is1_x_to_null(ctx, x, i - sh));
            sh += 1;
        }
    }
    (false, v)
}

fn xmod4is1_reduce(ctx: &Context, x: Letter, str: &[Letter]) -> Vec<Op> {
    assert_eq!(ctx[x] % 4, 1);
    let mut str = str.to_vec();
    let y = x.other();
    let mut v = vec![];
    let (x_left, x_removals) = xmod4is1_remove_xs(ctx, x, &str);
    str = apply(ctx, &x_removals, &str);
    v.extend(x_removals);
    if !x_left {
        for i in 0..(str.len() / 2) {
            let ops = x2_to_y2(ctx, y, i * 2);
            str = apply(ctx, &ops, &str);
            v.extend(ops);
        }
    }
    let (_, x_removals) = xmod4is1_remove_xs(ctx, x, &str);
    v.extend(x_removals);
    v
}

#[allow(unused_variables)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [a, p] = read.u32s();
    let s1 = read.word().chars().map(Letter::from_ch).collect_vec();
    let s2 = read.word().chars().map(Letter::from_ch).collect_vec();
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
    fn test_basic_operations() {
        use Kind::*;
        use Letter::*;
        let ctx = Context { a: 3, p: 5 };
        assert_eq!(apply_one(&ctx, Op::new(P, Plus, 0), &[A]), &[P, A, P]);
        assert_eq!(apply_one(&ctx, Op::new(P, Del, 2), &[P, A, P, P, P, P, P, P, A, A]), &[P, A, P, A, A]);
    }

    #[test]
    fn test_inv_operations() {
        use Kind::*;
        use Letter::*;
        let ctx = Context { a: 5, p: 20 };
        assert_eq!(apply(&ctx, &plus_inv(&ctx, A, 0), &[A, P, A]), vec![P]);
        assert_eq!(apply(&ctx, &plus_inv(&ctx, A, 2), &[A, P, A, P, A, A, P, P]), vec![A, P, P, A, P, P]);
        assert_eq!(apply(&ctx, &plus_inv(&ctx, P, 1), &[A, P, A, P, A, A, P, P]), vec![A, A, A, A, P, P]);

        let v = vec![A];
        let ops = del_inv(&ctx, &v, A, 1);
        assert_eq!(apply(&ctx, &ops, &v), vec![A, A, A, A, A, A]);

        let v = vec![A];
        let ops = del_inv(&ctx, &v, A, 0);
        assert_eq!(apply(&ctx, &ops, &v), vec![A, A, A, A, A, A]);

        let v = vec![A];
        let ops = inverse(&ctx, &[del(A, 0)], &v);
        assert_eq!(apply(&ctx, &ops, &v), vec![A, A, A, A, A, A]);

        let v = vec![P];
        let ops = del_inv(&ctx, &v, A, 1);
        assert_eq!(apply(&ctx, &ops, &v), vec![P, A, A, A, A, A]);

        let v = vec![P];
        let ops = del_inv(&ctx, &v, A, 0);
        assert_eq!(apply(&ctx, &ops, &v), vec![A, A, A, A, A, P]);
    }

    #[test]
    fn test_complex_operations() {
        use Kind::*;
        use Letter::*;
        // let ctx = Context { a: 5, p: 20 };
        let ctx = Context { a: 5, p: 1 };

        let v = vec![A, A, A, A, A];
        let ops = yy_to_x4(&ctx, A, 0);
        assert_eq!(apply(&ctx, &ops, &v), vec![P, P, P, P]);

        let v = vec![A, A, P, P, A, A];
        let ops = x2_to_y2(&ctx, A, 4);
        assert_eq!(apply(&ctx, &ops, &v), vec![A, A, P, P, P, P]);

        let v = vec![A, A, P, P, A, A];
        let ops = xmod4is1_x_to_null(&ctx, A, 4);
        assert_eq!(apply(&ctx, &ops, &v), vec![A, A, P, P, A]);

        let v = vec![A, A, P, P, A, A];
        let ops = xmod4is1_y_to_yx(&ctx, A, 3);
        assert_eq!(apply(&ctx, &ops, &v), vec![A, A, P, P, A, A, A]);

        let v = vec![A, A, P, P, A, A];
        let ops = xmod4is1_x_to_x2(&ctx, A, 4);
        assert_eq!(apply(&ctx, &ops, &v), vec![A, A, P, P, A, A, A]);

        let v = vec![A, A, P, P, A, A, P, A];
        let (_, ops) = xmod4is1_remove_xs(&ctx, A, &v);
        assert_eq!(apply(&ctx, &ops, &v), vec![P, P, P]);

        let v = vec![A, A, A, A, A];
        let (_, ops) = xmod4is1_remove_xs(&ctx, A, &v);
        assert_eq!(apply(&ctx, &ops, &v), vec![A]);

        let v = vec![A, A, A, A, A, P];
        let (_, ops) = xmod4is1_remove_xs(&ctx, A, &v);
        assert_eq!(apply(&ctx, &ops, &v), vec![P]);

        let v = vec![A, A, P, P, A, A, P, A];
        let ops = xmod4is1_reduce(&ctx, A, &v);
        assert_eq!(apply(&ctx, &ops, &v), vec![P]);

        let v = vec![A, A, P, P, A, A, P, A, P];
        let ops = xmod4is1_reduce(&ctx, A, &v);
        assert_eq!(apply(&ctx, &ops, &v), vec![A]);
    }

    #[test]
    fn test_converts() {
        use Kind::*;
        use Letter::*;
        let ctx = Context { a: 5, p: 20 };
        let v1 = vec![A, A, P, P, A, A, P, A, P];
        let v2 = vec![P, P, A, A, P, A, P, A, A, A, P, A, P, P, A, P];
        let ops1 = xmod4is1_reduce(&ctx, A, &v1);
        let ops2 = xmod4is1_reduce(&ctx, A, &v2);
        let v2p = apply(&ctx, &ops2, &v2);
        let ops2inv = inverse(&ctx, &ops2, &v2p);
        let ops = [&ops1[..], &ops2inv[..]].concat();
        println!("# = {}", ops.len());
        assert_eq!(apply(&ctx, &ops, &v1), v2);
    }

    #[test]
    fn test() {
        // assert_trimmed_eq!(&run_solver(solve, ""), "");
    }
}
