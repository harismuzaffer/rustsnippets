fn slicetest(s: &String){
    let slice = &s[0..5];
    println!("slice is {}", slice);
}

fn calculate_len(name: &mut String)-> usize{
    name.push_str(" hello!");
    name.len()
}
