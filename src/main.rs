use clap::Parser;
use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{size, Clear, ClearType},
};
use std::io::{stdout};

/// Push content offscreen by printing blank lines
#[derive(Parser, Debug)]
#[command(name = "ptop")]
#[command(about = "Push content offscreen.")]
struct Cli {}

fn main() -> std::io::Result<()> {
    let _args = Cli::parse();

    let (_, rows) = size()?; // Get terminal height

    for _ in 0..rows {
        println!();
    }

    execute!(
        stdout(),
        MoveTo(0, 0),
        Clear(ClearType::FromCursorDown)
    )?;

    Ok(())
}