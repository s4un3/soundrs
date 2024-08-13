mod utils;

use crate::definitions::{Float, PI};
use crate::function::Function;
use utils::{clip_value, scale_wave};

#[derive(Clone)]
struct AudioWave {
    significance: u32,
    samplerate: u32,
    duration: Float, // TODO: maybe use `std::time::Duration`?
    wave: Vec<Float>,
}

impl AudioWave {
    fn new(
        freq: Function,
        amp: Function,
        duration: Float,
        latency: Option<Float>,
        samplerate: Option<u32>,
        waveform: Option<Function>,
        yclip: Option<Float>,
    ) -> Option<AudioWave> {
        let latency = latency.unwrap_or(0.0);
        let samplerate = samplerate.unwrap_or(0);
        let yclip = yclip.unwrap_or(0.0);
        let waveform = waveform.unwrap_or(Function::Function(|t: Float| (2.0 * PI * t).sin()));

        let f_samplerate: Float = samplerate as Float;
        let computed_capacity = f_samplerate * duration;
        let veccapacity: usize = computed_capacity.ceil() as usize;
        let mut wave: Vec<Float> = Vec::with_capacity(veccapacity);

        let duration = duration.abs();
        let latency = latency.abs();
        let yclip = yclip.abs();

        let significance: u32 = 1;

        let mut Y: Float = 0.0;
        let mut t: Float = 0.0;
        let dt: Float = 1.0 / samplerate as Float;

        if latency > 0.0 {
            t = -latency;
            while t < 0.0 {
                wave.push(0.0);
                t += dt;
            }
        }
        while t < duration {
            Y += freq.get(t) * dt;
            wave.push(clip_value(waveform.get(Y) * amp.get(t), yclip));
            t += dt;
        }
        Some(AudioWave {
            significance,
            samplerate,
            duration,
            wave,
        })
    }

    fn add(self, other: AudioWave) -> Option<AudioWave> {
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

    fn append(self, other: AudioWave) -> Option<AudioWave> {
        todo!()
    }

    fn change_sample_rate(self, new_sample_rate: u32) -> AudioWave {
        todo!()
    }

    fn play(&self) {
        todo!()
    }

    fn export_wav(&self, path: &std::path::Path) -> Result<(), std::io::Error> {
        todo!()
    }
}
