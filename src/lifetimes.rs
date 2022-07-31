pub fn dangling_ref() {
    let r;

    {
        let s = String::from("hello world");
        r = &s;
    }

    // it is a compile error to print r because r refers to a string slice that is already dropped
    // println!("{}", r);
}


pub mod life_times_and_elision_rules {
     // A very useful blog to understand lifetimes: https://richardanaya.medium.com/a-journey-through-rust-lifetimes-5a08782c7091

    pub mod understanding_lifetimes {
        // lifetime is all about references. If there is no reference involved, no need of
        // lifetimes
        
        // When there are references involved, borrow checker sometimes is not able to infer
        // lifetime of certain references
        
        // example when borrow checker is able to infer lifetimes ...
        // let x;
        // {
        //      let a = 10;
        //      z = &a;
        //      println!("perfectly fine to print z as it refers to `a` which is still alive i.e. still lives {}", z);
        //      `a` is dropped afret this block ends, any reference to `a` is invalid
        // }
        // println!("invalid to print z as it refers to `a` which doesn't live enough so that `z` 
        //          can not be further used {}", z);
        
        // example when borrow checker is not able to infer lifetimes, thus needs programmers help
        // to validate lifetimes of some references...
        // let x = 10;
        // let result;
        // {
        //      let y = 20;
        //      r = max(&x, &y); max returns the reference of the maximum of the two values passed
        //      // compiler doesn't know whether reference of x or y would be returned, so it is
        //      not gauranteed that the next print statement is printing a valid reference or not
        //      because if reference of x is returned by max, then print statement is valid but in
        //      case y is returned then it is not valid which is why there is no gaurantee
        // }
        // println!("invalid to print result as it refers to either `x` or `y`  which in case of y doesn't live enough so that `result` 
        //          can be further used {}", z);
        // To fix the above, we need help compiler to figure out the correct lifetime by doing the
        // following:
        // - set lifetime of the returned value as the lifetime of minimum of lifetimes of x and y.
        // By doing so, we are safe no matter which one(x or y) is returned by max. We were already
        // safe in case reference of x is returned by max because x live enough for println to
        // print it. Now with lifetimes, we are safe even if y is returned because we have told
        // compiler to set lifetime of the returned value as the minimum of x and y  i.e. returned
        // reference should live atleast as y lives. 

        fn applying_lifetimes () {
            let string1 = String::from("long string is long");
            let result;
            {
                let string2 = String::from("xyz");
                // the fn longest is a compilation error becuase rust compiler doenst know which 
                // string will be returned from it. If string string2 ref is returned then the
                // print statement after this block cant print result because result refers to
                // strins2 which dies at the end of this block.
                
                // We need to hit the compiler and tell it what is the lifetime relationship here
               
                // in other words, if we dont tell compiler that lifetime relationships, it wont
                // allow us to proceed without that and if we add lifetime relationships, the 
                // print after this block is still an error but now it is because result refers
                // to string2 which doesn live enough 
                
                result = longest(string1.as_str(), string2.as_str());
            }
            // println!("The longest string is {}", result);
        } 

        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            // The two references passed to compare have different lifetimes (or scopes).
            // Since y has a smaller lifetime(y referes to string2 and lives only during the block
            // above), it becomes the concrete lifetime. This means that all the parameters that 
            // have 'a written next to them should have a lifetime at least as long as y. This also
            // means that answer, which stores the returned reference, should be valid as long as y
            // is valid. Therefore, if we use result out of the scope of y, we get a compilation error:
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }

        pub mod more_lifetimes {
            // This is about how static data can affect your lifetimes
            // We will create two variants of Foo. Both Foos hold a single integer. One of the Foos
            // hold an owned integer and another one holds a reference to an integer.
            struct FooOwned {
                bar: i32
            }

            // In case of Foo holding a reference, we need to apply lifetimes to tell compiler the
            // relationship between the Foo instance and the data reference that it holds
            struct FooReferred<'a> {
                bar: &'a i32
            }

            // We have get_bar_owned and get_bar_referred for FooOwned and FooReferred respectively. Both return a integer reference
            
            // get_bar_owned takes a FooOwned reference and returns an i32 reference. Rust is able
            // to infer that the only lifetime it’s output could be based upon is it’s input. So it
            // auto sets lifetime of the input and the output which basically would say: output will
            // last at least as long as it’s sole input
            fn get_bar_owned(f: &FooOwned) -> &i32 {
                &f.bar 
            }

            // get_bar_referred takes a FooReferred reference and returns an i32 reference. Rust is
            // not able to infer that output lifetime because here the reference of FooReferred
            // Becasuse the parameter passed could be a static instance. 
            // We need to tell Rust whether the output lifetime is based on the lifetime of
            // the member of the struct instance or the parameter passed. Why? because FooReferred
            // holds a reference which means that if FooReferred instance is static, the reference
            // it holds might not be static which in turn means reference can live shorter than the
            // instance itself. In case of FooOwned, its static instance would mean that the data
            // it holds would always live as long as the instance itself because it is owned by the instance
            //
            // One way to fix it to tell compiler that output lasts as long as the parameter
            fn get_bar_referred<'a>(f: &'a FooReferred) -> &'a i32 {
                &f.bar 
            }

            // Another way is to tell compiler that output lasts as long as member of FooReferred
            // instance
            fn get_bar_referred_v2<'a>(f: &FooReferred<'a>) -> &'a i32 {
                &f.bar 
            }

            pub fn apply_more_lifetime() {
                // let n = 100;
                // let ref_n;
                // {
                //     let f: FooReferred = FooReferred{bar: &n};
                //     ref_n = get_bar_referred(&f);
                // }
                // // following print is an error because we have stated that output of get_bar_referred
                // // lives as long as the instance of FooReferred passed but here f(i.e. FooReferred
                // // instance doesn live enough so that we can print ref_n)
                // // println!("{}", ref_n);
            }

            pub fn apply_more_lifetimes_v2() {
                let n = 200;
                let ref_n;
                {
                    let f: FooReferred = FooReferred{bar: &n};
                    ref_n = get_bar_referred_v2(&f);
                }
                // following print works because output of get_bar_referred_v2 lives as long as the
                // member of FooReferred instance i.e. bar
                println!("{}", ref_n);
            }
        }

    }

    // by applying the first and second elision rule, there is no need of explicit litetimes here.
    // the output reference is given the same lifetime as that of the input reference.
    pub fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    // by applying all the three rules of elision, rust is not sure about the lifetime of the
    // params and the return type. So we need explicit lifetimes here. We say that the x and y both
    // should stay as long as the return value.
    pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } 
        else{
            y
        }
    }

   // lifetime here means an instance of ImportantExcerpt can’t outlive the reference it holds in its part field 
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    
    pub fn struct_lifetime() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ImportantExcerpt {
            part: first_sentence,
        };
    }
}

