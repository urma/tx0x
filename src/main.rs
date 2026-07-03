mod app;
mod audio;
mod ui;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::time::{Duration, Instant};

use app::App;
use audio::AudioEngine;

fn main() -> Result<()> {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic| {
        let _ = restore_terminal();
        original_hook(panic);
    }));

    let mut terminal = setup_terminal()?;
    let mut app = App::new(44100);
    let mut audio = AudioEngine::new();
    if audio.is_none() {
        eprintln!("Warning: No audio device or no samples found. Running without sound.");
    }

    let res = run(&mut terminal, &mut app, &mut audio);

    restore_terminal()?;
    res
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal() -> Result<()> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}

fn run(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    audio: &mut Option<AudioEngine>,
) -> Result<()> {
    loop {
        if let Some(audio) = audio {
            app.sample_counter = audio.sample_position() as f64;
        } else {
            let now = Instant::now();
            let elapsed = now.duration_since(app.last_update).as_secs_f64();
            app.sample_counter += elapsed * app.sample_rate as f64;
            app.last_update = now;
        }

        while app.should_advance() {
            app.advance();
            let step = app.current_step;
            if let Some(audio) = audio {
                for (i, track) in app.tracks.iter().enumerate() {
                    if track.steps[step] {
                        audio.trigger(i);
                    }
                }
            }
        }

        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Enter => {
                        if app.playing {
                            app.stop();
                        } else {
                            app.start();
                        }
                    }
                    KeyCode::Char(' ') => app.toggle_step(),
                    KeyCode::Up | KeyCode::Char('k') => app.prev_track(),
                    KeyCode::Down | KeyCode::Char('j') => app.next_track(),
                    KeyCode::Left | KeyCode::Char('h') => app.prev_step(),
                    KeyCode::Right | KeyCode::Char('l') => app.next_step(),
                    KeyCode::Char('+') | KeyCode::Char('=') => app.increase_bpm(),
                    KeyCode::Char('-') | KeyCode::Char('_') => app.decrease_bpm(),
                    _ => {}
                }
            }
        }

        terminal.draw(|f| ui::render(f, app))?;
    }

    Ok(())
}
