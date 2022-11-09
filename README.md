# beep.rs

Play an indefinite tone of a given `frequency`.

For instance:

```rust
extern crate beep;

use std::{thread, time};
use beep::beep;

fn main() {
  beep(440);
  thread::sleep(time::Duration::from_millis(500));
  beep(880);
  thread::sleep(time::Duration::from_millis(500));
  beep(0);
}
```
