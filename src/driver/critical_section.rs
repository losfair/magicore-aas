struct CriticalSection;
critical_section::custom_impl!(CriticalSection);

unsafe impl critical_section::Impl for CriticalSection {
  unsafe fn acquire() -> u8 {
    let interrupts_active = riscv::register::mstatus::read().mie();
    riscv::interrupt::disable();
    interrupts_active as _
  }

  unsafe fn release(token: u8) {
    if token != 0 {
      riscv::interrupt::enable();
    }
  }
}
