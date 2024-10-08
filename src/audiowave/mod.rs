mod utils;

use crate::definitions::{Float, PI};
use crate::function::Function;
use hound;
use std::error::Error;
use std::fmt::Display;
use std::i16;
use utils::{clip_value, scale_wave};

#[derive(Debug)]
pub enum WavImportError {
    IOErr(std::io::Error),
    ParseError(String),
}

impl Display for WavImportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WavImportError::IOErr(e) => write!(f, "{}", e),
            WavImportError::ParseError(s) => write!(f, "Error importing wav: {}", s),
        }
    }
}

impl Error for WavImportError {}

#[derive(Clone)]
pub struct AudioWave {
    significance: Float,
    samplerate: u32,
    duration: Float, // TODO: maybe use `std::time::Duration`?
    pub wave: Vec<Float>,
}

impl AudioWave {
    pub fn new(
        freq: &Function,
        amp: &Function,
        duration: &Float,
        latency: Option<Float>,
        samplerate: Option<u32>,
        waveform: Option<Function>,
        yclip: Option<Float>,
    ) -> Option<AudioWave> {
        let latency: Float = latency.unwrap_or(0.0);
        let samplerate: u32 = samplerate.unwrap_or(44100);
        let yclip: Float = yclip.unwrap_or(1.0);
        let waveform: Function = waveform.unwrap_or(Function::Function(Box::new(|t: Float| {
            (2.0 * PI * t).sin()
        })));

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
        let samplerate = self.samplerate;
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

        let mut first_wave: Vec<Float> =
            scale_wave(self.wave, new_significance / self.significance);
        let second_wave: Vec<Float> = scale_wave(other.wave, new_significance / other.significance);
        first_wave.extend(second_wave.into_iter());

        Some(AudioWave {
            significance: new_significance,
            samplerate: self.samplerate,
            duration: self.duration + other.duration,
            wave: first_wave,
        })
    }

    pub fn get_samplerate(&self) -> u32 {
        self.samplerate
    }

    pub fn get_duration(&self) -> Float {
        self.duration
    }

    pub fn change_sample_rate(self, new_sample_rate: u32) -> AudioWave {
        todo!()
    }

    pub fn play(&self) {
        todo!()
    }

    pub fn export_wav(self, path: &std::path::Path) -> Result<(), hound::Error> {
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: self.get_samplerate(),
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut writer = hound::WavWriter::create("sine.wav", spec)?;

        for sample in self.wave.into_iter().map(|x| {
            (x / self.significance) * (i16::MAX as Float) // Normalize by significance then scale to i16 range
        }) {
            writer
                .write_sample(sample as i16)
                .expect("should be able to write i16 to 16 bit wav")
        }

        writer.finalize()?;
        Ok(())
    }

    pub fn from_wav(path: &std::path::Path) -> Result<Self, WavImportError> {
        todo!()
    }
}
