use crate::audiowave::AudioWave;
use crate::definitions::Float;
use std::iter::Rev;

#[derive(Debug)]
pub enum Semitone {
    Semitone(Float),
    Rest,
}

pub fn note_to_semitone(note: &str, default_octave: Option<u8>) -> Option<Semitone> {
    if note == "_" {
        Some(Semitone::Rest);
    };

    let default_octave = default_octave.unwrap_or(4);
    let mut s: Float = 0.0;

    let mut note = note.chars();

    s += match note.next()? {
        'A' => Some(0.0),
        'B' => Some(2.0),
        'C' => Some(3.0 - 12.0),
        'D' => Some(5.0 - 12.0),
        'E' => Some(7.0 - 12.0),
        'F' => Some(8.0 - 12.0),
        'G' => Some(10.0 - 12.0),
        _ => return None,
    }?;

    let modifiers = note.as_str();
    if !modifiers.is_empty() {
        let mut i: usize = 0;
        let mut state: u8 = 0; // 0 - sharps and flats
        let mut tempstring = "".to_owned();
        let mut isoctaverelative = false;
        loop {
            let current_char: char;
            let extracting_char = modifiers.chars().nth(i);
            match extracting_char {
                Some(c) => current_char = c,
                None => break,
            }
            i += 1;
            if state == 0 {
                match current_char {
                    '#' => s += 1.0,
                    'b' => s -= 1.0,
                    ')' => return None,
                    '(' => state = 1, // 1 - microtones
                    '+' => {
                        isoctaverelative = true;
                        state = 2; // 2 - octave adjustments
                        s += 12.0;
                    }
                    '-' => {
                        isoctaverelative = true;
                        state = 2;
                        s -= 12.0;
                    }
                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                        if isoctaverelative {
                            return None;
                        }
                        let val: Result<u8, _> = current_char.to_string().parse();
                        match val {
                            Ok(v) => s += 12.0 * (v - default_octave) as Float,
                            Err(_) => (),
                        }
                        break;
                    }
                    _ => return None,
                };
            } else if state == 1 {
                match current_char {
                    'c' => {
                        match tempstring.parse::<Float>() {
                            Ok(value) => s += value / 100.0,
                            Err(_) => return None,
                        };
                        tempstring = "".to_string();
                    }
                    ')' => state = 2,
                    _ => tempstring.push(current_char),
                }
            } else if state == 2 {
                match current_char {
                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                        if isoctaverelative {
                            return None;
                        }
                        let val: Result<u8, _> = current_char.to_string().parse();
                        match val {
                            Ok(v) => s += 12.0 * (v - default_octave) as Float,
                            Err(_) => (),
                        }
                        break;
                    }
                    '+' => s += 12.0,
                    '-' => s -= 12.0,
                    _ => break,
                }
            }
        }
    }
    Some(Semitone::Semitone(s))
}