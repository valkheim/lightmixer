use std::{
    error::Error,
    io,
    mem::swap,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Gauge},
    Frame, Terminal,
};

use crate::light::controller::{update_controller, Controller};

struct App {
    controllers: Vec<Controller>,
    cursor: usize,
}

impl App {
    pub fn new(controllers: Vec<Controller>) -> Result<App, Box<dyn Error>> {
        Ok(App {
            controllers: controllers,
            cursor: 0,
        })
    }

    fn render_main_block<B: Backend>(&self, f: &mut Frame<B>) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title("light mixer")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);
        f.render_widget(block, f.size());
    }

    fn render_gauges<B: Backend>(&self, f: &mut Frame<B>) {
        let constraints: Vec<_> = std::iter::repeat_with(|| Constraint::Length(1)) // or Percentage((100 / controllers.len()) as u16)
            .take(self.controllers.len() + 1)
            .collect();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(constraints.as_ref())
            .split(f.size());

        for (i, c) in (&self.controllers).into_iter().enumerate() {
            let label = Span::styled(
                format!(
                    "{} ({} / {} = {} %)",
                    c.path.display().to_string(),
                    c.brightness,
                    c.max_brightness,
                    c.brightness * 100 / c.max_brightness
                ),
                Style::default(),
            );

            let mut fg = Color::Black;
            let mut bg = Color::LightYellow;
            if i % 2 == 0 {
                bg = Color::Yellow;
            }

            if i == self.cursor {
                swap(&mut fg, &mut bg);
            }

            let gauge = Gauge::default()
                .block(Block::default())
                .gauge_style(Style::default().bg(bg).fg(fg))
                .percent((c.brightness * 100 / c.max_brightness) as u16)
                .label(label);

            f.render_widget(gauge, chunks[i]);
        }
    }

    pub fn render<B: Backend>(&self, f: &mut Frame<B>) {
        self.render_main_block(f);
        self.render_gauges(f);
    }

    fn down(&mut self) {
        self.cursor = (self.cursor + 1) % self.controllers.len()
    }

    fn up(&mut self) {
        if self.cursor == 0 {
            self.cursor = self.controllers.len() - 1
        } else {
            self.cursor -= 1
        }
    }

    fn left(&mut self) {
        let controller = &mut self.controllers[self.cursor];
        if controller.brightness == 0 {
            return;
        }

        controller.brightness -= 1;
        if let Err(_) = update_controller(controller, controller.brightness) {
            return;
        }
    }

    fn right(&mut self) {
        let controller = &mut self.controllers[self.cursor];
        if controller.brightness == controller.max_brightness {
            return;
        }

        controller.brightness += 1;
        if let Err(_) = update_controller(controller, controller.brightness) {
            return;
        }
    }

    pub fn handle_keys(&mut self, timeout: Duration) -> io::Result<bool> {
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(true),
                    KeyCode::Char('j') => self.down(),
                    KeyCode::Char('k') => self.up(),
                    KeyCode::Char('h') => self.left(),
                    KeyCode::Char('l') => self.right(),
                    _ => return Ok(false),
                };
            }
        }

        Ok(false)
    }
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| app.render(f))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if app.handle_keys(timeout)? {
            break;
        }

        if last_tick.elapsed() >= tick_rate {
            terminal.draw(|f| app.render(f))?;
            last_tick = Instant::now();
        }
    }

    Ok(())
}

pub fn run(controllers: Vec<Controller>) -> Result<(), Box<dyn std::error::Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // start app
    let app = App::new(controllers)?;
    let tick_rate = Duration::from_millis(250);
    let res = run_app(&mut terminal, app, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}
