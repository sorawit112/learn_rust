use std::rc::Rc;
use std::cell::RefCell;

// Define a recursive cons list using an enum
#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>), // Head (i32) and Tail (Rc<List>)
    Nil,                 // Empty list
}

impl List {
    // Function to create a new empty list
    fn new() -> Rc<List> {
        Rc::new(List::Nil)
    }

    // Function to prepend an element to the list (like cons)
    fn prepend(list: Rc<List>, value: i32) -> Rc<List> {
        Rc::new(List::Cons(value, list))
    }

    // Function to print the elements of the list
    fn print_list(list: &Rc<List>) {
        match **list {
            List::Cons(value, ref tail) => {
                print!("{} -> ", value);
                List::print_list(tail);
            }
            List::Nil => {
                println!("Nil");
            }
        }
    }
}

fn main() {
    // Create an empty list
    let empty_list = List::new();

    // Prepend elements to the list
    let list = List::prepend(empty_list, 1);
    let list = List::prepend(list, 2);
    let list = List::prepend(list, 3);

    // Print the list
    println!("List contents:");
    List::print_list(&list);


    let a = Rc::new(RefCell::new(5));
    *a.borrow_mut() = 5;
}
