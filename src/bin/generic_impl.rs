/* struct Point<T> {
    x: T,
    y: T,
}

impl<T: Copy> Point<T> {
    fn hello(&self) -> T {
        self.x
    }
}

impl Point<u32> {
    fn bye(&self) -> u32 {
        1000
    }
}

fn main() {
    let point1 = Point { x: 10, y: 11 };
    println!("{}", point1.hello());
    println!("{}", point1.bye());

    let point2 = Point { x: 10.0, y: 11.0 };
    println!("{}", point2.hello());
    println!("{}", point2.bye());

    let point3 = Point {
        x: "hello".to_string(),
        y: "bye".to_string(),
    };
    println!("{}", point3.hello());
    println!("{}", point3.bye());
}
 */

fn main() {}
