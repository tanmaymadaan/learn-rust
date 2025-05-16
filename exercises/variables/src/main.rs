fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    // Demonstrating shadowing
    let x = x * 2;  // Shadows the previous x with a new immutable variable
    println!("After shadowing, x is: {x}");

    // We can change the type when shadowing
    let x = "hello";  // Shadows x again with a different type
    println!("After shadowing with a string: {x}");

    // Shadowing in a new scope
    {
        let x = "inner scope";  // Shadows x only within this scope
        println!("Inside inner scope: {x}");
    }
    
    // Original shadowed x is still in scope
    println!("Outside inner scope: {x}");
}
