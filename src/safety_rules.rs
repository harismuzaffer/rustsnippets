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
        // we can have any number of immutable references to a data, but there can be only one mutable
        // references at at time. Also mutable and immutable references cant coexist
        let mut s = String::from("hello world");
        let ss = &s; // ok
        let sss = &s; // ok
        println!("{}, {}", ss, sss);

        let mut k = String::from("hello workd");
        let kk = &k; // ok
        let kk = &mut k; // not ok because mutable and immutable coexisting, the next line is an error
        // println!("{}, {}", kk, kkk);

        let mut l = String::from("hello world");
        let ll = &mut l; // ok
        let lll = &mut l; // not ok because two immutable references, next line is an error
        // println!("{}, {}", ll, lll);


        let mut m = String::from("hello world");
        let mm = &mut s; // ok
        println!("mm is a reference holding an address: {:p}", mm);
        let mmm = &mut s; // ok because the non-lexical lifetime of mm is over so we can have new set of references to m
        println!("{}", mmm);
    }
}
