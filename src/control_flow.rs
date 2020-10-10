fn control_flow(){
    let c: u32 = 10;
    if c > 100{
        println!("value is greater than 100");
    }
    else{
        println!("value is less than 100");
    }
}

fn test_for_equality(x: u32, y: u32) {
    if x == y {
        println!("equal numbers");
    }
    else {
        println!("different number");
    }
}
