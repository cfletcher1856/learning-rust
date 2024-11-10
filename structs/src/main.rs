struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
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
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("cfletcher1856@gmail.com"),
        username: String::from("cfletcher1856"),
        active: true,
        sign_in_count: 1,
    };

    let _name: String = user1.username;
    user1.username = String::from("colinfletcher");

    let user2: User = build_user(String::from("colin@gmail.com"), String::from("colinf"));

    let _user3: User = User {
        email: String::from("katie@gmail.com"),
        username: String::from("katsnake"),
        ..user2
    };

    let rect: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    let rect1: Rectangle = Rectangle {
        width: 20,
        height: 40,
    };

    let rect2: Rectangle = Rectangle {
        width: 40,
        height: 50,
    };

    let rect3: Rectangle = Rectangle::square(30);

    println!("rect can hold rect1: {}", rect.can_hold(&rect1));
    println!("rect can hold rect2: {}", rect.can_hold(&rect2));

    println!("rect: {:#?}", rect);
    println!("rect3: {:#?}", rect3);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
