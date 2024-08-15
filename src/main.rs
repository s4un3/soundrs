#![allow(dead_code)]
#![allow(unused_variables)]
mod audiowave;
mod definitions;
mod function;
mod parser;

use crate::parser::{Manager};

fn main() {
    let x =
    "duration 0.5;
bpm 150;

_ 2;

section bassline;
C;
Eb | G;
C;
F# | A;
C;
Eb | G;
C;
D | F#;
end;

debugtime starting it;
jump bassline;
debugtime first iteration of the bassline;
jump bassline 9; $ repeating it 9 times after the first iteration;
debugtime end;

% 
$ new voice for the melody;

duration 0.5;
bpm 150;
octave 6;

_ 10;

_; C-; G-; C; Eb 1; C 1; B-; C; D 1; C; G-; Eb-; C-; D-; Eb-; F#- 1; G-; Eb-; C-; A--; B--; C-; D- 1; C-;
debugtime end of first phrase;
_ 1.5;
_; C-; G-; C; Eb 1; glissando Ab A#; A#; A; G; A 1; A#; G; Eb; C; B-; C; D 1; Eb; C; G-; Eb-; B-; A-; trill B- C 6 p 1; C;".to_owned();


    let mut m = Manager::new();
    match m.run(x){
        Ok(v) => println!("{:?}", v.wave),
        Err(e) => println!("{}",e),
    }
}
