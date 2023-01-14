use std::cell::RefCell;
use std::rc::Rc;
use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn main() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    *value.borrow_mut()+=10;

    let b = Cons(Rc::new(RefCell::new (4)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new (3)), Rc::clone (&a));

    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("c = {:?}", c);
}