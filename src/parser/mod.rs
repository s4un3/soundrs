use crate::audiowave::AudioWave;
use crate::definitions::Float;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Semitone {
    Semitone(Float),
    Rest,
}

pub fn note_to_semitone(note: &String, default_octave: Option<u8>) -> Option<Semitone> {
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

pub fn split_by_whitespace(text: &String) -> Vec<String> {
    let mut result = Vec::new();
    let mut current_word = String::new();

    for ch in text.chars() {
        if ch.is_whitespace() {
            if !current_word.is_empty() {
                result.push(current_word.clone());
                current_word.clear();
            }
        } else {
            current_word.push(ch);
        }
    }

    // Push the last word if there is one
    if !current_word.is_empty() {
        result.push(current_word);
    }

    result
}

pub fn str_is_whitespace_or_empty(s: &String) -> bool {
    if s.is_empty() {
        return true;
    }
    for ch in s.chars().into_iter() {
        if !ch.is_whitespace() {
            return false;
        }
    }
    true
}

pub enum VoiceContent {
    Raw(Vec<String>),
    Processed(Vec<(String, Float)>),
}

pub struct Voice {
    pub contents: VoiceContent,
    bpm: Float,
    tuning: Float,
    default_duration: Float,
    default_octave: Float,
    intensity: Float,
    pub waiting: String
}
impl Voice {
    pub fn get_time(mut self) {
        let content: Vec<String>;
        match self.contents {
            VoiceContent::Raw(r) => content = r,
            VoiceContent::Processed(_) => return,
        }
        let mut processed:Vec<(String, Float)> = Vec::new();
        for line in content{
            let words = split_by_whitespace(&line);
            let possibly_a_note = &words[1];
            let nullstr = &("".to_owned());
            let lastword = words.last().unwrap_or(nullstr);
            let mut beats: Float = 0.0;

            if line.starts_with("glissando") || line.starts_with("trill") || note_to_semitone(possibly_a_note, Some(4)).is_some(){
                let _lastword = lastword;
                match _lastword.parse::<Float>(){
                    Ok(v) => beats = v,
                    Err(_) => beats = self.default_duration,
                }
            }
            processed.push((line, beats));
        }
        self.contents = VoiceContent::Processed(processed);
    }
    pub fn get_audio(self){
        todo!()
    }
}

/// If syntax error is found, returns None
/// Each element in the vector corresponds to one voice and each voice is split by lines
pub fn preprocess(text: String) -> Option<Vec<Vec<String>>> {
    let voices = text.split('%');
    let mut chunks: Vec<Vec<String>> = Vec::new();
    let mut sections: HashMap<String, Vec<String>, _> = HashMap::new();
    let mut onsection = false;
    let mut current_section: String = "".to_owned();
    for voice in voices {
        let mut voicevec: Vec<String> = Vec::new();
        for mut line in voice.split(';') {
            if str_is_whitespace_or_empty(&line.to_string()) {
                continue;
            }
            line = line.trim_matches(char::is_whitespace);

            if line.starts_with("$") {
                continue;
            }
            if line.starts_with("section") {
                if onsection {
                    return None;
                }
                onsection = true;
                let aux = split_by_whitespace(&line.to_string());
                let section_name = aux[1].replace("", "");
                if sections.contains_key(&section_name) {
                    return None;
                }
                current_section = section_name.to_owned();
                sections.insert(section_name, Vec::new());
            } else if line.starts_with("end") {
                if !onsection {
                    return None;
                }
                onsection = false;
            } else if line.starts_with("jump") {
                let aux = split_by_whitespace(&line.to_string());
                let section_name = aux[1].replace("", "");
                match sections.get(&section_name) {
                    Some(v) => {
                        let mut repetitions: u32 = 1;
                        if aux.len() >= 3 {
                            let rep: Result<u32, _> = aux[2].replace("", "").parse();
                            match rep {
                                Ok(u) => repetitions = u,
                                Err(_) => return None,
                            }
                        }
                        for _ in 0..repetitions {
                            voicevec.extend(v.clone());
                        }
                    }
                    None => return None,
                }
            } else if onsection {
                if current_section.is_empty() {
                    return None;
                }
                if let Some(x) = sections.get_mut(&current_section) {
                    (*x).push(line.to_owned());
                }
            } else {
                voicevec.push(line.to_owned());
            }
        }
        chunks.push(voicevec);
    }
    Some(chunks)
}
