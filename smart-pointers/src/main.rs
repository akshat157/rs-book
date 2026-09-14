enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

use std::fmt::Display;

use crate::List::{Cons, Nil};

fn main() {
    let a = 5;
    let b = Box::new(5);
    println!("b = {b}");

    let list1 = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    // let list = Box::new(List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil)))));

    print_list(list1);

    let aptr = Box::new(a);

    assert_eq!(a, *aptr);
}

fn print_list<T: Display>(list: List<T>) {
    match list {
        Nil => (),
        Cons(x, listlist) => {
            println!("{x}");
            print_list(*listlist);
        }
    }
}
