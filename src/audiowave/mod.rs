mod utils;

use crate::function::Function;
use num::{abs, traits::FloatConst, Float, Signed};
use std::{time::Duration, vec};
use utils::{clip_wave, scale_wave};

pub trait Number: Float + Signed + FloatConst + std::ops::AddAssign {}
impl<T> Number for T where T: Float + Signed + FloatConst + std::ops::AddAssign {}

#[derive(Clone)]
struct AudioWave<N: Number> {
    significance: u32,
    samplerate: u32,
    duration: N, // TODO: maybe use `std::time:Duration`?
    wave: Vec<N>,
}

impl<N: Number> AudioWave<N> {
    fn new(
        freq: Function<N>,
        amp: Function<N>,
        duration: N,
        latency: Option<N>,
        samplerate: Option<u32>,
        waveform: Option<Function<N>>,
        yclip: Option<N>,
    ) -> Option<AudioWave<N>> {
        let latency = latency.unwrap_or(N::zero());
        let samplerate = samplerate.unwrap_or(num::zero());
        let yclip = yclip.unwrap_or(N::zero());
        let waveform = waveform.unwrap_or(Function::Function(|t: N| {
            (N::from(2).expect("2 should be a valid Float") * N::PI() * t).sin()
        }));

        let f_samplerate: N = N::from(samplerate)?;
        let computed_capacity = f_samplerate * duration;
        let veccapacity: usize = computed_capacity.to_usize()?;
        let mut wave: Vec<N> = Vec::with_capacity(veccapacity);

        let duration = abs(duration);
        let latency = abs(latency);
        let yclip = abs(yclip);

        let significance: u32 = 1;

        let mut Y: N = N::zero();
        let mut t: N = N::zero();
        let dt: N = N::from(1).expect("1 should be a valid Float")
            / N::from(samplerate).expect("The samplerate should be convertable to float");

        if latency > N::zero() {
            t = -latency;
            while t < N::zero() {
                wave.push(N::zero());
                t += dt;
            }
        }
        while t < duration {
            Y += freq.get(&t) * dt;
            wave.push(clip_wave(&(waveform.get(&Y) * amp.get(&t)), &yclip));
            t += dt;
        }
        Some(AudioWave {
            significance,
            samplerate,
            duration,
            wave,
        })
    }

    fn add(self, other: AudioWave<N>) -> Option<AudioWave<N>> {
        if self.samplerate != other.samplerate {
            return None;
        }
        let significance = self.significance + other.significance;
        let wave = utils::sum_waves(self.wave, other.wave);
        let duration = self.duration.max(other.duration);
        let samplerate = self.samplerate;
        Some(AudioWave {
            significance,
            samplerate,
            duration,
            wave,
        })
    }

    fn append(self, other: AudioWave<N>) -> Option<AudioWave<N>> {
        todo!()
    }

    fn change_sample_rate(self, new_sample_rate: u32) -> AudioWave<N> {
        todo!()
    }

    fn play(&self) {
        todo!()
    }

    fn export_wav(&self, path: &std::path::Path) -> Result<(), std::io::Error> {
        todo!()
    }
}
