use std::fmt::Display;

trait Super {
    /* fn hello() {
        println!("Hello") // Will not work due to lack of object safety
    } */

    fn hello(&self) {
        println!("Hello") // Will work
    }
    // fn hello1() -> Self {
    //     println!("Hello2"); // Will NOT work. Object-safe methods may not return Self
    //     Bar {}
    // }

    fn hello1() -> Self
    where
        Self: Sized;

    // fn typed<T: Super>(t: T) { //Will NOT work
    //     println!("hello")
    // }

    // fn typed1<T: Display>(&self, t: T) {
    //     //Will NOT work
    //     println!("hello")
    // }

    fn hello2() -> Self
    where
        Self: Sized;
}

#[derive(Debug)]
struct Foo {}
struct Bar {}

impl Super for Foo {
    fn hello1() -> Self
    where
        Self: Sized,
    {
        Foo {}
    }

    fn hello2() -> Self
    where
        Self: Sized,
    {
        todo!()
    }
}
impl Super for Bar {
    fn hello1() -> Self
    where
        Self: Sized,
    {
        Bar {}
    }

    fn hello2() -> Self
    where
        Self: Sized,
    {
        todo!()
    }
}

fn dispatch(flag: bool) -> Box<dyn Super> {
    if flag {
        Box::new(Foo {})
    } else {
        Box::new(Bar {})
    }
}

fn main() {
    let to = dispatch(true);
    to.hello();

    let foo = Foo::hello1();
    dbg!(foo);
}
