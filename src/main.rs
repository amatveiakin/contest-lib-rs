// TODO: For the compiler: auto wrap each file in `mod`

#![allow(unused_imports)]

pub mod bits;
pub mod io;
pub mod segment_tree;
pub mod u32_index;

use bits::*;
use io::*;
use segment_tree::*;
use u32_index::*;


fn main() {
    println!("Hello, world!");

    let n = read_i32();
    let v = read_vec_i32(n as usize);
    emitln!(n, v);
}
