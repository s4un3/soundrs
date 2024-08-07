use num::Float;

pub enum Function<T: Float> {
    Const(T),
    Function(fn(T) -> T),
}

impl<T: Float> Function<T> {
    pub fn get(self, t: T) -> T {
        match self {
            Function::Const(c) => c,
            Function::Function(f) => f(t),
        }
    }
}