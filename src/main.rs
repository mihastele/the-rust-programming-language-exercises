// use rand::Rng;
// use std::cmp::Ordering;
// use std::io;

pub mod garden;
mod chapter2;
mod chapter3;
mod chapter4;
mod chapter5;
mod chapter6;
mod chapter7;
mod chapter8;
mod chapter9;

fn main() {
    //chapter2::guessgame();
    // chapter8::convert_to_pig_latin("hello world wonderful amazing world");

    // won't compile because it doesn't return Result<T, E>
    //let greeting_file = File::open("hello.txt")?;


    chapter9::recoverable_errors_with_result()
}