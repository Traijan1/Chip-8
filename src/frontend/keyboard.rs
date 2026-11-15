use macroquad::input::{KeyCode, get_keys_down};

const KEY_MAP: [KeyCode; 16] = [
    KeyCode::X,    // 0x0
    KeyCode::Key1, // 0x1
    KeyCode::Key2, // 0x2
    KeyCode::Key3, // 0x3
    KeyCode::Q,    // 0x4
    KeyCode::W,    // 0x5
    KeyCode::E,    // 0x6
    KeyCode::A,    // 0x7
    KeyCode::S,    // 0x8
    KeyCode::D,    // 0x9
    KeyCode::Y,    // 0xA
    KeyCode::C,    // 0xB
    KeyCode::Key4, // 0xC
    KeyCode::R,    // 0xD
    KeyCode::F,    // 0xE
    KeyCode::V,    // 0xF
];


pub fn keys_down() -> [bool; 16] {
    let keys_down = get_keys_down();
    KEY_MAP.map(|key_to_check | keys_down.contains(&key_to_check))
}