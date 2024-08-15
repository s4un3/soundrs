use crate::definitions::Float;

pub enum Function {
    Const(Float),
    Function(Box<dyn Fn(Float) -> Float>),
}

impl Function {
    pub fn get(&self, t: Float) -> Float {
        match self {
            Function::Const(c) => *c,
            Function::Function(f) => f(t),
        }
    }
}
