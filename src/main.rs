//
// uwurandom.rs
// Written by mldkyt in 2023
// MIT license
//
mod uwurandom;

fn main() {
    let uwu = uwurandom::Uwurandom::generate(100000);
    println!("{}", uwu);
}
