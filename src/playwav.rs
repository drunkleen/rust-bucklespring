use rodio::{Decoder, OutputStream, Sink};
use std::error::Error;
use std::thread;
use std::io::Cursor;

use crate::embed::Asset;

/// Spawns a new thread to play a sound associated with the given key.
///
/// The sound is played asynchronously, and any errors encountered during playback
/// are printed to the standard error output. The function returns immediately,
/// regardless of whether the sound was successfully played.
///
/// # Arguments
///
/// * `key` - A string slice that holds the key used to identify the sound file.
///
/// # Returns
///
/// * `Result<(), Box<dyn Error>>` - An `Ok(())` result is always returned.
pub fn play(key: &str) -> Result<(), Box<dyn Error>> {
    let key = key.to_string();
    thread::spawn(move || {
        if let Err(e) = play_sound(&key) {
            eprintln!("Error playing sound: {:?}", e);
        }
    });
    Ok(())
}

/// Play a sound from an embedded `.wav` file.
///
/// The sound is loaded at runtime from assets embedded at compile time using the `rust-embed` crate.
/// The filename is constructed by appending `.wav` to the given key (e.g. `01-0.wav`, `e01d-1.wav`, etc.).
///
/// If the specified embedded file does not exist, the function attempts to fall back to a default
/// key with the form `63-<suffix>`, where `<suffix>` is typically `"0"` (key press) or `"1"` (release).
///
/// If neither the original file nor the fallback is found, the function will return `Ok(())`
/// and not treat it as an error. The function always plays sounds synchronously and blocks
/// until playback ends.
fn play_sound(key: &str) -> Result<(), Box<dyn Error>> {
    let file_name = format!("{}.wav", key);

    if let Some(content) = Asset::get(&file_name) {
        let (_stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;

        let cursor = Cursor::new(content.data);
        let source = Decoder::new(cursor)?;

        sink.append(source);
        sink.sleep_until_end();
        return Ok(());
    } else {
        // Fallback logic
        if let Some(suffix) = key.split('-').nth(1) {
            let fallback_key = format!("63-{}", suffix);
            let fallback_file = format!("{}.wav", fallback_key);

            if let Some(fallback) = Asset::get(&fallback_file) {
                let (_stream, stream_handle) = OutputStream::try_default()?;
                let sink = Sink::try_new(&stream_handle)?;

                let cursor = Cursor::new(fallback.data);
                let source = Decoder::new(cursor)?;

                sink.append(source);
                sink.sleep_until_end();
            }
        }
        return Ok(());
    }
}