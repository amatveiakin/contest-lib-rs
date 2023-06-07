use std::cmp::max;
use std::cmp::Ordering;
use std::mem::swap;
use std::mem::replace;

use contest_lib_rs::{io, emitln};


// #[allow(unused_variables)]
// fn do_solve_case(n: usize, a: &[i32]) -> u64 {
//     let mut tb: u64 = 0;
//     for l in 0..n {
//         let al = &a[l..];
//         // let mut all = AvlTreeSet::new();
//         let mut good = vec![];
//         // all.insert(al[0]);
//         good.push(0);
//         for r in 1..al.len() {
//             // all.insert(al[r]);
//             // let rank = all.rank(&al[r]);
//             let rank = r;
//             if rank == r {
//                 good.push(r);
//             } else {
//                 while let Some(&g) = good.last() {
//                     if g <= rank {
//                         break;
//                     }
//                     good.pop();
//                 }
//             }
//             tb += (r + 1 - good.len()) as u64;
//         }
//     }
//     tb
// }

#[allow(unused_variables)]
fn do_solve_case(n: usize, a: &[i32]) -> u64 {
    let mut tb: u64 = 0;
    for l in 0..n {
        let al = &a[l..];
        let mut all = AvlTreeSet::new();
        let mut good = vec![];
        all.insert(al[0]);
        good.push(0);
        for r in 1..al.len() {
            all.insert(al[r]);
            let rank = all.rank(&al[r]);
            if rank == r {
                good.push(r);
            } else {
                while let Some(&g) = good.last() {
                    if g <= rank {
                        break;
                    }
                    good.pop();
                }
            }
            tb += (r + 1 - good.len()) as u64;
        }
    }
    tb
}

#[allow(unused_variables, dead_code)]
fn solve_case<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let n = read.usize();
    let a = read.vec_i32(n);
    let tb = do_solve_case(n, &a);
    emitln!(write, tb);
}

#[allow(unused_variables, dead_code)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut io::Reader<R>, write: &mut W) {
    let t = read.usize();
    for _ in 0..t {
        solve_case(read, write);
    }
}

fn main() {
    // let mut t = AvlTreeSet::<i32>::new();
    // t.insert(1);
    // t.insert(2);
    // eprint!("{:#?}", t);

    // let mut set = AvlTreeSet::new();
    // set.insert(7);
    // set.insert(4);
    // set.insert(1);
    // set.insert(2);
    // set.insert(3);
    // set.insert(5);
    // set.insert(6);
    // set.insert(8);
    // set.insert(0);
    // println!("{}", set.rank(&0));
    // println!("{}", set.rank(&1));
    // println!("{}", set.rank(&2));
    // println!("{}", set.rank(&3));
    // println!("{}", set.rank(&4));
    // println!("{}", set.rank(&5));
    // println!("{}", set.rank(&6));
    // println!("{}", set.rank(&7));
    // println!("{}", set.rank(&8));
    // println!("{}", set.rank(&9));
    // println!("{}", set.rank(&10));

    let n = 5000;
    let a = (0..n).map(|x| (x as i32) * 257 % 10000).collect::<Vec<_>>();
    let tb = do_solve_case(n, &a);
    println!("{}", tb);

    // let mut read = io::Reader::new(std::io::stdin().lock());
    // let mut write = std::io::stdout().lock();
    // solve(&mut read, &mut write);
}


#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use pretty_assertions::assert_eq;
    use contest_lib_rs::{solution_testing::run_solver, assert_trimmed_eq};

    #[test]
    fn test() {
        // assert_trimmed_eq!(&run_solver(solve, "\
        // 5
        // 2
        // 6 4
        // 3
        // 3 10 6
        // 4
        // 4 8 7 2
        // 5
        // 9 8 2 4 6
        // 12
        // 2 6 13 3 15 5 10 8 16 9 11 18
        // "), "\
        // 1
        // 2
        // 8
        // 16
        // 232");
    }
}


type AvlNodeList<T> = Vec<AvlNode<T>>;

