// Importing necessary modules and traits
#![allow(dead_code, unused_imports, unused_variables)]
use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{event, terminal, ExecutableCommand};
// use invaders::frame::{new_frame, Drawable};
// use invaders::invaders::Invaders;
// use invaders::player::Player;
// use invaders::{frame, render};
use rusty_audio::Audio;
use std::error::Error;
use std::sync::mpsc;
use std::time::{Duration, Instant};
use std::{io, thread};


// The main function with a return type that can accommodate any error that implements the `Error` trait.
fn main() -> Result<(), Box<dyn Error>> {
    // Create a new Audio object. This object will be used for handling audio playback.
    let mut audio = Audio::new();

    // Adding various sound effects to the audio handler. Each sound file is associated with a key.
    // The audio files are expected to be in the 'sounds' directory relative to the current working directory.
    audio.add("explode", "assets/explode.wav"); // Adding an 'explode' sound effect.
    audio.add("lose", "assets/lose.wav");       // Adding a 'lose' sound effect.
    audio.add("move", "assets/move.wav");       // Adding a 'move' sound effect.
    audio.add("pew", "assets/pew.wav");         // Adding a 'pew' sound effect.
    audio.add("startup", "assets/startup.wav"); // Adding a 'startup' sound effect.
    audio.add("win", "assets/win.wav");         // Adding a 'win' sound effect.
    audio.play("startup");

    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // GameLoop
    'gameloop: loop {
        // Input events
        while event::poll(Duration::default())?{
            if let Event::Key(key_event) = event:: read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') =>{
                        audio.play("loose");
                        break 'gameloop;
                    }
                    _ =>{}
                }
            }
        }
    }

    // Clean up
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
