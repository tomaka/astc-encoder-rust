#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(ambiguous_glob_imports)]
#![allow(ambiguous_glob_reexports)]
#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

pub mod src {
    pub mod astcenc_averages_and_directions_cbe;
    pub mod astcenc_block_sizes_cbe;
    pub mod astcenc_color_quantize_cbe;
    pub mod astcenc_color_unquantize_cbe;
    pub mod astcenc_compress_symbolic_cbe;
    pub mod astcenc_compute_variance_cbe;
    pub mod astcenc_decompress_symbolic_cbe;
    pub mod astcenc_diagnostic_trace_cbe;
    pub mod astcenc_entry_cbe;
    pub mod astcenc_find_best_partitioning_cbe;
    pub mod astcenc_ideal_endpoints_and_weights_cbe;
    pub mod astcenc_image_cbe;
    pub mod astcenc_integer_sequence_cbe;
    pub mod astcenc_mathlib_cbe;
    pub mod astcenc_mathlib_softfloat_cbe;
    pub mod astcenc_partition_tables_cbe;
    pub mod astcenc_percentile_tables_cbe;
    pub mod astcenc_pick_best_endpoint_format_cbe;
    pub mod astcenc_quantization_cbe;
    pub mod astcenc_symbolic_physical_cbe;
    pub mod astcenc_weight_align_cbe;
    pub mod astcenc_weight_quant_xfer_tables_cbe;
} // mod src
pub mod bindings;
pub use bindings::*;
use src::astcenc_averages_and_directions_cbe::*;
use src::astcenc_block_sizes_cbe::*;
use src::astcenc_color_quantize_cbe::*;
use src::astcenc_color_unquantize_cbe::*;
use src::astcenc_compress_symbolic_cbe::*;
use src::astcenc_compute_variance_cbe::*;
use src::astcenc_decompress_symbolic_cbe::*;
use src::astcenc_diagnostic_trace_cbe::*;
use src::astcenc_entry_cbe::*;
use src::astcenc_find_best_partitioning_cbe::*;
use src::astcenc_ideal_endpoints_and_weights_cbe::*;
use src::astcenc_image_cbe::*;
use src::astcenc_integer_sequence_cbe::*;
use src::astcenc_mathlib_cbe::*;
use src::astcenc_mathlib_softfloat_cbe::*;
use src::astcenc_partition_tables_cbe::*;
use src::astcenc_percentile_tables_cbe::*;
use src::astcenc_pick_best_endpoint_format_cbe::*;
use src::astcenc_quantization_cbe::*;
use src::astcenc_symbolic_physical_cbe::*;
use src::astcenc_weight_align_cbe::*;
use src::astcenc_weight_quant_xfer_tables_cbe::*;
unsafe fn memcpy(
    d: *mut core::ffi::c_void,
    s: *mut core::ffi::c_void,
    c: u64,
) -> *mut core::ffi::c_void {
    core::ptr::copy_nonoverlapping::<u8>(s.cast_const().cast(), d.cast(), c as usize);
    d
}
unsafe fn memset(
    d: *mut core::ffi::c_void,
    ch: core::ffi::c_int,
    c: u64,
) -> *mut core::ffi::c_void {
    assert!(ch <= 255);
    core::ptr::write_bytes::<u8>(d.cast(), ch as u8, c as usize);
    d
}
fn abs(v: u32) -> u32 {
    v
}
fn cosf(v: f32) -> f32 {
    v.cos()
}
fn sinf(v: f32) -> f32 {
    v.sin()
}
fn fabsf(v: f32) -> f32 {
    v.abs()
}
fn roundf(v: f32) -> f32 {
    v.round()
}
fn sqrtf(v: f32) -> f32 {
    v.sqrt()
}
fn logf(v: f32) -> f32 {
    v.ln()
}
fn fegetround() -> core::ffi::c_uint {
    0
}
fn __assert_fail(
    _assertion: *mut core::ffi::c_void,
    _file: *mut core::ffi::c_void,
    _line: core::ffi::c_uint,
    _function: *mut core::ffi::c_void,
) -> ! {
    panic!()
}
fn LLVMMul_uov(_: core::ffi::c_ulong, a: &mut u64, b: &mut u64, out: &mut u64) -> u8 {
    let (res, carry) = (*a).overflowing_mul(*b);
    *out = res;
    carry as u8
}
unsafe fn posix_memalign(
    memptr: *mut core::ffi::c_void,
    align: u64,
    size: u64,
) -> core::ffi::c_int {
    libc::posix_memalign(memptr as *mut _, align as usize, size as usize)
}
use libc::free;
unsafe fn _Znwm(size: u64) -> *mut core::ffi::c_void {
    libc::malloc(size as libc::size_t)
}
unsafe fn _Znam(size: u64) -> *mut core::ffi::c_void {
    libc::malloc(size as libc::size_t)
}
unsafe fn _ZdaPv(ptr: *mut core::ffi::c_void) {
    libc::free(ptr)
}
unsafe fn _ZdlPvm(ptr: *mut core::ffi::c_void, _: u64) {
    libc::free(ptr)
}
unsafe fn _ZSt25__throw_bad_function_callv() -> ! {
    panic!()
}
unsafe fn _ZSt20__throw_system_errori<T>(_: T) -> ! {
    panic!()
}
// __cxa_begin_catch is only ever used right before a call to terminate()
unsafe fn __cxa_begin_catch(_: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}
unsafe fn _ZSt9terminatev() -> ! {
    panic!()
}
unsafe fn pthread_mutex_lock(mutex: *mut core::ffi::c_void) -> core::ffi::c_uint {
    libc::pthread_mutex_lock(mutex as *mut _) as core::ffi::c_uint
}
unsafe fn pthread_mutex_unlock(mutex: *mut core::ffi::c_void) -> core::ffi::c_uint {
    libc::pthread_mutex_unlock(mutex as *mut _) as core::ffi::c_uint
}
