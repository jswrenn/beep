# beep.rs

Play an indefinite tone of a given `frequency`.

For instance:

```rust
extern crate beep;
extern crate dimensioned;

use std::{thread, time};
use beep::beep;
use dimensioned::si;

fn main() {
  beep(440. * si::HZ);
  thread::sleep(time::Duration::from_millis(500));
  beep(si::Hertz::new(880));
  thread::sleep(time::Duration::from_millis(500));
  beep(0. * si::HZ);
}
```