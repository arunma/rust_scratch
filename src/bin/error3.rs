use std::fmt::{Display, Formatter};
use anyhow::{anyhow, bail, Result};
use thiserror::Error;
use crate::AppError::{CustomErrorA, CustomErrorB};

#[derive(Debug, Error)]
enum AppError {
    #[error("CustomErrorA with message {message:?}")]
    CustomErrorA { message: String },
    #[error("CustomErrorB with message1 {message1:?} and message 2 {message2:?}")]
    CustomErrorB { message1: String, message2: String },
}


fn error_function_a() -> Result<()> {
    bail!(CustomErrorA { message: "customer error message".to_string() })
}

fn error_function_b() -> Result<()> {
    bail!(CustomErrorB { message1: "customer error message 1".to_string(), message2: "customer error message 2".to_string() })
}

fn error_function_c() -> Result<()> {
    Err(anyhow!("Adhoc error Function c"))
}

fn main() {
    //error_function_a().unwrap();
    error_function_b().unwrap();
    //error_function_c().unwrap();
}