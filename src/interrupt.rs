use riscv::register::{
  mcause::{Interrupt, Trap},
  mtvec::TrapMode,
};

use crate::{
  device::{PLIC_MASKS, PLIC_PENDINGS},
  driver::clint,
  io::{io_read_32, io_write_32}, println,
};

pub const CYCLES_PER_SECOND: u64 = 80_000_000;
pub const CYCLES_PER_MICROSECOND: u64 = CYCLES_PER_SECOND / 1_000_000;
pub const TICKS_PER_SECOND: u64 = 1000;
pub const CYCLES_PER_TICK: u64 = CYCLES_PER_SECOND / TICKS_PER_SECOND;

extern "C" {
  fn asm_intr_entry();
  fn asm_return_from_interrupt(ctx: &IntrContext) -> !;
}

#[repr(C)]
pub struct IntrContext {
  pub regs: [u64; 31],
}

impl IntrContext {
  fn read_reg(&self, reg: usize) -> u64 {
    if reg == 0 {
      0
    } else {
      let index = reg - 1;
      assert!(index < self.regs.len());
      self.regs[index]
    }
  }
}

pub unsafe fn init() {
  riscv::register::mtvec::write(asm_intr_entry as _, TrapMode::Direct);
}

pub fn mask_irq(id: u32) {
  critical_section::with(|_| unsafe {
    let prev = io_read_32(PLIC_MASKS);
    io_write_32(PLIC_MASKS, prev & !(1 << id));
  })
}

pub fn unmask_irq(id: u32) {
  critical_section::with(|_| unsafe {
    let prev = io_read_32(PLIC_MASKS);
    io_write_32(PLIC_MASKS, prev | (1 << id));
  })
}

pub type IntrHandler = fn(u32) -> IntrAction;

pub enum IntrAction {
  Ack,
  DoNothing,
}

static mut INTR_HANDLERS: [Option<IntrHandler>; 256] = [None; 256];

#[no_mangle]
pub extern "C" fn rust_intr_entry(ctx: &mut IntrContext) {
  let mepc = riscv::register::mepc::read();
  let mcause = riscv::register::mcause::read();

  match mcause.cause() {
    Trap::Exception(exc) => {
      let mtval = riscv::register::mtval::read();
      panic!("Exception: mepc=0x{:016x} exc={:?} mtval=0x{:016x}", mepc, exc, mtval);
    }
    Trap::Interrupt(intr) => match intr {
      Interrupt::MachineExternal => unsafe {
        let mut clears = 0u32;
        let pending = io_read_32(PLIC_PENDINGS);
        for i in 0u32..32u32 {
          if pending & (1 << i) != 0 {
            let handler = INTR_HANDLERS[i as usize]
              .unwrap_or_else(|| panic!("interrupt handler not found: {}", i));
            let action = handler(i);
            match action {
              IntrAction::Ack => {
                clears |= 1 << i;
              }
              IntrAction::DoNothing => {}
            }
          }
        }
        if clears != 0 {
          io_write_32(PLIC_PENDINGS, clears);
        }
      },
      Interrupt::MachineSoft => {}
      Interrupt::MachineTimer => {
        let now = clint::get_mtime();
        clint::set_mtimecmp(now + CYCLES_PER_TICK);
      }
      _ => {}
    },
  }
}
