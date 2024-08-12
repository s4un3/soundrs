use num::{Float, Num};
use crate::audiowave::Number;

pub enum Function<T: Number> {
    Const(T),
    Function(fn(T) -> T),
}

impl<T: Number> Function<T> {
    pub fn get(&self, t: &T) -> T {
        match self {
            Function::Const(c) => *c,
            Function::Function(f) => f(*t),
        }
    }
}