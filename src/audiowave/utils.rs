use crate::definitions::Float;

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
