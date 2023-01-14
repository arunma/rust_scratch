use std::fmt::{Display, Formatter};
use anyhow::Result;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("CustomErrorA")]
struct CustomErrorA{
    #[from]
    source: CustomErrorB
}


#[derive(Debug, Error)]
#[error("CustomErrorB")]
struct CustomErrorB{
}



fn error_function_a() -> Result<(), CustomErrorA> {
    Err(CustomErrorA{source: CustomErrorB{}})
}

fn error_function_b() -> Result<(), CustomErrorB> {
    Err(CustomErrorB{})
}


fn main() {
    //error_function_a().unwrap();
    error_function_b().unwrap();

}