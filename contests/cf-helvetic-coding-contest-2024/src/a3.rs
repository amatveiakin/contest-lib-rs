use std::fmt::Debug;
use std::usize;

use contest_lib_rs::io::prelude::*;

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

fn merge_nodes(a: Node, b: Node) -> Node {
    Node {
        left_count: a.left_count,
        right_count: b.right_count,
        seq: a.seq + &b.seq,
    }
}

struct List<T> {
    data: Vec<Option<T>>,
    head: usize,
    tail: usize,
    prev: Vec<usize>,
    next: Vec<usize>,
}

#[derive(Clone, Copy)]
struct ListIterator {
    index: usize,
}

impl<T> List<T> {
    fn new() -> Self {
        List {
            data: Vec::new(),
            head: usize::MAX,
            tail: usize::MAX,
            prev: Vec::new(),
            next: Vec::new(),
        }
    }

    fn push_back(&mut self, value: T) {
        let new_index = self.data.len();
        self.data.push(Some(value));
        self.prev.push(self.tail);
        if self.tail != usize::MAX {
            self.next[self.tail] = new_index;
        }
        self.next.push(usize::MAX);
        self.tail = new_index;
        if self.head == usize::MAX {
            self.head = new_index;
        }
    }

    fn iter(&self) -> ListIterator {
        ListIterator {
            index: self.head,
        }
    }

    fn tail_iter(&self) -> ListIterator {
        ListIterator {
            index: self.tail,
        }
    }
}

impl ListIterator {
    fn valid(&self) -> bool {
        self.index != usize::MAX
    }

    fn next<T>(&self, list: &List<T>) -> Self {
        ListIterator {
            index: list.next[self.index]
        }
    }

    fn prev<T>(&self, list: &List<T>) -> Self {
        ListIterator {
            index: list.prev[self.index]
        }
    }

    fn get<'a, T>(&self, list: &'a List<T>) -> &'a T {
        list.data[self.index].as_ref().unwrap()
    }

    fn insert_right<T>(&mut self, list: &mut List<T>, value: T) {
        let new_index = list.data.len();
        list.data.push(Some(value));
        list.prev.push(self.index);
        list.next.push(list.next[self.index]);
        list.next[self.index] = new_index;
        list.prev[list.next[new_index]] = new_index;
    }

    fn merge_right<T>(&mut self, list: &mut List<T>, f: impl FnOnce(T, T) -> T) {
        let current = self.index;
        let next = list.next[current];
        let current_value = list.data[current].take().unwrap();
        let next_value = list.data[next].take().unwrap();
        list.data[current] = Some(f(current_value, next_value));
        list.next[current] = list.next[next];
        if list.next[next] != usize::MAX {
            list.prev[list.next[next]] = current;
        }
        if list.tail == next {
            list.tail = current;
        }
    }
}

fn concat_all(list: &mut List<Node>, cur_count: i64) {
    let mut prev = list.iter();
    while prev.next(&list).valid() {
        assert!(prev.valid());
        let next = prev.next(&list);
        if prev.get(&list).right_count < cur_count || next.get(&list).left_count < cur_count {
            prev.merge_right(list, merge_nodes);
        } else {
            prev = next
        }
    }
}

impl<T: Debug> Debug for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut values = vec![];
        let mut iter = self.iter();
        while iter.valid() {
            values.push(iter.get(&self));
            iter = iter.next(&self);
        }
        write!(f, "{:?}", values)
        // writeln!(f, "----------------");
        // writeln!(f, "{:?}", self.data);
        // writeln!(f, "{:?}", self.prev);
        // writeln!(f, "{:?}", self.next);
        // Ok(())
    }
}

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let s = read.word_as_chars();
    let n = s.len();

    let mut list = List::new();
    list.push_back(Node::new("(".to_string(), 0));

    let mut i = 0;
    while s[i] == '(' {
        list.push_back(Node::new("(".to_string(), 0));
        i += 1;
    }

    let mut cur_count = 1;
    while i < n {
        // println!("### {:?}", list);
        let mut cur = list.tail_iter();
        while i < n && cur.valid() {
            while i < n && s[i] == '(' {
                cur.insert_right(&mut list, Node::new("(".to_string(), cur_count));
                i += 1;
            }
            let prev = cur.prev(&list);
            if !prev.valid() {
                break;
            }
            cur = prev;
            cur.insert_right(&mut list, Node::new(")".to_string(), cur_count));
            i += 1;
        }
        concat_all(&mut list, cur_count);
        cur_count += 1;
    }
    let mut ans = list.iter().get(&list).seq.clone();
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
