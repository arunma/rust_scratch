use std::rc::Rc;
use crate::List::{Cons, Nil};
//
// #[derive(Debug)]
// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }
//
// fn main() {
//     let a = Cons(3, Box::new(Cons(4, Box::new(Nil))));
//
//     let b = Cons(2, Box::new(a));
//     let c = Cons(1, Box::new(a));
//
//     println!("a = {:?}", &a);
//     println!("b = {:?}", &b);
//     println!("c = {:?}", &c);
// }


#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let a = Rc::new(Cons(3, Rc::new(Cons(4, Rc::new(Nil)))));

    let b = Cons(2, Rc::clone(&a));
    let c = Cons(1, Rc::clone(&a));

    println!("a = {:?}", &a);
    println!("b = {:?}", &b);
    println!("c = {:?}", &c);

    println!("Strong count of a {}", Rc::strong_count(&a));
}