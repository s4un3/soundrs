use num::abs;
use crate::audiowave::Number;
 
pub fn sum_waves<N: Number>(this: Vec<N>, other: Vec<N>) -> Vec<N> {
    let identity = num::zero();
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
/// 
/// Returns `Some(Vec<N>)` on success,
/// otherwise `None` (if cast from `u32` to `N` failed).
pub fn scale_wave<N: Number>(this: Vec<N>, c: N) -> Vec<N> {
    this.into_iter().map(|x| {x*c}).collect()
}

pub fn clip_wave<N: Number>(x: &N, upper_lower_boundary: &N) -> N {
    let upper_lower_boundary = abs(*upper_lower_boundary);

    if *x > upper_lower_boundary {
        upper_lower_boundary
    } else if *x < -upper_lower_boundary {
        -upper_lower_boundary
    } else {
        *x
    }
}
