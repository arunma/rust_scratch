use std::fmt::Display;

fn largest<T>(items: &[T]) -> T
where
    T: PartialOrd + Display + Copy,
{
    let mut result = items[0];
    for &item in items.iter() {
        if result < item {
            result = item;
        }
    }
    result
}

fn main() {
    println!("Largest is : {}", largest(&[1, 2, 3, 4, 6, 5, 34, 2]));
    println!("Largest is : {}", largest(&['a', 'u', '7', 'h']));
}
