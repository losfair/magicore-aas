// https://github.com/llvm/llvm-project/blob/09c2b7c35af8c4bad39f03e9f60df8bd07323028/compiler-rt/test/builtins/Unit/atomic_test.c

// uint64_t __atomic_fetch_add_8(uint64_t *ptr, uint64_t val, int model);
.globl __atomic_fetch_add_8
__atomic_fetch_add_8:
1:
  lr.d t0, (a0)
  add t1, t0, a1
  sc.d t1, t1, (a0)
  bnez t1, 1b
  mv a0, t0
  ret

// uint32_t __atomic_fetch_add_4(uint32_t *ptr, uint32_t val, int model);
.globl __atomic_fetch_add_4
__atomic_fetch_add_4:
1:
  lr.w t0, (a0)
  addw t1, t0, a1
  sc.w t1, t1, (a0)
  bnez t1, 1b
  mv a0, t0
  ret

// uint64_t __atomic_fetch_sub_8(uint64_t *ptr, uint64_t val, int model);
.globl __atomic_fetch_sub_8
__atomic_fetch_sub_8:
1:
  lr.d t0, (a0)
  sub t1, t0, a1
  sc.d t1, t1, (a0)
  bnez t1, 1b
  mv a0, t0
  ret

// uint64_t __atomic_fetch_sub_4(uint32_t *ptr, uint32_t val, int model);
.globl __atomic_fetch_sub_4
__atomic_fetch_sub_4:
1:
  lr.w t0, (a0)
  subw t1, t0, a1
  sc.w t1, t1, (a0)
  bnez t1, 1b
  mv a0, t0
  ret

// uint64_t __atomic_fetch_and_8(uint64_t *ptr, uint64_t val, int model);
.globl __atomic_fetch_and_8
__atomic_fetch_and_8:
1:
  lr.d t0, (a0)
  and t1, t0, a1
  sc.d t1, t1, (a0)
  bnez t1, 1b
  mv a0, t0
  ret

// uint64_t __atomic_fetch_and_4(uint32_t *ptr, uint32_t val, int model);
.globl __atomic_fetch_and_4
__atomic_fetch_and_4:
1:
  lr.w t0, (a0)
  and t1, t0, a1
  sc.w t1, t1, (a0)
  bnez t1, 1b
  mv a0, t0
  ret

// void __atomic_store_8(uint64_t *dest, uint64_t val, int model);
.globl __atomic_store_8
__atomic_store_8:
  sd a1, (a0)
  ret

// void __atomic_store_4(uint32_t *dest, uint32_t val, int model);
.globl __atomic_store_4
__atomic_store_4:
  sw a1, (a0)
  ret

// void __atomic_store_2(uint16_t *dest, uint16_t val, int model);
.globl __atomic_store_2
__atomic_store_2:
  sh a1, (a0)
  ret

// void __atomic_store_1(uint8_t *dest, uint8_t val, int model);
.globl __atomic_store_1
__atomic_store_1:
  sb a1, (a0)
  ret

// uint64_t __atomic_load_8(uint64_t *src, int model);
.globl __atomic_load_8
__atomic_load_8:
  ld a0, (a0)
  ret

// uint32_t __atomic_load_4(uint32_t *src, int model);
.globl __atomic_load_4
__atomic_load_4:
  lwu a0, (a0)
  ret

// uint16_t __atomic_load_2(uint16_t *src, int model);
.globl __atomic_load_2
__atomic_load_2:
  lhu a0, (a0)
  ret

// uint8_t __atomic_load_1(uint8_t *src, int model);
.globl __atomic_load_1
__atomic_load_1:
  lbu a0, (a0)
  ret

// bool __atomic_compare_exchange_8(uint64_t *ptr, uint64_t *expected,
//                                 uint64_t desired, int success, int failure);
.globl __atomic_compare_exchange_8
__atomic_compare_exchange_8:
  ld t1, (a1)
  1:
  lr.d t0, (a0)
  bne t0, t1, 2f
  sc.d t2, a2, (a0)
  bnez t2, 1b
  li a0, 1
  ret
  2:
  sd t0, (a1)
  li a0, 0 
  ret

// bool __atomic_compare_exchange_4(uint32_t *ptr, uint32_t *expected,
//                                 uint32_t desired, int success, int failure);
.globl __atomic_compare_exchange_4
__atomic_compare_exchange_4:
  lw t1, (a1)
1:
  lr.w t0, (a0)
  bne t0, t1, 2f
  sc.w t2, a2, (a0)
  bnez t2, 1b
  li a0, 1
  ret
2:
  sw t0, (a1)
  li a0, 0 
  ret

// uint64_t __atomic_exchange_8(uint64_t *dest, uint64_t val, int model);
.globl __atomic_exchange_8
__atomic_exchange_8:
1:
  lr.d t0, (a0)
  sc.d t2, a1, (a0)
  bnez t2, 1b
  mv a0, t0
  ret
