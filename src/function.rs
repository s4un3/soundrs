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

pub fn turn_wave_to_fn(wave: Vec<Float>, samplerate: Option<u32>) -> Function {
    let samplerate = samplerate.unwrap_or(44100);
    let seclen = (wave.len() as Float) / (samplerate as Float);
    Function::Function(Box::new(move |t: Float| -> Float {
        let mut t = ((t % seclen) + seclen) % seclen;
        t *= samplerate as Float;
        wave[t as usize]
    }))
}
