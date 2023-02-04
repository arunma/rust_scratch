use std::{fs::OpenOptions, io::Write};

fn main() {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .write(true)
        .read(true)
        .open("sample.txt")
        .unwrap();
    let data = vec!["Hello\r\nWorld", "Foo\r\nBar", "Whatever man"];

    for &d in data.iter() {
        file.write_all(d.as_bytes()).unwrap();
    }
}
