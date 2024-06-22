fn main() {
    println!("Hello, world!");

    another_function();
    another_function_with_parameter(5);
    print_labeled_measurement(5, 'h');

    // Statements are instructions that perform some action and do not return a value.
    // Expressions evaluate to a resultant value. Let’s look at some examples.

    // uncommend got error hence let is statement then let y = 6 not return value for x 
    // let x = (let y = 6);

    // A new scope block created with curly brackets is an expression
    let y = {
        let x = 3;
        x + 1 // Expressions do not include ending semicolons
    };

    println!("The value of y is: {y}");

    //function with return value
    let x = five();
    println!("The value of x is: {x}");
    let x = plus_one(5);
    println!("The value of x is: {x}");

}

// Rust code uses snake case as the conventional style for function and variable names, 
// in which all letters are lowercase and underscores separate words
fn another_function() {
    println!("Another function.");
}

// function with parameters
fn another_function_with_parameter(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}