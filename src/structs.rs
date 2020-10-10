pub mod rectangle {

    #[derive(Debug)]
    pub struct Rectangle {
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

pub mod user {
    pub struct User{
        name: String,
        age: u32
    }
}

