mod utils;

use std::error::Error;
use crate::definitions::{Float, PI};
use crate::function::Function;
use utils::{clip_value, scale_wave};

enum WavImportError {
    IOErr(std::io::Error),
    ParseError(str)
}

impl Error for WavImportError {}

#[derive(Clone)]
pub struct AudioWave {
    significance: Float,
    samplerate: u32,
    duration: Float, // TODO: maybe use `std::time::Duration`?
    wave: Vec<Float>,
}

impl AudioWave {
    pub fn new(
        freq: Function,
        amp: Function,
        duration: Float,
        latency: Option<Float>,
        samplerate: Option<u32>,
        waveform: Option<Function>,
        yclip: Option<Float>,
    ) -> Option<AudioWave> {
        let latency: Float = latency.unwrap_or(0.0);
        let samplerate: Float = samplerate.unwrap_or(44100);
        let yclip: Float = yclip.unwrap_or(1.0);
        let waveform: Float = waveform.unwrap_or(Function::Function(|t: Float| (2.0 * PI * t).sin()));

        let f_samplerate: Float = samplerate as Float;
        let computed_capacity: Float = f_samplerate * duration;
        let veccapacity: usize = computed_capacity.ceil() as usize;
        let mut wave: Vec<Float> = Vec::with_capacity(veccapacity);

        let duration: Float = duration.abs();
        let latency: Float = latency.abs();
        let yclip: Float = yclip.abs();

        let significance: Float = 1.0;

        let mut y: Float = 0.0;
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
            y += freq.get(t) * dt;
            wave.push(clip_value(waveform.get(y) * amp.get(t), yclip));
            t += dt;
        }
        Some(AudioWave {
            significance,
            samplerate,
            duration,
            wave,
        })
    }

    pub fn add(self, other: AudioWave) -> Option<AudioWave> {
        if self.samplerate != other.samplerate {
            return None;
        }
        let significance: Float = self.significance + other.significance;
        let wave: Vec<Float> = utils::sum_waves(self.wave, other.wave);
        let duration: Float = self.duration.max(other.duration);
        let samplerate: Float = self.samplerate;
        Some(AudioWave {
            significance,
            samplerate,
            duration,
            wave,
        })
    }

    pub fn append(self, other: AudioWave, new_significance: Option<Float>) -> Option<AudioWave> {
        if self.samplerate != other.samplerate {
            return None;
        }

        let new_significance: Float = new_significance.unwrap_or(1.0);
        
        let mut first_wave: Vec<Float> = scale_wave(self.wave, new_significance / self.significance);
        let second_wave: Vec<Float> = scale_wave(other.wave, new_significance / other.significance);
        first_wave.extend(second_wave.into_iter());
        
        Some(AudioWave {
            significance: new_significance,
            samplerate: self.samplerate,
            duration: self.duration + other.duration,
            wave: first_wave,
        })
    }

    pub fn change_sample_rate(self, new_sample_rate: u32) -> AudioWave {
        todo!()
    }

    pub fn play(&self) {
        todo!()
    }

    pub fn export_wav(&self, path: &std::path::Path) -> Result<(), std::io::Error> {
        todo!()
    }

    pub fn from_wav(path: &std::path::Path) -> Result<Self, WavImportError> {
        todo!()
    }
}
