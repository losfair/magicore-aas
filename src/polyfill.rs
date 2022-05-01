// bool __atomic_compare_exchange_1(uint8_t *ptr, uint8_t *expected, uint8_t desired, int success, int failure);
#[no_mangle]
pub unsafe extern "C" fn __atomic_compare_exchange_1(
  ptr: *mut u8,
  expected: *mut u8,
  desired: u8,
  success: i32,
  failure: i32,
) -> i32 {
  critical_section::with(|_| {
    let current_value = core::ptr::read_volatile(ptr);
    let expected_value = core::ptr::read_volatile(expected);
    if current_value == expected_value {
      core::ptr::write_volatile(ptr, desired);
      1
    } else {
      core::ptr::write_volatile(expected, current_value);
      0
    }
  })
}
