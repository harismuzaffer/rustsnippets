use rand::Rng;
use std::io;
use std::cmp::Ordering;

pub fn play() {
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
