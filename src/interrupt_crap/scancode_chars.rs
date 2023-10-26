use crate::print;

pub fn get_char(scancode: u8) -> Option<char> {
    let output_unknown = false;
    match scancode {
        0x02 => Some('1'),
        0x03 => Some('2'),
        0x04 => Some('3'),
        0x05 => Some('4'),
        0x06 => Some('5'),
        0x07 => Some('6'),
        0x08 => Some('7'),
        0x09 => Some('8'),
        0x0a => Some('9'),
        0x0b => Some('0'),
        0x0c => Some('-'),
        0x0e => None,
        0x0d => Some('='),
        0x0f => Some('\t'),
        0x10 => Some('q'),
        0x11 => Some('w'),
        0x12 => Some('e'),
        0x13 => Some('r'),
        0x14 => Some('t'),
        0x15 => Some('y'),
        0x16 => Some('u'),
        0x17 => Some('i'),
        0x18 => Some('o'),
        0x19 => Some('p'),
        0x1a => Some('['),
        0x1b => Some(']'),
        0x1c => Some('\n'),
        0x1d => None,
        0x1e => Some('a'),
        0x1f => Some('s'),
        0x20 => Some('d'),
        0x21 => Some('f'),
        0x22 => Some('g'),
        0x23 => Some('h'),
        0x24 => Some('j'),
        0x25 => Some('k'),
        0x26 => Some('l'),
        0x27 => Some(';'),
        0x28 => Some('\''),
        0x29 => Some('`'),
        0x2a => None,
        0x2b => Some('\\'),
        0x2c => Some('z'),
        0x2d => Some('x'),
        0x2e => Some('c'),
        0x2f => Some('v'),
        0x30 => Some('b'),
        0x31 => Some('n'),
        0x32 => Some('m'),
        0x33 => Some(','),
        0x34 => Some('.'),
        0x35 => Some('/'),
        0x39 => Some(' '),
        _ => {
            if output_unknown {
                print!("{}", scancode);
            }
            None
        }
    }
}
