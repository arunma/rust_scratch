fn hello(x: &'static u32, y: &'static u32) -> u32 {
    return *x + *y;
}

// fn rule2(x: &u32) -> &u32 {
//     x
// }

fn main() {
    println!("hello");

    println!("Hello {}", hello(&5, &2));
}
