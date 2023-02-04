use std::borrow::Cow;

fn abs_all<'a>(input: &'a mut Cow<'a, [i32]>) -> &'a Cow<'a, [i32]> {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[i] = -v;
        }
    }
    input
}

fn hello() {
    // No clone occurs because `input` is already owned.
    let slice = vec![-1, 0, 1];
    let mut input = Cow::from(slice);
    match abs_all(&mut input) {
        // TODO
        Cow::Owned(_) => println!("I own this slice!"),
        Cow::Borrowed(_) => println!("I own this slice!"),
        _ => panic!("expected borrowed value"),
    }
}

fn main() {
    hello()
}
