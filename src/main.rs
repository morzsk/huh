use std::time::Duration;

use color_eyre::{Result, eyre::Context};
use ratatui::{
    DefaultTerminal, Frame, TerminalOptions, Viewport,
    crossterm::event::{self, Event, KeyCode},
    widgets::{Block, Borders},
};
use std::process::Command;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init_with_options(TerminalOptions {
        viewport: Viewport::Inline(8),
    });
    let app = run(terminal).context("app loop failed");
    ratatui::restore();
    app
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let command = "cargo updat";
    let output = Command::new("sh").arg("-c").arg(command).output()?;

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        println!("Huh?: {}", err);
    }

    while !should_quit()? {
        terminal.draw(draw)?;
    }
    Ok(())
}

fn draw(frame: &mut Frame) {
    let area = frame.area();
    // let greeting = Paragraph::new("Hello World! (press 'q' to quit)");
    let block = Block::default().title(" huh? ").borders(Borders::ALL);
    frame.render_widget(block, area);
}

fn should_quit() -> Result<bool> {
    if event::poll(Duration::from_millis(250)).context("event poll failed")? {
        if let Event::Key(key) = event::read().context("event read failed")? {
            return Ok(KeyCode::Char('q') == key.code);
        }
    }
    Ok(false)
}
