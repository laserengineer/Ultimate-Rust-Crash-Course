// Importing necessary modules and traits
use std::error::Error;
use rusty_audio::Audio;

// The main function with a return type that can accommodate any error that implements the `Error` trait.
fn main() -> Result<(), Box<dyn Error>> {
    // Create a new Audio object. This object will be used for handling audio playback.
    let mut audio = Audio::new();

    // Adding various sound effects to the audio handler. Each sound file is associated with a key.
    // The audio files are expected to be in the 'sounds' directory relative to the current working directory.
    audio.add("explode", "sounds/explode.wav"); // Adding an 'explode' sound effect.
    audio.add("lose", "sounds/lose.wav");       // Adding a 'lose' sound effect.
    audio.add("move", "sounds/move.wav");       // Adding a 'move' sound effect.
    audio.add("pew", "sounds/pew.wav");         // Adding a 'pew' sound effect.
    audio.add("startup", "sounds/startup.wav"); // Adding a 'startup' sound effect.
    audio.add("win", "sounds/win.wav");         // Adding a 'win' sound effect.

    // Playing the 'startup' sound effect.
    // This will start playing the sound file associated with the key "startup".
    audio.play("startup");

    // Block the current thread until all sounds that are currently playing have finished.
    // This is important to ensure that the program doesn't exit prematurely,
    // cutting off sound playback.
    audio.wait();

    // If everything executed successfully, return Ok.
    // The '()' signifies that the function is not returning any meaningful value.
    Ok(())
}
