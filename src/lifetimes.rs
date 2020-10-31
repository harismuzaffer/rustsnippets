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
    // by applying the first and second elision rule, there is no need of explicit litetimes here
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

