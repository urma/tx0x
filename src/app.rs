use std::time::Instant;

pub const TRACK_COUNT: usize = 8;
pub const STEP_COUNT: usize = 16;
pub const TRACK_NAMES: [&str; TRACK_COUNT] = [
    "Kick", "Snare", "L.Tom", "M.Tom", "H.Tom", "Clap", "Hihat", "Cymbal",
];

#[derive(Clone)]
pub struct Track {
    pub steps: [bool; STEP_COUNT],
}

impl Track {
    pub fn new() -> Self {
        Self { steps: [false; STEP_COUNT] }
    }
}

pub struct App {
    pub tracks: Vec<Track>,
    pub selected_track: usize,
    pub cursor_step: usize,
    pub current_step: usize,
    pub playing: bool,
    pub bpm: u32,
    pub sample_rate: u32,
    pub sample_counter: f64,
    pub next_step_sample: f64,
    pub last_update: Instant,
}

impl App {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            tracks: (0..TRACK_COUNT).map(|_| Track::new()).collect(),
            selected_track: 0,
            cursor_step: 0,
            current_step: 0,
            playing: false,
            bpm: 120,
            sample_rate,
            sample_counter: 0.0,
            next_step_sample: 0.0,
            last_update: Instant::now(),
        }
    }

    pub fn toggle_step(&mut self) {
        self.tracks[self.selected_track].steps[self.cursor_step] ^= true;
    }

    pub fn should_advance(&self) -> bool {
        self.playing && self.sample_counter >= self.next_step_sample
    }

    pub fn advance(&mut self) {
        self.current_step = (self.current_step + 1) % STEP_COUNT;
        self.next_step_sample += self.step_duration_samples();
    }

    pub fn start(&mut self) {
        self.current_step = 0;
        self.next_step_sample = self.sample_counter + self.step_duration_samples();
        self.playing = true;
    }

    pub fn stop(&mut self) {
        self.playing = false;
    }

    pub fn step_duration_samples(&self) -> f64 {
        60.0 / (self.bpm as f64 * 4.0) * self.sample_rate as f64
    }

    pub fn prev_track(&mut self) {
        self.selected_track = self.selected_track.saturating_sub(1);
    }

    pub fn next_track(&mut self) {
        self.selected_track = (self.selected_track + 1).min(TRACK_COUNT - 1);
    }

    pub fn prev_step(&mut self) {
        self.cursor_step = self.cursor_step.saturating_sub(1);
    }

    pub fn next_step(&mut self) {
        self.cursor_step = (self.cursor_step + 1).min(STEP_COUNT - 1);
    }

    pub fn increase_bpm(&mut self) {
        self.bpm = (self.bpm + 5).min(300);
    }

    pub fn decrease_bpm(&mut self) {
        self.bpm = self.bpm.saturating_sub(5).max(40);
    }
}
