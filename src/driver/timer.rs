use core::time::Duration;

use embassy::util::yield_now;

use crate::interrupt::{CYCLES_PER_MICROSECOND, CYCLES_PER_SECOND};

use super::clint::get_mtime;

pub async fn sleep(dur: Duration) {
  let num_cycles = dur.as_micros() as u64 * CYCLES_PER_MICROSECOND;
  let end = get_mtime() + num_cycles;
  loop {
    yield_now().await;
    if get_mtime() >= end {
      break;
    }
  }
}
