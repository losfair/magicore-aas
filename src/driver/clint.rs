use crate::{
  device::{CLINT_CMP, CLINT_TIME},
  io::{io_read_32, io_write_32},
};

pub fn get_mtime() -> u64 {
  unsafe {
    let hi_reg = CLINT_TIME.offset(1);
    let lo_reg = CLINT_TIME;
    loop {
      let hi = io_read_32(hi_reg);
      let lo = io_read_32(lo_reg);
      let hi2 = io_read_32(hi_reg);
      if hi == hi2 {
        return (hi as u64) << 32 | (lo as u64);
      }
    }
  }
}

pub fn set_mtimecmp(value: u64) {
  unsafe {
    let hi_reg = CLINT_CMP.offset(1);
    let lo_reg = CLINT_CMP;
    io_write_32(hi_reg, 0xffff_ffff);
    io_write_32(lo_reg, value as u32);
    io_write_32(hi_reg, (value >> 32) as u32);
  }
}
