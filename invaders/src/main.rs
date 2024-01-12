#![allow(dead_code, unused_imports, unused_variables)]
use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{event, terminal, ExecutableCommand};
use invaders::frame::{new_frame, Drawable};
use invaders::invaders::Invaders;
use invaders::player::Player;
use invaders::{frame, render};
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

    // Render loop in a separate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
     
    });


    // GameLoop
    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();
    'gameloop: loop {
        // Per-frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = new_frame();
        
        // Input events
        while event::poll(Duration::default())?{
            if let Event::Key(key_event) = event:: read()? {
                match key_event.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') | KeyCode :: Enter => {
                        if player.shoot() {
                            audio.play("pew");
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('q') => { 
                        audio.play("loose");
                        break 'gameloop;
                    },   
                    _ =>{}
                }
            }
        }


        // Updates
        player.update(delta);
        if invaders.update(delta){
            audio.play("move");
        }

        if player.detect_hits(&mut invaders){
            audio.play("explode");
        }

        // Draw & render
        // player.draw(&mut curr_frame);
        // invaders.draw(&mut curr_frame);
        let drawables : Vec<&dyn Drawable> = vec![&player, &invaders];
        for drawable in drawables {
            drawable.draw(&mut curr_frame);
        }
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1)); 

        // Win or lose?
        if invaders.all_killed(){
            audio.play("win");
            break 'gameloop;
        }
        if invaders.reached_bottom(){
            audio.play("lose");
            break 'gameloop;
        }


    }

    // Clean up
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
