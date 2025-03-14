use crate::*;

pub type __uint32_t = core::ffi::c_uint;
pub type __int64_t = core::ffi::c_long;
pub type __uint64_t = core::ffi::c_ulong;
pub type int64_t = __int64_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[inline(always)]
unsafe extern "C" fn llvm_add_u64(mut a: uint64_t, mut b: uint64_t) -> uint64_t {
    let mut r: uint64_t = a.wrapping_add(b);
    return r;
}
#[inline(always)]
unsafe extern "C" fn llvm_sub_u32(mut a: uint32_t, mut b: uint32_t) -> uint32_t {
    let mut r: uint32_t = a.wrapping_sub(b);
    return r;
}
#[inline(always)]
unsafe extern "C" fn llvm_lshr_u64(mut a: uint64_t, mut b: uint64_t) -> uint64_t {
    let mut r: uint64_t = a >> b;
    return r;
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn _ZN4astc9rand_initEPm(mut _1: *mut core::ffi::c_void) {
    let mut _2: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _4: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    _2 = _1;
    _3 = _2;
    *(_3 as *mut uint64_t) = -(362010416112341909 as core::ffi::c_long) as uint64_t;
    _4 = _2;
    *(&raw mut *(_4 as *mut uint64_t).offset(1 as core::ffi::c_int as int64_t as isize)
        as *mut uint64_t) = -(1030452625173160591 as core::ffi::c_long) as uint64_t;
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn _ZN4astc4randEPm(mut _5: *mut core::ffi::c_void) -> uint64_t {
    let mut _6: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _7: uint64_t = 0;
    let mut _8: uint64_t = 0;
    let mut _9: uint64_t = 0;
    let mut _10: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _11: uint64_t = 0;
    let mut _12: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _13: uint64_t = 0;
    let mut _14: uint64_t = 0;
    let mut _15: uint64_t = 0;
    let mut _16: uint64_t = 0;
    let mut _17: uint64_t = 0;
    let mut _18: uint64_t = 0;
    let mut _19: uint64_t = 0;
    let mut _20: uint64_t = 0;
    let mut _21: uint64_t = 0;
    let mut _22: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _23: uint64_t = 0;
    let mut _24: uint64_t = 0;
    let mut _25: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _26: uint64_t = 0;
    _6 = _5;
    _10 = _6;
    _11 = *(_10 as *mut uint64_t);
    _7 = _11;
    _12 = _6;
    _13 = *(&raw mut *(_12 as *mut uint64_t).offset(1 as core::ffi::c_int as int64_t as isize)
        as *mut uint64_t);
    _8 = _13;
    _14 = _7;
    _15 = _8;
    _9 = llvm_add_u64(_14, _15);
    _16 = _7;
    _17 = _8;
    _8 = _17 ^ _16;
    _18 = _7;
    _19 = _ZL4rotlmi(_18, 24);
    _20 = _8;
    _21 = _8;
    _22 = _6;
    *(_22 as *mut uint64_t) = _19 ^ _20 ^ _21 << 16 as core::ffi::c_int;
    _23 = _8;
    _24 = _ZL4rotlmi(_23, 37);
    _25 = _6;
    *(&raw mut *(_25 as *mut uint64_t).offset(1 as core::ffi::c_int as int64_t as isize)
        as *mut uint64_t) = _24;
    _26 = _9;
    return _26;
}
#[inline(never)]
unsafe extern "C" fn _ZL4rotlmi(mut _27: uint64_t, mut _28: uint32_t) -> uint64_t {
    let mut _29: uint64_t = 0;
    let mut _30: uint32_t = 0;
    let mut _31: uint64_t = 0;
    let mut _32: uint32_t = 0;
    let mut _33: uint64_t = 0;
    let mut _34: uint32_t = 0;
    _29 = _27;
    _30 = _28;
    _31 = _29;
    _32 = _30;
    _33 = _29;
    _34 = _30;
    return _31 << _32 as uint64_t | llvm_lshr_u64(_33, llvm_sub_u32(64, _34) as uint64_t);
}
