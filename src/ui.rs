use ratatui::{prelude::*, widgets::*};
use crate::app::{App, TRACK_COUNT, STEP_COUNT, TRACK_NAMES};

pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();

    if area.height < TRACK_COUNT as u16 + 3 || area.width < 58 {
        let msg = format!("need {}x{} min", 58, TRACK_COUNT + 3);
        frame.render_widget(Paragraph::new(msg).style(Style::default().fg(Color::Red)), area);
        return;
    }

    let chunks = Layout::vertical([
        Constraint::Length(2),
        Constraint::Length(TRACK_COUNT as u16),
        Constraint::Length(1),
    ])
    .split(area);

    render_header(frame, chunks[0], app);
    render_grid(frame, chunks[1], app);
    render_controls(frame, chunks[2], app);
}

fn fmt_step_num(n: usize) -> String {
    match n {
        0..=9 => format!(" {} ", n),
        _ => format!("{:<3}", n),
    }
}

fn render_header(frame: &mut Frame, area: Rect, app: &App) {
    let rows = Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).split(area);

    let play_indicator = if app.playing {
        Span::styled(" ▶", Style::default().fg(Color::Green))
    } else {
        Span::styled(" ■", Style::default().fg(Color::Red).add_modifier(Modifier::DIM))
    };

    let title = Line::from(vec![
        Span::styled(" tx0x ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::styled(format!("BPM: {}", app.bpm), Style::default().fg(Color::Yellow)),
        play_indicator,
    ]);
    frame.render_widget(Paragraph::new(title), rows[0]);

    let mut spans = Vec::with_capacity(1 + STEP_COUNT + 3);
    spans.push(Span::raw("       "));
    for group in 0..4 {
        if group > 0 {
            spans.push(Span::styled("│", Style::default().fg(Color::DarkGray)));
        }
        for step_in_group in 0..4 {
            let step = group * 4 + step_in_group;
            let is_current = step == app.current_step && app.playing;
            let span = if is_current {
                Span::styled(
                    fmt_step_num(step + 1),
                    Style::default().fg(Color::Black).bg(Color::Yellow),
                )
            } else {
                Span::styled(fmt_step_num(step + 1), Style::default().fg(Color::Gray))
            };
            spans.push(span);
        }
    }
    frame.render_widget(Paragraph::new(Line::from(spans)), rows[1]);
}

fn render_grid(frame: &mut Frame, area: Rect, app: &App) {
    let rows = Layout::vertical(vec![Constraint::Length(1); TRACK_COUNT]).split(area);

    for i in 0..TRACK_COUNT {
        let is_selected = i == app.selected_track;
        let bg = if is_selected {
            Color::DarkGray
        } else if i % 2 == 0 {
            Color::Reset
        } else {
            Color::Indexed(236)
        };
        let row_style = Style::default().bg(bg);
        let line = track_line(i, app);
        frame.render_widget(Paragraph::new(line).style(row_style), rows[i]);
    }
}

fn cell_span(track_idx: usize, step: usize, is_active: bool, app: &App) -> Span<'static> {
    let is_current = step == app.current_step;
    let is_cursor = step == app.cursor_step && track_idx == app.selected_track;

    let content = if is_active { "[█]" } else { "[·]" };

    let style = if is_current && app.playing {
        if is_active {
            Style::default().fg(Color::Green).bg(Color::Yellow)
        } else {
            Style::default().fg(Color::DarkGray).bg(Color::Yellow)
        }
    } else if is_cursor {
        if is_active {
            Style::default().fg(Color::Green).bg(Color::Blue)
        } else {
            Style::default().fg(Color::White).bg(Color::Blue)
        }
    } else if is_active {
        Style::default().fg(Color::Green)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    Span::styled(content, style)
}

fn track_line(track_idx: usize, app: &App) -> Line<'static> {
    let track = &app.tracks[track_idx];
    let mut spans = Vec::with_capacity(1 + STEP_COUNT + 3);

    let name = format!(" {:<6}", TRACK_NAMES[track_idx]);
    spans.push(Span::raw(name));

    for group in 0..4 {
        if group > 0 {
            spans.push(Span::styled("│", Style::default().fg(Color::DarkGray)));
        }
        for step_in_group in 0..4 {
            let step = group * 4 + step_in_group;
            let is_active = track.steps[step];
            spans.push(cell_span(track_idx, step, is_active, app));
        }
    }
    Line::from(spans)
}

fn render_controls(frame: &mut Frame, area: Rect, app: &App) {
    let status = if app.playing {
        format!(" {} BPM  ▶", app.bpm)
    } else {
        format!(" {} BPM  ■", app.bpm)
    };
    let controls = format!(
        " [Space] Toggle  [↑↓] Track  [←→] Step  [Enter] Play/Stop  [+/-] BPM  {}  [q] Quit",
        status
    );
    let para = Paragraph::new(Span::styled(controls, Style::default().fg(Color::DarkGray)));
    frame.render_widget(para, area);
}
