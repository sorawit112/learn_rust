// The Rules of References
// - At any given time, you can have either one mutable reference or any number of immutable references.
// - References must always be valid. (no dangle pointer)

fn main() {
    let s1 = String::from("hello");

    let len: usize = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    let mut s = String::from("hello");
    change(&s);
    mute_change(&mut s);

    let mut s = String::from("hello");

    let r1 = &mut s;

    // mutable borrow can used once at a time
    // uncomment after this to see error 
    let r2 = &mut s;  
    println!("{}, {}", r1, r2);

    // this prevent data race 
    // - Two or more pointers access the same data at the same time.
    // - At least one of the pointers is being used to write to the data.
    // - There’s no mechanism being used to synchronize access to the data.

    // use curly brackets to create a new scope, allowing for multiple mutable reference
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

    // cannot have a mutable reference while we have an immutable one to the same value.
    // multiple immutable references are allowed because no one who is just reading the data
    // has the ability to affect anyone else’s reading of the data.
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM
    println!("{}, {}, and {}", r1, r2, r3);

    // Dangling References
    // dangling pointer—a pointer that references a location in memory 
    // that may have been given to someone else—by freeing some memory 
    // while preserving a pointer to that memory
    let reference_to_nothing = dangle();



}


fn calculate_length(s: &str) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

fn change(some_string: &String) {
    // some_string.push_str(", world"); // this caused error becaused we borrow some_string 
                                        // then we cannot change value of some_string
}

fn mute_change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger! -> this reference would be pointing to an invalid String use no_dangle instead

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}