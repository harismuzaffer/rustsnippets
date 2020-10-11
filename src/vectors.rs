fn iterate_vector(v: &Vec<String>) {
    println!("printing vector ...");
    for item in v{
        println!("item is {}", item);
    }
}

pub fn vectors_test() {
    let mut v: Vec<u32> = Vec::new(); // an empty vector
    let mut vv = vec![String::from("hello"), String::from("world")]; // a vector using vec macro
    iterate_vector(&vv);
    vv.push(String::from("another item"));
    iterate_vector(&vv);
}

pub fn get_second_itme_from_vector(){
    let v = vec![45, 67, 12, 890];
    let second_item = v[1]; // one way
    let second_item_prime = v.get(1); // another way. This will return an Option enum
    println!("second element using index is {}", second_item);

    match second_item_prime {
        Some(item) => {
            println!("second element using get method is {}", item);
        },
        None => {
            println!("Got a None");
        }
    }
}
