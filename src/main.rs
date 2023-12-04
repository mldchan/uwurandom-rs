//
// uwurandom.rs
// Written by mldkyt in 2023
// MIT license
//
mod uwurandom;

fn main() {
    let uwu = uwurandom::uwurandom::generate(10000000000);
    println!("{}", uwu);
}
