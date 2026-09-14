use crate::List::{Cons, Nil};
use std::{fmt::Display, ops::Deref};

enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let a = 5;
    let b = Box::new(5);
    println!("b = {b}");

    let list1 = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    // let list = Box::new(List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil)))));

    print_list(list1);

    let aptr = Box::new(a);

    assert_eq!(a, *aptr);

    let aa = 5;
    let bb = MyBox::new(aa);

    assert_eq!(5, aa);
    assert_eq!(5, *bb);

    // let name = "Rust";
    let m = MyBox::new(String::from("RustString"));
    hello(&m);
    hello(&(*m)[..]);
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

fn hello(name: &str) {
    println!("Hello, {name}!");
}
