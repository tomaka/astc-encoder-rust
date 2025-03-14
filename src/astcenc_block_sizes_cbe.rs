use crate::*;

pub type __uint8_t = core::ffi::c_uchar;
pub type __uint16_t = core::ffi::c_ushort;
pub type __int32_t = core::ffi::c_int;
pub type __uint32_t = core::ffi::c_uint;
pub type __int64_t = core::ffi::c_long;
pub type __uint64_t = core::ffi::c_ulong;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type bool_0 = core::ffi::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_4_uint32_t {
    pub array: [uint32_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_struct_struct_OC_decimation_mode {
    pub field0: uint8_t,
    pub field1: uint8_t,
    pub field2: uint16_t,
    pub field3: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_87_struct_AC_l_struct_struct_OC_decimation_mode {
    pub array: [l_struct_struct_OC_decimation_mode; 87],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_216_uint8_t {
    pub array: [uint8_t; 216],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_4_struct_AC_l_array_216_uint8_t {
    pub array: [l_array_216_uint8_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_216_float {
    pub array: [core::ffi::c_float; 216],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_4_struct_AC_l_array_216_float {
    pub array: [l_array_216_float; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_64_uint8_t {
    pub array: [uint8_t; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_216_struct_AC_l_array_64_uint8_t {
    pub array: [l_array_64_uint8_t; 216],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_64_float {
    pub array: [core::ffi::c_float; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_216_struct_AC_l_array_64_float {
    pub array: [l_array_64_float; 216],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_struct_struct_OC_decimation_info {
    pub field0: uint8_t,
    pub field1: uint8_t,
    pub field2: uint8_t,
    pub field3: uint8_t,
    pub field4: uint8_t,
    pub field5: uint8_t,
    pub field6: crate::l_array_216_uint8_t,
    pub field7: crate::l_array_4_struct_AC_l_array_216_uint8_t,
    pub field8: crate::l_array_4_struct_AC_l_array_216_uint8_t,
    pub field9: crate::l_array_4_struct_AC_l_array_216_float,
    pub field10: crate::l_array_64_uint8_t,
    pub field11: crate::l_array_216_struct_AC_l_array_64_uint8_t,
    pub field12: crate::l_array_216_struct_AC_l_array_64_float,
    pub field13: crate::l_array_216_struct_AC_l_array_64_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_87_struct_AC_l_struct_struct_OC_decimation_info {
    pub array: [l_struct_struct_OC_decimation_info; 87],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_2048_uint16_t {
    pub array: [uint16_t; 2048],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_struct_struct_OC_block_mode {
    pub field0: uint16_t,
    pub field1: uint8_t,
    pub field2: uint8_t,
    pub field3: uint8_t,
    pub field4: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_2048_struct_AC_l_struct_struct_OC_block_mode {
    pub array: [l_struct_struct_OC_block_mode; 2048],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_4_uint8_t {
    pub array: [uint8_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_struct_struct_OC_partition_info {
    pub field0: uint16_t,
    pub field1: uint16_t,
    pub field2: crate::l_array_4_uint8_t,
    pub field3: crate::l_array_216_uint8_t,
    pub field4: crate::l_array_4_struct_AC_l_array_216_uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_3073_struct_AC_l_struct_struct_OC_partition_info {
    pub array: [l_struct_struct_OC_partition_info; 3073],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_1024_uint16_t {
    pub array: [uint16_t; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_3_struct_AC_l_array_1024_uint16_t {
    pub array: [l_array_1024_uint16_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_2_uint64_t {
    pub array: [uint64_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_1024_struct_AC_l_array_2_uint64_t {
    pub array: [l_array_2_uint64_t; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_3_uint64_t {
    pub array: [uint64_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_1024_struct_AC_l_array_3_uint64_t {
    pub array: [l_array_3_uint64_t; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_4_uint64_t {
    pub array: [uint64_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_1024_struct_AC_l_array_4_uint64_t {
    pub array: [l_array_4_uint64_t; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_struct_struct_OC_block_size_descriptor {
    pub field0: uint8_t,
    pub field1: uint8_t,
    pub field2: uint8_t,
    pub field3: uint8_t,
    pub field4: uint32_t,
    pub field5: uint32_t,
    pub field6: uint32_t,
    pub field7: uint32_t,
    pub field8: uint32_t,
    pub field9: uint32_t,
    pub field10: uint32_t,
    pub field11: crate::l_array_4_uint32_t,
    pub field12: crate::l_array_4_uint32_t,
    pub field13: crate::l_array_87_struct_AC_l_struct_struct_OC_decimation_mode,
    pub field14: crate::l_array_87_struct_AC_l_struct_struct_OC_decimation_info,
    pub field15: crate::l_array_2048_uint16_t,
    pub field16: crate::l_array_2048_struct_AC_l_struct_struct_OC_block_mode,
    pub field17: crate::l_array_3073_struct_AC_l_struct_struct_OC_partition_info,
    pub field18: crate::l_array_3_struct_AC_l_array_1024_uint16_t,
    pub field19: crate::l_array_64_uint8_t,
    pub field20: crate::l_array_1024_struct_AC_l_array_2_uint64_t,
    pub field21: crate::l_array_1024_struct_AC_l_array_3_uint64_t,
    pub field22: crate::l_array_1024_struct_AC_l_array_4_uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_216_struct_AC_l_array_4_uint8_t {
    pub array: [l_array_4_uint8_t; 216],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_64_struct_AC_l_array_216_uint8_t {
    pub array: [l_array_216_uint8_t; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_struct_struct_OC_dt_init_working_buffers {
    pub field0: crate::l_array_216_uint8_t,
    pub field1: crate::l_array_216_struct_AC_l_array_4_uint8_t,
    pub field2: crate::l_array_216_struct_AC_l_array_4_uint8_t,
    pub field3: crate::l_array_64_uint8_t,
    pub field4: crate::l_array_64_struct_AC_l_array_216_uint8_t,
    pub field5: crate::l_array_64_struct_AC_l_array_216_uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_22_uint8_t {
    pub array: [uint8_t; 22],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_50_uint8_t {
    pub array: [uint8_t; 50],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_159_uint8_t {
    pub array: [uint8_t; 159],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_21_uint8_t {
    pub array: [uint8_t; 21],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_39_uint8_t {
    pub array: [uint8_t; 39],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_106_uint8_t {
    pub array: [uint8_t; 106],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_37_uint8_t {
    pub array: [uint8_t; 37],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_34_uint8_t {
    pub array: [uint8_t; 34],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_149_uint8_t {
    pub array: [uint8_t; 149],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_44_uint8_t {
    pub array: [uint8_t; 44],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_131_uint8_t {
    pub array: [uint8_t; 131],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_13_uint8_t {
    pub array: [uint8_t; 13],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_438_uint32_t {
    pub array: [uint32_t; 438],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_2_uint32_t {
    pub array: [uint32_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_204_uint32_t {
    pub array: [uint32_t; 204],
}
#[inline(always)]
unsafe extern "C" fn llvm_fcmp_ole(
    mut X: core::ffi::c_double,
    mut Y: core::ffi::c_double,
) -> core::ffi::c_int {
    return (X <= Y) as core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn llvm_fcmp_une(
    mut X: core::ffi::c_double,
    mut Y: core::ffi::c_double,
) -> core::ffi::c_int {
    return (X != Y) as core::ffi::c_int;
}
static mut _OC_str: crate::l_array_22_uint8_t = unsafe {
    {
        let mut init = crate::l_array_22_uint8_t {
            array: *::core::mem::transmute::<&[u8; 22], &mut [uint8_t; 22]>(
                b"weights_per_block > 0\0",
            ),
        };
        init
    }
};
static mut _OC_str_OC_1: crate::l_array_50_uint8_t = unsafe {
    {
        let mut init = crate::l_array_50_uint8_t {
            array: *::core::mem::transmute::<&[u8; 50], &mut [uint8_t; 50]>(
                b"/root/astc-encoder/Source/astcenc_block_sizes.cpp\0",
            ),
        };
        init
    }
};
static mut __PRETTY_FUNCTION___OC__ZL23init_decimation_info_3djjjjjjR15decimation_infoR23dt_init_working_buffers: crate::l_array_159_uint8_t = unsafe {
    {
        let mut init = crate::l_array_159_uint8_t {
            array: *::core::mem::transmute::<
                &[u8; 159],
                &mut [uint8_t; 159],
            >(
                b"void init_decimation_info_3d(unsigned int, unsigned int, unsigned int, unsigned int, unsigned int, unsigned int, decimation_info &, dt_init_working_buffers &)\0",
            ),
        };
        init
    }
};
static mut _OC_str_OC_2: crate::l_array_21_uint8_t = unsafe {
    {
        let mut init = crate::l_array_21_uint8_t {
            array: *::core::mem::transmute::<&[u8; 21], &mut [uint8_t; 21]>(
                b"texels_per_block > 0\0",
            ),
        };
        init
    }
};
static mut _OC_str_OC_3: crate::l_array_39_uint8_t = unsafe {
    {
        let mut init = crate::l_array_39_uint8_t {
            array: *::core::mem::transmute::<&[u8; 39], &mut [uint8_t; 39]>(
                b"bsd.block_mode_count_1plane_always > 0\0",
            ),
        };
        init
    }
};
static mut __PRETTY_FUNCTION___OC__ZL34construct_block_size_descriptor_2djjbfR21block_size_descriptor: crate::l_array_106_uint8_t = unsafe {
    {
        let mut init = crate::l_array_106_uint8_t {
            array: *::core::mem::transmute::<
                &[u8; 106],
                &mut [uint8_t; 106],
            >(
                b"void construct_block_size_descriptor_2d(unsigned int, unsigned int, bool, float, block_size_descriptor &)\0",
            ),
        };
        init
    }
};
static mut _OC_str_OC_4: crate::l_array_37_uint8_t = unsafe {
    {
        let mut init = crate::l_array_37_uint8_t {
            array: *::core::mem::transmute::<&[u8; 37], &mut [uint8_t; 37]>(
                b"bsd.decimation_mode_count_always > 0\0",
            ),
        };
        init
    }
};
static mut _OC_str_OC_5: crate::l_array_34_uint8_t = unsafe {
    {
        let mut init = crate::l_array_34_uint8_t {
            array: *::core::mem::transmute::<&[u8; 34], &mut [uint8_t; 34]>(
                b"weight_count <= BLOCK_MAX_WEIGHTS\0",
            ),
        };
        init
    }
};
static mut __PRETTY_FUNCTION___OC__ZL21construct_dt_entry_2djjjjR21block_size_descriptorR23dt_init_working_buffersj: crate::l_array_149_uint8_t = unsafe {
    {
        let mut init = crate::l_array_149_uint8_t {
            array: *::core::mem::transmute::<
                &[u8; 149],
                &mut [uint8_t; 149],
            >(
                b"void construct_dt_entry_2d(unsigned int, unsigned int, unsigned int, unsigned int, block_size_descriptor &, dt_init_working_buffers &, unsigned int)\0",
            ),
        };
        init
    }
};
static mut _OC_str_OC_6: crate::l_array_44_uint8_t = unsafe {
    {
        let mut init = crate::l_array_44_uint8_t {
            array: *::core::mem::transmute::<&[u8; 44], &mut [uint8_t; 44]>(
                b"maxprec_1plane >= 0 || maxprec_2planes >= 0\0",
            ),
        };
        init
    }
};
static mut __PRETTY_FUNCTION___OC__ZL23init_decimation_info_2djjjjR15decimation_infoR23dt_init_working_buffers: crate::l_array_131_uint8_t = unsafe {
    {
        let mut init = crate::l_array_131_uint8_t {
            array: *::core::mem::transmute::<
                &[u8; 131],
                &mut [uint8_t; 131],
            >(
                b"void init_decimation_info_2d(unsigned int, unsigned int, unsigned int, unsigned int, decimation_info &, dt_init_working_buffers &)\0",
            ),
        };
        init
    }
};
static mut _OC_str_OC_7: crate::l_array_13_uint8_t = unsafe {
    {
        let mut init = crate::l_array_13_uint8_t {
            array: *::core::mem::transmute::<&[u8; 13], &mut [uint8_t; 13]>(b"x_texels > 0\0"),
        };
        init
    }
};
static mut _OC_str_OC_8: crate::l_array_13_uint8_t = unsafe {
    {
        let mut init = crate::l_array_13_uint8_t {
            array: *::core::mem::transmute::<&[u8; 13], &mut [uint8_t; 13]>(b"y_texels > 0\0"),
        };
        init
    }
};
#[inline(always)]
unsafe extern "C" fn llvm_select_u32(
    mut condition: bool_0,
    mut iftrue: uint32_t,
    mut ifnot: uint32_t,
) -> uint32_t {
    let mut r: uint32_t = 0;
    r = if condition as core::ffi::c_int != 0 {
        iftrue
    } else {
        ifnot
    };
    return r;
}
#[inline(always)]
unsafe extern "C" fn llvm_add_u8(mut a: uint8_t, mut b: uint8_t) -> uint8_t {
    let mut r: uint8_t = (a as core::ffi::c_int + b as core::ffi::c_int) as uint8_t;
    return r;
}
#[inline(always)]
unsafe extern "C" fn llvm_add_u32(mut a: uint32_t, mut b: uint32_t) -> uint32_t {
    let mut r: uint32_t = a.wrapping_add(b);
    return r;
}
#[inline(always)]
unsafe extern "C" fn llvm_sub_u32(mut a: uint32_t, mut b: uint32_t) -> uint32_t {
    let mut r: uint32_t = a.wrapping_sub(b);
    return r;
}
#[inline(always)]
unsafe extern "C" fn llvm_mul_u32(mut a: uint32_t, mut b: uint32_t) -> uint32_t {
    let mut r: uint32_t = a * b;
    return r;
}
#[inline(always)]
unsafe extern "C" fn llvm_fmul_f32(
    mut a: core::ffi::c_float,
    mut b: core::ffi::c_float,
) -> core::ffi::c_float {
    let mut r: core::ffi::c_float = a * b;
    return r;
}
#[inline(always)]
unsafe extern "C" fn llvm_udiv_u32(mut a: uint32_t, mut b: uint32_t) -> uint32_t {
    let mut r: uint32_t = a / b;
    return r;
}
#[inline(always)]
unsafe extern "C" fn llvm_srem_u32(mut a: int32_t, mut b: int32_t) -> uint32_t {
    let mut r: uint32_t = (a % b) as uint32_t;
    return r;
}
#[inline(always)]
unsafe extern "C" fn llvm_lshr_u32(mut a: uint32_t, mut b: uint32_t) -> uint32_t {
    let mut r: uint32_t = a >> b;
    return r;
}
#[inline(always)]
unsafe extern "C" fn llvm_ashr_u32(mut a: int32_t, mut b: int32_t) -> uint32_t {
    let mut r: uint32_t = (a >> b) as uint32_t;
    return r;
}
#[inline(always)]
unsafe extern "C" fn llvm_and_u8(mut a: uint8_t, mut b: uint8_t) -> uint8_t {
    let mut r: uint8_t = (a as core::ffi::c_int & b as core::ffi::c_int) as uint8_t;
    return r;
}
#[inline(always)]
unsafe extern "C" fn llvm_or_u8(mut a: uint8_t, mut b: uint8_t) -> uint8_t {
    let mut r: uint8_t = (a as core::ffi::c_int | b as core::ffi::c_int) as uint8_t;
    return r;
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn _Z26init_block_size_descriptorjjjbjfR21block_size_descriptor(
    mut _1: uint32_t,
    mut _2: uint32_t,
    mut _3: uint32_t,
    mut _4: bool_0,
    mut _5: uint32_t,
    mut _6: core::ffi::c_float,
    mut _7: *mut core::ffi::c_void,
) {
    let mut _8: uint32_t = 0;
    let mut _9: uint32_t = 0;
    let mut _10: uint32_t = 0;
    let mut _11: uint8_t = 0;
    let mut _12: uint32_t = 0;
    let mut _13: core::ffi::c_float = 0.;
    let mut _14: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _15: uint32_t = 0;
    let mut _16: uint32_t = 0;
    let mut _17: uint32_t = 0;
    let mut _18: uint32_t = 0;
    let mut _19: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _20: uint32_t = 0;
    let mut _21: uint32_t = 0;
    let mut _22: uint8_t = 0;
    let mut _23: core::ffi::c_float = 0.;
    let mut _24: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _25: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _26: uint8_t = 0;
    let mut _27: uint32_t = 0;
    _8 = _1;
    _9 = _2;
    _10 = _3;
    _11 = _4;
    _12 = _5;
    _13 = _6;
    _14 = _7;
    _15 = _10;
    if _15 > 1 as core::ffi::c_uint {
        _16 = _8;
        _17 = _9;
        _18 = _10;
        _19 = _14;
        _ZL34construct_block_size_descriptor_3djjjR21block_size_descriptor(_16, _17, _18, _19);
    } else {
        _20 = _8;
        _21 = _9;
        _22 = _11;
        _23 = _13;
        _24 = _14;
        _ZL34construct_block_size_descriptor_2djjbfR21block_size_descriptor(
            _20,
            _21,
            (_22 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0,
            _23,
            _24,
        );
    }
    _25 = _14;
    _26 = _11;
    _27 = _12;
    _Z21init_partition_tablesR21block_size_descriptorbj(
        _25,
        (_26 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0,
        _27,
    );
}
#[inline(never)]
unsafe extern "C" fn _ZL34construct_block_size_descriptor_3djjjR21block_size_descriptor(
    mut _31: uint32_t,
    mut _32: uint32_t,
    mut _33: uint32_t,
    mut _34: *mut core::ffi::c_void,
) {
    let mut current_block: u64;
    let mut _35: uint32_t = 0;
    let mut _36: uint32_t = 0;
    let mut _37: uint32_t = 0;
    let mut _38: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _39: crate::l_array_438_uint32_t = crate::l_array_438_uint32_t { array: [0; 438] };
    let mut _40: uint32_t = 0;
    let mut _41: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _42: uint32_t = 0;
    let mut _43: uint32_t = 0;
    let mut _44: uint32_t = 0;
    let mut _45: uint32_t = 0;
    let mut _46: uint32_t = 0;
    let mut _47: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _48: uint32_t = 0;
    let mut _49: uint32_t = 0;
    let mut _50: uint32_t = 0;
    let mut _51: uint32_t = 0;
    let mut _52: uint32_t = 0;
    let mut _53: uint32_t = 0;
    let mut _54: uint32_t = 0;
    let mut _55: uint32_t = 0;
    let mut _56: crate::l_array_2_uint32_t = crate::l_array_2_uint32_t { array: [0; 2] };
    let mut _57: uint32_t = 0;
    let mut _58: uint32_t = 0;
    let mut _59: uint32_t = 0;
    let mut _60: uint32_t = 0;
    let mut _61: uint32_t = 0;
    let mut _62: uint8_t = 0;
    let mut _63: uint32_t = 0;
    let mut _64: uint32_t = 0;
    let mut _65: uint8_t = 0;
    let mut _66: uint32_t = 0;
    let mut _67: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _68: uint32_t = 0;
    let mut _69: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _70: uint32_t = 0;
    let mut _71: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _72: uint32_t = 0;
    let mut _73: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _74: uint32_t = 0;
    let mut _75: uint32_t = 0;
    let mut _76: uint32_t = 0;
    let mut _77: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _78: uint32_t = 0;
    let mut _79: uint32_t = 0;
    let mut _80: uint32_t = 0;
    let mut _81: uint32_t = 0;
    let mut _82: uint32_t = 0;
    let mut _83: uint32_t = 0;
    let mut _84: uint32_t = 0;
    let mut _85: uint32_t = 0;
    let mut _86: uint32_t = 0;
    let mut _87: uint32_t = 0;
    let mut _88: uint32_t = 0;
    let mut _89: uint32_t = 0;
    let mut _90: uint32_t = 0;
    let mut _91: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _92: uint32_t = 0;
    let mut _93: uint32_t = 0;
    let mut _94: uint32_t = 0;
    let mut _95: uint32_t = 0;
    let mut _96: uint32_t = 0;
    let mut _97: uint32_t = 0;
    let mut _98: uint32_t = 0;
    let mut _99: uint32_t = 0;
    let mut _100: uint32_t = 0;
    let mut _101: uint32_t = 0;
    let mut _102: uint32_t = 0;
    let mut _103: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _104: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _105: uint32_t = 0;
    let mut _106: uint32_t = 0;
    let mut _107: uint32_t = 0;
    let mut _108: uint32_t = 0;
    let mut _109: uint32_t = 0;
    let mut _110: uint32_t = 0;
    let mut _111: uint32_t = 0;
    let mut _112: uint32_t = 0;
    let mut _113: uint32_t = 0;
    let mut _114: uint32_t = 0;
    let mut _115: uint32_t = 0;
    let mut _116: uint32_t = 0;
    let mut _117: uint32_t = 0;
    let mut _118: uint32_t = 0;
    let mut _119: uint32_t = 0;
    let mut _120: uint32_t = 0;
    let mut _121: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _122: uint32_t = 0;
    let mut _123: uint32_t = 0;
    let mut _124: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _125: uint32_t = 0;
    let mut _126: uint32_t = 0;
    let mut _127: uint64_t = 0;
    let mut _128: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _129: uint32_t = 0;
    let mut _130: uint32_t = 0;
    let mut _131: uint64_t = 0;
    let mut _132: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _133: uint32_t = 0;
    let mut _134: uint32_t = 0;
    let mut _135: uint32_t = 0;
    let mut _136: uint32_t = 0;
    let mut _137: uint32_t = 0;
    let mut _138: uint32_t = 0;
    let mut _139: uint32_t = 0;
    let mut _140: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _141: uint32_t = 0;
    let mut _142: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _143: uint32_t = 0;
    let mut _144: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _145: uint32_t = 0;
    let mut _146: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _147: uint32_t = 0;
    let mut _148: uint32_t = 0;
    let mut _149: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _150: uint32_t = 0;
    let mut _151: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _152: uint32_t = 0;
    let mut _153: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _154: uint32_t = 0;
    let mut _155: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _156: uint32_t = 0;
    let mut _157: uint32_t = 0;
    let mut _158: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _159: uint32_t = 0;
    let mut _160: uint32_t = 0;
    let mut _161: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _162: uint32_t = 0;
    let mut _163: uint16_t = 0;
    let mut _164: uint32_t = 0;
    let mut _165: bool_0 = 0;
    let mut _166: uint8_t = 0;
    let mut _167: uint32_t = 0;
    let mut _168: uint32_t = 0;
    let mut _169: uint32_t = 0;
    let mut _170: uint32_t = 0;
    let mut _171: uint32_t = 0;
    let mut _172: uint32_t = 0;
    let mut _173: uint32_t = 0;
    let mut _174: uint8_t = 0;
    let mut _175: uint32_t = 0;
    let mut _176: uint8_t = 0;
    let mut _177: uint8_t = 0;
    let mut _178: uint32_t = 0;
    let mut _179: uint32_t = 0;
    let mut _180: uint32_t = 0;
    let mut _181: uint32_t = 0;
    let mut _182: uint32_t = 0;
    let mut _183: uint32_t = 0;
    let mut _184: uint32_t = 0;
    let mut _185: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _186: uint32_t = 0;
    let mut _187: uint32_t = 0;
    let mut _188: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _189: uint32_t = 0;
    let mut _190: uint32_t = 0;
    let mut _191: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _192: uint32_t = 0;
    let mut _193: uint8_t = 0;
    let mut _194: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _195: uint32_t = 0;
    let mut _196: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _197: uint8_t = 0;
    let mut _198: uint32_t = 0;
    let mut _199: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _200: uint32_t = 0;
    let mut _201: uint32_t = 0;
    let mut _202: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _203: uint32_t = 0;
    let mut _204: uint32_t = 0;
    let mut _205: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _206: uint32_t = 0;
    let mut _207: uint32_t = 0;
    let mut _208: uint32_t = 0;
    let mut _209: uint32_t = 0;
    let mut _210: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _211: uint32_t = 0;
    let mut _212: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _213: uint32_t = 0;
    let mut _214: uint32_t = 0;
    let mut _215: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _216: uint32_t = 0;
    let mut _217: uint32_t = 0;
    let mut _218: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _219: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _220: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    _35 = _31;
    _36 = _32;
    _37 = _33;
    _38 = _34;
    _40 = 0;
    _67 = _Znwm(29656);
    _41 = _67;
    _68 = _35;
    _69 = _38;
    *(&raw mut (*(_69 as *mut l_struct_struct_OC_block_size_descriptor)).field0 as *mut uint8_t) =
        _68 as uint8_t;
    _70 = _36;
    _71 = _38;
    *(&raw mut (*(_71 as *mut l_struct_struct_OC_block_size_descriptor)).field1 as *mut uint8_t) =
        _70 as uint8_t;
    _72 = _37;
    _73 = _38;
    *(&raw mut (*(_73 as *mut l_struct_struct_OC_block_size_descriptor)).field2 as *mut uint8_t) =
        _72 as uint8_t;
    _74 = _35;
    _75 = _36;
    _76 = _37;
    _77 = _38;
    *(&raw mut (*(_77 as *mut l_struct_struct_OC_block_size_descriptor)).field3 as *mut uint8_t) =
        llvm_mul_u32(llvm_mul_u32(_74, _75), _76) as uint8_t;
    _42 = 0;
    loop {
        _78 = _42;
        if !(_78 < 438 as core::ffi::c_uint) {
            break;
        }
        _79 = _42;
        *(&raw mut *(_39.array)
            .as_mut_ptr()
            .offset(_79 as uint64_t as int64_t as isize) as *mut uint32_t) =
            -(1 as core::ffi::c_int) as uint32_t;
        _80 = _42;
        _42 = llvm_add_u32(_80, 1);
    }
    _43 = 2;
    loop {
        _81 = _43;
        _82 = _35;
        if !(_81 <= _82) {
            break;
        }
        _44 = 2;
        loop {
            _83 = _44;
            _84 = _36;
            if !(_83 <= _84) {
                break;
            }
            _45 = 2;
            loop {
                _85 = _45;
                _86 = _37;
                if !(_85 <= _86) {
                    break;
                }
                _87 = _43;
                _88 = _44;
                _89 = _45;
                _46 = llvm_mul_u32(llvm_mul_u32(_87, _88), _89);
                _90 = _46;
                if !(_90 > 64 as core::ffi::c_uint) {
                    _91 = _38;
                    _92 = _40;
                    _47 = &raw mut *((*(&raw mut (*(_91
                        as *mut l_struct_struct_OC_block_size_descriptor))
                        .field14
                        as *mut crate::l_array_87_struct_AC_l_struct_struct_OC_decimation_info))
                        .array)
                        .as_mut_ptr()
                        .offset(_92 as uint64_t as int64_t as isize)
                        as *mut l_struct_struct_OC_decimation_info
                        as *mut core::ffi::c_void;
                    _93 = _40;
                    _94 = _45;
                    _95 = _44;
                    _96 = _43;
                    *(&raw mut *(_39.array).as_mut_ptr().offset((llvm_add_u32
                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        (llvm_add_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                _94, 64,
                            ),
                            (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                _95, 8,
                            ),
                        ),
                        _96,
                    ) as uint64_t
                        as int64_t
                        as isize) as *mut uint32_t) = _93;
                    _97 = _35;
                    _98 = _36;
                    _99 = _37;
                    _100 = _43;
                    _101 = _44;
                    _102 = _45;
                    _103 = _47;
                    _104 = _41;
                    _ZL23init_decimation_info_3djjjjjjR15decimation_infoR23dt_init_working_buffers(
                        _97, _98, _99, _100, _101, _102, _103, _104,
                    );
                    _48 = -(1 as core::ffi::c_int) as uint32_t;
                    _49 = -(1 as core::ffi::c_int) as uint32_t;
                    _50 = 0;
                    loop {
                        _105 = _50;
                        if !(_105 < 12 as core::ffi::c_uint) {
                            break;
                        }
                        _106 = _46;
                        _107 = _50;
                        _108 = _Z25get_ise_sequence_bitcountj12quant_method(_106, _107);
                        _51 = _108;
                        _109 = _51;
                        if _109 >= 24 as core::ffi::c_uint {
                            _110 = _51;
                            if _110 <= 96 as core::ffi::c_uint {
                                _111 = _50;
                                _48 = _111;
                            }
                        }
                        _112 = _46;
                        _113 = _50;
                        _114 = _Z25get_ise_sequence_bitcountj12quant_method(
                            llvm_mul_u32(2, _112),
                            _113,
                        );
                        _52 = _114;
                        _115 = _52;
                        if _115 >= 24 as core::ffi::c_uint {
                            _116 = _52;
                            if _116 <= 96 as core::ffi::c_uint {
                                _117 = _50;
                                _49 = _117;
                            }
                        }
                        _118 = _50;
                        _50 = llvm_add_u32(_118, 1);
                    }
                    _119 = _46;
                    if llvm_mul_u32(2, _119) > 64 as core::ffi::c_uint {
                        _49 = -(1 as core::ffi::c_int) as uint32_t;
                    }
                    _120 = _48;
                    _121 = _38;
                    _122 = _40;
                    *(&raw mut (*(&raw mut *((*(&raw mut (*(_121
                        as *mut l_struct_struct_OC_block_size_descriptor))
                        .field13
                        as *mut crate::l_array_87_struct_AC_l_struct_struct_OC_decimation_mode))
                        .array)
                        .as_mut_ptr()
                        .offset(_122 as uint64_t as int64_t as isize)
                        as *mut l_struct_struct_OC_decimation_mode))
                        .field0 as *mut uint8_t) = _120 as uint8_t;
                    _123 = _49;
                    _124 = _38;
                    _125 = _40;
                    *(&raw mut (*(&raw mut *((*(&raw mut (*(_124
                        as *mut l_struct_struct_OC_block_size_descriptor))
                        .field13
                        as *mut crate::l_array_87_struct_AC_l_struct_struct_OC_decimation_mode))
                        .array)
                        .as_mut_ptr()
                        .offset(_125 as uint64_t as int64_t as isize)
                        as *mut l_struct_struct_OC_decimation_mode))
                        .field1 as *mut uint8_t) = _123 as uint8_t;
                    _126 = _48;
                    _127 = (_126 == 4294967295 as core::ffi::c_uint) as core::ffi::c_int as bool_0
                        as uint64_t;
                    _128 = _38;
                    _129 = _40;
                    *(&raw mut (*(&raw mut *((*(&raw mut (*(_128
                        as *mut l_struct_struct_OC_block_size_descriptor))
                        .field13
                        as *mut crate::l_array_87_struct_AC_l_struct_struct_OC_decimation_mode))
                        .array)
                        .as_mut_ptr()
                        .offset(_129 as uint64_t as int64_t as isize)
                        as *mut l_struct_struct_OC_decimation_mode))
                        .field2 as *mut uint16_t) = llvm_select_u32(
                        (_126 == 4294967295 as core::ffi::c_uint) as core::ffi::c_int as bool_0,
                        0,
                        65535,
                    ) as uint16_t;
                    _130 = _49;
                    _131 = (_130 == 4294967295 as core::ffi::c_uint) as core::ffi::c_int as bool_0
                        as uint64_t;
                    _132 = _38;
                    _133 = _40;
                    *(&raw mut (*(&raw mut *((*(&raw mut (*(_132
                        as *mut l_struct_struct_OC_block_size_descriptor))
                        .field13
                        as *mut crate::l_array_87_struct_AC_l_struct_struct_OC_decimation_mode))
                        .array)
                        .as_mut_ptr()
                        .offset(_133 as uint64_t as int64_t as isize)
                        as *mut l_struct_struct_OC_decimation_mode))
                        .field3 as *mut uint16_t) = llvm_select_u32(
                        (_130 == 4294967295 as core::ffi::c_uint) as core::ffi::c_int as bool_0,
                        0,
                        65535,
                    ) as uint16_t;
                    _134 = _40;
                    _40 = llvm_add_u32(_134, 1);
                }
                _135 = _45;
                _45 = llvm_add_u32(_135, 1);
            }
            _136 = _44;
            _44 = llvm_add_u32(_136, 1);
        }
        _137 = _43;
        _43 = llvm_add_u32(_137, 1);
    }
    _138 = _40;
    _53 = _138;
    loop {
        _139 = _53;
        if !(_139 < 87 as core::ffi::c_uint) {
            break;
        }
        _140 = _38;
        _141 = _53;
        *(&raw mut (*(&raw mut *((*(&raw mut (*(_140
            as *mut l_struct_struct_OC_block_size_descriptor))
            .field13
            as *mut crate::l_array_87_struct_AC_l_struct_struct_OC_decimation_mode))
            .array)
            .as_mut_ptr()
            .offset(_141 as uint64_t as int64_t as isize)
            as *mut l_struct_struct_OC_decimation_mode))
            .field0 as *mut uint8_t) = -(1 as core::ffi::c_int) as uint8_t;
        _142 = _38;
        _143 = _53;
        *(&raw mut (*(&raw mut *((*(&raw mut (*(_142
            as *mut l_struct_struct_OC_block_size_descriptor))
            .field13
            as *mut crate::l_array_87_struct_AC_l_struct_struct_OC_decimation_mode))
            .array)
            .as_mut_ptr()
            .offset(_143 as uint64_t as int64_t as isize)
            as *mut l_struct_struct_OC_decimation_mode))
            .field1 as *mut uint8_t) = -(1 as core::ffi::c_int) as uint8_t;
        _144 = _38;
        _145 = _53;
        *(&raw mut (*(&raw mut *((*(&raw mut (*(_144
            as *mut l_struct_struct_OC_block_size_descriptor))
            .field13
            as *mut crate::l_array_87_struct_AC_l_struct_struct_OC_decimation_mode))
            .array)
            .as_mut_ptr()
            .offset(_145 as uint64_t as int64_t as isize)
            as *mut l_struct_struct_OC_decimation_mode))
            .field2 as *mut uint16_t) = 0;
        _146 = _38;
        _147 = _53;
        *(&raw mut (*(&raw mut *((*(&raw mut (*(_146
            as *mut l_struct_struct_OC_block_size_descriptor))
            .field13
            as *mut crate::l_array_87_struct_AC_l_struct_struct_OC_decimation_mode))
            .array)
            .as_mut_ptr()
            .offset(_147 as uint64_t as int64_t as isize)
            as *mut l_struct_struct_OC_decimation_mode))
            .field3 as *mut uint16_t) = 0;
        _148 = _53;
        _53 = llvm_add_u32(_148, 1);
    }
    _149 = _38;
    *(&raw mut (*(_149 as *mut l_struct_struct_OC_block_size_descriptor)).field4
        as *mut uint32_t) = 0;
    _150 = _40;
    _151 = _38;
    *(&raw mut (*(_151 as *mut l_struct_struct_OC_block_size_descriptor)).field5
        as *mut uint32_t) = _150;
    _152 = _40;
    _153 = _38;
    *(&raw mut (*(_153 as *mut l_struct_struct_OC_block_size_descriptor)).field6
        as *mut uint32_t) = _152;
    _54 = 0;
    loop {
        _154 = _54;
        if !(_154 < 2048 as core::ffi::c_uint) {
            break;
        }
        _155 = _38;
        _156 = _54;
        *(&raw mut *((*(&raw mut (*(_155 as *mut l_struct_struct_OC_block_size_descriptor)).field15
            as *mut crate::l_array_2048_uint16_t))
            .array)
            .as_mut_ptr()
            .offset(_156 as uint64_t as int64_t as isize) as *mut uint16_t) =
            -(1 as core::ffi::c_int) as uint16_t;
        _157 = _54;
        _54 = llvm_add_u32(_157, 1);
    }
    _55 = 0;
    _158 = memset(
        &mut _56 as *mut crate::l_array_2_uint32_t as *mut core::ffi::c_void,
        0,
        8,
    );
    _57 = 0;
    loop {
        _159 = _57;
        if !(_159 < 2 as core::ffi::c_uint) {
            break;
        }
        _58 = 0;
        loop {
            _160 = _58;
            if !(_160 < 2048 as core::ffi::c_uint) {
                break;
            }
            _161 = _38;
            _162 = _58;
            _163 = *(&raw mut *((*(&raw mut (*(_161
                as *mut l_struct_struct_OC_block_size_descriptor))
                .field15 as *mut crate::l_array_2048_uint16_t))
                .array)
                .as_mut_ptr()
                .offset(_162 as uint64_t as int64_t as isize)
                as *mut uint16_t);
            if !(_163 as uint32_t != 65535 as core::ffi::c_uint) {
                _164 = _58;
                _165 = _ZL20decode_block_mode_3djRjS_S_RbS_S_(
                    _164,
                    &mut _59 as *mut uint32_t as *mut core::ffi::c_void,
                    &mut _60 as *mut uint32_t as *mut core::ffi::c_void,
                    &mut _61 as *mut uint32_t as *mut core::ffi::c_void,
                    &mut _62 as *mut uint8_t as *mut core::ffi::c_void,
                    &mut _63 as *mut uint32_t as *mut core::ffi::c_void,
                    &mut _64 as *mut uint32_t as *mut core::ffi::c_void,
                );
                _65 = _165;
                _166 = _65;
                if _166 as core::ffi::c_uint & 1 as core::ffi::c_uint != 0 {
                    _167 = _59;
                    _168 = _35;
                    if !(_167 > _168) {
                        _169 = _60;
                        _170 = _36;
                        if !(_169 > _170) {
                            _171 = _61;
                            _172 = _37;
                            if !(_171 > _172) {
                                _173 = _57;
                                if _173 == 0 as core::ffi::c_uint {
                                    _174 = _62;
                                    if _174 as core::ffi::c_uint & 1 as core::ffi::c_uint != 0 {
                                        current_block = 3444006581145821682;
                                    } else {
                                        current_block = 3545748811593777663;
                                    }
                                } else {
                                    current_block = 3545748811593777663;
                                }
                                match current_block {
                                    3444006581145821682 => {}
                                    _ => {
                                        _175 = _57;
                                        if _175 == 1 as core::ffi::c_uint {
                                            _176 = _62;
                                            if _176 as core::ffi::c_uint & 1 as core::ffi::c_uint
                                                != 0
                                            {
                                                current_block = 12288404210271891859;
                                            } else {
                                                current_block = 3444006581145821682;
                                            }
                                        } else {
                                            current_block = 12288404210271891859;
                                        }
                                        match current_block {
                                            3444006581145821682 => {}
                                            _ => {
                                                _177 = _62;
                                                if _177 as core::ffi::c_uint
                                                    & 1 as core::ffi::c_uint
                                                    != 0
                                                {
                                                    _178 = _64;
                                                    if llvm_sub_u32(109, _178)
                                                        <= 0 as core::ffi::c_uint
                                                    {
                                                        current_block = 3444006581145821682;
                                                    } else {
                                                        current_block = 2521726344681605858;
                                                    }
                                                } else {
                                                    _179 = _64;
                                                    if llvm_sub_u32(111, _179)
                                                        <= 0 as core::ffi::c_uint
                                                    {
                                                        current_block = 3444006581145821682;
                                                    } else {
                                                        current_block = 2521726344681605858;
                                                    }
                                                }
                                                match current_block {
                                                    3444006581145821682 => {}
                                                    _ => {
                                                        _180 = _61;
                                                        _181 = _60;
                                                        _182 = _59;
                                                        _183 = *(&raw mut *(_39.array)
                                                            .as_mut_ptr()
                                                            .offset(
                                                                (llvm_add_u32
                                                                    as unsafe extern "C" fn(
                                                                        uint32_t,
                                                                        uint32_t,
                                                                    ) -> uint32_t)(
                                                                    (llvm_add_u32
                                                                        as unsafe extern "C" fn(
                                                                            uint32_t,
                                                                            uint32_t,
                                                                        ) -> uint32_t)(
                                                                        (llvm_mul_u32
                                                                            as unsafe extern "C" fn(
                                                                                uint32_t,
                                                                                uint32_t,
                                                                            ) -> uint32_t)(_180, 64),
                                                                        (llvm_mul_u32
                                                                            as unsafe extern "C" fn(
                                                                                uint32_t,
                                                                                uint32_t,
                                                                            ) -> uint32_t)(_181, 8),
                                                                    ),
                                                                    _182,
                                                                ) as uint64_t as int64_t as isize,
                                                            ) as *mut uint32_t);
                                                        _66 = _183;
                                                        _184 = _66;
                                                        _185 = _38;
                                                        _186 = _55;
                                                        *(&raw mut (*(&raw mut *((*(&raw mut (*(_185
                                                            as *mut l_struct_struct_OC_block_size_descriptor))
                                                            .field16
                                                            as *mut crate::l_array_2048_struct_AC_l_struct_struct_OC_block_mode))
                                                            .array)
                                                            .as_mut_ptr()
                                                            .offset(_186 as uint64_t as int64_t as isize)
                                                            as *mut l_struct_struct_OC_block_mode))
                                                            .field1 as *mut uint8_t) = _184 as uint8_t;
                                                        _187 = _63;
                                                        _188 = _38;
                                                        _189 = _55;
                                                        *(&raw mut (*(&raw mut *((*(&raw mut (*(_188
                                                            as *mut l_struct_struct_OC_block_size_descriptor))
                                                            .field16
                                                            as *mut crate::l_array_2048_struct_AC_l_struct_struct_OC_block_mode))
                                                            .array)
                                                            .as_mut_ptr()
                                                            .offset(_189 as uint64_t as int64_t as isize)
                                                            as *mut l_struct_struct_OC_block_mode))
                                                            .field2 as *mut uint8_t) = _187 as uint8_t;
                                                        _190 = _64;
                                                        _191 = _38;
                                                        _192 = _55;
                                                        *(&raw mut (*(&raw mut *((*(&raw mut (*(_191
                                                            as *mut l_struct_struct_OC_block_size_descriptor))
                                                            .field16
                                                            as *mut crate::l_array_2048_struct_AC_l_struct_struct_OC_block_mode))
                                                            .array)
                                                            .as_mut_ptr()
                                                            .offset(_192 as uint64_t as int64_t as isize)
                                                            as *mut l_struct_struct_OC_block_mode))
                                                            .field3 as *mut uint8_t) = _190 as uint8_t;
                                                        _193 = _62;
                                                        _194 = _38;
                                                        _195 = _55;
                                                        _196 = &raw mut (*(&raw mut *((*(&raw mut (*(_194
                                                            as *mut l_struct_struct_OC_block_size_descriptor))
                                                            .field16
                                                            as *mut crate::l_array_2048_struct_AC_l_struct_struct_OC_block_mode))
                                                            .array)
                                                            .as_mut_ptr()
                                                            .offset(_195 as uint64_t as int64_t as isize)
                                                            as *mut l_struct_struct_OC_block_mode))
                                                            .field4 as *mut uint8_t as *mut core::ffi::c_void;
                                                        _197 = *(_196 as *mut uint8_t);
                                                        *(_196 as *mut uint8_t) = llvm_or_u8(
                                                            llvm_and_u8(
                                                                _197,
                                                                -(2 as core::ffi::c_int) as uint8_t,
                                                            ),
                                                            llvm_and_u8(
                                                                (_193 as core::ffi::c_uint
                                                                    & 1 as core::ffi::c_uint)
                                                                    as bool_0,
                                                                1,
                                                            ),
                                                        );
                                                        _198 = _58;
                                                        _199 = _38;
                                                        _200 = _55;
                                                        *(&raw mut (*(&raw mut *((*(&raw mut (*(_199
                                                            as *mut l_struct_struct_OC_block_size_descriptor))
                                                            .field16
                                                            as *mut crate::l_array_2048_struct_AC_l_struct_struct_OC_block_mode))
                                                            .array)
                                                            .as_mut_ptr()
                                                            .offset(_200 as uint64_t as int64_t as isize)
                                                            as *mut l_struct_struct_OC_block_mode))
                                                            .field0 as *mut uint16_t) = _198 as uint16_t;
                                                        _201 = _55;
                                                        _202 = _38;
                                                        _203 = _58;
                                                        *(&raw mut *((*(&raw mut (*(_202
                                                            as *mut l_struct_struct_OC_block_size_descriptor))
                                                            .field15 as *mut crate::l_array_2048_uint16_t))
                                                            .array)
                                                            .as_mut_ptr()
                                                            .offset(_203 as uint64_t as int64_t as isize)
                                                            as *mut uint16_t) = _201 as uint16_t;
                                                        _204 = _57;
                                                        _205 = &raw mut *(_56.array)
                                                            .as_mut_ptr()
                                                            .offset(
                                                                _204 as uint64_t as int64_t
                                                                    as isize,
                                                            )
                                                            as *mut uint32_t
                                                            as *mut core::ffi::c_void;
                                                        _206 = *(_205 as *mut uint32_t);
                                                        *(_205 as *mut uint32_t) =
                                                            llvm_add_u32(_206, 1);
                                                        _207 = _55;
                                                        _55 = llvm_add_u32(_207, 1);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _208 = _58;
            _58 = llvm_add_u32(_208, 1);
        }
        _209 = _57;
        _57 = llvm_add_u32(_209, 1);
    }
    _210 = _38;
    *(&raw mut (*(_210 as *mut l_struct_struct_OC_block_size_descriptor)).field7
        as *mut uint32_t) = 0;
    _211 = *(&raw mut *(_56.array)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as int64_t as isize) as *mut uint32_t);
    _212 = _38;
    *(&raw mut (*(_212 as *mut l_struct_struct_OC_block_size_descriptor)).field8
        as *mut uint32_t) = _211;
    _213 = *(&raw mut *(_56.array)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as int64_t as isize) as *mut uint32_t);
    _214 = *(&raw mut *(_56.array)
        .as_mut_ptr()
        .offset(1 as core::ffi::c_int as int64_t as isize) as *mut uint32_t);
    _215 = _38;
    *(&raw mut (*(_215 as *mut l_struct_struct_OC_block_size_descriptor)).field9
        as *mut uint32_t) = llvm_add_u32(_213, _214);
    _216 = *(&raw mut *(_56.array)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as int64_t as isize) as *mut uint32_t);
    _217 = *(&raw mut *(_56.array)
        .as_mut_ptr()
        .offset(1 as core::ffi::c_int as int64_t as isize) as *mut uint32_t);
    _218 = _38;
    *(&raw mut (*(_218 as *mut l_struct_struct_OC_block_size_descriptor)).field10
        as *mut uint32_t) = llvm_add_u32(_216, _217);
    _219 = _38;
    _ZL20assign_kmeans_texelsR21block_size_descriptor(_219);
    _220 = _41;
    if !_220.is_null() {
        _ZdlPvm(_220, 29656);
    }
}
#[inline(never)]
unsafe extern "C" fn _ZL34construct_block_size_descriptor_2djjbfR21block_size_descriptor(
    mut _288: uint32_t,
    mut _289: uint32_t,
    mut _290: bool_0,
    mut _291: core::ffi::c_float,
    mut _292: *mut core::ffi::c_void,
) {
    let mut current_block: u64;
    let mut _293: uint32_t = 0;
    let mut _294: uint32_t = 0;
    let mut _295: uint8_t = 0;
    let mut _296: core::ffi::c_float = 0.;
    let mut _297: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _298: crate::l_array_204_uint32_t = crate::l_array_204_uint32_t { array: [0; 204] };
    let mut _299: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _300: uint32_t = 0;
    let mut _301: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _302: core::ffi::c_float = 0.;
    let mut _303: uint32_t = 0;
    let mut _304: uint32_t = 0;
    let mut _305: crate::l_array_4_uint32_t = crate::l_array_4_uint32_t { array: [0; 4] };
    let mut _306: crate::l_array_4_uint32_t = crate::l_array_4_uint32_t { array: [0; 4] };
    let mut _307: uint32_t = 0;
    let mut _308: uint32_t = 0;
    let mut _309: uint32_t = 0;
    let mut _310: uint32_t = 0;
    let mut _311: uint32_t = 0;
    let mut _312: uint32_t = 0;
    let mut _313: uint8_t = 0;
    let mut _314: uint32_t = 0;
    let mut _315: uint32_t = 0;
    let mut _316: uint8_t = 0;
    let mut _317: uint8_t = 0;
    let mut _318: uint32_t = 0;
    let mut _319: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _320: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _321: uint32_t = 0;
    let mut _322: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _323: uint32_t = 0;
    let mut _324: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _325: uint32_t = 0;
    let mut _326: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _327: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _328: uint32_t = 0;
    let mut _329: uint32_t = 0;
    let mut _330: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _331: uint32_t = 0;
    let mut _332: uint32_t = 0;
    let mut _333: uint32_t = 0;
    let mut _334: uint32_t = 0;
    let mut _335: uint32_t = 0;
    let mut _336: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _337: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _338: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _339: uint32_t = 0;
    let mut _340: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _341: uint32_t = 0;
    let mut _342: uint32_t = 0;
    let mut _343: uint8_t = 0;
    let mut _344: bool_0 = 0;
    let mut _345: uint64_t = 0;
    let mut _346: uint32_t = 0;
    let mut _347: uint32_t = 0;
    let mut _348: uint32_t = 0;
    let mut _349: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _350: uint32_t = 0;
    let mut _351: uint16_t = 0;
    let mut _352: uint32_t = 0;
    let mut _353: bool_0 = 0;
    let mut _354: uint8_t = 0;
    let mut _355: uint32_t = 0;
    let mut _356: uint32_t = 0;
    let mut _357: uint32_t = 0;
    let mut _358: uint32_t = 0;
    let mut _359: uint32_t = 0;
    let mut _360: uint8_t = 0;
    let mut _361: uint32_t = 0;
    let mut _362: uint8_t = 0;
    let mut _363: uint8_t = 0;
    let mut _364: uint32_t = 0;
    let mut _365: uint32_t = 0;
    let mut _366: uint32_t = 0;
    let mut _367: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _368: uint32_t = 0;
    let mut _369: core::ffi::c_float = 0.;
    let mut _370: core::ffi::c_float = 0.;
    let mut _371: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _372: uint32_t = 0;
    let mut _373: core::ffi::c_float = 0.;
    let mut _374: core::ffi::c_float = 0.;
    let mut _375: uint32_t = 0;
    let mut _376: uint8_t = 0;
    let mut _377: uint32_t = 0;
    let mut _378: uint32_t = 0;
    let mut _379: uint32_t = 0;
    let mut _380: uint32_t = 0;
    let mut _381: uint32_t = 0;
    let mut _382: uint32_t = 0;
    let mut _383: uint32_t = 0;
    let mut _384: uint32_t = 0;
    let mut _385: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _386: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _387: uint32_t = 0;
    let mut _388: uint32_t = 0;
    let mut _389: uint32_t = 0;
    let mut _390: uint32_t = 0;
    let mut _391: uint32_t = 0;
    let mut _392: uint32_t = 0;
    let mut _393: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _394: uint32_t = 0;
    let mut _395: uint32_t = 0;
    let mut _396: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _397: uint32_t = 0;
    let mut _398: uint32_t = 0;
    let mut _399: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _400: uint32_t = 0;
    let mut _401: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _402: uint8_t = 0;
    let mut _403: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _404: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _405: uint8_t = 0;
    let mut _406: uint32_t = 0;
    let mut _407: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _408: uint32_t = 0;
    let mut _409: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _410: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _411: uint32_t = 0;
    let mut _412: uint8_t = 0;
    let mut _413: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _414: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _415: uint32_t = 0;
    let mut _416: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _417: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _418: uint32_t = 0;
    let mut _419: uint32_t = 0;
    let mut _420: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _421: uint32_t = 0;
    let mut _422: uint32_t = 0;
    let mut _423: uint32_t = 0;
    let mut _424: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _425: uint32_t = 0;
    let mut _426: uint32_t = 0;
    let mut _427: uint32_t = 0;
    let mut _428: uint32_t = 0;
    let mut _429: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _430: uint32_t = 0;
    let mut _431: uint32_t = 0;
    let mut _432: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _433: uint32_t = 0;
    let mut _434: uint32_t = 0;
    let mut _435: uint32_t = 0;
    let mut _436: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _437: uint32_t = 0;
    let mut _438: uint32_t = 0;
    let mut _439: uint32_t = 0;
    let mut _440: uint32_t = 0;
    let mut _441: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _442: uint32_t = 0;
    let mut _443: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _444: uint32_t = 0;
    let mut _445: uint32_t = 0;
    let mut _446: uint32_t = 0;
    let mut _447: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _448: uint32_t = 0;
    let mut _449: uint32_t = 0;
    let mut _450: uint32_t = 0;
    let mut _451: uint32_t = 0;
    let mut _452: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _453: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _454: uint32_t = 0;
    let mut _455: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _456: uint32_t = 0;
    let mut _457: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _458: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _459: uint32_t = 0;
    let mut _460: uint32_t = 0;
    let mut _461: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _462: uint32_t = 0;
    let mut _463: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _464: uint32_t = 0;
    let mut _465: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _466: uint32_t = 0;
    let mut _467: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _468: uint32_t = 0;
    let mut _469: uint32_t = 0;
    let mut _470: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _471: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    _293 = _288;
    _294 = _289;
    _295 = _290;
    _296 = _291;
    _297 = _292;
    _322 = _Znwm(29656);
    _299 = _322;
    _323 = _293;
    _324 = _297;
    *(&raw mut (*(_324 as *mut l_struct_struct_OC_block_size_descriptor)).field0 as *mut uint8_t) =
        _323 as uint8_t;
    _325 = _294;
    _326 = _297;
    *(&raw mut (*(_326 as *mut l_struct_struct_OC_block_size_descriptor)).field1 as *mut uint8_t) =
        _325 as uint8_t;
    _327 = _297;
    *(&raw mut (*(_327 as *mut l_struct_struct_OC_block_size_descriptor)).field2 as *mut uint8_t) =
        1;
    _328 = _293;
    _329 = _294;
    _330 = _297;
    *(&raw mut (*(_330 as *mut l_struct_struct_OC_block_size_descriptor)).field3 as *mut uint8_t) =
        llvm_mul_u32(_328, _329) as uint8_t;
    _300 = 0;
    loop {
        _331 = _300;
        if !(_331 < 204 as core::ffi::c_uint) {
            break;
        }
        _332 = _300;
        *(&raw mut *(_298.array)
            .as_mut_ptr()
            .offset(_332 as uint64_t as int64_t as isize) as *mut uint32_t) =
            -(1 as core::ffi::c_int) as uint32_t;
        _333 = _300;
        _300 = llvm_add_u32(_333, 1);
    }
    _334 = _293;
    _335 = _294;
    _336 = _Z23get_2d_percentile_tablejj(_334, _335);
    _301 = _336;
    _302 = 0 as core::ffi::c_int as core::ffi::c_float;
    _303 = 0;
    _304 = 0;
    _337 = memset(
        &mut _305 as *mut crate::l_array_4_uint32_t as *mut core::ffi::c_void,
        0,
        16,
    );
    _338 = memset(
        &mut _306 as *mut crate::l_array_4_uint32_t as *mut core::ffi::c_void,
        0,
        16,
    );
    _307 = 0;
    loop {
        _339 = _307;
        if !(_339 < 2048 as core::ffi::c_uint) {
            break;
        }
        _340 = _297;
        _341 = _307;
        *(&raw mut *((*(&raw mut (*(_340 as *mut l_struct_struct_OC_block_size_descriptor)).field15
            as *mut crate::l_array_2048_uint16_t))
            .array)
            .as_mut_ptr()
            .offset(_341 as uint64_t as int64_t as isize) as *mut uint16_t) =
            -(1 as core::ffi::c_int) as uint16_t;
        _342 = _307;
        _307 = llvm_add_u32(_342, 1);
    }
    _343 = _295;
    _344 = (_343 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0;
    _345 = _344 as uint64_t;
    _308 = llvm_select_u32(_344, 3, 4);
    _309 = 0;
    loop {
        _346 = _309;
        _347 = _308;
        if !(_346 < _347) {
            break;
        }
        _310 = 0;
        loop {
            _348 = _310;
            if !(_348 < 2048 as core::ffi::c_uint) {
                break;
            }
            _349 = _297;
            _350 = _310;
            _351 = *(&raw mut *((*(&raw mut (*(_349
                as *mut l_struct_struct_OC_block_size_descriptor))
                .field15 as *mut crate::l_array_2048_uint16_t))
                .array)
                .as_mut_ptr()
                .offset(_350 as uint64_t as int64_t as isize)
                as *mut uint16_t);
            if !(_351 as uint32_t != 65535 as core::ffi::c_uint) {
                _352 = _310;
                _353 = _ZL20decode_block_mode_2djRjS_RbS_S_(
                    _352,
                    &mut _311 as *mut uint32_t as *mut core::ffi::c_void,
                    &mut _312 as *mut uint32_t as *mut core::ffi::c_void,
                    &mut _313 as *mut uint8_t as *mut core::ffi::c_void,
                    &mut _314 as *mut uint32_t as *mut core::ffi::c_void,
                    &mut _315 as *mut uint32_t as *mut core::ffi::c_void,
                );
                _316 = _353;
                _354 = _316;
                if _354 as core::ffi::c_uint & 1 as core::ffi::c_uint != 0 {
                    _355 = _311;
                    _356 = _293;
                    if !(_355 > _356) {
                        _357 = _312;
                        _358 = _294;
                        if !(_357 > _358) {
                            _359 = _309;
                            if _359 <= 1 as core::ffi::c_uint {
                                _360 = _313;
                                if _360 as core::ffi::c_uint & 1 as core::ffi::c_uint != 0 {
                                    current_block = 1550379751883021490;
                                } else {
                                    current_block = 17304209765350733790;
                                }
                            } else {
                                current_block = 17304209765350733790;
                            }
                            match current_block {
                                1550379751883021490 => {}
                                _ => {
                                    _361 = _309;
                                    if _361 == 2 as core::ffi::c_uint {
                                        _362 = _313;
                                        if _362 as core::ffi::c_uint & 1 as core::ffi::c_uint != 0 {
                                            current_block = 815396569228717678;
                                        } else {
                                            current_block = 1550379751883021490;
                                        }
                                    } else {
                                        current_block = 815396569228717678;
                                    }
                                    match current_block {
                                        1550379751883021490 => {}
                                        _ => {
                                            _363 = _313;
                                            if _363 as core::ffi::c_uint & 1 as core::ffi::c_uint
                                                != 0
                                            {
                                                _364 = _315;
                                                if llvm_sub_u32(109, _364) <= 0 as core::ffi::c_uint
                                                {
                                                    current_block = 1550379751883021490;
                                                } else {
                                                    current_block = 1412074291996928727;
                                                }
                                            } else {
                                                _365 = _315;
                                                if llvm_sub_u32(111, _365) <= 0 as core::ffi::c_uint
                                                {
                                                    current_block = 1550379751883021490;
                                                } else {
                                                    current_block = 1412074291996928727;
                                                }
                                            }
                                            match current_block {
                                                1550379751883021490 => {}
                                                _ => {
                                                    _317 = 0;
                                                    _366 = _309;
                                                    if _366 == 0 as core::ffi::c_uint {
                                                        _367 = _301;
                                                        _368 = _310;
                                                        _369 = *(&raw mut *(_367
                                                            as *mut core::ffi::c_float)
                                                            .offset(
                                                                _368 as uint64_t as int64_t
                                                                    as isize,
                                                            )
                                                            as *mut core::ffi::c_float);
                                                        _370 = _302;
                                                        _317 = llvm_fcmp_ole(
                                                            _369 as core::ffi::c_double,
                                                            _370 as core::ffi::c_double,
                                                        )
                                                            as bool_0;
                                                    } else {
                                                        _371 = _301;
                                                        _372 = _310;
                                                        _373 = *(&raw mut *(_371
                                                            as *mut core::ffi::c_float)
                                                            .offset(
                                                                _372 as uint64_t as int64_t
                                                                    as isize,
                                                            )
                                                            as *mut core::ffi::c_float);
                                                        _374 = _296;
                                                        _317 = llvm_fcmp_ole(
                                                            _373 as core::ffi::c_double,
                                                            _374 as core::ffi::c_double,
                                                        )
                                                            as bool_0;
                                                    }
                                                    _375 = _309;
                                                    if _375 != 3 as core::ffi::c_uint {
                                                        _376 = _317;
                                                        if _376 as core::ffi::c_uint
                                                            & 1 as core::ffi::c_uint
                                                            != 0
                                                        {
                                                            current_block = 1200329877519293794;
                                                        } else {
                                                            current_block = 1550379751883021490;
                                                        }
                                                    } else {
                                                        current_block = 1200329877519293794;
                                                    }
                                                    match current_block {
                                                        1550379751883021490 => {}
                                                        _ => {
                                                            _377 = _312;
                                                            _378 = _311;
                                                            _379 = *(&raw mut *(_298.array)
                                                                .as_mut_ptr()
                                                                .offset(
                                                                    (llvm_add_u32
                                                                        as unsafe extern "C" fn(
                                                                            uint32_t,
                                                                            uint32_t,
                                                                        ) -> uint32_t)(
                                                                        (llvm_mul_u32
                                                                            as unsafe extern "C" fn(
                                                                                uint32_t,
                                                                                uint32_t,
                                                                            ) -> uint32_t)(_377, 16),
                                                                        _378,
                                                                    ) as uint64_t as int64_t as isize,
                                                                ) as *mut uint32_t);
                                                            _318 = _379;
                                                            _380 = _318;
                                                            if (_380 as int32_t)
                                                                < 0 as core::ffi::c_uint as int32_t
                                                            {
                                                                _381 = _293;
                                                                _382 = _294;
                                                                _383 = _311;
                                                                _384 = _312;
                                                                _385 = _297;
                                                                _386 = _299;
                                                                _387 = _304;
                                                                _ZL21construct_dt_entry_2djjjjR21block_size_descriptorR23dt_init_working_buffersj(
                                                                    _381,
                                                                    _382,
                                                                    _383,
                                                                    _384,
                                                                    _385,
                                                                    _386,
                                                                    _387,
                                                                );
                                                                _388 = _304;
                                                                _389 = _312;
                                                                _390 = _311;
                                                                *(&raw mut *(_298.array)
                                                                    .as_mut_ptr()
                                                                    .offset(
                                                                        (llvm_add_u32
                                                                            as unsafe extern "C" fn(
                                                                                uint32_t,
                                                                                uint32_t,
                                                                            ) -> uint32_t)(
                                                                            (llvm_mul_u32
                                                                                as unsafe extern "C" fn(
                                                                                    uint32_t,
                                                                                    uint32_t,
                                                                                ) -> uint32_t)(_389, 16),
                                                                            _390,
                                                                        ) as uint64_t as int64_t as isize,
                                                                    ) as *mut uint32_t) = _388;
                                                                _391 = _304;
                                                                _318 = _391;
                                                                _392 = _309;
                                                                _393 = &raw mut *(_306.array)
                                                                    .as_mut_ptr()
                                                                    .offset(
                                                                        _392 as uint64_t as int64_t
                                                                            as isize,
                                                                    )
                                                                    as *mut uint32_t
                                                                    as *mut core::ffi::c_void;
                                                                _394 = *(_393 as *mut uint32_t);
                                                                *(_393 as *mut uint32_t) =
                                                                    llvm_add_u32(_394, 1);
                                                                _395 = _304;
                                                                _304 = llvm_add_u32(_395, 1);
                                                            }
                                                            _396 = _297;
                                                            _397 = _303;
                                                            _319 = &raw mut *((*(&raw mut (*(_396
                                                                as *mut l_struct_struct_OC_block_size_descriptor))
                                                                .field16
                                                                as *mut crate::l_array_2048_struct_AC_l_struct_struct_OC_block_mode))
                                                                .array)
                                                                .as_mut_ptr()
                                                                .offset(_397 as uint64_t as int64_t as isize)
                                                                as *mut l_struct_struct_OC_block_mode as *mut core::ffi::c_void;
                                                            _398 = _318;
                                                            _399 = _319;
                                                            *(&raw mut (*(_399 as *mut l_struct_struct_OC_block_mode))
                                                                .field1 as *mut uint8_t) = _398 as uint8_t;
                                                            _400 = _314;
                                                            _401 = _319;
                                                            *(&raw mut (*(_401 as *mut l_struct_struct_OC_block_mode))
                                                                .field2 as *mut uint8_t) = _400 as uint8_t;
                                                            _402 = _313;
                                                            _403 = _319;
                                                            _404 = &raw mut (*(_403 as *mut l_struct_struct_OC_block_mode))
                                                                .field4 as *mut uint8_t as *mut core::ffi::c_void;
                                                            _405 = *(_404 as *mut uint8_t);
                                                            *(_404 as *mut uint8_t) = llvm_or_u8(
                                                                llvm_and_u8(
                                                                    _405,
                                                                    -(2 as core::ffi::c_int)
                                                                        as uint8_t,
                                                                ),
                                                                llvm_and_u8(
                                                                    (_402 as core::ffi::c_uint
                                                                        & 1 as core::ffi::c_uint)
                                                                        as bool_0,
                                                                    1,
                                                                ),
                                                            );
                                                            _406 = _315;
                                                            _407 = _319;
                                                            *(&raw mut (*(_407 as *mut l_struct_struct_OC_block_mode))
                                                                .field3 as *mut uint8_t) = _406 as uint8_t;
                                                            _408 = _310;
                                                            _409 = _319;
                                                            *(&raw mut (*(_409 as *mut l_struct_struct_OC_block_mode))
                                                                .field0 as *mut uint16_t) = _408 as uint16_t;
                                                            _410 = _297;
                                                            _411 = _318;
                                                            _320 = &raw mut *((*(&raw mut (*(_410
                                                                as *mut l_struct_struct_OC_block_size_descriptor))
                                                                .field13
                                                                as *mut crate::l_array_87_struct_AC_l_struct_struct_OC_decimation_mode))
                                                                .array)
                                                                .as_mut_ptr()
                                                                .offset(_411 as int32_t as int64_t as isize)
                                                                as *mut l_struct_struct_OC_decimation_mode
                                                                as *mut core::ffi::c_void;
                                                            _412 = _313;
                                                            if _412 as core::ffi::c_uint
                                                                & 1 as core::ffi::c_uint
                                                                != 0
                                                            {
                                                                _413 = _320;
                                                                _414 = _319;
                                                                _415 = _ZNK10block_mode21get_weight_quant_modeEv(_414);
                                                                _ZN15decimation_mode14set_ref_2planeE12quant_method(
                                                                    _413,
                                                                    _415,
                                                                );
                                                            } else {
                                                                _416 = _320;
                                                                _417 = _319;
                                                                _418 = _ZNK10block_mode21get_weight_quant_modeEv(_417);
                                                                _ZN15decimation_mode14set_ref_1planeE12quant_method(
                                                                    _416,
                                                                    _418,
                                                                );
                                                            }
                                                            _419 = _303;
                                                            _420 = _297;
                                                            _421 = _310;
                                                            *(&raw mut *((*(&raw mut (*(_420
                                                                as *mut l_struct_struct_OC_block_size_descriptor))
                                                                .field15 as *mut crate::l_array_2048_uint16_t))
                                                                .array)
                                                                .as_mut_ptr()
                                                                .offset(_421 as uint64_t as int64_t as isize)
                                                                as *mut uint16_t) = _419 as uint16_t;
                                                            _422 = _303;
                                                            _303 = llvm_add_u32(_422, 1);
                                                            _423 = _309;
                                                            _424 = &raw mut *(_305.array)
                                                                .as_mut_ptr()
                                                                .offset(
                                                                    _423 as uint64_t as int64_t
                                                                        as isize,
                                                                )
                                                                as *mut uint32_t
                                                                as *mut core::ffi::c_void;
                                                            _425 = *(_424 as *mut uint32_t);
                                                            *(_424 as *mut uint32_t) =
                                                                llvm_add_u32(_425, 1);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _426 = _310;
            _310 = llvm_add_u32(_426, 1);
        }
        _427 = _309;
        _309 = llvm_add_u32(_427, 1);
    }
    _428 = *(&raw mut *(_305.array)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as int64_t as isize) as *mut uint32_t);
    _429 = _297;
    *(&raw mut (*(_429 as *mut l_struct_struct_OC_block_size_descriptor)).field7
        as *mut uint32_t) = _428;
    _430 = *(&raw mut *(_305.array)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as int64_t as isize) as *mut uint32_t);
    _431 = *(&raw mut *(_305.array)
        .as_mut_ptr()
        .offset(1 as core::ffi::c_int as int64_t as isize) as *mut uint32_t);
    _432 = _297;
    *(&raw mut (*(_432 as *mut l_struct_struct_OC_block_size_descriptor)).field8
        as *mut uint32_t) = llvm_add_u32(_430, _431);
    _433 = *(&raw mut *(_305.array)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as int64_t as isize) as *mut uint32_t);
    _434 = *(&raw mut *(_305.array)
        .as_mut_ptr()
        .offset(1 as core::ffi::c_int as int64_t as isize) as *mut uint32_t);
    _435 = *(&raw mut *(_305.array)
        .as_mut_ptr()
        .offset(2 as core::ffi::c_int as int64_t as isize) as *mut uint32_t);
    _436 = _297;
    *(&raw mut (*(_436 as *mut l_struct_struct_OC_block_size_descriptor)).field9
        as *mut uint32_t) = llvm_add_u32(llvm_add_u32(_433, _434), _435);
    _437 = *(&raw mut *(_305.array)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as int64_t as isize) as *mut uint32_t);
    _438 = *(&raw mut *(_305.array)
        .as_mut_ptr()
        .offset(1 as core::ffi::c_int as int64_t as isize) as *mut uint32_t);
    _439 = *(&raw mut *(_305.array)
        .as_mut_ptr()
        .offset(2 as core::ffi::c_int as int64_t as isize) as *mut uint32_t);
    _440 = *(&raw mut *(_305.array)
        .as_mut_ptr()
        .offset(3 as core::ffi::c_int as int64_t as isize) as *mut uint32_t);
    _441 = _297;
    *(&raw mut (*(_441 as *mut l_struct_struct_OC_block_size_descriptor)).field10
        as *mut uint32_t) = llvm_add_u32(llvm_add_u32(llvm_add_u32(_437, _438), _439), _440);
    _442 = *(&raw mut *(_306.array)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as int64_t as isize) as *mut uint32_t);
    _443 = _297;
    *(&raw mut (*(_443 as *mut l_struct_struct_OC_block_size_descriptor)).field4
        as *mut uint32_t) = _442;
    _444 = *(&raw mut *(_306.array)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as int64_t as isize) as *mut uint32_t);
    _445 = *(&raw mut *(_306.array)
        .as_mut_ptr()
        .offset(1 as core::ffi::c_int as int64_t as isize) as *mut uint32_t);
    _446 = *(&raw mut *(_306.array)
        .as_mut_ptr()
        .offset(2 as core::ffi::c_int as int64_t as isize) as *mut uint32_t);
    _447 = _297;
    *(&raw mut (*(_447 as *mut l_struct_struct_OC_block_size_descriptor)).field5
        as *mut uint32_t) = llvm_add_u32(llvm_add_u32(_444, _445), _446);
    _448 = *(&raw mut *(_306.array)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as int64_t as isize) as *mut uint32_t);
    _449 = *(&raw mut *(_306.array)
        .as_mut_ptr()
        .offset(1 as core::ffi::c_int as int64_t as isize) as *mut uint32_t);
    _450 = *(&raw mut *(_306.array)
        .as_mut_ptr()
        .offset(2 as core::ffi::c_int as int64_t as isize) as *mut uint32_t);
    _451 = *(&raw mut *(_306.array)
        .as_mut_ptr()
        .offset(3 as core::ffi::c_int as int64_t as isize) as *mut uint32_t);
    _452 = _297;
    *(&raw mut (*(_452 as *mut l_struct_struct_OC_block_size_descriptor)).field6
        as *mut uint32_t) = llvm_add_u32(llvm_add_u32(llvm_add_u32(_448, _449), _450), _451);
    _453 = _297;
    _454 = *(&raw mut (*(_453 as *mut l_struct_struct_OC_block_size_descriptor)).field7
        as *mut uint32_t);
    if _454 > 0 as core::ffi::c_uint {
        _455 = _297;
        _456 = *(&raw mut (*(_455 as *mut l_struct_struct_OC_block_size_descriptor)).field4
            as *mut uint32_t);
        if _456 > 0 as core::ffi::c_uint {
            _457 = _301;
            if !_457.is_null() {
                _ZdaPv(_457);
            }
            _458 = _297;
            _459 = *(&raw mut (*(_458 as *mut l_struct_struct_OC_block_size_descriptor)).field6
                as *mut uint32_t);
            _321 = _459;
            loop {
                _460 = _321;
                if !(_460 < 87 as core::ffi::c_uint) {
                    break;
                }
                _461 = _297;
                _462 = _321;
                *(&raw mut (*(&raw mut *((*(&raw mut (*(_461
                    as *mut l_struct_struct_OC_block_size_descriptor))
                    .field13
                    as *mut crate::l_array_87_struct_AC_l_struct_struct_OC_decimation_mode))
                    .array)
                    .as_mut_ptr()
                    .offset(_462 as uint64_t as int64_t as isize)
                    as *mut l_struct_struct_OC_decimation_mode))
                    .field0 as *mut uint8_t) = -(1 as core::ffi::c_int) as uint8_t;
                _463 = _297;
                _464 = _321;
                *(&raw mut (*(&raw mut *((*(&raw mut (*(_463
                    as *mut l_struct_struct_OC_block_size_descriptor))
                    .field13
                    as *mut crate::l_array_87_struct_AC_l_struct_struct_OC_decimation_mode))
                    .array)
                    .as_mut_ptr()
                    .offset(_464 as uint64_t as int64_t as isize)
                    as *mut l_struct_struct_OC_decimation_mode))
                    .field1 as *mut uint8_t) = -(1 as core::ffi::c_int) as uint8_t;
                _465 = _297;
                _466 = _321;
                *(&raw mut (*(&raw mut *((*(&raw mut (*(_465
                    as *mut l_struct_struct_OC_block_size_descriptor))
                    .field13
                    as *mut crate::l_array_87_struct_AC_l_struct_struct_OC_decimation_mode))
                    .array)
                    .as_mut_ptr()
                    .offset(_466 as uint64_t as int64_t as isize)
                    as *mut l_struct_struct_OC_decimation_mode))
                    .field2 as *mut uint16_t) = 0;
                _467 = _297;
                _468 = _321;
                *(&raw mut (*(&raw mut *((*(&raw mut (*(_467
                    as *mut l_struct_struct_OC_block_size_descriptor))
                    .field13
                    as *mut crate::l_array_87_struct_AC_l_struct_struct_OC_decimation_mode))
                    .array)
                    .as_mut_ptr()
                    .offset(_468 as uint64_t as int64_t as isize)
                    as *mut l_struct_struct_OC_decimation_mode))
                    .field3 as *mut uint16_t) = 0;
                _469 = _321;
                _321 = llvm_add_u32(_469, 1);
            }
            _470 = _297;
            _ZL20assign_kmeans_texelsR21block_size_descriptor(_470);
            _471 = _299;
            if !_471.is_null() {
                _ZdlPvm(_471, 29656);
            }
            return;
        } else {
            __assert_fail(
                &_OC_str_OC_4 as *const crate::l_array_37_uint8_t as *mut core::ffi::c_void,
                &_OC_str_OC_1 as *const crate::l_array_50_uint8_t as *mut core::ffi::c_void,
                962,
                &__PRETTY_FUNCTION___OC__ZL34construct_block_size_descriptor_2djjbfR21block_size_descriptor
                    as *const crate::l_array_106_uint8_t as *mut core::ffi::c_void,
            );
        }
    } else {
        __assert_fail(
            &_OC_str_OC_3 as *const crate::l_array_39_uint8_t as *mut core::ffi::c_void,
            &_OC_str_OC_1 as *const crate::l_array_50_uint8_t as *mut core::ffi::c_void,
            961,
            &__PRETTY_FUNCTION___OC__ZL34construct_block_size_descriptor_2djjbfR21block_size_descriptor
                as *const crate::l_array_106_uint8_t as *mut core::ffi::c_void,
        );
    };
}
#[inline(never)]
unsafe extern "C" fn _ZL23init_decimation_info_3djjjjjjR15decimation_infoR23dt_init_working_buffers(
    mut _531: uint32_t,
    mut _532: uint32_t,
    mut _533: uint32_t,
    mut _534: uint32_t,
    mut _535: uint32_t,
    mut _536: uint32_t,
    mut _537: *mut core::ffi::c_void,
    mut _538: *mut core::ffi::c_void,
) {
    let mut current_block: u64;
    let mut _539: uint32_t = 0;
    let mut _540: uint32_t = 0;
    let mut _541: uint32_t = 0;
    let mut _542: uint32_t = 0;
    let mut _543: uint32_t = 0;
    let mut _544: uint32_t = 0;
    let mut _545: uint32_t = 0;
    let mut _546: uint32_t = 0;
    let mut _547: uint32_t = 0;
    let mut _548: uint32_t = 0;
    let mut _549: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _550: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _551: uint32_t = 0;
    let mut _552: uint32_t = 0;
    let mut _553: uint8_t = 0;
    let mut _554: uint32_t = 0;
    let mut _555: uint32_t = 0;
    let mut _556: uint32_t = 0;
    let mut _557: uint32_t = 0;
    let mut _558: uint32_t = 0;
    let mut _559: uint32_t = 0;
    let mut _560: uint32_t = 0;
    let mut _561: uint32_t = 0;
    let mut _562: uint32_t = 0;
    let mut _563: uint32_t = 0;
    let mut _564: uint32_t = 0;
    let mut _565: uint32_t = 0;
    let mut _566: uint32_t = 0;
    let mut _567: uint32_t = 0;
    let mut _568: uint32_t = 0;
    let mut _569: crate::l_array_4_uint32_t = crate::l_array_4_uint32_t { array: [0; 4] };
    let mut _570: crate::l_array_4_uint32_t = crate::l_array_4_uint32_t { array: [0; 4] };
    let mut _571: uint32_t = 0;
    let mut _572: uint32_t = 0;
    let mut _573: uint32_t = 0;
    let mut _574: uint32_t = 0;
    let mut _575: uint32_t = 0;
    let mut _576: uint32_t = 0;
    let mut _577: uint32_t = 0;
    let mut _578: uint32_t = 0;
    let mut _579: uint32_t = 0;
    let mut _580: uint32_t = 0;
    let mut _581: uint32_t = 0;
    let mut _582: uint32_t = 0;
    let mut _583: uint32_t = 0;
    let mut _584: uint8_t = 0;
    let mut _585: uint32_t = 0;
    let mut _586: uint32_t = 0;
    let mut _587: uint32_t = 0;
    let mut _588: uint32_t = 0;
    let mut _589: uint32_t = 0;
    let mut _590: uint32_t = 0;
    let mut _591: uint32_t = 0;
    let mut _592: uint32_t = 0;
    let mut _593: uint8_t = 0;
    let mut _594: core::ffi::c_float = 0.;
    let mut _595: uint8_t = 0;
    let mut _596: uint32_t = 0;
    let mut _597: uint32_t = 0;
    let mut _598: uint32_t = 0;
    let mut _599: uint32_t = 0;
    let mut _600: uint32_t = 0;
    let mut _601: uint8_t = 0;
    let mut _602: uint32_t = 0;
    let mut _603: uint32_t = 0;
    let mut _604: uint32_t = 0;
    let mut _605: uint32_t = 0;
    let mut _606: uint32_t = 0;
    let mut _607: uint32_t = 0;
    let mut _608: uint32_t = 0;
    let mut _609: uint32_t = 0;
    let mut _610: uint32_t = 0;
    let mut _611: uint32_t = 0;
    let mut _612: uint32_t = 0;
    let mut _613: uint32_t = 0;
    let mut _614: uint32_t = 0;
    let mut _615: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _616: uint32_t = 0;
    let mut _617: uint32_t = 0;
    let mut _618: uint32_t = 0;
    let mut _619: uint32_t = 0;
    let mut _620: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _621: uint32_t = 0;
    let mut _622: uint32_t = 0;
    let mut _623: uint32_t = 0;
    let mut _624: uint32_t = 0;
    let mut _625: uint32_t = 0;
    let mut _626: uint32_t = 0;
    let mut _627: uint32_t = 0;
    let mut _628: uint32_t = 0;
    let mut _629: uint32_t = 0;
    let mut _630: uint32_t = 0;
    let mut _631: uint32_t = 0;
    let mut _632: uint32_t = 0;
    let mut _633: uint32_t = 0;
    let mut _634: uint32_t = 0;
    let mut _635: uint32_t = 0;
    let mut _636: uint32_t = 0;
    let mut _637: uint32_t = 0;
    let mut _638: uint32_t = 0;
    let mut _639: uint32_t = 0;
    let mut _640: uint32_t = 0;
    let mut _641: uint32_t = 0;
    let mut _642: uint32_t = 0;
    let mut _643: uint32_t = 0;
    let mut _644: uint32_t = 0;
    let mut _645: uint32_t = 0;
    let mut _646: uint32_t = 0;
    let mut _647: uint32_t = 0;
    let mut _648: uint32_t = 0;
    let mut _649: uint32_t = 0;
    let mut _650: uint32_t = 0;
    let mut _651: uint32_t = 0;
    let mut _652: uint32_t = 0;
    let mut _653: uint32_t = 0;
    let mut _654: uint32_t = 0;
    let mut _655: uint32_t = 0;
    let mut _656: uint32_t = 0;
    let mut _657: uint32_t = 0;
    let mut _658: uint32_t = 0;
    let mut _659: uint32_t = 0;
    let mut _660: uint32_t = 0;
    let mut _661: uint32_t = 0;
    let mut _662: uint32_t = 0;
    let mut _663: uint32_t = 0;
    let mut _664: uint32_t = 0;
    let mut _665: uint32_t = 0;
    let mut _666: uint32_t = 0;
    let mut _667: uint32_t = 0;
    let mut _668: uint32_t = 0;
    let mut _669: uint32_t = 0;
    let mut _670: uint32_t = 0;
    let mut _671: uint32_t = 0;
    let mut _672: uint32_t = 0;
    let mut _673: uint32_t = 0;
    let mut _674: uint32_t = 0;
    let mut _675: uint32_t = 0;
    let mut _676: uint32_t = 0;
    let mut _677: uint32_t = 0;
    let mut _678: uint32_t = 0;
    let mut _679: uint32_t = 0;
    let mut _680: uint32_t = 0;
    let mut _681: uint32_t = 0;
    let mut _682: uint32_t = 0;
    let mut _683: uint32_t = 0;
    let mut _684: uint32_t = 0;
    let mut _685: uint32_t = 0;
    let mut _686: uint32_t = 0;
    let mut _687: uint32_t = 0;
    let mut _688: uint32_t = 0;
    let mut _689: uint32_t = 0;
    let mut _690: uint32_t = 0;
    let mut _691: uint32_t = 0;
    let mut _692: uint32_t = 0;
    let mut _693: uint32_t = 0;
    let mut _694: uint32_t = 0;
    let mut _695: uint32_t = 0;
    let mut _696: uint32_t = 0;
    let mut _697: uint32_t = 0;
    let mut _698: uint32_t = 0;
    let mut _699: uint32_t = 0;
    let mut _700: uint32_t = 0;
    let mut _701: uint32_t = 0;
    let mut _702: uint32_t = 0;
    let mut _703: uint32_t = 0;
    let mut _704: uint32_t = 0;
    let mut _705: uint32_t = 0;
    let mut _706: uint32_t = 0;
    let mut _707: uint32_t = 0;
    let mut _708: uint32_t = 0;
    let mut _709: uint32_t = 0;
    let mut _710: uint32_t = 0;
    let mut _711: uint32_t = 0;
    let mut _712: uint32_t = 0;
    let mut _713: uint32_t = 0;
    let mut _714: uint32_t = 0;
    let mut _715: uint32_t = 0;
    let mut _716: uint32_t = 0;
    let mut _717: uint32_t = 0;
    let mut _718: uint32_t = 0;
    let mut _719: uint32_t = 0;
    let mut _720: uint32_t = 0;
    let mut _721: uint32_t = 0;
    let mut _722: uint32_t = 0;
    let mut _723: uint32_t = 0;
    let mut _724: uint32_t = 0;
    let mut _725: uint32_t = 0;
    let mut _726: uint32_t = 0;
    let mut _727: uint32_t = 0;
    let mut _728: uint32_t = 0;
    let mut _729: uint32_t = 0;
    let mut _730: uint32_t = 0;
    let mut _731: uint32_t = 0;
    let mut _732: uint32_t = 0;
    let mut _733: uint32_t = 0;
    let mut _734: uint32_t = 0;
    let mut _735: uint32_t = 0;
    let mut _736: uint32_t = 0;
    let mut _737: uint32_t = 0;
    let mut _738: uint32_t = 0;
    let mut _739: uint32_t = 0;
    let mut _740: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _741: uint32_t = 0;
    let mut _742: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _743: uint32_t = 0;
    let mut _744: uint8_t = 0;
    let mut _745: uint32_t = 0;
    let mut _746: uint32_t = 0;
    let mut _747: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _748: uint32_t = 0;
    let mut _749: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _750: uint32_t = 0;
    let mut _751: uint8_t = 0;
    let mut _752: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _753: uint32_t = 0;
    let mut _754: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _755: uint8_t = 0;
    let mut _756: uint32_t = 0;
    let mut _757: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _758: uint32_t = 0;
    let mut _759: uint32_t = 0;
    let mut _760: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _761: uint32_t = 0;
    let mut _762: uint32_t = 0;
    let mut _763: uint8_t = 0;
    let mut _764: uint32_t = 0;
    let mut _765: uint32_t = 0;
    let mut _766: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _767: uint32_t = 0;
    let mut _768: uint32_t = 0;
    let mut _769: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _770: uint32_t = 0;
    let mut _771: uint32_t = 0;
    let mut _772: uint8_t = 0;
    let mut _773: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _774: uint32_t = 0;
    let mut _775: uint32_t = 0;
    let mut _776: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _777: uint8_t = 0;
    let mut _778: uint8_t = 0;
    let mut _779: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _780: uint32_t = 0;
    let mut _781: uint32_t = 0;
    let mut _782: uint8_t = 0;
    let mut _783: uint8_t = 0;
    let mut _784: uint32_t = 0;
    let mut _785: uint32_t = 0;
    let mut _786: uint32_t = 0;
    let mut _787: uint32_t = 0;
    let mut _788: uint32_t = 0;
    let mut _789: uint32_t = 0;
    let mut _790: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _791: uint32_t = 0;
    let mut _792: uint8_t = 0;
    let mut _793: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _794: uint32_t = 0;
    let mut _795: uint8_t = 0;
    let mut _796: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _797: uint32_t = 0;
    let mut _798: uint8_t = 0;
    let mut _799: uint8_t = 0;
    let mut _800: uint32_t = 0;
    let mut _801: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _802: uint32_t = 0;
    let mut _803: uint32_t = 0;
    let mut _804: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _805: uint32_t = 0;
    let mut _806: uint32_t = 0;
    let mut _807: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _808: uint32_t = 0;
    let mut _809: uint32_t = 0;
    let mut _810: uint32_t = 0;
    let mut _811: uint32_t = 0;
    let mut _812: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _813: uint32_t = 0;
    let mut _814: uint8_t = 0;
    let mut _815: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _816: uint32_t = 0;
    let mut _817: uint32_t = 0;
    let mut _818: uint8_t = 0;
    let mut _819: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _820: uint32_t = 0;
    let mut _821: uint32_t = 0;
    let mut _822: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _823: uint32_t = 0;
    let mut _824: uint32_t = 0;
    let mut _825: uint8_t = 0;
    let mut _826: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _827: uint32_t = 0;
    let mut _828: uint32_t = 0;
    let mut _829: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _830: uint32_t = 0;
    let mut _831: uint32_t = 0;
    let mut _832: uint8_t = 0;
    let mut _833: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _834: uint32_t = 0;
    let mut _835: uint32_t = 0;
    let mut _836: uint32_t = 0;
    let mut _837: uint32_t = 0;
    let mut _838: uint8_t = 0;
    let mut _839: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _840: uint32_t = 0;
    let mut _841: uint32_t = 0;
    let mut _842: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _843: uint32_t = 0;
    let mut _844: uint8_t = 0;
    let mut _845: uint32_t = 0;
    let mut _846: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _847: uint32_t = 0;
    let mut _848: uint32_t = 0;
    let mut _849: uint32_t = 0;
    let mut _850: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _851: uint32_t = 0;
    let mut _852: uint32_t = 0;
    let mut _853: uint8_t = 0;
    let mut _854: uint32_t = 0;
    let mut _855: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _856: uint32_t = 0;
    let mut _857: uint32_t = 0;
    let mut _858: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _859: uint32_t = 0;
    let mut _860: uint32_t = 0;
    let mut _861: uint8_t = 0;
    let mut _862: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _863: uint32_t = 0;
    let mut _864: uint32_t = 0;
    let mut _865: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _866: uint32_t = 0;
    let mut _867: uint32_t = 0;
    let mut _868: uint32_t = 0;
    let mut _869: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _870: uint32_t = 0;
    let mut _871: uint32_t = 0;
    let mut _872: uint8_t = 0;
    let mut _873: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _874: uint32_t = 0;
    let mut _875: uint32_t = 0;
    let mut _876: core::ffi::c_float = 0.;
    let mut _877: uint8_t = 0;
    let mut _878: uint32_t = 0;
    let mut _879: core::ffi::c_float = 0.;
    let mut _880: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _881: uint32_t = 0;
    let mut _882: uint32_t = 0;
    let mut _883: core::ffi::c_float = 0.;
    let mut _884: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _885: uint32_t = 0;
    let mut _886: uint32_t = 0;
    let mut _887: uint32_t = 0;
    let mut _888: uint32_t = 0;
    let mut _889: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _890: uint32_t = 0;
    let mut _891: uint32_t = 0;
    let mut _892: uint8_t = 0;
    let mut _893: uint32_t = 0;
    let mut _894: uint32_t = 0;
    let mut _895: uint8_t = 0;
    let mut _896: uint8_t = 0;
    let mut _897: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _898: uint32_t = 0;
    let mut _899: uint32_t = 0;
    let mut _900: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _901: uint32_t = 0;
    let mut _902: uint32_t = 0;
    let mut _903: uint32_t = 0;
    let mut _904: uint32_t = 0;
    let mut _905: uint32_t = 0;
    let mut _906: uint32_t = 0;
    let mut _907: uint32_t = 0;
    let mut _908: uint32_t = 0;
    let mut _909: uint32_t = 0;
    let mut _910: uint32_t = 0;
    let mut _911: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _912: uint32_t = 0;
    let mut _913: uint32_t = 0;
    let mut _914: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _915: uint32_t = 0;
    let mut _916: uint32_t = 0;
    let mut _917: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _918: uint32_t = 0;
    let mut _919: uint32_t = 0;
    let mut _920: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _921: uint32_t = 0;
    let mut _922: uint32_t = 0;
    let mut _923: uint32_t = 0;
    let mut _924: uint32_t = 0;
    let mut _925: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _926: uint32_t = 0;
    let mut _927: uint8_t = 0;
    let mut _928: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _929: uint32_t = 0;
    let mut _930: uint32_t = 0;
    let mut _931: uint8_t = 0;
    let mut _932: uint32_t = 0;
    let mut _933: uint32_t = 0;
    let mut _934: uint32_t = 0;
    let mut _935: uint32_t = 0;
    let mut _936: uint32_t = 0;
    let mut _937: uint32_t = 0;
    let mut _938: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _939: uint32_t = 0;
    let mut _940: uint32_t = 0;
    let mut _941: uint8_t = 0;
    let mut _942: uint8_t = 0;
    let mut _943: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _944: uint32_t = 0;
    let mut _945: uint32_t = 0;
    let mut _946: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _947: uint32_t = 0;
    let mut _948: uint32_t = 0;
    let mut _949: uint32_t = 0;
    let mut _950: uint32_t = 0;
    let mut _951: uint32_t = 0;
    let mut _952: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _953: uint32_t = 0;
    let mut _954: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _955: uint32_t = 0;
    let mut _956: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _957: uint32_t = 0;
    let mut _958: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _959: uint32_t = 0;
    let mut _960: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    _543 = _531;
    _544 = _532;
    _545 = _533;
    _546 = _534;
    _547 = _535;
    _548 = _536;
    _549 = _537;
    _550 = _538;
    _605 = _543;
    _606 = _544;
    _607 = _545;
    _551 = llvm_mul_u32(llvm_mul_u32(_605, _606), _607);
    _608 = _546;
    _609 = _547;
    _610 = _548;
    _552 = llvm_mul_u32(llvm_mul_u32(_608, _609), _610);
    _553 = 0;
    _611 = _552;
    if _611 > 0 as core::ffi::c_uint {
        _612 = _551;
        if _612 > 0 as core::ffi::c_uint {
            _554 = 0;
            loop {
                _613 = _554;
                _614 = _552;
                if !(_613 < _614) {
                    break;
                }
                _615 = _550;
                _616 = _554;
                *(&raw mut *((*(&raw mut (*(_615
                    as *mut l_struct_struct_OC_dt_init_working_buffers))
                    .field3 as *mut crate::l_array_64_uint8_t))
                    .array)
                    .as_mut_ptr()
                    .offset(_616 as uint64_t as int64_t as isize)
                    as *mut uint8_t) = 0;
                _617 = _554;
                _554 = llvm_add_u32(_617, 1);
            }
            _555 = 0;
            loop {
                _618 = _555;
                _619 = _551;
                if !(_618 < _619) {
                    break;
                }
                _620 = _550;
                _621 = _555;
                *(&raw mut *((*(&raw mut (*(_620
                    as *mut l_struct_struct_OC_dt_init_working_buffers))
                    .field0 as *mut crate::l_array_216_uint8_t))
                    .array)
                    .as_mut_ptr()
                    .offset(_621 as uint64_t as int64_t as isize)
                    as *mut uint8_t) = 0;
                _622 = _555;
                _555 = llvm_add_u32(_622, 1);
            }
            _556 = 0;
            loop {
                _623 = _556;
                _624 = _545;
                if !(_623 < _624) {
                    break;
                }
                _557 = 0;
                loop {
                    _625 = _557;
                    _626 = _544;
                    if !(_625 < _626) {
                        break;
                    }
                    _558 = 0;
                    loop {
                        _627 = _558;
                        _628 = _543;
                        if !(_627 < _628) {
                            break;
                        }
                        _629 = _556;
                        _630 = _544;
                        _631 = _557;
                        _632 = _543;
                        _633 = _558;
                        _559 = llvm_add_u32(
                            llvm_mul_u32(llvm_add_u32(llvm_mul_u32(_629, _630), _631), _632),
                            _633,
                        );
                        _634 = _543;
                        _635 = _543;
                        _636 = _558;
                        _637 = _546;
                        _560 = llvm_lshr_u32(
                            llvm_add_u32(
                                llvm_mul_u32(
                                    llvm_mul_u32(
                                        llvm_udiv_u32(
                                            llvm_add_u32(1024, llvm_udiv_u32(_634, 2)),
                                            llvm_sub_u32(_635, 1),
                                        ),
                                        _636,
                                    ),
                                    llvm_sub_u32(_637, 1),
                                ),
                                32,
                            ),
                            6,
                        );
                        _638 = _544;
                        _639 = _544;
                        _640 = _557;
                        _641 = _547;
                        _561 = llvm_lshr_u32(
                            llvm_add_u32(
                                llvm_mul_u32(
                                    llvm_mul_u32(
                                        llvm_udiv_u32(
                                            llvm_add_u32(1024, llvm_udiv_u32(_638, 2)),
                                            llvm_sub_u32(_639, 1),
                                        ),
                                        _640,
                                    ),
                                    llvm_sub_u32(_641, 1),
                                ),
                                32,
                            ),
                            6,
                        );
                        _642 = _545;
                        _643 = _545;
                        _644 = _556;
                        _645 = _548;
                        _562 = llvm_lshr_u32(
                            llvm_add_u32(
                                llvm_mul_u32(
                                    llvm_mul_u32(
                                        llvm_udiv_u32(
                                            llvm_add_u32(1024, llvm_udiv_u32(_642, 2)),
                                            llvm_sub_u32(_643, 1),
                                        ),
                                        _644,
                                    ),
                                    llvm_sub_u32(_645, 1),
                                ),
                                32,
                            ),
                            6,
                        );
                        _646 = _560;
                        _563 = _646 & 15;
                        _647 = _561;
                        _564 = _647 & 15;
                        _648 = _562;
                        _565 = _648 & 15;
                        _649 = _560;
                        _566 = llvm_ashr_u32(_649 as int32_t, 4 as core::ffi::c_int);
                        _650 = _561;
                        _567 = llvm_ashr_u32(_650 as int32_t, 4 as core::ffi::c_int);
                        _651 = _562;
                        _568 = llvm_ashr_u32(_651 as int32_t, 4 as core::ffi::c_int);
                        _652 = _568;
                        _653 = _547;
                        _654 = _567;
                        _655 = _546;
                        _656 = _566;
                        *(&raw mut *(_569.array)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as int64_t as isize)
                            as *mut uint32_t) = llvm_add_u32(
                            llvm_mul_u32(llvm_add_u32(llvm_mul_u32(_652, _653), _654), _655),
                            _656,
                        );
                        _657 = _568;
                        _658 = _547;
                        _659 = _567;
                        _660 = _546;
                        _661 = _566;
                        *(&raw mut *(_569.array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut uint32_t) = llvm_add_u32(
                            llvm_mul_u32(
                                llvm_add_u32(
                                    llvm_mul_u32(llvm_add_u32(_657, 1), _658),
                                    llvm_add_u32(_659, 1),
                                ),
                                _660,
                            ),
                            llvm_add_u32(_661, 1),
                        );
                        _662 = _563;
                        _571 = _662;
                        _663 = _564;
                        _572 = _663;
                        _664 = _565;
                        _573 = _664;
                        _665 = _571;
                        _666 = _572;
                        _667 = _572;
                        _668 = _573;
                        _669 = _571;
                        _670 = _573;
                        _574 = llvm_add_u32(
                            llvm_add_u32(
                                ((_665 as int32_t > _666 as int32_t) as core::ffi::c_int as bool_0
                                    as uint32_t)
                                    << 2 as core::ffi::c_int,
                                ((_667 as int32_t > _668 as int32_t) as core::ffi::c_int as bool_0
                                    as uint32_t)
                                    << 1 as core::ffi::c_int,
                            ),
                            (_669 as int32_t > _670 as int32_t) as core::ffi::c_int as bool_0
                                as uint32_t,
                        );
                        _671 = _546;
                        _575 = _671;
                        _672 = _546;
                        _673 = _547;
                        _576 = llvm_mul_u32(_672, _673);
                        _674 = _574;
                        match _674 {
                            7 => {
                                _577 = 1;
                                _675 = _575;
                                _578 = _675;
                                _676 = _571;
                                _579 = llvm_sub_u32(16, _676);
                                _677 = _571;
                                _678 = _572;
                                _580 = llvm_sub_u32(_677, _678);
                                _679 = _572;
                                _680 = _573;
                                _581 = llvm_sub_u32(_679, _680);
                                _681 = _573;
                                _582 = _681;
                            }
                            3 => {
                                _682 = _575;
                                _577 = _682;
                                _578 = 1;
                                _683 = _572;
                                _579 = llvm_sub_u32(16, _683);
                                _684 = _572;
                                _685 = _571;
                                _580 = llvm_sub_u32(_684, _685);
                                _686 = _571;
                                _687 = _573;
                                _581 = llvm_sub_u32(_686, _687);
                                _688 = _573;
                                _582 = _688;
                            }
                            5 => {
                                _577 = 1;
                                _689 = _576;
                                _578 = _689;
                                _690 = _571;
                                _579 = llvm_sub_u32(16, _690);
                                _691 = _571;
                                _692 = _573;
                                _580 = llvm_sub_u32(_691, _692);
                                _693 = _573;
                                _694 = _572;
                                _581 = llvm_sub_u32(_693, _694);
                                _695 = _572;
                                _582 = _695;
                            }
                            4 => {
                                _696 = _576;
                                _577 = _696;
                                _578 = 1;
                                _697 = _573;
                                _579 = llvm_sub_u32(16, _697);
                                _698 = _573;
                                _699 = _571;
                                _580 = llvm_sub_u32(_698, _699);
                                _700 = _571;
                                _701 = _572;
                                _581 = llvm_sub_u32(_700, _701);
                                _702 = _572;
                                _582 = _702;
                            }
                            2 => {
                                _703 = _575;
                                _577 = _703;
                                _704 = _576;
                                _578 = _704;
                                _705 = _572;
                                _579 = llvm_sub_u32(16, _705);
                                _706 = _572;
                                _707 = _573;
                                _580 = llvm_sub_u32(_706, _707);
                                _708 = _573;
                                _709 = _571;
                                _581 = llvm_sub_u32(_708, _709);
                                _710 = _571;
                                _582 = _710;
                            }
                            0 => {
                                _711 = _576;
                                _577 = _711;
                                _712 = _575;
                                _578 = _712;
                                _713 = _573;
                                _579 = llvm_sub_u32(16, _713);
                                _714 = _573;
                                _715 = _572;
                                _580 = llvm_sub_u32(_714, _715);
                                _716 = _572;
                                _717 = _571;
                                _581 = llvm_sub_u32(_716, _717);
                                _718 = _571;
                                _582 = _718;
                            }
                            _ => {
                                _719 = _576;
                                _577 = _719;
                                _720 = _575;
                                _578 = _720;
                                _721 = _573;
                                _579 = llvm_sub_u32(16, _721);
                                _722 = _573;
                                _723 = _572;
                                _580 = llvm_sub_u32(_722, _723);
                                _724 = _572;
                                _725 = _571;
                                _581 = llvm_sub_u32(_724, _725);
                                _726 = _571;
                                _582 = _726;
                            }
                        }
                        _727 = *(&raw mut *(_569.array)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as int64_t as isize)
                            as *mut uint32_t);
                        _728 = _577;
                        *(&raw mut *(_569.array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut uint32_t) = llvm_add_u32(_727, _728);
                        _729 = *(&raw mut *(_569.array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut uint32_t);
                        _730 = _578;
                        *(&raw mut *(_569.array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut uint32_t) = llvm_add_u32(_729, _730);
                        _731 = _579;
                        *(&raw mut *(_570.array)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as int64_t as isize)
                            as *mut uint32_t) = _731;
                        _732 = _580;
                        *(&raw mut *(_570.array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut uint32_t) = _732;
                        _733 = _581;
                        *(&raw mut *(_570.array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut uint32_t) = _733;
                        _734 = _582;
                        *(&raw mut *(_570.array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut uint32_t) = _734;
                        _583 = 0;
                        loop {
                            _735 = _583;
                            if !(_735 < 4 as core::ffi::c_uint) {
                                break;
                            }
                            _736 = _583;
                            _737 = *(&raw mut *(_570.array)
                                .as_mut_ptr()
                                .offset(_736 as uint64_t as int64_t as isize)
                                as *mut uint32_t);
                            if _737 != 0 as core::ffi::c_uint {
                                _738 = _583;
                                _739 = *(&raw mut *(_569.array)
                                    .as_mut_ptr()
                                    .offset(_738 as uint64_t as int64_t as isize)
                                    as *mut uint32_t);
                                _740 = _550;
                                _741 = _559;
                                _742 = _550;
                                _743 = _559;
                                _744 = *(&raw mut *((*(&raw mut (*(_742
                                    as *mut l_struct_struct_OC_dt_init_working_buffers))
                                    .field0
                                    as *mut crate::l_array_216_uint8_t))
                                    .array)
                                    .as_mut_ptr()
                                    .offset(_743 as int32_t as int64_t as isize)
                                    as *mut uint8_t);
                                *(&raw mut *((*(&raw mut *((*(&raw mut (*(_740
                                    as *mut l_struct_struct_OC_dt_init_working_buffers))
                                    .field1
                                    as *mut crate::l_array_216_struct_AC_l_array_4_uint8_t))
                                    .array)
                                    .as_mut_ptr()
                                    .offset(_741 as int32_t as int64_t as isize)
                                    as *mut crate::l_array_4_uint8_t))
                                    .array)
                                    .as_mut_ptr()
                                    .offset(_744 as uint64_t as int64_t as isize)
                                    as *mut uint8_t) = _739 as uint8_t;
                                _745 = _583;
                                _746 = *(&raw mut *(_570.array)
                                    .as_mut_ptr()
                                    .offset(_745 as uint64_t as int64_t as isize)
                                    as *mut uint32_t);
                                _747 = _550;
                                _748 = _559;
                                _749 = _550;
                                _750 = _559;
                                _751 = *(&raw mut *((*(&raw mut (*(_749
                                    as *mut l_struct_struct_OC_dt_init_working_buffers))
                                    .field0
                                    as *mut crate::l_array_216_uint8_t))
                                    .array)
                                    .as_mut_ptr()
                                    .offset(_750 as int32_t as int64_t as isize)
                                    as *mut uint8_t);
                                *(&raw mut *((*(&raw mut *((*(&raw mut (*(_747
                                    as *mut l_struct_struct_OC_dt_init_working_buffers))
                                    .field2
                                    as *mut crate::l_array_216_struct_AC_l_array_4_uint8_t))
                                    .array)
                                    .as_mut_ptr()
                                    .offset(_748 as int32_t as int64_t as isize)
                                    as *mut crate::l_array_4_uint8_t))
                                    .array)
                                    .as_mut_ptr()
                                    .offset(_751 as uint64_t as int64_t as isize)
                                    as *mut uint8_t) = _746 as uint8_t;
                                _752 = _550;
                                _753 = _559;
                                _754 = &raw mut *((*(&raw mut (*(_752
                                    as *mut l_struct_struct_OC_dt_init_working_buffers))
                                    .field0
                                    as *mut crate::l_array_216_uint8_t))
                                    .array)
                                    .as_mut_ptr()
                                    .offset(_753 as int32_t as int64_t as isize)
                                    as *mut uint8_t
                                    as *mut core::ffi::c_void;
                                _755 = *(_754 as *mut uint8_t);
                                *(_754 as *mut uint8_t) = llvm_add_u8(_755, 1);
                                _756 = _559;
                                _757 = _550;
                                _758 = _583;
                                _759 = *(&raw mut *(_569.array)
                                    .as_mut_ptr()
                                    .offset(_758 as uint64_t as int64_t as isize)
                                    as *mut uint32_t);
                                _760 = _550;
                                _761 = _583;
                                _762 = *(&raw mut *(_569.array)
                                    .as_mut_ptr()
                                    .offset(_761 as uint64_t as int64_t as isize)
                                    as *mut uint32_t);
                                _763 = *(&raw mut *((*(&raw mut (*(_760
                                    as *mut l_struct_struct_OC_dt_init_working_buffers))
                                    .field3
                                    as *mut crate::l_array_64_uint8_t))
                                    .array)
                                    .as_mut_ptr()
                                    .offset(_762 as int32_t as int64_t as isize)
                                    as *mut uint8_t);
                                *(&raw mut *((*(&raw mut *((*(&raw mut (*(_757
                                    as *mut l_struct_struct_OC_dt_init_working_buffers))
                                    .field4
                                    as *mut crate::l_array_64_struct_AC_l_array_216_uint8_t))
                                    .array)
                                    .as_mut_ptr()
                                    .offset(_759 as int32_t as int64_t as isize)
                                    as *mut crate::l_array_216_uint8_t))
                                    .array)
                                    .as_mut_ptr()
                                    .offset(_763 as uint64_t as int64_t as isize)
                                    as *mut uint8_t) = _756 as uint8_t;
                                _764 = _583;
                                _765 = *(&raw mut *(_570.array)
                                    .as_mut_ptr()
                                    .offset(_764 as uint64_t as int64_t as isize)
                                    as *mut uint32_t);
                                _766 = _550;
                                _767 = _583;
                                _768 = *(&raw mut *(_569.array)
                                    .as_mut_ptr()
                                    .offset(_767 as uint64_t as int64_t as isize)
                                    as *mut uint32_t);
                                _769 = _550;
                                _770 = _583;
                                _771 = *(&raw mut *(_569.array)
                                    .as_mut_ptr()
                                    .offset(_770 as uint64_t as int64_t as isize)
                                    as *mut uint32_t);
                                _772 = *(&raw mut *((*(&raw mut (*(_769
                                    as *mut l_struct_struct_OC_dt_init_working_buffers))
                                    .field3
                                    as *mut crate::l_array_64_uint8_t))
                                    .array)
                                    .as_mut_ptr()
                                    .offset(_771 as int32_t as int64_t as isize)
                                    as *mut uint8_t);
                                *(&raw mut *((*(&raw mut *((*(&raw mut (*(_766
                                    as *mut l_struct_struct_OC_dt_init_working_buffers))
                                    .field5
                                    as *mut crate::l_array_64_struct_AC_l_array_216_uint8_t))
                                    .array)
                                    .as_mut_ptr()
                                    .offset(_768 as int32_t as int64_t as isize)
                                    as *mut crate::l_array_216_uint8_t))
                                    .array)
                                    .as_mut_ptr()
                                    .offset(_772 as uint64_t as int64_t as isize)
                                    as *mut uint8_t) = _765 as uint8_t;
                                _773 = _550;
                                _774 = _583;
                                _775 = *(&raw mut *(_569.array)
                                    .as_mut_ptr()
                                    .offset(_774 as uint64_t as int64_t as isize)
                                    as *mut uint32_t);
                                _776 = &raw mut *((*(&raw mut (*(_773
                                    as *mut l_struct_struct_OC_dt_init_working_buffers))
                                    .field3
                                    as *mut crate::l_array_64_uint8_t))
                                    .array)
                                    .as_mut_ptr()
                                    .offset(_775 as int32_t as int64_t as isize)
                                    as *mut uint8_t
                                    as *mut core::ffi::c_void;
                                _777 = *(_776 as *mut uint8_t);
                                *(_776 as *mut uint8_t) = llvm_add_u8(_777, 1);
                                _778 = _553;
                                _779 = _550;
                                _780 = _583;
                                _781 = *(&raw mut *(_569.array)
                                    .as_mut_ptr()
                                    .offset(_780 as uint64_t as int64_t as isize)
                                    as *mut uint32_t);
                                _782 = *(&raw mut *((*(&raw mut (*(_779
                                    as *mut l_struct_struct_OC_dt_init_working_buffers))
                                    .field3
                                    as *mut crate::l_array_64_uint8_t))
                                    .array)
                                    .as_mut_ptr()
                                    .offset(_781 as int32_t as int64_t as isize)
                                    as *mut uint8_t);
                                _783 = _ZN4astcL3maxIhEET_S1_S1_(_778, _782);
                                _553 = _783;
                            }
                            _784 = _583;
                            _583 = llvm_add_u32(_784, 1);
                        }
                        _785 = _558;
                        _558 = llvm_add_u32(_785, 1);
                    }
                    _786 = _557;
                    _557 = llvm_add_u32(_786, 1);
                }
                _787 = _556;
                _556 = llvm_add_u32(_787, 1);
            }
            _584 = 0;
            _585 = 0;
            loop {
                _788 = _585;
                _789 = _551;
                if !(_788 < _789) {
                    break;
                }
                _790 = _550;
                _791 = _585;
                _792 = *(&raw mut *((*(&raw mut (*(_790
                    as *mut l_struct_struct_OC_dt_init_working_buffers))
                    .field0
                    as *mut crate::l_array_216_uint8_t))
                    .array)
                    .as_mut_ptr()
                    .offset(_791 as uint64_t as int64_t as isize)
                    as *mut uint8_t);
                _793 = _549;
                _794 = _585;
                *(&raw mut *((*(&raw mut (*(_793 as *mut l_struct_struct_OC_decimation_info))
                    .field6 as *mut crate::l_array_216_uint8_t))
                    .array)
                    .as_mut_ptr()
                    .offset(_794 as uint64_t as int64_t as isize)
                    as *mut uint8_t) = _792;
                _795 = _584;
                _796 = _549;
                _797 = _585;
                _798 =
                    *(&raw mut *((*(&raw mut (*(_796 as *mut l_struct_struct_OC_decimation_info))
                        .field6
                        as *mut crate::l_array_216_uint8_t))
                        .array)
                        .as_mut_ptr()
                        .offset(_797 as uint64_t as int64_t as isize)
                        as *mut uint8_t);
                _799 = _ZN4astcL3maxIhEET_S1_S1_(_795, _798);
                _584 = _799;
                _586 = 0;
                loop {
                    _800 = _586;
                    if !(_800 < 4 as core::ffi::c_uint) {
                        break;
                    }
                    _801 = _549;
                    _802 = _586;
                    _803 = _585;
                    *(&raw mut *((*(&raw mut *((*(&raw mut (*(_801
                        as *mut l_struct_struct_OC_decimation_info))
                        .field8
                        as *mut crate::l_array_4_struct_AC_l_array_216_uint8_t))
                        .array)
                        .as_mut_ptr()
                        .offset(_802 as uint64_t as int64_t as isize)
                        as *mut crate::l_array_216_uint8_t))
                        .array)
                        .as_mut_ptr()
                        .offset(_803 as uint64_t as int64_t as isize)
                        as *mut uint8_t) = 0;
                    _804 = _549;
                    _805 = _586;
                    _806 = _585;
                    *(&raw mut *((*(&raw mut *((*(&raw mut (*(_804
                        as *mut l_struct_struct_OC_decimation_info))
                        .field9
                        as *mut crate::l_array_4_struct_AC_l_array_216_float))
                        .array)
                        .as_mut_ptr()
                        .offset(_805 as uint64_t as int64_t as isize)
                        as *mut crate::l_array_216_float))
                        .array)
                        .as_mut_ptr()
                        .offset(_806 as uint64_t as int64_t as isize)
                        as *mut core::ffi::c_float) = 0 as core::ffi::c_int as core::ffi::c_float;
                    _807 = _549;
                    _808 = _586;
                    _809 = _585;
                    *(&raw mut *((*(&raw mut *((*(&raw mut (*(_807
                        as *mut l_struct_struct_OC_decimation_info))
                        .field7
                        as *mut crate::l_array_4_struct_AC_l_array_216_uint8_t))
                        .array)
                        .as_mut_ptr()
                        .offset(_808 as uint64_t as int64_t as isize)
                        as *mut crate::l_array_216_uint8_t))
                        .array)
                        .as_mut_ptr()
                        .offset(_809 as uint64_t as int64_t as isize)
                        as *mut uint8_t) = 0;
                    _810 = _586;
                    _586 = llvm_add_u32(_810, 1);
                }
                _587 = 0;
                loop {
                    _811 = _587;
                    _812 = _550;
                    _813 = _585;
                    _814 = *(&raw mut *((*(&raw mut (*(_812
                        as *mut l_struct_struct_OC_dt_init_working_buffers))
                        .field0
                        as *mut crate::l_array_216_uint8_t))
                        .array)
                        .as_mut_ptr()
                        .offset(_813 as uint64_t as int64_t as isize)
                        as *mut uint8_t);
                    if !(_811 < _814 as uint32_t) {
                        break;
                    }
                    _815 = _550;
                    _816 = _585;
                    _817 = _587;
                    _818 = *(&raw mut *((*(&raw mut *((*(&raw mut (*(_815
                        as *mut l_struct_struct_OC_dt_init_working_buffers))
                        .field2
                        as *mut crate::l_array_216_struct_AC_l_array_4_uint8_t))
                        .array)
                        .as_mut_ptr()
                        .offset(_816 as uint64_t as int64_t as isize)
                        as *mut crate::l_array_4_uint8_t))
                        .array)
                        .as_mut_ptr()
                        .offset(_817 as uint64_t as int64_t as isize)
                        as *mut uint8_t);
                    _819 = _549;
                    _820 = _587;
                    _821 = _585;
                    *(&raw mut *((*(&raw mut *((*(&raw mut (*(_819
                        as *mut l_struct_struct_OC_decimation_info))
                        .field8
                        as *mut crate::l_array_4_struct_AC_l_array_216_uint8_t))
                        .array)
                        .as_mut_ptr()
                        .offset(_820 as uint64_t as int64_t as isize)
                        as *mut crate::l_array_216_uint8_t))
                        .array)
                        .as_mut_ptr()
                        .offset(_821 as uint64_t as int64_t as isize)
                        as *mut uint8_t) = _818;
                    _822 = _550;
                    _823 = _585;
                    _824 = _587;
                    _825 = *(&raw mut *((*(&raw mut *((*(&raw mut (*(_822
                        as *mut l_struct_struct_OC_dt_init_working_buffers))
                        .field2
                        as *mut crate::l_array_216_struct_AC_l_array_4_uint8_t))
                        .array)
                        .as_mut_ptr()
                        .offset(_823 as uint64_t as int64_t as isize)
                        as *mut crate::l_array_4_uint8_t))
                        .array)
                        .as_mut_ptr()
                        .offset(_824 as uint64_t as int64_t as isize)
                        as *mut uint8_t);
                    _826 = _549;
                    _827 = _587;
                    _828 = _585;
                    *(&raw mut *((*(&raw mut *((*(&raw mut (*(_826
                        as *mut l_struct_struct_OC_decimation_info))
                        .field9
                        as *mut crate::l_array_4_struct_AC_l_array_216_float))
                        .array)
                        .as_mut_ptr()
                        .offset(_827 as uint64_t as int64_t as isize)
                        as *mut crate::l_array_216_float))
                        .array)
                        .as_mut_ptr()
                        .offset(_828 as uint64_t as int64_t as isize)
                        as *mut core::ffi::c_float) =
                        llvm_fmul_f32(_825 as core::ffi::c_float, 0.0625f64 as core::ffi::c_float);
                    _829 = _550;
                    _830 = _585;
                    _831 = _587;
                    _832 = *(&raw mut *((*(&raw mut *((*(&raw mut (*(_829
                        as *mut l_struct_struct_OC_dt_init_working_buffers))
                        .field1
                        as *mut crate::l_array_216_struct_AC_l_array_4_uint8_t))
                        .array)
                        .as_mut_ptr()
                        .offset(_830 as uint64_t as int64_t as isize)
                        as *mut crate::l_array_4_uint8_t))
                        .array)
                        .as_mut_ptr()
                        .offset(_831 as uint64_t as int64_t as isize)
                        as *mut uint8_t);
                    _833 = _549;
                    _834 = _587;
                    _835 = _585;
                    *(&raw mut *((*(&raw mut *((*(&raw mut (*(_833
                        as *mut l_struct_struct_OC_decimation_info))
                        .field7
                        as *mut crate::l_array_4_struct_AC_l_array_216_uint8_t))
                        .array)
                        .as_mut_ptr()
                        .offset(_834 as uint64_t as int64_t as isize)
                        as *mut crate::l_array_216_uint8_t))
                        .array)
                        .as_mut_ptr()
                        .offset(_835 as uint64_t as int64_t as isize)
                        as *mut uint8_t) = _832;
                    _836 = _587;
                    _587 = llvm_add_u32(_836, 1);
                }
                _837 = _585;
                _585 = llvm_add_u32(_837, 1);
            }
            _838 = _584;
            _839 = _549;
            *(&raw mut (*(_839 as *mut l_struct_struct_OC_decimation_info)).field1
                as *mut uint8_t) = _838;
            _588 = 0;
            loop {
                _840 = _588;
                _841 = _552;
                if !(_840 < _841) {
                    break;
                }
                _842 = _550;
                _843 = _588;
                _844 = *(&raw mut *((*(&raw mut (*(_842
                    as *mut l_struct_struct_OC_dt_init_working_buffers))
                    .field3
                    as *mut crate::l_array_64_uint8_t))
                    .array)
                    .as_mut_ptr()
                    .offset(_843 as uint64_t as int64_t as isize)
                    as *mut uint8_t);
                _589 = _844 as uint32_t;
                _845 = _589;
                _846 = _549;
                _847 = _588;
                *(&raw mut *((*(&raw mut (*(_846 as *mut l_struct_struct_OC_decimation_info))
                    .field10 as *mut crate::l_array_64_uint8_t))
                    .array)
                    .as_mut_ptr()
                    .offset(_847 as uint64_t as int64_t as isize)
                    as *mut uint8_t) = _845 as uint8_t;
                _590 = 0;
                loop {
                    _848 = _590;
                    _849 = _589;
                    if !(_848 < _849) {
                        break;
                    }
                    _850 = _550;
                    _851 = _588;
                    _852 = _590;
                    _853 = *(&raw mut *((*(&raw mut *((*(&raw mut (*(_850
                        as *mut l_struct_struct_OC_dt_init_working_buffers))
                        .field4
                        as *mut crate::l_array_64_struct_AC_l_array_216_uint8_t))
                        .array)
                        .as_mut_ptr()
                        .offset(_851 as uint64_t as int64_t as isize)
                        as *mut crate::l_array_216_uint8_t))
                        .array)
                        .as_mut_ptr()
                        .offset(_852 as uint64_t as int64_t as isize)
                        as *mut uint8_t);
                    _591 = _853 as uint32_t;
                    _854 = _591;
                    _855 = _549;
                    _856 = _590;
                    _857 = _588;
                    *(&raw mut *((*(&raw mut *((*(&raw mut (*(_855
                        as *mut l_struct_struct_OC_decimation_info))
                        .field11
                        as *mut crate::l_array_216_struct_AC_l_array_64_uint8_t))
                        .array)
                        .as_mut_ptr()
                        .offset(_856 as uint64_t as int64_t as isize)
                        as *mut crate::l_array_64_uint8_t))
                        .array)
                        .as_mut_ptr()
                        .offset(_857 as uint64_t as int64_t as isize)
                        as *mut uint8_t) = _854 as uint8_t;
                    _858 = _550;
                    _859 = _588;
                    _860 = _590;
                    _861 = *(&raw mut *((*(&raw mut *((*(&raw mut (*(_858
                        as *mut l_struct_struct_OC_dt_init_working_buffers))
                        .field5
                        as *mut crate::l_array_64_struct_AC_l_array_216_uint8_t))
                        .array)
                        .as_mut_ptr()
                        .offset(_859 as uint64_t as int64_t as isize)
                        as *mut crate::l_array_216_uint8_t))
                        .array)
                        .as_mut_ptr()
                        .offset(_860 as uint64_t as int64_t as isize)
                        as *mut uint8_t);
                    _862 = _549;
                    _863 = _590;
                    _864 = _588;
                    *(&raw mut *((*(&raw mut *((*(&raw mut (*(_862
                        as *mut l_struct_struct_OC_decimation_info))
                        .field12
                        as *mut crate::l_array_216_struct_AC_l_array_64_float))
                        .array)
                        .as_mut_ptr()
                        .offset(_863 as uint64_t as int64_t as isize)
                        as *mut crate::l_array_64_float))
                        .array)
                        .as_mut_ptr()
                        .offset(_864 as uint64_t as int64_t as isize)
                        as *mut core::ffi::c_float) = _861 as core::ffi::c_float;
                    _865 = _549;
                    _866 = _590;
                    _867 = _588;
                    *(&raw mut *((*(&raw mut *((*(&raw mut (*(_865
                        as *mut l_struct_struct_OC_decimation_info))
                        .field13
                        as *mut crate::l_array_216_struct_AC_l_array_64_float))
                        .array)
                        .as_mut_ptr()
                        .offset(_866 as uint64_t as int64_t as isize)
                        as *mut crate::l_array_64_float))
                        .array)
                        .as_mut_ptr()
                        .offset(_867 as uint64_t as int64_t as isize)
                        as *mut core::ffi::c_float) = 0 as core::ffi::c_int as core::ffi::c_float;
                    _592 = 0;
                    loop {
                        _868 = _592;
                        if !(_868 < 4 as core::ffi::c_uint) {
                            current_block = 10423119982448545950;
                            break;
                        }
                        _869 = _549;
                        _870 = _592;
                        _871 = _591;
                        _872 = *(&raw mut *((*(&raw mut *((*(&raw mut (*(_869
                            as *mut l_struct_struct_OC_decimation_info))
                            .field7
                            as *mut crate::l_array_4_struct_AC_l_array_216_uint8_t))
                            .array)
                            .as_mut_ptr()
                            .offset(_870 as uint64_t as int64_t as isize)
                            as *mut crate::l_array_216_uint8_t))
                            .array)
                            .as_mut_ptr()
                            .offset(_871 as uint64_t as int64_t as isize)
                            as *mut uint8_t);
                        _593 = _872;
                        _873 = _549;
                        _874 = _592;
                        _875 = _591;
                        _876 = *(&raw mut *((*(&raw mut *((*(&raw mut (*(_873
                            as *mut l_struct_struct_OC_decimation_info))
                            .field9
                            as *mut crate::l_array_4_struct_AC_l_array_216_float))
                            .array)
                            .as_mut_ptr()
                            .offset(_874 as uint64_t as int64_t as isize)
                            as *mut crate::l_array_216_float))
                            .array)
                            .as_mut_ptr()
                            .offset(_875 as uint64_t as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _594 = _876;
                        _877 = _593;
                        _878 = _588;
                        if _877 as uint32_t == _878 {
                            _879 = _594;
                            if llvm_fcmp_une(
                                _879 as core::ffi::c_double,
                                0 as core::ffi::c_int as core::ffi::c_double,
                            ) != 0
                            {
                                current_block = 4102975741765736954;
                                break;
                            }
                        }
                        _887 = _592;
                        _592 = llvm_add_u32(_887, 1);
                    }
                    match current_block {
                        4102975741765736954 => {
                            _880 = _549;
                            _881 = _592;
                            _882 = _591;
                            _883 = *(&raw mut *((*(&raw mut *((*(&raw mut (*(_880
                                as *mut l_struct_struct_OC_decimation_info))
                                .field9
                                as *mut crate::l_array_4_struct_AC_l_array_216_float))
                                .array)
                                .as_mut_ptr()
                                .offset(_881 as uint64_t as int64_t as isize)
                                as *mut crate::l_array_216_float))
                                .array)
                                .as_mut_ptr()
                                .offset(_882 as uint64_t as int64_t as isize)
                                as *mut core::ffi::c_float);
                            _884 = _549;
                            _885 = _590;
                            _886 = _588;
                            *(&raw mut *((*(&raw mut *((*(&raw mut (*(_884
                                as *mut l_struct_struct_OC_decimation_info))
                                .field13
                                as *mut crate::l_array_216_struct_AC_l_array_64_float))
                                .array)
                                .as_mut_ptr()
                                .offset(_885 as uint64_t as int64_t as isize)
                                as *mut crate::l_array_64_float))
                                .array)
                                .as_mut_ptr()
                                .offset(_886 as uint64_t as int64_t as isize)
                                as *mut core::ffi::c_float) = _883;
                        }
                        _ => {}
                    }
                    _888 = _590;
                    _590 = llvm_add_u32(_888, 1);
                }
                _889 = _549;
                _890 = _589;
                _891 = _588;
                _892 = *(&raw mut *((*(&raw mut *((*(&raw mut (*(_889
                    as *mut l_struct_struct_OC_decimation_info))
                    .field11
                    as *mut crate::l_array_216_struct_AC_l_array_64_uint8_t))
                    .array)
                    .as_mut_ptr()
                    .offset(
                        (llvm_sub_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            _890, 1,
                        ) as uint64_t as int64_t as isize,
                    ) as *mut crate::l_array_64_uint8_t))
                    .array)
                    .as_mut_ptr()
                    .offset(_891 as uint64_t as int64_t as isize)
                    as *mut uint8_t);
                _595 = _892;
                _893 = _589;
                _596 = _893;
                loop {
                    _894 = _596;
                    _895 = _553;
                    if !(_894 < _895 as uint32_t) {
                        break;
                    }
                    _896 = _595;
                    _897 = _549;
                    _898 = _596;
                    _899 = _588;
                    *(&raw mut *((*(&raw mut *((*(&raw mut (*(_897
                        as *mut l_struct_struct_OC_decimation_info))
                        .field11
                        as *mut crate::l_array_216_struct_AC_l_array_64_uint8_t))
                        .array)
                        .as_mut_ptr()
                        .offset(_898 as uint64_t as int64_t as isize)
                        as *mut crate::l_array_64_uint8_t))
                        .array)
                        .as_mut_ptr()
                        .offset(_899 as uint64_t as int64_t as isize)
                        as *mut uint8_t) = _896;
                    _900 = _549;
                    _901 = _596;
                    _902 = _588;
                    *(&raw mut *((*(&raw mut *((*(&raw mut (*(_900
                        as *mut l_struct_struct_OC_decimation_info))
                        .field12
                        as *mut crate::l_array_216_struct_AC_l_array_64_float))
                        .array)
                        .as_mut_ptr()
                        .offset(_901 as uint64_t as int64_t as isize)
                        as *mut crate::l_array_64_float))
                        .array)
                        .as_mut_ptr()
                        .offset(_902 as uint64_t as int64_t as isize)
                        as *mut core::ffi::c_float) = 0 as core::ffi::c_int as core::ffi::c_float;
                    _903 = _596;
                    _596 = llvm_add_u32(_903, 1);
                }
                _904 = _588;
                _588 = llvm_add_u32(_904, 1);
            }
            _905 = _551;
            _539 = _905;
            _906 = _539;
            _540 = llvm_udiv_u32(llvm_sub_u32(llvm_add_u32(_906, 4), 1), 4);
            _907 = _540;
            _597 = llvm_mul_u32(_907, 4);
            _908 = _551;
            _598 = _908;
            loop {
                _909 = _598;
                _910 = _597;
                if !(_909 < _910) {
                    break;
                }
                _911 = _549;
                _912 = _598;
                *(&raw mut *((*(&raw mut (*(_911 as *mut l_struct_struct_OC_decimation_info))
                    .field6 as *mut crate::l_array_216_uint8_t))
                    .array)
                    .as_mut_ptr()
                    .offset(_912 as uint64_t as int64_t as isize)
                    as *mut uint8_t) = 0;
                _599 = 0;
                loop {
                    _913 = _599;
                    if !(_913 < 4 as core::ffi::c_uint) {
                        break;
                    }
                    _914 = _549;
                    _915 = _599;
                    _916 = _598;
                    *(&raw mut *((*(&raw mut *((*(&raw mut (*(_914
                        as *mut l_struct_struct_OC_decimation_info))
                        .field9
                        as *mut crate::l_array_4_struct_AC_l_array_216_float))
                        .array)
                        .as_mut_ptr()
                        .offset(_915 as uint64_t as int64_t as isize)
                        as *mut crate::l_array_216_float))
                        .array)
                        .as_mut_ptr()
                        .offset(_916 as uint64_t as int64_t as isize)
                        as *mut core::ffi::c_float) = 0 as core::ffi::c_int as core::ffi::c_float;
                    _917 = _549;
                    _918 = _599;
                    _919 = _598;
                    *(&raw mut *((*(&raw mut *((*(&raw mut (*(_917
                        as *mut l_struct_struct_OC_decimation_info))
                        .field7
                        as *mut crate::l_array_4_struct_AC_l_array_216_uint8_t))
                        .array)
                        .as_mut_ptr()
                        .offset(_918 as uint64_t as int64_t as isize)
                        as *mut crate::l_array_216_uint8_t))
                        .array)
                        .as_mut_ptr()
                        .offset(_919 as uint64_t as int64_t as isize)
                        as *mut uint8_t) = 0;
                    _920 = _549;
                    _921 = _599;
                    _922 = _598;
                    *(&raw mut *((*(&raw mut *((*(&raw mut (*(_920
                        as *mut l_struct_struct_OC_decimation_info))
                        .field8
                        as *mut crate::l_array_4_struct_AC_l_array_216_uint8_t))
                        .array)
                        .as_mut_ptr()
                        .offset(_921 as uint64_t as int64_t as isize)
                        as *mut crate::l_array_216_uint8_t))
                        .array)
                        .as_mut_ptr()
                        .offset(_922 as uint64_t as int64_t as isize)
                        as *mut uint8_t) = 0;
                    _923 = _599;
                    _599 = llvm_add_u32(_923, 1);
                }
                _924 = _598;
                _598 = llvm_add_u32(_924, 1);
            }
            _925 = _550;
            _926 = _552;
            _927 = *(&raw mut *((*(&raw mut (*(_925
                as *mut l_struct_struct_OC_dt_init_working_buffers))
                .field3 as *mut crate::l_array_64_uint8_t))
                .array)
                .as_mut_ptr()
                .offset(
                    (llvm_sub_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(_926, 1)
                        as uint64_t as int64_t as isize,
                ) as *mut uint8_t);
            _600 = _927 as uint32_t;
            _928 = _549;
            _929 = _600;
            _930 = _552;
            _931 = *(&raw mut *((*(&raw mut *((*(&raw mut (*(_928
                as *mut l_struct_struct_OC_decimation_info))
                .field11
                as *mut crate::l_array_216_struct_AC_l_array_64_uint8_t))
                .array)
                .as_mut_ptr()
                .offset(
                    (llvm_sub_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(_929, 1)
                        as int32_t as int64_t as isize,
                ) as *mut crate::l_array_64_uint8_t))
                .array)
                .as_mut_ptr()
                .offset(
                    (llvm_sub_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(_930, 1)
                        as uint64_t as int64_t as isize,
                ) as *mut uint8_t);
            _601 = _931;
            _932 = _552;
            _541 = _932;
            _933 = _541;
            _542 = llvm_udiv_u32(llvm_sub_u32(llvm_add_u32(_933, 4), 1), 4);
            _934 = _542;
            _602 = llvm_mul_u32(_934, 4);
            _935 = _552;
            _603 = _935;
            loop {
                _936 = _603;
                _937 = _602;
                if !(_936 < _937) {
                    break;
                }
                _938 = _549;
                _939 = _603;
                *(&raw mut *((*(&raw mut (*(_938 as *mut l_struct_struct_OC_decimation_info))
                    .field10 as *mut crate::l_array_64_uint8_t))
                    .array)
                    .as_mut_ptr()
                    .offset(_939 as uint64_t as int64_t as isize)
                    as *mut uint8_t) = 0;
                _604 = 0;
                loop {
                    _940 = _604;
                    _941 = _553;
                    if !((_940 as int32_t) < _941 as uint32_t as int32_t) {
                        break;
                    }
                    _942 = _601;
                    _943 = _549;
                    _944 = _604;
                    _945 = _603;
                    *(&raw mut *((*(&raw mut *((*(&raw mut (*(_943
                        as *mut l_struct_struct_OC_decimation_info))
                        .field11
                        as *mut crate::l_array_216_struct_AC_l_array_64_uint8_t))
                        .array)
                        .as_mut_ptr()
                        .offset(_944 as int32_t as int64_t as isize)
                        as *mut crate::l_array_64_uint8_t))
                        .array)
                        .as_mut_ptr()
                        .offset(_945 as uint64_t as int64_t as isize)
                        as *mut uint8_t) = _942;
                    _946 = _549;
                    _947 = _604;
                    _948 = _603;
                    *(&raw mut *((*(&raw mut *((*(&raw mut (*(_946
                        as *mut l_struct_struct_OC_decimation_info))
                        .field12
                        as *mut crate::l_array_216_struct_AC_l_array_64_float))
                        .array)
                        .as_mut_ptr()
                        .offset(_947 as int32_t as int64_t as isize)
                        as *mut crate::l_array_64_float))
                        .array)
                        .as_mut_ptr()
                        .offset(_948 as uint64_t as int64_t as isize)
                        as *mut core::ffi::c_float) = 0 as core::ffi::c_int as core::ffi::c_float;
                    _949 = _604;
                    _604 = llvm_add_u32(_949, 1);
                }
                _950 = _603;
                _603 = llvm_add_u32(_950, 1);
            }
            _951 = _551;
            _952 = _549;
            *(&raw mut (*(_952 as *mut l_struct_struct_OC_decimation_info)).field0
                as *mut uint8_t) = _951 as uint8_t;
            _953 = _552;
            _954 = _549;
            *(&raw mut (*(_954 as *mut l_struct_struct_OC_decimation_info)).field2
                as *mut uint8_t) = _953 as uint8_t;
            _955 = _546;
            _956 = _549;
            *(&raw mut (*(_956 as *mut l_struct_struct_OC_decimation_info)).field3
                as *mut uint8_t) = _955 as uint8_t;
            _957 = _547;
            _958 = _549;
            *(&raw mut (*(_958 as *mut l_struct_struct_OC_decimation_info)).field4
                as *mut uint8_t) = _957 as uint8_t;
            _959 = _548;
            _960 = _549;
            *(&raw mut (*(_960 as *mut l_struct_struct_OC_decimation_info)).field5
                as *mut uint8_t) = _959 as uint8_t;
            return;
        } else {
            __assert_fail(
                &_OC_str_OC_2 as *const crate::l_array_21_uint8_t as *mut core::ffi::c_void,
                &_OC_str_OC_1 as *const crate::l_array_50_uint8_t as *mut core::ffi::c_void,
                452,
                &__PRETTY_FUNCTION___OC__ZL23init_decimation_info_3djjjjjjR15decimation_infoR23dt_init_working_buffers
                    as *const crate::l_array_159_uint8_t as *mut core::ffi::c_void,
            );
        }
    } else {
        __assert_fail(
            &_OC_str as *const crate::l_array_22_uint8_t as *mut core::ffi::c_void,
            &_OC_str_OC_1 as *const crate::l_array_50_uint8_t as *mut core::ffi::c_void,
            451,
            &__PRETTY_FUNCTION___OC__ZL23init_decimation_info_3djjjjjjR15decimation_infoR23dt_init_working_buffers
                as *const crate::l_array_159_uint8_t as *mut core::ffi::c_void,
        );
    };
}
#[inline(never)]
unsafe extern "C" fn _ZL20decode_block_mode_3djRjS_S_RbS_S_(
    mut _1048: uint32_t,
    mut _1049: *mut core::ffi::c_void,
    mut _1050: *mut core::ffi::c_void,
    mut _1051: *mut core::ffi::c_void,
    mut _1052: *mut core::ffi::c_void,
    mut _1053: *mut core::ffi::c_void,
    mut _1054: *mut core::ffi::c_void,
) -> bool_0 {
    let mut current_block: u64;
    let mut _1055: bool_0 = 0;
    let mut _1056: uint32_t = 0;
    let mut _1057: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1058: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1059: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1060: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1061: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1062: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1063: uint32_t = 0;
    let mut _1064: uint32_t = 0;
    let mut _1065: uint32_t = 0;
    let mut _1066: uint32_t = 0;
    let mut _1067: uint32_t = 0;
    let mut _1068: uint32_t = 0;
    let mut _1069: uint32_t = 0;
    let mut _1070: uint32_t = 0;
    let mut _1071: uint32_t = 0;
    let mut _1072: uint32_t = 0;
    let mut _1073: uint32_t = 0;
    let mut _1074: uint32_t = 0;
    let mut _1075: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1076: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1077: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1078: uint32_t = 0;
    let mut _1079: uint32_t = 0;
    let mut _1080: uint32_t = 0;
    let mut _1081: uint32_t = 0;
    let mut _1082: uint32_t = 0;
    let mut _1083: uint32_t = 0;
    let mut _1084: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1085: uint32_t = 0;
    let mut _1086: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1087: uint32_t = 0;
    let mut _1088: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1089: uint32_t = 0;
    let mut _1090: uint32_t = 0;
    let mut _1091: uint32_t = 0;
    let mut _1092: uint32_t = 0;
    let mut _1093: uint32_t = 0;
    let mut _1094: uint32_t = 0;
    let mut _1095: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1096: uint32_t = 0;
    let mut _1097: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1098: uint32_t = 0;
    let mut _1099: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1100: uint32_t = 0;
    let mut _1101: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1102: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1103: uint32_t = 0;
    let mut _1104: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1105: uint32_t = 0;
    let mut _1106: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1107: uint32_t = 0;
    let mut _1108: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1109: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1110: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1111: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1112: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1113: uint32_t = 0;
    let mut _1114: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1115: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1116: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1117: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1118: uint32_t = 0;
    let mut _1119: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1120: uint32_t = 0;
    let mut _1121: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1122: uint32_t = 0;
    let mut _1123: uint32_t = 0;
    let mut _1124: uint32_t = 0;
    let mut _1125: uint32_t = 0;
    let mut _1126: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1127: uint32_t = 0;
    let mut _1128: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1129: uint32_t = 0;
    let mut _1130: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1131: uint32_t = 0;
    let mut _1132: uint32_t = 0;
    let mut _1133: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1134: uint32_t = 0;
    let mut _1135: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1136: uint32_t = 0;
    let mut _1137: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1138: uint32_t = 0;
    let mut _1139: bool_0 = 0;
    let mut _1139__PHI_TEMPORARY: bool_0 = 0;
    let mut _1140: bool_0 = 0;
    _1056 = _1048;
    _1057 = _1049;
    _1058 = _1050;
    _1059 = _1051;
    _1060 = _1052;
    _1061 = _1053;
    _1062 = _1054;
    _1071 = _1056;
    _1063 = llvm_lshr_u32(_1071, 4) & 1;
    _1072 = _1056;
    _1064 = llvm_lshr_u32(_1072, 9) & 1;
    _1073 = _1056;
    _1065 = llvm_lshr_u32(_1073, 10) & 1;
    _1074 = _1056;
    _1066 = llvm_lshr_u32(_1074, 5) & 3;
    _1075 = _1057;
    *(_1075 as *mut uint32_t) = 0;
    _1076 = _1058;
    *(_1076 as *mut uint32_t) = 0;
    _1077 = _1059;
    *(_1077 as *mut uint32_t) = 0;
    _1078 = _1056;
    if _1078 & 3 != 0 as core::ffi::c_uint {
        _1079 = _1056;
        _1080 = _1063;
        _1063 = _1080 | (_1079 & 3) << 1 as core::ffi::c_int;
        _1081 = _1056;
        _1067 = llvm_lshr_u32(_1081, 7) & 3;
        _1082 = _1056;
        _1068 = llvm_lshr_u32(_1082, 2) & 3;
        _1083 = _1066;
        _1084 = _1057;
        *(_1084 as *mut uint32_t) = llvm_add_u32(_1083, 2);
        _1085 = _1067;
        _1086 = _1058;
        *(_1086 as *mut uint32_t) = llvm_add_u32(_1085, 2);
        _1087 = _1068;
        _1088 = _1059;
        *(_1088 as *mut uint32_t) = llvm_add_u32(_1087, 2);
        current_block = 17020090522943023520;
    } else {
        _1089 = _1056;
        _1090 = _1063;
        _1063 = _1090 | (llvm_lshr_u32(_1089, 2) & 3) << 1 as core::ffi::c_int;
        _1091 = _1056;
        if llvm_lshr_u32(_1091, 2) & 3 == 0 as core::ffi::c_uint {
            _1055 = 0 as core::ffi::c_int as bool_0;
            current_block = 13987004837005789085;
        } else {
            _1092 = _1056;
            _1069 = llvm_lshr_u32(_1092, 9) & 3;
            _1093 = _1056;
            if llvm_lshr_u32(_1093, 7) & 3 != 3 as core::ffi::c_uint {
                _1065 = 0;
                _1064 = 0;
            }
            _1094 = _1056;
            match llvm_lshr_u32(_1094, 7) & 3 {
                0 => {
                    current_block = 13557016295891107554;
                    match current_block {
                        4536441701461369174 => {
                            _1110 = _1057;
                            *(_1110 as *mut uint32_t) = 2;
                            _1111 = _1058;
                            *(_1111 as *mut uint32_t) = 2;
                            _1112 = _1059;
                            *(_1112 as *mut uint32_t) = 2;
                            _1113 = _1056;
                            match llvm_lshr_u32(_1113, 5) & 3 {
                                0 => {
                                    current_block = 11798445329271324050;
                                    match current_block {
                                        8417943979444626301 => {
                                            _1055 = 0 as core::ffi::c_int as bool_0;
                                            current_block = 13987004837005789085;
                                        }
                                        831407651183063401 => {
                                            _1115 = _1058;
                                            *(_1115 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                        12558956782898289905 => {
                                            _1116 = _1059;
                                            *(_1116 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                        _ => {
                                            _1114 = _1057;
                                            *(_1114 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                    }
                                }
                                1 => {
                                    current_block = 831407651183063401;
                                    match current_block {
                                        8417943979444626301 => {
                                            _1055 = 0 as core::ffi::c_int as bool_0;
                                            current_block = 13987004837005789085;
                                        }
                                        831407651183063401 => {
                                            _1115 = _1058;
                                            *(_1115 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                        12558956782898289905 => {
                                            _1116 = _1059;
                                            *(_1116 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                        _ => {
                                            _1114 = _1057;
                                            *(_1114 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                    }
                                }
                                2 => {
                                    current_block = 12558956782898289905;
                                    match current_block {
                                        8417943979444626301 => {
                                            _1055 = 0 as core::ffi::c_int as bool_0;
                                            current_block = 13987004837005789085;
                                        }
                                        831407651183063401 => {
                                            _1115 = _1058;
                                            *(_1115 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                        12558956782898289905 => {
                                            _1116 = _1059;
                                            *(_1116 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                        _ => {
                                            _1114 = _1057;
                                            *(_1114 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                    }
                                }
                                3 => {
                                    current_block = 8417943979444626301;
                                    match current_block {
                                        8417943979444626301 => {
                                            _1055 = 0 as core::ffi::c_int as bool_0;
                                            current_block = 13987004837005789085;
                                        }
                                        831407651183063401 => {
                                            _1115 = _1058;
                                            *(_1115 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                        12558956782898289905 => {
                                            _1116 = _1059;
                                            *(_1116 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                        _ => {
                                            _1114 = _1057;
                                            *(_1114 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                    }
                                }
                                _ => {
                                    current_block = 17020090522943023520;
                                }
                            }
                        }
                        6357002656807115528 => {
                            _1100 = _1066;
                            _1101 = _1057;
                            *(_1101 as *mut uint32_t) = llvm_add_u32(_1100, 2);
                            _1102 = _1058;
                            *(_1102 as *mut uint32_t) = 6;
                            _1103 = _1069;
                            _1104 = _1059;
                            *(_1104 as *mut uint32_t) = llvm_add_u32(_1103, 2);
                            current_block = 17020090522943023520;
                        }
                        7431584555264274889 => {
                            _1105 = _1066;
                            _1106 = _1057;
                            *(_1106 as *mut uint32_t) = llvm_add_u32(_1105, 2);
                            _1107 = _1069;
                            _1108 = _1058;
                            *(_1108 as *mut uint32_t) = llvm_add_u32(_1107, 2);
                            _1109 = _1059;
                            *(_1109 as *mut uint32_t) = 6;
                            current_block = 17020090522943023520;
                        }
                        _ => {
                            _1095 = _1057;
                            *(_1095 as *mut uint32_t) = 6;
                            _1096 = _1069;
                            _1097 = _1058;
                            *(_1097 as *mut uint32_t) = llvm_add_u32(_1096, 2);
                            _1098 = _1066;
                            _1099 = _1059;
                            *(_1099 as *mut uint32_t) = llvm_add_u32(_1098, 2);
                            current_block = 17020090522943023520;
                        }
                    }
                }
                1 => {
                    current_block = 6357002656807115528;
                    match current_block {
                        4536441701461369174 => {
                            _1110 = _1057;
                            *(_1110 as *mut uint32_t) = 2;
                            _1111 = _1058;
                            *(_1111 as *mut uint32_t) = 2;
                            _1112 = _1059;
                            *(_1112 as *mut uint32_t) = 2;
                            _1113 = _1056;
                            match llvm_lshr_u32(_1113, 5) & 3 {
                                0 => {
                                    current_block = 11798445329271324050;
                                    match current_block {
                                        8417943979444626301 => {
                                            _1055 = 0 as core::ffi::c_int as bool_0;
                                            current_block = 13987004837005789085;
                                        }
                                        831407651183063401 => {
                                            _1115 = _1058;
                                            *(_1115 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                        12558956782898289905 => {
                                            _1116 = _1059;
                                            *(_1116 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                        _ => {
                                            _1114 = _1057;
                                            *(_1114 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                    }
                                }
                                1 => {
                                    current_block = 831407651183063401;
                                    match current_block {
                                        8417943979444626301 => {
                                            _1055 = 0 as core::ffi::c_int as bool_0;
                                            current_block = 13987004837005789085;
                                        }
                                        831407651183063401 => {
                                            _1115 = _1058;
                                            *(_1115 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                        12558956782898289905 => {
                                            _1116 = _1059;
                                            *(_1116 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                        _ => {
                                            _1114 = _1057;
                                            *(_1114 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                    }
                                }
                                2 => {
                                    current_block = 12558956782898289905;
                                    match current_block {
                                        8417943979444626301 => {
                                            _1055 = 0 as core::ffi::c_int as bool_0;
                                            current_block = 13987004837005789085;
                                        }
                                        831407651183063401 => {
                                            _1115 = _1058;
                                            *(_1115 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                        12558956782898289905 => {
                                            _1116 = _1059;
                                            *(_1116 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                        _ => {
                                            _1114 = _1057;
                                            *(_1114 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                    }
                                }
                                3 => {
                                    current_block = 8417943979444626301;
                                    match current_block {
                                        8417943979444626301 => {
                                            _1055 = 0 as core::ffi::c_int as bool_0;
                                            current_block = 13987004837005789085;
                                        }
                                        831407651183063401 => {
                                            _1115 = _1058;
                                            *(_1115 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                        12558956782898289905 => {
                                            _1116 = _1059;
                                            *(_1116 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                        _ => {
                                            _1114 = _1057;
                                            *(_1114 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                    }
                                }
                                _ => {
                                    current_block = 17020090522943023520;
                                }
                            }
                        }
                        6357002656807115528 => {
                            _1100 = _1066;
                            _1101 = _1057;
                            *(_1101 as *mut uint32_t) = llvm_add_u32(_1100, 2);
                            _1102 = _1058;
                            *(_1102 as *mut uint32_t) = 6;
                            _1103 = _1069;
                            _1104 = _1059;
                            *(_1104 as *mut uint32_t) = llvm_add_u32(_1103, 2);
                            current_block = 17020090522943023520;
                        }
                        7431584555264274889 => {
                            _1105 = _1066;
                            _1106 = _1057;
                            *(_1106 as *mut uint32_t) = llvm_add_u32(_1105, 2);
                            _1107 = _1069;
                            _1108 = _1058;
                            *(_1108 as *mut uint32_t) = llvm_add_u32(_1107, 2);
                            _1109 = _1059;
                            *(_1109 as *mut uint32_t) = 6;
                            current_block = 17020090522943023520;
                        }
                        _ => {
                            _1095 = _1057;
                            *(_1095 as *mut uint32_t) = 6;
                            _1096 = _1069;
                            _1097 = _1058;
                            *(_1097 as *mut uint32_t) = llvm_add_u32(_1096, 2);
                            _1098 = _1066;
                            _1099 = _1059;
                            *(_1099 as *mut uint32_t) = llvm_add_u32(_1098, 2);
                            current_block = 17020090522943023520;
                        }
                    }
                }
                2 => {
                    current_block = 7431584555264274889;
                    match current_block {
                        4536441701461369174 => {
                            _1110 = _1057;
                            *(_1110 as *mut uint32_t) = 2;
                            _1111 = _1058;
                            *(_1111 as *mut uint32_t) = 2;
                            _1112 = _1059;
                            *(_1112 as *mut uint32_t) = 2;
                            _1113 = _1056;
                            match llvm_lshr_u32(_1113, 5) & 3 {
                                0 => {
                                    current_block = 11798445329271324050;
                                    match current_block {
                                        8417943979444626301 => {
                                            _1055 = 0 as core::ffi::c_int as bool_0;
                                            current_block = 13987004837005789085;
                                        }
                                        831407651183063401 => {
                                            _1115 = _1058;
                                            *(_1115 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                        12558956782898289905 => {
                                            _1116 = _1059;
                                            *(_1116 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                        _ => {
                                            _1114 = _1057;
                                            *(_1114 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                    }
                                }
                                1 => {
                                    current_block = 831407651183063401;
                                    match current_block {
                                        8417943979444626301 => {
                                            _1055 = 0 as core::ffi::c_int as bool_0;
                                            current_block = 13987004837005789085;
                                        }
                                        831407651183063401 => {
                                            _1115 = _1058;
                                            *(_1115 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                        12558956782898289905 => {
                                            _1116 = _1059;
                                            *(_1116 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                        _ => {
                                            _1114 = _1057;
                                            *(_1114 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                    }
                                }
                                2 => {
                                    current_block = 12558956782898289905;
                                    match current_block {
                                        8417943979444626301 => {
                                            _1055 = 0 as core::ffi::c_int as bool_0;
                                            current_block = 13987004837005789085;
                                        }
                                        831407651183063401 => {
                                            _1115 = _1058;
                                            *(_1115 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                        12558956782898289905 => {
                                            _1116 = _1059;
                                            *(_1116 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                        _ => {
                                            _1114 = _1057;
                                            *(_1114 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                    }
                                }
                                3 => {
                                    current_block = 8417943979444626301;
                                    match current_block {
                                        8417943979444626301 => {
                                            _1055 = 0 as core::ffi::c_int as bool_0;
                                            current_block = 13987004837005789085;
                                        }
                                        831407651183063401 => {
                                            _1115 = _1058;
                                            *(_1115 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                        12558956782898289905 => {
                                            _1116 = _1059;
                                            *(_1116 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                        _ => {
                                            _1114 = _1057;
                                            *(_1114 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                    }
                                }
                                _ => {
                                    current_block = 17020090522943023520;
                                }
                            }
                        }
                        6357002656807115528 => {
                            _1100 = _1066;
                            _1101 = _1057;
                            *(_1101 as *mut uint32_t) = llvm_add_u32(_1100, 2);
                            _1102 = _1058;
                            *(_1102 as *mut uint32_t) = 6;
                            _1103 = _1069;
                            _1104 = _1059;
                            *(_1104 as *mut uint32_t) = llvm_add_u32(_1103, 2);
                            current_block = 17020090522943023520;
                        }
                        7431584555264274889 => {
                            _1105 = _1066;
                            _1106 = _1057;
                            *(_1106 as *mut uint32_t) = llvm_add_u32(_1105, 2);
                            _1107 = _1069;
                            _1108 = _1058;
                            *(_1108 as *mut uint32_t) = llvm_add_u32(_1107, 2);
                            _1109 = _1059;
                            *(_1109 as *mut uint32_t) = 6;
                            current_block = 17020090522943023520;
                        }
                        _ => {
                            _1095 = _1057;
                            *(_1095 as *mut uint32_t) = 6;
                            _1096 = _1069;
                            _1097 = _1058;
                            *(_1097 as *mut uint32_t) = llvm_add_u32(_1096, 2);
                            _1098 = _1066;
                            _1099 = _1059;
                            *(_1099 as *mut uint32_t) = llvm_add_u32(_1098, 2);
                            current_block = 17020090522943023520;
                        }
                    }
                }
                3 => {
                    current_block = 4536441701461369174;
                    match current_block {
                        4536441701461369174 => {
                            _1110 = _1057;
                            *(_1110 as *mut uint32_t) = 2;
                            _1111 = _1058;
                            *(_1111 as *mut uint32_t) = 2;
                            _1112 = _1059;
                            *(_1112 as *mut uint32_t) = 2;
                            _1113 = _1056;
                            match llvm_lshr_u32(_1113, 5) & 3 {
                                0 => {
                                    current_block = 11798445329271324050;
                                    match current_block {
                                        8417943979444626301 => {
                                            _1055 = 0 as core::ffi::c_int as bool_0;
                                            current_block = 13987004837005789085;
                                        }
                                        831407651183063401 => {
                                            _1115 = _1058;
                                            *(_1115 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                        12558956782898289905 => {
                                            _1116 = _1059;
                                            *(_1116 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                        _ => {
                                            _1114 = _1057;
                                            *(_1114 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                    }
                                }
                                1 => {
                                    current_block = 831407651183063401;
                                    match current_block {
                                        8417943979444626301 => {
                                            _1055 = 0 as core::ffi::c_int as bool_0;
                                            current_block = 13987004837005789085;
                                        }
                                        831407651183063401 => {
                                            _1115 = _1058;
                                            *(_1115 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                        12558956782898289905 => {
                                            _1116 = _1059;
                                            *(_1116 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                        _ => {
                                            _1114 = _1057;
                                            *(_1114 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                    }
                                }
                                2 => {
                                    current_block = 12558956782898289905;
                                    match current_block {
                                        8417943979444626301 => {
                                            _1055 = 0 as core::ffi::c_int as bool_0;
                                            current_block = 13987004837005789085;
                                        }
                                        831407651183063401 => {
                                            _1115 = _1058;
                                            *(_1115 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                        12558956782898289905 => {
                                            _1116 = _1059;
                                            *(_1116 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                        _ => {
                                            _1114 = _1057;
                                            *(_1114 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                    }
                                }
                                3 => {
                                    current_block = 8417943979444626301;
                                    match current_block {
                                        8417943979444626301 => {
                                            _1055 = 0 as core::ffi::c_int as bool_0;
                                            current_block = 13987004837005789085;
                                        }
                                        831407651183063401 => {
                                            _1115 = _1058;
                                            *(_1115 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                        12558956782898289905 => {
                                            _1116 = _1059;
                                            *(_1116 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                        _ => {
                                            _1114 = _1057;
                                            *(_1114 as *mut uint32_t) = 6;
                                            current_block = 17020090522943023520;
                                        }
                                    }
                                }
                                _ => {
                                    current_block = 17020090522943023520;
                                }
                            }
                        }
                        6357002656807115528 => {
                            _1100 = _1066;
                            _1101 = _1057;
                            *(_1101 as *mut uint32_t) = llvm_add_u32(_1100, 2);
                            _1102 = _1058;
                            *(_1102 as *mut uint32_t) = 6;
                            _1103 = _1069;
                            _1104 = _1059;
                            *(_1104 as *mut uint32_t) = llvm_add_u32(_1103, 2);
                            current_block = 17020090522943023520;
                        }
                        7431584555264274889 => {
                            _1105 = _1066;
                            _1106 = _1057;
                            *(_1106 as *mut uint32_t) = llvm_add_u32(_1105, 2);
                            _1107 = _1069;
                            _1108 = _1058;
                            *(_1108 as *mut uint32_t) = llvm_add_u32(_1107, 2);
                            _1109 = _1059;
                            *(_1109 as *mut uint32_t) = 6;
                            current_block = 17020090522943023520;
                        }
                        _ => {
                            _1095 = _1057;
                            *(_1095 as *mut uint32_t) = 6;
                            _1096 = _1069;
                            _1097 = _1058;
                            *(_1097 as *mut uint32_t) = llvm_add_u32(_1096, 2);
                            _1098 = _1066;
                            _1099 = _1059;
                            *(_1099 as *mut uint32_t) = llvm_add_u32(_1098, 2);
                            current_block = 17020090522943023520;
                        }
                    }
                }
                _ => {
                    current_block = 17020090522943023520;
                }
            }
        }
    }
    match current_block {
        17020090522943023520 => {
            _1117 = _1057;
            _1118 = *(_1117 as *mut uint32_t);
            _1119 = _1058;
            _1120 = *(_1119 as *mut uint32_t);
            _1121 = _1059;
            _1122 = *(_1121 as *mut uint32_t);
            _1123 = _1065;
            _1070 = llvm_mul_u32(
                llvm_mul_u32(llvm_mul_u32(_1118, _1120), _1122),
                llvm_add_u32(_1123, 1),
            );
            _1124 = _1063;
            _1125 = _1064;
            _1126 = _1061;
            *(_1126 as *mut uint32_t) =
                llvm_add_u32(llvm_sub_u32(_1124, 2), llvm_mul_u32(6, _1125));
            _1127 = _1065;
            _1128 = _1060;
            *(_1128 as *mut uint8_t) =
                (_1127 != 0 as core::ffi::c_uint) as core::ffi::c_int as bool_0;
            _1129 = _1070;
            _1130 = _1061;
            _1131 = *(_1130 as *mut uint32_t);
            _1132 = _Z25get_ise_sequence_bitcountj12quant_method(_1129, _1131);
            _1133 = _1062;
            *(_1133 as *mut uint32_t) = _1132;
            _1134 = _1070;
            if _1134 <= 64 as core::ffi::c_uint {
                _1135 = _1062;
                _1136 = *(_1135 as *mut uint32_t);
                if _1136 >= 24 as core::ffi::c_uint {
                    _1137 = _1062;
                    _1138 = *(_1137 as *mut uint32_t);
                    _1139__PHI_TEMPORARY =
                        (_1138 <= 96 as core::ffi::c_uint) as core::ffi::c_int as bool_0;
                } else {
                    _1139__PHI_TEMPORARY = 0 as core::ffi::c_int as bool_0;
                }
            } else {
                _1139__PHI_TEMPORARY = 0 as core::ffi::c_int as bool_0;
            }
            _1139 = _1139__PHI_TEMPORARY;
            _1055 = _1139;
        }
        _ => {}
    }
    _1140 = _1055;
    return _1140;
}
#[inline(never)]
unsafe extern "C" fn _ZL20assign_kmeans_texelsR21block_size_descriptor(
    mut _1162: *mut core::ffi::c_void,
) {
    let mut _1163: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1164: uint8_t = 0;
    let mut _1165: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _1166: crate::l_array_216_uint8_t = crate::l_array_216_uint8_t { array: [0; 216] };
    let mut _1167: uint8_t = 0;
    let mut _1168: uint32_t = 0;
    let mut _1169: uint8_t = 0;
    let mut _1170: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1171: uint8_t = 0;
    let mut _1172: uint8_t = 0;
    let mut _1173: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1174: uint8_t = 0;
    let mut _1175: uint8_t = 0;
    let mut _1176: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1177: uint8_t = 0;
    let mut _1178: uint8_t = 0;
    let mut _1179: uint8_t = 0;
    let mut _1180: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1181: uint8_t = 0;
    let mut _1182: uint8_t = 0;
    let mut _1183: uint8_t = 0;
    let mut _1184: uint32_t = 0;
    let mut _1185: uint64_t = 0;
    let mut _1186: uint8_t = 0;
    let mut _1187: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1188: uint8_t = 0;
    let mut _1189: uint8_t = 0;
    let mut _1190: uint8_t = 0;
    let mut _1191: uint8_t = 0;
    let mut _1192: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1193: uint32_t = 0;
    let mut _1194: uint8_t = 0;
    _1163 = _1162;
    _1170 = _1163;
    _1171 = *(&raw mut (*(_1170 as *mut l_struct_struct_OC_block_size_descriptor)).field3
        as *mut uint8_t);
    if _1171 as uint32_t as int32_t <= 64 as core::ffi::c_uint as int32_t {
        _1164 = 0;
        loop {
            _1172 = _1164;
            _1173 = _1163;
            _1174 = *(&raw mut (*(_1173 as *mut l_struct_struct_OC_block_size_descriptor)).field3
                as *mut uint8_t);
            if !((_1172 as uint32_t as int32_t) < _1174 as uint32_t as int32_t) {
                break;
            }
            _1175 = _1164;
            _1176 = _1163;
            _1177 = _1164;
            *(&raw mut *((*(&raw mut (*(_1176 as *mut l_struct_struct_OC_block_size_descriptor))
                .field19 as *mut crate::l_array_64_uint8_t))
                .array)
                .as_mut_ptr()
                .offset(_1177 as uint64_t as int64_t as isize) as *mut uint8_t) = _1175;
            _1178 = _1164;
            _1164 = llvm_add_u8(_1178, 1);
        }
    } else {
        _ZN4astc9rand_initEPm(
            &raw mut *(_1165.array)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as int64_t as isize) as *mut uint64_t
                as *mut core::ffi::c_void,
        );
        _1167 = 0;
        loop {
            _1179 = _1167;
            _1180 = _1163;
            _1181 = *(&raw mut (*(_1180 as *mut l_struct_struct_OC_block_size_descriptor)).field3
                as *mut uint8_t);
            if !((_1179 as uint32_t as int32_t) < _1181 as uint32_t as int32_t) {
                break;
            }
            _1182 = _1167;
            *(&raw mut *(_1166.array)
                .as_mut_ptr()
                .offset(_1182 as uint64_t as int64_t as isize) as *mut uint8_t) = 0;
            _1183 = _1167;
            _1167 = llvm_add_u8(_1183, 1);
        }
        _1168 = 0;
        loop {
            _1184 = _1168;
            if !(_1184 < 64 as core::ffi::c_uint) {
                break;
            }
            _1185 = _ZN4astc4randEPm(
                &raw mut *(_1165.array)
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as int64_t as isize)
                    as *mut uint64_t as *mut core::ffi::c_void,
            );
            _1169 = _1185 as uint8_t;
            _1186 = _1169;
            _1187 = _1163;
            _1188 = *(&raw mut (*(_1187 as *mut l_struct_struct_OC_block_size_descriptor)).field3
                as *mut uint8_t);
            _1169 = llvm_srem_u32(_1186 as uint32_t as int32_t, _1188 as uint32_t as int32_t)
                as uint8_t;
            _1189 = _1169;
            _1190 = *(&raw mut *(_1166.array)
                .as_mut_ptr()
                .offset(_1189 as uint64_t as int64_t as isize)
                as *mut uint8_t);
            if _1190 as core::ffi::c_uint & 1 as core::ffi::c_uint != 0 {
                continue;
            }
            _1191 = _1169;
            _1192 = _1163;
            _1193 = _1168;
            _1168 = llvm_add_u32(_1193, 1);
            *(&raw mut *((*(&raw mut (*(_1192 as *mut l_struct_struct_OC_block_size_descriptor))
                .field19 as *mut crate::l_array_64_uint8_t))
                .array)
                .as_mut_ptr()
                .offset(_1193 as uint64_t as int64_t as isize) as *mut uint8_t) = _1191;
            _1194 = _1169;
            *(&raw mut *(_1166.array)
                .as_mut_ptr()
                .offset(_1194 as uint64_t as int64_t as isize) as *mut uint8_t) = 1;
        }
    };
}
#[inline(never)]
unsafe extern "C" fn _ZN4astcL3maxIhEET_S1_S1_(mut _1210: uint8_t, mut _1211: uint8_t) -> uint8_t {
    let mut _1212: uint8_t = 0;
    let mut _1213: uint8_t = 0;
    let mut _1214: uint8_t = 0;
    let mut _1215: uint8_t = 0;
    let mut _1216: uint8_t = 0;
    let mut _1217: uint8_t = 0;
    let mut _1218: uint8_t = 0;
    let mut _1218__PHI_TEMPORARY: uint8_t = 0;
    _1212 = _1210;
    _1213 = _1211;
    _1214 = _1212;
    _1215 = _1213;
    if _1214 as uint32_t as int32_t > _1215 as uint32_t as int32_t {
        _1216 = _1212;
        _1218__PHI_TEMPORARY = _1216;
    } else {
        _1217 = _1213;
        _1218__PHI_TEMPORARY = _1217;
    }
    _1218 = _1218__PHI_TEMPORARY;
    return _1218;
}
#[inline(never)]
unsafe extern "C" fn _ZL20decode_block_mode_2djRjS_RbS_S_(
    mut _1222: uint32_t,
    mut _1223: *mut core::ffi::c_void,
    mut _1224: *mut core::ffi::c_void,
    mut _1225: *mut core::ffi::c_void,
    mut _1226: *mut core::ffi::c_void,
    mut _1227: *mut core::ffi::c_void,
) -> bool_0 {
    let mut current_block: u64;
    let mut _1228: bool_0 = 0;
    let mut _1229: uint32_t = 0;
    let mut _1230: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1231: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1232: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1233: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1234: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1235: uint32_t = 0;
    let mut _1236: uint32_t = 0;
    let mut _1237: uint32_t = 0;
    let mut _1238: uint32_t = 0;
    let mut _1239: uint32_t = 0;
    let mut _1240: uint32_t = 0;
    let mut _1241: uint32_t = 0;
    let mut _1242: uint32_t = 0;
    let mut _1243: uint32_t = 0;
    let mut _1244: uint32_t = 0;
    let mut _1245: uint32_t = 0;
    let mut _1246: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1247: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1248: uint32_t = 0;
    let mut _1249: uint32_t = 0;
    let mut _1250: uint32_t = 0;
    let mut _1251: uint32_t = 0;
    let mut _1252: uint32_t = 0;
    let mut _1253: uint32_t = 0;
    let mut _1254: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1255: uint32_t = 0;
    let mut _1256: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1257: uint32_t = 0;
    let mut _1258: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1259: uint32_t = 0;
    let mut _1260: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1261: uint32_t = 0;
    let mut _1262: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1263: uint32_t = 0;
    let mut _1264: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1265: uint32_t = 0;
    let mut _1266: uint32_t = 0;
    let mut _1267: uint32_t = 0;
    let mut _1268: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1269: uint32_t = 0;
    let mut _1270: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1271: uint32_t = 0;
    let mut _1272: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1273: uint32_t = 0;
    let mut _1274: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1275: uint32_t = 0;
    let mut _1276: uint32_t = 0;
    let mut _1277: uint32_t = 0;
    let mut _1278: uint32_t = 0;
    let mut _1279: uint32_t = 0;
    let mut _1280: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1281: uint32_t = 0;
    let mut _1282: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1283: uint32_t = 0;
    let mut _1284: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1285: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1286: uint32_t = 0;
    let mut _1287: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1288: uint32_t = 0;
    let mut _1289: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1290: uint32_t = 0;
    let mut _1291: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1292: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1293: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1294: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1295: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1296: uint32_t = 0;
    let mut _1297: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1298: uint32_t = 0;
    let mut _1299: uint32_t = 0;
    let mut _1300: uint32_t = 0;
    let mut _1301: uint32_t = 0;
    let mut _1302: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1303: uint32_t = 0;
    let mut _1304: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1305: uint32_t = 0;
    let mut _1306: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1307: uint32_t = 0;
    let mut _1308: uint32_t = 0;
    let mut _1309: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1310: uint32_t = 0;
    let mut _1311: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1312: uint32_t = 0;
    let mut _1313: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1314: uint32_t = 0;
    let mut _1315: bool_0 = 0;
    let mut _1315__PHI_TEMPORARY: bool_0 = 0;
    let mut _1316: bool_0 = 0;
    _1229 = _1222;
    _1230 = _1223;
    _1231 = _1224;
    _1232 = _1225;
    _1233 = _1226;
    _1234 = _1227;
    _1242 = _1229;
    _1235 = llvm_lshr_u32(_1242, 4) & 1;
    _1243 = _1229;
    _1236 = llvm_lshr_u32(_1243, 9) & 1;
    _1244 = _1229;
    _1237 = llvm_lshr_u32(_1244, 10) & 1;
    _1245 = _1229;
    _1238 = llvm_lshr_u32(_1245, 5) & 3;
    _1246 = _1230;
    *(_1246 as *mut uint32_t) = 0;
    _1247 = _1231;
    *(_1247 as *mut uint32_t) = 0;
    _1248 = _1229;
    if _1248 & 3 != 0 as core::ffi::c_uint {
        _1249 = _1229;
        _1250 = _1235;
        _1235 = _1250 | (_1249 & 3) << 1 as core::ffi::c_int;
        _1251 = _1229;
        _1239 = llvm_lshr_u32(_1251, 7) & 3;
        _1252 = _1229;
        match llvm_lshr_u32(_1252, 2) & 3 {
            0 => {
                current_block = 5709750634026180216;
                match current_block {
                    14576285893972662037 => {
                        _1265 = _1239;
                        _1239 = _1265 & 1;
                        _1266 = _1229;
                        if _1266 & 256 != 0 as core::ffi::c_uint {
                            _1267 = _1239;
                            _1268 = _1230;
                            *(_1268 as *mut uint32_t) = llvm_add_u32(_1267, 2);
                            _1269 = _1238;
                            _1270 = _1231;
                            *(_1270 as *mut uint32_t) = llvm_add_u32(_1269, 2);
                        } else {
                            _1271 = _1238;
                            _1272 = _1230;
                            *(_1272 as *mut uint32_t) = llvm_add_u32(_1271, 2);
                            _1273 = _1239;
                            _1274 = _1231;
                            *(_1274 as *mut uint32_t) = llvm_add_u32(_1273, 6);
                        }
                    }
                    8085028216555051758 => {
                        _1257 = _1239;
                        _1258 = _1230;
                        *(_1258 as *mut uint32_t) = llvm_add_u32(_1257, 8);
                        _1259 = _1238;
                        _1260 = _1231;
                        *(_1260 as *mut uint32_t) = llvm_add_u32(_1259, 2);
                    }
                    6252783108487921354 => {
                        _1261 = _1238;
                        _1262 = _1230;
                        *(_1262 as *mut uint32_t) = llvm_add_u32(_1261, 2);
                        _1263 = _1239;
                        _1264 = _1231;
                        *(_1264 as *mut uint32_t) = llvm_add_u32(_1263, 8);
                    }
                    _ => {
                        _1253 = _1239;
                        _1254 = _1230;
                        *(_1254 as *mut uint32_t) = llvm_add_u32(_1253, 4);
                        _1255 = _1238;
                        _1256 = _1231;
                        *(_1256 as *mut uint32_t) = llvm_add_u32(_1255, 2);
                    }
                }
                current_block = 6557385158750052051;
            }
            1 => {
                current_block = 8085028216555051758;
                match current_block {
                    14576285893972662037 => {
                        _1265 = _1239;
                        _1239 = _1265 & 1;
                        _1266 = _1229;
                        if _1266 & 256 != 0 as core::ffi::c_uint {
                            _1267 = _1239;
                            _1268 = _1230;
                            *(_1268 as *mut uint32_t) = llvm_add_u32(_1267, 2);
                            _1269 = _1238;
                            _1270 = _1231;
                            *(_1270 as *mut uint32_t) = llvm_add_u32(_1269, 2);
                        } else {
                            _1271 = _1238;
                            _1272 = _1230;
                            *(_1272 as *mut uint32_t) = llvm_add_u32(_1271, 2);
                            _1273 = _1239;
                            _1274 = _1231;
                            *(_1274 as *mut uint32_t) = llvm_add_u32(_1273, 6);
                        }
                    }
                    8085028216555051758 => {
                        _1257 = _1239;
                        _1258 = _1230;
                        *(_1258 as *mut uint32_t) = llvm_add_u32(_1257, 8);
                        _1259 = _1238;
                        _1260 = _1231;
                        *(_1260 as *mut uint32_t) = llvm_add_u32(_1259, 2);
                    }
                    6252783108487921354 => {
                        _1261 = _1238;
                        _1262 = _1230;
                        *(_1262 as *mut uint32_t) = llvm_add_u32(_1261, 2);
                        _1263 = _1239;
                        _1264 = _1231;
                        *(_1264 as *mut uint32_t) = llvm_add_u32(_1263, 8);
                    }
                    _ => {
                        _1253 = _1239;
                        _1254 = _1230;
                        *(_1254 as *mut uint32_t) = llvm_add_u32(_1253, 4);
                        _1255 = _1238;
                        _1256 = _1231;
                        *(_1256 as *mut uint32_t) = llvm_add_u32(_1255, 2);
                    }
                }
                current_block = 6557385158750052051;
            }
            2 => {
                current_block = 6252783108487921354;
                match current_block {
                    14576285893972662037 => {
                        _1265 = _1239;
                        _1239 = _1265 & 1;
                        _1266 = _1229;
                        if _1266 & 256 != 0 as core::ffi::c_uint {
                            _1267 = _1239;
                            _1268 = _1230;
                            *(_1268 as *mut uint32_t) = llvm_add_u32(_1267, 2);
                            _1269 = _1238;
                            _1270 = _1231;
                            *(_1270 as *mut uint32_t) = llvm_add_u32(_1269, 2);
                        } else {
                            _1271 = _1238;
                            _1272 = _1230;
                            *(_1272 as *mut uint32_t) = llvm_add_u32(_1271, 2);
                            _1273 = _1239;
                            _1274 = _1231;
                            *(_1274 as *mut uint32_t) = llvm_add_u32(_1273, 6);
                        }
                    }
                    8085028216555051758 => {
                        _1257 = _1239;
                        _1258 = _1230;
                        *(_1258 as *mut uint32_t) = llvm_add_u32(_1257, 8);
                        _1259 = _1238;
                        _1260 = _1231;
                        *(_1260 as *mut uint32_t) = llvm_add_u32(_1259, 2);
                    }
                    6252783108487921354 => {
                        _1261 = _1238;
                        _1262 = _1230;
                        *(_1262 as *mut uint32_t) = llvm_add_u32(_1261, 2);
                        _1263 = _1239;
                        _1264 = _1231;
                        *(_1264 as *mut uint32_t) = llvm_add_u32(_1263, 8);
                    }
                    _ => {
                        _1253 = _1239;
                        _1254 = _1230;
                        *(_1254 as *mut uint32_t) = llvm_add_u32(_1253, 4);
                        _1255 = _1238;
                        _1256 = _1231;
                        *(_1256 as *mut uint32_t) = llvm_add_u32(_1255, 2);
                    }
                }
                current_block = 6557385158750052051;
            }
            3 => {
                current_block = 14576285893972662037;
                match current_block {
                    14576285893972662037 => {
                        _1265 = _1239;
                        _1239 = _1265 & 1;
                        _1266 = _1229;
                        if _1266 & 256 != 0 as core::ffi::c_uint {
                            _1267 = _1239;
                            _1268 = _1230;
                            *(_1268 as *mut uint32_t) = llvm_add_u32(_1267, 2);
                            _1269 = _1238;
                            _1270 = _1231;
                            *(_1270 as *mut uint32_t) = llvm_add_u32(_1269, 2);
                        } else {
                            _1271 = _1238;
                            _1272 = _1230;
                            *(_1272 as *mut uint32_t) = llvm_add_u32(_1271, 2);
                            _1273 = _1239;
                            _1274 = _1231;
                            *(_1274 as *mut uint32_t) = llvm_add_u32(_1273, 6);
                        }
                    }
                    8085028216555051758 => {
                        _1257 = _1239;
                        _1258 = _1230;
                        *(_1258 as *mut uint32_t) = llvm_add_u32(_1257, 8);
                        _1259 = _1238;
                        _1260 = _1231;
                        *(_1260 as *mut uint32_t) = llvm_add_u32(_1259, 2);
                    }
                    6252783108487921354 => {
                        _1261 = _1238;
                        _1262 = _1230;
                        *(_1262 as *mut uint32_t) = llvm_add_u32(_1261, 2);
                        _1263 = _1239;
                        _1264 = _1231;
                        *(_1264 as *mut uint32_t) = llvm_add_u32(_1263, 8);
                    }
                    _ => {
                        _1253 = _1239;
                        _1254 = _1230;
                        *(_1254 as *mut uint32_t) = llvm_add_u32(_1253, 4);
                        _1255 = _1238;
                        _1256 = _1231;
                        *(_1256 as *mut uint32_t) = llvm_add_u32(_1255, 2);
                    }
                }
                current_block = 6557385158750052051;
            }
            _ => {
                current_block = 6557385158750052051;
            }
        }
    } else {
        _1275 = _1229;
        _1276 = _1235;
        _1235 = _1276 | (llvm_lshr_u32(_1275, 2) & 3) << 1 as core::ffi::c_int;
        _1277 = _1229;
        if llvm_lshr_u32(_1277, 2) & 3 == 0 as core::ffi::c_uint {
            _1228 = 0 as core::ffi::c_int as bool_0;
            current_block = 7184762872219971120;
        } else {
            _1278 = _1229;
            _1240 = llvm_lshr_u32(_1278, 9) & 3;
            _1279 = _1229;
            match llvm_lshr_u32(_1279, 7) & 3 {
                0 => {
                    current_block = 4852828230763076236;
                    match current_block {
                        4370218660032574045 => {
                            _1290 = _1229;
                            match llvm_lshr_u32(_1290, 5) & 3 {
                                0 => {
                                    current_block = 15549359616499231096;
                                    match current_block {
                                        10345864283262044410 => {
                                            _1228 = 0 as core::ffi::c_int as bool_0;
                                            current_block = 7184762872219971120;
                                        }
                                        12138441713811679466 => {
                                            _1293 = _1230;
                                            *(_1293 as *mut uint32_t) = 10;
                                            _1294 = _1231;
                                            *(_1294 as *mut uint32_t) = 6;
                                            current_block = 6557385158750052051;
                                        }
                                        _ => {
                                            _1291 = _1230;
                                            *(_1291 as *mut uint32_t) = 6;
                                            _1292 = _1231;
                                            *(_1292 as *mut uint32_t) = 10;
                                            current_block = 6557385158750052051;
                                        }
                                    }
                                }
                                1 => {
                                    current_block = 12138441713811679466;
                                    match current_block {
                                        10345864283262044410 => {
                                            _1228 = 0 as core::ffi::c_int as bool_0;
                                            current_block = 7184762872219971120;
                                        }
                                        12138441713811679466 => {
                                            _1293 = _1230;
                                            *(_1293 as *mut uint32_t) = 10;
                                            _1294 = _1231;
                                            *(_1294 as *mut uint32_t) = 6;
                                            current_block = 6557385158750052051;
                                        }
                                        _ => {
                                            _1291 = _1230;
                                            *(_1291 as *mut uint32_t) = 6;
                                            _1292 = _1231;
                                            *(_1292 as *mut uint32_t) = 10;
                                            current_block = 6557385158750052051;
                                        }
                                    }
                                }
                                2 | 3 => {
                                    current_block = 10345864283262044410;
                                    match current_block {
                                        10345864283262044410 => {
                                            _1228 = 0 as core::ffi::c_int as bool_0;
                                            current_block = 7184762872219971120;
                                        }
                                        12138441713811679466 => {
                                            _1293 = _1230;
                                            *(_1293 as *mut uint32_t) = 10;
                                            _1294 = _1231;
                                            *(_1294 as *mut uint32_t) = 6;
                                            current_block = 6557385158750052051;
                                        }
                                        _ => {
                                            _1291 = _1230;
                                            *(_1291 as *mut uint32_t) = 6;
                                            _1292 = _1231;
                                            *(_1292 as *mut uint32_t) = 10;
                                            current_block = 6557385158750052051;
                                        }
                                    }
                                }
                                _ => {
                                    current_block = 6557385158750052051;
                                }
                            }
                        }
                        1206243784775337676 => {
                            _1283 = _1238;
                            _1284 = _1230;
                            *(_1284 as *mut uint32_t) = llvm_add_u32(_1283, 2);
                            _1285 = _1231;
                            *(_1285 as *mut uint32_t) = 12;
                            current_block = 6557385158750052051;
                        }
                        806864743717811983 => {
                            _1286 = _1238;
                            _1287 = _1230;
                            *(_1287 as *mut uint32_t) = llvm_add_u32(_1286, 6);
                            _1288 = _1240;
                            _1289 = _1231;
                            *(_1289 as *mut uint32_t) = llvm_add_u32(_1288, 6);
                            _1237 = 0;
                            _1236 = 0;
                            current_block = 6557385158750052051;
                        }
                        _ => {
                            _1280 = _1230;
                            *(_1280 as *mut uint32_t) = 12;
                            _1281 = _1238;
                            _1282 = _1231;
                            *(_1282 as *mut uint32_t) = llvm_add_u32(_1281, 2);
                            current_block = 6557385158750052051;
                        }
                    }
                }
                1 => {
                    current_block = 1206243784775337676;
                    match current_block {
                        4370218660032574045 => {
                            _1290 = _1229;
                            match llvm_lshr_u32(_1290, 5) & 3 {
                                0 => {
                                    current_block = 15549359616499231096;
                                    match current_block {
                                        10345864283262044410 => {
                                            _1228 = 0 as core::ffi::c_int as bool_0;
                                            current_block = 7184762872219971120;
                                        }
                                        12138441713811679466 => {
                                            _1293 = _1230;
                                            *(_1293 as *mut uint32_t) = 10;
                                            _1294 = _1231;
                                            *(_1294 as *mut uint32_t) = 6;
                                            current_block = 6557385158750052051;
                                        }
                                        _ => {
                                            _1291 = _1230;
                                            *(_1291 as *mut uint32_t) = 6;
                                            _1292 = _1231;
                                            *(_1292 as *mut uint32_t) = 10;
                                            current_block = 6557385158750052051;
                                        }
                                    }
                                }
                                1 => {
                                    current_block = 12138441713811679466;
                                    match current_block {
                                        10345864283262044410 => {
                                            _1228 = 0 as core::ffi::c_int as bool_0;
                                            current_block = 7184762872219971120;
                                        }
                                        12138441713811679466 => {
                                            _1293 = _1230;
                                            *(_1293 as *mut uint32_t) = 10;
                                            _1294 = _1231;
                                            *(_1294 as *mut uint32_t) = 6;
                                            current_block = 6557385158750052051;
                                        }
                                        _ => {
                                            _1291 = _1230;
                                            *(_1291 as *mut uint32_t) = 6;
                                            _1292 = _1231;
                                            *(_1292 as *mut uint32_t) = 10;
                                            current_block = 6557385158750052051;
                                        }
                                    }
                                }
                                2 | 3 => {
                                    current_block = 10345864283262044410;
                                    match current_block {
                                        10345864283262044410 => {
                                            _1228 = 0 as core::ffi::c_int as bool_0;
                                            current_block = 7184762872219971120;
                                        }
                                        12138441713811679466 => {
                                            _1293 = _1230;
                                            *(_1293 as *mut uint32_t) = 10;
                                            _1294 = _1231;
                                            *(_1294 as *mut uint32_t) = 6;
                                            current_block = 6557385158750052051;
                                        }
                                        _ => {
                                            _1291 = _1230;
                                            *(_1291 as *mut uint32_t) = 6;
                                            _1292 = _1231;
                                            *(_1292 as *mut uint32_t) = 10;
                                            current_block = 6557385158750052051;
                                        }
                                    }
                                }
                                _ => {
                                    current_block = 6557385158750052051;
                                }
                            }
                        }
                        1206243784775337676 => {
                            _1283 = _1238;
                            _1284 = _1230;
                            *(_1284 as *mut uint32_t) = llvm_add_u32(_1283, 2);
                            _1285 = _1231;
                            *(_1285 as *mut uint32_t) = 12;
                            current_block = 6557385158750052051;
                        }
                        806864743717811983 => {
                            _1286 = _1238;
                            _1287 = _1230;
                            *(_1287 as *mut uint32_t) = llvm_add_u32(_1286, 6);
                            _1288 = _1240;
                            _1289 = _1231;
                            *(_1289 as *mut uint32_t) = llvm_add_u32(_1288, 6);
                            _1237 = 0;
                            _1236 = 0;
                            current_block = 6557385158750052051;
                        }
                        _ => {
                            _1280 = _1230;
                            *(_1280 as *mut uint32_t) = 12;
                            _1281 = _1238;
                            _1282 = _1231;
                            *(_1282 as *mut uint32_t) = llvm_add_u32(_1281, 2);
                            current_block = 6557385158750052051;
                        }
                    }
                }
                2 => {
                    current_block = 806864743717811983;
                    match current_block {
                        4370218660032574045 => {
                            _1290 = _1229;
                            match llvm_lshr_u32(_1290, 5) & 3 {
                                0 => {
                                    current_block = 15549359616499231096;
                                    match current_block {
                                        10345864283262044410 => {
                                            _1228 = 0 as core::ffi::c_int as bool_0;
                                            current_block = 7184762872219971120;
                                        }
                                        12138441713811679466 => {
                                            _1293 = _1230;
                                            *(_1293 as *mut uint32_t) = 10;
                                            _1294 = _1231;
                                            *(_1294 as *mut uint32_t) = 6;
                                            current_block = 6557385158750052051;
                                        }
                                        _ => {
                                            _1291 = _1230;
                                            *(_1291 as *mut uint32_t) = 6;
                                            _1292 = _1231;
                                            *(_1292 as *mut uint32_t) = 10;
                                            current_block = 6557385158750052051;
                                        }
                                    }
                                }
                                1 => {
                                    current_block = 12138441713811679466;
                                    match current_block {
                                        10345864283262044410 => {
                                            _1228 = 0 as core::ffi::c_int as bool_0;
                                            current_block = 7184762872219971120;
                                        }
                                        12138441713811679466 => {
                                            _1293 = _1230;
                                            *(_1293 as *mut uint32_t) = 10;
                                            _1294 = _1231;
                                            *(_1294 as *mut uint32_t) = 6;
                                            current_block = 6557385158750052051;
                                        }
                                        _ => {
                                            _1291 = _1230;
                                            *(_1291 as *mut uint32_t) = 6;
                                            _1292 = _1231;
                                            *(_1292 as *mut uint32_t) = 10;
                                            current_block = 6557385158750052051;
                                        }
                                    }
                                }
                                2 | 3 => {
                                    current_block = 10345864283262044410;
                                    match current_block {
                                        10345864283262044410 => {
                                            _1228 = 0 as core::ffi::c_int as bool_0;
                                            current_block = 7184762872219971120;
                                        }
                                        12138441713811679466 => {
                                            _1293 = _1230;
                                            *(_1293 as *mut uint32_t) = 10;
                                            _1294 = _1231;
                                            *(_1294 as *mut uint32_t) = 6;
                                            current_block = 6557385158750052051;
                                        }
                                        _ => {
                                            _1291 = _1230;
                                            *(_1291 as *mut uint32_t) = 6;
                                            _1292 = _1231;
                                            *(_1292 as *mut uint32_t) = 10;
                                            current_block = 6557385158750052051;
                                        }
                                    }
                                }
                                _ => {
                                    current_block = 6557385158750052051;
                                }
                            }
                        }
                        1206243784775337676 => {
                            _1283 = _1238;
                            _1284 = _1230;
                            *(_1284 as *mut uint32_t) = llvm_add_u32(_1283, 2);
                            _1285 = _1231;
                            *(_1285 as *mut uint32_t) = 12;
                            current_block = 6557385158750052051;
                        }
                        806864743717811983 => {
                            _1286 = _1238;
                            _1287 = _1230;
                            *(_1287 as *mut uint32_t) = llvm_add_u32(_1286, 6);
                            _1288 = _1240;
                            _1289 = _1231;
                            *(_1289 as *mut uint32_t) = llvm_add_u32(_1288, 6);
                            _1237 = 0;
                            _1236 = 0;
                            current_block = 6557385158750052051;
                        }
                        _ => {
                            _1280 = _1230;
                            *(_1280 as *mut uint32_t) = 12;
                            _1281 = _1238;
                            _1282 = _1231;
                            *(_1282 as *mut uint32_t) = llvm_add_u32(_1281, 2);
                            current_block = 6557385158750052051;
                        }
                    }
                }
                3 => {
                    current_block = 4370218660032574045;
                    match current_block {
                        4370218660032574045 => {
                            _1290 = _1229;
                            match llvm_lshr_u32(_1290, 5) & 3 {
                                0 => {
                                    current_block = 15549359616499231096;
                                    match current_block {
                                        10345864283262044410 => {
                                            _1228 = 0 as core::ffi::c_int as bool_0;
                                            current_block = 7184762872219971120;
                                        }
                                        12138441713811679466 => {
                                            _1293 = _1230;
                                            *(_1293 as *mut uint32_t) = 10;
                                            _1294 = _1231;
                                            *(_1294 as *mut uint32_t) = 6;
                                            current_block = 6557385158750052051;
                                        }
                                        _ => {
                                            _1291 = _1230;
                                            *(_1291 as *mut uint32_t) = 6;
                                            _1292 = _1231;
                                            *(_1292 as *mut uint32_t) = 10;
                                            current_block = 6557385158750052051;
                                        }
                                    }
                                }
                                1 => {
                                    current_block = 12138441713811679466;
                                    match current_block {
                                        10345864283262044410 => {
                                            _1228 = 0 as core::ffi::c_int as bool_0;
                                            current_block = 7184762872219971120;
                                        }
                                        12138441713811679466 => {
                                            _1293 = _1230;
                                            *(_1293 as *mut uint32_t) = 10;
                                            _1294 = _1231;
                                            *(_1294 as *mut uint32_t) = 6;
                                            current_block = 6557385158750052051;
                                        }
                                        _ => {
                                            _1291 = _1230;
                                            *(_1291 as *mut uint32_t) = 6;
                                            _1292 = _1231;
                                            *(_1292 as *mut uint32_t) = 10;
                                            current_block = 6557385158750052051;
                                        }
                                    }
                                }
                                2 | 3 => {
                                    current_block = 10345864283262044410;
                                    match current_block {
                                        10345864283262044410 => {
                                            _1228 = 0 as core::ffi::c_int as bool_0;
                                            current_block = 7184762872219971120;
                                        }
                                        12138441713811679466 => {
                                            _1293 = _1230;
                                            *(_1293 as *mut uint32_t) = 10;
                                            _1294 = _1231;
                                            *(_1294 as *mut uint32_t) = 6;
                                            current_block = 6557385158750052051;
                                        }
                                        _ => {
                                            _1291 = _1230;
                                            *(_1291 as *mut uint32_t) = 6;
                                            _1292 = _1231;
                                            *(_1292 as *mut uint32_t) = 10;
                                            current_block = 6557385158750052051;
                                        }
                                    }
                                }
                                _ => {
                                    current_block = 6557385158750052051;
                                }
                            }
                        }
                        1206243784775337676 => {
                            _1283 = _1238;
                            _1284 = _1230;
                            *(_1284 as *mut uint32_t) = llvm_add_u32(_1283, 2);
                            _1285 = _1231;
                            *(_1285 as *mut uint32_t) = 12;
                            current_block = 6557385158750052051;
                        }
                        806864743717811983 => {
                            _1286 = _1238;
                            _1287 = _1230;
                            *(_1287 as *mut uint32_t) = llvm_add_u32(_1286, 6);
                            _1288 = _1240;
                            _1289 = _1231;
                            *(_1289 as *mut uint32_t) = llvm_add_u32(_1288, 6);
                            _1237 = 0;
                            _1236 = 0;
                            current_block = 6557385158750052051;
                        }
                        _ => {
                            _1280 = _1230;
                            *(_1280 as *mut uint32_t) = 12;
                            _1281 = _1238;
                            _1282 = _1231;
                            *(_1282 as *mut uint32_t) = llvm_add_u32(_1281, 2);
                            current_block = 6557385158750052051;
                        }
                    }
                }
                _ => {
                    current_block = 6557385158750052051;
                }
            }
        }
    }
    match current_block {
        6557385158750052051 => {
            _1295 = _1230;
            _1296 = *(_1295 as *mut uint32_t);
            _1297 = _1231;
            _1298 = *(_1297 as *mut uint32_t);
            _1299 = _1237;
            _1241 = llvm_mul_u32(llvm_mul_u32(_1296, _1298), llvm_add_u32(_1299, 1));
            _1300 = _1235;
            _1301 = _1236;
            _1302 = _1233;
            *(_1302 as *mut uint32_t) =
                llvm_add_u32(llvm_sub_u32(_1300, 2), llvm_mul_u32(6, _1301));
            _1303 = _1237;
            _1304 = _1232;
            *(_1304 as *mut uint8_t) =
                (_1303 != 0 as core::ffi::c_uint) as core::ffi::c_int as bool_0;
            _1305 = _1241;
            _1306 = _1233;
            _1307 = *(_1306 as *mut uint32_t);
            _1308 = _Z25get_ise_sequence_bitcountj12quant_method(_1305, _1307);
            _1309 = _1234;
            *(_1309 as *mut uint32_t) = _1308;
            _1310 = _1241;
            if _1310 <= 64 as core::ffi::c_uint {
                _1311 = _1234;
                _1312 = *(_1311 as *mut uint32_t);
                if _1312 >= 24 as core::ffi::c_uint {
                    _1313 = _1234;
                    _1314 = *(_1313 as *mut uint32_t);
                    _1315__PHI_TEMPORARY =
                        (_1314 <= 96 as core::ffi::c_uint) as core::ffi::c_int as bool_0;
                } else {
                    _1315__PHI_TEMPORARY = 0 as core::ffi::c_int as bool_0;
                }
            } else {
                _1315__PHI_TEMPORARY = 0 as core::ffi::c_int as bool_0;
            }
            _1315 = _1315__PHI_TEMPORARY;
            _1228 = _1315;
        }
        _ => {}
    }
    _1316 = _1228;
    return _1316;
}
#[inline(never)]
unsafe extern "C" fn _ZL21construct_dt_entry_2djjjjR21block_size_descriptorR23dt_init_working_buffersj(
    mut _1343: uint32_t,
    mut _1344: uint32_t,
    mut _1345: uint32_t,
    mut _1346: uint32_t,
    mut _1347: *mut core::ffi::c_void,
    mut _1348: *mut core::ffi::c_void,
    mut _1349: uint32_t,
) {
    let mut _1350: uint32_t = 0;
    let mut _1351: uint32_t = 0;
    let mut _1352: uint32_t = 0;
    let mut _1353: uint32_t = 0;
    let mut _1354: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1355: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1356: uint32_t = 0;
    let mut _1357: uint32_t = 0;
    let mut _1358: uint8_t = 0;
    let mut _1359: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1360: uint32_t = 0;
    let mut _1361: uint32_t = 0;
    let mut _1362: uint32_t = 0;
    let mut _1363: uint32_t = 0;
    let mut _1364: uint32_t = 0;
    let mut _1365: uint32_t = 0;
    let mut _1366: uint32_t = 0;
    let mut _1367: uint32_t = 0;
    let mut _1368: uint32_t = 0;
    let mut _1369: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1370: uint32_t = 0;
    let mut _1371: uint32_t = 0;
    let mut _1372: uint32_t = 0;
    let mut _1373: uint32_t = 0;
    let mut _1374: uint32_t = 0;
    let mut _1375: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1376: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1377: uint32_t = 0;
    let mut _1378: uint32_t = 0;
    let mut _1379: uint32_t = 0;
    let mut _1380: uint32_t = 0;
    let mut _1381: uint32_t = 0;
    let mut _1382: uint32_t = 0;
    let mut _1383: uint32_t = 0;
    let mut _1384: uint8_t = 0;
    let mut _1385: uint32_t = 0;
    let mut _1386: uint32_t = 0;
    let mut _1387: uint32_t = 0;
    let mut _1388: uint32_t = 0;
    let mut _1389: uint32_t = 0;
    let mut _1390: uint32_t = 0;
    let mut _1391: uint32_t = 0;
    let mut _1392: uint32_t = 0;
    let mut _1393: uint32_t = 0;
    let mut _1394: bool_0 = 0;
    let mut _1394__PHI_TEMPORARY: bool_0 = 0;
    let mut _1395: uint32_t = 0;
    let mut _1396: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1397: uint32_t = 0;
    let mut _1398: uint32_t = 0;
    let mut _1399: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1400: uint32_t = 0;
    let mut _1401: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1402: uint32_t = 0;
    let mut _1403: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1404: uint32_t = 0;
    _1350 = _1343;
    _1351 = _1344;
    _1352 = _1345;
    _1353 = _1346;
    _1354 = _1347;
    _1355 = _1348;
    _1356 = _1349;
    _1365 = _1352;
    _1366 = _1353;
    _1357 = llvm_mul_u32(_1365, _1366);
    _1367 = _1357;
    if _1367 <= 64 as core::ffi::c_uint {
        _1368 = _1357;
        _1358 = (llvm_mul_u32(2, _1368) <= 64 as core::ffi::c_uint) as core::ffi::c_int as bool_0;
        _1369 = _1354;
        _1370 = _1356;
        _1359 = &raw mut *((*(&raw mut (*(_1369 as *mut l_struct_struct_OC_block_size_descriptor))
            .field14
            as *mut crate::l_array_87_struct_AC_l_struct_struct_OC_decimation_info))
            .array)
            .as_mut_ptr()
            .offset(_1370 as uint64_t as int64_t as isize)
            as *mut l_struct_struct_OC_decimation_info as *mut core::ffi::c_void;
        _1371 = _1350;
        _1372 = _1351;
        _1373 = _1352;
        _1374 = _1353;
        _1375 = _1359;
        _1376 = _1355;
        _ZL23init_decimation_info_2djjjjR15decimation_infoR23dt_init_working_buffers(
            _1371, _1372, _1373, _1374, _1375, _1376,
        );
        _1360 = -(1 as core::ffi::c_int) as uint32_t;
        _1361 = -(1 as core::ffi::c_int) as uint32_t;
        _1362 = 0;
        loop {
            _1377 = _1362;
            if !((_1377 as int32_t) < 12 as core::ffi::c_uint as int32_t) {
                break;
            }
            _1378 = _1357;
            _1379 = _1362;
            _1380 = _Z25get_ise_sequence_bitcountj12quant_method(_1378, _1379);
            _1363 = _1380;
            _1381 = _1363;
            if _1381 >= 24 as core::ffi::c_uint {
                _1382 = _1363;
                if _1382 <= 96 as core::ffi::c_uint {
                    _1383 = _1362;
                    _1360 = _1383;
                }
            }
            _1384 = _1358;
            if _1384 as core::ffi::c_uint & 1 as core::ffi::c_uint != 0 {
                _1385 = _1357;
                _1386 = _1362;
                _1387 = _Z25get_ise_sequence_bitcountj12quant_method(llvm_mul_u32(2, _1385), _1386);
                _1364 = _1387;
                _1388 = _1364;
                if _1388 >= 24 as core::ffi::c_uint {
                    _1389 = _1364;
                    if _1389 <= 96 as core::ffi::c_uint {
                        _1390 = _1362;
                        _1361 = _1390;
                    }
                }
            }
            _1391 = _1362;
            _1362 = llvm_add_u32(_1391, 1);
        }
        _1392 = _1360;
        if _1392 as int32_t >= 0 as core::ffi::c_uint as int32_t {
            _1394__PHI_TEMPORARY = 1 as core::ffi::c_int as bool_0;
        } else {
            _1393 = _1361;
            _1394__PHI_TEMPORARY = (_1393 as int32_t >= 0 as core::ffi::c_uint as int32_t)
                as core::ffi::c_int as bool_0;
        }
        _1394 = _1394__PHI_TEMPORARY;
        if _1394 != 0 {
            _1395 = _1360;
            _1396 = _1354;
            _1397 = _1356;
            *(&raw mut (*(&raw mut *((*(&raw mut (*(_1396
                as *mut l_struct_struct_OC_block_size_descriptor))
                .field13
                as *mut crate::l_array_87_struct_AC_l_struct_struct_OC_decimation_mode))
                .array)
                .as_mut_ptr()
                .offset(_1397 as uint64_t as int64_t as isize)
                as *mut l_struct_struct_OC_decimation_mode))
                .field0 as *mut uint8_t) = _1395 as uint8_t;
            _1398 = _1361;
            _1399 = _1354;
            _1400 = _1356;
            *(&raw mut (*(&raw mut *((*(&raw mut (*(_1399
                as *mut l_struct_struct_OC_block_size_descriptor))
                .field13
                as *mut crate::l_array_87_struct_AC_l_struct_struct_OC_decimation_mode))
                .array)
                .as_mut_ptr()
                .offset(_1400 as uint64_t as int64_t as isize)
                as *mut l_struct_struct_OC_decimation_mode))
                .field1 as *mut uint8_t) = _1398 as uint8_t;
            _1401 = _1354;
            _1402 = _1356;
            *(&raw mut (*(&raw mut *((*(&raw mut (*(_1401
                as *mut l_struct_struct_OC_block_size_descriptor))
                .field13
                as *mut crate::l_array_87_struct_AC_l_struct_struct_OC_decimation_mode))
                .array)
                .as_mut_ptr()
                .offset(_1402 as uint64_t as int64_t as isize)
                as *mut l_struct_struct_OC_decimation_mode))
                .field2 as *mut uint16_t) = 0;
            _1403 = _1354;
            _1404 = _1356;
            *(&raw mut (*(&raw mut *((*(&raw mut (*(_1403
                as *mut l_struct_struct_OC_block_size_descriptor))
                .field13
                as *mut crate::l_array_87_struct_AC_l_struct_struct_OC_decimation_mode))
                .array)
                .as_mut_ptr()
                .offset(_1404 as uint64_t as int64_t as isize)
                as *mut l_struct_struct_OC_decimation_mode))
                .field3 as *mut uint16_t) = 0;
            return;
        } else {
            __assert_fail(
                &_OC_str_OC_6 as *const crate::l_array_44_uint8_t as *mut core::ffi::c_void,
                &_OC_str_OC_1 as *const crate::l_array_50_uint8_t as *mut core::ffi::c_void,
                776,
                &__PRETTY_FUNCTION___OC__ZL21construct_dt_entry_2djjjjR21block_size_descriptorR23dt_init_working_buffersj
                    as *const crate::l_array_149_uint8_t as *mut core::ffi::c_void,
            );
        }
    } else {
        __assert_fail(
            &_OC_str_OC_5 as *const crate::l_array_34_uint8_t as *mut core::ffi::c_void,
            &_OC_str_OC_1 as *const crate::l_array_50_uint8_t as *mut core::ffi::c_void,
            748,
            &__PRETTY_FUNCTION___OC__ZL21construct_dt_entry_2djjjjR21block_size_descriptorR23dt_init_working_buffersj
                as *const crate::l_array_149_uint8_t as *mut core::ffi::c_void,
        );
    };
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn _ZN15decimation_mode14set_ref_2planeE12quant_method(
    mut _1425: *mut core::ffi::c_void,
    mut _1426: uint32_t,
) {
    let mut _1427: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1428: uint32_t = 0;
    let mut _1429: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1430: uint32_t = 0;
    let mut _1431: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1432: uint16_t = 0;
    _1427 = _1425;
    _1428 = _1426;
    _1429 = _1427;
    _1430 = _1428;
    _1431 = &raw mut (*(_1429 as *mut l_struct_struct_OC_decimation_mode)).field3 as *mut uint16_t
        as *mut core::ffi::c_void;
    _1432 = *(_1431 as *mut uint16_t);
    *(_1431 as *mut uint16_t) = (_1432 as uint32_t
        | ((1 as core::ffi::c_int) << _1430) as uint16_t as uint32_t)
        as uint16_t;
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn _ZNK10block_mode21get_weight_quant_modeEv(
    mut _1433: *mut core::ffi::c_void,
) -> uint32_t {
    let mut _1434: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1435: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1436: uint8_t = 0;
    _1434 = _1433;
    _1435 = _1434;
    _1436 = *(&raw mut (*(_1435 as *mut l_struct_struct_OC_block_mode)).field2 as *mut uint8_t);
    return _1436 as uint32_t;
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn _ZN15decimation_mode14set_ref_1planeE12quant_method(
    mut _1437: *mut core::ffi::c_void,
    mut _1438: uint32_t,
) {
    let mut _1439: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1440: uint32_t = 0;
    let mut _1441: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1442: uint32_t = 0;
    let mut _1443: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1444: uint16_t = 0;
    _1439 = _1437;
    _1440 = _1438;
    _1441 = _1439;
    _1442 = _1440;
    _1443 = &raw mut (*(_1441 as *mut l_struct_struct_OC_decimation_mode)).field2 as *mut uint16_t
        as *mut core::ffi::c_void;
    _1444 = *(_1443 as *mut uint16_t);
    *(_1443 as *mut uint16_t) =
        (_1444 as uint32_t | ((1 as core::ffi::c_int) << _1442) as uint32_t) as uint16_t;
}
#[inline(never)]
unsafe extern "C" fn _ZL23init_decimation_info_2djjjjR15decimation_infoR23dt_init_working_buffers(
    mut _1445: uint32_t,
    mut _1446: uint32_t,
    mut _1447: uint32_t,
    mut _1448: uint32_t,
    mut _1449: *mut core::ffi::c_void,
    mut _1450: *mut core::ffi::c_void,
) {
    let mut current_block: u64;
    let mut _1451: uint32_t = 0;
    let mut _1452: uint32_t = 0;
    let mut _1453: uint32_t = 0;
    let mut _1454: uint32_t = 0;
    let mut _1455: uint32_t = 0;
    let mut _1456: uint32_t = 0;
    let mut _1457: uint32_t = 0;
    let mut _1458: uint32_t = 0;
    let mut _1459: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1460: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1461: uint32_t = 0;
    let mut _1462: uint32_t = 0;
    let mut _1463: uint8_t = 0;
    let mut _1464: uint32_t = 0;
    let mut _1465: uint32_t = 0;
    let mut _1466: uint32_t = 0;
    let mut _1467: uint32_t = 0;
    let mut _1468: uint32_t = 0;
    let mut _1469: uint32_t = 0;
    let mut _1470: uint32_t = 0;
    let mut _1471: uint32_t = 0;
    let mut _1472: uint32_t = 0;
    let mut _1473: uint32_t = 0;
    let mut _1474: uint32_t = 0;
    let mut _1475: crate::l_array_4_uint32_t = crate::l_array_4_uint32_t { array: [0; 4] };
    let mut _1476: uint32_t = 0;
    let mut _1477: crate::l_array_4_uint32_t = crate::l_array_4_uint32_t { array: [0; 4] };
    let mut _1478: uint32_t = 0;
    let mut _1479: uint8_t = 0;
    let mut _1480: uint32_t = 0;
    let mut _1481: uint32_t = 0;
    let mut _1482: uint32_t = 0;
    let mut _1483: uint32_t = 0;
    let mut _1484: uint32_t = 0;
    let mut _1485: uint32_t = 0;
    let mut _1486: uint8_t = 0;
    let mut _1487: uint32_t = 0;
    let mut _1488: uint8_t = 0;
    let mut _1489: core::ffi::c_float = 0.;
    let mut _1490: uint8_t = 0;
    let mut _1491: uint32_t = 0;
    let mut _1492: uint32_t = 0;
    let mut _1493: uint32_t = 0;
    let mut _1494: uint32_t = 0;
    let mut _1495: uint32_t = 0;
    let mut _1496: uint8_t = 0;
    let mut _1497: uint32_t = 0;
    let mut _1498: uint32_t = 0;
    let mut _1499: uint32_t = 0;
    let mut _1500: uint32_t = 0;
    let mut _1501: uint32_t = 0;
    let mut _1502: uint32_t = 0;
    let mut _1503: uint32_t = 0;
    let mut _1504: uint32_t = 0;
    let mut _1505: uint32_t = 0;
    let mut _1506: uint32_t = 0;
    let mut _1507: uint32_t = 0;
    let mut _1508: uint32_t = 0;
    let mut _1509: uint32_t = 0;
    let mut _1510: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1511: uint32_t = 0;
    let mut _1512: uint32_t = 0;
    let mut _1513: uint32_t = 0;
    let mut _1514: uint32_t = 0;
    let mut _1515: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1516: uint32_t = 0;
    let mut _1517: uint32_t = 0;
    let mut _1518: uint32_t = 0;
    let mut _1519: uint32_t = 0;
    let mut _1520: uint32_t = 0;
    let mut _1521: uint32_t = 0;
    let mut _1522: uint32_t = 0;
    let mut _1523: uint32_t = 0;
    let mut _1524: uint32_t = 0;
    let mut _1525: uint32_t = 0;
    let mut _1526: uint32_t = 0;
    let mut _1527: uint32_t = 0;
    let mut _1528: uint32_t = 0;
    let mut _1529: uint32_t = 0;
    let mut _1530: uint32_t = 0;
    let mut _1531: uint32_t = 0;
    let mut _1532: uint32_t = 0;
    let mut _1533: uint32_t = 0;
    let mut _1534: uint32_t = 0;
    let mut _1535: uint32_t = 0;
    let mut _1536: uint32_t = 0;
    let mut _1537: uint32_t = 0;
    let mut _1538: uint32_t = 0;
    let mut _1539: uint32_t = 0;
    let mut _1540: uint32_t = 0;
    let mut _1541: uint32_t = 0;
    let mut _1542: uint32_t = 0;
    let mut _1543: uint32_t = 0;
    let mut _1544: uint32_t = 0;
    let mut _1545: uint32_t = 0;
    let mut _1546: uint32_t = 0;
    let mut _1547: uint32_t = 0;
    let mut _1548: uint32_t = 0;
    let mut _1549: uint32_t = 0;
    let mut _1550: uint32_t = 0;
    let mut _1551: uint32_t = 0;
    let mut _1552: uint32_t = 0;
    let mut _1553: uint32_t = 0;
    let mut _1554: uint32_t = 0;
    let mut _1555: uint32_t = 0;
    let mut _1556: uint32_t = 0;
    let mut _1557: uint32_t = 0;
    let mut _1558: uint32_t = 0;
    let mut _1559: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1560: uint32_t = 0;
    let mut _1561: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1562: uint32_t = 0;
    let mut _1563: uint8_t = 0;
    let mut _1564: uint32_t = 0;
    let mut _1565: uint32_t = 0;
    let mut _1566: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1567: uint32_t = 0;
    let mut _1568: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1569: uint32_t = 0;
    let mut _1570: uint8_t = 0;
    let mut _1571: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1572: uint32_t = 0;
    let mut _1573: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1574: uint8_t = 0;
    let mut _1575: uint32_t = 0;
    let mut _1576: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1577: uint32_t = 0;
    let mut _1578: uint32_t = 0;
    let mut _1579: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1580: uint32_t = 0;
    let mut _1581: uint32_t = 0;
    let mut _1582: uint8_t = 0;
    let mut _1583: uint32_t = 0;
    let mut _1584: uint32_t = 0;
    let mut _1585: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1586: uint32_t = 0;
    let mut _1587: uint32_t = 0;
    let mut _1588: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1589: uint32_t = 0;
    let mut _1590: uint32_t = 0;
    let mut _1591: uint8_t = 0;
    let mut _1592: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1593: uint32_t = 0;
    let mut _1594: uint32_t = 0;
    let mut _1595: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1596: uint8_t = 0;
    let mut _1597: uint8_t = 0;
    let mut _1598: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1599: uint32_t = 0;
    let mut _1600: uint32_t = 0;
    let mut _1601: uint8_t = 0;
    let mut _1602: uint8_t = 0;
    let mut _1603: uint32_t = 0;
    let mut _1604: uint32_t = 0;
    let mut _1605: uint32_t = 0;
    let mut _1606: uint32_t = 0;
    let mut _1607: uint32_t = 0;
    let mut _1608: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1609: uint32_t = 0;
    let mut _1610: uint8_t = 0;
    let mut _1611: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1612: uint32_t = 0;
    let mut _1613: uint8_t = 0;
    let mut _1614: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1615: uint32_t = 0;
    let mut _1616: uint8_t = 0;
    let mut _1617: uint8_t = 0;
    let mut _1618: uint32_t = 0;
    let mut _1619: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1620: uint32_t = 0;
    let mut _1621: uint8_t = 0;
    let mut _1622: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1623: uint32_t = 0;
    let mut _1624: uint32_t = 0;
    let mut _1625: uint8_t = 0;
    let mut _1626: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1627: uint32_t = 0;
    let mut _1628: uint32_t = 0;
    let mut _1629: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1630: uint32_t = 0;
    let mut _1631: uint32_t = 0;
    let mut _1632: uint8_t = 0;
    let mut _1633: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1634: uint32_t = 0;
    let mut _1635: uint32_t = 0;
    let mut _1636: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1637: uint32_t = 0;
    let mut _1638: uint32_t = 0;
    let mut _1639: uint8_t = 0;
    let mut _1640: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1641: uint32_t = 0;
    let mut _1642: uint32_t = 0;
    let mut _1643: uint32_t = 0;
    let mut _1644: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1645: uint32_t = 0;
    let mut _1646: uint8_t = 0;
    let mut _1647: uint32_t = 0;
    let mut _1648: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1649: uint32_t = 0;
    let mut _1650: uint32_t = 0;
    let mut _1651: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1652: uint32_t = 0;
    let mut _1653: uint32_t = 0;
    let mut _1654: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1655: uint32_t = 0;
    let mut _1656: uint32_t = 0;
    let mut _1657: uint32_t = 0;
    let mut _1658: uint32_t = 0;
    let mut _1659: uint8_t = 0;
    let mut _1660: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1661: uint32_t = 0;
    let mut _1662: uint32_t = 0;
    let mut _1663: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1664: uint32_t = 0;
    let mut _1665: uint8_t = 0;
    let mut _1666: uint32_t = 0;
    let mut _1667: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1668: uint32_t = 0;
    let mut _1669: uint32_t = 0;
    let mut _1670: uint32_t = 0;
    let mut _1671: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1672: uint32_t = 0;
    let mut _1673: uint32_t = 0;
    let mut _1674: uint8_t = 0;
    let mut _1675: uint8_t = 0;
    let mut _1676: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1677: uint32_t = 0;
    let mut _1678: uint32_t = 0;
    let mut _1679: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1680: uint32_t = 0;
    let mut _1681: uint32_t = 0;
    let mut _1682: uint8_t = 0;
    let mut _1683: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1684: uint32_t = 0;
    let mut _1685: uint32_t = 0;
    let mut _1686: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1687: uint32_t = 0;
    let mut _1688: uint32_t = 0;
    let mut _1689: uint32_t = 0;
    let mut _1690: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1691: uint32_t = 0;
    let mut _1692: uint8_t = 0;
    let mut _1693: uint8_t = 0;
    let mut _1694: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1695: uint32_t = 0;
    let mut _1696: uint8_t = 0;
    let mut _1697: core::ffi::c_float = 0.;
    let mut _1698: uint8_t = 0;
    let mut _1699: uint32_t = 0;
    let mut _1700: core::ffi::c_float = 0.;
    let mut _1701: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1702: uint32_t = 0;
    let mut _1703: uint8_t = 0;
    let mut _1704: core::ffi::c_float = 0.;
    let mut _1705: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1706: uint32_t = 0;
    let mut _1707: uint32_t = 0;
    let mut _1708: uint32_t = 0;
    let mut _1709: uint32_t = 0;
    let mut _1710: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1711: uint32_t = 0;
    let mut _1712: uint32_t = 0;
    let mut _1713: uint8_t = 0;
    let mut _1714: uint32_t = 0;
    let mut _1715: uint32_t = 0;
    let mut _1716: uint8_t = 0;
    let mut _1717: uint8_t = 0;
    let mut _1718: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1719: uint32_t = 0;
    let mut _1720: uint32_t = 0;
    let mut _1721: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1722: uint32_t = 0;
    let mut _1723: uint32_t = 0;
    let mut _1724: uint32_t = 0;
    let mut _1725: uint32_t = 0;
    let mut _1726: uint32_t = 0;
    let mut _1727: uint32_t = 0;
    let mut _1728: uint32_t = 0;
    let mut _1729: uint32_t = 0;
    let mut _1730: uint32_t = 0;
    let mut _1731: uint32_t = 0;
    let mut _1732: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1733: uint32_t = 0;
    let mut _1734: uint32_t = 0;
    let mut _1735: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1736: uint32_t = 0;
    let mut _1737: uint32_t = 0;
    let mut _1738: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1739: uint32_t = 0;
    let mut _1740: uint32_t = 0;
    let mut _1741: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1742: uint32_t = 0;
    let mut _1743: uint32_t = 0;
    let mut _1744: uint32_t = 0;
    let mut _1745: uint32_t = 0;
    let mut _1746: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1747: uint32_t = 0;
    let mut _1748: uint8_t = 0;
    let mut _1749: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1750: uint32_t = 0;
    let mut _1751: uint32_t = 0;
    let mut _1752: uint8_t = 0;
    let mut _1753: uint32_t = 0;
    let mut _1754: uint32_t = 0;
    let mut _1755: uint32_t = 0;
    let mut _1756: uint32_t = 0;
    let mut _1757: uint32_t = 0;
    let mut _1758: uint32_t = 0;
    let mut _1759: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1760: uint32_t = 0;
    let mut _1761: uint32_t = 0;
    let mut _1762: uint8_t = 0;
    let mut _1763: uint8_t = 0;
    let mut _1764: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1765: uint32_t = 0;
    let mut _1766: uint32_t = 0;
    let mut _1767: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1768: uint32_t = 0;
    let mut _1769: uint32_t = 0;
    let mut _1770: uint32_t = 0;
    let mut _1771: uint32_t = 0;
    let mut _1772: uint32_t = 0;
    let mut _1773: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1774: uint32_t = 0;
    let mut _1775: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1776: uint32_t = 0;
    let mut _1777: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1778: uint32_t = 0;
    let mut _1779: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1780: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    _1455 = _1445;
    _1456 = _1446;
    _1457 = _1447;
    _1458 = _1448;
    _1459 = _1449;
    _1460 = _1450;
    _1500 = _1455;
    _1501 = _1456;
    _1461 = llvm_mul_u32(_1500, _1501);
    _1502 = _1457;
    _1503 = _1458;
    _1462 = llvm_mul_u32(_1502, _1503);
    _1463 = 0;
    _1504 = _1462;
    if _1504 > 0 as core::ffi::c_uint {
        _1505 = _1461;
        if _1505 > 0 as core::ffi::c_uint {
            _1506 = _1455;
            if _1506 > 0 as core::ffi::c_uint {
                _1507 = _1456;
                if _1507 > 0 as core::ffi::c_uint {
                    _1464 = 0;
                    loop {
                        _1508 = _1464;
                        _1509 = _1462;
                        if !(_1508 < _1509) {
                            break;
                        }
                        _1510 = _1460;
                        _1511 = _1464;
                        *(&raw mut *((*(&raw mut (*(_1510
                            as *mut l_struct_struct_OC_dt_init_working_buffers))
                            .field3
                            as *mut crate::l_array_64_uint8_t))
                            .array)
                            .as_mut_ptr()
                            .offset(_1511 as uint64_t as int64_t as isize)
                            as *mut uint8_t) = 0;
                        _1512 = _1464;
                        _1464 = llvm_add_u32(_1512, 1);
                    }
                    _1465 = 0;
                    loop {
                        _1513 = _1465;
                        _1514 = _1461;
                        if !(_1513 < _1514) {
                            break;
                        }
                        _1515 = _1460;
                        _1516 = _1465;
                        *(&raw mut *((*(&raw mut (*(_1515
                            as *mut l_struct_struct_OC_dt_init_working_buffers))
                            .field0
                            as *mut crate::l_array_216_uint8_t))
                            .array)
                            .as_mut_ptr()
                            .offset(_1516 as uint64_t as int64_t as isize)
                            as *mut uint8_t) = 0;
                        _1517 = _1465;
                        _1465 = llvm_add_u32(_1517, 1);
                    }
                    _1466 = 0;
                    loop {
                        _1518 = _1466;
                        _1519 = _1456;
                        if !(_1518 < _1519) {
                            break;
                        }
                        _1467 = 0;
                        loop {
                            _1520 = _1467;
                            _1521 = _1455;
                            if !(_1520 < _1521) {
                                break;
                            }
                            _1522 = _1466;
                            _1523 = _1455;
                            _1524 = _1467;
                            _1468 = llvm_add_u32(llvm_mul_u32(_1522, _1523), _1524);
                            _1525 = _1455;
                            _1526 = _1455;
                            _1527 = _1467;
                            _1528 = _1457;
                            _1469 = llvm_lshr_u32(
                                llvm_add_u32(
                                    llvm_mul_u32(
                                        llvm_mul_u32(
                                            llvm_udiv_u32(
                                                llvm_add_u32(1024, llvm_udiv_u32(_1525, 2)),
                                                llvm_sub_u32(_1526, 1),
                                            ),
                                            _1527,
                                        ),
                                        llvm_sub_u32(_1528, 1),
                                    ),
                                    32,
                                ),
                                6,
                            );
                            _1529 = _1456;
                            _1530 = _1456;
                            _1531 = _1466;
                            _1532 = _1458;
                            _1470 = llvm_lshr_u32(
                                llvm_add_u32(
                                    llvm_mul_u32(
                                        llvm_mul_u32(
                                            llvm_udiv_u32(
                                                llvm_add_u32(1024, llvm_udiv_u32(_1529, 2)),
                                                llvm_sub_u32(_1530, 1),
                                            ),
                                            _1531,
                                        ),
                                        llvm_sub_u32(_1532, 1),
                                    ),
                                    32,
                                ),
                                6,
                            );
                            _1533 = _1469;
                            _1471 = _1533 & 15;
                            _1534 = _1470;
                            _1472 = _1534 & 15;
                            _1535 = _1469;
                            _1473 = llvm_lshr_u32(_1535, 4);
                            _1536 = _1470;
                            _1474 = llvm_lshr_u32(_1536, 4);
                            _1537 = _1473;
                            _1538 = _1474;
                            _1539 = _1457;
                            *(&raw mut *(_1475.array)
                                .as_mut_ptr()
                                .offset(0 as core::ffi::c_int as int64_t as isize)
                                as *mut uint32_t) = llvm_add_u32(_1537, llvm_mul_u32(_1538, _1539));
                            _1540 = *(&raw mut *(_1475.array)
                                .as_mut_ptr()
                                .offset(0 as core::ffi::c_int as int64_t as isize)
                                as *mut uint32_t);
                            *(&raw mut *(_1475.array)
                                .as_mut_ptr()
                                .offset(1 as core::ffi::c_int as int64_t as isize)
                                as *mut uint32_t) = llvm_add_u32(_1540, 1);
                            _1541 = *(&raw mut *(_1475.array)
                                .as_mut_ptr()
                                .offset(0 as core::ffi::c_int as int64_t as isize)
                                as *mut uint32_t);
                            _1542 = _1457;
                            *(&raw mut *(_1475.array)
                                .as_mut_ptr()
                                .offset(2 as core::ffi::c_int as int64_t as isize)
                                as *mut uint32_t) = llvm_add_u32(_1541, _1542);
                            _1543 = *(&raw mut *(_1475.array)
                                .as_mut_ptr()
                                .offset(2 as core::ffi::c_int as int64_t as isize)
                                as *mut uint32_t);
                            *(&raw mut *(_1475.array)
                                .as_mut_ptr()
                                .offset(3 as core::ffi::c_int as int64_t as isize)
                                as *mut uint32_t) = llvm_add_u32(_1543, 1);
                            _1544 = _1471;
                            _1545 = _1472;
                            _1476 = llvm_mul_u32(_1544, _1545);
                            _1546 = _1476;
                            *(&raw mut *(_1477.array)
                                .as_mut_ptr()
                                .offset(3 as core::ffi::c_int as int64_t as isize)
                                as *mut uint32_t) = llvm_lshr_u32(llvm_add_u32(_1546, 8), 4);
                            _1547 = _1471;
                            _1548 = *(&raw mut *(_1477.array)
                                .as_mut_ptr()
                                .offset(3 as core::ffi::c_int as int64_t as isize)
                                as *mut uint32_t);
                            *(&raw mut *(_1477.array)
                                .as_mut_ptr()
                                .offset(1 as core::ffi::c_int as int64_t as isize)
                                as *mut uint32_t) = llvm_sub_u32(_1547, _1548);
                            _1549 = _1472;
                            _1550 = *(&raw mut *(_1477.array)
                                .as_mut_ptr()
                                .offset(3 as core::ffi::c_int as int64_t as isize)
                                as *mut uint32_t);
                            *(&raw mut *(_1477.array)
                                .as_mut_ptr()
                                .offset(2 as core::ffi::c_int as int64_t as isize)
                                as *mut uint32_t) = llvm_sub_u32(_1549, _1550);
                            _1551 = _1471;
                            _1552 = _1472;
                            _1553 = *(&raw mut *(_1477.array)
                                .as_mut_ptr()
                                .offset(3 as core::ffi::c_int as int64_t as isize)
                                as *mut uint32_t);
                            *(&raw mut *(_1477.array)
                                .as_mut_ptr()
                                .offset(0 as core::ffi::c_int as int64_t as isize)
                                as *mut uint32_t) =
                                llvm_add_u32(llvm_sub_u32(llvm_sub_u32(16, _1551), _1552), _1553);
                            _1478 = 0;
                            loop {
                                _1554 = _1478;
                                if !(_1554 < 4 as core::ffi::c_uint) {
                                    break;
                                }
                                _1555 = _1478;
                                _1556 = *(&raw mut *(_1477.array)
                                    .as_mut_ptr()
                                    .offset(_1555 as uint64_t as int64_t as isize)
                                    as *mut uint32_t);
                                if _1556 != 0 as core::ffi::c_uint {
                                    _1557 = _1478;
                                    _1558 = *(&raw mut *(_1475.array)
                                        .as_mut_ptr()
                                        .offset(_1557 as uint64_t as int64_t as isize)
                                        as *mut uint32_t);
                                    _1559 = _1460;
                                    _1560 = _1468;
                                    _1561 = _1460;
                                    _1562 = _1468;
                                    _1563 = *(&raw mut *((*(&raw mut (*(_1561
                                        as *mut l_struct_struct_OC_dt_init_working_buffers))
                                        .field0
                                        as *mut crate::l_array_216_uint8_t))
                                        .array)
                                        .as_mut_ptr()
                                        .offset(_1562 as uint64_t as int64_t as isize)
                                        as *mut uint8_t);
                                    *(&raw mut *((*(&raw mut *((*(&raw mut (*(_1559
                                        as *mut l_struct_struct_OC_dt_init_working_buffers))
                                        .field1
                                        as *mut crate::l_array_216_struct_AC_l_array_4_uint8_t))
                                        .array)
                                        .as_mut_ptr()
                                        .offset(_1560 as uint64_t as int64_t as isize)
                                        as *mut crate::l_array_4_uint8_t))
                                        .array)
                                        .as_mut_ptr()
                                        .offset(_1563 as uint64_t as int64_t as isize)
                                        as *mut uint8_t) = _1558 as uint8_t;
                                    _1564 = _1478;
                                    _1565 = *(&raw mut *(_1477.array)
                                        .as_mut_ptr()
                                        .offset(_1564 as uint64_t as int64_t as isize)
                                        as *mut uint32_t);
                                    _1566 = _1460;
                                    _1567 = _1468;
                                    _1568 = _1460;
                                    _1569 = _1468;
                                    _1570 = *(&raw mut *((*(&raw mut (*(_1568
                                        as *mut l_struct_struct_OC_dt_init_working_buffers))
                                        .field0
                                        as *mut crate::l_array_216_uint8_t))
                                        .array)
                                        .as_mut_ptr()
                                        .offset(_1569 as uint64_t as int64_t as isize)
                                        as *mut uint8_t);
                                    *(&raw mut *((*(&raw mut *((*(&raw mut (*(_1566
                                        as *mut l_struct_struct_OC_dt_init_working_buffers))
                                        .field2
                                        as *mut crate::l_array_216_struct_AC_l_array_4_uint8_t))
                                        .array)
                                        .as_mut_ptr()
                                        .offset(_1567 as uint64_t as int64_t as isize)
                                        as *mut crate::l_array_4_uint8_t))
                                        .array)
                                        .as_mut_ptr()
                                        .offset(_1570 as uint64_t as int64_t as isize)
                                        as *mut uint8_t) = _1565 as uint8_t;
                                    _1571 = _1460;
                                    _1572 = _1468;
                                    _1573 = &raw mut *((*(&raw mut (*(_1571
                                        as *mut l_struct_struct_OC_dt_init_working_buffers))
                                        .field0
                                        as *mut crate::l_array_216_uint8_t))
                                        .array)
                                        .as_mut_ptr()
                                        .offset(_1572 as uint64_t as int64_t as isize)
                                        as *mut uint8_t
                                        as *mut core::ffi::c_void;
                                    _1574 = *(_1573 as *mut uint8_t);
                                    *(_1573 as *mut uint8_t) = llvm_add_u8(_1574, 1);
                                    _1575 = _1468;
                                    _1576 = _1460;
                                    _1577 = _1478;
                                    _1578 = *(&raw mut *(_1475.array)
                                        .as_mut_ptr()
                                        .offset(_1577 as uint64_t as int64_t as isize)
                                        as *mut uint32_t);
                                    _1579 = _1460;
                                    _1580 = _1478;
                                    _1581 = *(&raw mut *(_1475.array)
                                        .as_mut_ptr()
                                        .offset(_1580 as uint64_t as int64_t as isize)
                                        as *mut uint32_t);
                                    _1582 = *(&raw mut *((*(&raw mut (*(_1579
                                        as *mut l_struct_struct_OC_dt_init_working_buffers))
                                        .field3
                                        as *mut crate::l_array_64_uint8_t))
                                        .array)
                                        .as_mut_ptr()
                                        .offset(_1581 as uint64_t as int64_t as isize)
                                        as *mut uint8_t);
                                    *(&raw mut *((*(&raw mut *((*(&raw mut (*(_1576
                                        as *mut l_struct_struct_OC_dt_init_working_buffers))
                                        .field4
                                        as *mut crate::l_array_64_struct_AC_l_array_216_uint8_t))
                                        .array)
                                        .as_mut_ptr()
                                        .offset(_1578 as uint64_t as int64_t as isize)
                                        as *mut crate::l_array_216_uint8_t))
                                        .array)
                                        .as_mut_ptr()
                                        .offset(_1582 as uint64_t as int64_t as isize)
                                        as *mut uint8_t) = _1575 as uint8_t;
                                    _1583 = _1478;
                                    _1584 = *(&raw mut *(_1477.array)
                                        .as_mut_ptr()
                                        .offset(_1583 as uint64_t as int64_t as isize)
                                        as *mut uint32_t);
                                    _1585 = _1460;
                                    _1586 = _1478;
                                    _1587 = *(&raw mut *(_1475.array)
                                        .as_mut_ptr()
                                        .offset(_1586 as uint64_t as int64_t as isize)
                                        as *mut uint32_t);
                                    _1588 = _1460;
                                    _1589 = _1478;
                                    _1590 = *(&raw mut *(_1475.array)
                                        .as_mut_ptr()
                                        .offset(_1589 as uint64_t as int64_t as isize)
                                        as *mut uint32_t);
                                    _1591 = *(&raw mut *((*(&raw mut (*(_1588
                                        as *mut l_struct_struct_OC_dt_init_working_buffers))
                                        .field3
                                        as *mut crate::l_array_64_uint8_t))
                                        .array)
                                        .as_mut_ptr()
                                        .offset(_1590 as uint64_t as int64_t as isize)
                                        as *mut uint8_t);
                                    *(&raw mut *((*(&raw mut *((*(&raw mut (*(_1585
                                        as *mut l_struct_struct_OC_dt_init_working_buffers))
                                        .field5
                                        as *mut crate::l_array_64_struct_AC_l_array_216_uint8_t))
                                        .array)
                                        .as_mut_ptr()
                                        .offset(_1587 as uint64_t as int64_t as isize)
                                        as *mut crate::l_array_216_uint8_t))
                                        .array)
                                        .as_mut_ptr()
                                        .offset(_1591 as uint64_t as int64_t as isize)
                                        as *mut uint8_t) = _1584 as uint8_t;
                                    _1592 = _1460;
                                    _1593 = _1478;
                                    _1594 = *(&raw mut *(_1475.array)
                                        .as_mut_ptr()
                                        .offset(_1593 as uint64_t as int64_t as isize)
                                        as *mut uint32_t);
                                    _1595 = &raw mut *((*(&raw mut (*(_1592
                                        as *mut l_struct_struct_OC_dt_init_working_buffers))
                                        .field3
                                        as *mut crate::l_array_64_uint8_t))
                                        .array)
                                        .as_mut_ptr()
                                        .offset(_1594 as uint64_t as int64_t as isize)
                                        as *mut uint8_t
                                        as *mut core::ffi::c_void;
                                    _1596 = *(_1595 as *mut uint8_t);
                                    *(_1595 as *mut uint8_t) = llvm_add_u8(_1596, 1);
                                    _1597 = _1463;
                                    _1598 = _1460;
                                    _1599 = _1478;
                                    _1600 = *(&raw mut *(_1475.array)
                                        .as_mut_ptr()
                                        .offset(_1599 as uint64_t as int64_t as isize)
                                        as *mut uint32_t);
                                    _1601 = *(&raw mut *((*(&raw mut (*(_1598
                                        as *mut l_struct_struct_OC_dt_init_working_buffers))
                                        .field3
                                        as *mut crate::l_array_64_uint8_t))
                                        .array)
                                        .as_mut_ptr()
                                        .offset(_1600 as uint64_t as int64_t as isize)
                                        as *mut uint8_t);
                                    _1602 = _ZN4astcL3maxIhEET_S1_S1_(_1597, _1601);
                                    _1463 = _1602;
                                }
                                _1603 = _1478;
                                _1478 = llvm_add_u32(_1603, 1);
                            }
                            _1604 = _1467;
                            _1467 = llvm_add_u32(_1604, 1);
                        }
                        _1605 = _1466;
                        _1466 = llvm_add_u32(_1605, 1);
                    }
                    _1479 = 0;
                    _1480 = 0;
                    loop {
                        _1606 = _1480;
                        _1607 = _1461;
                        if !(_1606 < _1607) {
                            break;
                        }
                        _1608 = _1460;
                        _1609 = _1480;
                        _1610 = *(&raw mut *((*(&raw mut (*(_1608
                            as *mut l_struct_struct_OC_dt_init_working_buffers))
                            .field0
                            as *mut crate::l_array_216_uint8_t))
                            .array)
                            .as_mut_ptr()
                            .offset(_1609 as uint64_t as int64_t as isize)
                            as *mut uint8_t);
                        _1611 = _1459;
                        _1612 = _1480;
                        *(&raw mut *((*(&raw mut (*(_1611
                            as *mut l_struct_struct_OC_decimation_info))
                            .field6
                            as *mut crate::l_array_216_uint8_t))
                            .array)
                            .as_mut_ptr()
                            .offset(_1612 as uint64_t as int64_t as isize)
                            as *mut uint8_t) = _1610;
                        _1613 = _1479;
                        _1614 = _1459;
                        _1615 = _1480;
                        _1616 = *(&raw mut *((*(&raw mut (*(_1614
                            as *mut l_struct_struct_OC_decimation_info))
                            .field6
                            as *mut crate::l_array_216_uint8_t))
                            .array)
                            .as_mut_ptr()
                            .offset(_1615 as uint64_t as int64_t as isize)
                            as *mut uint8_t);
                        _1617 = _ZN4astcL3maxIhEET_S1_S1_(_1613, _1616);
                        _1479 = _1617;
                        _1481 = 0;
                        loop {
                            _1618 = _1481;
                            _1619 = _1460;
                            _1620 = _1480;
                            _1621 = *(&raw mut *((*(&raw mut (*(_1619
                                as *mut l_struct_struct_OC_dt_init_working_buffers))
                                .field0
                                as *mut crate::l_array_216_uint8_t))
                                .array)
                                .as_mut_ptr()
                                .offset(_1620 as uint64_t as int64_t as isize)
                                as *mut uint8_t);
                            if !(_1618 < _1621 as uint32_t) {
                                break;
                            }
                            _1622 = _1460;
                            _1623 = _1480;
                            _1624 = _1481;
                            _1625 = *(&raw mut *((*(&raw mut *((*(&raw mut (*(_1622
                                as *mut l_struct_struct_OC_dt_init_working_buffers))
                                .field2
                                as *mut crate::l_array_216_struct_AC_l_array_4_uint8_t))
                                .array)
                                .as_mut_ptr()
                                .offset(_1623 as uint64_t as int64_t as isize)
                                as *mut crate::l_array_4_uint8_t))
                                .array)
                                .as_mut_ptr()
                                .offset(_1624 as uint64_t as int64_t as isize)
                                as *mut uint8_t);
                            _1626 = _1459;
                            _1627 = _1481;
                            _1628 = _1480;
                            *(&raw mut *((*(&raw mut *((*(&raw mut (*(_1626
                                as *mut l_struct_struct_OC_decimation_info))
                                .field8
                                as *mut crate::l_array_4_struct_AC_l_array_216_uint8_t))
                                .array)
                                .as_mut_ptr()
                                .offset(_1627 as uint64_t as int64_t as isize)
                                as *mut crate::l_array_216_uint8_t))
                                .array)
                                .as_mut_ptr()
                                .offset(_1628 as uint64_t as int64_t as isize)
                                as *mut uint8_t) = _1625;
                            _1629 = _1460;
                            _1630 = _1480;
                            _1631 = _1481;
                            _1632 = *(&raw mut *((*(&raw mut *((*(&raw mut (*(_1629
                                as *mut l_struct_struct_OC_dt_init_working_buffers))
                                .field2
                                as *mut crate::l_array_216_struct_AC_l_array_4_uint8_t))
                                .array)
                                .as_mut_ptr()
                                .offset(_1630 as uint64_t as int64_t as isize)
                                as *mut crate::l_array_4_uint8_t))
                                .array)
                                .as_mut_ptr()
                                .offset(_1631 as uint64_t as int64_t as isize)
                                as *mut uint8_t);
                            _1633 = _1459;
                            _1634 = _1481;
                            _1635 = _1480;
                            *(&raw mut *((*(&raw mut *((*(&raw mut (*(_1633
                                as *mut l_struct_struct_OC_decimation_info))
                                .field9
                                as *mut crate::l_array_4_struct_AC_l_array_216_float))
                                .array)
                                .as_mut_ptr()
                                .offset(_1634 as uint64_t as int64_t as isize)
                                as *mut crate::l_array_216_float))
                                .array)
                                .as_mut_ptr()
                                .offset(_1635 as uint64_t as int64_t as isize)
                                as *mut core::ffi::c_float) = llvm_fmul_f32(
                                _1632 as core::ffi::c_float,
                                0.0625f64 as core::ffi::c_float,
                            );
                            _1636 = _1460;
                            _1637 = _1480;
                            _1638 = _1481;
                            _1639 = *(&raw mut *((*(&raw mut *((*(&raw mut (*(_1636
                                as *mut l_struct_struct_OC_dt_init_working_buffers))
                                .field1
                                as *mut crate::l_array_216_struct_AC_l_array_4_uint8_t))
                                .array)
                                .as_mut_ptr()
                                .offset(_1637 as uint64_t as int64_t as isize)
                                as *mut crate::l_array_4_uint8_t))
                                .array)
                                .as_mut_ptr()
                                .offset(_1638 as uint64_t as int64_t as isize)
                                as *mut uint8_t);
                            _1640 = _1459;
                            _1641 = _1481;
                            _1642 = _1480;
                            *(&raw mut *((*(&raw mut *((*(&raw mut (*(_1640
                                as *mut l_struct_struct_OC_decimation_info))
                                .field7
                                as *mut crate::l_array_4_struct_AC_l_array_216_uint8_t))
                                .array)
                                .as_mut_ptr()
                                .offset(_1641 as uint64_t as int64_t as isize)
                                as *mut crate::l_array_216_uint8_t))
                                .array)
                                .as_mut_ptr()
                                .offset(_1642 as uint64_t as int64_t as isize)
                                as *mut uint8_t) = _1639;
                            _1643 = _1481;
                            _1481 = llvm_add_u32(_1643, 1);
                        }
                        _1644 = _1460;
                        _1645 = _1480;
                        _1646 = *(&raw mut *((*(&raw mut (*(_1644
                            as *mut l_struct_struct_OC_dt_init_working_buffers))
                            .field0
                            as *mut crate::l_array_216_uint8_t))
                            .array)
                            .as_mut_ptr()
                            .offset(_1645 as uint64_t as int64_t as isize)
                            as *mut uint8_t);
                        _1482 = _1646 as uint32_t;
                        loop {
                            _1647 = _1482;
                            if !(_1647 < 4 as core::ffi::c_uint) {
                                break;
                            }
                            _1648 = _1459;
                            _1649 = _1482;
                            _1650 = _1480;
                            *(&raw mut *((*(&raw mut *((*(&raw mut (*(_1648
                                as *mut l_struct_struct_OC_decimation_info))
                                .field8
                                as *mut crate::l_array_4_struct_AC_l_array_216_uint8_t))
                                .array)
                                .as_mut_ptr()
                                .offset(_1649 as uint64_t as int64_t as isize)
                                as *mut crate::l_array_216_uint8_t))
                                .array)
                                .as_mut_ptr()
                                .offset(_1650 as uint64_t as int64_t as isize)
                                as *mut uint8_t) = 0;
                            _1651 = _1459;
                            _1652 = _1482;
                            _1653 = _1480;
                            *(&raw mut *((*(&raw mut *((*(&raw mut (*(_1651
                                as *mut l_struct_struct_OC_decimation_info))
                                .field9
                                as *mut crate::l_array_4_struct_AC_l_array_216_float))
                                .array)
                                .as_mut_ptr()
                                .offset(_1652 as uint64_t as int64_t as isize)
                                as *mut crate::l_array_216_float))
                                .array)
                                .as_mut_ptr()
                                .offset(_1653 as uint64_t as int64_t as isize)
                                as *mut core::ffi::c_float) =
                                0 as core::ffi::c_int as core::ffi::c_float;
                            _1654 = _1459;
                            _1655 = _1482;
                            _1656 = _1480;
                            *(&raw mut *((*(&raw mut *((*(&raw mut (*(_1654
                                as *mut l_struct_struct_OC_decimation_info))
                                .field7
                                as *mut crate::l_array_4_struct_AC_l_array_216_uint8_t))
                                .array)
                                .as_mut_ptr()
                                .offset(_1655 as uint64_t as int64_t as isize)
                                as *mut crate::l_array_216_uint8_t))
                                .array)
                                .as_mut_ptr()
                                .offset(_1656 as uint64_t as int64_t as isize)
                                as *mut uint8_t) = 0;
                            _1657 = _1482;
                            _1482 = llvm_add_u32(_1657, 1);
                        }
                        _1658 = _1480;
                        _1480 = llvm_add_u32(_1658, 1);
                    }
                    _1659 = _1479;
                    _1660 = _1459;
                    *(&raw mut (*(_1660 as *mut l_struct_struct_OC_decimation_info)).field1
                        as *mut uint8_t) = _1659;
                    _1483 = 0;
                    loop {
                        _1661 = _1483;
                        _1662 = _1462;
                        if !(_1661 < _1662) {
                            break;
                        }
                        _1663 = _1460;
                        _1664 = _1483;
                        _1665 = *(&raw mut *((*(&raw mut (*(_1663
                            as *mut l_struct_struct_OC_dt_init_working_buffers))
                            .field3
                            as *mut crate::l_array_64_uint8_t))
                            .array)
                            .as_mut_ptr()
                            .offset(_1664 as uint64_t as int64_t as isize)
                            as *mut uint8_t);
                        _1484 = _1665 as uint32_t;
                        _1666 = _1484;
                        _1667 = _1459;
                        _1668 = _1483;
                        *(&raw mut *((*(&raw mut (*(_1667
                            as *mut l_struct_struct_OC_decimation_info))
                            .field10
                            as *mut crate::l_array_64_uint8_t))
                            .array)
                            .as_mut_ptr()
                            .offset(_1668 as uint64_t as int64_t as isize)
                            as *mut uint8_t) = _1666 as uint8_t;
                        _1485 = 0;
                        loop {
                            _1669 = _1485;
                            _1670 = _1484;
                            if !(_1669 < _1670) {
                                break;
                            }
                            _1671 = _1460;
                            _1672 = _1483;
                            _1673 = _1485;
                            _1674 = *(&raw mut *((*(&raw mut *((*(&raw mut (*(_1671
                                as *mut l_struct_struct_OC_dt_init_working_buffers))
                                .field4
                                as *mut crate::l_array_64_struct_AC_l_array_216_uint8_t))
                                .array)
                                .as_mut_ptr()
                                .offset(_1672 as uint64_t as int64_t as isize)
                                as *mut crate::l_array_216_uint8_t))
                                .array)
                                .as_mut_ptr()
                                .offset(_1673 as uint64_t as int64_t as isize)
                                as *mut uint8_t);
                            _1486 = _1674;
                            _1675 = _1486;
                            _1676 = _1459;
                            _1677 = _1485;
                            _1678 = _1483;
                            *(&raw mut *((*(&raw mut *((*(&raw mut (*(_1676
                                as *mut l_struct_struct_OC_decimation_info))
                                .field11
                                as *mut crate::l_array_216_struct_AC_l_array_64_uint8_t))
                                .array)
                                .as_mut_ptr()
                                .offset(_1677 as uint64_t as int64_t as isize)
                                as *mut crate::l_array_64_uint8_t))
                                .array)
                                .as_mut_ptr()
                                .offset(_1678 as uint64_t as int64_t as isize)
                                as *mut uint8_t) = _1675;
                            _1679 = _1460;
                            _1680 = _1483;
                            _1681 = _1485;
                            _1682 = *(&raw mut *((*(&raw mut *((*(&raw mut (*(_1679
                                as *mut l_struct_struct_OC_dt_init_working_buffers))
                                .field5
                                as *mut crate::l_array_64_struct_AC_l_array_216_uint8_t))
                                .array)
                                .as_mut_ptr()
                                .offset(_1680 as uint64_t as int64_t as isize)
                                as *mut crate::l_array_216_uint8_t))
                                .array)
                                .as_mut_ptr()
                                .offset(_1681 as uint64_t as int64_t as isize)
                                as *mut uint8_t);
                            _1683 = _1459;
                            _1684 = _1485;
                            _1685 = _1483;
                            *(&raw mut *((*(&raw mut *((*(&raw mut (*(_1683
                                as *mut l_struct_struct_OC_decimation_info))
                                .field12
                                as *mut crate::l_array_216_struct_AC_l_array_64_float))
                                .array)
                                .as_mut_ptr()
                                .offset(_1684 as uint64_t as int64_t as isize)
                                as *mut crate::l_array_64_float))
                                .array)
                                .as_mut_ptr()
                                .offset(_1685 as uint64_t as int64_t as isize)
                                as *mut core::ffi::c_float) = _1682 as core::ffi::c_float;
                            _1686 = _1459;
                            _1687 = _1485;
                            _1688 = _1483;
                            *(&raw mut *((*(&raw mut *((*(&raw mut (*(_1686
                                as *mut l_struct_struct_OC_decimation_info))
                                .field13
                                as *mut crate::l_array_216_struct_AC_l_array_64_float))
                                .array)
                                .as_mut_ptr()
                                .offset(_1687 as uint64_t as int64_t as isize)
                                as *mut crate::l_array_64_float))
                                .array)
                                .as_mut_ptr()
                                .offset(_1688 as uint64_t as int64_t as isize)
                                as *mut core::ffi::c_float) =
                                0 as core::ffi::c_int as core::ffi::c_float;
                            _1487 = 0;
                            loop {
                                _1689 = _1487;
                                if !(_1689 < 4 as core::ffi::c_uint) {
                                    current_block = 9458205389579459589;
                                    break;
                                }
                                _1690 = _1459;
                                _1691 = _1487;
                                _1692 = _1486;
                                _1693 = *(&raw mut *((*(&raw mut *((*(&raw mut (*(_1690
                                    as *mut l_struct_struct_OC_decimation_info))
                                    .field7
                                    as *mut crate::l_array_4_struct_AC_l_array_216_uint8_t))
                                    .array)
                                    .as_mut_ptr()
                                    .offset(_1691 as uint64_t as int64_t as isize)
                                    as *mut crate::l_array_216_uint8_t))
                                    .array)
                                    .as_mut_ptr()
                                    .offset(_1692 as uint64_t as int64_t as isize)
                                    as *mut uint8_t);
                                _1488 = _1693;
                                _1694 = _1459;
                                _1695 = _1487;
                                _1696 = _1486;
                                _1697 = *(&raw mut *((*(&raw mut *((*(&raw mut (*(_1694
                                    as *mut l_struct_struct_OC_decimation_info))
                                    .field9
                                    as *mut crate::l_array_4_struct_AC_l_array_216_float))
                                    .array)
                                    .as_mut_ptr()
                                    .offset(_1695 as uint64_t as int64_t as isize)
                                    as *mut crate::l_array_216_float))
                                    .array)
                                    .as_mut_ptr()
                                    .offset(_1696 as uint64_t as int64_t as isize)
                                    as *mut core::ffi::c_float);
                                _1489 = _1697;
                                _1698 = _1488;
                                _1699 = _1483;
                                if _1698 as uint32_t == _1699 {
                                    _1700 = _1489;
                                    if llvm_fcmp_une(
                                        _1700 as core::ffi::c_double,
                                        0 as core::ffi::c_int as core::ffi::c_double,
                                    ) != 0
                                    {
                                        current_block = 3392222251218167844;
                                        break;
                                    }
                                }
                                _1708 = _1487;
                                _1487 = llvm_add_u32(_1708, 1);
                            }
                            match current_block {
                                3392222251218167844 => {
                                    _1701 = _1459;
                                    _1702 = _1487;
                                    _1703 = _1486;
                                    _1704 = *(&raw mut *((*(&raw mut *((*(&raw mut (*(_1701
                                        as *mut l_struct_struct_OC_decimation_info))
                                        .field9
                                        as *mut crate::l_array_4_struct_AC_l_array_216_float))
                                        .array)
                                        .as_mut_ptr()
                                        .offset(_1702 as uint64_t as int64_t as isize)
                                        as *mut crate::l_array_216_float))
                                        .array)
                                        .as_mut_ptr()
                                        .offset(_1703 as uint64_t as int64_t as isize)
                                        as *mut core::ffi::c_float);
                                    _1705 = _1459;
                                    _1706 = _1485;
                                    _1707 = _1483;
                                    *(&raw mut *((*(&raw mut *((*(&raw mut (*(_1705
                                        as *mut l_struct_struct_OC_decimation_info))
                                        .field13
                                        as *mut crate::l_array_216_struct_AC_l_array_64_float))
                                        .array)
                                        .as_mut_ptr()
                                        .offset(_1706 as uint64_t as int64_t as isize)
                                        as *mut crate::l_array_64_float))
                                        .array)
                                        .as_mut_ptr()
                                        .offset(_1707 as uint64_t as int64_t as isize)
                                        as *mut core::ffi::c_float) = _1704;
                                }
                                _ => {}
                            }
                            _1709 = _1485;
                            _1485 = llvm_add_u32(_1709, 1);
                        }
                        _1710 = _1459;
                        _1711 = _1484;
                        _1712 = _1483;
                        _1713 = *(&raw mut *((*(&raw mut *((*(&raw mut (*(_1710
                            as *mut l_struct_struct_OC_decimation_info))
                            .field11
                            as *mut crate::l_array_216_struct_AC_l_array_64_uint8_t))
                            .array)
                            .as_mut_ptr()
                            .offset((llvm_sub_u32
                                as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                _1711, 1
                            ) as uint64_t as int64_t
                                as isize)
                            as *mut crate::l_array_64_uint8_t))
                            .array)
                            .as_mut_ptr()
                            .offset(_1712 as uint64_t as int64_t as isize)
                            as *mut uint8_t);
                        _1490 = _1713;
                        _1714 = _1484;
                        _1491 = _1714;
                        loop {
                            _1715 = _1491;
                            _1716 = _1463;
                            if !(_1715 < _1716 as uint32_t) {
                                break;
                            }
                            _1717 = _1490;
                            _1718 = _1459;
                            _1719 = _1491;
                            _1720 = _1483;
                            *(&raw mut *((*(&raw mut *((*(&raw mut (*(_1718
                                as *mut l_struct_struct_OC_decimation_info))
                                .field11
                                as *mut crate::l_array_216_struct_AC_l_array_64_uint8_t))
                                .array)
                                .as_mut_ptr()
                                .offset(_1719 as uint64_t as int64_t as isize)
                                as *mut crate::l_array_64_uint8_t))
                                .array)
                                .as_mut_ptr()
                                .offset(_1720 as uint64_t as int64_t as isize)
                                as *mut uint8_t) = _1717;
                            _1721 = _1459;
                            _1722 = _1491;
                            _1723 = _1483;
                            *(&raw mut *((*(&raw mut *((*(&raw mut (*(_1721
                                as *mut l_struct_struct_OC_decimation_info))
                                .field12
                                as *mut crate::l_array_216_struct_AC_l_array_64_float))
                                .array)
                                .as_mut_ptr()
                                .offset(_1722 as uint64_t as int64_t as isize)
                                as *mut crate::l_array_64_float))
                                .array)
                                .as_mut_ptr()
                                .offset(_1723 as uint64_t as int64_t as isize)
                                as *mut core::ffi::c_float) =
                                0 as core::ffi::c_int as core::ffi::c_float;
                            _1724 = _1491;
                            _1491 = llvm_add_u32(_1724, 1);
                        }
                        _1725 = _1483;
                        _1483 = llvm_add_u32(_1725, 1);
                    }
                    _1726 = _1461;
                    _1451 = _1726;
                    _1727 = _1451;
                    _1452 = llvm_udiv_u32(llvm_sub_u32(llvm_add_u32(_1727, 4), 1), 4);
                    _1728 = _1452;
                    _1492 = llvm_mul_u32(_1728, 4);
                    _1729 = _1461;
                    _1493 = _1729;
                    loop {
                        _1730 = _1493;
                        _1731 = _1492;
                        if !(_1730 < _1731) {
                            break;
                        }
                        _1732 = _1459;
                        _1733 = _1493;
                        *(&raw mut *((*(&raw mut (*(_1732
                            as *mut l_struct_struct_OC_decimation_info))
                            .field6
                            as *mut crate::l_array_216_uint8_t))
                            .array)
                            .as_mut_ptr()
                            .offset(_1733 as uint64_t as int64_t as isize)
                            as *mut uint8_t) = 0;
                        _1494 = 0;
                        loop {
                            _1734 = _1494;
                            if !(_1734 < 4 as core::ffi::c_uint) {
                                break;
                            }
                            _1735 = _1459;
                            _1736 = _1494;
                            _1737 = _1493;
                            *(&raw mut *((*(&raw mut *((*(&raw mut (*(_1735
                                as *mut l_struct_struct_OC_decimation_info))
                                .field9
                                as *mut crate::l_array_4_struct_AC_l_array_216_float))
                                .array)
                                .as_mut_ptr()
                                .offset(_1736 as uint64_t as int64_t as isize)
                                as *mut crate::l_array_216_float))
                                .array)
                                .as_mut_ptr()
                                .offset(_1737 as uint64_t as int64_t as isize)
                                as *mut core::ffi::c_float) =
                                0 as core::ffi::c_int as core::ffi::c_float;
                            _1738 = _1459;
                            _1739 = _1494;
                            _1740 = _1493;
                            *(&raw mut *((*(&raw mut *((*(&raw mut (*(_1738
                                as *mut l_struct_struct_OC_decimation_info))
                                .field7
                                as *mut crate::l_array_4_struct_AC_l_array_216_uint8_t))
                                .array)
                                .as_mut_ptr()
                                .offset(_1739 as uint64_t as int64_t as isize)
                                as *mut crate::l_array_216_uint8_t))
                                .array)
                                .as_mut_ptr()
                                .offset(_1740 as uint64_t as int64_t as isize)
                                as *mut uint8_t) = 0;
                            _1741 = _1459;
                            _1742 = _1494;
                            _1743 = _1493;
                            *(&raw mut *((*(&raw mut *((*(&raw mut (*(_1741
                                as *mut l_struct_struct_OC_decimation_info))
                                .field8
                                as *mut crate::l_array_4_struct_AC_l_array_216_uint8_t))
                                .array)
                                .as_mut_ptr()
                                .offset(_1742 as uint64_t as int64_t as isize)
                                as *mut crate::l_array_216_uint8_t))
                                .array)
                                .as_mut_ptr()
                                .offset(_1743 as uint64_t as int64_t as isize)
                                as *mut uint8_t) = 0;
                            _1744 = _1494;
                            _1494 = llvm_add_u32(_1744, 1);
                        }
                        _1745 = _1493;
                        _1493 = llvm_add_u32(_1745, 1);
                    }
                    _1746 = _1460;
                    _1747 = _1462;
                    _1748 = *(&raw mut *((*(&raw mut (*(_1746
                        as *mut l_struct_struct_OC_dt_init_working_buffers))
                        .field3
                        as *mut crate::l_array_64_uint8_t))
                        .array)
                        .as_mut_ptr()
                        .offset((llvm_sub_u32
                            as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            _1747, 1
                        ) as uint64_t as int64_t as isize)
                        as *mut uint8_t);
                    _1495 = _1748 as uint32_t;
                    _1749 = _1459;
                    _1750 = _1495;
                    _1751 = _1462;
                    _1752 = *(&raw mut *((*(&raw mut *((*(&raw mut (*(_1749
                        as *mut l_struct_struct_OC_decimation_info))
                        .field11
                        as *mut crate::l_array_216_struct_AC_l_array_64_uint8_t))
                        .array)
                        .as_mut_ptr()
                        .offset((llvm_sub_u32
                            as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            _1750, 1
                        ) as uint64_t as int64_t as isize)
                        as *mut crate::l_array_64_uint8_t))
                        .array)
                        .as_mut_ptr()
                        .offset((llvm_sub_u32
                            as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            _1751, 1
                        ) as uint64_t as int64_t as isize)
                        as *mut uint8_t);
                    _1496 = _1752;
                    _1753 = _1462;
                    _1453 = _1753;
                    _1754 = _1453;
                    _1454 = llvm_udiv_u32(llvm_sub_u32(llvm_add_u32(_1754, 4), 1), 4);
                    _1755 = _1454;
                    _1497 = llvm_mul_u32(_1755, 4);
                    _1756 = _1462;
                    _1498 = _1756;
                    loop {
                        _1757 = _1498;
                        _1758 = _1497;
                        if !(_1757 < _1758) {
                            break;
                        }
                        _1759 = _1459;
                        _1760 = _1498;
                        *(&raw mut *((*(&raw mut (*(_1759
                            as *mut l_struct_struct_OC_decimation_info))
                            .field10
                            as *mut crate::l_array_64_uint8_t))
                            .array)
                            .as_mut_ptr()
                            .offset(_1760 as uint64_t as int64_t as isize)
                            as *mut uint8_t) = 0;
                        _1499 = 0;
                        loop {
                            _1761 = _1499;
                            _1762 = _1463;
                            if !(_1761 < _1762 as uint32_t) {
                                break;
                            }
                            _1763 = _1496;
                            _1764 = _1459;
                            _1765 = _1499;
                            _1766 = _1498;
                            *(&raw mut *((*(&raw mut *((*(&raw mut (*(_1764
                                as *mut l_struct_struct_OC_decimation_info))
                                .field11
                                as *mut crate::l_array_216_struct_AC_l_array_64_uint8_t))
                                .array)
                                .as_mut_ptr()
                                .offset(_1765 as uint64_t as int64_t as isize)
                                as *mut crate::l_array_64_uint8_t))
                                .array)
                                .as_mut_ptr()
                                .offset(_1766 as uint64_t as int64_t as isize)
                                as *mut uint8_t) = _1763;
                            _1767 = _1459;
                            _1768 = _1499;
                            _1769 = _1498;
                            *(&raw mut *((*(&raw mut *((*(&raw mut (*(_1767
                                as *mut l_struct_struct_OC_decimation_info))
                                .field12
                                as *mut crate::l_array_216_struct_AC_l_array_64_float))
                                .array)
                                .as_mut_ptr()
                                .offset(_1768 as uint64_t as int64_t as isize)
                                as *mut crate::l_array_64_float))
                                .array)
                                .as_mut_ptr()
                                .offset(_1769 as uint64_t as int64_t as isize)
                                as *mut core::ffi::c_float) =
                                0 as core::ffi::c_int as core::ffi::c_float;
                            _1770 = _1499;
                            _1499 = llvm_add_u32(_1770, 1);
                        }
                        _1771 = _1498;
                        _1498 = llvm_add_u32(_1771, 1);
                    }
                    _1772 = _1461;
                    _1773 = _1459;
                    *(&raw mut (*(_1773 as *mut l_struct_struct_OC_decimation_info)).field0
                        as *mut uint8_t) = _1772 as uint8_t;
                    _1774 = _1462;
                    _1775 = _1459;
                    *(&raw mut (*(_1775 as *mut l_struct_struct_OC_decimation_info)).field2
                        as *mut uint8_t) = _1774 as uint8_t;
                    _1776 = _1457;
                    _1777 = _1459;
                    *(&raw mut (*(_1777 as *mut l_struct_struct_OC_decimation_info)).field3
                        as *mut uint8_t) = _1776 as uint8_t;
                    _1778 = _1458;
                    _1779 = _1459;
                    *(&raw mut (*(_1779 as *mut l_struct_struct_OC_decimation_info)).field4
                        as *mut uint8_t) = _1778 as uint8_t;
                    _1780 = _1459;
                    *(&raw mut (*(_1780 as *mut l_struct_struct_OC_decimation_info)).field5
                        as *mut uint8_t) = 1;
                    return;
                } else {
                    __assert_fail(
                        &_OC_str_OC_8 as *const crate::l_array_13_uint8_t as *mut core::ffi::c_void,
                        &_OC_str_OC_1 as *const crate::l_array_50_uint8_t as *mut core::ffi::c_void,
                        268,
                        &__PRETTY_FUNCTION___OC__ZL23init_decimation_info_2djjjjR15decimation_infoR23dt_init_working_buffers
                            as *const crate::l_array_131_uint8_t as *mut core::ffi::c_void,
                    );
                }
            } else {
                __assert_fail(
                    &_OC_str_OC_7 as *const crate::l_array_13_uint8_t as *mut core::ffi::c_void,
                    &_OC_str_OC_1 as *const crate::l_array_50_uint8_t as *mut core::ffi::c_void,
                    267,
                    &__PRETTY_FUNCTION___OC__ZL23init_decimation_info_2djjjjR15decimation_infoR23dt_init_working_buffers
                        as *const crate::l_array_131_uint8_t as *mut core::ffi::c_void,
                );
            }
        } else {
            __assert_fail(
                &_OC_str_OC_2 as *const crate::l_array_21_uint8_t as *mut core::ffi::c_void,
                &_OC_str_OC_1 as *const crate::l_array_50_uint8_t as *mut core::ffi::c_void,
                266,
                &__PRETTY_FUNCTION___OC__ZL23init_decimation_info_2djjjjR15decimation_infoR23dt_init_working_buffers
                    as *const crate::l_array_131_uint8_t as *mut core::ffi::c_void,
            );
        }
    } else {
        __assert_fail(
            &_OC_str as *const crate::l_array_22_uint8_t as *mut core::ffi::c_void,
            &_OC_str_OC_1 as *const crate::l_array_50_uint8_t as *mut core::ffi::c_void,
            265,
            &__PRETTY_FUNCTION___OC__ZL23init_decimation_info_2djjjjR15decimation_infoR23dt_init_working_buffers
                as *const crate::l_array_131_uint8_t as *mut core::ffi::c_void,
        );
    };
}
