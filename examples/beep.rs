extern crate beep;
extern crate dimensioned;

use std::{thread, time};
use beep::beep;
use dimensioned::si::Hertz;

fn main() {

  let C = Hertz::new(2093.0f64);
  let D = Hertz::new(2349.0f64);
  let E = Hertz::new(2673.0f64);
  let F = Hertz::new(2673.0f64);
  let G = Hertz::new(3136.0f64);
  let rest = Hertz::new(0.0f64);

  let note_length = time::Duration::from_millis(250);

  let song =
    [E, D, C, D, E, E, E, rest,
     D, D, D, rest,
     E, G, G, rest,
     E, D, C, D, E, E, E,
     E, D, D, E, D, C];

  for &note in song.iter() {
    beep(note);
    thread::sleep(note_length);
  }

  beep(rest);
}