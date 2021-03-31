pub mod borrowing {
    pub fn borrowing(){
        let name = String::from("haris shah");
        // calculate_len is borrowing the string name(notice ampersand), that is why in the next 
        // line, we are able to reuse name. Without ampersand, it would have owned `name` and we
        // wont be able to use `name` afterwards
        let len = calculate_len(&name);
        println!("length of {} is {}", name, len);
    }

    pub fn calculate_len(name: &String)-> usize{
        name.len()
    }
}

pub mod owning {
    pub fn owning() {
        let color = String::from("magenta");
        ownit(color);
        // the next line is an error because the string `color` ownsership has changed now
        // println!("my colors is {}", color);
    }
    fn ownit(s: String){
        println!("im owning the string, the calling function cant access it now");
    }
}

pub mod references {
    pub fn mutable_and_immutable_references() {
        // we can have any number of immutable references to a data, but there can be only one 
        // mutable references at at time. Also mutable and immutable references cant coexist
        let mut s = String::from("hello world");
        let ss = &s; // ok ... non-lexical lifetime of ss starts here
        let sss = &s; // ok
        println!("{}", ss); // non-lexical lifetime of ss ends here

        let mut k = String::from("hello workd");
        let kk = &k; // ok ... non-lexical lifetime of kk starts here
        let kkk = &mut k; // not ok if we uncomment the next print because it would mean mut and 
        // immutable references coexist throughout the non-lexical lifetime of kk
        // println!("{}", kk); // non-lexical lifetime of kk ends here

        let mut l = String::from("hello world");
        let ll = &mut l; // ok ... non-lexical lifetime of ll starts here
        let lll = &mut l; // not ok if we uncomment the next print as that would mean two immutable
        // references throughout the non-lexical lifetime of ll
        // println!("{}", ll);


        let mut m = String::from("hello world");
        let mm = &mut m; // ok ... non-lexical lifetime of mm starts here
        println!("mm is a ref holding an address: {:p}", mm); // non-lexical lifetime of mm ends here
        let mmm = &mut m; // ok because the non-lexical lifetime of mm is over 
        // so we can have new set of references to m
    }
}
