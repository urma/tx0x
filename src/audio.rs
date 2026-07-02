use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::io::Cursor;

pub struct AudioEngine {
    _stream: OutputStream,
    handle: OutputStreamHandle,
    samples: Vec<Option<Vec<u8>>>,
    sinks: Vec<Sink>,
}

const SAMPLE_NAMES: [&str; 8] = [
    "kick", "snare", "low_tom", "mid_tom", "hi_tom", "clap", "hihat", "cymbal",
];

impl AudioEngine {
    pub fn new() -> Option<Self> {
        let (_stream, handle) = OutputStream::try_default().ok()?;

        let samples: Vec<Option<Vec<u8>>> = SAMPLE_NAMES
            .iter()
            .map(|name| {
                let path = format!("samples/{}.wav", name);
                std::fs::read(&path).ok()
            })
            .collect();

        Some(Self { _stream, handle, samples, sinks: Vec::new() })
    }

    pub fn play(&mut self, track: usize) {
        if let Some(Some(data)) = self.samples.get(track) {
            if let Ok(source) = Decoder::new(Cursor::new(data.clone())) {
                if let Ok(sink) = Sink::try_new(&self.handle) {
                    sink.append(source);
                    self.sinks.push(sink);
                }
            }
        }
        self.cleanup();
    }

    fn cleanup(&mut self) {
        self.sinks.retain(|s| !s.empty());
    }
}
