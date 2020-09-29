use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main(){
    // guessing_game();
    // tupletest();
    // control_flow();
    // loops();
    // fibonacci(100);
    // borrowing();
    let mut s = String::from("haris shah");
    // ownit(s);
    // let arr = [3, 7, 8, 23, 4, 8];
    // loops2(&arr);
    // slicetest(&s);
    // s.clear(); we can not have more than one mutable references at a time
    // or one mutable and one immutable. But here clear although does take
    // mutable reference but that goes out of scope here, so we cant have a mutable
    // reference in the next line
    // let ss = &mut s;
    // println!("s is {}", ss);
    let user: User = User {
        name: "haris".to_string(),
        age: 26
    };

    let user2 = User {
        name: "hammad".to_string(),
        ..user
    };

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

    let ip_v4 = IpAddrKind::V4;
    let ip_v6 = IpAddrKind::V6;
}

enum IpAddrKind {
    V4,
    V6
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
}

struct User{
    name: String,
    age: u32
}


fn slicetest(s: &String){
   let slice = &s[0..5];
   println!("slice is {}", slice);
}

fn loops2(arrr: &[u32]){
    for (i, item) in arrr.iter().enumerate(){
        println!("index {} and value {}", i, item);
    }
}

fn ownit(s: String){
    println!("inside ownit");
}

fn borrowing(){
    let mut name = String::from("haris shah");
    let len = calculate_len(&mut name);
    println!("length of {} is {}", name, len);
}

fn calculate_len(name: &mut String)-> usize{
    name.push_str(" hello!");
    name.len()
}

fn fibonacci(limit: u32){
    let mut current: u32 = 1;
    let mut pre: u32 = 0;
    let mut next: u32 = 0;

    while next <= limit - pre{
        next = current + pre;
        println!("{}", next);
        pre = current;
        current = next;
    }
}

fn loops(){
    let arr = ["jan", "feb", "mar"];
    for month in arr.iter(){
        println!("month is {}", month);
    }
}

fn control_flow(){
    let c: u32 = 10;
    if c > 100{
        println!("value is greater than 100");
    }
    else{
        println!("value is less than 100");
    }
}

fn tupletest(){
    let tup = (45.7, "haris", 'c', 90);
    let (x, y, z, o) = tup;
    println!("{}", x);
}

fn guessing_game() {
    println!("Guessing game\n");

    print!("please enter a number\n");

    loop{
        let secret_number = rand::thread_rng().gen_range(1, 101);
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).
            expect("something went wrong");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) =>{
                println!("please try again with a valid number");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too less"),
            Ordering::Greater => println!("too large"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }
}
