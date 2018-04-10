use std::io;
use std::io::prelude::*;


fn main() {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        for word in line.unwrap().trim().split_whitespace() {
        println!("{:?}", word.to_string().parse::<i32>().unwrap() );

        }
    }
}
