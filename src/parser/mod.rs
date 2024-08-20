use crate::audiowave::AudioWave;
use crate::definitions::Float;
use crate::function::Function;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Semitone {
    Semitone(Float),
    Rest,
}

pub fn note_to_semitone(note: &String, default_octave: &Option<u8>) -> Option<Semitone> {
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

pub fn get_freq_value(string: &String, octave: &u8, tuning: &Float) -> Result<Float, String> {
    if string.ends_with("Hz") {
        match string.replace("Hz", "").parse::<Float>() {
            Ok(v) => return Ok(v),
            Err(e) => return Err(format!("Error: '{}' is not a valid number", string)),
        };
    } else {
        match note_to_semitone(string, &Some(*octave)) {
            Some(v) => match v {
                Semitone::Semitone(x) => Ok(tuning * (2.0 as Float).powf(x / (12 as Float))),
                Semitone::Rest => Ok(0.0),
            },
            None => return Err(format!("Could not understand '{}' as a note name", string)),
        }
    }
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

#[derive(Clone)]
pub enum VoiceContent {
    Raw(Vec<String>),
    Processed(Vec<(String, Float)>),
}

#[derive(Clone)]
pub struct Voice {
    pub contents: VoiceContent,
    bpm: Float,
    tuning: Float,
    default_duration: Float,
    default_octave: u8,
    intensity: Float,
    pub waiting: Option<String>,
}
impl Voice {
    pub fn new() -> Self {
        Voice {
            bpm: 120.0,
            tuning: 440.0,
            default_duration: 1.0,
            default_octave: 4,
            intensity: 1.0,
            waiting: None,
            contents: VoiceContent::Raw(vec!["".to_owned()]),
        }
    }
    pub fn get_time(&mut self) {
        let content: Vec<String>;
        match &self.contents {
            VoiceContent::Raw(r) => content = r.to_vec(),
            VoiceContent::Processed(_) => return,
        }
        let mut processed: Vec<(String, Float)> = Vec::new();
        for line in content {
            let words = split_by_whitespace(&line);
            let possibly_a_note = &words[1];
            let nullstr = &("".to_owned());
            let lastword = words.last().unwrap_or(nullstr);
            let mut seconds: Float = 0.0;

            if line.starts_with("glissando")
                || line.starts_with("trill")
                || note_to_semitone(possibly_a_note, &Some(4)).is_some()
            {
                let _lastword = lastword;
                match _lastword.parse::<Float>() {
                    Ok(v) => seconds = v * 60.0 / self.bpm,
                    Err(_) => seconds = self.default_duration * 60.0 / self.bpm,
                }
            }
            processed.push((line, seconds));
        }
        self.contents = VoiceContent::Processed(processed);
    }
    pub fn get_audio(&mut self) -> Result<Option<(Option<AudioWave>, Option<String>)>, String> {
        if self.waiting.is_some() {
            return Ok(None);
        }
        let mut audio: AudioWave = AudioWave::new(
            &Function::Const(0.0),
            &Function::Const(0.0),
            &0.0,
            None,
            None,
            None,
            None,
        )
        .expect("Should be able to create empty wave");
        match &mut self.contents {
            VoiceContent::Raw(_) => return Ok(None),
            VoiceContent::Processed(ref mut p) => {
                if p.is_empty() {
                    return Err("Empty".to_owned());
                }
                let clonep = p.clone();
                for i in 0..p.len() {
                    let line = clonep[i].clone();
                    p.remove(0);
                    let words = split_by_whitespace(&line.0);
                    if words[0] == "bpm" {
                        match words[1].parse::<Float>() {
                            Ok(v) => self.bpm = v,
                            Err(e) => {
                                return Err(format!("Invalid syntax at line: {}\n{}", line.0, e))
                            }
                        };
                    } else if words[0] == "tuning" {
                        match words[1].parse::<Float>() {
                            Ok(v) => self.tuning = v,
                            Err(e) => {
                                return Err(format!("Invalid syntax at line: {}\n{}", line.0, e))
                            }
                        };
                    } else if words[0] == "duration" {
                        match words[1].parse::<Float>() {
                            Ok(v) => self.default_duration = v,
                            Err(e) => {
                                return Err(format!("Invalid syntax at line: {}\n{}", line.0, e))
                            }
                        };
                    } else if words[0] == "octave" {
                        match words[1].parse::<u8>() {
                            Ok(v) => self.default_octave = v,
                            Err(e) => {
                                return Err(format!("Invalid syntax at line: {}\n{}", line.0, e))
                            }
                        };
                    } else if words[0] == "intensity" {
                        match words[1].parse::<Float>() {
                            Ok(v) => self.intensity = v,
                            Err(e) => {
                                return Err(format!("Invalid syntax at line: {}\n{}", line.0, e))
                            }
                        };
                    } else if words[0] == "wait" {
                        self.waiting = Some(words[1].clone());
                        return Ok(Some((Some(audio), None)));
                    } else if words[0] == "sync" {
                        return Ok(Some((Some(audio), Some(words[1].clone()))));
                    } else if words[0] == "glissando" {
                        let line_replicate = line.clone();
                        let line_replicate2 = line.clone();
                        let line_replicate3 = line;
                        let mut first_note: Float = 0.0;
                        let mut last_note: Float = 0.0;
                        match get_freq_value(&words[1], &self.default_octave, &self.tuning) {
                            Ok(v) => {
                                if v == 0.0 {
                                    return Err(
                                        "Error: rests cannot be part of a glissando".to_owned()
                                    );
                                }
                                first_note = v;
                            }
                            Err(e) => return Err(e),
                        }
                        match get_freq_value(&words[2], &self.default_octave, &self.tuning) {
                            Ok(v) => {
                                if v == 0.0 {
                                    return Err(
                                        "Error: rests cannot be part of a glissando".to_owned()
                                    );
                                }
                                last_note = v;
                            }
                            Err(e) => return Err(e),
                        }
                        let f = {
                            let _first_note = first_note;
                            let _last_note = last_note;
                            let _line = line_replicate;
                            move |t: Float| -> Float {
                                _first_note * ((_last_note / _first_note).powf(t / _line.1))
                            }
                        };
                        match AudioWave::new(
                            &Function::Function(Box::new(f)),
                            &Function::Const(self.intensity),
                            &line_replicate2.1,
                            None,
                            None,
                            None,
                            None,
                        ) {
                            Some(v) => {
                                audio.clone().append(v, Some(1.0));
                                return Ok(Some((Some(audio), None)));
                            }
                            None => {
                                return Err(format!(
                                    "Error: failed to generate wave corresponding to line: {}",
                                    line_replicate3.0
                                ))
                            }
                        };
                    } else if words[0] == "trill" {
                        let mut first_note: Float = 0.0;
                        let mut second_note: Float = 0.0;
                        match get_freq_value(&words[1], &self.default_octave, &self.tuning) {
                            Ok(v) => {
                                first_note = v;
                            }
                            Err(e) => return Err(e),
                        }
                        match get_freq_value(&words[2], &self.default_octave, &self.tuning) {
                            Ok(v) => {
                                second_note = v;
                            }
                            Err(e) => return Err(e),
                        }
                        let mut parts: u32 = 1;
                        match &words[3].parse::<u32>() {
                            Ok(v) => {
                                parts = *v;
                            }
                            Err(e) => {
                                return Err(format!("Invalid syntax at line: {}\n{}", line.0, e))
                            }
                        };
                        let first_audio = AudioWave::new(
                            &Function::Const(first_note),
                            &Function::Const(self.intensity),
                            &(line.1 / (parts as Float)),
                            None,
                            None,
                            None,
                            None,
                        );
                        let second_audio = AudioWave::new(
                            &Function::Const(second_note),
                            &Function::Const(self.intensity),
                            &(line.1 / (parts as Float)),
                            None,
                            None,
                            None,
                            None,
                        );
                        let first_audio_as_some: AudioWave;
                        let second_audio_as_some: AudioWave;
                        match first_audio {
                            Some(v) => {
                                first_audio_as_some = v;
                            }
                            None => {
                                return Err(format!(
                                    "Error: failed to generate wave corresponding to line: {}",
                                    line.0
                                ))
                            }
                        }
                        match second_audio {
                            Some(v) => {
                                second_audio_as_some = v;
                            }
                            None => {
                                return Err(format!(
                                    "Error: failed to generate wave corresponding to line: {}",
                                    line.0
                                ))
                            }
                        }

                        let mut is_first = true;
                        for _ in 0..parts {
                            if is_first {
                                audio = audio.clone().append(first_audio_as_some.clone(), Some(1.0)).expect("Waves generated by this module should always be compatible");
                            } else {
                                audio = audio.clone().append(second_audio_as_some.clone(), Some(1.0)).expect("Waves generated by this module should always be compatible");
                            }
                            is_first = !is_first;
                        }
                        return Ok(Some((Some(audio), None)));
                    } else {
                        let mut line_audio = AudioWave::new(
                            &Function::Const(0.0),
                            &Function::Const(0.0),
                            &0.0,
                            None,
                            None,
                            None,
                            None,
                        )
                        .expect("Should be able to create empty wave");
                        for i in 0..line.0.len() {
                            if words[i] == "|".to_owned() {
                                continue;
                            };
                            match words[i].parse::<u32>() {
                                Ok(_) => continue,
                                Err(_) => (),
                            }
                            match get_freq_value(&words[i], &self.default_octave, &self.tuning) {
                                Ok(v) => {
                                    match AudioWave::new(&Function::Const(v), &Function::Const(self.intensity), &line.1, None, None, None, None){
                                        Some(u) => line_audio = line_audio.clone().add(u).expect("Waves generated by this module should always be compatible"),
                                        None => return Err(format!("Error: failed to generate wave corresponding to line: {}",line.0)),
                                    };
                                }
                                Err(e) => {
                                    return Err(format!(
                                        "Error: failed to generate wave corresponding to line: {}",
                                        line.0
                                    ))
                                }
                            };
                        }
                        audio = audio
                            .clone()
                            .append(line_audio, Some(1.0))
                            .expect("Waves generated by this module should always be compatible");
                        return Ok(Some((Some(audio), None)));
                    }
                }
            }
        }
        return Err("Unable to understand text".to_owned());
    }
}

/// If syntax error is found, returns None
/// Each element in the vector corresponds to one voice and each voice is split by lines
pub fn preprocess(text: String) -> Result<Vec<Vec<String>>, String> {
    // there is a bug here, still need to find out what it's causing it
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
                    return Err(format!(
                        "Invalid syntax on line: {}\nAlready on a section",
                        line
                    ));
                }
                onsection = true;
                let aux = split_by_whitespace(&line.to_string());
                let section_name = aux[1].replace("", "");
                if sections.contains_key(&section_name) {
                    return Err(format!("Invalid syntax on line: {}\nSection '{}' already defined (or being defined)", line, section_name));
                }
                current_section = section_name.to_owned();
                sections.insert(section_name, Vec::new());
            } else if line.starts_with("end") {
                if !onsection {
                    return Err(format!(
                        "Invalid syntax on line: {}\nNo section to end",
                        line
                    ));
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
                                Err(e) => {
                                    return Err(format!("Invalid syntax on line: {}\n{}", line, e))
                                }
                            }
                        }
                        for _ in 0..repetitions {
                            voicevec.extend(v.clone());
                        }
                    }
                    None => {
                        return Err(format!(
                            "Error on line: {}\nNo section named '{}'",
                            line, section_name
                        ))
                    }
                }
            } else if onsection {
                if current_section.is_empty() {
                    return Err(format!(
                        "Error on line: {}\nCannot insert data on unnamed section",
                        line
                    ));
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
    Ok(chunks)
}

pub struct Manager {
    voices: Vec<(Voice, bool)>,
    voice_audios: Vec<AudioWave>,
    time_elapsed: Vec<Float>,
    point_flag: (String, Float),
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            voices: vec![],
            voice_audios: vec![],
            time_elapsed: vec![],
            point_flag: ("".to_owned(), 0.0),
        }
    }
    pub fn run(&mut self, text: String) -> Result<AudioWave, String> {
        let mut vec: Vec<Vec<String>>;
        match preprocess(text) {
            Ok(v) => vec = v,
            Err(e) => return Err(e),
        };
        for item in vec {
            let mut voice = Voice::new();
            voice.contents = VoiceContent::Raw(item);
            voice.get_time();
            self.voices.push((voice, false));
            self.time_elapsed.push(0.0);
            self.voice_audios.push(
                AudioWave::new(
                    &Function::Const(0.0),
                    &Function::Const(0.0),
                    &0.0,
                    None,
                    None,
                    None,
                    None,
                )
                .expect("Should be able to create empty wave"),
            );
        }
        loop {
            let mut late_one = Float::INFINITY;
            let mut late_one_index: usize = 0;
            let mut completed = true;
            for i in 0..self.voices.len() {
                completed = completed && self.voices[i].1;
                match &self.voices[i].0.waiting {
                    None => {
                        if self.voices[i].1 {
                            continue;
                        }
                        if late_one > self.time_elapsed[i] {
                            late_one = self.time_elapsed[i];
                            late_one_index = i;
                        }
                    }
                    Some(s) => {
                        if *s == self.point_flag.0 {
                            self.voices[i].0.waiting = None;
                            self.voice_audios[i] = self.voice_audios[i]
                                .clone()
                                .append(
                                    AudioWave::new(
                                        &Function::Const(0.0),
                                        &Function::Const(0.0),
                                        &(self.point_flag.1),
                                        None,
                                        None,
                                        None,
                                        None,
                                    )
                                    .expect("Sould be able to create rest"),
                                    Some(1.0),
                                )
                                .expect(
                                    "Waves generated by this module should always be compatible",
                                );
                            self.time_elapsed[i] += self.point_flag.1;
                        }
                    }
                }
            }
            let mut t: Float = 0.0;
            match &self.voices[late_one_index].0.get_audio() {
                Err(e) => {
                    if e == "Empty" {
                        self.voices[late_one_index].1 = true;
                        continue;
                    }
                    return Err(e.clone());
                }
                Ok(v) => match v {
                    Some(u) => {
                        match u.0.clone() {
                            Some(w) => {
                                self.voice_audios[late_one_index] = self.voice_audios[late_one_index].clone().append(w.clone(), Some(1.0)).expect("Waves generated by this module should always be compatible");
                                t = w.get_duration();
                            }
                            None => (),
                        };
                        match u.1.clone() {
                            Some(s) => self.point_flag = (s, t),
                            None => (),
                        };
                    }
                    None => continue,
                },
            };
            if completed {
                break;
            }
        }

        let mut result: AudioWave = AudioWave::new(
            &Function::Const(0.0),
            &Function::Const(0.0),
            &0.0,
            None,
            None,
            None,
            None,
        )
        .expect("Should be able to create empty wave");
        for audio in self.voice_audios.clone() {
            result = result
                .clone()
                .add(audio)
                .expect("Waves generated by this module should always be compatible");
        }
        return Ok(result);
    }
}
