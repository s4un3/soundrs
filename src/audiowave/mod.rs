mod utils;

use std::vec;

use crate::function::Function;
use num::Float;
use utils::scale_wave;

#[derive(Clone)]
struct AudioWave<N: Float, M: Float> {
    significance: N,
    samplerate: u32,
    duration: M, // TODO: maybe use `std::time:Duration`?
    wave: Vec<N>,
}

impl<N: Float, M: Float> AudioWave<N, M>
where
    usize: From<M>,
    M: From<u32>,
{
    fn new(
        freq: Function<N>,
        amp: Function<N>,
        duration: M,
        latency: M,
        samplerate: u32,
        waveform: Function<N>,
        yclip: N
    ) -> Option<AudioWave<N, M>> {
        let f_samplerate: M = samplerate.into();
        let computed_capacity = f_samplerate * duration;
        let veccapacity: usize = usize::from(computed_capacity);
        let mut vec: Vec<N> = Vec::with_capacity(veccapacity);
        todo!()
    }

    fn add(self, other: AudioWave<N, M>) -> Option<AudioWave<N, M>> {
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

    fn append(self, other: AudioWave<N, M>) -> Option<AudioWave<N, M>> {
        todo!()
    }

    fn change_sample_rate(self, new_sample_rate: u32) -> AudioWave<N, M> {
        todo!()
    }

    fn play(&self) {
        todo!()
    }

    fn export_wav(&self, path: &std::path::Path) -> Result<(), std::io::Error> {
        todo!()
    }
}
