use rodio::Decoder;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::thread;

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

/// Play a sound from a wav file
//
/// The filename is constructed by appending the key to `wav/` and adding a `.wav` extension.
/// If the constructed file does not exist, it will fall back to a certain key if the key has the
/// form `<key>-<num>`.
///
/// The fallback key is `63-<num>`.
///
/// If the fallback file does not exist either, the function will return `Ok(())` and not treat it
/// as an error.
///
/// The function will spawn a new thread and return immediately.
fn play_sound(key: &str) -> Result<(), Box<dyn Error>> {
    let file_path = format!("wav/{}.wav", key);

    if !std::path::Path::new(&file_path).exists() {
        let fallback = "63";
        let fallback_key = format!("{}-{}", fallback, key.split('-').nth(1).unwrap_or("0"));
        let fallback_path = format!("wav/{}.wav", fallback_key);

        if std::path::Path::new(&fallback_path).exists() {
            return play_sound(&fallback_key);
        }
        return Ok(()); // not treating as an error
    }

    let (_stream, stream_handle) = rodio::OutputStream::try_default()?;
    let sink = rodio::Sink::try_new(&stream_handle)?;

    let file = File::open(&file_path)?;
    let source = Decoder::new(BufReader::new(file))?;

    sink.append(source);
    sink.sleep_until_end();

    Ok(())
}