#[derive(Debug)]
pub struct AvlNode<T: Ord> {
    pub value: T,
    pub left: Option<usize>,
    pub right: Option<usize>,
    pub height: usize,
    pub size: usize,
}

impl<'a, T: 'a + Ord> AvlNode<T> {
    pub fn left_node<'t>(&self, nodes: &'t AvlNodeList<T>) -> Option<&'t AvlNode<T>> {
        self.left.map(|left| &nodes[left])
    }
    pub fn left_node_mut<'t>(&self, nodes: &'t mut AvlNodeList<T>) -> Option<&'t mut AvlNode<T>> {
        self.left.map(|left| &mut nodes[left])
    }

    pub fn right_node<'t>(&self, nodes: &'t AvlNodeList<T>) -> Option<&'t AvlNode<T>> {
        self.right.map(|right| &nodes[right])
    }
    pub fn right_node_mut<'t>(&self, nodes: &'t mut AvlNodeList<T>) -> Option<&'t mut AvlNode<T>> {
        self.right.map(|right| &mut nodes[right])
    }

    pub fn left_height(&self, nodes: &AvlNodeList<T>) -> usize {
        self.left_node(nodes).as_ref().map_or(0, |left| left.height)
    }

    pub fn right_height(&self, nodes: &AvlNodeList<T>) -> usize {
        self.right_node(nodes).as_ref().map_or(0, |right| right.height)
    }

    pub fn left_size(&self, nodes: &AvlNodeList<T>) -> usize {
        self.left_node(nodes).as_ref().map_or(0, |left| left.size)
    }

    pub fn right_size(&self, nodes: &AvlNodeList<T>) -> usize {
        self.right_node(nodes).as_ref().map_or(0, |right| right.size)
    }

    pub fn update_height_and_size(&mut self, nodes: &AvlNodeList<T>) {
        self.height = 1 + max(self.left_height(nodes), self.right_height(nodes));
        self.size = 1 + self.left_size(nodes) + self.right_size(nodes);
    }

    pub fn balance_factor(&self, nodes: &AvlNodeList<T>) -> i8 {
        let left_height = self.left_height(nodes);
        let right_height = self.right_height(nodes);

        if left_height >= right_height {
            (left_height - right_height) as i8
        } else {
            -((right_height - left_height) as i8)
        }
    }

    pub fn rotate_left(&mut self, nodes: &mut AvlNodeList<T>) -> bool {
        if self.right.is_none() {
            return false;
        }

        let right_node = self.right_node_mut(nodes).unwrap();
        let right_left_tree = right_node.left.take();
        let right_right_tree = right_node.right.take();

        let new_left_tree = replace(&mut self.right, right_right_tree);
        swap(&mut self.value, &mut nodes[new_left_tree.unwrap()].value);
        let left_tree = self.left.take();

        let new_left_node = new_left_tree.unwrap();
        nodes[new_left_node].right = right_left_tree;
        nodes[new_left_node].left = left_tree;
        self.left = new_left_tree;

        let nodes2 = unsafe { &mut *(nodes as *mut AvlNodeList<T>) };
        if let Some(node) = self.left_node_mut(nodes) {
            node.update_height_and_size(nodes2);
        }

        self.update_height_and_size(nodes);

        true
    }

    pub fn rotate_right(&mut self, nodes: &mut AvlNodeList<T>) -> bool {
        if self.left.is_none() {
            return false;
        }

        let left_node = self.left_node_mut(nodes).unwrap();
        let left_right_tree = left_node.right.take();
        let left_left_tree = left_node.left.take();

        let new_right_tree = replace(&mut self.left, left_left_tree);
        swap(&mut self.value, &mut nodes[new_right_tree.unwrap()].value);
        let right_tree = self.right.take();

        let new_right_node = new_right_tree.unwrap();
        nodes[new_right_node].left = left_right_tree;
        nodes[new_right_node].right = right_tree;
        self.right = new_right_tree;

        let nodes2 = unsafe { &mut *(nodes as *mut AvlNodeList<T>) };
        if let Some(node) = self.right_node_mut(nodes) {
            node.update_height_and_size(nodes2);
        }

        self.update_height_and_size(nodes);

        true
    }

    pub fn rebalance(&mut self, nodes: &mut AvlNodeList<T>) -> bool {
        match self.balance_factor(nodes) {
            -2 => {
                let nodes2 = unsafe { &mut *(nodes as *mut AvlNodeList<T>) };
                let right_node = self.right_node_mut(nodes2).unwrap();

                if right_node.balance_factor(nodes) == 1 {
                    right_node.rotate_right(nodes);
                }

                self.rotate_left(nodes);

                true
            }

            2 => {
                let nodes2 = unsafe { &mut *(nodes as *mut AvlNodeList<T>) };
                let left_node = self.left_node_mut(nodes2).unwrap();

                if left_node.balance_factor(nodes) == -1 {
                    left_node.rotate_left(nodes);
                }

                self.rotate_right(nodes);

                true
            }
            _ => false,
        }
    }
}


