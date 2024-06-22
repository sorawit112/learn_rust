// Ownership: set of rules that RUST manages memory
// Stack and Heap is kind of memory storage of the system
// Data in stack: must have known and fixed size 
// Data in heap: allow an unknown size at compile or a size that might change

// pushing data: stack faster than heap ; no allocating step in stack
// accessing data: stack faster than heap ; heap need to follow pointer but stack not 

fn main() {
    // ownerships rules
    // - each value in Rust has an owner
    // - there can only be one owner at a time
    // - when the owner goes out of scope, the value will be dropped

    // variable scope
    {                            // s is not valid here, it’s not yet declared
        let _s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                            // this scope is now over, and s is no longer valid

    // The String Type: unknwon size -> store on heap
    // String is not string literal(str)
    let _s = String::from("hello");
    
    {
        let mut s = String::from("hello");

        s.push_str(", world!"); // push_str() appends a literal to a String

        println!("{s}"); // This will print `hello, world!`
    } // RUST called 'drop' automatically when closing curly bracket then s is no longer valid

    let _s = "hello"; // string literals immutability

    // Variables and Data Interacting with Move
    let x = 5; 
    let _y = x; // copy value of x then bind it to y

    
    // both x,y have value 5 and two 5 values pushed onto stack

    let s1 = String::from("hello");
    println!("{:?}", s1.as_ptr());
    let _s2 = s1; // not quit happend like known size data types,
                         // value from s1 not copied and not bind to s2
    // string attr on stack are ptr, len, capacity
    // which mean we just copy the ptr from s1 to s2 ptr

    // println!("{s1}, world!"); // this get error hence RUST considers s1 is no longer valid
                              // hence s1 and s2 ptr point to data which same memomry location
                              // if s1 still valid then it will cause double free memory
                              //  when s1, s2 go out of scope
    // so in Rust it called s1 was 'move' to s2 -> s1 moved then s1 no longer valid
    //  unlike other language may call 'shallow copy' wich copy pointer length and capacity

    // Variables and Data Interacting with Clone
    let s1 = String::from("hello");
    let s2 = s1.clone(); // used clone to copy data in heap then s1 still valid
    // data on heap was clone to both s1 and s2 then s1 wasn't moved to s2 but clone is may be expensive
    println!("s1 = {s1}, s2 = {s2}");

    // Stack-Only Data: Copy
    let x = 5;
    let y = x; // clone wouldn't do anything different

    println!("x = {x}, y = {y}"); // data in stack which knwon size, so copied is happend here

    // Ownership and Functions
    {
        let s = String::from("hello");  // s comes into scope

        takes_ownership(s); // s's value moves into the function...
                                        // ... and so is no longer valid here

        let x = 5;                 // x comes into scope

        makes_copy(x);    // x would move into the function,
                                        // but i32 is Copy, so it's okay to still
                                        // use x afterward

    } // Here, x goes out of scope, then s. But because s's value was moved, nothing
      // special happens.    


    {
        let _s1 = gives_ownership(); // gives_ownership moves its return
                                            // value into s1

        let s2 = String::from("hello");     // s2 comes into scope

        let _s3 = takes_and_gives_back(s2);  // s2 is moved into
                                            // takes_and_gives_back, which also
                                            // moves its return value into s3
        // println!("{s2}");  // s2 was moved then invalid

    } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
    // happens. s1 goes out of scope and is dropped.

    {   
        // what if we want value passed to function to do something back 
        // then we can return multiple by tuple which include input and output from function

        let s1 = String::from("hello");

        let (s2, len) = calculate_length(s1);

        println!("The length of '{s2}' is {len}.");
        // but this kind of function is too much ceremony and a lot of work that should be common 
        // Then let take a look at project References and Borrowing 
    }
  
} 

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.


fn gives_ownership() -> String {    // gives_ownership will move its
                                    // return value into the function
                                    // that calls it
    let some_string = String::from("yours"); // some_string comes into scope
    some_string                                      // some_string is returned and
                                                     // moves out to the calling
                                                     // function
}    

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}