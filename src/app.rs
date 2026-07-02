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
    pub step_time: Instant,
    pub playing: bool,
    pub bpm: u32,
}

impl App {
    pub fn new() -> Self {
        Self {
            tracks: (0..TRACK_COUNT).map(|_| Track::new()).collect(),
            selected_track: 0,
            cursor_step: 0,
            current_step: 0,
            step_time: Instant::now(),
            playing: false,
            bpm: 120,
        }
    }

    pub fn toggle_step(&mut self) {
        self.tracks[self.selected_track].steps[self.cursor_step] ^= true;
    }

    pub fn advance(&mut self) {
        self.current_step = (self.current_step + 1) % STEP_COUNT;
        self.step_time = Instant::now();
    }

    pub fn start(&mut self) {
        self.current_step = 0;
        self.step_time = Instant::now();
        self.playing = true;
    }

    pub fn stop(&mut self) {
        self.playing = false;
    }

    pub fn step_duration_secs(&self) -> f64 {
        60.0 / (self.bpm as f64 * 4.0)
    }

    pub fn time_since_step(&self) -> f64 {
        self.step_time.elapsed().as_secs_f64()
    }

    pub fn should_advance(&self) -> bool {
        self.playing && self.time_since_step() >= self.step_duration_secs()
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
