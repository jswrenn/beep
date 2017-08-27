#![feature(libc)]
extern crate libc;
#[macro_use]
extern crate lazy_static;
extern crate dimensioned;
extern crate num_traits;

use libc::ioctl;
use std::fs::{File, OpenOptions};
use std::os::unix::io::AsRawFd;
use num_traits::{Num, ToPrimitive};

use dimensioned::si::Hertz;

lazy_static! {
  static ref CONSOLE: File =
    OpenOptions::new()
      .append(true)
      .open("/dev/console")
      .expect("Could not open /dev/console:");
}

const KIOCSOUND       : u64 = 0x4B2F;
const TIMER_FREQUENCY : u32 = 1193180;

/// Play an indefinite tone of a given `frequency`.
///
/// For instance:
///
/// ```
/// extern crate beep;
/// extern crate dimensioned;
///
/// use std::{thread, time};
/// use beep::beep;
/// use dimensioned::si;
///
/// fn main() {
///   beep(440. * si::HZ);
///   thread::sleep(time::Duration::from_millis(500));
///   beep(si::Hertz::new(880));
///   thread::sleep(time::Duration::from_millis(500));
///   beep(0. * si::HZ);
/// }
/// ```
///
pub fn beep<F, N>(frequency: F)
  where F: Into<Hertz<N>>,
        N: Num + ToPrimitive
{
  unsafe {
    ioctl(CONSOLE.as_raw_fd(), KIOCSOUND,
      frequency.into().value_unsafe.to_u32()
        .and_then(|frequency|
          TIMER_FREQUENCY.checked_div(frequency))
        .unwrap_or(0));
  }
}