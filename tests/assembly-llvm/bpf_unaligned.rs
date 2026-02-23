//@ assembly-output: emit-asm
//@ compile-flags: --target bpfel-unknown-none -C target_feature=+allows-misaligned-mem-access
//@ needs-llvm-components: bpf
#![crate_type = "rlib"]
#![no_std]

use core::ptr;

// CHECK-LABEL: test_load_i64:
// CHECK:       r0 = *(u64 *)(r1 + 0)
#[no_mangle]
pub unsafe fn test_load_i64(p: *const u64) -> u64 {
    ptr::read_unaligned(p)
}

// CHECK-LABEL: test_store_i64:
// CHECK:       *(u64 *)(r1 + 0) = r2
#[no_mangle]
pub unsafe fn test_store_i64(p: *mut u64, v: u64) {
    ptr::write_unaligned(p, v);
}
