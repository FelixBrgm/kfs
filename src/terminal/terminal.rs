use super::{ps2::Key, screen::Screen};

const NBR_OF_SCREENS_PER_TERMINAL: usize = 5;

pub struct Terminal {
    active_screen: usize,
    screens: [Screen; NBR_OF_SCREENS_PER_TERMINAL],
}

impl Terminal {
    pub fn default() -> Terminal {
        Terminal {
            active_screen: 0,
            screens: [Screen::default(); NBR_OF_SCREENS_PER_TERMINAL],
        }
    }

    pub fn handle_key(&mut self, key: Key) {
        match key {
            Key::Tab => {
                self.active_screen += 1;
                if self.active_screen >= NBR_OF_SCREENS_PER_TERMINAL {
                    self.active_screen = 0;
                }
            }
            _ => self.screens[self.active_screen].handle_key(key),
        }
    }

    pub fn write_str(&mut self, string: &str) {
        self.screens[self.active_screen].write_str(string);
    }

    pub fn write_color_str(&mut self, string: &str, color: u8) {
        self.screens[self.active_screen].write_color_str(string, color);
    }

    pub fn flush(&self) {
        self.screens[self.active_screen].flush();
    }
}
