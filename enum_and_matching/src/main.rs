enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    {
        enum IpAddr {
            V4(String),
            V6(String),
        }
    
        let home = IpAddr::V4(String::from("127.0.0.1"));
    
        let loopback = IpAddr::V6(String::from("::1"));
    }

    {
        enum IpAddr {
            V4(u8, u8, u8, u8),
            V6(String),
        }
    
        let home = IpAddr::V4(127, 0, 0, 1);
    
        let loopback = IpAddr::V6(String::from("::1"));
    }

    {
        struct Ipv4Addr {
            // --snip--
        }
        
        struct Ipv6Addr {
            // --snip--
        }
        
        enum IpAddr {
            V4(Ipv4Addr),
            V6(Ipv6Addr),
        }
    }

    {   
        #[derive(Debug)]
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }
        
        // equiveriance with below struct but if we want to create function that take Message as input
        // below struct can't do that hence there are 4 data types instead 1 type with enum

        struct QuitMessage; // unit struct
        struct MoveMessage {
            x: i32,
            y: i32,
        }
        struct WriteMessage(String); // tuple struct
        struct ChangeColorMessage(i32, i32, i32); // tuple struct

        impl Message {
            fn call(&self) {
                // method body would be defined here
            }
        }
    
        let m = Message::Write(String::from("hello"));
        m.call();

        println!("{m:?}");
    }

    // std::option enum
    // enum Option<T> { <T> : hold piece of data in any type
    //     None,
    //     Some(T),
    // }
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None; //None have to annotate Option<i32> hence compliler need to know
    // let absent_number2 = None; //this one error need annotate something

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y; // error occur, hence some has no operator
    let sum = x + y.unwrap(); // unwrap method to get value inside

    // unwrap by match
    let unwrap_y: i8 = match y {
        None => 0,
        Some(i) => i,
    };

    println!("{y:?}, unwrap_y: {unwrap_y}");
    println!("{sum:?}");

    // using match with Option
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);


    // using match with int 
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),  //catch-all arm which bind variable = other
    };

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),  //catch-all arm which unbinding variable
    };

    let dice_roll = 9;
    let dice = match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), //catch-all arm with Tuple 'Unit' value
    };

    println!("{dice:?}");

    // using if let vs match
    let config_max: Option<i8> = Some(15);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alaska);
    match coin {
        Coin::Quarter(state) => println!("State quarter from {state:?}!"),
        _ => count += 1,
    }

    let coin = Coin::Penny;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }
    println!("{count}")

}

fn route(ip_kind: IpAddrKind) {}

// using match with Option
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}