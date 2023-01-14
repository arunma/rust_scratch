#[derive(Debug)]
struct Basic<'a> {
    param0: &'a str,
}

impl<'a> Basic<'a> {
    fn print_hello(&self, param1: &'a str) -> &str {
        println!("{}: {}", &self.param0, param1);
        param1
    }

    fn dummy(&self, param1: &'a str) -> &str {
        param1
    }
}

fn first_word(param: &str) -> &str {
    let bytes = param.as_bytes();
    for (i, &bt) in bytes.iter().enumerate() {
        if bt == b' ' {
            return &param[..i];
        }
    }
    param
}

fn main() {
    let basic = Basic { param0: "basic1" };
    let result = basic.print_hello("param1");
    println!("{}", result);

    println!("{}", first_word("hello world whatever"));
    println!("{}", basic.dummy("hello world whatever"));
}
