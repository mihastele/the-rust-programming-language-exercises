struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


// This would fail, it requires the use of lifetimes
// struct User2 {
//     active: bool,
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
// }



// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;

pub fn defining_and_instantiating_structs() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    // shorthand of the above to set the remainder of the fields from user1
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // You cannot instantiate Point in a type color despite having similar structure

    let subject = AlwaysEqual;

}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn build_user_with_shorthand(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}


// Derive debug outer attribute allows println! macro to use {:?} to print the contents of the struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


pub fn an_example_program_using_structs() {
    let width = 30;
    let height = 50;

    println!("The area of the rectangle is {} square pixels", area(width, height));

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_rect(&rect1)
    );

    println!("rect1 is {:?}", rect1);

    let scale = 2;
    let rect1 = Rectangle {
        // dbg! returns ownership of the expression's value
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(scale);
    // dbg! takes ownership of the value and prints it out, since we don't want to give ownership, we pass a reference
    dbg!(&rect1);
    dbg!(&rect1);
    // dbg!(rect1); // move rect
    // dbg!(&rect1);  This would fail since we already moved rect1
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_w_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_rect(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}