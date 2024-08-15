use crate::{definitions::Float, function::Function};

pub fn sum_waves(this: Vec<Float>, other: Vec<Float>) -> Vec<Float> {
    let identity = 0.0;
    let max_len = this.len().max(other.len());
    let mut result = Vec::with_capacity(max_len);

    for i in 0..max_len {
        let val1 = this.get(i).unwrap_or(&identity);
        let val2 = other.get(i).unwrap_or(&identity);
        result.push(*val1 + *val2);
    }

    result
}

/// Scales the wave vector with a significance factor.
pub fn scale_wave(this: Vec<Float>, c: Float) -> Vec<Float> {
    this.into_iter().map(|x| x * c).collect()
}

pub fn clip_value(x: Float, upper_lower_boundary: Float) -> Float {
    let upper_lower_boundary = upper_lower_boundary.abs();

    if x > upper_lower_boundary {
        upper_lower_boundary
    } else if x < -upper_lower_boundary {
        -upper_lower_boundary
    } else {
        x
    }
}

pub fn turn_wave_to_fn(wave: Vec<Float>, samplerate: Option<u32>)-> Function{
    let samplerate = samplerate.unwrap_or(44100);
    let seclen = (wave.len() as Float)/(samplerate as Float);
    Function::Function(Box::new(move |t: Float| -> Float {
        let mut t = ((t % seclen) + seclen) % seclen;
        t *= samplerate as Float;
        wave[t as usize]
    }))
}