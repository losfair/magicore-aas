#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

mod device;
mod driver;
mod executor;
mod interrupt;
mod io;
mod polyfill;

use core::{
  arch::global_asm,
  panic::PanicInfo,
  sync::atomic::{AtomicUsize, Ordering},
  time::Duration,
};

use driver::timer::sleep;
use embassy::util::{yield_now, Forever};
use executor::Executor;

global_asm!(include_str!("interrupt.S"));

static EXECUTOR: Forever<Executor> = Forever::new();

#[no_mangle]
pub unsafe extern "C" fn rust_main() -> ! {
  println!("Hello, world!");
  interrupt::init();
  let executor = EXECUTOR.put(Executor::new());
  executor.run(|spawner| {
    spawner.spawn(task1()).unwrap();
    spawner.spawn(task2()).unwrap();
  });
}

#[embassy::task]
async fn task1() {
  loop {
    sleep(Duration::from_secs(1)).await;
    println!("Task 1 tick");
  }
}

#[embassy::task]
async fn task2() {
  loop {
    sleep(Duration::from_millis(1500)).await;
    println!("Task 2 tick");
  }
}

#[panic_handler]
fn on_panic(info: &PanicInfo) -> ! {
  println!("PANIC: {:?}", info);
  loop {}
}

unsafe fn init_clock() {}

unsafe fn print_startup_info() {}
