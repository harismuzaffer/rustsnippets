pub mod generics {
    pub mod generic_funcs {        
        // this function is generic but with trait bound of PartialOrd and Copy. That means only
        // those parameters are valid that implement both these traits
        fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
            let mut l = list[0];
            for &item in list {
                if item > l {
                    l = item;
                }
            }
            l
        }

        pub fn driver_code() {
            let integers = vec![5, 7, 12, 56];
            let chars = vec!['c', 'v', 'r', 'a', 'g', 't'];

            println!{"largest integer is {}", largest(&integers)};
            println!{"largest chat is {}", largest(&chars)};
        }
    }

    pub mod generics_with_types {
        pub struct Point<T, U> {
            x: T,
            y: U,
        }

        pub fn driver_code() {
            let both_ints = Point {x: 34, y: 89};
            let float_and_char = Point {x: 56.8, y: 'c'};
            println!("x of both ints: {:}", both_ints.x);
            println!("y of float and char: {:?}", float_and_char.y);
        }
    }
}

pub mod traits {
    pub trait Value {
        // Value is a trait. That means any type can implement the functions within this trait.
        // It also means that if multiple types implement this trait, those types are sharing some
        // code
        fn value(&self) -> String;
    }

    struct Point {
        x: u32,
        y: u32,
    }

    impl Value for Point {
        fn value(&self) -> String {
            format!("{}, {}", self.x, self.y)
        }
    }
    
    impl Value for f32 {
        fn value(&self) -> String {
            format!("{}", self.to_string())
        }
    }

    pub fn driver_code() {
        // Point and f32 both implement the trait Value, so we can call value func on them
        let p = Point {x: 45, y: 90};
        println!("{}", p.value());

        let f: f32 = 89.90;
        println!("{}", f.value());

        // we can use traits as parameters as below. We call a func that accepts a generic paramter
        // but with the constraint that the paramter should be implementing Value trait
        print_value(&p);
        print_value(&f);

        // this version uses trait bound syntax as is exactly same in functionality as the first
        // version.
        print_value_v2(&p);
        print_value_v2(&f);
    }

    fn print_value(item: &impl Value) {
        // any type that implements Value trait is a valid paramter
        println!("value is: {}", item.value());
    }

    fn print_value_v2<T: Value>(item: &T) {
        // generic paramter with Value bound - meaning only those types are valid that implement
        // Value
        println!("value is: {}", item.value());
    }
}
