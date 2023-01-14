use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
struct CustomErrorA{
}

#[derive(Debug)]
struct CustomErrorB{
}

impl Display for CustomErrorA {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for CustomErrorA{

}

impl Display for CustomErrorB {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for CustomErrorB{

}


fn error_function_a() -> Result<(), Box<dyn Error>> {
    Err(Box::new(CustomErrorA{}))
}

fn error_function_b() -> Result<(), Box<dyn Error>> {
    Err(Box::new(CustomErrorB{}))
}


fn main() {
    error_function_a().unwrap();
    //error_function_b().unwrap();

}