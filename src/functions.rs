pub fn fibonacci(limit: u32){
    let mut current: u32 = 1;
    let mut pre: u32 = 0;
    let mut next: u32 = 0;

    while next <= limit - pre{
        next = current + pre;
        println!("{}", next);
        pre = current;
        current = next;
    }
}

pub mod functions_pass_and_return {
    pub fn test_pass_and_return() {
        let x = 78;
        let y = 97;
        let sum = sum(x, y);
        println!("sum of x and y: {}", sum);
    }
    
    fn sum(x: u32, y: u32)-> u32 {
        let s = x + y;
        s
    }
}
