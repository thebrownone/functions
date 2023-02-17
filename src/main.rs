fn main() {
    println!("Hello, world!");

    another_function (5);

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn another_function(x: i32) /*functions must always declare type of its parameters*/{
    println!("The value of x is: {x}"); // this is a statement because it ends with a ;
    let y =6; // also a statement, doesnt return a value. you cant assign it to something else
    // the "6" in the statement let y = 6; is an expression that evaluates to the value 6. 
    //Calling a function is an expression. 
    //Calling a macro is an expression. 
    //A new scope block created with curly brackets is an expression,
}