use once_cell::sync::Lazy;
use rdev::{Event, EventType, Key};
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

use crate::playwav;

static KEYCODE_MAP: Lazy<HashMap<Key, &'static str>> = Lazy::new(|| {
    HashMap::from([
        (Key::Escape, "01"),
        (Key::Num1, "02"),
        (Key::Num2, "03"),
        (Key::Num3, "04"),
        (Key::Num4, "05"),
        (Key::Num5, "06"),
        (Key::Num6, "07"),
        (Key::Num7, "08"),
        (Key::Num8, "09"),
        (Key::Num9, "0a"),
        (Key::Num0, "0b"),
        (Key::Minus, "0c"),
        (Key::Equal, "0d"),
        (Key::Backspace, "0e"),
        (Key::Tab, "0f"),
        (Key::KeyQ, "10"),
        (Key::KeyW, "11"),
        (Key::KeyE, "12"),
        (Key::KeyR, "13"),
        (Key::KeyT, "14"),
        (Key::KeyY, "15"),
        (Key::KeyU, "16"),
        (Key::KeyI, "17"),
        (Key::KeyO, "18"),
        (Key::KeyP, "19"),
        (Key::LeftBracket, "1a"),
        (Key::RightBracket, "1b"),
        (Key::Return, "1c"),
        (Key::ControlLeft, "1d"),
        (Key::KeyA, "1e"),
        (Key::KeyS, "1f"),
        (Key::KeyD, "20"),
        (Key::KeyF, "21"),
        (Key::KeyG, "22"),
        (Key::KeyH, "23"),
        (Key::KeyJ, "24"),
        (Key::KeyK, "25"),
        (Key::KeyL, "26"),
        (Key::SemiColon, "27"),
        (Key::Quote, "28"),
        (Key::BackQuote, "29"),
        (Key::ShiftLeft, "2a"),
        (Key::BackSlash, "2b"),
        (Key::KeyZ, "2c"),
        (Key::KeyX, "2d"),
        (Key::KeyC, "2e"),
        (Key::KeyV, "2f"),
        (Key::KeyB, "30"),
        (Key::KeyN, "31"),
        (Key::KeyM, "32"),
        (Key::Comma, "33"),
        (Key::Dot, "34"),
        (Key::Slash, "35"),
        (Key::ShiftRight, "36"),
        (Key::Alt, "38"),
        (Key::Space, "39"),
        (Key::CapsLock, "3a"),
        (Key::F1, "3b"),
        (Key::F2, "3c"),
        (Key::F3, "3d"),
        (Key::F4, "3e"),
        (Key::F5, "3f"),
        (Key::F6, "40"),
        (Key::F7, "41"),
        (Key::F8, "42"),
        (Key::F9, "43"),
        (Key::F10, "44"),
        (Key::NumLock, "45"),
        (Key::ScrollLock, "46"),
        (Key::Home, "47"),
        (Key::UpArrow, "48"),
        (Key::PageUp, "49"),
        (Key::LeftArrow, "4b"),
        (Key::RightArrow, "4d"),
        (Key::End, "4f"),
        (Key::DownArrow, "50"),
        (Key::PageDown, "51"),
        (Key::Insert, "52"),
        (Key::Delete, "53"),
        (Key::F11, "57"),
        (Key::F12, "58"),
        (Key::ControlRight, "1d"),
        (Key::AltGr, "38"),
        (Key::MetaLeft, "5b"),
        (Key::MetaRight, "5b"),
        (Key::Function, "ff"),
        // Numpad digits
        (Key::Kp0, "52"),
        (Key::Kp1, "4f"),
        (Key::Kp2, "50"),
        (Key::Kp3, "51"),
        (Key::Kp4, "4b"),
        (Key::Kp5, "4c"),
        (Key::Kp6, "4d"),
        (Key::Kp7, "47"),
        (Key::Kp8, "48"),
        (Key::Kp9, "49"),
        // Numpad symbols
        (Key::KpDelete, "53"),
        (Key::KpPlus, "4e"),
        (Key::KpMinus, "4a"),
        (Key::KpMultiply, "37"),
        (Key::KpDivide, "35"),
    ])
});

// Track key press state
static PRESSED_KEYS: Lazy<Mutex<HashSet<Key>>> = Lazy::new(|| Mutex::new(HashSet::new()));

/// Play key press and release sounds based on the given event.
///
/// If the given event is a key press, and the key is not already in the
/// pressed keys set, it plays a key press sound and adds the key to the set.
///
/// If the given event is a key release, and the key is in the pressed keys set,
/// it plays a key release sound and removes the key from the set.
///
/// It does nothing if the event is not a key press or release.
pub fn callback(event: Event) {
    let mut pressed = PRESSED_KEYS.lock().unwrap();

    match event.event_type {
        EventType::KeyPress(key) => {
            if pressed.insert(key) {
                if let Some(&scancode) = KEYCODE_MAP.get(&key) {
                    let filename = format!("{}-0", scancode);
                    let _ = playwav::play(&filename);
                }
            }
        }
        EventType::KeyRelease(key) => {
            if pressed.remove(&key) {
                if let Some(&scancode) = KEYCODE_MAP.get(&key) {
                    let filename = format!("{}-1", scancode);
                    let _ = playwav::play(&filename);
                }
            }
        }
        _ => {}
    }
}
