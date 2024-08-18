use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};
use crate::chapter15_smart_pointers::List::{Cons, Nil};

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }

// use crate::List::{Cons, Nil};

pub fn box_t_to_point_to_data_on_the_heap() {
    let b = Box::new(5);
    println!("b = {b}");
}

pub fn treating_smart_pointer_like_a_regular_references_with_deref_trait() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // This fails if we don't implement Deref trait
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

pub fn running_code_on_cleanup_with_drop_trait() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");


    // let c = CustomSmartPointer {
    //     data: String::from("some data"),
    // };
    // println!("CustomSmartPointer created.");
    // c.drop(); // nonono, we cannot call drop trait manually
    // println!("CustomSmartPointer dropped before the end of main.");

    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c); // std::mem::drop
    println!("CustomSmartPointer dropped before the end of main.");
}

// pub fn rct_reference_counted_smart_pointer() {
//     // This wont work, value moved twice
//     // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
//     // let b = Cons(3, Box::new(a));
//     // let c = Cons(4, Box::new(a));
//
//     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//     let b = Cons(3, Rc::clone(&a)); // RC clone doesn't make a deep clone, it only increases reference count
//     let c = Cons(4, Rc::clone(&a));
//
//     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//     println!("count after creating a = {}", Rc::strong_count(&a));
//     let b = Cons(3, Rc::clone(&a));
//     println!("count after creating b = {}", Rc::strong_count(&a));
//     {
//         let c = Cons(4, Rc::clone(&a));
//         println!("count after creating c = {}", Rc::strong_count(&a));
//     }
//     println!("count after c goes out of scope = {}", Rc::strong_count(&a));
//
// }

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where
        T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

pub fn refcell_and_the_interior_mutability_pattern() {

}


// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     struct MockMessenger {
//         sent_messages: Vec<String>,
//     }
//
//     impl MockMessenger {
//         fn new() -> MockMessenger {
//             MockMessenger {
//                 sent_messages: vec![],
//             }
//         }
//     }
//
//     impl Messenger for MockMessenger {
//         fn send(&self, message: &str) {
//             self.sent_messages.push(String::from(message));
//         }
//     }
//
//     #[test]
//     fn it_sends_an_over_75_percent_warning_message() {
//         let mock_messenger = MockMessenger::new();
//         let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
//
//         limit_tracker.set_value(80);
//
//         assert_eq!(mock_messenger.sent_messages.len(), 1);
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

pub fn refcell_list_example() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c =  Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}


#[derive(Debug)]
enum RefCycleList {
    Cons(i32, RefCell<Rc<RefCycleList>>),
    Nil,
}

impl RefCycleList {
    fn tail(&self) -> Option<&RefCell<Rc<RefCycleList>>> {
        match self {
            RefCycleList::Cons(_, item) => Some(item),
            RefCycleList::Nil => None,
        }
    }
}


#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}


pub fn reference_cycles_can_leak_memory() {
    // // let a = Rc::new(RefCycleList::Cons(5, RefCell::new(Rc::new(RefCycleList::Nil))));
    // //
    // // println!("a initial rc count = {}", Rc::strong_count(&a));
    // // println!("a next item = {:?}", a.tail());
    // //
    // // let b = Rc::new(RefCycleList::Cons(10, RefCell::new(Rc::clone(&a))));
    // //
    // // println!("a rc count after b creation = {}", Rc::strong_count(&a));
    // // println!("b initial rc count = {}", Rc::strong_count(&b));
    // // println!("b next item = {:?}", b.tail());
    // //
    // // if let Some(link) = a.tail() {
    // //     *link.borrow_mut() = Rc::clone(&b);
    // // }
    // //
    // // println!("b rc count after changing a = {}", Rc::strong_count(&b));
    // // println!("a rc count after changing a = {}", Rc::strong_count(&a));
    //
    // // Uncomment the next line to see that we have a cycle;
    // // it will overflow the stack
    // // println!("a next item = {:?}", a.tail());
    //

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());;


    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}