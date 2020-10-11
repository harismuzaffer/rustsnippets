pub fn struct_test() {
    struct Color(u32, u32, u32); // a tuple struct
    let white = Color(255, 255, 255);
    let Color(r, g, b) = white; // destructuring the struct
    println!("r g b values are {} {} {}", r, g, b);
    
    let user: user::User = user::User {
        name: "haris".to_string(),
        age: 26
    };
    println!("printing struct as it as {:?}", user);
}

pub mod user {
    #[derive(Debug)]
    pub struct User{
        pub name: String,
        pub age: u32
    }
}

pub mod rectangle {
    pub fn rectangle_struct() {
        let rectangle1 = Rectangle {
            length: 12,
            width: 10
        };

        println!("area of {:?} is {}", rectangle1, rectangle1.area());
        let rect2: Rectangle = Rectangle::new();
        println!("rect2 is {:?}", rect2);
    }

    #[derive(Debug)]
    struct Rectangle {
        length: u32,
        width: u32
    }

    impl Rectangle {
        fn area(&self)-> u32 {
            self.length * self.width
        }

        fn new()-> Rectangle{
            Rectangle{
                length: 0,
                width: 0
            }
        }
    }
}


