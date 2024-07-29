use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");

    //get -> Option<&T> -> copied() -> Option<T>
    let score = scores.get(&team_name).copied().unwrap_or(0);

    print!("score: {score}");
    {
        let mut x: Option<&i32> = Some(&1);
        let copied_x: Option<i32> = x.copied();

        x = Some(&2);
        println!("x:{}, copied_x:{}",x.unwrap(), copied_x.unwrap());
    }

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // String dont have copy trait then field_value cannot be used after inserted to map hence insert take ownership
    // print!("{field_value}"); //this will error


    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    // let try inserted by refernece it can use filed_value after inserted but u have to concern that the lifetime of field_value must be valid
    // for at least as long as lifetime of map
    let mut map = HashMap::new();
    map.insert(field_name, &field_value);

    print!("{field_value}"); 


    // Adding a Key and Value Only If a Key Isn’t Present
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");

    //or_insert return mutable reference to the value in the entry
    let yellow_value = scores.entry(String::from("Yellow")).or_insert(50);
    println!("{yellow_value}");
    println!("{scores:?}"); //scores don't change hence Yellow already in the map
    let yellow_value = scores.entry(String::from("Yellow")).or_insert(50);
    *yellow_value += 10; //adding 10 to the yellow 
    println!("{scores:?}");

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");

}
