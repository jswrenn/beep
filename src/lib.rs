use nix::{self, ioctl_write_int_bad};
use std::fs::{File, OpenOptions};
use std::os::unix::io::AsRawFd;
use lazy_static::lazy_static;

pub use nix::Error;

const FILE            : &str = "/dev/console";
const KIOCSOUND       : u64  = 0x4B2F;
const TIMER_FREQUENCY : u32  = 1193180;

lazy_static! {
    static ref DEVICE: File =
        OpenOptions::new()
            .append(true)
            .open(FILE)
            .expect(&format!("Could not open {}", FILE));
}

ioctl_write_int_bad!(kiocsound, KIOCSOUND);

/// Play an indefinite tone of a given `hertz`.
///
/// For instance:
///
/// ```
/// use std::{thread, time::Duration};
/// use beep::beep;
///
/// fn main() {
///     beep(440);
///     thread::sleep(Duration::from_millis(500));
///     beep(880);
///     thread::sleep(Duration::from_millis(500));
///     beep(0);
/// }
/// ```
///
pub fn beep(hertz: u16) -> nix::Result<()>
{
    let period_in_clock_cycles =
        TIMER_FREQUENCY.checked_div(hertz as u32).unwrap_or(0);

    unsafe {
        kiocsound(DEVICE.as_raw_fd(), period_in_clock_cycles as i32)?;
    }

    Ok(())
}