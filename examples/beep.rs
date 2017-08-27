extern crate beep;
extern crate dimensioned;

use std::{thread, time};
use beep::beep;
use dimensioned::si::Hertz;
use std::{thread, time};

fn main() {

  let C = Hertz::new(2093.0f64);
  let D = Hertz::new(2349.0f64);
  let E = Hertz::new(2673.0f64);
  let F = Hertz::new(2673.0f64);
  let G = Hertz::new(3136.0f64);

  let silence = Hertz::new(0.0f64);

  let note_length = time::Duration::from_millis(250);

  let song =
    [E, D, C, D, E, E, E, silence,
     D, D, D, silence,
     E, G, G, silence,
     E, D, C, D, E, E, E,
     E, D, D, E, D, C];

  for &note in song.iter() {
    beep(note);
    thread::sleep(note_length);
  }

  beep(silence);
}