use std::fmt::format;

fn main(){
    // create empty vector with i32 anotation hence rust need to know at complie time
    let v : Vec<i32> = Vec::new(); 

    let mut v = vec![1,2,3];

    println!("{:?}",v);

    let mut v2 = vec![4,5,6];

    // append another vec to its vec and consume all value in another vec
    v.append(&mut v2);

    println!("{:?}",v);
    println!("{:?}",v2);

    // also can push single var
    v2.push(4);
    v2.push(5);
    v2.push(6);

    println!("v: {:?}",v);
    println!("v2: {:?}",v2);

    // reading elements in vector 0-based index
    let third = &v2[2];
    println!("The third element v2 is {third}");

    // use get -> Option also 0-based
    let third = v2.get(2);
    match third {
        Some(val) => println!("The third element v2 is {val}"),
        None => println!("The third element v2 is None")
    }

    v2.pop();
    
    let third = v2.get(2);
    match third {
        Some(val) => println!("The third element v2 is {val}"),
        None => println!("The third element v2 is None")
    }

    // not error at complie time but will panicking at run time
    // read element by this method if want the program to panick
    // let value_not_exists = &v[100]; 

    // not panick but the program need to handle None value by match or if let
    let value_not_exists = v.get(100);

    if let Some(value) = value_not_exists {
        println!("value: {value}");
    } else {
        println!("value is None");
    }

    // this scope won't work because we cannot borrow mutable and immutable reference at the same scope
    {
        let mut v = vec![1,2,3];
        let first = &v[0]; // immutable reference
        v.push(4);  // borrow mutable reference -> push(&mut self)

        // following line will error
        // println!("first : {first}"); // immutable reference used here after mutable reference was borrowed before

        println!("v: {:?}", v); 
    }

    let v = vec![100, 32, 57];

    for i in v {
        println!("{i}");
    }
    // v moved before then cannot use v after interate
    // print!("{:?}",v);

    let mut v = vec![100, 32, 57];

    for i in &v {
        println!("{i}");
    }

    println!("{:?}",v);
    for i in &mut v {
        *i += 10;
    }

    println!("{:?}",v);
    

}