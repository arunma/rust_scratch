use std::fmt::{Debug, Display};

trait Bar: Debug {
    fn bar(&self) -> u32 {
        10
    }
}

#[derive(Debug)]
struct Foo {}
#[derive(Debug)]
struct Baz {}

impl Display for Foo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Foo")
    }
}

impl Display for Baz {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Baz")
    }
}

impl Bar for Foo {}
impl Bar for Baz {}

/* fn f(a: bool) -> impl Bar {
    if a {
        Foo {}
    } else {
        Baz {}
    }
} */

fn f1(a: bool) -> Box<dyn Bar> {
    if a {
        Box::new(Foo {})
    } else {
        Box::new(Baz {})
    }
}

fn main() {
    let h = f1(false);
    // println!("{:?}", h);
    dbg!(&h);
}
