fn main() {

    // immutable --> compilation leads to error!
    // let x = 5;
    // println!("The value x is: {}", x);
    // x = 6;
    // println!("The value x is: {}", x);

    // mutable --> compiles without a problem
    let mut x = 5;
    println!("The value x is: {}", x);
    x = 6;
    println!("The value x is: {}", x);


    // shadowing
    let x = 5;
    println!("The value of x: {}", x);
    
    let x = x + 1;
    println!("The value of x: {}", x);

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}
