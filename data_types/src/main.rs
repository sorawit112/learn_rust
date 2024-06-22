// Rust is a statically typed language, 
// which means that it must know the types of all variables at compile time


fn main() {
    // trying to change string to u32 but we need to annotations type for complier
    
    // this line get error hence no anotate
    // let guess = "42".parse().expect("Not a number!");

    // also error hence invalid type annotate 
    // let guess: u32 = "42.1".parse().expect("Not a number!"); 

    let guess: u32 = "42".parse().expect("Not a number!");

    println!("guess: {guess}");

    // data types in rust have two subsets : Scalar and Compund

    // Scalar Types
    // Integer: singed and unsigned i/u[8,16,32,64,128,arch(system architecture)] length (default is i32)
    let _x = 57u8; // integer can use suffix to designate the type
    let _x = 1_000; // integer can use _ as visual separator make easier to read 1_000 = 1000
    // Interger Overflow when value greater than the define bitlength
    // method to handle overflow: wrapping_*, checked_*, overflowing_*, sturating_*

    // Floating-Point: f32, f64 (defualt is f32)
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

    // Numeric Operations
    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // Results in -1

    // remainder
    let _remainder = 43 % 5;

    // Boolean: true, false (1 byte size)
    let _t: bool = true;

    // Character: Unicode Scalar Values range from U+0000 to U+D7FF and U+E000 to U+10FFFF
    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
    let _heart_eyed_cat = '😻';

    // Compound Types
    // Tuple: fixed length: once declared, they cannot grow or shrink in size.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, _y, _z) = tup;

    println!("tup: {:?}",tup);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    // accessing tuple by '.' followed by index
    let _five_hundred = x.0;
    let _six_point_four = x.1;
    let _one = x.2;

    // Array: every element of an array must have the same type. and also fixed length
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    // initialize array with 5 elements of 3 value: [3, 3, 3, 3, 3]
    let a = [3; 5]; 

    // unlike tuple accessing array by indexing with square brackets 
    let _first = a[0];
    let _second = a[1];

}
