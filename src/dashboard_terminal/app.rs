use crate::dashboard_terminal::domain;
use crate::dashboard_terminal::widgets::{
    dashboard_error, dashboard_loading, dashboard_sensor_data,
};
use crate::shared::types::sensor_data::SensorData;
use crossterm::event;
use crossterm::event::Event as CEvent;
use crossterm::event::KeyCode;
use std::path::PathBuf;
use std::sync::mpsc;
use std::time::{Duration, Instant};
use std::{io, thread};
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders};
use tui::Terminal;
use crate::dashboard_terminal::app_error::AppErrorResult;

#[derive(Debug)]
pub enum AppState {
    Loading,
    Dashboard(SensorData),
    Error,
}

#[derive(Debug)]
pub struct App<B: Backend> {
    terminal: Terminal<B>,
    state: AppState,
    tick_countdown_to_fetch_data: u32,
    output_dir_path: PathBuf,
}

impl<B: Backend> App<B> {
    pub fn new(backend: B, output_dir_path: PathBuf) -> AppErrorResult<Self> {
        let terminal = Terminal::new(backend)?;
        Ok(Self {
            terminal,
            state: AppState::Loading,
            tick_countdown_to_fetch_data: 0,
            output_dir_path,
        })
    }

    pub fn run(&mut self) -> Result<(), io::Error> {
        // Setup input handling
        let (tx, rx) = mpsc::channel();

        let tick_rate = Duration::from_millis(1000);
        thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                // poll for tick rate duration, if no events, sent tick event.
                if event::poll(tick_rate - last_tick.elapsed()).unwrap() {
                    if let CEvent::Key(key) = event::read().unwrap() {
                        tx.send(Event::Input(key)).unwrap();
                    }
                }
                if last_tick.elapsed() >= tick_rate {
                    tx.send(Event::Tick).unwrap();
                    last_tick = Instant::now();
                }
            }
        });

        self.terminal.clear()?;
        self.terminal.hide_cursor()?;

        loop {
            match &self.state {
                AppState::Loading => self
                    .terminal
                    .draw(|mut frame| dashboard_loading(&mut frame))?,
                AppState::Dashboard(sensor_data) => self
                    .terminal
                    .draw(|mut frame| dashboard_sensor_data(&mut frame, sensor_data))?,
                AppState::Error => self
                    .terminal
                    .draw(|mut frame| dashboard_error(&mut frame))?,
            }

            match rx.recv().unwrap() {
                Event::Input(event) => match event.code {
                    // KeyCode::Char('q') => {
                    //     disable_raw_mode()?;
                    //     execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                    //     terminal.show_cursor()?;
                    //     break;
                    // }
                    // KeyCode::Char(c) => app.on_key(c),
                    // KeyCode::Left => app.on_left(),
                    // KeyCode::Up => app.on_up(),
                    // KeyCode::Right => app.on_right(),
                    // KeyCode::Down => app.on_down(),
                    _ => {}
                },
                Event::Tick => {
                    self.on_tick();
                }
            }
        }
    }

    fn update_data(&mut self) {
        let data = domain::read_latest_sensor_data_from_directory(&self.output_dir_path);

        if let Ok(sensor_data) = data {
            self.state = AppState::Dashboard(sensor_data);
        } else {
            self.state = AppState::Error;
            // let debug_sensor_data = SensorData::new(chrono::Utc::now(), 1., 2., 3., 4., 5., 6., 7.);
            // self.state = AppState::Dashboard(debug_sensor_data)
        }
    }

    fn on_tick(&mut self) {
        if self.tick_countdown_to_fetch_data == 0 {
            self.update_data();
            self.tick_countdown_to_fetch_data = 60 * 5;
        }

        self.tick_countdown_to_fetch_data -= 1;
    }
}

enum Event<I> {
    Input(I),
    Tick,
}
