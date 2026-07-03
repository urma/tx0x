use rodio::{Decoder, OutputStream, Source};
use std::io::Cursor;
use std::sync::{Arc, Mutex};
use std::time::Duration;

const SAMPLE_NAMES: [&str; 8] = [
    "kick", "snare", "low_tom", "mid_tom", "hi_tom", "clap", "hihat", "cymbal",
];

struct Voice {
    track_index: usize,
    read_pos: f64,
}

struct SharedState {
    sample_counter: u64,
    voices: Vec<Voice>,
    triggers: Vec<usize>,
    samples: Vec<Vec<f32>>,
}

struct SequencerSource {
    state: Arc<Mutex<SharedState>>,
    sample_rate: u32,
    channel: u16,
    current_sample: f32,
}

impl SequencerSource {
    fn new(state: Arc<Mutex<SharedState>>, sample_rate: u32) -> Self {
        Self { state, sample_rate, channel: 0, current_sample: 0.0 }
    }
}

impl Iterator for SequencerSource {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if self.channel == 0 {
            let mut guard = self.state.lock().unwrap();
            let s = &mut *guard;

            for track_idx in s.triggers.drain(..) {
                if track_idx < s.samples.len() && !s.samples[track_idx].is_empty() {
                    s.voices.push(Voice { track_index: track_idx, read_pos: 0.0 });
                }
            }

            let mut mixed = 0.0f32;
            let mut i = 0;
            while i < s.voices.len() {
                let sample = &s.samples[s.voices[i].track_index];
                let pos = s.voices[i].read_pos as usize;
                if pos >= sample.len() {
                    s.voices.swap_remove(i);
                    continue;
                }
                let frac = s.voices[i].read_pos.fract();
                let s0 = sample[pos];
                let s1 = if pos + 1 < sample.len() { sample[pos + 1] } else { s0 };
                mixed += s0 + (s1 - s0) * frac as f32;
                s.voices[i].read_pos += 1.0;
                i += 1;
            }

            s.sample_counter += 1;
            self.current_sample = mixed.clamp(-1.0, 1.0);
            self.channel = 1;
        } else {
            self.channel = 0;
        }
        Some(self.current_sample)
    }
}

impl Source for SequencerSource {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        2
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

pub struct AudioEngine {
    _stream: OutputStream,
    state: Arc<Mutex<SharedState>>,
}

impl AudioEngine {
    pub fn new() -> Option<Self> {
        let (_stream, handle) = OutputStream::try_default().ok()?;
        let sample_rate = 44100;

        let samples: Vec<Vec<f32>> = SAMPLE_NAMES
            .iter()
            .map(|name| {
                let path = format!("samples/{}.wav", name);
                load_wav_as_mono(&path).unwrap_or_default()
            })
            .collect();

        if samples.iter().all(|s| s.is_empty()) {
            return None;
        }

        let state = Arc::new(Mutex::new(SharedState {
            sample_counter: 0,
            voices: Vec::new(),
            triggers: Vec::new(),
            samples,
        }));

        let source = SequencerSource::new(state.clone(), sample_rate);
        let _ = handle.play_raw(source);

        Some(Self { _stream, state })
    }

    pub fn trigger(&mut self, track: usize) {
        self.state.lock().unwrap().triggers.push(track);
    }

    pub fn sample_position(&self) -> u64 {
        self.state.lock().unwrap().sample_counter
    }
}

fn load_wav_as_mono(path: &str) -> Option<Vec<f32>> {
    let data = std::fs::read(path).ok()?;
    let decoder = Decoder::new(Cursor::new(data)).ok()?;
    let channels = decoder.channels();
    let samples: Vec<f32> = decoder.convert_samples().collect();
    if channels == 2 {
        Some(samples.chunks(2).map(|c| (c[0] + c[1]) * 0.5).collect())
    } else {
        Some(samples)
    }
}
