/* fn foo<'a>(x: &'a u32, y: &'a u32) -> &'a u32 {
    &x
}

fn main() {
    let x = 10;
    let z = {
        let y = 15;
        foo(&x, &y)
    };

    println!("z is {}", z);
}
 */
fn main() {}
