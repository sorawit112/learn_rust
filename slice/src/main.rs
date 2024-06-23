// slice: reference a contiguous sequence of elements in a collection 
// A slice is a kind of reference, so it does not have ownership.

fn main() {
    let s = String::from("hello world");

    let _hello = &s[0..5]; 
    let _world = &s[6..11]; 

    let _slice = &s[0..2];
    let _slice = &s[..2];

    let len = s.len();

    let _slice = &s[3..len];
    let _slice = &s[3..];

    let _slice = &s[0..len];
    let _slice = &s[..];

    let mut s = String::from("hello world");

    let word = first_word(&s);

    // s.clear(); // error!

    println!("the first word is: {word}");

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);


    // slice with array
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
