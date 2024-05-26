use std::io;

pub fn variables_and_mutability() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");
    println!("The value of x is: {x}");

    let spaces = "   ";
    let spaces = spaces.len();

    println!("Spaces: {spaces}");

    // this will throw an error since mut disallows changing the type, while shadowing does
    // let mut mutspaces = "   ";
    // mutspaces = spaces.len();

    

}

pub fn data_types() {
    // let guess = "42".parse().expect("Not a number!"); // Would fail since it cannot infer type
    let guess: u64 = "42".parse().expect("Not a number!");


    // SCALAR AND COMPOUND TYPES

    // SCALAR TYPES: integers, floats, booleans, characters
    // signed ints: i8, i16, i32, i64, i128, isize, unsigned ints: u8, u16, u32, u64, u128, usize
    // you can use underscores to improve readability in numbers like 1_000_000 for 1 million, 0xff is hex representation, 0o77 is octal, 0b1111_0000 binary and b'A' byte (u8 only)

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32, all floating points are signed
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    let t = true;
    let f: bool = false; // with explicit type annotation

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // COMPOUND TYPES: tuples, arrays

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 6.4, 1);

    // tuple destructuring using pattern matching
    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    // Acessing tuple elements

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // Arrays
    // arrays are fixed length, and all elements must be the same type

    let a = [1, 2, 3, 4, 5];

    // Vectors are more flexible arrays, but are not as efficient. You should use arrays when you know the size and know that elements will not change
    let months = ["January", "February", "March", "April", "May", "June", "July",
            "August", "September", "October", "November", "December"];

    // define array with type annotation, you provide the type and length
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5]; // [3, 3, 3, 3, 3]

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

}

pub fn functions() {
    another_function(-22);
    print_labeled_measurement(5, 'h');

    // curly brace is an expression
    // expressions return a value
    // statements don't return a value
    // functions are statement
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    let x = five();

    print!("Five {x}");
}

fn five() -> i32 {
    5
}

// Semicolon would fail in this function because it has a semicolon, if we want to return last expression without return, we must not use semicolon
// fn five() -> i32 {
//     5;
// }

fn another_function(x: i32) {
    println!("The value of x is {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

pub fn control_flow() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 3;

    // This would fail, the type in if condition must be bool, unlike javascript
    // if number {
    //     println!("number was three");
    // }

    if number != 0 {
        println!("number was three");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // let number = if condition { 5 } else { "six" }; // fails due to type mismatch

    // loop {
    //     println!("again!");
    // }

    // One of the uses of a loop is to retry an operation you know might fail, such as checking whether a
    // thread has completed its job. You might also need to pass the result of that operation out of the
    // loop to the rest of your code. To do this, you can add the value you want returned after the break 
    // expression you use to stop the loop; that value will be returned out of the loop so you can use it, as shown here:

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");

    let _i = 'ddd: loop {
        println!("i = 55");
        break 'ddd 55
    };

    // Conditional loops

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");


    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // for

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

}