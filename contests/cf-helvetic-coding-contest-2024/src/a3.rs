use std::fmt::Debug;

use contest_lib_rs::io::prelude::*;
use contest_lib_rs::linked_list_on_vec::LinkedListOnVec;

#[derive(Debug)]
struct Node {
    left_count: i64,
    right_count: i64,
    seq: String,
}

impl Node {
    fn new(seq: String, count: i64) -> Node {
        Node {
            left_count: count,
            right_count: count,
            seq
        }
    }
}

fn merge_node(a: &mut Node, b: Node) {
    a.right_count = b.right_count;
    a.seq.push_str(&b.seq);
}

fn concat_all(list: &mut LinkedListOnVec<Node>, cur_count: i64) {
    let mut prev = list.head();
    while list.next(prev).is_valid() {
        assert!(prev.is_valid());
        let next = list.next(prev);
        if list[prev].right_count < cur_count || list[next].left_count < cur_count {
            let next_value = list.remove(next);
            merge_node(&mut list[prev], next_value);
        } else {
            prev = next
        }
    }
}

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let s = read.word_as_chars();
    let n = s.len();

    let mut list = LinkedListOnVec::new();
    list.push_back(Node::new("(".to_string(), 0));

    let mut i = 0;
    while s[i] == '(' {
        list.push_back(Node::new("(".to_string(), 0));
        i += 1;
    }

    let mut cur_count = 1;
    while i < n {
        // println!("### {:?}", list);
        let mut cur = list.tail();
        while i < n && cur.is_valid() {
            while i < n && s[i] == '(' {
                list.insert_after(cur, Node::new("(".to_string(), cur_count));
                i += 1;
            }
            let prev = list.prev(cur);
            if !prev.is_valid() {
                break;
            }
            cur = prev;
            list.insert_after(cur, Node::new(")".to_string(), cur_count));
            i += 1;
        }
        concat_all(&mut list, cur_count);
        cur_count += 1;
    }
    let mut ans = list[list.head()].seq.clone();
    ans.pop();
    emitln!(write, ans);
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
        assert_trimmed_eq!(&run_solver(solve, "()(()())"), "(()(()))");
        assert_trimmed_eq!(&run_solver(solve, "(()(()))"), "()(()())");
        assert_trimmed_eq!(&run_solver(solve, "()"), "()");
        // assert_trimmed_eq!(&run_solver(solve, ""), "");
    }
}
