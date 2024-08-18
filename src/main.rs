// use rand::Rng;
// use std::cmp::Ordering;
// use std::io;

use therustbook;

fn main() {
    //chapter2::guessgame();
    // chapter8::convert_to_pig_latin("hello world wonderful amazing world");

    // won't compile because it doesn't return Result<T, E>
    //let greeting_file = File::open("hello.txt")?;


    // therustbook::chapter12_io_project::io_project();

    therustbook::chapter16_fearless_concurrency::using_threads_to_run_code_simultaneously();
}