#[derive(Debug)]
pub struct AvlTreeSet<T: Ord> {
    root: Option<usize>,
    nodes: AvlNodeList<T>,
}

impl<'a, T: 'a + Ord> AvlTreeSet<T> {
    pub fn new() -> Self {
        Self { root: None, nodes: Vec::new() }
    }

    pub fn insert(&mut self, value: T) -> bool {
        let mut prev_ptrs = Vec::<usize>::new();
        let mut current_tree = &mut self.root;

        let nodes2 = unsafe { &mut *(&mut self.nodes as *mut AvlNodeList<T>) };
        while let Some(current_node) = *current_tree {
            prev_ptrs.push(current_node);

            match self.nodes[current_node].value.cmp(&value) {
                Ordering::Less => current_tree = &mut self.nodes[current_node].right,
                Ordering::Equal => {
                    return false;
                }
                Ordering::Greater => current_tree = &mut self.nodes[current_node].left,
            }
        }

        nodes2.push(AvlNode {
            value,
            left: None,
            right: None,
            height: 1,
            size: 1,
        });
        *current_tree = Some(nodes2.len() - 1);

        for node_ptr in prev_ptrs.into_iter().rev() {
            let nodes3 = unsafe { &mut *(&mut self.nodes as *mut AvlNodeList<T>) };
            let node = &mut nodes3[node_ptr];
            node.update_height_and_size(&mut self.nodes);
            node.rebalance(&mut self.nodes);
        }

        true
    }

    // pub fn contains(&self, value: &T) -> bool {
    //     let mut current_tree = &self.root;

    //     while let Some(current_node) = current_tree {
    //         match current_node.value.cmp(&value) {
    //             Ordering::Less => {
    //                 current_tree = &current_node.right;
    //             }
    //             Ordering::Equal => {
    //                 return true;
    //             }
    //             Ordering::Greater => {
    //                 current_tree = &current_node.left;
    //             }
    //         };
    //     }

    //     false
    // }

    // pub fn get(&self, value: &T) -> Option<&T> {
    //     let mut current_tree = &self.root;

    //     while let Some(current_node) = current_tree {
    //         match current_node.value.cmp(&value) {
    //             Ordering::Less => {
    //                 current_tree = &current_node.right;
    //             }
    //             Ordering::Equal => {
    //                 return Some(&current_node.value);
    //             }
    //             Ordering::Greater => {
    //                 current_tree = &current_node.left;
    //             }
    //         };
    //     }

    //     None
    // }

    pub fn rank(&self, value: &T) -> usize {
        let mut current_tree = self.root;
        let mut rank = 0;

        while let Some(current_node) = current_tree {
            match self.nodes[current_node].value.cmp(&value) {
                Ordering::Less => {
                    rank += 1 + self.nodes[current_node].left_size(&self.nodes);
                    current_tree = self.nodes[current_node].right;
                }
                Ordering::Equal => {
                    rank += self.nodes[current_node].left_size(&self.nodes);
                    return rank;
                }
                Ordering::Greater => {
                    current_tree = self.nodes[current_node].left;
                }
            };
        }

        rank
    }

    pub fn clear(&mut self) {
        self.root.take();
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }
}
