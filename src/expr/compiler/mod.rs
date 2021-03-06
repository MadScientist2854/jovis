mod env;
mod dtype;
mod interpreter;
mod type_checker;

use env::Environment;
use dtype::{DType, Object};
use super::Expr;

pub use interpreter::Interpret;