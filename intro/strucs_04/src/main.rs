#[derive(Debug)]

// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {  // associated function
        Rectangle { width: size, height: size }
    }
}

fn main() {
    // let mut user1 = User{
    //     email: String::from("example@email.com"),
    //     username: String::from("example"),
    //     active: true,
    //     sign_in_count: 1,
    // };

    // let name = user1.username;
    // user1.username = String::from("new_name");

    // let _user2 = build_user(String::from("new_user1"), String::from("example1@email.com"));
    
    // create instance from another instance
    // let user3 = User {
    //     email: String::from("example2@email.com"),
    //     username: String::from("new_user2"),
    //     ..user2
    // };

    // tuple struct
    // struct Color(i32, i32, i32);
    // struct Point(i32, i32, i32);

    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect is {:#?}", rect);
    println!("The area of the rectangle is {} square pixels.", area(&rect));
    println!("The area of the rectangle is {} square pixels.", rect.area());

    let rect1 = Rectangle {
        width: 20,
        height: 40,
    };
    let rect2 = Rectangle {
        width: 40,
        height: 50,
    };

    println!("Can rect hold rect1? {}", rect.can_hold(&rect1));
    println!("Can rect hold rect2? {}", rect.can_hold(&rect2));

    let rect3 = Rectangle::square(30);
    println!("rect3 is {:#?}", rect3);


}


// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,

//         sign_in_count: 1,
//     }
// }

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}