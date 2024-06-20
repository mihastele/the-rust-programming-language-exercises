use std::fmt::{Debug, Display};

pub fn generic_data_types() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    // let wont_work = Point { x: 5, y: 4.0 };

    let both_integer = Point2 { x: 5, y: 4 };
    let both_float = Point2 { x: 5.0, y: 4.0 };
    let integer_and_float = Point2 { x: 5, y: 4.0 };

    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());

    let p1 = PointMx { x: 5, y: 10.4 };
    let p2 = PointMx { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

//same type point
// struct Point<T> {
//     x: T,
//     y: T,
// }

struct Point2<T, U> {
    x: T,
    y: U,
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// specific type implementation
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}


enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

struct PointMx<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> PointMx<X1, Y1> {
    fn mixup<X2, Y2>(self, other: PointMx<X2, Y2>) -> PointMx<X1, Y2> {
        PointMx {
            x: self.x,
            y: other.y,
        }
    }
}


pub fn traits() {

}

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait SummaryWDefault {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl SummaryWDefault for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// traits as params
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait bound, the non synactic sugar
pub fn notifyOriginal<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notifyWithTwoTraits(item: &(impl Summary + Display)) {
}

// the same code
// pub fn notify<T: Summary + Display>(item: &T) {


// hard to read
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {

// syntactic sugar for easier readability
// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {
// }


// Can only return one type
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}