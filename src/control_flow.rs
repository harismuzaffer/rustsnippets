pub fn test_for_equality(x: u32, y: u32) {
    if x == y {
        println!("equal numbers");
    }
    else {
        println!("different number");
    }
}

pub fn loop_n_times(n: u32) {
    let mut visit: u32 = 0;

    loop {
        if visit == n {
            break;
        }
        println!("visit no. {}", visit);
        visit += 1;
    }
}
