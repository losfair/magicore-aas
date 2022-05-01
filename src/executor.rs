use core::marker::PhantomData;
use core::ptr;

use embassy::executor::{raw, Spawner};

use crate::{println, driver::clint::set_mtimecmp};

pub struct Executor {
  inner: raw::Executor,
  not_send: PhantomData<*mut ()>,
}

impl Executor {
  /// Create a new Executor.
  pub fn new() -> Self {
    Self {
      inner: raw::Executor::new(|_| (), ptr::null_mut()),
      not_send: PhantomData,
    }
  }

  pub fn run(&'static mut self, init: impl FnOnce(Spawner)) -> ! {
    init(self.inner.spawner());

    unsafe {
      riscv::register::mie::set_mtimer();
      riscv::register::mie::set_mext();
      riscv::register::mstatus::set_mie();
      set_mtimecmp(0);
    }

    loop {
      unsafe {
        self.inner.poll();
        riscv::asm::wfi();
      }
    }
  }
}
