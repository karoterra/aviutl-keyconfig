#[derive(Debug)]
pub struct Key {
    code: u32,
}

impl Key {
    pub const BACK: u32 = 0x08;
    pub const TAB: u32 = 0x09;
    pub const RETURN: u32 = 0x0D;
    pub const SPACE: u32 = 0x20;
    pub const PAGE_UP: u32 = 0x21;
    pub const PAGE_DOWN: u32 = 0x22;
    pub const END: u32 = 0x23;
    pub const HOME: u32 = 0x24;
    pub const INSERT: u32 = 0x2D;
    pub const DELETE: u32 = 0x2E;
    pub const UP: u32 = 0x26;
    pub const DOWN: u32 = 0x28;
    pub const LEFT: u32 = 0x25;
    pub const RIGHT: u32 = 0x27;
    pub const MULTIPLY: u32 = 0x6A;
    pub const ADD: u32 = 0x6B;
    pub const SUBTRACT: u32 = 0x6D;
    pub const DECIMAL: u32 = 0x6E;
    pub const DIVIDE: u32 = 0x6F;
    pub const HYPHEN: u32 = 0xBD;
    pub const PERIOD: u32 = 0xBE;
    pub const COMMA: u32 = 0xBC;
    pub const AT: u32 = 0xC0;
    pub const CARET: u32 = 0xDE;
    pub const COLON: u32 = 0xBA;
    pub const SEMICOLON: u32 = 0xBB;
    pub const BRACKET_LEFT: u32 = 0xDB;
    pub const BRACKET_RIGHT: u32 = 0xDD;
    pub const YEN: u32 = 0xDC;
    pub const SLASH: u32 = 0xBF;
    pub const BACKSLASH: u32 = 0xE2;
    pub const ZERO: u32 = 0x30;
    pub const NINE: u32 = 0x39;
    pub const A: u32 = 0x41;
    pub const Z: u32 = 0x5A;
    pub const F1: u32 = 0x70;
    pub const F12: u32 = 0x7B;
    pub const NUMPAD0: u32 = 0x60;
    pub const NUMPAD9: u32 = 0x69;

    pub fn new(code: u8) -> Key {
        Key { code: code as u32 }
    }

    pub fn code(&self) -> u32 {
        self.code
    }

    pub fn name(&self) -> String {
        match self.code() {
            Key::BACK => String::from("BackSpace"),
            Key::TAB => String::from("Tab"),
            Key::RETURN => String::from("Enter"),
            Key::SPACE => String::from("Space"),
            Key::PAGE_UP => String::from("PageUp"),
            Key::PAGE_DOWN => String::from("PageDown"),
            Key::END => String::from("End"),
            Key::HOME => String::from("Home"),
            Key::INSERT => String::from("Insert"),
            Key::DELETE => String::from("Delete"),
            Key::UP => String::from("↑"),
            Key::DOWN => String::from("↓"),
            Key::LEFT => String::from("←"),
            Key::RIGHT => String::from("→"),
            Key::MULTIPLY => String::from("Numpad *"),
            Key::ADD => String::from("Numpad +"),
            Key::SUBTRACT => String::from("Numpad -"),
            Key::DECIMAL => String::from("Numpad ."),
            Key::DIVIDE => String::from("Numpad /"),
            Key::HYPHEN => String::from("-"),
            Key::PERIOD => String::from("."),
            Key::COMMA => String::from(","),
            Key::AT => String::from("@"),
            Key::CARET => String::from("^"),
            Key::COLON => String::from(":"),
            Key::SEMICOLON => String::from(";"),
            Key::BRACKET_LEFT => String::from("["),
            Key::BRACKET_RIGHT => String::from("]"),
            Key::YEN => String::from("￥"),
            Key::SLASH => String::from("/"),
            Key::BACKSLASH => String::from("＼"),
            Key::ZERO..=Key::NINE => ((self.code() as u8) as char).to_string(),
            Key::A..=Key::Z => ((self.code() as u8) as char).to_string(),
            Key::F1..=Key::F12 => format!("F{}", self.code() - Key::F1 + 1),
            Key::NUMPAD0..=Key::NUMPAD9 => format!("Numpad {}", self.code() - Key::NUMPAD0),
            _ => format!("Key code {}", self.code()),
        }
    }
}
