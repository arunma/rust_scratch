use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let list = Cons(5,
                    Box::new(Cons(6,
                                  Box::new(Nil))));
    println!("{:?}", &list);

    match list {
        Cons(5, tail) => println!("{:?}", &tail),
        _ => println!("unmatched")
    }
}