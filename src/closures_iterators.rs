pub mod closures {
    pub fn simple_closure() {
        let mut x = String::from("hello");

        // this is a closure accepting NO parameter. It however captures the environemnt i.e. x
        // here and it doesn'e consume it, rather it just borrows it as mutable. Rust will
        // automatically use FnMut trait here
        let mut push_to_x = || {
            x.push_str("world");
        } ;

        // this println will is an error because x is alredy borrowed as mutable
        // println!("can't use x here: {:?}", x);

        let y = String::from("world");

        push_to_x();
        println!("x can still be used because it is not consumed by the closure: {:?}", x);
    }

    // a struct that has a closure as one of its fields
    pub struct Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        calculation: T,
        value: Option<u32>
    }

    impl<T> Cacher<T>
        where 
            T: Fn(u32) -> u32,
    {
        pub fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }

}


pub mod iterators {
    pub mod consuming_iterators {
        pub fn map_ref() {
            let v1: Vec<i32> = vec![1, 12, 7];
            let v2 = v1.iter();
            let v3: Vec<i32> = v2.map(|x| x + 1).collect();
            println!("Original vector, v1 is {:?}", v1); // the original vector is intact
            println!("New vector, v3 is {:?}", v3);
        }

        pub fn map_mut_ref() {
            let mut v1: Vec<String> = vec!["arif ".to_string(), "haris ".to_string()];
            let v2 = v1.iter_mut(); // this iter will get items from v1 as mutable references. 
            // That means it doesnt own the vector v1 needs to be declared as mut to achieve this
            let v3: Vec<&mut String> = v2.map(|x| {
                x.push_str("hello"); // the original vector items are being modified
                x
            }).collect();
            println!("Original vector, v1 is{:?}", v1);
        }

        pub fn map_into_iter() {
            let v1: Vec<String> = vec!["arif ".to_string(), "haris ".to_string()];
            let v2 = v1.into_iter(); // this iter will pull items as values(not as references). That means 
            // v1 is owned by the iter and after every next call item is getting removed from the vector
            let v3: Vec<String> = v2.map(|mut x| {
                x.push_str("hello");
                x
            }).collect();
            // next line is an error, because v1 is owned by the iter
            // println!("v1 is{:?}", v1);
            println!("New vector, v3 is{:?}", v3);
        }

        pub fn filter_with_map() {
            let v1: Vec<i32> = vec![109, 34, 78, 190, 199, 321, 48, 99];
            let v2 = v1.iter();
            let v3 = v2.filter(|&&x| x < 100); // iter() yields a reference to the itme of the vector 
            // and filter passes a reference to the closure... that means x here is a ref to ref. 
            // We need to dereference/ destructure it to compare it with an integer
            let v4: Vec<i32> = v3.map(|x| x + 900).collect();
            
            println!("Original vector is  {:?}", v1);
            println!("Filteted vector is  {:?}", v4);
        }
    }
}
