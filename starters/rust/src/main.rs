#![allow(unused_imports)]
mod directbot;
mod event;
use event::{Config, Event, Events};

use directbot::*;
use reqwest::get;
use std::{env::args, error::Error, f64, io::stdout, time::Duration};

use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::Terminal;
use tui::{
    layout::*,
    style::Color,
    widgets::{canvas::*, *},
};

const CANVAS_SIZE: f64 = 500.0;

#[allow(unused_variables)]
fn main() -> Result<(), Box<dyn Error>> {
    // https://github.com/noops-challenge/directbot/blob/master/API.md
    let url = args().nth(1).unwrap_or(
        "https://api.noopschallenge.com/directbot?count=1000&pattern=invader".to_string(),
    );
    let stdout = stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    // Setup event handlers
    let config = Config {
        tick_rate: Duration::from_millis(2000),
        ..Default::default()
    };
    let events = Events::with_config(config);

    let mut data: Response = get(&url)?.json()?;
    loop {
        terminal.draw(|mut f| {
            let size = f.size();
            Canvas::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Github Noops Directbot"),
                )
                .x_bounds([-CANVAS_SIZE, CANVAS_SIZE])
                .y_bounds([-CANVAS_SIZE, CANVAS_SIZE])
                .paint(|ctx| {
                    for direction in &data.directions {
                        let directbot::Direction { speed, distance, direction, coordinates } = direction;
                        if let Some(coordinates) = coordinates {
                            ctx.draw(&Line {
                                x1: guard(coordinates.a.x),
                                y1: guard(coordinates.a.y),
                                x2: guard(coordinates.b.x),
                                y2: guard(coordinates.b.y),
                                color: Color::Indexed(speed+distance),
                            });
                        }
                    }
                })
                .render(&mut f, size);
        })?;

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    terminal.show_cursor()?;
                    break;
                }
                _ => {}
            },
            Event::Tick => data = get(&url)?.json()?,
        }
    }

    Ok(())
}

fn guard(val: f64) -> f64 {
    if val.abs() > CANVAS_SIZE {
        CANVAS_SIZE * val.signum() - val.abs()
    } else {
        val
    }
}
