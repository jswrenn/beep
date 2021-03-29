use std::{thread, time::Duration};
use beep::beep;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let c = 2093;
    let d = 2349;
    let e = 2673;
    let g = 3136;
    let rest = 0;

    let note_length = Duration::from_millis(250);

    let song =
        [e, d, c, d, e, e, e, rest,
         d, d, d, rest,
         e, g, g, rest,
         e, d, c, d, e, e, e,
         e, d, d, e, d, c];

    for &note in song.iter() {
        beep(note)?;
        thread::sleep(note_length);
    }

    beep(rest)?;

    Ok(())
}