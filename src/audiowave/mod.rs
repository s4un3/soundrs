mod utils;

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

impl<N: Float, M: Float> AudioWave<N, M> {
    fn new(
        freq: Function<N>,
        amp: Function<N>,
        duration: M,
        latency: M,
        samplerate: u32,
        waveform: Function<N>,
        yclip: N,
    ) -> AudioWave<N, M> {
        todo!()
    }

    fn add(self, other: AudioWave<N, M>) -> Option<AudioWave<N, M>> {
        if self.samplerate != other.samplerate {return None}
        let significance = self.significance + other.significance;
        let wave = utils::scale_wave(
            utils::sum_waves(
                utils::scale_wave(self.wave, self.significance),
                scale_wave(other.wave, other.significance),
            ),
            N::from(1.0).expect("1.0 should be a valid float.") / significance,
        );
        let duration = self.duration.max(other.duration);
        let samplerate = self.samplerate;
        Some(AudioWave{ significance, samplerate, duration, wave })
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
