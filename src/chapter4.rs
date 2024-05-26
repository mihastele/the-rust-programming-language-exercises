
pub fn what_is_ownership() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s2);
    // println!("{}, world!", s1); // cannot reuse borrowed variable to avoid double free memory allocation bugs
}