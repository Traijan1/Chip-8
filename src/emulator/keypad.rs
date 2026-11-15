pub struct Keypad {
    pressed: [bool; 16],
    previous: [bool; 16]
}

impl Keypad {
    pub(super) fn new() -> Self {
        Self {
            pressed: [false; 16],
            previous: [false; 16]
        }
    }

    pub fn set_keys(&mut self, keys: [bool; 16]) {
        self.previous = self.pressed; 
        self.pressed = keys;
    }

    pub fn get_key(&self, key: u8) -> bool {
        // TODO: Can throw an error because key can be higher than 16
        self.pressed[key as usize]
    }

    pub(super) fn was_key_pressed(&self) -> Option<u8> {
        self.previous
            .iter()
            .zip(self.pressed.iter())
            .position(|(&was_down, &is_down)| !was_down && is_down)
            .map(|index| index as u8)
    } 
}