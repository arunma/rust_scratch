#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}
fn main() {
    let excerpt = ImportantExcerpt { part: "hello" };
    println!("{:?}", excerpt);
}
