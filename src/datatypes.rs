pub mod arrays {
    pub fn print_months() {
        let months = ["jan", "feb", "march", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec"]; 
        for month in months.iter() {
            println!("{}", month);
        }
    }
    pub fn print_months_with_month_number() {
        let months = ["jan", "feb", "march", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec"]; 
        for (i, month) in months.iter().enumerate(){
            println!("number of {} is {}", month, i);
        }
    }
}

pub mod tuples {
    pub fn tupletest(){
        // it is not required that all elements of a tuple should be of the same type
        let tup = (45.7, "haris", 'c', 90);
        // we are destructuring a tuple here to capture the values in three separate variables
        let (x, y, z, o) = tup;
        println!("{}", x);
    }
}
