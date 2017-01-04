extern crate ci_test;
extern crate redis;

use ci_test::{add_two, fetch_an_integer};

fn main() {
    println!("{:?}", add_two(2));
    let result = fetch_an_integer().unwrap();
    println!("{:?}", result);
}