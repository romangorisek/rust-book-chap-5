#[derive(Debug)]
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
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("roman.gorisek2@gmail.com"),
        username: String::from("RomanG"),
        sign_in_count: 0,
        active: false,
    };

    let name = user1.username;
    user1.username = String::from("romang");
    println!("{}", name);
    println!("{:#?}", user1);

    let user2 = build_user(String::from("test@test"), String::from("test_user"));
    println!("{:#?}", user2);

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("Area of the square is {}", area(&rect));
    println!("{:#?}", &rect);

    println!("Using impl, area of the square is {}", rect.area());

    let rect2 = Rectangle {
        width: 10,
        height: 15,
    };
    println!("Rect can hold rect2: {}", rect.can_hold(&rect2));

    let square = Rectangle::square(50);
    println!("Area of the square is {}", square.area());
}

fn area(dimensions: &Rectangle) -> u32 {
    dimensions.width * dimensions.height
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 1,
        active: true,
    }
}
