pub fn string_manipulation() {
    let mut s = String::from("hello");
    let ss = String::from("world");
    s.push(' ');
    s.push_str(&ss);


    let s = String::from("Здравствуйте");
    println!("{}", &s[0..2]);
}

pub fn iterate_string() {
    let s = String::from("Здравствуйте");
    for b in s.bytes() {
        println!("{}", b);
    }

    for c in s.chars() {
        println!("{}", c);
    }
}

pub fn concatenate_strings() {
    let s1 = String::from("hello");
    let s2 = String::from("world");

    let s3 = String::from("rust");

    // let s4 = s1 + &s2 + &s3;
    let s4 = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s4);

    println!("slice is {} {:p}", &s4[1..5], &s4);
}
