
pub fn what_is_ownership() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s2);
    // println!("{}, world!", s1); // cannot reuse borrowed variable to avoid double free memory allocation bugs


    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    ownage();
    ownership2();
    owner3();
}

fn ownage() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn ownership2() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}

fn owner3() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}


pub fn references_and_borrowing() {
    let s1 = String::from("hello");

    let len = calculate_length_ref(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s1 = String::from("hello");

    change(&mut s1);

    println!("{}", s1);

    

    // Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value. This code that attempts to create two mutable references to s will fail:
    // That is why this would fail
    // let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);

    // As always, we can use curly brackets to create a new scope, allowing for multiple mutable references, just not simultaneous ones:
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;


    // Rust enforces a similar rule for combining mutable and immutable references. This code results in an error:
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    // You cannot borrow as mutable when it is borrowed as immutable

    // println!("{}, {}, and {}", r1, r2, r3);

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);


}

fn calculate_length_ref(s: &String) -> usize {
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.


// Borrowed values are not modifiable. Borrowing is done with taking a reference
// some string -> &String (ptr, len, capacity) -> String
// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

// REFERENCES (BORROWED VALUES) ARE IMMUTABLE


// mutable reference
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}


// Dangling pointers, or pointers to invalid memory or mem. that was given to something else is prevented in Rust
// fn dangle() -> &String { // dangle returns a reference to a String

//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
//   // Danger!

// Solution, return String directly
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

pub fn the_slice_type() {

    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

//     Luckily, Rust has a solution to this problem: string slices.
//     String Slices

//     A string slice is a reference to part of a String, and it looks like this:

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    let s = String::from("hello");

    let slice = &s[0..2];
    let slice = &s[..2]; // equal to one above

    let s = String::from("hello");

    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..]; // equal to one above

    let s = String::from("hello");

    let len = s.len();

    let slice = &s[0..len];
    let slice = &s[..]; // equal to one above

    // let mut s = String::from("hello world");
    //
    // let word = first_word_slice(&s);
    //
    // s.clear(); // error!
    //
    // println!("the first word is: {}", word);

    let s = "Hello, world!"; // String literal, immutable string

    // Defining a function to take a string slice instead of a reference to a String makes our API more general and useful without losing any functionality:
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word_slice(&my_string[0..6]);
    let word = first_word_slice(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word_slice(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word_slice(&my_string_literal[0..6]);
    let word = first_word_slice(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_slice(my_string_literal);

    // other slices
    let a = [1, 2, 3, 4, 5];

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);

}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// &str allow &str and String s (implicit coercions)
fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}