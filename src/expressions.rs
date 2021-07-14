pub mod expressions {
    pub fn simple_expressions() -> i32{
        // expressions evaluate to value and they return that value. In the below example
        // `78 - 10` is an expression
        let a = (78 - 10); 
        // the above line is a statemnt which assigns `78 - 10` to variable `a`. `78 - 10` here is an expresion
        // because it evaluates to a value i.e. `68`.
        
        // statements end with a semicolon
        let a = (56 * 2); // a statemnt
        // let a = (56 *2)  Error. We cant turn a statement into an expresion by removing the
        // semicolon because statements dont evaluate to a value
        
        // Expressions dont end with a semicolon
        56 * 2; // Adding a semicolon turns an expresion into a statement
        56 * 2 // an expresion
    }

    pub fn if_expression() {
        // All if statements are actually expressions that evaluate to a value
        let a = if true {
            5
        } else { 
            7 
        };
        println!("{:?}", a); // 5
        // In the above example, we are assigning `a` a value. Which value? The value that `if`
        // expression evaluates to. Here `if` expression evaluates to 5

        // Why is `if` an expression? Consider the following fn
        fn some_fn() -> i32 {
            return 10;
        }
        // This fn return 10 i.e. it evaluates to 10. That means we can assign its result to a
        // variable like so:
        let x = some_fn(); // x holds 10

        // Consider this `if`
        if true  {
            return ();
        }
        // This `if` always evaluates to a value(() in this case) without calling it like a fn. Functions evaluate
        // to a value when we call the fn, but `if` doesn't require calling. That is the reason 
        // `if` is always an expression
    }

    pub fn if_expression_example() {
        let condition = true;

        let x = if condition { "hello"  } else { "world" };
        println!("{}", x);
    }
}
