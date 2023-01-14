use std::collections::HashMap;

fn main() {
    let mut vec = vec![
        1, 1, 2, 2, 3, 4, 5
    ];


    vec.sort_unstable();
    vec.reverse();

    println!("{:?}", vec);

    println!("{:?}", vec[4]);

}