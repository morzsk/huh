use std::time::Duration;

use color_eyre::{Result, eyre::Context};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode},
    widgets::Paragraph,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app = run(terminal).context("app loop failed");
    ratatui::restore();
    app
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    while !should_quit()? {
        terminal.draw(draw)?;
    }
    Ok(())
}

fn draw(frame: &mut Frame) {
    let greeting = Paragraph::new("Hello World! (press 'q' to quit)");
    frame.render_widget(greeting, frame.area());
}

fn should_quit() -> Result<bool> {
    if event::poll(Duration::from_millis(250)).context("event poll failed")? {
        if let Event::Key(key) = event::read().context("event read failed")? {
            return Ok(KeyCode::Char('q') == key.code);
        }
    }
    Ok(false)
}
