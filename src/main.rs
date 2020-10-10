pub mod strings;
pub mod datatypes;
pub mod structs;
pub mod enums;
pub mod control_flow;
pub mod vectors;
pub mod guessing_game;



fn main() {
    guessing_game::play();
    // tupletest();
    // control_flow();
    // loops();
    // fibonacci(100);
    // borrowing();
    let mut s = String::from("haris shah");
    // ownit(s);
    // let arr = [3, 7, 8, 23, 4, 8];
    // loops2(&arr);
    slicetest(&s);
    s.clear(); // we can not have more than one mutable references at a time
    // or one mutable and one immutable. But here clear although does take
    // mutable reference but that goes out of scope here, so we cant have a mutable
    // reference in the next line
    // let ss = &mut s;
    // println!("s is {}", ss);
    // let user: User = User {
    //     name: "haris".to_string(),
    //     age: 26
    // };

    // let user2 = User {
    //     name: "hammad".to_string(),
    //     ..user
    // };

    struct Color(u32, u32, u32);
    let white = Color(255, 255, 255);
    let Color(r, g, b) = white;
    println!("white is {} {} {}", r, g, b);
    // let j = "hello";
    // let k = j;
    // println!("j is {}", j);
    // println!("k is {}", k);
    // println!("j is {}", j);
    let a = "hello world!";
    let b = a; // this doesnt move ownership from a to b becoz a is a string literal
    // string literals are references by default... let a = "" is like let a: &str = ""
    println!("a is {}", a);

    let rectangle1 = Rectangle {
        length: 12,
        width: 10
    };

    println!("area of {:?} is {}", rectangle1, rectangle1.area());
    let rect2: Rectangle = Rectangle::new();
    println!("rect2 is {:?}", rect2);

    // let ip_v4 = IpAddrKind::V4;
    // let ip_v6 = IpAddrKind::V6;


    // let mut v: Vec<u32> = Vec::new();
    let mut vv = vec![String::from("hello"), String::from("world")];

    let second = &vv[0];
    vv[1] = String::from("hello again");
    // println!("second item is {:?} {}", vv, second);
    println!("updated vector is {}", second);

    let second2 = vv.get(100);

    match second2 {
        Some(item) => {
            println!("got second element {}", item);
        },
        None => {
            println!("got a None");
        }
    }
    iterateVector(&vv);
}

