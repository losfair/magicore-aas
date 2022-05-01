#![allow(dead_code)]

use core::arch::asm;

pub unsafe fn io_write_64(memory: *mut u64, value: u64) {
  core::ptr::write_volatile(memory, value);
  asm!("fence w, rw");
}

pub unsafe fn io_read_64(memory: *const u64) -> u64 {
  core::ptr::read_volatile(memory)
}

pub unsafe fn io_write_32(memory: *mut u32, value: u32) {
  core::ptr::write_volatile(memory, value);
  asm!("fence w, rw");
}

pub unsafe fn io_read_32(memory: *const u32) -> u32 {
  core::ptr::read_volatile(memory)
}
