pub fn slicetest(s: &String){
    let slice = &s[0..5];
    println!("slice is {}", slice);
    let a = "hello world!";
    let b = a; // this doesnt move ownership from a to b becoz a is a string literal
    // string literals are references by default... let a = "" is like let a: &str = ""
    println!("a is {}", a);
}

pub fn slice_string() {
    let normal_string = String::from("hello world");

    // by default, string literals are slices, that means the variables holding the slice is always
    // a reference variable
    let slice_string = "a slice string";
    // another way of having a slice
    let another_slice_string = &normal_string[0..5];

    // let's print the content that the slice is referring to and also print the address it holds
    println!("slice string is { } and address it holds is {:p}", slice_string, slice_string);
}

fn calculate_len(name: &String)-> usize{
    name.len()
}

pub fn string_manipulation() {
    // to change this string, we need to declare it mutable
    let mut s = String::from("hello world");
    let mut length = calculate_len(&s);
    println!("initial string: {}\n length: {}", s, length);

    s.push_str("... string updated");

    length = calculate_len(&s);
    println!("updated string: {}\n length: {}", s, length);
}
