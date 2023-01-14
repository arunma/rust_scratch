fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn main() {
    println!("{}", longest("hello", "world123"));

    let result;
    let one = String::from("hello");
    {
        let two = String::from("world123");
        result = longest(one.as_str(), two.as_str());
        println!("{}", result);
    }
}
