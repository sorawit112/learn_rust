// constant is defult imutable and always imutable
const THIS_IS_CONSTANT: u32 = 10*10*5; 
// The compiler is able to evaluate a limited set of operations at compile time, 
// which lets us choose to write out this value in a way that’s easier to understand and verify


fn main() {
    // Mutability
    // defualt is immutable (can't change variable value)
    // add mut when declare variable to make variable mutable
    
    let x = 5;
    println!("The value of x is: {x}");
    // x = 6; cannot assign twice to immutable variable
    
    let mut y: i32 = 5;
    println!("The value of y is: {y}");
    y = 6;
    println!("The value of y is: {y}");
    
    println!("constant: {THIS_IS_CONSTANT}");

    // Shadowing 

    let x = 5; // This x is shadowed by second x
    let x = x + 1; // This x take overshadow the first x
    // We can change value of x: By using 'let', we can perform a few transformations on a value 
    // but have the variable be immutable after those transformations have been completed.

    {
        // third x overshadow the second x only with in this scope
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // let vs mut
    // let can change variable type and variable value
    let spaces = "   ";
    let spaces = spaces.len();

    // uncomment to see error because mut able to change value but can't change type
    // let mut spaces = "   ";
    // spaces = spaces.len();

    println!("number of spaces: {spaces}");

}
