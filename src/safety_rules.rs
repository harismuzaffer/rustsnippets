pub mod borrowing {
    fn borrowing(){
        let mut name = String::from("haris shah");
        let len = calculate_len(&mut name);
        println!("length of {} is {}", name, len);
    }
}

pub mod owning {
    fn ownit(s: String){
        println!("inside ownit");
    }
}
