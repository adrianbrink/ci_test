extern crate ci_test;
extern crate redis;
extern crate postgres;

use ci_test::{add_two, fetch_an_integer};
use postgres::{Connection, TlsMode};

fn main() {
    let conn = Connection::connect("postgres://postgres@postgres", TlsMode::None).unwrap();
    loop {
        println!("{:?}", add_two(2));
        let result = fetch_an_integer().unwrap();
        println!("{:?}", result);
    }
}