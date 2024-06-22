fn main() {
    // if expresions
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Rust will not automatically try to convert non-Boolean types to a Boolean.
    // this get error, hence number is integer not bool
    // if number {
    //     println!("number was three");
    // }

    // Handling Multiple Conditions with else if 
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using if in a let Statement
    let condition = true;
    let number = if condition { 5 } else { 6 };

    // the values that have the potential to be results from each arm of the if must be the same type;
    // let number = if condition { 5 } else { "six" }; // error type not match
    println!("The value of number is: {number}");


    // Repetition with Loops

    // infinite loop
    // loop {
    //     println!("again!");
    // }

    // Returning Values from Loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // result = cointer * 2
        }
    };

    println!("The result is {result}");

    // Loop Labels to Disambiguate Between Multiple Loops
    // specify a loop label on a loop that you can then use with break or continue 
    // to specify that those keywords apply to the labeled loop instead
    // Loop labels must begin with a single quote
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // Conditional Loops with while
    // While a condition evaluates to true, the code runs; otherwise, it exits the loop.
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // Looping Through a Collection with for
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

}
