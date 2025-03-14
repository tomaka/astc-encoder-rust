use crate::*;

pub type __int8_t = core::ffi::c_schar;
pub type __uint8_t = core::ffi::c_uchar;
pub type __uint16_t = core::ffi::c_ushort;
pub type __int32_t = core::ffi::c_int;
pub type __uint32_t = core::ffi::c_uint;
pub type __int64_t = core::ffi::c_long;
pub type __uint64_t = core::ffi::c_ulong;
pub type int8_t = __int8_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type bool_0 = core::ffi::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_4_float {
    pub array: [core::ffi::c_float; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_struct_struct_OC_vfloat4 {
    pub field0: crate::l_array_4_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_4_struct_AC_l_struct_struct_OC_vfloat4 {
    pub array: [l_struct_struct_OC_vfloat4; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_struct_struct_OC_endpoints {
    pub field0: uint32_t,
    pub field1: crate::l_array_4_struct_AC_l_struct_struct_OC_vfloat4,
    pub field2: crate::l_array_4_struct_AC_l_struct_struct_OC_vfloat4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_216_float {
    pub array: [core::ffi::c_float; 216],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_struct_struct_OC_endpoints_and_weights {
    pub field0: uint8_t,
    pub field1: l_struct_struct_OC_endpoints,
    pub field2: crate::l_array_216_float,
    pub field3: crate::l_array_216_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_5568_float {
    pub array: [core::ffi::c_float; 5568],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_131072_uint8_t {
    pub array: [uint8_t; 131072],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_2048_float {
    pub array: [core::ffi::c_float; 2048],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_2048_uint8_t {
    pub array: [uint8_t; 2048],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_4_uint8_t {
    pub array: [uint8_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_2048_struct_AC_l_array_4_uint8_t {
    pub array: [l_array_4_uint8_t; 2048],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_8_float {
    pub array: [core::ffi::c_float; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_87_struct_AC_l_array_8_float {
    pub array: [l_array_8_float; 87],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_struct_struct_OC_compression_working_buffers {
    pub field0: l_struct_struct_OC_endpoints_and_weights,
    pub field1: l_struct_struct_OC_endpoints_and_weights,
    pub field2: crate::l_array_5568_float,
    pub field3: crate::l_array_131072_uint8_t,
    pub field4: crate::l_array_2048_float,
    pub field5: crate::l_array_2048_uint8_t,
    pub field6: crate::l_array_2048_uint8_t,
    pub field7: crate::l_array_2048_struct_AC_l_array_4_uint8_t,
    pub field8: crate::l_array_2048_uint8_t,
    pub field9: crate::l_array_2048_float,
    pub field10: crate::l_array_2048_float,
    pub field11: crate::l_array_2048_float,
    pub field12: crate::l_array_87_struct_AC_l_array_8_float,
    pub field13: crate::l_array_87_struct_AC_l_array_8_float,
    pub field14: crate::l_array_2048_float,
    pub field15: crate::l_array_2048_float,
    pub field16: crate::l_array_87_struct_AC_l_array_8_float,
    pub field17: crate::l_array_87_struct_AC_l_array_8_float,
}
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
pub struct l_struct_struct_OC_vmask4 {
    pub field0: crate::l_array_4_uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_struct_struct_OC_vint4 {
    pub field0: crate::l_array_4_uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_32_float {
    pub array: [core::ffi::c_float; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_64_struct_AC_l_array_32_float {
    pub array: [l_array_32_float; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_25_uint8_t {
    pub array: [uint8_t; 25],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_51_uint8_t {
    pub array: [uint8_t; 51],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_135_uint8_t {
    pub array: [uint8_t; 135],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_20_uint8_t {
    pub array: [uint8_t; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_18_uint8_t {
    pub array: [uint8_t; 18],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_39_uint8_t {
    pub array: [uint8_t; 39],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_130_uint8_t {
    pub array: [uint8_t; 130],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_12_uint8_t {
    pub array: [uint8_t; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_109_uint8_t {
    pub array: [uint8_t; 109],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_22_uint8_t {
    pub array: [uint8_t; 22],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_17_uint8_t {
    pub array: [uint8_t; 17],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_81_uint8_t {
    pub array: [uint8_t; 81],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_154_uint8_t {
    pub array: [uint8_t; 154],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_34_uint8_t {
    pub array: [uint8_t; 34],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_54_uint8_t {
    pub array: [uint8_t; 54],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_23_uint8_t {
    pub array: [uint8_t; 23],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_32_uint32_t {
    pub array: [uint32_t; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_36_struct_AC_l_struct_struct_OC_vfloat4 {
    pub array: [l_struct_struct_OC_vfloat4; 36],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_64_uint32_t {
    pub array: [uint32_t; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub data: crate::l_array_2_uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub data: crate::l_array_2_uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub data: crate::l_array_2_uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub data: crate::l_array_2_uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub data: crate::l_array_2_uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub data: crate::l_array_2_uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub data: crate::l_array_2_uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub data: crate::l_array_2_uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub data: crate::l_array_2_uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub data: crate::l_array_2_uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub data: crate::l_array_2_uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub data: crate::l_array_2_uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub data: crate::l_array_2_uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub data: crate::l_array_2_uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub data: crate::l_array_2_uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub data: crate::l_array_2_uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub data: crate::l_array_2_uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub data: crate::l_array_2_uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub data: crate::l_array_2_uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub data: crate::l_array_2_uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub data: crate::l_array_2_uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub data: crate::l_array_2_uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
    pub data: crate::l_array_2_uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub data: crate::l_array_2_uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
    pub data: crate::l_array_2_uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    pub data: crate::l_array_2_uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_25 {
    pub data: crate::l_array_2_uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub data: crate::l_array_2_uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_27 {
    pub data: crate::l_array_2_uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_28 {
    pub data: crate::l_array_2_uint64_t,
}
#[inline(always)]
unsafe extern "C" fn llvm_fcmp_ogt(
    mut X: core::ffi::c_double,
    mut Y: core::ffi::c_double,
) -> core::ffi::c_int {
    return (X > Y) as core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn llvm_fcmp_olt(
    mut X: core::ffi::c_double,
    mut Y: core::ffi::c_double,
) -> core::ffi::c_int {
    return (X < Y) as core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn llvm_fcmp_oeq(
    mut X: core::ffi::c_double,
    mut Y: core::ffi::c_double,
) -> core::ffi::c_int {
    return (X == Y) as core::ffi::c_int;
}
static mut _ZL9sin_table: crate::l_array_64_struct_AC_l_array_32_float =
    crate::l_array_64_struct_AC_l_array_32_float {
        array: [l_array_32_float { array: [0.; 32] }; 64],
    };
static mut _ZL9cos_table: crate::l_array_64_struct_AC_l_array_32_float =
    crate::l_array_64_struct_AC_l_array_32_float {
        array: [l_array_32_float { array: [0.; 32] }; 64],
    };
static mut _OC_str: crate::l_array_25_uint8_t = unsafe {
    {
        let mut init = crate::l_array_25_uint8_t {
            array: *::core::mem::transmute::<&[u8; 25], &mut [uint8_t; 25]>(
                b"max_decimation_modes > 0\0",
            ),
        };
        init
    }
};
static mut _OC_str_OC_1: crate::l_array_51_uint8_t = unsafe {
    {
        let mut init = crate::l_array_51_uint8_t {
            array: *::core::mem::transmute::<&[u8; 51], &mut [uint8_t; 51]>(
                b"/root/astc-encoder/Source/astcenc_weight_align.cpp\0",
            ),
        };
        init
    }
};
static mut __PRETTY_FUNCTION___OC__Z32compute_angular_endpoints_1planebRK21block_size_descriptorPKfjR27compression_working_buffers: crate::l_array_135_uint8_t = unsafe {
    {
        let mut init = crate::l_array_135_uint8_t {
            array: *::core::mem::transmute::<
                &[u8; 135],
                &mut [uint8_t; 135],
            >(
                b"void compute_angular_endpoints_1plane(bool, const block_size_descriptor &, const float *, unsigned int, compression_working_buffers &)\0",
            ),
        };
        init
    }
};
static mut _OC_str_OC_2: crate::l_array_20_uint8_t = unsafe {
    {
        let mut init = crate::l_array_20_uint8_t {
            array: *::core::mem::transmute::<&[u8; 20], &mut [uint8_t; 20]>(
                b"max_block_modes > 0\0",
            ),
        };
        init
    }
};
static mut _OC_str_OC_3: crate::l_array_18_uint8_t = unsafe {
    {
        let mut init = crate::l_array_18_uint8_t {
            array: *::core::mem::transmute::<&[u8; 18], &mut [uint8_t; 18]>(b"!bm.is_dual_plane\0"),
        };
        init
    }
};
static mut _OC_str_OC_4: crate::l_array_39_uint8_t = unsafe {
    {
        let mut init = crate::l_array_39_uint8_t {
            array: *::core::mem::transmute::<&[u8; 39], &mut [uint8_t; 39]>(
                b"bsd.decimation_mode_count_selected > 0\0",
            ),
        };
        init
    }
};
static mut __PRETTY_FUNCTION___OC__Z33compute_angular_endpoints_2planesRK21block_size_descriptorPKfjR27compression_working_buffers: crate::l_array_130_uint8_t = unsafe {
    {
        let mut init = crate::l_array_130_uint8_t {
            array: *::core::mem::transmute::<
                &[u8; 130],
                &mut [uint8_t; 130],
            >(
                b"void compute_angular_endpoints_2planes(const block_size_descriptor &, const float *, unsigned int, compression_working_buffers &)\0",
            ),
        };
        init
    }
};
static mut _ZL21steps_for_quant_level: crate::l_array_12_uint8_t = unsafe {
    {
        let mut init = crate::l_array_12_uint8_t {
            array: *::core::mem::transmute::<&[u8; 12], &mut [uint8_t; 12]>(
                b"\x02\x03\x04\x05\x06\x08\n\x0C\x10\x14\x18 ",
            ),
        };
        init
    }
};
static mut _OC_str_OC_5: crate::l_array_20_uint8_t = unsafe {
    {
        let mut init = crate::l_array_20_uint8_t {
            array: *::core::mem::transmute::<&[u8; 20], &mut [uint8_t; 20]>(
                b"max_quant_steps > 0\0",
            ),
        };
        init
    }
};
static mut __PRETTY_FUNCTION___OC__ZL42compute_angular_endpoints_for_quant_levelsjPKfjPfS1_:
    crate::l_array_109_uint8_t = unsafe {
    {
        let mut init = crate::l_array_109_uint8_t {
            array: *::core::mem::transmute::<
                &[u8; 109],
                &mut [uint8_t; 109],
            >(
                b"void compute_angular_endpoints_for_quant_levels(unsigned int, const float *, unsigned int, float *, float *)\0",
            ),
        };
        init
    }
};
static mut _OC_str_OC_6: crate::l_array_22_uint8_t = unsafe {
    {
        let mut init = crate::l_array_22_uint8_t {
            array: *::core::mem::transmute::<&[u8; 22], &mut [uint8_t; 22]>(
                b"max_angular_steps > 0\0",
            ),
        };
        init
    }
};
static mut _OC_str_OC_7: crate::l_array_17_uint8_t = unsafe {
    {
        let mut init = crate::l_array_17_uint8_t {
            array: *::core::mem::transmute::<&[u8; 17], &mut [uint8_t; 17]>(b"weight_count > 0\0"),
        };
        init
    }
};
static mut __PRETTY_FUNCTION___OC__ZL23compute_angular_offsetsjPKfjPf: crate::l_array_81_uint8_t = unsafe {
    {
        let mut init = crate::l_array_81_uint8_t {
            array: *::core::mem::transmute::<
                &[u8; 81],
                &mut [uint8_t; 81],
            >(
                b"void compute_angular_offsets(unsigned int, const float *, unsigned int, float *)\0",
            ),
        };
        init
    }
};
static mut __PRETTY_FUNCTION___OC__ZL33compute_lowest_and_highest_weightjPKfjjS0_PfPiS1_S1_S1_:
    crate::l_array_154_uint8_t = unsafe {
    {
        let mut init = crate::l_array_154_uint8_t {
            array: *::core::mem::transmute::<
                &[u8; 154],
                &mut [uint8_t; 154],
            >(
                b"void compute_lowest_and_highest_weight(unsigned int, const float *, unsigned int, unsigned int, const float *, float *, int *, float *, float *, float *)\0",
            ),
        };
        init
    }
};
static mut _OC_str_OC_8: crate::l_array_34_uint8_t = unsafe {
    {
        let mut init = crate::l_array_34_uint8_t {
            array: *::core::mem::transmute::<&[u8; 34], &mut [uint8_t; 34]>(
                b"std::fegetround() == FE_TONEAREST\0",
            ),
        };
        init
    }
};
static mut _OC_str_OC_9: crate::l_array_54_uint8_t = unsafe {
    {
        let mut init = crate::l_array_54_uint8_t {
            array: *::core::mem::transmute::<&[u8; 54], &mut [uint8_t; 54]>(
                b"/root/astc-encoder/Source/astcenc_vecmathlib_none_4.h\0",
            ),
        };
        init
    }
};
static mut __PRETTY_FUNCTION___OC__Z5round7vfloat4: crate::l_array_23_uint8_t = unsafe {
    {
        let mut init = crate::l_array_23_uint8_t {
            array: *::core::mem::transmute::<&[u8; 23], &mut [uint8_t; 23]>(
                b"vfloat4 round(vfloat4)\0",
            ),
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
unsafe extern "C" fn llvm_add_u32(mut a: uint32_t, mut b: uint32_t) -> uint32_t {
    let mut r: uint32_t = a.wrapping_add(b);
    return r;
}
#[inline(always)]
unsafe extern "C" fn llvm_fadd_f32(
    mut a: core::ffi::c_float,
    mut b: core::ffi::c_float,
) -> core::ffi::c_float {
    let mut r: core::ffi::c_float = a + b;
    return r;
}
#[inline(always)]
unsafe extern "C" fn llvm_sub_u32(mut a: uint32_t, mut b: uint32_t) -> uint32_t {
    let mut r: uint32_t = a.wrapping_sub(b);
    return r;
}
#[inline(always)]
unsafe extern "C" fn llvm_fsub_f32(
    mut a: core::ffi::c_float,
    mut b: core::ffi::c_float,
) -> core::ffi::c_float {
    let mut r: core::ffi::c_float = a - b;
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
unsafe extern "C" fn llvm_fdiv_f32(
    mut a: core::ffi::c_float,
    mut b: core::ffi::c_float,
) -> core::ffi::c_float {
    let mut r: core::ffi::c_float = a / b;
    return r;
}
#[inline(always)]
unsafe extern "C" fn llvm_and_u8(mut a: uint8_t, mut b: uint8_t) -> uint8_t {
    let mut r: uint8_t = (a as core::ffi::c_int & b as core::ffi::c_int) as uint8_t;
    return r;
}
#[inline(always)]
unsafe extern "C" fn llvm_OC_fabs_OC_f32(mut a: core::ffi::c_float) -> core::ffi::c_float {
    let mut r: core::ffi::c_float = 0.;
    r = fabsf(a);
    return r;
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn _Z22prepare_angular_tablesv() {
    let mut _1: uint32_t = 0;
    let mut _2: core::ffi::c_float = 0.;
    let mut _3: uint32_t = 0;
    let mut _4: uint32_t = 0;
    let mut _5: uint32_t = 0;
    let mut _6: uint32_t = 0;
    let mut _7: core::ffi::c_float = 0.;
    let mut _8: uint32_t = 0;
    let mut _9: core::ffi::c_float = 0.;
    let mut _10: uint32_t = 0;
    let mut _11: uint32_t = 0;
    let mut _12: core::ffi::c_float = 0.;
    let mut _13: uint32_t = 0;
    let mut _14: core::ffi::c_float = 0.;
    let mut _15: uint32_t = 0;
    let mut _16: uint32_t = 0;
    let mut _17: uint32_t = 0;
    let mut _18: uint32_t = 0;
    _1 = 0;
    loop {
        _4 = _1;
        if !(_4 < 32 as core::ffi::c_uint) {
            break;
        }
        _5 = _1;
        _2 = llvm_add_u32(_5, 1) as core::ffi::c_float;
        _3 = 0;
        loop {
            _6 = _3;
            if !(_6 < 64 as core::ffi::c_uint) {
                break;
            }
            _7 = _2;
            _8 = _3;
            _9 = sinf(llvm_fmul_f32(
                llvm_fmul_f32(0.0997330993f64 as core::ffi::c_float, _7),
                _8 as core::ffi::c_float,
            ));
            _10 = _3;
            _11 = _1;
            *(&raw mut *((*(&raw mut *(_ZL9sin_table.array)
                .as_mut_ptr()
                .offset(_10 as uint64_t as int64_t as isize)
                as *mut crate::l_array_32_float))
                .array)
                .as_mut_ptr()
                .offset(_11 as uint64_t as int64_t as isize)
                as *mut core::ffi::c_float) = _9;
            _12 = _2;
            _13 = _3;
            _14 = cosf(llvm_fmul_f32(
                llvm_fmul_f32(0.0997330993f64 as core::ffi::c_float, _12),
                _13 as core::ffi::c_float,
            ));
            _15 = _3;
            _16 = _1;
            *(&raw mut *((*(&raw mut *(_ZL9cos_table.array)
                .as_mut_ptr()
                .offset(_15 as uint64_t as int64_t as isize)
                as *mut crate::l_array_32_float))
                .array)
                .as_mut_ptr()
                .offset(_16 as uint64_t as int64_t as isize)
                as *mut core::ffi::c_float) = _14;
            _17 = _3;
            _3 = llvm_add_u32(_17, 1);
        }
        _18 = _1;
        _1 = llvm_add_u32(_18, 1);
    }
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn _Z32compute_angular_endpoints_1planebRK21block_size_descriptorPKfjR27compression_working_buffers(
    mut _27: bool_0,
    mut _28: *mut core::ffi::c_void,
    mut _29: *mut core::ffi::c_void,
    mut _30: uint32_t,
    mut _31: *mut core::ffi::c_void,
) {
    let mut current_block: u64;
    let mut _32: uint8_t = 0;
    let mut _33: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _34: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _35: uint32_t = 0;
    let mut _36: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _37: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _38: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _39: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _40: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _41: uint32_t = 0;
    let mut _42: uint32_t = 0;
    let mut _43: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _44: uint32_t = 0;
    let mut _45: uint32_t = 0;
    let mut _46: uint32_t = 0;
    let mut _47: uint32_t = 0;
    let mut _48: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _49: uint32_t = 0;
    let mut _50: uint32_t = 0;
    let mut _51: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _52: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _53: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _54: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _55: uint8_t = 0;
    let mut _56: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _57: uint32_t = 0;
    let mut _58: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _59: uint32_t = 0;
    let mut _60: uint32_t = 0;
    let mut _60__PHI_TEMPORARY: uint32_t = 0;
    let mut _61: uint32_t = 0;
    let mut _62: uint32_t = 0;
    let mut _63: uint32_t = 0;
    let mut _64: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _65: uint32_t = 0;
    let mut _66: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _67: uint32_t = 0;
    let mut _68: bool_0 = 0;
    let mut _69: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _70: uint32_t = 0;
    let mut _71: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _72: uint8_t = 0;
    let mut _73: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _74: uint8_t = 0;
    let mut _75: uint32_t = 0;
    let mut _76: uint32_t = 0;
    let mut _77: uint32_t = 0;
    let mut _78: uint32_t = 0;
    let mut _79: uint32_t = 0;
    let mut _80: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _81: uint32_t = 0;
    let mut _82: uint32_t = 0;
    let mut _83: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _84: uint32_t = 0;
    let mut _85: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _86: uint32_t = 0;
    let mut _87: uint32_t = 0;
    let mut _88: uint8_t = 0;
    let mut _89: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _90: uint32_t = 0;
    let mut _91: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _92: uint32_t = 0;
    let mut _93: uint32_t = 0;
    let mut _93__PHI_TEMPORARY: uint32_t = 0;
    let mut _94: uint32_t = 0;
    let mut _95: uint32_t = 0;
    let mut _96: uint32_t = 0;
    let mut _97: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _98: uint32_t = 0;
    let mut _99: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _100: uint8_t = 0;
    let mut _101: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _102: uint8_t = 0;
    let mut _103: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _104: uint8_t = 0;
    let mut _105: uint32_t = 0;
    let mut _106: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _107: uint32_t = 0;
    let mut _108: uint32_t = 0;
    let mut _109: core::ffi::c_float = 0.;
    let mut _110: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _111: uint32_t = 0;
    let mut _112: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _113: uint32_t = 0;
    let mut _114: uint32_t = 0;
    let mut _115: core::ffi::c_float = 0.;
    let mut _116: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _117: uint32_t = 0;
    let mut _118: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _119: uint32_t = 0;
    let mut _120: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _121: uint32_t = 0;
    let mut _122: uint32_t = 0;
    _32 = _27;
    _33 = _28;
    _34 = _29;
    _35 = _30;
    _36 = _31;
    _51 = _36;
    _37 = &raw mut (*(_51 as *mut l_struct_struct_OC_compression_working_buffers)).field10
        as *mut crate::l_array_2048_float as *mut core::ffi::c_void;
    _52 = _36;
    _38 = &raw mut (*(_52 as *mut l_struct_struct_OC_compression_working_buffers)).field11
        as *mut crate::l_array_2048_float as *mut core::ffi::c_void;
    _53 = _36;
    _39 = &raw mut (*(_53 as *mut l_struct_struct_OC_compression_working_buffers)).field12
        as *mut crate::l_array_87_struct_AC_l_array_8_float as *mut core::ffi::c_void;
    _54 = _36;
    _40 = &raw mut (*(_54 as *mut l_struct_struct_OC_compression_working_buffers)).field13
        as *mut crate::l_array_87_struct_AC_l_array_8_float as *mut core::ffi::c_void;
    _55 = _32;
    if _55 as core::ffi::c_uint & 1 as core::ffi::c_uint != 0 {
        _56 = _33;
        _57 = *(&raw mut (*(_56 as *mut l_struct_struct_OC_block_size_descriptor)).field4
            as *mut uint32_t);
        _60__PHI_TEMPORARY = _57;
    } else {
        _58 = _33;
        _59 = *(&raw mut (*(_58 as *mut l_struct_struct_OC_block_size_descriptor)).field5
            as *mut uint32_t);
        _60__PHI_TEMPORARY = _59;
    }
    _60 = _60__PHI_TEMPORARY;
    _41 = _60;
    _61 = _41;
    if _61 > 0 as core::ffi::c_uint {
        _42 = 0;
        loop {
            _62 = _42;
            _63 = _41;
            if !(_62 < _63) {
                break;
            }
            _64 = _33;
            _65 = _42;
            _43 = &raw mut *((*(&raw mut (*(_64 as *mut l_struct_struct_OC_block_size_descriptor))
                .field13
                as *mut crate::l_array_87_struct_AC_l_struct_struct_OC_decimation_mode))
                .array)
                .as_mut_ptr()
                .offset(_65 as uint64_t as int64_t as isize)
                as *mut l_struct_struct_OC_decimation_mode
                as *mut core::ffi::c_void;
            _66 = _43;
            _67 = _35;
            _68 = _ZNK15decimation_mode13is_ref_1planeE12quant_method(_66, _67);
            if _68 != 0 {
                _69 = _33;
                _70 = _42;
                _71 = _ZNK21block_size_descriptor19get_decimation_infoEj(_69, _70);
                _72 = *(&raw mut (*(_71 as *mut l_struct_struct_OC_decimation_info)).field2
                    as *mut uint8_t);
                _44 = _72 as uint32_t;
                _73 = _43;
                _74 = *(&raw mut (*(_73 as *mut l_struct_struct_OC_decimation_mode)).field0
                    as *mut uint8_t);
                _45 = _74 as int8_t as int32_t as uint32_t;
                _75 = _45;
                if _75 > 7 as core::ffi::c_uint {
                    _45 = 7;
                }
                _76 = _45;
                _77 = _35;
                if _76 > _77 {
                    _78 = _35;
                    _45 = _78;
                }
                _79 = _44;
                _80 = _34;
                _81 = _42;
                _82 = _45;
                _83 = _39;
                _84 = _42;
                _85 = _40;
                _86 = _42;
                _ZL42compute_angular_endpoints_for_quant_levelsjPKfjPfS1_(
                    _79,
                    &raw mut *(_80 as *mut core::ffi::c_float).offset((llvm_mul_u32
                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        _81, 64
                    )
                        as uint64_t
                        as int64_t
                        as isize) as *mut core::ffi::c_float
                        as *mut core::ffi::c_void,
                    _82,
                    &raw mut *((*(&raw mut *((*(_83
                        as *mut crate::l_array_87_struct_AC_l_array_8_float))
                        .array)
                        .as_mut_ptr()
                        .offset(_84 as uint64_t as int64_t as isize)
                        as *mut crate::l_array_8_float))
                        .array)
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float as *mut core::ffi::c_void,
                    &raw mut *((*(&raw mut *((*(_85
                        as *mut crate::l_array_87_struct_AC_l_array_8_float))
                        .array)
                        .as_mut_ptr()
                        .offset(_86 as uint64_t as int64_t as isize)
                        as *mut crate::l_array_8_float))
                        .array)
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float as *mut core::ffi::c_void,
                );
            }
            _87 = _42;
            _42 = llvm_add_u32(_87, 1);
        }
        _88 = _32;
        if _88 as core::ffi::c_uint & 1 as core::ffi::c_uint != 0 {
            _89 = _33;
            _90 = *(&raw mut (*(_89 as *mut l_struct_struct_OC_block_size_descriptor)).field7
                as *mut uint32_t);
            _93__PHI_TEMPORARY = _90;
        } else {
            _91 = _33;
            _92 = *(&raw mut (*(_91 as *mut l_struct_struct_OC_block_size_descriptor)).field8
                as *mut uint32_t);
            _93__PHI_TEMPORARY = _92;
        }
        _93 = _93__PHI_TEMPORARY;
        _46 = _93;
        _94 = _46;
        if _94 > 0 as core::ffi::c_uint {
            _47 = 0;
            loop {
                _95 = _47;
                _96 = _46;
                if !(_95 < _96) {
                    current_block = 8632333977224026159;
                    break;
                }
                _97 = _33;
                _98 = _47;
                _48 = &raw mut *((*(&raw mut (*(_97
                    as *mut l_struct_struct_OC_block_size_descriptor))
                    .field16
                    as *mut crate::l_array_2048_struct_AC_l_struct_struct_OC_block_mode))
                    .array)
                    .as_mut_ptr()
                    .offset(_98 as uint64_t as int64_t as isize)
                    as *mut l_struct_struct_OC_block_mode
                    as *mut core::ffi::c_void;
                _99 = _48;
                _100 = *(&raw mut (*(_99 as *mut l_struct_struct_OC_block_mode)).field4
                    as *mut uint8_t);
                if !((llvm_and_u8(_100, 1) as core::ffi::c_int != 0 as core::ffi::c_int)
                    as core::ffi::c_int
                    ^ 1 as core::ffi::c_int
                    != 0)
                {
                    current_block = 13494617940354982096;
                    break;
                }
                _101 = _48;
                _102 = *(&raw mut (*(_101 as *mut l_struct_struct_OC_block_mode)).field2
                    as *mut uint8_t);
                _49 = _102 as uint32_t;
                _103 = _48;
                _104 = *(&raw mut (*(_103 as *mut l_struct_struct_OC_block_mode)).field1
                    as *mut uint8_t);
                _50 = _104 as uint32_t;
                _105 = _49;
                if _105 <= 7 as core::ffi::c_uint {
                    _106 = _39;
                    _107 = _50;
                    _108 = _49;
                    _109 = *(&raw mut *((*(&raw mut *((*(_106
                        as *mut crate::l_array_87_struct_AC_l_array_8_float))
                        .array)
                        .as_mut_ptr()
                        .offset(_107 as uint64_t as int64_t as isize)
                        as *mut crate::l_array_8_float))
                        .array)
                        .as_mut_ptr()
                        .offset(_108 as uint64_t as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _110 = _37;
                    _111 = _47;
                    *(&raw mut *((*(_110 as *mut crate::l_array_2048_float)).array)
                        .as_mut_ptr()
                        .offset(_111 as uint64_t as int64_t as isize)
                        as *mut core::ffi::c_float) = _109;
                    _112 = _40;
                    _113 = _50;
                    _114 = _49;
                    _115 = *(&raw mut *((*(&raw mut *((*(_112
                        as *mut crate::l_array_87_struct_AC_l_array_8_float))
                        .array)
                        .as_mut_ptr()
                        .offset(_113 as uint64_t as int64_t as isize)
                        as *mut crate::l_array_8_float))
                        .array)
                        .as_mut_ptr()
                        .offset(_114 as uint64_t as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _116 = _38;
                    _117 = _47;
                    *(&raw mut *((*(_116 as *mut crate::l_array_2048_float)).array)
                        .as_mut_ptr()
                        .offset(_117 as uint64_t as int64_t as isize)
                        as *mut core::ffi::c_float) = _115;
                } else {
                    _118 = _37;
                    _119 = _47;
                    *(&raw mut *((*(_118 as *mut crate::l_array_2048_float)).array)
                        .as_mut_ptr()
                        .offset(_119 as uint64_t as int64_t as isize)
                        as *mut core::ffi::c_float) = 0 as core::ffi::c_int as core::ffi::c_float;
                    _120 = _38;
                    _121 = _47;
                    *(&raw mut *((*(_120 as *mut crate::l_array_2048_float)).array)
                        .as_mut_ptr()
                        .offset(_121 as uint64_t as int64_t as isize)
                        as *mut core::ffi::c_float) = 1 as core::ffi::c_int as core::ffi::c_float;
                }
                _122 = _47;
                _47 = llvm_add_u32(_122, 1);
            }
            match current_block {
                8632333977224026159 => return,
                _ => {
                    __assert_fail(
                        &_OC_str_OC_3 as *const crate::l_array_18_uint8_t as *mut core::ffi::c_void,
                        &_OC_str_OC_1 as *const crate::l_array_51_uint8_t as *mut core::ffi::c_void,
                        403,
                        &__PRETTY_FUNCTION___OC__Z32compute_angular_endpoints_1planebRK21block_size_descriptorPKfjR27compression_working_buffers
                            as *const crate::l_array_135_uint8_t as *mut core::ffi::c_void,
                    );
                }
            }
        } else {
            __assert_fail(
                &_OC_str_OC_2 as *const crate::l_array_20_uint8_t as *mut core::ffi::c_void,
                &_OC_str_OC_1 as *const crate::l_array_51_uint8_t as *mut core::ffi::c_void,
                399,
                &__PRETTY_FUNCTION___OC__Z32compute_angular_endpoints_1planebRK21block_size_descriptorPKfjR27compression_working_buffers
                    as *const crate::l_array_135_uint8_t as *mut core::ffi::c_void,
            );
        }
    } else {
        __assert_fail(
            &_OC_str as *const crate::l_array_25_uint8_t as *mut core::ffi::c_void,
            &_OC_str_OC_1 as *const crate::l_array_51_uint8_t as *mut core::ffi::c_void,
            369,
            &__PRETTY_FUNCTION___OC__Z32compute_angular_endpoints_1planebRK21block_size_descriptorPKfjR27compression_working_buffers
                as *const crate::l_array_135_uint8_t as *mut core::ffi::c_void,
        );
    };
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn _ZNK15decimation_mode13is_ref_1planeE12quant_method(
    mut _155: *mut core::ffi::c_void,
    mut _156: uint32_t,
) -> bool_0 {
    let mut _157: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _158: uint32_t = 0;
    let mut _159: uint16_t = 0;
    let mut _160: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _161: uint32_t = 0;
    let mut _162: uint16_t = 0;
    let mut _163: uint16_t = 0;
    _157 = _155;
    _158 = _156;
    _160 = _157;
    _161 = _158;
    _159 = llvm_sub_u32(
        ((1 as core::ffi::c_int) << llvm_add_u32(_161, 1)) as uint32_t,
        1,
    ) as uint16_t;
    _162 = *(&raw mut (*(_160 as *mut l_struct_struct_OC_decimation_mode)).field2 as *mut uint16_t);
    _163 = _159;
    return (_162 as uint32_t & _163 as uint32_t != 0 as core::ffi::c_uint) as core::ffi::c_int
        as bool_0;
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn _ZNK21block_size_descriptor19get_decimation_infoEj(
    mut _164: *mut core::ffi::c_void,
    mut _165: uint32_t,
) -> *mut core::ffi::c_void {
    let mut _166: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _167: uint32_t = 0;
    let mut _168: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _169: uint32_t = 0;
    _166 = _164;
    _167 = _165;
    _168 = _166;
    _169 = _167;
    return &raw mut *((*(&raw mut (*(_168 as *mut l_struct_struct_OC_block_size_descriptor))
        .field14 as *mut crate::l_array_87_struct_AC_l_struct_struct_OC_decimation_info))
        .array)
        .as_mut_ptr()
        .offset(_169 as uint64_t as int64_t as isize)
        as *mut l_struct_struct_OC_decimation_info as *mut core::ffi::c_void;
}
#[inline(never)]
unsafe extern "C" fn _ZL42compute_angular_endpoints_for_quant_levelsjPKfjPfS1_(
    mut _170: uint32_t,
    mut _171: *mut core::ffi::c_void,
    mut _172: uint32_t,
    mut _173: *mut core::ffi::c_void,
    mut _174: *mut core::ffi::c_void,
) {
    let mut _175: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _176: uint8_t = 0;
    let mut _177: uint8_t = 0;
    let mut _178: uint8_t = 0;
    let mut _179: uint8_t = 0;
    let mut _180: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _181: uint8_t = 0;
    let mut _182: uint8_t = 0;
    let mut _183: uint8_t = 0;
    let mut _184: uint8_t = 0;
    let mut _185: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _186: uint8_t = 0;
    let mut _187: uint8_t = 0;
    let mut _188: uint8_t = 0;
    let mut _189: uint8_t = 0;
    let mut _190: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _191: uint8_t = 0;
    let mut _192: uint8_t = 0;
    let mut _193: uint8_t = 0;
    let mut _194: uint8_t = 0;
    let mut _195: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _196: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _197: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _198: core::ffi::c_float = 0.;
    let mut _199: core::ffi::c_float = 0.;
    let mut _200: core::ffi::c_float = 0.;
    let mut _201: core::ffi::c_float = 0.;
    let mut _202: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _203: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _204: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _205: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _206: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _207: core::ffi::c_float = 0.;
    let mut _208: core::ffi::c_float = 0.;
    let mut _209: core::ffi::c_float = 0.;
    let mut _210: core::ffi::c_float = 0.;
    let mut _211: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _212: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _213: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _214: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _215: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _216: core::ffi::c_float = 0.;
    let mut _217: core::ffi::c_float = 0.;
    let mut _218: core::ffi::c_float = 0.;
    let mut _219: core::ffi::c_float = 0.;
    let mut _220: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _221: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _222: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _223: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _224: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _225: core::ffi::c_float = 0.;
    let mut _226: core::ffi::c_float = 0.;
    let mut _227: core::ffi::c_float = 0.;
    let mut _228: core::ffi::c_float = 0.;
    let mut _229: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _230: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _231: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _232: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _233: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _234: core::ffi::c_float = 0.;
    let mut _235: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _236: core::ffi::c_float = 0.;
    let mut _237: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _238: core::ffi::c_float = 0.;
    let mut _239: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _240: core::ffi::c_float = 0.;
    let mut _241: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _242: core::ffi::c_float = 0.;
    let mut _243: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _244: core::ffi::c_float = 0.;
    let mut _245: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _246: core::ffi::c_float = 0.;
    let mut _247: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _248: core::ffi::c_float = 0.;
    let mut _249: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _250: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _251: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _252: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _253: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _254: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _255: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _256: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _257: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _258: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _259: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _260: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _261: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _262: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _263: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _264: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _265: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _266: core::ffi::c_float = 0.;
    let mut _267: core::ffi::c_float = 0.;
    let mut _268: core::ffi::c_float = 0.;
    let mut _269: core::ffi::c_float = 0.;
    let mut _270: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _271: core::ffi::c_float = 0.;
    let mut _272: core::ffi::c_float = 0.;
    let mut _273: core::ffi::c_float = 0.;
    let mut _274: core::ffi::c_float = 0.;
    let mut _275: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _276: core::ffi::c_float = 0.;
    let mut _277: core::ffi::c_float = 0.;
    let mut _278: core::ffi::c_float = 0.;
    let mut _279: core::ffi::c_float = 0.;
    let mut _280: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _281: core::ffi::c_float = 0.;
    let mut _282: core::ffi::c_float = 0.;
    let mut _283: core::ffi::c_float = 0.;
    let mut _284: core::ffi::c_float = 0.;
    let mut _285: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _286: core::ffi::c_float = 0.;
    let mut _287: core::ffi::c_float = 0.;
    let mut _288: core::ffi::c_float = 0.;
    let mut _289: core::ffi::c_float = 0.;
    let mut _290: uint32_t = 0;
    let mut _291: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _292: uint32_t = 0;
    let mut _293: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _294: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _295: uint32_t = 0;
    let mut _296: uint32_t = 0;
    let mut _297: crate::l_array_32_float = crate::l_array_32_float { array: [0.; 32] };
    let mut _298: crate::l_array_32_float = crate::l_array_32_float { array: [0.; 32] };
    let mut _299: crate::l_array_32_uint32_t = crate::l_array_32_uint32_t { array: [0; 32] };
    let mut _300: crate::l_array_32_float = crate::l_array_32_float { array: [0.; 32] };
    let mut _301: crate::l_array_32_float = crate::l_array_32_float { array: [0.; 32] };
    let mut _302: crate::l_array_32_float = crate::l_array_32_float { array: [0.; 32] };
    let mut _303: crate::l_array_36_struct_AC_l_struct_struct_OC_vfloat4 =
        crate::l_array_36_struct_AC_l_struct_struct_OC_vfloat4 {
            array: [l_struct_struct_OC_vfloat4 {
                field0: crate::l_array_4_float { array: [0.; 4] },
            }; 36],
        };
    let mut _304: uint32_t = 0;
    let mut _305: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _306: uint32_t = 0;
    let mut _307: core::ffi::c_float = 0.;
    let mut _308: uint32_t = 0;
    let mut _309: core::ffi::c_float = 0.;
    let mut _310: core::ffi::c_float = 0.;
    let mut _311: core::ffi::c_float = 0.;
    let mut _312: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _313: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _314: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _315: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _316: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _317: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _318: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _319: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _320: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _321: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _322: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _323: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _324: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _325: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _326: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _327: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _328: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _329: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _330: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _331: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _332: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _333: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _334: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _335: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _336: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _337: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _338: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _339: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _340: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _341: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _342: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _343: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _344: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _345: uint32_t = 0;
    let mut _346: uint32_t = 0;
    let mut _347: uint32_t = 0;
    let mut _348: core::ffi::c_float = 0.;
    let mut _349: core::ffi::c_float = 0.;
    let mut _350: core::ffi::c_float = 0.;
    let mut _351: uint32_t = 0;
    let mut _352: uint8_t = 0;
    let mut _353: uint32_t = 0;
    let mut _354: uint8_t = 0;
    let mut _355: uint32_t = 0;
    let mut _356: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _357: uint32_t = 0;
    let mut _358: uint32_t = 0;
    let mut _359: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _360: uint32_t = 0;
    let mut _361: uint32_t = 0;
    let mut _362: uint32_t = 0;
    let mut _363: uint32_t = 0;
    let mut _364: uint32_t = 0;
    let mut _365: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _366: core::ffi::c_float = 0.;
    let mut _367: core::ffi::c_float = 0.;
    let mut _368: core::ffi::c_float = 0.;
    let mut _369: core::ffi::c_float = 0.;
    let mut _370: uint32_t = 0;
    let mut _371: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _372: uint32_t = 0;
    let mut _373: uint32_t = 0;
    let mut _374: uint32_t = 0;
    let mut _375: uint32_t = 0;
    let mut _376: uint32_t = 0;
    let mut _377: uint32_t = 0;
    let mut _378: uint32_t = 0;
    let mut _379: uint32_t = 0;
    let mut _380: core::ffi::c_float = 0.;
    let mut _381: uint32_t = 0;
    let mut _382: core::ffi::c_float = 0.;
    let mut _383: uint32_t = 0;
    let mut _384: core::ffi::c_float = 0.;
    let mut _385: uint32_t = 0;
    let mut _386: core::ffi::c_float = 0.;
    let mut _387: uint32_t = 0;
    let mut _388: core::ffi::c_float = 0.;
    let mut _389: uint32_t = 0;
    let mut _390: core::ffi::c_float = 0.;
    let mut _391: uint32_t = 0;
    let mut _392: core::ffi::c_float = 0.;
    let mut _393: uint32_t = 0;
    let mut _394: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _395: uint32_t = 0;
    let mut _396: core::ffi::c_float = 0.;
    let mut _397: core::ffi::c_float = 0.;
    let mut _398: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _399: core::ffi::c_float = 0.;
    let mut _400: core::ffi::c_float = 0.;
    let mut _401: core::ffi::c_float = 0.;
    let mut _402: core::ffi::c_float = 0.;
    let mut _403: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _404: core::ffi::c_float = 0.;
    let mut _405: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _406: core::ffi::c_float = 0.;
    let mut _407: core::ffi::c_float = 0.;
    let mut _408: core::ffi::c_float = 0.;
    let mut _409: core::ffi::c_float = 0.;
    let mut _410: uint32_t = 0;
    let mut _411: core::ffi::c_float = 0.;
    let mut _412: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _413: core::ffi::c_float = 0.;
    let mut _414: core::ffi::c_float = 0.;
    let mut _415: core::ffi::c_float = 0.;
    let mut _416: core::ffi::c_float = 0.;
    let mut _417: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _418: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _419: core::ffi::c_float = 0.;
    let mut _420: core::ffi::c_float = 0.;
    let mut _421: core::ffi::c_float = 0.;
    let mut _422: core::ffi::c_float = 0.;
    let mut _423: core::ffi::c_float = 0.;
    let mut _424: core::ffi::c_float = 0.;
    let mut _425: core::ffi::c_float = 0.;
    let mut _426: core::ffi::c_float = 0.;
    let mut _427: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _428: uint8_t = 0;
    let mut _429: uint64_t = 0;
    let mut _430: uint8_t = 0;
    let mut _431: uint64_t = 0;
    let mut _432: uint8_t = 0;
    let mut _433: uint64_t = 0;
    let mut _434: uint8_t = 0;
    let mut _435: uint64_t = 0;
    let mut _436: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _437: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _438: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _439: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _440: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _441: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _442: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _443: uint32_t = 0;
    let mut _444: core::ffi::c_float = 0.;
    let mut _445: core::ffi::c_float = 0.;
    let mut _446: core::ffi::c_float = 0.;
    let mut _446__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _447: uint32_t = 0;
    let mut _448: core::ffi::c_float = 0.;
    let mut _449: core::ffi::c_float = 0.;
    let mut _450: core::ffi::c_float = 0.;
    let mut _450__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _451: uint32_t = 0;
    let mut _452: core::ffi::c_float = 0.;
    let mut _453: core::ffi::c_float = 0.;
    let mut _454: core::ffi::c_float = 0.;
    let mut _454__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _455: uint32_t = 0;
    let mut _456: core::ffi::c_float = 0.;
    let mut _457: core::ffi::c_float = 0.;
    let mut _458: core::ffi::c_float = 0.;
    let mut _458__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _459: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _460: core::ffi::c_float = 0.;
    let mut _461: core::ffi::c_float = 0.;
    let mut _462: core::ffi::c_float = 0.;
    let mut _463: core::ffi::c_float = 0.;
    let mut _464: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _465: uint32_t = 0;
    let mut _466: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _467: uint32_t = 0;
    let mut _468: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _469: core::ffi::c_float = 0.;
    let mut _470: core::ffi::c_float = 0.;
    let mut _471: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _472: core::ffi::c_float = 0.;
    let mut _473: core::ffi::c_float = 0.;
    let mut _474: core::ffi::c_float = 0.;
    let mut _475: core::ffi::c_float = 0.;
    let mut _476: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _477: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _478: core::ffi::c_float = 0.;
    let mut _479: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _480: core::ffi::c_float = 0.;
    let mut _481: core::ffi::c_float = 0.;
    let mut _482: core::ffi::c_float = 0.;
    let mut _483: core::ffi::c_float = 0.;
    let mut _484: core::ffi::c_float = 0.;
    let mut _485: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _486: core::ffi::c_float = 0.;
    let mut _487: core::ffi::c_float = 0.;
    let mut _488: core::ffi::c_float = 0.;
    let mut _489: core::ffi::c_float = 0.;
    let mut _490: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _491: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _492: core::ffi::c_float = 0.;
    let mut _493: core::ffi::c_float = 0.;
    let mut _494: core::ffi::c_float = 0.;
    let mut _495: core::ffi::c_float = 0.;
    let mut _496: core::ffi::c_float = 0.;
    let mut _497: core::ffi::c_float = 0.;
    let mut _498: core::ffi::c_float = 0.;
    let mut _499: core::ffi::c_float = 0.;
    let mut _500: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _501: uint8_t = 0;
    let mut _502: uint64_t = 0;
    let mut _503: uint8_t = 0;
    let mut _504: uint64_t = 0;
    let mut _505: uint8_t = 0;
    let mut _506: uint64_t = 0;
    let mut _507: uint8_t = 0;
    let mut _508: uint64_t = 0;
    let mut _509: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _510: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _511: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _512: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _513: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _514: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _515: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _516: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _517: uint32_t = 0;
    let mut _518: core::ffi::c_float = 0.;
    let mut _519: core::ffi::c_float = 0.;
    let mut _520: core::ffi::c_float = 0.;
    let mut _520__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _521: uint32_t = 0;
    let mut _522: core::ffi::c_float = 0.;
    let mut _523: core::ffi::c_float = 0.;
    let mut _524: core::ffi::c_float = 0.;
    let mut _524__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _525: uint32_t = 0;
    let mut _526: core::ffi::c_float = 0.;
    let mut _527: core::ffi::c_float = 0.;
    let mut _528: core::ffi::c_float = 0.;
    let mut _528__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _529: uint32_t = 0;
    let mut _530: core::ffi::c_float = 0.;
    let mut _531: core::ffi::c_float = 0.;
    let mut _532: core::ffi::c_float = 0.;
    let mut _532__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _533: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _534: core::ffi::c_float = 0.;
    let mut _535: core::ffi::c_float = 0.;
    let mut _536: core::ffi::c_float = 0.;
    let mut _537: core::ffi::c_float = 0.;
    let mut _538: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _539: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _540: core::ffi::c_float = 0.;
    let mut _541: core::ffi::c_float = 0.;
    let mut _542: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _543: core::ffi::c_float = 0.;
    let mut _544: core::ffi::c_float = 0.;
    let mut _545: core::ffi::c_float = 0.;
    let mut _546: core::ffi::c_float = 0.;
    let mut _547: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _548: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _549: core::ffi::c_float = 0.;
    let mut _550: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _551: core::ffi::c_float = 0.;
    let mut _552: core::ffi::c_float = 0.;
    let mut _553: core::ffi::c_float = 0.;
    let mut _554: core::ffi::c_float = 0.;
    let mut _555: core::ffi::c_float = 0.;
    let mut _556: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _557: core::ffi::c_float = 0.;
    let mut _558: core::ffi::c_float = 0.;
    let mut _559: core::ffi::c_float = 0.;
    let mut _560: core::ffi::c_float = 0.;
    let mut _561: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _562: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _563: core::ffi::c_float = 0.;
    let mut _564: core::ffi::c_float = 0.;
    let mut _565: core::ffi::c_float = 0.;
    let mut _566: core::ffi::c_float = 0.;
    let mut _567: core::ffi::c_float = 0.;
    let mut _568: core::ffi::c_float = 0.;
    let mut _569: core::ffi::c_float = 0.;
    let mut _570: core::ffi::c_float = 0.;
    let mut _571: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _572: uint8_t = 0;
    let mut _573: uint64_t = 0;
    let mut _574: uint8_t = 0;
    let mut _575: uint64_t = 0;
    let mut _576: uint8_t = 0;
    let mut _577: uint64_t = 0;
    let mut _578: uint8_t = 0;
    let mut _579: uint64_t = 0;
    let mut _580: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _581: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _582: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _583: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _584: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _585: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _586: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _587: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _588: uint32_t = 0;
    let mut _589: core::ffi::c_float = 0.;
    let mut _590: core::ffi::c_float = 0.;
    let mut _591: core::ffi::c_float = 0.;
    let mut _591__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _592: uint32_t = 0;
    let mut _593: core::ffi::c_float = 0.;
    let mut _594: core::ffi::c_float = 0.;
    let mut _595: core::ffi::c_float = 0.;
    let mut _595__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _596: uint32_t = 0;
    let mut _597: core::ffi::c_float = 0.;
    let mut _598: core::ffi::c_float = 0.;
    let mut _599: core::ffi::c_float = 0.;
    let mut _599__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _600: uint32_t = 0;
    let mut _601: core::ffi::c_float = 0.;
    let mut _602: core::ffi::c_float = 0.;
    let mut _603: core::ffi::c_float = 0.;
    let mut _603__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _604: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _605: core::ffi::c_float = 0.;
    let mut _606: core::ffi::c_float = 0.;
    let mut _607: core::ffi::c_float = 0.;
    let mut _608: core::ffi::c_float = 0.;
    let mut _609: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _610: uint32_t = 0;
    let mut _611: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _612: uint32_t = 0;
    let mut _613: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _614: core::ffi::c_float = 0.;
    let mut _615: core::ffi::c_float = 0.;
    let mut _616: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _617: core::ffi::c_float = 0.;
    let mut _618: core::ffi::c_float = 0.;
    let mut _619: core::ffi::c_float = 0.;
    let mut _620: core::ffi::c_float = 0.;
    let mut _621: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _622: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _623: core::ffi::c_float = 0.;
    let mut _624: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _625: core::ffi::c_float = 0.;
    let mut _626: core::ffi::c_float = 0.;
    let mut _627: core::ffi::c_float = 0.;
    let mut _628: core::ffi::c_float = 0.;
    let mut _629: core::ffi::c_float = 0.;
    let mut _630: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _631: core::ffi::c_float = 0.;
    let mut _632: core::ffi::c_float = 0.;
    let mut _633: core::ffi::c_float = 0.;
    let mut _634: core::ffi::c_float = 0.;
    let mut _635: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _636: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _637: core::ffi::c_float = 0.;
    let mut _638: core::ffi::c_float = 0.;
    let mut _639: core::ffi::c_float = 0.;
    let mut _640: core::ffi::c_float = 0.;
    let mut _641: core::ffi::c_float = 0.;
    let mut _642: core::ffi::c_float = 0.;
    let mut _643: core::ffi::c_float = 0.;
    let mut _644: core::ffi::c_float = 0.;
    let mut _645: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _646: uint8_t = 0;
    let mut _647: uint64_t = 0;
    let mut _648: uint8_t = 0;
    let mut _649: uint64_t = 0;
    let mut _650: uint8_t = 0;
    let mut _651: uint64_t = 0;
    let mut _652: uint8_t = 0;
    let mut _653: uint64_t = 0;
    let mut _654: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _655: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _656: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _657: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _658: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _659: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _660: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _661: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _662: uint32_t = 0;
    let mut _663: core::ffi::c_float = 0.;
    let mut _664: core::ffi::c_float = 0.;
    let mut _665: core::ffi::c_float = 0.;
    let mut _665__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _666: uint32_t = 0;
    let mut _667: core::ffi::c_float = 0.;
    let mut _668: core::ffi::c_float = 0.;
    let mut _669: core::ffi::c_float = 0.;
    let mut _669__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _670: uint32_t = 0;
    let mut _671: core::ffi::c_float = 0.;
    let mut _672: core::ffi::c_float = 0.;
    let mut _673: core::ffi::c_float = 0.;
    let mut _673__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _674: uint32_t = 0;
    let mut _675: core::ffi::c_float = 0.;
    let mut _676: core::ffi::c_float = 0.;
    let mut _677: core::ffi::c_float = 0.;
    let mut _677__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _678: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _679: core::ffi::c_float = 0.;
    let mut _680: core::ffi::c_float = 0.;
    let mut _681: core::ffi::c_float = 0.;
    let mut _682: core::ffi::c_float = 0.;
    let mut _683: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _684: uint32_t = 0;
    let mut _685: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _686: uint32_t = 0;
    let mut _687: uint32_t = 0;
    let mut _688: uint32_t = 0;
    let mut _689: uint32_t = 0;
    let mut _690: uint8_t = 0;
    let mut _691: uint32_t = 0;
    let mut _692: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _693: core::ffi::c_float = 0.;
    let mut _694: uint32_t = 0;
    let mut _695: uint32_t = 0;
    let mut _696: uint32_t = 0;
    let mut _697: core::ffi::c_float = 0.;
    let mut _698: uint32_t = 0;
    let mut _699: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _700: core::ffi::c_float = 0.;
    let mut _701: core::ffi::c_float = 0.;
    let mut _702: uint32_t = 0;
    let mut _703: uint32_t = 0;
    let mut _704: uint32_t = 0;
    let mut _705: core::ffi::c_float = 0.;
    let mut _706: core::ffi::c_float = 0.;
    let mut _707: core::ffi::c_float = 0.;
    let mut _708: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _709: uint32_t = 0;
    let mut _710: uint32_t = 0;
    let mut _711: core::ffi::c_float = 0.;
    let mut _712: core::ffi::c_float = 0.;
    let mut _713: core::ffi::c_float = 0.;
    let mut _714: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _715: uint32_t = 0;
    let mut _716: uint32_t = 0;
    _290 = _170;
    _291 = _171;
    _292 = _172;
    _293 = _173;
    _294 = _174;
    _351 = _292;
    _352 = *(&raw const *(_ZL21steps_for_quant_level.array)
        .as_ptr()
        .offset(_351 as uint64_t as int64_t as isize) as *const uint8_t
        as *mut uint8_t);
    _295 = _352 as uint32_t;
    _353 = _292;
    _354 = *(&raw const *(_ZL21steps_for_quant_level.array)
        .as_ptr()
        .offset(_353 as uint64_t as int64_t as isize) as *const uint8_t
        as *mut uint8_t);
    _296 = _354 as uint32_t;
    _355 = _290;
    _356 = _291;
    _357 = _296;
    _ZL23compute_angular_offsetsjPKfjPf(
        _355,
        _356,
        _357,
        &raw mut *(_297.array)
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as int64_t as isize) as *mut core::ffi::c_float
            as *mut core::ffi::c_void,
    );
    _358 = _290;
    _359 = _291;
    _360 = _296;
    _361 = _295;
    _ZL33compute_lowest_and_highest_weightjPKfjjS0_PfPiS1_S1_S1_(
        _358,
        _359,
        _360,
        _361,
        &raw mut *(_297.array)
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as int64_t as isize) as *mut core::ffi::c_float
            as *mut core::ffi::c_void,
        &raw mut *(_298.array)
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as int64_t as isize) as *mut core::ffi::c_float
            as *mut core::ffi::c_void,
        &raw mut *(_299.array)
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as int64_t as isize) as *mut uint32_t
            as *mut core::ffi::c_void,
        &raw mut *(_300.array)
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as int64_t as isize) as *mut core::ffi::c_float
            as *mut core::ffi::c_void,
        &raw mut *(_301.array)
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as int64_t as isize) as *mut core::ffi::c_float
            as *mut core::ffi::c_void,
        &raw mut *(_302.array)
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as int64_t as isize) as *mut core::ffi::c_float
            as *mut core::ffi::c_void,
    );
    _362 = _295;
    if _362 > 0 as core::ffi::c_uint {
        _304 = 0;
        loop {
            _363 = _304;
            _364 = _295;
            if !(_363 < llvm_add_u32(_364, 4)) {
                break;
            }
            _285 = &mut _305 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
            _286 = 1.00000002E+30f64 as core::ffi::c_float;
            _287 = -(1 as core::ffi::c_int) as core::ffi::c_float;
            _288 = 0 as core::ffi::c_int as core::ffi::c_float;
            _289 = 0 as core::ffi::c_int as core::ffi::c_float;
            _365 = _285;
            _366 = _286;
            *(_365 as *mut core::ffi::c_float) = _366;
            _367 = _287;
            *(&raw mut *((*(_365 as *mut crate::l_array_4_float)).array)
                .as_mut_ptr()
                .offset(1 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float) = _367;
            _368 = _288;
            *(&raw mut *((*(_365 as *mut crate::l_array_4_float)).array)
                .as_mut_ptr()
                .offset(2 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float) = _368;
            _369 = _289;
            *(&raw mut *((*(_365 as *mut crate::l_array_4_float)).array)
                .as_mut_ptr()
                .offset(3 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float) = _369;
            _370 = _304;
            _371 = memcpy(
                &raw mut *(_303.array)
                    .as_mut_ptr()
                    .offset(_370 as uint64_t as int64_t as isize)
                    as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                &mut _305 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                16,
            );
            _372 = _304;
            _304 = llvm_add_u32(_372, 1);
        }
        _373 = _296;
        if _373 > 0 as core::ffi::c_uint {
            _306 = 0;
            loop {
                _374 = _306;
                _375 = _296;
                if !(_374 < _375) {
                    break;
                }
                _376 = _306;
                _307 = _376 as core::ffi::c_float;
                _377 = _306;
                _378 = *(&raw mut *(_299.array)
                    .as_mut_ptr()
                    .offset(_377 as uint64_t as int64_t as isize)
                    as *mut uint32_t);
                _308 = _378;
                _379 = _306;
                _380 = *(&raw mut *(_300.array)
                    .as_mut_ptr()
                    .offset(_379 as uint64_t as int64_t as isize)
                    as *mut core::ffi::c_float);
                _381 = _306;
                _382 = *(&raw mut *(_301.array)
                    .as_mut_ptr()
                    .offset(_381 as uint64_t as int64_t as isize)
                    as *mut core::ffi::c_float);
                _309 = llvm_fadd_f32(_380, _382);
                _383 = _306;
                _384 = *(&raw mut *(_300.array)
                    .as_mut_ptr()
                    .offset(_383 as uint64_t as int64_t as isize)
                    as *mut core::ffi::c_float);
                _385 = _306;
                _386 = *(&raw mut *(_302.array)
                    .as_mut_ptr()
                    .offset(_385 as uint64_t as int64_t as isize)
                    as *mut core::ffi::c_float);
                _310 = llvm_fadd_f32(_384, _386);
                _387 = _306;
                _388 = *(&raw mut *(_300.array)
                    .as_mut_ptr()
                    .offset(_387 as uint64_t as int64_t as isize)
                    as *mut core::ffi::c_float);
                _389 = _306;
                _390 = *(&raw mut *(_301.array)
                    .as_mut_ptr()
                    .offset(_389 as uint64_t as int64_t as isize)
                    as *mut core::ffi::c_float);
                _391 = _306;
                _392 = *(&raw mut *(_302.array)
                    .as_mut_ptr()
                    .offset(_391 as uint64_t as int64_t as isize)
                    as *mut core::ffi::c_float);
                _311 = llvm_fadd_f32(llvm_fadd_f32(_388, _390), _392);
                _393 = _308;
                _394 = memcpy(
                    &mut _312 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &raw mut *(_303.array)
                        .as_mut_ptr()
                        .offset(_393 as int32_t as int64_t as isize)
                        as *mut l_struct_struct_OC_vfloat4
                        as *mut core::ffi::c_void,
                    16,
                );
                _395 = _306;
                _396 = *(&raw mut *(_300.array)
                    .as_mut_ptr()
                    .offset(_395 as uint64_t as int64_t as isize)
                    as *mut core::ffi::c_float);
                _397 = _307;
                _280 = &mut _313 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _281 = _396;
                _282 = _397;
                _283 = 0 as core::ffi::c_int as core::ffi::c_float;
                _284 = 0 as core::ffi::c_int as core::ffi::c_float;
                _398 = _280;
                _399 = _281;
                *(_398 as *mut core::ffi::c_float) = _399;
                _400 = _282;
                *(&raw mut *((*(_398 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _400;
                _401 = _283;
                *(&raw mut *((*(_398 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _401;
                _402 = _284;
                *(&raw mut *((*(_398 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _402;
                _249 = &mut _312 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _403 = _249;
                _404 = *(_403 as *mut core::ffi::c_float);
                _247 = &mut _315 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _248 = _404;
                _405 = _247;
                _406 = _248;
                *(_405 as *mut core::ffi::c_float) = _406;
                _407 = _248;
                *(&raw mut *((*(_405 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _407;
                _408 = _248;
                *(&raw mut *((*(_405 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _408;
                _409 = _248;
                *(&raw mut *((*(_405 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _409;
                _410 = _306;
                _411 = *(&raw mut *(_300.array)
                    .as_mut_ptr()
                    .offset(_410 as uint64_t as int64_t as isize)
                    as *mut core::ffi::c_float);
                _245 = &mut _316 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _246 = _411;
                _412 = _245;
                _413 = _246;
                *(_412 as *mut core::ffi::c_float) = _413;
                _414 = _246;
                *(&raw mut *((*(_412 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _414;
                _415 = _246;
                *(&raw mut *((*(_412 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _415;
                _416 = _246;
                *(&raw mut *((*(_412 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _416;
                _417 = *(&mut _315.field0 as *mut crate::l_array_4_float);
                _418 = *(&mut _316.field0 as *mut crate::l_array_4_float);
                *(&mut _254 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _417;
                *(&mut _255 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _418;
                _419 = *(&mut _254 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _420 = *(&mut _255 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _421 = *(&raw mut *((*(&mut _254 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _422 = *(&raw mut *((*(&mut _255 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _423 = *(&raw mut *((*(&mut _254 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _424 = *(&raw mut *((*(&mut _255 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _425 = *(&raw mut *((*(&mut _254 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _426 = *(&raw mut *((*(&mut _255 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _190 = &mut _253 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void;
                _191 = llvm_fcmp_ogt(_419 as core::ffi::c_double, _420 as core::ffi::c_double)
                    as bool_0;
                _192 = llvm_fcmp_ogt(_421 as core::ffi::c_double, _422 as core::ffi::c_double)
                    as bool_0;
                _193 = llvm_fcmp_ogt(_423 as core::ffi::c_double, _424 as core::ffi::c_double)
                    as bool_0;
                _194 = llvm_fcmp_ogt(_425 as core::ffi::c_double, _426 as core::ffi::c_double)
                    as bool_0;
                _427 = _190;
                _428 = _191;
                _429 = ((_428 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0 as uint32_t
                    == 0 as core::ffi::c_uint) as core::ffi::c_int as bool_0
                    as uint64_t;
                *(_427 as *mut uint32_t) = llvm_select_u32(
                    ((_428 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0 as uint32_t
                        == 0 as core::ffi::c_uint) as core::ffi::c_int
                        as bool_0,
                    0,
                    -(1 as core::ffi::c_int) as uint32_t,
                );
                _430 = _192;
                _431 = ((_430 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0 as uint32_t
                    == 0 as core::ffi::c_uint) as core::ffi::c_int as bool_0
                    as uint64_t;
                *(&raw mut *((*(_427 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = llvm_select_u32(
                    ((_430 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0 as uint32_t
                        == 0 as core::ffi::c_uint) as core::ffi::c_int
                        as bool_0,
                    0,
                    -(1 as core::ffi::c_int) as uint32_t,
                );
                _432 = _193;
                _433 = ((_432 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0 as uint32_t
                    == 0 as core::ffi::c_uint) as core::ffi::c_int as bool_0
                    as uint64_t;
                *(&raw mut *((*(_427 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = llvm_select_u32(
                    ((_432 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0 as uint32_t
                        == 0 as core::ffi::c_uint) as core::ffi::c_int
                        as bool_0,
                    0,
                    -(1 as core::ffi::c_int) as uint32_t,
                );
                _434 = _194;
                _435 = ((_434 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0 as uint32_t
                    == 0 as core::ffi::c_uint) as core::ffi::c_int as bool_0
                    as uint64_t;
                *(&raw mut *((*(_427 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = llvm_select_u32(
                    ((_434 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0 as uint32_t
                        == 0 as core::ffi::c_uint) as core::ffi::c_int
                        as bool_0,
                    0,
                    -(1 as core::ffi::c_int) as uint32_t,
                );
                _436 = *(&mut _253 as *mut l_struct_struct_OC_vmask4
                    as *mut crate::l_array_2_uint64_t);
                (*(&mut _314.field0 as *mut crate::l_array_4_uint32_t as *mut C2RustUnnamed_6))
                    .data = _436;
                _437 = memcpy(
                    &mut _318 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _312 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _438 = memcpy(
                    &mut _319 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _313 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _439 = memcpy(
                    &mut _320 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void,
                    &mut _314 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void,
                    16,
                );
                _440 = *(&mut _318.field0 as *mut crate::l_array_4_float);
                _441 = *(&mut _319.field0 as *mut crate::l_array_4_float);
                _442 = (*(&mut _320.field0 as *mut crate::l_array_4_uint32_t
                    as *mut C2RustUnnamed_5))
                    .data;
                *(&mut _203 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _440;
                *(&mut _204 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _441;
                *(&mut _205 as *mut l_struct_struct_OC_vmask4 as *mut crate::l_array_2_uint64_t) =
                    _442;
                _443 = *(&mut _205 as *mut l_struct_struct_OC_vmask4 as *mut uint32_t);
                if _443 & 2147483648 as core::ffi::c_uint != 0 as core::ffi::c_uint {
                    _444 =
                        *(&mut _204 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _446__PHI_TEMPORARY = _444;
                } else {
                    _445 =
                        *(&mut _203 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _446__PHI_TEMPORARY = _445;
                }
                _446 = _446__PHI_TEMPORARY;
                _447 = *(&raw mut *((*(&mut _205 as *mut l_struct_struct_OC_vmask4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                if _447 & 2147483648 as core::ffi::c_uint != 0 as core::ffi::c_uint {
                    _448 = *(&raw mut *((*(&mut _204 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _450__PHI_TEMPORARY = _448;
                } else {
                    _449 = *(&raw mut *((*(&mut _203 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _450__PHI_TEMPORARY = _449;
                }
                _450 = _450__PHI_TEMPORARY;
                _451 = *(&raw mut *((*(&mut _205 as *mut l_struct_struct_OC_vmask4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                if _451 & 2147483648 as core::ffi::c_uint != 0 as core::ffi::c_uint {
                    _452 = *(&raw mut *((*(&mut _204 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _454__PHI_TEMPORARY = _452;
                } else {
                    _453 = *(&raw mut *((*(&mut _203 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _454__PHI_TEMPORARY = _453;
                }
                _454 = _454__PHI_TEMPORARY;
                _455 = *(&raw mut *((*(&mut _205 as *mut l_struct_struct_OC_vmask4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                if _455 & 2147483648 as core::ffi::c_uint != 0 as core::ffi::c_uint {
                    _456 = *(&raw mut *((*(&mut _204 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _458__PHI_TEMPORARY = _456;
                } else {
                    _457 = *(&raw mut *((*(&mut _203 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _458__PHI_TEMPORARY = _457;
                }
                _458 = _458__PHI_TEMPORARY;
                _197 = &mut _202 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _198 = _446;
                _199 = _450;
                _200 = _454;
                _201 = _458;
                _459 = _197;
                _460 = _198;
                *(_459 as *mut core::ffi::c_float) = _460;
                _461 = _199;
                *(&raw mut *((*(_459 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _461;
                _462 = _200;
                *(&raw mut *((*(_459 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _462;
                _463 = _201;
                *(&raw mut *((*(_459 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _463;
                _464 = _202;
                *(&mut _317.field0 as *mut crate::l_array_4_float) = _464.field0;
                _465 = _308;
                _466 = memcpy(
                    &raw mut *(_303.array)
                        .as_mut_ptr()
                        .offset(_465 as int32_t as int64_t as isize)
                        as *mut l_struct_struct_OC_vfloat4
                        as *mut core::ffi::c_void,
                    &mut _317 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _467 = _308;
                _468 = memcpy(
                    &mut _312 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &raw mut *(_303.array).as_mut_ptr().offset((llvm_sub_u32
                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        _467, 1
                    ) as int32_t
                        as int64_t
                        as isize) as *mut l_struct_struct_OC_vfloat4
                        as *mut core::ffi::c_void,
                    16,
                );
                _469 = _309;
                _470 = _307;
                _275 = &mut _321 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _276 = _469;
                _277 = _470;
                _278 = 1 as core::ffi::c_int as core::ffi::c_float;
                _279 = 0 as core::ffi::c_int as core::ffi::c_float;
                _471 = _275;
                _472 = _276;
                *(_471 as *mut core::ffi::c_float) = _472;
                _473 = _277;
                *(&raw mut *((*(_471 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _473;
                _474 = _278;
                *(&raw mut *((*(_471 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _474;
                _475 = _279;
                *(&raw mut *((*(_471 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _475;
                _476 = memcpy(
                    &mut _313 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _321 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _250 = &mut _312 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _477 = _250;
                _478 = *(_477 as *mut core::ffi::c_float);
                _243 = &mut _323 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _244 = _478;
                _479 = _243;
                _480 = _244;
                *(_479 as *mut core::ffi::c_float) = _480;
                _481 = _244;
                *(&raw mut *((*(_479 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _481;
                _482 = _244;
                *(&raw mut *((*(_479 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _482;
                _483 = _244;
                *(&raw mut *((*(_479 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _483;
                _484 = _309;
                _241 = &mut _324 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _242 = _484;
                _485 = _241;
                _486 = _242;
                *(_485 as *mut core::ffi::c_float) = _486;
                _487 = _242;
                *(&raw mut *((*(_485 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _487;
                _488 = _242;
                *(&raw mut *((*(_485 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _488;
                _489 = _242;
                *(&raw mut *((*(_485 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _489;
                _490 = *(&mut _323.field0 as *mut crate::l_array_4_float);
                _491 = *(&mut _324.field0 as *mut crate::l_array_4_float);
                *(&mut _257 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _490;
                *(&mut _258 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _491;
                _492 = *(&mut _257 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _493 = *(&mut _258 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _494 = *(&raw mut *((*(&mut _257 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _495 = *(&raw mut *((*(&mut _258 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _496 = *(&raw mut *((*(&mut _257 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _497 = *(&raw mut *((*(&mut _258 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _498 = *(&raw mut *((*(&mut _257 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _499 = *(&raw mut *((*(&mut _258 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _185 = &mut _256 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void;
                _186 = llvm_fcmp_ogt(_492 as core::ffi::c_double, _493 as core::ffi::c_double)
                    as bool_0;
                _187 = llvm_fcmp_ogt(_494 as core::ffi::c_double, _495 as core::ffi::c_double)
                    as bool_0;
                _188 = llvm_fcmp_ogt(_496 as core::ffi::c_double, _497 as core::ffi::c_double)
                    as bool_0;
                _189 = llvm_fcmp_ogt(_498 as core::ffi::c_double, _499 as core::ffi::c_double)
                    as bool_0;
                _500 = _185;
                _501 = _186;
                _502 = ((_501 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0 as uint32_t
                    == 0 as core::ffi::c_uint) as core::ffi::c_int as bool_0
                    as uint64_t;
                *(_500 as *mut uint32_t) = llvm_select_u32(
                    ((_501 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0 as uint32_t
                        == 0 as core::ffi::c_uint) as core::ffi::c_int
                        as bool_0,
                    0,
                    -(1 as core::ffi::c_int) as uint32_t,
                );
                _503 = _187;
                _504 = ((_503 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0 as uint32_t
                    == 0 as core::ffi::c_uint) as core::ffi::c_int as bool_0
                    as uint64_t;
                *(&raw mut *((*(_500 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = llvm_select_u32(
                    ((_503 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0 as uint32_t
                        == 0 as core::ffi::c_uint) as core::ffi::c_int
                        as bool_0,
                    0,
                    -(1 as core::ffi::c_int) as uint32_t,
                );
                _505 = _188;
                _506 = ((_505 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0 as uint32_t
                    == 0 as core::ffi::c_uint) as core::ffi::c_int as bool_0
                    as uint64_t;
                *(&raw mut *((*(_500 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = llvm_select_u32(
                    ((_505 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0 as uint32_t
                        == 0 as core::ffi::c_uint) as core::ffi::c_int
                        as bool_0,
                    0,
                    -(1 as core::ffi::c_int) as uint32_t,
                );
                _507 = _189;
                _508 = ((_507 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0 as uint32_t
                    == 0 as core::ffi::c_uint) as core::ffi::c_int as bool_0
                    as uint64_t;
                *(&raw mut *((*(_500 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = llvm_select_u32(
                    ((_507 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0 as uint32_t
                        == 0 as core::ffi::c_uint) as core::ffi::c_int
                        as bool_0,
                    0,
                    -(1 as core::ffi::c_int) as uint32_t,
                );
                _509 = *(&mut _256 as *mut l_struct_struct_OC_vmask4
                    as *mut crate::l_array_2_uint64_t);
                (*(&mut _322.field0 as *mut crate::l_array_4_uint32_t as *mut C2RustUnnamed_4))
                    .data = _509;
                _510 = memcpy(
                    &mut _314 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void,
                    &mut _322 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void,
                    16,
                );
                _511 = memcpy(
                    &mut _326 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _312 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _512 = memcpy(
                    &mut _327 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _313 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _513 = memcpy(
                    &mut _328 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void,
                    &mut _314 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void,
                    16,
                );
                _514 = *(&mut _326.field0 as *mut crate::l_array_4_float);
                _515 = *(&mut _327.field0 as *mut crate::l_array_4_float);
                _516 = (*(&mut _328.field0 as *mut crate::l_array_4_uint32_t
                    as *mut C2RustUnnamed_3))
                    .data;
                *(&mut _212 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _514;
                *(&mut _213 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _515;
                *(&mut _214 as *mut l_struct_struct_OC_vmask4 as *mut crate::l_array_2_uint64_t) =
                    _516;
                _517 = *(&mut _214 as *mut l_struct_struct_OC_vmask4 as *mut uint32_t);
                if _517 & 2147483648 as core::ffi::c_uint != 0 as core::ffi::c_uint {
                    _518 =
                        *(&mut _213 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _520__PHI_TEMPORARY = _518;
                } else {
                    _519 =
                        *(&mut _212 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _520__PHI_TEMPORARY = _519;
                }
                _520 = _520__PHI_TEMPORARY;
                _521 = *(&raw mut *((*(&mut _214 as *mut l_struct_struct_OC_vmask4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                if _521 & 2147483648 as core::ffi::c_uint != 0 as core::ffi::c_uint {
                    _522 = *(&raw mut *((*(&mut _213 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _524__PHI_TEMPORARY = _522;
                } else {
                    _523 = *(&raw mut *((*(&mut _212 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _524__PHI_TEMPORARY = _523;
                }
                _524 = _524__PHI_TEMPORARY;
                _525 = *(&raw mut *((*(&mut _214 as *mut l_struct_struct_OC_vmask4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                if _525 & 2147483648 as core::ffi::c_uint != 0 as core::ffi::c_uint {
                    _526 = *(&raw mut *((*(&mut _213 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _528__PHI_TEMPORARY = _526;
                } else {
                    _527 = *(&raw mut *((*(&mut _212 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _528__PHI_TEMPORARY = _527;
                }
                _528 = _528__PHI_TEMPORARY;
                _529 = *(&raw mut *((*(&mut _214 as *mut l_struct_struct_OC_vmask4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                if _529 & 2147483648 as core::ffi::c_uint != 0 as core::ffi::c_uint {
                    _530 = *(&raw mut *((*(&mut _213 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _532__PHI_TEMPORARY = _530;
                } else {
                    _531 = *(&raw mut *((*(&mut _212 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _532__PHI_TEMPORARY = _531;
                }
                _532 = _532__PHI_TEMPORARY;
                _206 = &mut _211 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _207 = _520;
                _208 = _524;
                _209 = _528;
                _210 = _532;
                _533 = _206;
                _534 = _207;
                *(_533 as *mut core::ffi::c_float) = _534;
                _535 = _208;
                *(&raw mut *((*(_533 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _535;
                _536 = _209;
                *(&raw mut *((*(_533 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _536;
                _537 = _210;
                *(&raw mut *((*(_533 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _537;
                _538 = _211;
                *(&mut _325.field0 as *mut crate::l_array_4_float) = _538.field0;
                _539 = memcpy(
                    &mut _312 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _325 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _540 = _310;
                _541 = _307;
                _270 = &mut _329 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _271 = _540;
                _272 = _541;
                _273 = 0 as core::ffi::c_int as core::ffi::c_float;
                _274 = 0 as core::ffi::c_int as core::ffi::c_float;
                _542 = _270;
                _543 = _271;
                *(_542 as *mut core::ffi::c_float) = _543;
                _544 = _272;
                *(&raw mut *((*(_542 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _544;
                _545 = _273;
                *(&raw mut *((*(_542 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _545;
                _546 = _274;
                *(&raw mut *((*(_542 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _546;
                _547 = memcpy(
                    &mut _313 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _329 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _251 = &mut _312 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _548 = _251;
                _549 = *(_548 as *mut core::ffi::c_float);
                _239 = &mut _331 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _240 = _549;
                _550 = _239;
                _551 = _240;
                *(_550 as *mut core::ffi::c_float) = _551;
                _552 = _240;
                *(&raw mut *((*(_550 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _552;
                _553 = _240;
                *(&raw mut *((*(_550 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _553;
                _554 = _240;
                *(&raw mut *((*(_550 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _554;
                _555 = _310;
                _237 = &mut _332 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _238 = _555;
                _556 = _237;
                _557 = _238;
                *(_556 as *mut core::ffi::c_float) = _557;
                _558 = _238;
                *(&raw mut *((*(_556 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _558;
                _559 = _238;
                *(&raw mut *((*(_556 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _559;
                _560 = _238;
                *(&raw mut *((*(_556 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _560;
                _561 = *(&mut _331.field0 as *mut crate::l_array_4_float);
                _562 = *(&mut _332.field0 as *mut crate::l_array_4_float);
                *(&mut _260 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _561;
                *(&mut _261 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _562;
                _563 = *(&mut _260 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _564 = *(&mut _261 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _565 = *(&raw mut *((*(&mut _260 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _566 = *(&raw mut *((*(&mut _261 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _567 = *(&raw mut *((*(&mut _260 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _568 = *(&raw mut *((*(&mut _261 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _569 = *(&raw mut *((*(&mut _260 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _570 = *(&raw mut *((*(&mut _261 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _180 = &mut _259 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void;
                _181 = llvm_fcmp_ogt(_563 as core::ffi::c_double, _564 as core::ffi::c_double)
                    as bool_0;
                _182 = llvm_fcmp_ogt(_565 as core::ffi::c_double, _566 as core::ffi::c_double)
                    as bool_0;
                _183 = llvm_fcmp_ogt(_567 as core::ffi::c_double, _568 as core::ffi::c_double)
                    as bool_0;
                _184 = llvm_fcmp_ogt(_569 as core::ffi::c_double, _570 as core::ffi::c_double)
                    as bool_0;
                _571 = _180;
                _572 = _181;
                _573 = ((_572 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0 as uint32_t
                    == 0 as core::ffi::c_uint) as core::ffi::c_int as bool_0
                    as uint64_t;
                *(_571 as *mut uint32_t) = llvm_select_u32(
                    ((_572 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0 as uint32_t
                        == 0 as core::ffi::c_uint) as core::ffi::c_int
                        as bool_0,
                    0,
                    -(1 as core::ffi::c_int) as uint32_t,
                );
                _574 = _182;
                _575 = ((_574 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0 as uint32_t
                    == 0 as core::ffi::c_uint) as core::ffi::c_int as bool_0
                    as uint64_t;
                *(&raw mut *((*(_571 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = llvm_select_u32(
                    ((_574 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0 as uint32_t
                        == 0 as core::ffi::c_uint) as core::ffi::c_int
                        as bool_0,
                    0,
                    -(1 as core::ffi::c_int) as uint32_t,
                );
                _576 = _183;
                _577 = ((_576 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0 as uint32_t
                    == 0 as core::ffi::c_uint) as core::ffi::c_int as bool_0
                    as uint64_t;
                *(&raw mut *((*(_571 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = llvm_select_u32(
                    ((_576 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0 as uint32_t
                        == 0 as core::ffi::c_uint) as core::ffi::c_int
                        as bool_0,
                    0,
                    -(1 as core::ffi::c_int) as uint32_t,
                );
                _578 = _184;
                _579 = ((_578 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0 as uint32_t
                    == 0 as core::ffi::c_uint) as core::ffi::c_int as bool_0
                    as uint64_t;
                *(&raw mut *((*(_571 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = llvm_select_u32(
                    ((_578 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0 as uint32_t
                        == 0 as core::ffi::c_uint) as core::ffi::c_int
                        as bool_0,
                    0,
                    -(1 as core::ffi::c_int) as uint32_t,
                );
                _580 = *(&mut _259 as *mut l_struct_struct_OC_vmask4
                    as *mut crate::l_array_2_uint64_t);
                (*(&mut _330.field0 as *mut crate::l_array_4_uint32_t as *mut C2RustUnnamed_2))
                    .data = _580;
                _581 = memcpy(
                    &mut _314 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void,
                    &mut _330 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void,
                    16,
                );
                _582 = memcpy(
                    &mut _334 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _312 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _583 = memcpy(
                    &mut _335 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _313 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _584 = memcpy(
                    &mut _336 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void,
                    &mut _314 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void,
                    16,
                );
                _585 = *(&mut _334.field0 as *mut crate::l_array_4_float);
                _586 = *(&mut _335.field0 as *mut crate::l_array_4_float);
                _587 = (*(&mut _336.field0 as *mut crate::l_array_4_uint32_t
                    as *mut C2RustUnnamed_1))
                    .data;
                *(&mut _221 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _585;
                *(&mut _222 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _586;
                *(&mut _223 as *mut l_struct_struct_OC_vmask4 as *mut crate::l_array_2_uint64_t) =
                    _587;
                _588 = *(&mut _223 as *mut l_struct_struct_OC_vmask4 as *mut uint32_t);
                if _588 & 2147483648 as core::ffi::c_uint != 0 as core::ffi::c_uint {
                    _589 =
                        *(&mut _222 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _591__PHI_TEMPORARY = _589;
                } else {
                    _590 =
                        *(&mut _221 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _591__PHI_TEMPORARY = _590;
                }
                _591 = _591__PHI_TEMPORARY;
                _592 = *(&raw mut *((*(&mut _223 as *mut l_struct_struct_OC_vmask4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                if _592 & 2147483648 as core::ffi::c_uint != 0 as core::ffi::c_uint {
                    _593 = *(&raw mut *((*(&mut _222 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _595__PHI_TEMPORARY = _593;
                } else {
                    _594 = *(&raw mut *((*(&mut _221 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _595__PHI_TEMPORARY = _594;
                }
                _595 = _595__PHI_TEMPORARY;
                _596 = *(&raw mut *((*(&mut _223 as *mut l_struct_struct_OC_vmask4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                if _596 & 2147483648 as core::ffi::c_uint != 0 as core::ffi::c_uint {
                    _597 = *(&raw mut *((*(&mut _222 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _599__PHI_TEMPORARY = _597;
                } else {
                    _598 = *(&raw mut *((*(&mut _221 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _599__PHI_TEMPORARY = _598;
                }
                _599 = _599__PHI_TEMPORARY;
                _600 = *(&raw mut *((*(&mut _223 as *mut l_struct_struct_OC_vmask4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                if _600 & 2147483648 as core::ffi::c_uint != 0 as core::ffi::c_uint {
                    _601 = *(&raw mut *((*(&mut _222 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _603__PHI_TEMPORARY = _601;
                } else {
                    _602 = *(&raw mut *((*(&mut _221 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _603__PHI_TEMPORARY = _602;
                }
                _603 = _603__PHI_TEMPORARY;
                _215 = &mut _220 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _216 = _591;
                _217 = _595;
                _218 = _599;
                _219 = _603;
                _604 = _215;
                _605 = _216;
                *(_604 as *mut core::ffi::c_float) = _605;
                _606 = _217;
                *(&raw mut *((*(_604 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _606;
                _607 = _218;
                *(&raw mut *((*(_604 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _607;
                _608 = _219;
                *(&raw mut *((*(_604 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _608;
                _609 = _220;
                *(&mut _333.field0 as *mut crate::l_array_4_float) = _609.field0;
                _610 = _308;
                _611 = memcpy(
                    &raw mut *(_303.array).as_mut_ptr().offset((llvm_sub_u32
                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        _610, 1
                    ) as int32_t
                        as int64_t
                        as isize) as *mut l_struct_struct_OC_vfloat4
                        as *mut core::ffi::c_void,
                    &mut _333 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _612 = _308;
                _613 = memcpy(
                    &mut _312 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &raw mut *(_303.array).as_mut_ptr().offset((llvm_sub_u32
                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        _612, 2
                    ) as int32_t
                        as int64_t
                        as isize) as *mut l_struct_struct_OC_vfloat4
                        as *mut core::ffi::c_void,
                    16,
                );
                _614 = _311;
                _615 = _307;
                _265 = &mut _337 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _266 = _614;
                _267 = _615;
                _268 = 1 as core::ffi::c_int as core::ffi::c_float;
                _269 = 0 as core::ffi::c_int as core::ffi::c_float;
                _616 = _265;
                _617 = _266;
                *(_616 as *mut core::ffi::c_float) = _617;
                _618 = _267;
                *(&raw mut *((*(_616 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _618;
                _619 = _268;
                *(&raw mut *((*(_616 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _619;
                _620 = _269;
                *(&raw mut *((*(_616 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _620;
                _621 = memcpy(
                    &mut _313 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _337 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _252 = &mut _312 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _622 = _252;
                _623 = *(_622 as *mut core::ffi::c_float);
                _235 = &mut _339 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _236 = _623;
                _624 = _235;
                _625 = _236;
                *(_624 as *mut core::ffi::c_float) = _625;
                _626 = _236;
                *(&raw mut *((*(_624 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _626;
                _627 = _236;
                *(&raw mut *((*(_624 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _627;
                _628 = _236;
                *(&raw mut *((*(_624 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _628;
                _629 = _311;
                _233 = &mut _340 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _234 = _629;
                _630 = _233;
                _631 = _234;
                *(_630 as *mut core::ffi::c_float) = _631;
                _632 = _234;
                *(&raw mut *((*(_630 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _632;
                _633 = _234;
                *(&raw mut *((*(_630 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _633;
                _634 = _234;
                *(&raw mut *((*(_630 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _634;
                _635 = *(&mut _339.field0 as *mut crate::l_array_4_float);
                _636 = *(&mut _340.field0 as *mut crate::l_array_4_float);
                *(&mut _263 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _635;
                *(&mut _264 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _636;
                _637 = *(&mut _263 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _638 = *(&mut _264 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _639 = *(&raw mut *((*(&mut _263 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _640 = *(&raw mut *((*(&mut _264 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _641 = *(&raw mut *((*(&mut _263 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _642 = *(&raw mut *((*(&mut _264 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _643 = *(&raw mut *((*(&mut _263 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _644 = *(&raw mut *((*(&mut _264 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _175 = &mut _262 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void;
                _176 = llvm_fcmp_ogt(_637 as core::ffi::c_double, _638 as core::ffi::c_double)
                    as bool_0;
                _177 = llvm_fcmp_ogt(_639 as core::ffi::c_double, _640 as core::ffi::c_double)
                    as bool_0;
                _178 = llvm_fcmp_ogt(_641 as core::ffi::c_double, _642 as core::ffi::c_double)
                    as bool_0;
                _179 = llvm_fcmp_ogt(_643 as core::ffi::c_double, _644 as core::ffi::c_double)
                    as bool_0;
                _645 = _175;
                _646 = _176;
                _647 = ((_646 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0 as uint32_t
                    == 0 as core::ffi::c_uint) as core::ffi::c_int as bool_0
                    as uint64_t;
                *(_645 as *mut uint32_t) = llvm_select_u32(
                    ((_646 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0 as uint32_t
                        == 0 as core::ffi::c_uint) as core::ffi::c_int
                        as bool_0,
                    0,
                    -(1 as core::ffi::c_int) as uint32_t,
                );
                _648 = _177;
                _649 = ((_648 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0 as uint32_t
                    == 0 as core::ffi::c_uint) as core::ffi::c_int as bool_0
                    as uint64_t;
                *(&raw mut *((*(_645 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = llvm_select_u32(
                    ((_648 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0 as uint32_t
                        == 0 as core::ffi::c_uint) as core::ffi::c_int
                        as bool_0,
                    0,
                    -(1 as core::ffi::c_int) as uint32_t,
                );
                _650 = _178;
                _651 = ((_650 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0 as uint32_t
                    == 0 as core::ffi::c_uint) as core::ffi::c_int as bool_0
                    as uint64_t;
                *(&raw mut *((*(_645 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = llvm_select_u32(
                    ((_650 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0 as uint32_t
                        == 0 as core::ffi::c_uint) as core::ffi::c_int
                        as bool_0,
                    0,
                    -(1 as core::ffi::c_int) as uint32_t,
                );
                _652 = _179;
                _653 = ((_652 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0 as uint32_t
                    == 0 as core::ffi::c_uint) as core::ffi::c_int as bool_0
                    as uint64_t;
                *(&raw mut *((*(_645 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = llvm_select_u32(
                    ((_652 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0 as uint32_t
                        == 0 as core::ffi::c_uint) as core::ffi::c_int
                        as bool_0,
                    0,
                    -(1 as core::ffi::c_int) as uint32_t,
                );
                _654 = *(&mut _262 as *mut l_struct_struct_OC_vmask4
                    as *mut crate::l_array_2_uint64_t);
                (*(&mut _338.field0 as *mut crate::l_array_4_uint32_t as *mut C2RustUnnamed_0))
                    .data = _654;
                _655 = memcpy(
                    &mut _314 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void,
                    &mut _338 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void,
                    16,
                );
                _656 = memcpy(
                    &mut _342 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _312 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _657 = memcpy(
                    &mut _343 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _313 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _658 = memcpy(
                    &mut _344 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void,
                    &mut _314 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void,
                    16,
                );
                _659 = *(&mut _342.field0 as *mut crate::l_array_4_float);
                _660 = *(&mut _343.field0 as *mut crate::l_array_4_float);
                _661 = (*(&mut _344.field0 as *mut crate::l_array_4_uint32_t
                    as *mut C2RustUnnamed))
                    .data;
                *(&mut _230 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _659;
                *(&mut _231 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _660;
                *(&mut _232 as *mut l_struct_struct_OC_vmask4 as *mut crate::l_array_2_uint64_t) =
                    _661;
                _662 = *(&mut _232 as *mut l_struct_struct_OC_vmask4 as *mut uint32_t);
                if _662 & 2147483648 as core::ffi::c_uint != 0 as core::ffi::c_uint {
                    _663 =
                        *(&mut _231 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _665__PHI_TEMPORARY = _663;
                } else {
                    _664 =
                        *(&mut _230 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _665__PHI_TEMPORARY = _664;
                }
                _665 = _665__PHI_TEMPORARY;
                _666 = *(&raw mut *((*(&mut _232 as *mut l_struct_struct_OC_vmask4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                if _666 & 2147483648 as core::ffi::c_uint != 0 as core::ffi::c_uint {
                    _667 = *(&raw mut *((*(&mut _231 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _669__PHI_TEMPORARY = _667;
                } else {
                    _668 = *(&raw mut *((*(&mut _230 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _669__PHI_TEMPORARY = _668;
                }
                _669 = _669__PHI_TEMPORARY;
                _670 = *(&raw mut *((*(&mut _232 as *mut l_struct_struct_OC_vmask4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                if _670 & 2147483648 as core::ffi::c_uint != 0 as core::ffi::c_uint {
                    _671 = *(&raw mut *((*(&mut _231 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _673__PHI_TEMPORARY = _671;
                } else {
                    _672 = *(&raw mut *((*(&mut _230 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _673__PHI_TEMPORARY = _672;
                }
                _673 = _673__PHI_TEMPORARY;
                _674 = *(&raw mut *((*(&mut _232 as *mut l_struct_struct_OC_vmask4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                if _674 & 2147483648 as core::ffi::c_uint != 0 as core::ffi::c_uint {
                    _675 = *(&raw mut *((*(&mut _231 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _677__PHI_TEMPORARY = _675;
                } else {
                    _676 = *(&raw mut *((*(&mut _230 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _677__PHI_TEMPORARY = _676;
                }
                _677 = _677__PHI_TEMPORARY;
                _224 = &mut _229 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _225 = _665;
                _226 = _669;
                _227 = _673;
                _228 = _677;
                _678 = _224;
                _679 = _225;
                *(_678 as *mut core::ffi::c_float) = _679;
                _680 = _226;
                *(&raw mut *((*(_678 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _680;
                _681 = _227;
                *(&raw mut *((*(_678 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _681;
                _682 = _228;
                *(&raw mut *((*(_678 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _682;
                _683 = _229;
                *(&mut _341.field0 as *mut crate::l_array_4_float) = _683.field0;
                _684 = _308;
                _685 = memcpy(
                    &raw mut *(_303.array).as_mut_ptr().offset((llvm_sub_u32
                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        _684, 2
                    ) as int32_t
                        as int64_t
                        as isize) as *mut l_struct_struct_OC_vfloat4
                        as *mut core::ffi::c_void,
                    &mut _341 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _686 = _306;
                _306 = llvm_add_u32(_686, 1);
            }
            _345 = 0;
            loop {
                _687 = _345;
                _688 = _292;
                if !(_687 <= _688) {
                    break;
                }
                _689 = _345;
                _690 = *(&raw const *(_ZL21steps_for_quant_level.array)
                    .as_ptr()
                    .offset(_689 as uint64_t as int64_t as isize)
                    as *const uint8_t as *mut uint8_t);
                _346 = _690 as uint32_t;
                _691 = _346;
                _196 = &raw mut *(_303.array)
                    .as_mut_ptr()
                    .offset(_691 as uint64_t as int64_t as isize)
                    as *mut l_struct_struct_OC_vfloat4
                    as *mut core::ffi::c_void;
                _692 = _196;
                _693 = *(&raw mut *((*(_692 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _347 = _693 as int32_t as uint32_t;
                _694 = _347;
                _695 = _ZN4astcL3maxIiEET_S1_S1_(0, _694);
                _347 = _695;
                _696 = _347;
                _697 = *(&raw mut *(_298.array)
                    .as_mut_ptr()
                    .offset(_696 as int32_t as int64_t as isize)
                    as *mut core::ffi::c_float);
                _698 = _346;
                _195 = &raw mut *(_303.array)
                    .as_mut_ptr()
                    .offset(_698 as uint64_t as int64_t as isize)
                    as *mut l_struct_struct_OC_vfloat4
                    as *mut core::ffi::c_void;
                _699 = _195;
                _700 = *(&raw mut *((*(_699 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _348 = llvm_fadd_f32(_697, _700);
                _701 = _348;
                _702 = _346;
                _349 = llvm_fsub_f32(
                    llvm_fadd_f32(_701, _702 as core::ffi::c_float),
                    1 as core::ffi::c_int as core::ffi::c_float,
                );
                _703 = _347;
                _350 = llvm_fdiv_f32(
                    1 as core::ffi::c_int as core::ffi::c_float,
                    llvm_fadd_f32(
                        1 as core::ffi::c_int as core::ffi::c_float,
                        _703 as int32_t as core::ffi::c_float,
                    ),
                );
                _704 = _347;
                _705 = *(&raw mut *(_297.array)
                    .as_mut_ptr()
                    .offset(_704 as int32_t as int64_t as isize)
                    as *mut core::ffi::c_float);
                _706 = _348;
                _707 = _350;
                _708 = _293;
                _709 = _345;
                *(&raw mut *(_708 as *mut core::ffi::c_float)
                    .offset(_709 as uint64_t as int64_t as isize)
                    as *mut core::ffi::c_float) = llvm_fmul_f32(llvm_fadd_f32(_705, _706), _707);
                _710 = _347;
                _711 = *(&raw mut *(_297.array)
                    .as_mut_ptr()
                    .offset(_710 as int32_t as int64_t as isize)
                    as *mut core::ffi::c_float);
                _712 = _349;
                _713 = _350;
                _714 = _294;
                _715 = _345;
                *(&raw mut *(_714 as *mut core::ffi::c_float)
                    .offset(_715 as uint64_t as int64_t as isize)
                    as *mut core::ffi::c_float) = llvm_fmul_f32(llvm_fadd_f32(_711, _712), _713);
                _716 = _345;
                _345 = llvm_add_u32(_716, 1);
            }
            return;
        } else {
            __assert_fail(
                &_OC_str_OC_6 as *const crate::l_array_22_uint8_t as *mut core::ffi::c_void,
                &_OC_str_OC_1 as *const crate::l_array_51_uint8_t as *mut core::ffi::c_void,
                293,
                &__PRETTY_FUNCTION___OC__ZL42compute_angular_endpoints_for_quant_levelsjPKfjPfS1_
                    as *const crate::l_array_109_uint8_t as *mut core::ffi::c_void,
            );
        }
    } else {
        __assert_fail(
            &_OC_str_OC_5 as *const crate::l_array_20_uint8_t as *mut core::ffi::c_void,
            &_OC_str_OC_1 as *const crate::l_array_51_uint8_t as *mut core::ffi::c_void,
            284,
            &__PRETTY_FUNCTION___OC__ZL42compute_angular_endpoints_for_quant_levelsjPKfjPfS1_
                as *const crate::l_array_109_uint8_t as *mut core::ffi::c_void,
        );
    };
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn _Z33compute_angular_endpoints_2planesRK21block_size_descriptorPKfjR27compression_working_buffers(
    mut _783: *mut core::ffi::c_void,
    mut _784: *mut core::ffi::c_void,
    mut _785: uint32_t,
    mut _786: *mut core::ffi::c_void,
) {
    let mut _787: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _788: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _789: uint32_t = 0;
    let mut _790: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _791: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _792: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _793: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _794: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _795: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _796: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _797: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _798: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _799: uint32_t = 0;
    let mut _800: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _801: uint32_t = 0;
    let mut _802: uint32_t = 0;
    let mut _803: uint32_t = 0;
    let mut _804: uint32_t = 0;
    let mut _805: uint32_t = 0;
    let mut _806: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _807: uint32_t = 0;
    let mut _808: uint32_t = 0;
    let mut _809: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _810: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _811: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _812: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _813: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _814: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _815: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _816: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _817: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _818: uint32_t = 0;
    let mut _819: uint32_t = 0;
    let mut _820: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _821: uint32_t = 0;
    let mut _822: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _823: uint32_t = 0;
    let mut _824: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _825: uint32_t = 0;
    let mut _826: bool_0 = 0;
    let mut _827: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _828: uint32_t = 0;
    let mut _829: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _830: uint8_t = 0;
    let mut _831: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _832: uint8_t = 0;
    let mut _833: uint32_t = 0;
    let mut _834: uint32_t = 0;
    let mut _835: uint32_t = 0;
    let mut _836: uint32_t = 0;
    let mut _837: uint32_t = 0;
    let mut _838: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _839: uint32_t = 0;
    let mut _840: uint32_t = 0;
    let mut _841: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _842: uint32_t = 0;
    let mut _843: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _844: uint32_t = 0;
    let mut _845: uint32_t = 0;
    let mut _846: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _847: uint32_t = 0;
    let mut _848: uint32_t = 0;
    let mut _849: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _850: uint32_t = 0;
    let mut _851: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _852: uint32_t = 0;
    let mut _853: uint32_t = 0;
    let mut _854: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _855: uint32_t = 0;
    let mut _856: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _857: uint32_t = 0;
    let mut _858: uint32_t = 0;
    let mut _859: uint32_t = 0;
    let mut _860: uint32_t = 0;
    let mut _861: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _862: uint32_t = 0;
    let mut _863: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _864: uint8_t = 0;
    let mut _865: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _866: uint8_t = 0;
    let mut _867: uint32_t = 0;
    let mut _868: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _869: uint32_t = 0;
    let mut _870: uint32_t = 0;
    let mut _871: core::ffi::c_float = 0.;
    let mut _872: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _873: uint32_t = 0;
    let mut _874: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _875: uint32_t = 0;
    let mut _876: uint32_t = 0;
    let mut _877: core::ffi::c_float = 0.;
    let mut _878: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _879: uint32_t = 0;
    let mut _880: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _881: uint32_t = 0;
    let mut _882: uint32_t = 0;
    let mut _883: core::ffi::c_float = 0.;
    let mut _884: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _885: uint32_t = 0;
    let mut _886: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _887: uint32_t = 0;
    let mut _888: uint32_t = 0;
    let mut _889: core::ffi::c_float = 0.;
    let mut _890: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _891: uint32_t = 0;
    let mut _892: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _893: uint32_t = 0;
    let mut _894: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _895: uint32_t = 0;
    let mut _896: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _897: uint32_t = 0;
    let mut _898: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _899: uint32_t = 0;
    let mut _900: uint32_t = 0;
    _787 = _783;
    _788 = _784;
    _789 = _785;
    _790 = _786;
    _809 = _790;
    _791 = &raw mut (*(_809 as *mut l_struct_struct_OC_compression_working_buffers)).field10
        as *mut crate::l_array_2048_float as *mut core::ffi::c_void;
    _810 = _790;
    _792 = &raw mut (*(_810 as *mut l_struct_struct_OC_compression_working_buffers)).field11
        as *mut crate::l_array_2048_float as *mut core::ffi::c_void;
    _811 = _790;
    _793 = &raw mut (*(_811 as *mut l_struct_struct_OC_compression_working_buffers)).field14
        as *mut crate::l_array_2048_float as *mut core::ffi::c_void;
    _812 = _790;
    _794 = &raw mut (*(_812 as *mut l_struct_struct_OC_compression_working_buffers)).field15
        as *mut crate::l_array_2048_float as *mut core::ffi::c_void;
    _813 = _790;
    _795 = &raw mut (*(_813 as *mut l_struct_struct_OC_compression_working_buffers)).field12
        as *mut crate::l_array_87_struct_AC_l_array_8_float as *mut core::ffi::c_void;
    _814 = _790;
    _796 = &raw mut (*(_814 as *mut l_struct_struct_OC_compression_working_buffers)).field13
        as *mut crate::l_array_87_struct_AC_l_array_8_float as *mut core::ffi::c_void;
    _815 = _790;
    _797 = &raw mut (*(_815 as *mut l_struct_struct_OC_compression_working_buffers)).field16
        as *mut crate::l_array_87_struct_AC_l_array_8_float as *mut core::ffi::c_void;
    _816 = _790;
    _798 = &raw mut (*(_816 as *mut l_struct_struct_OC_compression_working_buffers)).field17
        as *mut crate::l_array_87_struct_AC_l_array_8_float as *mut core::ffi::c_void;
    _817 = _787;
    _818 = *(&raw mut (*(_817 as *mut l_struct_struct_OC_block_size_descriptor)).field5
        as *mut uint32_t);
    if _818 > 0 as core::ffi::c_uint {
        _799 = 0;
        loop {
            _819 = _799;
            _820 = _787;
            _821 = *(&raw mut (*(_820 as *mut l_struct_struct_OC_block_size_descriptor)).field5
                as *mut uint32_t);
            if !(_819 < _821) {
                break;
            }
            _822 = _787;
            _823 = _799;
            _800 =
                &raw mut *((*(&raw mut (*(_822 as *mut l_struct_struct_OC_block_size_descriptor))
                    .field13
                    as *mut crate::l_array_87_struct_AC_l_struct_struct_OC_decimation_mode))
                    .array)
                    .as_mut_ptr()
                    .offset(_823 as uint64_t as int64_t as isize)
                    as *mut l_struct_struct_OC_decimation_mode
                    as *mut core::ffi::c_void;
            _824 = _800;
            _825 = _789;
            _826 = _ZNK15decimation_mode13is_ref_2planeE12quant_method(_824, _825);
            if _826 != 0 {
                _827 = _787;
                _828 = _799;
                _829 = _ZNK21block_size_descriptor19get_decimation_infoEj(_827, _828);
                _830 = *(&raw mut (*(_829 as *mut l_struct_struct_OC_decimation_info)).field2
                    as *mut uint8_t);
                _801 = _830 as uint32_t;
                _831 = _800;
                _832 = *(&raw mut (*(_831 as *mut l_struct_struct_OC_decimation_mode)).field1
                    as *mut uint8_t);
                _802 = _832 as int8_t as int32_t as uint32_t;
                _833 = _802;
                if _833 > 7 as core::ffi::c_uint {
                    _802 = 7;
                }
                _834 = _802;
                _835 = _789;
                if _834 > _835 {
                    _836 = _789;
                    _802 = _836;
                }
                _837 = _801;
                _838 = _788;
                _839 = _799;
                _840 = _802;
                _841 = _795;
                _842 = _799;
                _843 = _796;
                _844 = _799;
                _ZL42compute_angular_endpoints_for_quant_levelsjPKfjPfS1_(
                    _837,
                    &raw mut *(_838 as *mut core::ffi::c_float).offset((llvm_mul_u32
                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        _839, 64
                    )
                        as uint64_t
                        as int64_t
                        as isize) as *mut core::ffi::c_float
                        as *mut core::ffi::c_void,
                    _840,
                    &raw mut *((*(&raw mut *((*(_841
                        as *mut crate::l_array_87_struct_AC_l_array_8_float))
                        .array)
                        .as_mut_ptr()
                        .offset(_842 as uint64_t as int64_t as isize)
                        as *mut crate::l_array_8_float))
                        .array)
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float as *mut core::ffi::c_void,
                    &raw mut *((*(&raw mut *((*(_843
                        as *mut crate::l_array_87_struct_AC_l_array_8_float))
                        .array)
                        .as_mut_ptr()
                        .offset(_844 as uint64_t as int64_t as isize)
                        as *mut crate::l_array_8_float))
                        .array)
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float as *mut core::ffi::c_void,
                );
                _845 = _801;
                _846 = _788;
                _847 = _799;
                _848 = _802;
                _849 = _797;
                _850 = _799;
                _851 = _798;
                _852 = _799;
                _ZL42compute_angular_endpoints_for_quant_levelsjPKfjPfS1_(
                    _845,
                    &raw mut *(&raw mut *(_846 as *mut core::ffi::c_float).offset((llvm_mul_u32
                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        _847, 64
                    )
                        as uint64_t
                        as int64_t
                        as isize) as *mut core::ffi::c_float)
                        .offset(32 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float as *mut core::ffi::c_void,
                    _848,
                    &raw mut *((*(&raw mut *((*(_849
                        as *mut crate::l_array_87_struct_AC_l_array_8_float))
                        .array)
                        .as_mut_ptr()
                        .offset(_850 as uint64_t as int64_t as isize)
                        as *mut crate::l_array_8_float))
                        .array)
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float as *mut core::ffi::c_void,
                    &raw mut *((*(&raw mut *((*(_851
                        as *mut crate::l_array_87_struct_AC_l_array_8_float))
                        .array)
                        .as_mut_ptr()
                        .offset(_852 as uint64_t as int64_t as isize)
                        as *mut crate::l_array_8_float))
                        .array)
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float as *mut core::ffi::c_void,
                );
            }
            _853 = _799;
            _799 = llvm_add_u32(_853, 1);
        }
        _854 = _787;
        _855 = *(&raw mut (*(_854 as *mut l_struct_struct_OC_block_size_descriptor)).field8
            as *mut uint32_t);
        _803 = _855;
        _856 = _787;
        _857 = *(&raw mut (*(_856 as *mut l_struct_struct_OC_block_size_descriptor)).field9
            as *mut uint32_t);
        _804 = _857;
        _858 = _803;
        _805 = _858;
        loop {
            _859 = _805;
            _860 = _804;
            if !(_859 < _860) {
                break;
            }
            _861 = _787;
            _862 = _805;
            _806 =
                &raw mut *((*(&raw mut (*(_861 as *mut l_struct_struct_OC_block_size_descriptor))
                    .field16
                    as *mut crate::l_array_2048_struct_AC_l_struct_struct_OC_block_mode))
                    .array)
                    .as_mut_ptr()
                    .offset(_862 as uint64_t as int64_t as isize)
                    as *mut l_struct_struct_OC_block_mode as *mut core::ffi::c_void;
            _863 = _806;
            _864 =
                *(&raw mut (*(_863 as *mut l_struct_struct_OC_block_mode)).field2 as *mut uint8_t);
            _807 = _864 as uint32_t;
            _865 = _806;
            _866 =
                *(&raw mut (*(_865 as *mut l_struct_struct_OC_block_mode)).field1 as *mut uint8_t);
            _808 = _866 as uint32_t;
            _867 = _807;
            if _867 <= 7 as core::ffi::c_uint {
                _868 = _795;
                _869 = _808;
                _870 = _807;
                _871 = *(&raw mut *((*(&raw mut *((*(_868
                    as *mut crate::l_array_87_struct_AC_l_array_8_float))
                    .array)
                    .as_mut_ptr()
                    .offset(_869 as uint64_t as int64_t as isize)
                    as *mut crate::l_array_8_float))
                    .array)
                    .as_mut_ptr()
                    .offset(_870 as uint64_t as int64_t as isize)
                    as *mut core::ffi::c_float);
                _872 = _791;
                _873 = _805;
                *(&raw mut *((*(_872 as *mut crate::l_array_2048_float)).array)
                    .as_mut_ptr()
                    .offset(_873 as uint64_t as int64_t as isize)
                    as *mut core::ffi::c_float) = _871;
                _874 = _796;
                _875 = _808;
                _876 = _807;
                _877 = *(&raw mut *((*(&raw mut *((*(_874
                    as *mut crate::l_array_87_struct_AC_l_array_8_float))
                    .array)
                    .as_mut_ptr()
                    .offset(_875 as uint64_t as int64_t as isize)
                    as *mut crate::l_array_8_float))
                    .array)
                    .as_mut_ptr()
                    .offset(_876 as uint64_t as int64_t as isize)
                    as *mut core::ffi::c_float);
                _878 = _792;
                _879 = _805;
                *(&raw mut *((*(_878 as *mut crate::l_array_2048_float)).array)
                    .as_mut_ptr()
                    .offset(_879 as uint64_t as int64_t as isize)
                    as *mut core::ffi::c_float) = _877;
                _880 = _797;
                _881 = _808;
                _882 = _807;
                _883 = *(&raw mut *((*(&raw mut *((*(_880
                    as *mut crate::l_array_87_struct_AC_l_array_8_float))
                    .array)
                    .as_mut_ptr()
                    .offset(_881 as uint64_t as int64_t as isize)
                    as *mut crate::l_array_8_float))
                    .array)
                    .as_mut_ptr()
                    .offset(_882 as uint64_t as int64_t as isize)
                    as *mut core::ffi::c_float);
                _884 = _793;
                _885 = _805;
                *(&raw mut *((*(_884 as *mut crate::l_array_2048_float)).array)
                    .as_mut_ptr()
                    .offset(_885 as uint64_t as int64_t as isize)
                    as *mut core::ffi::c_float) = _883;
                _886 = _798;
                _887 = _808;
                _888 = _807;
                _889 = *(&raw mut *((*(&raw mut *((*(_886
                    as *mut crate::l_array_87_struct_AC_l_array_8_float))
                    .array)
                    .as_mut_ptr()
                    .offset(_887 as uint64_t as int64_t as isize)
                    as *mut crate::l_array_8_float))
                    .array)
                    .as_mut_ptr()
                    .offset(_888 as uint64_t as int64_t as isize)
                    as *mut core::ffi::c_float);
                _890 = _794;
                _891 = _805;
                *(&raw mut *((*(_890 as *mut crate::l_array_2048_float)).array)
                    .as_mut_ptr()
                    .offset(_891 as uint64_t as int64_t as isize)
                    as *mut core::ffi::c_float) = _889;
            } else {
                _892 = _791;
                _893 = _805;
                *(&raw mut *((*(_892 as *mut crate::l_array_2048_float)).array)
                    .as_mut_ptr()
                    .offset(_893 as uint64_t as int64_t as isize)
                    as *mut core::ffi::c_float) = 0 as core::ffi::c_int as core::ffi::c_float;
                _894 = _792;
                _895 = _805;
                *(&raw mut *((*(_894 as *mut crate::l_array_2048_float)).array)
                    .as_mut_ptr()
                    .offset(_895 as uint64_t as int64_t as isize)
                    as *mut core::ffi::c_float) = 1 as core::ffi::c_int as core::ffi::c_float;
                _896 = _793;
                _897 = _805;
                *(&raw mut *((*(_896 as *mut crate::l_array_2048_float)).array)
                    .as_mut_ptr()
                    .offset(_897 as uint64_t as int64_t as isize)
                    as *mut core::ffi::c_float) = 0 as core::ffi::c_int as core::ffi::c_float;
                _898 = _794;
                _899 = _805;
                *(&raw mut *((*(_898 as *mut crate::l_array_2048_float)).array)
                    .as_mut_ptr()
                    .offset(_899 as uint64_t as int64_t as isize)
                    as *mut core::ffi::c_float) = 1 as core::ffi::c_int as core::ffi::c_float;
            }
            _900 = _805;
            _805 = llvm_add_u32(_900, 1);
        }
        return;
    } else {
        __assert_fail(
            &_OC_str_OC_4 as *const crate::l_array_39_uint8_t as *mut core::ffi::c_void,
            &_OC_str_OC_1 as *const crate::l_array_51_uint8_t as *mut core::ffi::c_void,
            438,
            &__PRETTY_FUNCTION___OC__Z33compute_angular_endpoints_2planesRK21block_size_descriptorPKfjR27compression_working_buffers
                as *const crate::l_array_130_uint8_t as *mut core::ffi::c_void,
        );
    };
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn _ZNK15decimation_mode13is_ref_2planeE12quant_method(
    mut _921: *mut core::ffi::c_void,
    mut _922: uint32_t,
) -> bool_0 {
    let mut _923: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _924: uint32_t = 0;
    let mut _925: uint16_t = 0;
    let mut _926: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _927: uint32_t = 0;
    let mut _928: uint16_t = 0;
    let mut _929: uint16_t = 0;
    _923 = _921;
    _924 = _922;
    _926 = _923;
    _927 = _924;
    _925 = llvm_sub_u32(
        ((1 as core::ffi::c_int) << llvm_add_u32(_927, 1)) as uint32_t,
        1,
    ) as uint16_t;
    _928 = *(&raw mut (*(_926 as *mut l_struct_struct_OC_decimation_mode)).field3 as *mut uint16_t);
    _929 = _925;
    return (_928 as uint32_t & _929 as uint32_t != 0 as core::ffi::c_uint) as core::ffi::c_int
        as bool_0;
}
#[inline(never)]
unsafe extern "C" fn _ZL23compute_angular_offsetsjPKfjPf(
    mut _930: uint32_t,
    mut _931: *mut core::ffi::c_void,
    mut _932: uint32_t,
    mut _933: *mut core::ffi::c_void,
) {
    let mut _934: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _935: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _936: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _937: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _938: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _939: uint32_t = 0;
    let mut _940: uint32_t = 0;
    let mut _941: uint32_t = 0;
    let mut _942: uint32_t = 0;
    let mut _943: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _944: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _945: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _946: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _947: uint32_t = 0;
    let mut _948: uint32_t = 0;
    let mut _949: uint32_t = 0;
    let mut _950: uint32_t = 0;
    let mut _951: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _952: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _953: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _954: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _955: uint32_t = 0;
    let mut _956: uint32_t = 0;
    let mut _957: uint32_t = 0;
    let mut _958: uint32_t = 0;
    let mut _959: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _960: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _961: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _962: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _963: uint32_t = 0;
    let mut _964: uint32_t = 0;
    let mut _965: uint32_t = 0;
    let mut _966: uint32_t = 0;
    let mut _967: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _968: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _969: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _970: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _971: uint32_t = 0;
    let mut _972: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _973: uint32_t = 0;
    let mut _974: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _975: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _976: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _977: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _978: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _979: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _980: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _981: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _982: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _983: uint8_t = 0;
    let mut _984: uint8_t = 0;
    let mut _985: uint8_t = 0;
    let mut _986: uint8_t = 0;
    let mut _987: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _988: uint8_t = 0;
    let mut _989: uint8_t = 0;
    let mut _990: uint8_t = 0;
    let mut _991: uint8_t = 0;
    let mut _992: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _993: core::ffi::c_float = 0.;
    let mut _994: core::ffi::c_float = 0.;
    let mut _995: core::ffi::c_float = 0.;
    let mut _996: core::ffi::c_float = 0.;
    let mut _997: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _998: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _999: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1000: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1001: core::ffi::c_float = 0.;
    let mut _1002: core::ffi::c_float = 0.;
    let mut _1003: core::ffi::c_float = 0.;
    let mut _1004: core::ffi::c_float = 0.;
    let mut _1005: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1006: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1007: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1008: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1009: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1010: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1011: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1012: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1013: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1014: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1015: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1016: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1017: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1018: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1019: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1020: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1021: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1022: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1023: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1024: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1025: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1026: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1027: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1028: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1029: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1030: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1031: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1032: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1033: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1034: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1035: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1036: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1037: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1038: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1039: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1040: core::ffi::c_float = 0.;
    let mut _1041: core::ffi::c_float = 0.;
    let mut _1042: core::ffi::c_float = 0.;
    let mut _1043: core::ffi::c_float = 0.;
    let mut _1044: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1045: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1046: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1047: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1048: core::ffi::c_float = 0.;
    let mut _1049: core::ffi::c_float = 0.;
    let mut _1050: core::ffi::c_float = 0.;
    let mut _1051: core::ffi::c_float = 0.;
    let mut _1052: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1053: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1054: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1055: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1056: core::ffi::c_float = 0.;
    let mut _1057: core::ffi::c_float = 0.;
    let mut _1058: core::ffi::c_float = 0.;
    let mut _1059: core::ffi::c_float = 0.;
    let mut _1060: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1061: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1062: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1063: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1064: core::ffi::c_float = 0.;
    let mut _1065: core::ffi::c_float = 0.;
    let mut _1066: core::ffi::c_float = 0.;
    let mut _1067: core::ffi::c_float = 0.;
    let mut _1068: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1069: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1070: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1071: core::ffi::c_float = 0.;
    let mut _1072: core::ffi::c_float = 0.;
    let mut _1073: core::ffi::c_float = 0.;
    let mut _1074: core::ffi::c_float = 0.;
    let mut _1075: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1076: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1077: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1078: core::ffi::c_float = 0.;
    let mut _1079: core::ffi::c_float = 0.;
    let mut _1080: core::ffi::c_float = 0.;
    let mut _1081: core::ffi::c_float = 0.;
    let mut _1082: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1083: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1084: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1085: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1086: core::ffi::c_float = 0.;
    let mut _1087: core::ffi::c_float = 0.;
    let mut _1088: core::ffi::c_float = 0.;
    let mut _1089: core::ffi::c_float = 0.;
    let mut _1090: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1091: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1092: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1093: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1094: core::ffi::c_float = 0.;
    let mut _1095: core::ffi::c_float = 0.;
    let mut _1096: core::ffi::c_float = 0.;
    let mut _1097: core::ffi::c_float = 0.;
    let mut _1098: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1099: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1100: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1101: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1102: core::ffi::c_float = 0.;
    let mut _1103: core::ffi::c_float = 0.;
    let mut _1104: core::ffi::c_float = 0.;
    let mut _1105: core::ffi::c_float = 0.;
    let mut _1106: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1107: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1108: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1109: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1110: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1111: core::ffi::c_float = 0.;
    let mut _1112: core::ffi::c_float = 0.;
    let mut _1113: core::ffi::c_float = 0.;
    let mut _1114: core::ffi::c_float = 0.;
    let mut _1115: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1116: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1117: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1118: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1119: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1120: core::ffi::c_float = 0.;
    let mut _1121: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1122: core::ffi::c_float = 0.;
    let mut _1123: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1124: core::ffi::c_float = 0.;
    let mut _1125: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1126: core::ffi::c_float = 0.;
    let mut _1127: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1128: core::ffi::c_float = 0.;
    let mut _1129: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1130: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1131: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1132: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1133: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1134: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1135: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1136: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1137: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1138: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1139: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1140: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1141: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1142: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1143: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1144: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1145: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1146: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1147: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1148: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1149: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1150: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1151: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1152: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1153: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1154: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1155: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1156: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1157: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1158: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1159: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1160: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1161: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1162: uint32_t = 0;
    let mut _1163: uint32_t = 0;
    let mut _1164: uint32_t = 0;
    let mut _1165: uint32_t = 0;
    let mut _1166: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1167: core::ffi::c_float = 0.;
    let mut _1168: core::ffi::c_float = 0.;
    let mut _1169: core::ffi::c_float = 0.;
    let mut _1170: core::ffi::c_float = 0.;
    let mut _1171: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1172: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1173: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1174: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1175: core::ffi::c_float = 0.;
    let mut _1176: core::ffi::c_float = 0.;
    let mut _1177: core::ffi::c_float = 0.;
    let mut _1178: core::ffi::c_float = 0.;
    let mut _1179: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1180: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1181: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1182: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1183: core::ffi::c_float = 0.;
    let mut _1184: core::ffi::c_float = 0.;
    let mut _1185: core::ffi::c_float = 0.;
    let mut _1186: core::ffi::c_float = 0.;
    let mut _1187: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1188: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1189: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1190: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1191: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1192: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1193: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1194: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1195: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1196: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1197: core::ffi::c_float = 0.;
    let mut _1198: core::ffi::c_float = 0.;
    let mut _1199: core::ffi::c_float = 0.;
    let mut _1200: core::ffi::c_float = 0.;
    let mut _1201: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1202: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1203: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1204: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1205: core::ffi::c_float = 0.;
    let mut _1206: core::ffi::c_float = 0.;
    let mut _1207: core::ffi::c_float = 0.;
    let mut _1208: core::ffi::c_float = 0.;
    let mut _1209: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1210: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1211: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1212: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1213: core::ffi::c_float = 0.;
    let mut _1214: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1215: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1216: core::ffi::c_float = 0.;
    let mut _1217: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1218: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1219: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1220: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1221: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1222: core::ffi::c_float = 0.;
    let mut _1223: core::ffi::c_float = 0.;
    let mut _1224: core::ffi::c_float = 0.;
    let mut _1225: core::ffi::c_float = 0.;
    let mut _1226: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1227: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1228: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1229: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1230: core::ffi::c_float = 0.;
    let mut _1231: core::ffi::c_float = 0.;
    let mut _1232: core::ffi::c_float = 0.;
    let mut _1233: core::ffi::c_float = 0.;
    let mut _1234: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1235: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1236: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1237: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1238: core::ffi::c_float = 0.;
    let mut _1239: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1240: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1241: core::ffi::c_float = 0.;
    let mut _1242: core::ffi::c_float = 0.;
    let mut _1243: core::ffi::c_float = 0.;
    let mut _1244: core::ffi::c_float = 0.;
    let mut _1245: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1246: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1247: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1248: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1249: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1250: core::ffi::c_float = 0.;
    let mut _1251: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1252: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1253: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1254: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1255: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1256: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1257: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1258: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1259: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1260: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1261: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1262: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1263: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1264: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1265: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1266: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1267: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1268: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1269: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1270: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1271: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1272: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1273: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1274: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1275: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1276: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1277: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1278: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1279: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1280: core::ffi::c_float = 0.;
    let mut _1281: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1282: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1283: core::ffi::c_float = 0.;
    let mut _1284: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1285: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1286: core::ffi::c_float = 0.;
    let mut _1287: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1288: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1289: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1290: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1291: core::ffi::c_float = 0.;
    let mut _1292: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1293: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1294: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1295: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1296: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1297: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1298: core::ffi::c_float = 0.;
    let mut _1299: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1300: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1301: core::ffi::c_float = 0.;
    let mut _1302: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1303: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1304: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1305: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1306: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1307: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1308: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1309: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1310: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1311: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1312: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1313: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1314: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1315: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1316: core::ffi::c_float = 0.;
    let mut _1317: uint32_t = 0;
    let mut _1318: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1319: uint32_t = 0;
    let mut _1320: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1321: crate::l_array_64_uint32_t = crate::l_array_64_uint32_t { array: [0; 64] };
    let mut _1322: uint32_t = 0;
    let mut _1323: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1324: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1325: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1326: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1327: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1328: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1329: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1330: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1331: uint32_t = 0;
    let mut _1332: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1333: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1334: uint32_t = 0;
    let mut _1335: uint32_t = 0;
    let mut _1336: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1337: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1338: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1339: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1340: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1341: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1342: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1343: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1344: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1345: uint32_t = 0;
    let mut _1346: uint32_t = 0;
    let mut _1347: uint32_t = 0;
    let mut _1348: uint32_t = 0;
    let mut _1349: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1350: uint32_t = 0;
    let mut _1351: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1352: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1353: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1354: core::ffi::c_float = 0.;
    let mut _1355: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1356: core::ffi::c_float = 0.;
    let mut _1357: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1358: core::ffi::c_float = 0.;
    let mut _1359: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1360: core::ffi::c_float = 0.;
    let mut _1361: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1362: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1363: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1364: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1365: core::ffi::c_float = 0.;
    let mut _1366: core::ffi::c_float = 0.;
    let mut _1367: core::ffi::c_float = 0.;
    let mut _1368: core::ffi::c_float = 0.;
    let mut _1369: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1370: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1371: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1372: core::ffi::c_float = 0.;
    let mut _1373: core::ffi::c_float = 0.;
    let mut _1374: core::ffi::c_float = 0.;
    let mut _1375: core::ffi::c_float = 0.;
    let mut _1376: core::ffi::c_float = 0.;
    let mut _1376__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _1377: core::ffi::c_float = 0.;
    let mut _1378: core::ffi::c_float = 0.;
    let mut _1379: core::ffi::c_float = 0.;
    let mut _1380: core::ffi::c_float = 0.;
    let mut _1381: core::ffi::c_float = 0.;
    let mut _1381__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _1382: core::ffi::c_float = 0.;
    let mut _1383: core::ffi::c_float = 0.;
    let mut _1384: core::ffi::c_float = 0.;
    let mut _1385: core::ffi::c_float = 0.;
    let mut _1386: core::ffi::c_float = 0.;
    let mut _1386__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _1387: core::ffi::c_float = 0.;
    let mut _1388: core::ffi::c_float = 0.;
    let mut _1389: core::ffi::c_float = 0.;
    let mut _1390: core::ffi::c_float = 0.;
    let mut _1391: core::ffi::c_float = 0.;
    let mut _1391__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _1392: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1393: core::ffi::c_float = 0.;
    let mut _1394: core::ffi::c_float = 0.;
    let mut _1395: core::ffi::c_float = 0.;
    let mut _1396: core::ffi::c_float = 0.;
    let mut _1397: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1398: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1399: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1400: core::ffi::c_float = 0.;
    let mut _1401: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1402: core::ffi::c_float = 0.;
    let mut _1403: core::ffi::c_float = 0.;
    let mut _1404: core::ffi::c_float = 0.;
    let mut _1405: core::ffi::c_float = 0.;
    let mut _1406: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1407: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1408: core::ffi::c_float = 0.;
    let mut _1409: core::ffi::c_float = 0.;
    let mut _1410: core::ffi::c_float = 0.;
    let mut _1411: core::ffi::c_float = 0.;
    let mut _1412: core::ffi::c_float = 0.;
    let mut _1412__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _1413: core::ffi::c_float = 0.;
    let mut _1414: core::ffi::c_float = 0.;
    let mut _1415: core::ffi::c_float = 0.;
    let mut _1416: core::ffi::c_float = 0.;
    let mut _1417: core::ffi::c_float = 0.;
    let mut _1417__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _1418: core::ffi::c_float = 0.;
    let mut _1419: core::ffi::c_float = 0.;
    let mut _1420: core::ffi::c_float = 0.;
    let mut _1421: core::ffi::c_float = 0.;
    let mut _1422: core::ffi::c_float = 0.;
    let mut _1422__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _1423: core::ffi::c_float = 0.;
    let mut _1424: core::ffi::c_float = 0.;
    let mut _1425: core::ffi::c_float = 0.;
    let mut _1426: core::ffi::c_float = 0.;
    let mut _1427: core::ffi::c_float = 0.;
    let mut _1427__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _1428: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1429: core::ffi::c_float = 0.;
    let mut _1430: core::ffi::c_float = 0.;
    let mut _1431: core::ffi::c_float = 0.;
    let mut _1432: core::ffi::c_float = 0.;
    let mut _1433: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1434: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1435: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1436: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1437: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1438: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1439: core::ffi::c_float = 0.;
    let mut _1440: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1441: core::ffi::c_float = 0.;
    let mut _1442: core::ffi::c_float = 0.;
    let mut _1443: core::ffi::c_float = 0.;
    let mut _1444: core::ffi::c_float = 0.;
    let mut _1445: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1446: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1447: core::ffi::c_float = 0.;
    let mut _1448: core::ffi::c_float = 0.;
    let mut _1449: core::ffi::c_float = 0.;
    let mut _1450: core::ffi::c_float = 0.;
    let mut _1451: core::ffi::c_float = 0.;
    let mut _1452: core::ffi::c_float = 0.;
    let mut _1453: core::ffi::c_float = 0.;
    let mut _1454: core::ffi::c_float = 0.;
    let mut _1455: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1456: core::ffi::c_float = 0.;
    let mut _1457: core::ffi::c_float = 0.;
    let mut _1458: core::ffi::c_float = 0.;
    let mut _1459: core::ffi::c_float = 0.;
    let mut _1460: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1461: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1462: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1463: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1464: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1465: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1466: core::ffi::c_float = 0.;
    let mut _1467: core::ffi::c_float = 0.;
    let mut _1468: core::ffi::c_float = 0.;
    let mut _1469: core::ffi::c_float = 0.;
    let mut _1470: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1471: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1472: core::ffi::c_float = 0.;
    let mut _1473: core::ffi::c_float = 0.;
    let mut _1474: core::ffi::c_float = 0.;
    let mut _1475: core::ffi::c_float = 0.;
    let mut _1476: core::ffi::c_float = 0.;
    let mut _1477: core::ffi::c_float = 0.;
    let mut _1478: core::ffi::c_float = 0.;
    let mut _1479: core::ffi::c_float = 0.;
    let mut _1480: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1481: core::ffi::c_float = 0.;
    let mut _1482: core::ffi::c_float = 0.;
    let mut _1483: core::ffi::c_float = 0.;
    let mut _1484: core::ffi::c_float = 0.;
    let mut _1485: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1486: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1487: core::ffi::c_float = 0.;
    let mut _1488: core::ffi::c_float = 0.;
    let mut _1489: core::ffi::c_float = 0.;
    let mut _1490: core::ffi::c_float = 0.;
    let mut _1491: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1492: uint32_t = 0;
    let mut _1493: uint32_t = 0;
    let mut _1494: uint32_t = 0;
    let mut _1495: uint32_t = 0;
    let mut _1496: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _1497: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1498: uint32_t = 0;
    let mut _1499: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _1500: uint32_t = 0;
    let mut _1501: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1502: uint32_t = 0;
    let mut _1503: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1504: uint32_t = 0;
    let mut _1505: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1506: uint32_t = 0;
    let mut _1507: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1508: uint32_t = 0;
    let mut _1509: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1510: core::ffi::c_float = 0.;
    let mut _1511: core::ffi::c_float = 0.;
    let mut _1512: core::ffi::c_float = 0.;
    let mut _1513: core::ffi::c_float = 0.;
    let mut _1514: uint32_t = 0;
    let mut _1515: uint32_t = 0;
    let mut _1516: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1517: core::ffi::c_float = 0.;
    let mut _1518: core::ffi::c_float = 0.;
    let mut _1519: core::ffi::c_float = 0.;
    let mut _1520: core::ffi::c_float = 0.;
    let mut _1521: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1522: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1523: core::ffi::c_float = 0.;
    let mut _1524: core::ffi::c_float = 0.;
    let mut _1525: core::ffi::c_float = 0.;
    let mut _1526: core::ffi::c_float = 0.;
    let mut _1527: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1528: uint32_t = 0;
    let mut _1529: uint32_t = 0;
    let mut _1530: uint32_t = 0;
    let mut _1531: uint32_t = 0;
    let mut _1532: uint32_t = 0;
    let mut _1533: uint32_t = 0;
    let mut _1534: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1535: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1536: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1537: core::ffi::c_float = 0.;
    let mut _1538: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1539: core::ffi::c_float = 0.;
    let mut _1540: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1541: core::ffi::c_float = 0.;
    let mut _1542: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1543: core::ffi::c_float = 0.;
    let mut _1544: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1545: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1546: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1547: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1548: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1549: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1550: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1551: core::ffi::c_float = 0.;
    let mut _1552: core::ffi::c_float = 0.;
    let mut _1553: core::ffi::c_float = 0.;
    let mut _1554: core::ffi::c_float = 0.;
    let mut _1555: core::ffi::c_float = 0.;
    let mut _1556: core::ffi::c_float = 0.;
    let mut _1557: core::ffi::c_float = 0.;
    let mut _1558: core::ffi::c_float = 0.;
    let mut _1559: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1560: core::ffi::c_float = 0.;
    let mut _1561: core::ffi::c_float = 0.;
    let mut _1562: core::ffi::c_float = 0.;
    let mut _1563: core::ffi::c_float = 0.;
    let mut _1564: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1565: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1566: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1567: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1568: uint32_t = 0;
    let mut _1569: uint32_t = 0;
    let mut _1570: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1571: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1572: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1573: core::ffi::c_float = 0.;
    let mut _1574: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1575: core::ffi::c_float = 0.;
    let mut _1576: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1577: core::ffi::c_float = 0.;
    let mut _1578: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1579: core::ffi::c_float = 0.;
    let mut _1580: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1581: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1582: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1583: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1584: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1585: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1586: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1587: core::ffi::c_float = 0.;
    let mut _1588: core::ffi::c_float = 0.;
    let mut _1589: core::ffi::c_float = 0.;
    let mut _1590: core::ffi::c_float = 0.;
    let mut _1591: core::ffi::c_float = 0.;
    let mut _1592: core::ffi::c_float = 0.;
    let mut _1593: core::ffi::c_float = 0.;
    let mut _1594: core::ffi::c_float = 0.;
    let mut _1595: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1596: core::ffi::c_float = 0.;
    let mut _1597: core::ffi::c_float = 0.;
    let mut _1598: core::ffi::c_float = 0.;
    let mut _1599: core::ffi::c_float = 0.;
    let mut _1600: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1601: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1602: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1603: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1604: uint32_t = 0;
    let mut _1605: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1606: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1607: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1608: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1609: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1610: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1611: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1612: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1613: core::ffi::c_float = 0.;
    let mut _1614: core::ffi::c_float = 0.;
    let mut _1615: core::ffi::c_float = 0.;
    let mut _1616: core::ffi::c_float = 0.;
    let mut _1617: core::ffi::c_float = 0.;
    let mut _1618: core::ffi::c_float = 0.;
    let mut _1619: core::ffi::c_float = 0.;
    let mut _1620: core::ffi::c_float = 0.;
    let mut _1621: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1622: core::ffi::c_float = 0.;
    let mut _1623: core::ffi::c_float = 0.;
    let mut _1624: core::ffi::c_float = 0.;
    let mut _1625: core::ffi::c_float = 0.;
    let mut _1626: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1627: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1628: core::ffi::c_float = 0.;
    let mut _1629: core::ffi::c_float = 0.;
    let mut _1630: core::ffi::c_float = 0.;
    let mut _1631: core::ffi::c_float = 0.;
    let mut _1632: core::ffi::c_float = 0.;
    let mut _1633: core::ffi::c_float = 0.;
    let mut _1634: core::ffi::c_float = 0.;
    let mut _1635: core::ffi::c_float = 0.;
    let mut _1636: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1637: core::ffi::c_float = 0.;
    let mut _1638: core::ffi::c_float = 0.;
    let mut _1639: core::ffi::c_float = 0.;
    let mut _1640: core::ffi::c_float = 0.;
    let mut _1641: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1642: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1643: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1644: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1645: core::ffi::c_float = 0.;
    let mut _1646: core::ffi::c_float = 0.;
    let mut _1647: core::ffi::c_float = 0.;
    let mut _1648: core::ffi::c_float = 0.;
    let mut _1649: core::ffi::c_float = 0.;
    let mut _1650: core::ffi::c_float = 0.;
    let mut _1651: core::ffi::c_float = 0.;
    let mut _1652: core::ffi::c_float = 0.;
    let mut _1653: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1654: core::ffi::c_float = 0.;
    let mut _1655: core::ffi::c_float = 0.;
    let mut _1656: core::ffi::c_float = 0.;
    let mut _1657: core::ffi::c_float = 0.;
    let mut _1658: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1659: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1660: core::ffi::c_float = 0.;
    let mut _1661: core::ffi::c_float = 0.;
    let mut _1662: core::ffi::c_float = 0.;
    let mut _1663: core::ffi::c_float = 0.;
    let mut _1664: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1665: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1666: core::ffi::c_float = 0.;
    let mut _1667: core::ffi::c_float = 0.;
    let mut _1668: core::ffi::c_float = 0.;
    let mut _1669: core::ffi::c_float = 0.;
    let mut _1670: core::ffi::c_float = 0.;
    let mut _1671: core::ffi::c_float = 0.;
    let mut _1672: core::ffi::c_float = 0.;
    let mut _1673: core::ffi::c_float = 0.;
    let mut _1674: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1675: uint8_t = 0;
    let mut _1676: uint64_t = 0;
    let mut _1677: uint8_t = 0;
    let mut _1678: uint64_t = 0;
    let mut _1679: uint8_t = 0;
    let mut _1680: uint64_t = 0;
    let mut _1681: uint8_t = 0;
    let mut _1682: uint64_t = 0;
    let mut _1683: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _1684: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1685: core::ffi::c_float = 0.;
    let mut _1686: core::ffi::c_float = 0.;
    let mut _1687: core::ffi::c_float = 0.;
    let mut _1688: core::ffi::c_float = 0.;
    let mut _1689: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1690: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1691: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1692: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1693: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1694: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1695: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _1696: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1697: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1698: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1699: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _1700: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1701: uint32_t = 0;
    let mut _1702: uint32_t = 0;
    let mut _1703: uint32_t = 0;
    let mut _1704: uint32_t = 0;
    let mut _1705: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1706: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1707: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1708: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _1709: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _1710: uint32_t = 0;
    let mut _1711: uint32_t = 0;
    let mut _1712: uint32_t = 0;
    let mut _1713: uint32_t = 0;
    let mut _1714: uint32_t = 0;
    let mut _1715: uint32_t = 0;
    let mut _1716: uint32_t = 0;
    let mut _1717: uint32_t = 0;
    let mut _1718: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1719: uint32_t = 0;
    let mut _1720: uint32_t = 0;
    let mut _1721: uint32_t = 0;
    let mut _1722: uint32_t = 0;
    let mut _1723: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _1724: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _1725: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _1726: uint32_t = 0;
    let mut _1727: uint32_t = 0;
    let mut _1728: uint32_t = 0;
    let mut _1729: uint32_t = 0;
    let mut _1730: uint32_t = 0;
    let mut _1731: uint32_t = 0;
    let mut _1732: uint32_t = 0;
    let mut _1733: uint32_t = 0;
    let mut _1734: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1735: uint32_t = 0;
    let mut _1736: uint32_t = 0;
    let mut _1737: uint32_t = 0;
    let mut _1738: uint32_t = 0;
    let mut _1739: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _1740: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1741: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _1742: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1743: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1744: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1745: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1746: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1747: core::ffi::c_float = 0.;
    let mut _1748: core::ffi::c_float = 0.;
    let mut _1749: core::ffi::c_float = 0.;
    let mut _1750: core::ffi::c_float = 0.;
    let mut _1751: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1752: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1753: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1754: core::ffi::c_float = 0.;
    let mut _1755: core::ffi::c_float = 0.;
    let mut _1756: core::ffi::c_float = 0.;
    let mut _1757: core::ffi::c_float = 0.;
    let mut _1758: core::ffi::c_float = 0.;
    let mut _1759: core::ffi::c_float = 0.;
    let mut _1760: core::ffi::c_float = 0.;
    let mut _1761: core::ffi::c_float = 0.;
    let mut _1762: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1763: core::ffi::c_float = 0.;
    let mut _1764: core::ffi::c_float = 0.;
    let mut _1765: core::ffi::c_float = 0.;
    let mut _1766: core::ffi::c_float = 0.;
    let mut _1767: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1768: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1769: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1770: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1771: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _1772: uint32_t = 0;
    let mut _1773: core::ffi::c_float = 0.;
    let mut _1774: core::ffi::c_float = 0.;
    let mut _1775: core::ffi::c_float = 0.;
    let mut _1775__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _1776: uint32_t = 0;
    let mut _1777: core::ffi::c_float = 0.;
    let mut _1778: core::ffi::c_float = 0.;
    let mut _1779: core::ffi::c_float = 0.;
    let mut _1779__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _1780: uint32_t = 0;
    let mut _1781: core::ffi::c_float = 0.;
    let mut _1782: core::ffi::c_float = 0.;
    let mut _1783: core::ffi::c_float = 0.;
    let mut _1783__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _1784: uint32_t = 0;
    let mut _1785: core::ffi::c_float = 0.;
    let mut _1786: core::ffi::c_float = 0.;
    let mut _1787: core::ffi::c_float = 0.;
    let mut _1787__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _1788: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1789: core::ffi::c_float = 0.;
    let mut _1790: core::ffi::c_float = 0.;
    let mut _1791: core::ffi::c_float = 0.;
    let mut _1792: core::ffi::c_float = 0.;
    let mut _1793: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1794: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1795: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1796: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1797: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1798: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1799: core::ffi::c_float = 0.;
    let mut _1800: core::ffi::c_float = 0.;
    let mut _1801: core::ffi::c_float = 0.;
    let mut _1802: core::ffi::c_float = 0.;
    let mut _1803: core::ffi::c_float = 0.;
    let mut _1804: core::ffi::c_float = 0.;
    let mut _1805: core::ffi::c_float = 0.;
    let mut _1806: core::ffi::c_float = 0.;
    let mut _1807: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1808: core::ffi::c_float = 0.;
    let mut _1809: core::ffi::c_float = 0.;
    let mut _1810: core::ffi::c_float = 0.;
    let mut _1811: core::ffi::c_float = 0.;
    let mut _1812: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1813: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1814: core::ffi::c_float = 0.;
    let mut _1815: core::ffi::c_float = 0.;
    let mut _1816: core::ffi::c_float = 0.;
    let mut _1817: core::ffi::c_float = 0.;
    let mut _1818: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1819: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1820: core::ffi::c_float = 0.;
    let mut _1821: core::ffi::c_float = 0.;
    let mut _1822: core::ffi::c_float = 0.;
    let mut _1823: core::ffi::c_float = 0.;
    let mut _1824: core::ffi::c_float = 0.;
    let mut _1825: core::ffi::c_float = 0.;
    let mut _1826: core::ffi::c_float = 0.;
    let mut _1827: core::ffi::c_float = 0.;
    let mut _1828: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1829: core::ffi::c_float = 0.;
    let mut _1830: core::ffi::c_float = 0.;
    let mut _1831: core::ffi::c_float = 0.;
    let mut _1832: core::ffi::c_float = 0.;
    let mut _1833: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1834: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1835: core::ffi::c_float = 0.;
    let mut _1836: core::ffi::c_float = 0.;
    let mut _1837: core::ffi::c_float = 0.;
    let mut _1838: core::ffi::c_float = 0.;
    let mut _1839: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1840: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1841: core::ffi::c_float = 0.;
    let mut _1842: core::ffi::c_float = 0.;
    let mut _1843: core::ffi::c_float = 0.;
    let mut _1844: core::ffi::c_float = 0.;
    let mut _1845: core::ffi::c_float = 0.;
    let mut _1846: core::ffi::c_float = 0.;
    let mut _1847: core::ffi::c_float = 0.;
    let mut _1848: core::ffi::c_float = 0.;
    let mut _1849: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1850: core::ffi::c_float = 0.;
    let mut _1851: core::ffi::c_float = 0.;
    let mut _1852: core::ffi::c_float = 0.;
    let mut _1853: core::ffi::c_float = 0.;
    let mut _1854: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1855: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1856: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1857: core::ffi::c_float = 0.;
    let mut _1858: core::ffi::c_float = 0.;
    let mut _1859: core::ffi::c_float = 0.;
    let mut _1860: core::ffi::c_float = 0.;
    let mut _1861: core::ffi::c_float = 0.;
    let mut _1862: core::ffi::c_float = 0.;
    let mut _1863: core::ffi::c_float = 0.;
    let mut _1864: core::ffi::c_float = 0.;
    let mut _1865: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1866: core::ffi::c_float = 0.;
    let mut _1867: core::ffi::c_float = 0.;
    let mut _1868: core::ffi::c_float = 0.;
    let mut _1869: core::ffi::c_float = 0.;
    let mut _1870: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1871: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1872: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1873: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1874: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1875: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1876: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1877: core::ffi::c_float = 0.;
    let mut _1878: core::ffi::c_float = 0.;
    let mut _1879: core::ffi::c_float = 0.;
    let mut _1880: core::ffi::c_float = 0.;
    let mut _1881: core::ffi::c_float = 0.;
    let mut _1882: core::ffi::c_float = 0.;
    let mut _1883: core::ffi::c_float = 0.;
    let mut _1884: core::ffi::c_float = 0.;
    let mut _1885: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1886: core::ffi::c_float = 0.;
    let mut _1887: core::ffi::c_float = 0.;
    let mut _1888: core::ffi::c_float = 0.;
    let mut _1889: core::ffi::c_float = 0.;
    let mut _1890: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1891: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1892: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1893: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1894: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _1895: uint32_t = 0;
    let mut _1896: core::ffi::c_float = 0.;
    let mut _1897: core::ffi::c_float = 0.;
    let mut _1898: core::ffi::c_float = 0.;
    let mut _1898__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _1899: uint32_t = 0;
    let mut _1900: core::ffi::c_float = 0.;
    let mut _1901: core::ffi::c_float = 0.;
    let mut _1902: core::ffi::c_float = 0.;
    let mut _1902__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _1903: uint32_t = 0;
    let mut _1904: core::ffi::c_float = 0.;
    let mut _1905: core::ffi::c_float = 0.;
    let mut _1906: core::ffi::c_float = 0.;
    let mut _1906__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _1907: uint32_t = 0;
    let mut _1908: core::ffi::c_float = 0.;
    let mut _1909: core::ffi::c_float = 0.;
    let mut _1910: core::ffi::c_float = 0.;
    let mut _1910__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _1911: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1912: core::ffi::c_float = 0.;
    let mut _1913: core::ffi::c_float = 0.;
    let mut _1914: core::ffi::c_float = 0.;
    let mut _1915: core::ffi::c_float = 0.;
    let mut _1916: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1917: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1918: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1919: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1920: core::ffi::c_float = 0.;
    let mut _1921: core::ffi::c_float = 0.;
    let mut _1922: core::ffi::c_float = 0.;
    let mut _1923: core::ffi::c_float = 0.;
    let mut _1924: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1925: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1926: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1927: core::ffi::c_float = 0.;
    let mut _1928: core::ffi::c_float = 0.;
    let mut _1929: core::ffi::c_float = 0.;
    let mut _1930: core::ffi::c_float = 0.;
    let mut _1931: core::ffi::c_float = 0.;
    let mut _1932: core::ffi::c_float = 0.;
    let mut _1933: core::ffi::c_float = 0.;
    let mut _1934: core::ffi::c_float = 0.;
    let mut _1935: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1936: uint8_t = 0;
    let mut _1937: uint64_t = 0;
    let mut _1938: uint8_t = 0;
    let mut _1939: uint64_t = 0;
    let mut _1940: uint8_t = 0;
    let mut _1941: uint64_t = 0;
    let mut _1942: uint8_t = 0;
    let mut _1943: uint64_t = 0;
    let mut _1944: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _1945: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1946: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1947: core::ffi::c_float = 0.;
    let mut _1948: core::ffi::c_float = 0.;
    let mut _1949: core::ffi::c_float = 0.;
    let mut _1950: core::ffi::c_float = 0.;
    let mut _1951: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1952: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1953: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1954: core::ffi::c_float = 0.;
    let mut _1955: core::ffi::c_float = 0.;
    let mut _1956: core::ffi::c_float = 0.;
    let mut _1957: core::ffi::c_float = 0.;
    let mut _1958: core::ffi::c_float = 0.;
    let mut _1959: core::ffi::c_float = 0.;
    let mut _1960: core::ffi::c_float = 0.;
    let mut _1961: core::ffi::c_float = 0.;
    let mut _1962: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1963: core::ffi::c_float = 0.;
    let mut _1964: core::ffi::c_float = 0.;
    let mut _1965: core::ffi::c_float = 0.;
    let mut _1966: core::ffi::c_float = 0.;
    let mut _1967: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1968: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1969: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1970: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1971: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _1972: uint32_t = 0;
    let mut _1973: core::ffi::c_float = 0.;
    let mut _1974: core::ffi::c_float = 0.;
    let mut _1975: core::ffi::c_float = 0.;
    let mut _1975__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _1976: uint32_t = 0;
    let mut _1977: core::ffi::c_float = 0.;
    let mut _1978: core::ffi::c_float = 0.;
    let mut _1979: core::ffi::c_float = 0.;
    let mut _1979__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _1980: uint32_t = 0;
    let mut _1981: core::ffi::c_float = 0.;
    let mut _1982: core::ffi::c_float = 0.;
    let mut _1983: core::ffi::c_float = 0.;
    let mut _1983__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _1984: uint32_t = 0;
    let mut _1985: core::ffi::c_float = 0.;
    let mut _1986: core::ffi::c_float = 0.;
    let mut _1987: core::ffi::c_float = 0.;
    let mut _1987__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _1988: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1989: core::ffi::c_float = 0.;
    let mut _1990: core::ffi::c_float = 0.;
    let mut _1991: core::ffi::c_float = 0.;
    let mut _1992: core::ffi::c_float = 0.;
    let mut _1993: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _1994: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1995: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1996: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1997: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _1998: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _1999: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2000: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _2001: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2002: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _2003: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2004: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _2005: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2006: uint32_t = 0;
    let mut _2007: uint32_t = 0;
    let mut _2008: uint32_t = 0;
    let mut _2009: uint32_t = 0;
    let mut _2010: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2011: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2012: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2013: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _2014: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _2015: uint32_t = 0;
    let mut _2016: uint32_t = 0;
    let mut _2017: uint32_t = 0;
    let mut _2018: uint32_t = 0;
    let mut _2019: uint32_t = 0;
    let mut _2020: uint32_t = 0;
    let mut _2021: uint32_t = 0;
    let mut _2022: uint32_t = 0;
    let mut _2023: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2024: uint32_t = 0;
    let mut _2025: uint32_t = 0;
    let mut _2026: uint32_t = 0;
    let mut _2027: uint32_t = 0;
    let mut _2028: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _2029: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _2030: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _2031: uint32_t = 0;
    let mut _2032: uint32_t = 0;
    let mut _2033: uint32_t = 0;
    let mut _2034: uint32_t = 0;
    let mut _2035: uint32_t = 0;
    let mut _2036: uint32_t = 0;
    let mut _2037: uint32_t = 0;
    let mut _2038: uint32_t = 0;
    let mut _2039: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2040: uint32_t = 0;
    let mut _2041: uint32_t = 0;
    let mut _2042: uint32_t = 0;
    let mut _2043: uint32_t = 0;
    let mut _2044: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _2045: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2046: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _2047: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2048: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2049: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2050: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2051: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2052: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2053: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _2054: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _2055: core::ffi::c_float = 0.;
    let mut _2056: core::ffi::c_float = 0.;
    let mut _2057: core::ffi::c_float = 0.;
    let mut _2058: core::ffi::c_float = 0.;
    let mut _2059: core::ffi::c_float = 0.;
    let mut _2060: core::ffi::c_float = 0.;
    let mut _2061: core::ffi::c_float = 0.;
    let mut _2062: core::ffi::c_float = 0.;
    let mut _2063: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2064: core::ffi::c_float = 0.;
    let mut _2065: core::ffi::c_float = 0.;
    let mut _2066: core::ffi::c_float = 0.;
    let mut _2067: core::ffi::c_float = 0.;
    let mut _2068: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2069: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2070: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2071: uint32_t = 0;
    let mut _2072: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _2073: core::ffi::c_float = 0.;
    let mut _2074: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2075: core::ffi::c_float = 0.;
    let mut _2076: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2077: core::ffi::c_float = 0.;
    let mut _2078: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2079: core::ffi::c_float = 0.;
    let mut _2080: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2081: uint32_t = 0;
    _1317 = _930;
    _1318 = _931;
    _1319 = _932;
    _1320 = _933;
    _1345 = _1317;
    if _1345 > 0 as core::ffi::c_uint {
        _1346 = _1319;
        if _1346 > 0 as core::ffi::c_uint {
            _1322 = 0;
            loop {
                _1347 = _1322;
                _1348 = _1317;
                if !(_1347 < _1348) {
                    break;
                }
                _1349 = _1318;
                _1350 = _1322;
                _1305 = &raw mut *(_1349 as *mut core::ffi::c_float)
                    .offset(_1350 as uint64_t as int64_t as isize)
                    as *mut core::ffi::c_float as *mut core::ffi::c_void;
                _1351 = _1305;
                _1194 = &mut _1304 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _1195 = _1351;
                _1352 = _1194;
                _1353 = _1195;
                _1354 = *(_1353 as *mut core::ffi::c_float);
                *(_1352 as *mut core::ffi::c_float) = _1354;
                _1355 = _1195;
                _1356 = *(&raw mut *(_1355 as *mut core::ffi::c_float)
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                *(&raw mut *((*(_1352 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1356;
                _1357 = _1195;
                _1358 = *(&raw mut *(_1357 as *mut core::ffi::c_float)
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                *(&raw mut *((*(_1352 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1358;
                _1359 = _1195;
                _1360 = *(&raw mut *(_1359 as *mut core::ffi::c_float)
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                *(&raw mut *((*(_1352 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1360;
                _1361 = _1304;
                *(&mut _1324.field0 as *mut crate::l_array_4_float) = _1361.field0;
                _1362 = *(&mut _1324.field0 as *mut crate::l_array_4_float);
                *(&mut _1311 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1362;
                _1363 = memcpy(
                    &mut _1313 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _1311 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _1285 = &mut _1287 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _1286 = 0 as core::ffi::c_int as core::ffi::c_float;
                _1364 = _1285;
                _1365 = _1286;
                *(_1364 as *mut core::ffi::c_float) = _1365;
                _1366 = _1286;
                *(&raw mut *((*(_1364 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1366;
                _1367 = _1286;
                *(&raw mut *((*(_1364 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1367;
                _1368 = _1286;
                *(&raw mut *((*(_1364 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1368;
                _1369 = _1287;
                *(&mut _1314 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1369.field0;
                _1370 =
                    *(&mut _1313 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                _1371 =
                    *(&mut _1314 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                *(&mut _1210 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1370;
                *(&mut _1211 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1371;
                _1372 = *(&mut _1210 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _1373 = *(&mut _1211 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                if llvm_fcmp_ogt(_1372 as core::ffi::c_double, _1373 as core::ffi::c_double) != 0 {
                    _1374 =
                        *(&mut _1210 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _1376__PHI_TEMPORARY = _1374;
                } else {
                    _1375 =
                        *(&mut _1211 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _1376__PHI_TEMPORARY = _1375;
                }
                _1376 = _1376__PHI_TEMPORARY;
                _1377 = *(&raw mut *((*(&mut _1210 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1378 = *(&raw mut *((*(&mut _1211 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                if llvm_fcmp_ogt(_1377 as core::ffi::c_double, _1378 as core::ffi::c_double) != 0 {
                    _1379 = *(&raw mut *((*(&mut _1210 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1381__PHI_TEMPORARY = _1379;
                } else {
                    _1380 = *(&raw mut *((*(&mut _1211 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1381__PHI_TEMPORARY = _1380;
                }
                _1381 = _1381__PHI_TEMPORARY;
                _1382 = *(&raw mut *((*(&mut _1210 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1383 = *(&raw mut *((*(&mut _1211 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                if llvm_fcmp_ogt(_1382 as core::ffi::c_double, _1383 as core::ffi::c_double) != 0 {
                    _1384 = *(&raw mut *((*(&mut _1210 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1386__PHI_TEMPORARY = _1384;
                } else {
                    _1385 = *(&raw mut *((*(&mut _1211 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1386__PHI_TEMPORARY = _1385;
                }
                _1386 = _1386__PHI_TEMPORARY;
                _1387 = *(&raw mut *((*(&mut _1210 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1388 = *(&raw mut *((*(&mut _1211 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                if llvm_fcmp_ogt(_1387 as core::ffi::c_double, _1388 as core::ffi::c_double) != 0 {
                    _1389 = *(&raw mut *((*(&mut _1210 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1391__PHI_TEMPORARY = _1389;
                } else {
                    _1390 = *(&raw mut *((*(&mut _1211 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1391__PHI_TEMPORARY = _1390;
                }
                _1391 = _1391__PHI_TEMPORARY;
                _1204 = &mut _1209 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _1205 = _1376;
                _1206 = _1381;
                _1207 = _1386;
                _1208 = _1391;
                _1392 = _1204;
                _1393 = _1205;
                *(_1392 as *mut core::ffi::c_float) = _1393;
                _1394 = _1206;
                *(&raw mut *((*(_1392 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1394;
                _1395 = _1207;
                *(&raw mut *((*(_1392 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1395;
                _1396 = _1208;
                *(&raw mut *((*(_1392 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1396;
                _1397 = _1209;
                *(&mut _1312 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1397.field0;
                _1398 =
                    *(&mut _1312 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                *(&mut _1215 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1398;
                _1216 = 1 as core::ffi::c_int as core::ffi::c_float;
                _1399 = memcpy(
                    &mut _1217 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _1215 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _1400 = _1216;
                _1212 = &mut _1218 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _1213 = _1400;
                _1401 = _1212;
                _1402 = _1213;
                *(_1401 as *mut core::ffi::c_float) = _1402;
                _1403 = _1213;
                *(&raw mut *((*(_1401 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1403;
                _1404 = _1213;
                *(&raw mut *((*(_1401 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1404;
                _1405 = _1213;
                *(&raw mut *((*(_1401 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1405;
                _1406 =
                    *(&mut _1217 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                _1407 =
                    *(&mut _1218 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                *(&mut _1202 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1406;
                *(&mut _1203 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1407;
                _1408 = *(&mut _1202 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _1409 = *(&mut _1203 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                if llvm_fcmp_olt(_1408 as core::ffi::c_double, _1409 as core::ffi::c_double) != 0 {
                    _1410 =
                        *(&mut _1202 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _1412__PHI_TEMPORARY = _1410;
                } else {
                    _1411 =
                        *(&mut _1203 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _1412__PHI_TEMPORARY = _1411;
                }
                _1412 = _1412__PHI_TEMPORARY;
                _1413 = *(&raw mut *((*(&mut _1202 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1414 = *(&raw mut *((*(&mut _1203 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                if llvm_fcmp_olt(_1413 as core::ffi::c_double, _1414 as core::ffi::c_double) != 0 {
                    _1415 = *(&raw mut *((*(&mut _1202 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1417__PHI_TEMPORARY = _1415;
                } else {
                    _1416 = *(&raw mut *((*(&mut _1203 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1417__PHI_TEMPORARY = _1416;
                }
                _1417 = _1417__PHI_TEMPORARY;
                _1418 = *(&raw mut *((*(&mut _1202 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1419 = *(&raw mut *((*(&mut _1203 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                if llvm_fcmp_olt(_1418 as core::ffi::c_double, _1419 as core::ffi::c_double) != 0 {
                    _1420 = *(&raw mut *((*(&mut _1202 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1422__PHI_TEMPORARY = _1420;
                } else {
                    _1421 = *(&raw mut *((*(&mut _1203 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1422__PHI_TEMPORARY = _1421;
                }
                _1422 = _1422__PHI_TEMPORARY;
                _1423 = *(&raw mut *((*(&mut _1202 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1424 = *(&raw mut *((*(&mut _1203 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                if llvm_fcmp_olt(_1423 as core::ffi::c_double, _1424 as core::ffi::c_double) != 0 {
                    _1425 = *(&raw mut *((*(&mut _1202 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1427__PHI_TEMPORARY = _1425;
                } else {
                    _1426 = *(&raw mut *((*(&mut _1203 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1427__PHI_TEMPORARY = _1426;
                }
                _1427 = _1427__PHI_TEMPORARY;
                _1196 = &mut _1201 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _1197 = _1412;
                _1198 = _1417;
                _1199 = _1422;
                _1200 = _1427;
                _1428 = _1196;
                _1429 = _1197;
                *(_1428 as *mut core::ffi::c_float) = _1429;
                _1430 = _1198;
                *(&raw mut *((*(_1428 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1430;
                _1431 = _1199;
                *(&raw mut *((*(_1428 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1431;
                _1432 = _1200;
                *(&raw mut *((*(_1428 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1432;
                _1433 = _1201;
                *(&mut _1214 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1433.field0;
                _1434 = _1214;
                *(&mut _1310 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1434.field0;
                _1435 = _1310;
                *(&mut _1323.field0 as *mut crate::l_array_4_float) = _1435.field0;
                _1436 = memcpy(
                    &mut _1326 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _1323 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _1437 = *(&mut _1326.field0 as *mut crate::l_array_4_float);
                *(&mut _1300 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1437;
                _1301 = 63 as core::ffi::c_int as core::ffi::c_float;
                _1438 = memcpy(
                    &mut _1302 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _1300 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _1439 = _1301;
                _1297 = &mut _1303 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _1298 = _1439;
                _1440 = _1297;
                _1441 = _1298;
                *(_1440 as *mut core::ffi::c_float) = _1441;
                _1442 = _1298;
                *(&raw mut *((*(_1440 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1442;
                _1443 = _1298;
                *(&raw mut *((*(_1440 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1443;
                _1444 = _1298;
                *(&raw mut *((*(_1440 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1444;
                _1445 =
                    *(&mut _1302 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                _1446 =
                    *(&mut _1303 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                *(&mut _1235 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1445;
                *(&mut _1236 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1446;
                _1447 = *(&mut _1235 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _1448 = *(&mut _1236 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _1449 = *(&raw mut *((*(&mut _1235 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1450 = *(&raw mut *((*(&mut _1236 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1451 = *(&raw mut *((*(&mut _1235 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1452 = *(&raw mut *((*(&mut _1236 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1453 = *(&raw mut *((*(&mut _1235 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1454 = *(&raw mut *((*(&mut _1236 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1229 = &mut _1234 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _1230 = llvm_fmul_f32(_1447, _1448);
                _1231 = llvm_fmul_f32(_1449, _1450);
                _1232 = llvm_fmul_f32(_1451, _1452);
                _1233 = llvm_fmul_f32(_1453, _1454);
                _1455 = _1229;
                _1456 = _1230;
                *(_1455 as *mut core::ffi::c_float) = _1456;
                _1457 = _1231;
                *(&raw mut *((*(_1455 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1457;
                _1458 = _1232;
                *(&raw mut *((*(_1455 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1458;
                _1459 = _1233;
                *(&raw mut *((*(_1455 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1459;
                _1460 = _1234;
                *(&mut _1299 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1460.field0;
                _1461 = _1299;
                *(&mut _1325.field0 as *mut crate::l_array_4_float) = _1461.field0;
                _1462 = memcpy(
                    &mut _1328 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _1325 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _1463 = *(&mut _1328.field0 as *mut crate::l_array_4_float);
                *(&mut _1293 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1463;
                _1464 = memcpy(
                    &mut _1295 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _1293 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _1290 = &mut _1296 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _1291 = 0.5f64 as core::ffi::c_float;
                _1465 = _1290;
                _1466 = _1291;
                *(_1465 as *mut core::ffi::c_float) = _1466;
                _1467 = _1291;
                *(&raw mut *((*(_1465 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1467;
                _1468 = _1291;
                *(&raw mut *((*(_1465 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1468;
                _1469 = _1291;
                *(&raw mut *((*(_1465 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1469;
                _1470 =
                    *(&mut _1295 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                _1471 =
                    *(&mut _1296 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                *(&mut _1172 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1470;
                *(&mut _1173 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1471;
                _1472 = *(&mut _1172 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _1473 = *(&mut _1173 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _1474 = *(&raw mut *((*(&mut _1172 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1475 = *(&raw mut *((*(&mut _1173 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1476 = *(&raw mut *((*(&mut _1172 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1477 = *(&raw mut *((*(&mut _1173 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1478 = *(&raw mut *((*(&mut _1172 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1479 = *(&raw mut *((*(&mut _1173 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1166 = &mut _1171 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _1167 = llvm_fadd_f32(_1472, _1473);
                _1168 = llvm_fadd_f32(_1474, _1475);
                _1169 = llvm_fadd_f32(_1476, _1477);
                _1170 = llvm_fadd_f32(_1478, _1479);
                _1480 = _1166;
                _1481 = _1167;
                *(_1480 as *mut core::ffi::c_float) = _1481;
                _1482 = _1168;
                *(&raw mut *((*(_1480 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1482;
                _1483 = _1169;
                *(&raw mut *((*(_1480 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1483;
                _1484 = _1170;
                *(&raw mut *((*(_1480 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1484;
                _1485 = _1171;
                *(&mut _1294 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1485.field0;
                _1486 = memcpy(
                    &mut _1293 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _1294 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _1487 = *(&mut _1293 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _1488 = *(&raw mut *((*(&mut _1293 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1489 = *(&raw mut *((*(&mut _1293 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1490 = *(&raw mut *((*(&mut _1293 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1161 = &mut _1292 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void;
                _1162 = _1487 as int32_t as uint32_t;
                _1163 = _1488 as int32_t as uint32_t;
                _1164 = _1489 as int32_t as uint32_t;
                _1165 = _1490 as int32_t as uint32_t;
                _1491 = _1161;
                _1492 = _1162;
                *(_1491 as *mut uint32_t) = _1492;
                _1493 = _1163;
                *(&raw mut *((*(_1491 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _1493;
                _1494 = _1164;
                *(&raw mut *((*(_1491 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _1494;
                _1495 = _1165;
                *(&raw mut *((*(_1491 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _1495;
                _1496 = *(&mut _1292 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_2_uint64_t);
                (*(&mut _1327.field0 as *mut crate::l_array_4_uint32_t as *mut C2RustUnnamed_28))
                    .data = _1496;
                _1497 = memcpy(
                    &mut _1329 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void,
                    &mut _1327 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void,
                    16,
                );
                _1498 = _1322;
                _1499 = (*(&mut _1329.field0 as *mut crate::l_array_4_uint32_t
                    as *mut C2RustUnnamed_27))
                    .data;
                *(&mut _1288 as *mut l_struct_struct_OC_vint4 as *mut crate::l_array_2_uint64_t) =
                    _1499;
                _1289 = &raw mut *(&raw mut *(_1321.array)
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t)
                    .offset(_1498 as uint64_t as int64_t as isize)
                    as *mut uint32_t as *mut core::ffi::c_void;
                _1500 = *(&mut _1288 as *mut l_struct_struct_OC_vint4 as *mut uint32_t);
                _1501 = _1289;
                *(_1501 as *mut uint32_t) = _1500;
                _1502 = *(&raw mut *((*(&mut _1288 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _1503 = _1289;
                *(&raw mut *(_1503 as *mut uint32_t)
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _1502;
                _1504 = *(&raw mut *((*(&mut _1288 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _1505 = _1289;
                *(&raw mut *(_1505 as *mut uint32_t)
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _1504;
                _1506 = *(&raw mut *((*(&mut _1288 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _1507 = _1289;
                *(&raw mut *(_1507 as *mut uint32_t)
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _1506;
                _1508 = _1322;
                _1322 = llvm_add_u32(_1508, 4);
            }
            _1315 = &mut _1330 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
            _1316 = 0.159154937f64 as core::ffi::c_float;
            _1509 = _1315;
            _1510 = _1316;
            *(_1509 as *mut core::ffi::c_float) = _1510;
            _1511 = _1316;
            *(&raw mut *((*(_1509 as *mut crate::l_array_4_float)).array)
                .as_mut_ptr()
                .offset(1 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float) = _1511;
            _1512 = _1316;
            *(&raw mut *((*(_1509 as *mut crate::l_array_4_float)).array)
                .as_mut_ptr()
                .offset(2 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float) = _1512;
            _1513 = _1316;
            *(&raw mut *((*(_1509 as *mut crate::l_array_4_float)).array)
                .as_mut_ptr()
                .offset(3 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float) = _1513;
            _1331 = 0;
            loop {
                _1514 = _1331;
                _1515 = _1319;
                if !(_1514 < _1515) {
                    break;
                }
                _1279 = &mut _1281 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _1280 = 0 as core::ffi::c_int as core::ffi::c_float;
                _1516 = _1279;
                _1517 = _1280;
                *(_1516 as *mut core::ffi::c_float) = _1517;
                _1518 = _1280;
                *(&raw mut *((*(_1516 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1518;
                _1519 = _1280;
                *(&raw mut *((*(_1516 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1519;
                _1520 = _1280;
                *(&raw mut *((*(_1516 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1520;
                _1521 = _1281;
                *(&mut _1332.field0 as *mut crate::l_array_4_float) = _1521.field0;
                _1282 = &mut _1284 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _1283 = 0 as core::ffi::c_int as core::ffi::c_float;
                _1522 = _1282;
                _1523 = _1283;
                *(_1522 as *mut core::ffi::c_float) = _1523;
                _1524 = _1283;
                *(&raw mut *((*(_1522 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1524;
                _1525 = _1283;
                *(&raw mut *((*(_1522 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1525;
                _1526 = _1283;
                *(&raw mut *((*(_1522 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1526;
                _1527 = _1284;
                *(&mut _1333.field0 as *mut crate::l_array_4_float) = _1527.field0;
                _1334 = 0;
                loop {
                    _1528 = _1334;
                    _1529 = _1317;
                    if !(_1528 < _1529) {
                        break;
                    }
                    _1530 = _1334;
                    _1531 = *(&raw mut *(_1321.array)
                        .as_mut_ptr()
                        .offset(_1530 as uint64_t as int64_t as isize)
                        as *mut uint32_t);
                    _1335 = _1531;
                    _1532 = _1335;
                    _1533 = _1331;
                    _1307 = &raw mut *(&raw mut *((*(&raw mut *(_ZL9cos_table.array)
                        .as_mut_ptr()
                        .offset(_1532 as int32_t as int64_t as isize)
                        as *mut crate::l_array_32_float))
                        .array)
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float)
                        .offset(_1533 as uint64_t as int64_t as isize)
                        as *mut core::ffi::c_float
                        as *mut core::ffi::c_void;
                    _1534 = _1307;
                    _1192 = &mut _1306 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                    _1193 = _1534;
                    _1535 = _1192;
                    _1536 = _1193;
                    _1537 = *(_1536 as *mut core::ffi::c_float);
                    *(_1535 as *mut core::ffi::c_float) = _1537;
                    _1538 = _1193;
                    _1539 = *(&raw mut *(_1538 as *mut core::ffi::c_float)
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    *(&raw mut *((*(_1535 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _1539;
                    _1540 = _1193;
                    _1541 = *(&raw mut *(_1540 as *mut core::ffi::c_float)
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    *(&raw mut *((*(_1535 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _1541;
                    _1542 = _1193;
                    _1543 = *(&raw mut *(_1542 as *mut core::ffi::c_float)
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    *(&raw mut *((*(_1535 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _1543;
                    _1544 = _1306;
                    *(&mut _1336.field0 as *mut crate::l_array_4_float) = _1544.field0;
                    _1269 = &mut _1332 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                    _1270 = &mut _1336 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                    _1545 = _1269;
                    _1546 = memcpy(
                        &mut _1272 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        _1545,
                        16,
                    );
                    _1547 = _1270;
                    _1548 = memcpy(
                        &mut _1273 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        _1547,
                        16,
                    );
                    _1549 = *(&mut _1272 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float);
                    _1550 = *(&mut _1273 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float);
                    *(&mut _1188 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float) = _1549;
                    *(&mut _1189 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float) = _1550;
                    _1551 =
                        *(&mut _1188 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _1552 =
                        *(&mut _1189 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _1553 = *(&raw mut *((*(&mut _1188 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1554 = *(&raw mut *((*(&mut _1189 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1555 = *(&raw mut *((*(&mut _1188 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1556 = *(&raw mut *((*(&mut _1189 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1557 = *(&raw mut *((*(&mut _1188 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1558 = *(&raw mut *((*(&mut _1189 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1182 = &mut _1187 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                    _1183 = llvm_fadd_f32(_1551, _1552);
                    _1184 = llvm_fadd_f32(_1553, _1554);
                    _1185 = llvm_fadd_f32(_1555, _1556);
                    _1186 = llvm_fadd_f32(_1557, _1558);
                    _1559 = _1182;
                    _1560 = _1183;
                    *(_1559 as *mut core::ffi::c_float) = _1560;
                    _1561 = _1184;
                    *(&raw mut *((*(_1559 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _1561;
                    _1562 = _1185;
                    *(&raw mut *((*(_1559 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _1562;
                    _1563 = _1186;
                    *(&raw mut *((*(_1559 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _1563;
                    _1564 = _1187;
                    *(&mut _1271 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float) = _1564.field0;
                    _1565 = _1269;
                    _1566 = memcpy(
                        _1565,
                        &mut _1271 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        16,
                    );
                    _1567 = _1269;
                    _1568 = _1335;
                    _1569 = _1331;
                    _1309 = &raw mut *(&raw mut *((*(&raw mut *(_ZL9sin_table.array)
                        .as_mut_ptr()
                        .offset(_1568 as int32_t as int64_t as isize)
                        as *mut crate::l_array_32_float))
                        .array)
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float)
                        .offset(_1569 as uint64_t as int64_t as isize)
                        as *mut core::ffi::c_float
                        as *mut core::ffi::c_void;
                    _1570 = _1309;
                    _1190 = &mut _1308 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                    _1191 = _1570;
                    _1571 = _1190;
                    _1572 = _1191;
                    _1573 = *(_1572 as *mut core::ffi::c_float);
                    *(_1571 as *mut core::ffi::c_float) = _1573;
                    _1574 = _1191;
                    _1575 = *(&raw mut *(_1574 as *mut core::ffi::c_float)
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    *(&raw mut *((*(_1571 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _1575;
                    _1576 = _1191;
                    _1577 = *(&raw mut *(_1576 as *mut core::ffi::c_float)
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    *(&raw mut *((*(_1571 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _1577;
                    _1578 = _1191;
                    _1579 = *(&raw mut *(_1578 as *mut core::ffi::c_float)
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    *(&raw mut *((*(_1571 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _1579;
                    _1580 = _1308;
                    *(&mut _1337.field0 as *mut crate::l_array_4_float) = _1580.field0;
                    _1274 = &mut _1333 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                    _1275 = &mut _1337 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                    _1581 = _1274;
                    _1582 = memcpy(
                        &mut _1277 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        _1581,
                        16,
                    );
                    _1583 = _1275;
                    _1584 = memcpy(
                        &mut _1278 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        _1583,
                        16,
                    );
                    _1585 = *(&mut _1277 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float);
                    _1586 = *(&mut _1278 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float);
                    *(&mut _1180 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float) = _1585;
                    *(&mut _1181 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float) = _1586;
                    _1587 =
                        *(&mut _1180 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _1588 =
                        *(&mut _1181 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _1589 = *(&raw mut *((*(&mut _1180 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1590 = *(&raw mut *((*(&mut _1181 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1591 = *(&raw mut *((*(&mut _1180 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1592 = *(&raw mut *((*(&mut _1181 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1593 = *(&raw mut *((*(&mut _1180 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1594 = *(&raw mut *((*(&mut _1181 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1174 = &mut _1179 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                    _1175 = llvm_fadd_f32(_1587, _1588);
                    _1176 = llvm_fadd_f32(_1589, _1590);
                    _1177 = llvm_fadd_f32(_1591, _1592);
                    _1178 = llvm_fadd_f32(_1593, _1594);
                    _1595 = _1174;
                    _1596 = _1175;
                    *(_1595 as *mut core::ffi::c_float) = _1596;
                    _1597 = _1176;
                    *(&raw mut *((*(_1595 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _1597;
                    _1598 = _1177;
                    *(&raw mut *((*(_1595 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _1598;
                    _1599 = _1178;
                    *(&raw mut *((*(_1595 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _1599;
                    _1600 = _1179;
                    *(&mut _1276 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float) = _1600.field0;
                    _1601 = _1274;
                    _1602 = memcpy(
                        _1601,
                        &mut _1276 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        16,
                    );
                    _1603 = _1274;
                    _1604 = _1334;
                    _1334 = llvm_add_u32(_1604, 1);
                }
                _1605 = memcpy(
                    &mut _1339 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _1333 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _1606 = memcpy(
                    &mut _1340 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _1332 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _1607 = *(&mut _1339.field0 as *mut crate::l_array_4_float);
                _1608 = *(&mut _1340.field0 as *mut crate::l_array_4_float);
                *(&mut _1252 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1607;
                *(&mut _1253 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1608;
                _1609 = memcpy(
                    &mut _1257 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _1252 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _1610 = memcpy(
                    &mut _1258 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _1253 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _1611 =
                    *(&mut _1257 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                _1612 =
                    *(&mut _1258 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                *(&mut _1045 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1611;
                *(&mut _1046 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1612;
                _1613 = *(&mut _1045 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _1614 = *(&mut _1046 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _1615 = *(&raw mut *((*(&mut _1045 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1616 = *(&raw mut *((*(&mut _1046 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1617 = *(&raw mut *((*(&mut _1045 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1618 = *(&raw mut *((*(&mut _1046 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1619 = *(&raw mut *((*(&mut _1045 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1620 = *(&raw mut *((*(&mut _1046 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1039 = &mut _1044 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _1040 = llvm_fdiv_f32(_1613, _1614);
                _1041 = llvm_fdiv_f32(_1615, _1616);
                _1042 = llvm_fdiv_f32(_1617, _1618);
                _1043 = llvm_fdiv_f32(_1619, _1620);
                _1621 = _1039;
                _1622 = _1040;
                *(_1621 as *mut core::ffi::c_float) = _1622;
                _1623 = _1041;
                *(&raw mut *((*(_1621 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1623;
                _1624 = _1042;
                *(&raw mut *((*(_1621 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1624;
                _1625 = _1043;
                *(&raw mut *((*(_1621 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1625;
                _1626 = _1044;
                *(&mut _1256 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1626.field0;
                _1627 =
                    *(&mut _1256 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                *(&mut _1069 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1627;
                _1628 = *(&mut _1069 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _1629 = _ZSt3absf(_1628);
                _1630 = *(&raw mut *((*(&mut _1069 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1631 = _ZSt3absf(_1630);
                _1632 = *(&raw mut *((*(&mut _1069 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1633 = _ZSt3absf(_1632);
                _1634 = *(&raw mut *((*(&mut _1069 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1635 = _ZSt3absf(_1634);
                _1063 = &mut _1068 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _1064 = _1629;
                _1065 = _1631;
                _1066 = _1633;
                _1067 = _1635;
                _1636 = _1063;
                _1637 = _1064;
                *(_1636 as *mut core::ffi::c_float) = _1637;
                _1638 = _1065;
                *(&raw mut *((*(_1636 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1638;
                _1639 = _1066;
                *(&raw mut *((*(_1636 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1639;
                _1640 = _1067;
                *(&raw mut *((*(_1636 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1640;
                _1641 = _1068;
                *(&mut _1255 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1641.field0;
                _1642 =
                    *(&mut _1255 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                *(&mut _1133 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1642;
                _1643 = memcpy(
                    &mut _1136 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _1133 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _1644 =
                    *(&mut _1136 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                *(&mut _1076 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1644;
                _1645 = *(&mut _1076 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _1646 = _ZSt3absf(_1645);
                _1647 = *(&raw mut *((*(&mut _1076 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1648 = _ZSt3absf(_1647);
                _1649 = *(&raw mut *((*(&mut _1076 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1650 = _ZSt3absf(_1649);
                _1651 = *(&raw mut *((*(&mut _1076 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1652 = _ZSt3absf(_1651);
                _1070 = &mut _1075 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _1071 = _1646;
                _1072 = _1648;
                _1073 = _1650;
                _1074 = _1652;
                _1653 = _1070;
                _1654 = _1071;
                *(_1653 as *mut core::ffi::c_float) = _1654;
                _1655 = _1072;
                *(&raw mut *((*(_1653 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1655;
                _1656 = _1073;
                *(&raw mut *((*(_1653 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1656;
                _1657 = _1074;
                *(&raw mut *((*(_1653 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1657;
                _1658 = _1075;
                *(&mut _1135 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1658.field0;
                _1127 = &mut _1137 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _1128 = 1 as core::ffi::c_int as core::ffi::c_float;
                _1659 = _1127;
                _1660 = _1128;
                *(_1659 as *mut core::ffi::c_float) = _1660;
                _1661 = _1128;
                *(&raw mut *((*(_1659 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1661;
                _1662 = _1128;
                *(&raw mut *((*(_1659 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1662;
                _1663 = _1128;
                *(&raw mut *((*(_1659 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1663;
                _1664 =
                    *(&mut _1135 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                _1665 =
                    *(&mut _1137 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                *(&mut _1130 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1664;
                *(&mut _1131 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1665;
                _1666 = *(&mut _1130 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _1667 = *(&mut _1131 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _1668 = *(&raw mut *((*(&mut _1130 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1669 = *(&raw mut *((*(&mut _1131 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1670 = *(&raw mut *((*(&mut _1130 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1671 = *(&raw mut *((*(&mut _1131 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1672 = *(&raw mut *((*(&mut _1130 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1673 = *(&raw mut *((*(&mut _1131 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _982 = &mut _1129 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void;
                _983 = llvm_fcmp_ogt(_1666 as core::ffi::c_double, _1667 as core::ffi::c_double)
                    as bool_0;
                _984 = llvm_fcmp_ogt(_1668 as core::ffi::c_double, _1669 as core::ffi::c_double)
                    as bool_0;
                _985 = llvm_fcmp_ogt(_1670 as core::ffi::c_double, _1671 as core::ffi::c_double)
                    as bool_0;
                _986 = llvm_fcmp_ogt(_1672 as core::ffi::c_double, _1673 as core::ffi::c_double)
                    as bool_0;
                _1674 = _982;
                _1675 = _983;
                _1676 = ((_1675 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0
                    as uint32_t
                    == 0 as core::ffi::c_uint) as core::ffi::c_int as bool_0
                    as uint64_t;
                *(_1674 as *mut uint32_t) = llvm_select_u32(
                    ((_1675 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0 as uint32_t
                        == 0 as core::ffi::c_uint) as core::ffi::c_int
                        as bool_0,
                    0,
                    -(1 as core::ffi::c_int) as uint32_t,
                );
                _1677 = _984;
                _1678 = ((_1677 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0
                    as uint32_t
                    == 0 as core::ffi::c_uint) as core::ffi::c_int as bool_0
                    as uint64_t;
                *(&raw mut *((*(_1674 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = llvm_select_u32(
                    ((_1677 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0 as uint32_t
                        == 0 as core::ffi::c_uint) as core::ffi::c_int
                        as bool_0,
                    0,
                    -(1 as core::ffi::c_int) as uint32_t,
                );
                _1679 = _985;
                _1680 = ((_1679 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0
                    as uint32_t
                    == 0 as core::ffi::c_uint) as core::ffi::c_int as bool_0
                    as uint64_t;
                *(&raw mut *((*(_1674 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = llvm_select_u32(
                    ((_1679 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0 as uint32_t
                        == 0 as core::ffi::c_uint) as core::ffi::c_int
                        as bool_0,
                    0,
                    -(1 as core::ffi::c_int) as uint32_t,
                );
                _1681 = _986;
                _1682 = ((_1681 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0
                    as uint32_t
                    == 0 as core::ffi::c_uint) as core::ffi::c_int as bool_0
                    as uint64_t;
                *(&raw mut *((*(_1674 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = llvm_select_u32(
                    ((_1681 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0 as uint32_t
                        == 0 as core::ffi::c_uint) as core::ffi::c_int
                        as bool_0,
                    0,
                    -(1 as core::ffi::c_int) as uint32_t,
                );
                _1683 = *(&mut _1129 as *mut l_struct_struct_OC_vmask4
                    as *mut crate::l_array_2_uint64_t);
                *(&mut _1134 as *mut l_struct_struct_OC_vmask4 as *mut crate::l_array_2_uint64_t) =
                    _1683;
                _1125 = &mut _1139 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _1126 = 1.57079637f64 as core::ffi::c_float;
                _1684 = _1125;
                _1685 = _1126;
                *(_1684 as *mut core::ffi::c_float) = _1685;
                _1686 = _1126;
                *(&raw mut *((*(_1684 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1686;
                _1687 = _1126;
                *(&raw mut *((*(_1684 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1687;
                _1688 = _1126;
                *(&raw mut *((*(_1684 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1688;
                _1689 = memcpy(
                    &mut _1140 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _1133 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _1690 =
                    *(&mut _1139 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                _1691 =
                    *(&mut _1140 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                *(&mut _1023 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1690;
                *(&mut _1024 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1691;
                _1692 = memcpy(
                    &mut _1026 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _1023 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _1693 =
                    *(&mut _1026 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                *(&mut _975 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1693;
                _1694 = memcpy(
                    &mut _974 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void,
                    &mut _975 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _1695 =
                    *(&mut _974 as *mut l_struct_struct_OC_vint4 as *mut crate::l_array_2_uint64_t);
                *(&mut _1025 as *mut l_struct_struct_OC_vint4 as *mut crate::l_array_2_uint64_t) =
                    _1695;
                _1696 = memcpy(
                    &mut _1028 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _1024 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _1697 =
                    *(&mut _1028 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                *(&mut _977 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1697;
                _1698 = memcpy(
                    &mut _976 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void,
                    &mut _977 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _1699 =
                    *(&mut _976 as *mut l_struct_struct_OC_vint4 as *mut crate::l_array_2_uint64_t);
                *(&mut _1027 as *mut l_struct_struct_OC_vint4 as *mut crate::l_array_2_uint64_t) =
                    _1699;
                _970 = &mut _1029 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void;
                _971 = 2147483648 as core::ffi::c_uint;
                _1700 = _970;
                _1701 = _971;
                *(_1700 as *mut uint32_t) = _1701;
                _1702 = _971;
                *(&raw mut *((*(_1700 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _1702;
                _1703 = _971;
                *(&raw mut *((*(_1700 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _1703;
                _1704 = _971;
                *(&raw mut *((*(_1700 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _1704;
                _1705 = memcpy(
                    &mut _1031 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void,
                    &mut _1025 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void,
                    16,
                );
                _1706 = memcpy(
                    &mut _1033 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void,
                    &mut _1027 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void,
                    16,
                );
                _1707 = memcpy(
                    &mut _1034 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void,
                    &mut _1029 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void,
                    16,
                );
                _1708 = *(&mut _1033 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_2_uint64_t);
                _1709 = *(&mut _1034 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_2_uint64_t);
                *(&mut _944 as *mut l_struct_struct_OC_vint4 as *mut crate::l_array_2_uint64_t) =
                    _1708;
                *(&mut _945 as *mut l_struct_struct_OC_vint4 as *mut crate::l_array_2_uint64_t) =
                    _1709;
                _1710 = *(&mut _944 as *mut l_struct_struct_OC_vint4 as *mut uint32_t);
                _1711 = *(&mut _945 as *mut l_struct_struct_OC_vint4 as *mut uint32_t);
                _1712 = *(&raw mut *((*(&mut _944 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _1713 = *(&raw mut *((*(&mut _945 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _1714 = *(&raw mut *((*(&mut _944 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _1715 = *(&raw mut *((*(&mut _945 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _1716 = *(&raw mut *((*(&mut _944 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _1717 = *(&raw mut *((*(&mut _945 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _938 = &mut _943 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void;
                _939 = _1710 & _1711;
                _940 = _1712 & _1713;
                _941 = _1714 & _1715;
                _942 = _1716 & _1717;
                _1718 = _938;
                _1719 = _939;
                *(_1718 as *mut uint32_t) = _1719;
                _1720 = _940;
                *(&raw mut *((*(_1718 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _1720;
                _1721 = _941;
                *(&raw mut *((*(_1718 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _1721;
                _1722 = _942;
                *(&raw mut *((*(_1718 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _1722;
                _1723 =
                    *(&mut _943 as *mut l_struct_struct_OC_vint4 as *mut crate::l_array_2_uint64_t);
                *(&mut _1032 as *mut l_struct_struct_OC_vint4 as *mut crate::l_array_2_uint64_t) =
                    _1723;
                _1724 = *(&mut _1031 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_2_uint64_t);
                _1725 = *(&mut _1032 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_2_uint64_t);
                *(&mut _960 as *mut l_struct_struct_OC_vint4 as *mut crate::l_array_2_uint64_t) =
                    _1724;
                *(&mut _961 as *mut l_struct_struct_OC_vint4 as *mut crate::l_array_2_uint64_t) =
                    _1725;
                _1726 = *(&mut _960 as *mut l_struct_struct_OC_vint4 as *mut uint32_t);
                _1727 = *(&mut _961 as *mut l_struct_struct_OC_vint4 as *mut uint32_t);
                _1728 = *(&raw mut *((*(&mut _960 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _1729 = *(&raw mut *((*(&mut _961 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _1730 = *(&raw mut *((*(&mut _960 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _1731 = *(&raw mut *((*(&mut _961 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _1732 = *(&raw mut *((*(&mut _960 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _1733 = *(&raw mut *((*(&mut _961 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _954 = &mut _959 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void;
                _955 = _1726 ^ _1727;
                _956 = _1728 ^ _1729;
                _957 = _1730 ^ _1731;
                _958 = _1732 ^ _1733;
                _1734 = _954;
                _1735 = _955;
                *(_1734 as *mut uint32_t) = _1735;
                _1736 = _956;
                *(&raw mut *((*(_1734 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _1736;
                _1737 = _957;
                *(&raw mut *((*(_1734 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _1737;
                _1738 = _958;
                *(&raw mut *((*(_1734 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _1738;
                _1739 =
                    *(&mut _959 as *mut l_struct_struct_OC_vint4 as *mut crate::l_array_2_uint64_t);
                *(&mut _1030 as *mut l_struct_struct_OC_vint4 as *mut crate::l_array_2_uint64_t) =
                    _1739;
                _1740 = memcpy(
                    &mut _1035 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void,
                    &mut _1030 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void,
                    16,
                );
                _1741 = *(&mut _1035 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_2_uint64_t);
                *(&mut _935 as *mut l_struct_struct_OC_vint4 as *mut crate::l_array_2_uint64_t) =
                    _1741;
                _1742 = memcpy(
                    &mut _934 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _935 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void,
                    16,
                );
                _1743 = _934;
                *(&mut _1022 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1743.field0;
                _1744 = _1022;
                *(&mut _1138 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1744.field0;
                _1745 = memcpy(
                    &mut _1142 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _1133 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _1123 = &mut _1144 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _1124 = 1 as core::ffi::c_int as core::ffi::c_float;
                _1746 = _1123;
                _1747 = _1124;
                *(_1746 as *mut core::ffi::c_float) = _1747;
                _1748 = _1124;
                *(&raw mut *((*(_1746 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1748;
                _1749 = _1124;
                *(&raw mut *((*(_1746 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1749;
                _1750 = _1124;
                *(&raw mut *((*(_1746 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1750;
                _1751 = memcpy(
                    &mut _1145 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _1133 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _1752 =
                    *(&mut _1144 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                _1753 =
                    *(&mut _1145 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                *(&mut _1053 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1752;
                *(&mut _1054 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1753;
                _1754 = *(&mut _1053 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _1755 = *(&mut _1054 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _1756 = *(&raw mut *((*(&mut _1053 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1757 = *(&raw mut *((*(&mut _1054 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1758 = *(&raw mut *((*(&mut _1053 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1759 = *(&raw mut *((*(&mut _1054 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1760 = *(&raw mut *((*(&mut _1053 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1761 = *(&raw mut *((*(&mut _1054 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1047 = &mut _1052 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _1048 = llvm_fdiv_f32(_1754, _1755);
                _1049 = llvm_fdiv_f32(_1756, _1757);
                _1050 = llvm_fdiv_f32(_1758, _1759);
                _1051 = llvm_fdiv_f32(_1760, _1761);
                _1762 = _1047;
                _1763 = _1048;
                *(_1762 as *mut core::ffi::c_float) = _1763;
                _1764 = _1049;
                *(&raw mut *((*(_1762 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1764;
                _1765 = _1050;
                *(&raw mut *((*(_1762 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1765;
                _1766 = _1051;
                *(&raw mut *((*(_1762 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1766;
                _1767 = _1052;
                *(&mut _1143 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1767.field0;
                _1768 = memcpy(
                    &mut _1146 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void,
                    &mut _1134 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void,
                    16,
                );
                _1769 =
                    *(&mut _1142 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                _1770 =
                    *(&mut _1143 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                _1771 = *(&mut _1146 as *mut l_struct_struct_OC_vmask4
                    as *mut crate::l_array_2_uint64_t);
                *(&mut _1107 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1769;
                *(&mut _1108 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1770;
                *(&mut _1109 as *mut l_struct_struct_OC_vmask4 as *mut crate::l_array_2_uint64_t) =
                    _1771;
                _1772 = *(&mut _1109 as *mut l_struct_struct_OC_vmask4 as *mut uint32_t);
                if _1772 & 2147483648 as core::ffi::c_uint != 0 as core::ffi::c_uint {
                    _1773 =
                        *(&mut _1108 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _1775__PHI_TEMPORARY = _1773;
                } else {
                    _1774 =
                        *(&mut _1107 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _1775__PHI_TEMPORARY = _1774;
                }
                _1775 = _1775__PHI_TEMPORARY;
                _1776 = *(&raw mut *((*(&mut _1109 as *mut l_struct_struct_OC_vmask4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                if _1776 & 2147483648 as core::ffi::c_uint != 0 as core::ffi::c_uint {
                    _1777 = *(&raw mut *((*(&mut _1108 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1779__PHI_TEMPORARY = _1777;
                } else {
                    _1778 = *(&raw mut *((*(&mut _1107 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1779__PHI_TEMPORARY = _1778;
                }
                _1779 = _1779__PHI_TEMPORARY;
                _1780 = *(&raw mut *((*(&mut _1109 as *mut l_struct_struct_OC_vmask4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                if _1780 & 2147483648 as core::ffi::c_uint != 0 as core::ffi::c_uint {
                    _1781 = *(&raw mut *((*(&mut _1108 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1783__PHI_TEMPORARY = _1781;
                } else {
                    _1782 = *(&raw mut *((*(&mut _1107 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1783__PHI_TEMPORARY = _1782;
                }
                _1783 = _1783__PHI_TEMPORARY;
                _1784 = *(&raw mut *((*(&mut _1109 as *mut l_struct_struct_OC_vmask4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                if _1784 & 2147483648 as core::ffi::c_uint != 0 as core::ffi::c_uint {
                    _1785 = *(&raw mut *((*(&mut _1108 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1787__PHI_TEMPORARY = _1785;
                } else {
                    _1786 = *(&raw mut *((*(&mut _1107 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1787__PHI_TEMPORARY = _1786;
                }
                _1787 = _1787__PHI_TEMPORARY;
                _1101 = &mut _1106 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _1102 = _1775;
                _1103 = _1779;
                _1104 = _1783;
                _1105 = _1787;
                _1788 = _1101;
                _1789 = _1102;
                *(_1788 as *mut core::ffi::c_float) = _1789;
                _1790 = _1103;
                *(&raw mut *((*(_1788 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1790;
                _1791 = _1104;
                *(&raw mut *((*(_1788 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1791;
                _1792 = _1105;
                *(&raw mut *((*(_1788 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1792;
                _1793 = _1106;
                *(&mut _1141 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1793.field0;
                _1794 = memcpy(
                    &mut _1148 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _1141 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _1795 = memcpy(
                    &mut _1152 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _1141 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _1796 = memcpy(
                    &mut _1153 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _1141 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _1797 =
                    *(&mut _1152 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                _1798 =
                    *(&mut _1153 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                *(&mut _1091 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1797;
                *(&mut _1092 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1798;
                _1799 = *(&mut _1091 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _1800 = *(&mut _1092 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _1801 = *(&raw mut *((*(&mut _1091 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1802 = *(&raw mut *((*(&mut _1092 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1803 = *(&raw mut *((*(&mut _1091 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1804 = *(&raw mut *((*(&mut _1092 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1805 = *(&raw mut *((*(&mut _1091 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1806 = *(&raw mut *((*(&mut _1092 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1085 = &mut _1090 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _1086 = llvm_fmul_f32(_1799, _1800);
                _1087 = llvm_fmul_f32(_1801, _1802);
                _1088 = llvm_fmul_f32(_1803, _1804);
                _1089 = llvm_fmul_f32(_1805, _1806);
                _1807 = _1085;
                _1808 = _1086;
                *(_1807 as *mut core::ffi::c_float) = _1808;
                _1809 = _1087;
                *(&raw mut *((*(_1807 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1809;
                _1810 = _1088;
                *(&raw mut *((*(_1807 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1810;
                _1811 = _1089;
                *(&raw mut *((*(_1807 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1811;
                _1812 = _1090;
                *(&mut _1151 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1812.field0;
                _1121 = &mut _1154 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _1122 = 0.280000001f64 as core::ffi::c_float;
                _1813 = _1121;
                _1814 = _1122;
                *(_1813 as *mut core::ffi::c_float) = _1814;
                _1815 = _1122;
                *(&raw mut *((*(_1813 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1815;
                _1816 = _1122;
                *(&raw mut *((*(_1813 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1816;
                _1817 = _1122;
                *(&raw mut *((*(_1813 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1817;
                _1818 =
                    *(&mut _1151 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                _1819 =
                    *(&mut _1154 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                *(&mut _1099 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1818;
                *(&mut _1100 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1819;
                _1820 = *(&mut _1099 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _1821 = *(&mut _1100 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _1822 = *(&raw mut *((*(&mut _1099 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1823 = *(&raw mut *((*(&mut _1100 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1824 = *(&raw mut *((*(&mut _1099 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1825 = *(&raw mut *((*(&mut _1100 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1826 = *(&raw mut *((*(&mut _1099 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1827 = *(&raw mut *((*(&mut _1100 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1093 = &mut _1098 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _1094 = llvm_fmul_f32(_1820, _1821);
                _1095 = llvm_fmul_f32(_1822, _1823);
                _1096 = llvm_fmul_f32(_1824, _1825);
                _1097 = llvm_fmul_f32(_1826, _1827);
                _1828 = _1093;
                _1829 = _1094;
                *(_1828 as *mut core::ffi::c_float) = _1829;
                _1830 = _1095;
                *(&raw mut *((*(_1828 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1830;
                _1831 = _1096;
                *(&raw mut *((*(_1828 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1831;
                _1832 = _1097;
                *(&raw mut *((*(_1828 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1832;
                _1833 = _1098;
                *(&mut _1150 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1833.field0;
                _1119 = &mut _1155 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _1120 = 1 as core::ffi::c_int as core::ffi::c_float;
                _1834 = _1119;
                _1835 = _1120;
                *(_1834 as *mut core::ffi::c_float) = _1835;
                _1836 = _1120;
                *(&raw mut *((*(_1834 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1836;
                _1837 = _1120;
                *(&raw mut *((*(_1834 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1837;
                _1838 = _1120;
                *(&raw mut *((*(_1834 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1838;
                _1839 =
                    *(&mut _1150 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                _1840 =
                    *(&mut _1155 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                *(&mut _1083 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1839;
                *(&mut _1084 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1840;
                _1841 = *(&mut _1083 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _1842 = *(&mut _1084 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _1843 = *(&raw mut *((*(&mut _1083 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1844 = *(&raw mut *((*(&mut _1084 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1845 = *(&raw mut *((*(&mut _1083 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1846 = *(&raw mut *((*(&mut _1084 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1847 = *(&raw mut *((*(&mut _1083 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1848 = *(&raw mut *((*(&mut _1084 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1077 = &mut _1082 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _1078 = llvm_fadd_f32(_1841, _1842);
                _1079 = llvm_fadd_f32(_1843, _1844);
                _1080 = llvm_fadd_f32(_1845, _1846);
                _1081 = llvm_fadd_f32(_1847, _1848);
                _1849 = _1077;
                _1850 = _1078;
                *(_1849 as *mut core::ffi::c_float) = _1850;
                _1851 = _1079;
                *(&raw mut *((*(_1849 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1851;
                _1852 = _1080;
                *(&raw mut *((*(_1849 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1852;
                _1853 = _1081;
                *(&raw mut *((*(_1849 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1853;
                _1854 = _1082;
                *(&mut _1149 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1854.field0;
                _1855 =
                    *(&mut _1148 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                _1856 =
                    *(&mut _1149 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                *(&mut _1061 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1855;
                *(&mut _1062 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1856;
                _1857 = *(&mut _1061 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _1858 = *(&mut _1062 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _1859 = *(&raw mut *((*(&mut _1061 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1860 = *(&raw mut *((*(&mut _1062 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1861 = *(&raw mut *((*(&mut _1061 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1862 = *(&raw mut *((*(&mut _1062 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1863 = *(&raw mut *((*(&mut _1061 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1864 = *(&raw mut *((*(&mut _1062 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1055 = &mut _1060 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _1056 = llvm_fdiv_f32(_1857, _1858);
                _1057 = llvm_fdiv_f32(_1859, _1860);
                _1058 = llvm_fdiv_f32(_1861, _1862);
                _1059 = llvm_fdiv_f32(_1863, _1864);
                _1865 = _1055;
                _1866 = _1056;
                *(_1865 as *mut core::ffi::c_float) = _1866;
                _1867 = _1057;
                *(&raw mut *((*(_1865 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1867;
                _1868 = _1058;
                *(&raw mut *((*(_1865 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1868;
                _1869 = _1059;
                *(&raw mut *((*(_1865 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1869;
                _1870 = _1060;
                *(&mut _1147 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1870.field0;
                _1871 = memcpy(
                    &mut _1141 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _1147 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _1872 = memcpy(
                    &mut _1156 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _1141 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _1873 = memcpy(
                    &mut _1158 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _1138 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _1874 = memcpy(
                    &mut _1159 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _1141 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _1875 =
                    *(&mut _1158 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                _1876 =
                    *(&mut _1159 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                *(&mut _1006 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1875;
                *(&mut _1007 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1876;
                _1877 = *(&mut _1006 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _1878 = *(&mut _1007 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _1879 = *(&raw mut *((*(&mut _1006 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1880 = *(&raw mut *((*(&mut _1007 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1881 = *(&raw mut *((*(&mut _1006 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1882 = *(&raw mut *((*(&mut _1007 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1883 = *(&raw mut *((*(&mut _1006 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1884 = *(&raw mut *((*(&mut _1007 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1000 = &mut _1005 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _1001 = llvm_fsub_f32(_1877, _1878);
                _1002 = llvm_fsub_f32(_1879, _1880);
                _1003 = llvm_fsub_f32(_1881, _1882);
                _1004 = llvm_fsub_f32(_1883, _1884);
                _1885 = _1000;
                _1886 = _1001;
                *(_1885 as *mut core::ffi::c_float) = _1886;
                _1887 = _1002;
                *(&raw mut *((*(_1885 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1887;
                _1888 = _1003;
                *(&raw mut *((*(_1885 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1888;
                _1889 = _1004;
                *(&raw mut *((*(_1885 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1889;
                _1890 = _1005;
                *(&mut _1157 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1890.field0;
                _1891 = memcpy(
                    &mut _1160 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void,
                    &mut _1134 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void,
                    16,
                );
                _1892 =
                    *(&mut _1156 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                _1893 =
                    *(&mut _1157 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                _1894 = *(&mut _1160 as *mut l_struct_struct_OC_vmask4
                    as *mut crate::l_array_2_uint64_t);
                *(&mut _1116 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1892;
                *(&mut _1117 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1893;
                *(&mut _1118 as *mut l_struct_struct_OC_vmask4 as *mut crate::l_array_2_uint64_t) =
                    _1894;
                _1895 = *(&mut _1118 as *mut l_struct_struct_OC_vmask4 as *mut uint32_t);
                if _1895 & 2147483648 as core::ffi::c_uint != 0 as core::ffi::c_uint {
                    _1896 =
                        *(&mut _1117 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _1898__PHI_TEMPORARY = _1896;
                } else {
                    _1897 =
                        *(&mut _1116 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _1898__PHI_TEMPORARY = _1897;
                }
                _1898 = _1898__PHI_TEMPORARY;
                _1899 = *(&raw mut *((*(&mut _1118 as *mut l_struct_struct_OC_vmask4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                if _1899 & 2147483648 as core::ffi::c_uint != 0 as core::ffi::c_uint {
                    _1900 = *(&raw mut *((*(&mut _1117 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1902__PHI_TEMPORARY = _1900;
                } else {
                    _1901 = *(&raw mut *((*(&mut _1116 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1902__PHI_TEMPORARY = _1901;
                }
                _1902 = _1902__PHI_TEMPORARY;
                _1903 = *(&raw mut *((*(&mut _1118 as *mut l_struct_struct_OC_vmask4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                if _1903 & 2147483648 as core::ffi::c_uint != 0 as core::ffi::c_uint {
                    _1904 = *(&raw mut *((*(&mut _1117 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1906__PHI_TEMPORARY = _1904;
                } else {
                    _1905 = *(&raw mut *((*(&mut _1116 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1906__PHI_TEMPORARY = _1905;
                }
                _1906 = _1906__PHI_TEMPORARY;
                _1907 = *(&raw mut *((*(&mut _1118 as *mut l_struct_struct_OC_vmask4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                if _1907 & 2147483648 as core::ffi::c_uint != 0 as core::ffi::c_uint {
                    _1908 = *(&raw mut *((*(&mut _1117 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1910__PHI_TEMPORARY = _1908;
                } else {
                    _1909 = *(&raw mut *((*(&mut _1116 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1910__PHI_TEMPORARY = _1909;
                }
                _1910 = _1910__PHI_TEMPORARY;
                _1110 = &mut _1115 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _1111 = _1898;
                _1112 = _1902;
                _1113 = _1906;
                _1114 = _1910;
                _1911 = _1110;
                _1912 = _1111;
                *(_1911 as *mut core::ffi::c_float) = _1912;
                _1913 = _1112;
                *(&raw mut *((*(_1911 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1913;
                _1914 = _1113;
                *(&raw mut *((*(_1911 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1914;
                _1915 = _1114;
                *(&raw mut *((*(_1911 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1915;
                _1916 = _1115;
                *(&mut _1132 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1916.field0;
                _1917 = _1132;
                *(&mut _1254 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1917.field0;
                _1918 = memcpy(
                    &mut _1260 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _1253 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _1237 = &mut _1239 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _1238 = 0 as core::ffi::c_int as core::ffi::c_float;
                _1919 = _1237;
                _1920 = _1238;
                *(_1919 as *mut core::ffi::c_float) = _1920;
                _1921 = _1238;
                *(&raw mut *((*(_1919 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1921;
                _1922 = _1238;
                *(&raw mut *((*(_1919 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1922;
                _1923 = _1238;
                *(&raw mut *((*(_1919 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1923;
                _1924 = _1239;
                *(&mut _1261 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1924.field0;
                _1925 =
                    *(&mut _1260 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                _1926 =
                    *(&mut _1261 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                *(&mut _1037 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1925;
                *(&mut _1038 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1926;
                _1927 = *(&mut _1037 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _1928 = *(&mut _1038 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _1929 = *(&raw mut *((*(&mut _1037 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1930 = *(&raw mut *((*(&mut _1038 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1931 = *(&raw mut *((*(&mut _1037 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1932 = *(&raw mut *((*(&mut _1038 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1933 = *(&raw mut *((*(&mut _1037 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1934 = *(&raw mut *((*(&mut _1038 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _987 = &mut _1036 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void;
                _988 = llvm_fcmp_olt(_1927 as core::ffi::c_double, _1928 as core::ffi::c_double)
                    as bool_0;
                _989 = llvm_fcmp_olt(_1929 as core::ffi::c_double, _1930 as core::ffi::c_double)
                    as bool_0;
                _990 = llvm_fcmp_olt(_1931 as core::ffi::c_double, _1932 as core::ffi::c_double)
                    as bool_0;
                _991 = llvm_fcmp_olt(_1933 as core::ffi::c_double, _1934 as core::ffi::c_double)
                    as bool_0;
                _1935 = _987;
                _1936 = _988;
                _1937 = ((_1936 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0
                    as uint32_t
                    == 0 as core::ffi::c_uint) as core::ffi::c_int as bool_0
                    as uint64_t;
                *(_1935 as *mut uint32_t) = llvm_select_u32(
                    ((_1936 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0 as uint32_t
                        == 0 as core::ffi::c_uint) as core::ffi::c_int
                        as bool_0,
                    0,
                    -(1 as core::ffi::c_int) as uint32_t,
                );
                _1938 = _989;
                _1939 = ((_1938 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0
                    as uint32_t
                    == 0 as core::ffi::c_uint) as core::ffi::c_int as bool_0
                    as uint64_t;
                *(&raw mut *((*(_1935 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = llvm_select_u32(
                    ((_1938 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0 as uint32_t
                        == 0 as core::ffi::c_uint) as core::ffi::c_int
                        as bool_0,
                    0,
                    -(1 as core::ffi::c_int) as uint32_t,
                );
                _1940 = _990;
                _1941 = ((_1940 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0
                    as uint32_t
                    == 0 as core::ffi::c_uint) as core::ffi::c_int as bool_0
                    as uint64_t;
                *(&raw mut *((*(_1935 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = llvm_select_u32(
                    ((_1940 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0 as uint32_t
                        == 0 as core::ffi::c_uint) as core::ffi::c_int
                        as bool_0,
                    0,
                    -(1 as core::ffi::c_int) as uint32_t,
                );
                _1942 = _991;
                _1943 = ((_1942 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0
                    as uint32_t
                    == 0 as core::ffi::c_uint) as core::ffi::c_int as bool_0
                    as uint64_t;
                *(&raw mut *((*(_1935 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = llvm_select_u32(
                    ((_1942 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0 as uint32_t
                        == 0 as core::ffi::c_uint) as core::ffi::c_int
                        as bool_0,
                    0,
                    -(1 as core::ffi::c_int) as uint32_t,
                );
                _1944 = *(&mut _1036 as *mut l_struct_struct_OC_vmask4
                    as *mut crate::l_array_2_uint64_t);
                *(&mut _1259 as *mut l_struct_struct_OC_vmask4 as *mut crate::l_array_2_uint64_t) =
                    _1944;
                _1945 = memcpy(
                    &mut _1263 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _1254 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _1249 = &mut _1265 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _1250 = 3.14159274f64 as core::ffi::c_float;
                _1946 = _1249;
                _1947 = _1250;
                *(_1946 as *mut core::ffi::c_float) = _1947;
                _1948 = _1250;
                *(&raw mut *((*(_1946 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1948;
                _1949 = _1250;
                *(&raw mut *((*(_1946 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1949;
                _1950 = _1250;
                *(&raw mut *((*(_1946 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1950;
                _1951 = memcpy(
                    &mut _1266 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _1254 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _1952 =
                    *(&mut _1265 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                _1953 =
                    *(&mut _1266 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                *(&mut _998 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1952;
                *(&mut _999 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1953;
                _1954 = *(&mut _998 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _1955 = *(&mut _999 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _1956 = *(&raw mut *((*(&mut _998 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1957 = *(&raw mut *((*(&mut _999 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1958 = *(&raw mut *((*(&mut _998 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1959 = *(&raw mut *((*(&mut _999 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1960 = *(&raw mut *((*(&mut _998 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1961 = *(&raw mut *((*(&mut _999 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _992 = &mut _997 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _993 = llvm_fsub_f32(_1954, _1955);
                _994 = llvm_fsub_f32(_1956, _1957);
                _995 = llvm_fsub_f32(_1958, _1959);
                _996 = llvm_fsub_f32(_1960, _1961);
                _1962 = _992;
                _1963 = _993;
                *(_1962 as *mut core::ffi::c_float) = _1963;
                _1964 = _994;
                *(&raw mut *((*(_1962 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1964;
                _1965 = _995;
                *(&raw mut *((*(_1962 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1965;
                _1966 = _996;
                *(&raw mut *((*(_1962 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1966;
                _1967 = _997;
                *(&mut _1264 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1967.field0;
                _1968 = memcpy(
                    &mut _1267 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void,
                    &mut _1259 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void,
                    16,
                );
                _1969 =
                    *(&mut _1263 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                _1970 =
                    *(&mut _1264 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                _1971 = *(&mut _1267 as *mut l_struct_struct_OC_vmask4
                    as *mut crate::l_array_2_uint64_t);
                *(&mut _1246 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1969;
                *(&mut _1247 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1970;
                *(&mut _1248 as *mut l_struct_struct_OC_vmask4 as *mut crate::l_array_2_uint64_t) =
                    _1971;
                _1972 = *(&mut _1248 as *mut l_struct_struct_OC_vmask4 as *mut uint32_t);
                if _1972 & 2147483648 as core::ffi::c_uint != 0 as core::ffi::c_uint {
                    _1973 =
                        *(&mut _1247 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _1975__PHI_TEMPORARY = _1973;
                } else {
                    _1974 =
                        *(&mut _1246 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _1975__PHI_TEMPORARY = _1974;
                }
                _1975 = _1975__PHI_TEMPORARY;
                _1976 = *(&raw mut *((*(&mut _1248 as *mut l_struct_struct_OC_vmask4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                if _1976 & 2147483648 as core::ffi::c_uint != 0 as core::ffi::c_uint {
                    _1977 = *(&raw mut *((*(&mut _1247 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1979__PHI_TEMPORARY = _1977;
                } else {
                    _1978 = *(&raw mut *((*(&mut _1246 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1979__PHI_TEMPORARY = _1978;
                }
                _1979 = _1979__PHI_TEMPORARY;
                _1980 = *(&raw mut *((*(&mut _1248 as *mut l_struct_struct_OC_vmask4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                if _1980 & 2147483648 as core::ffi::c_uint != 0 as core::ffi::c_uint {
                    _1981 = *(&raw mut *((*(&mut _1247 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1983__PHI_TEMPORARY = _1981;
                } else {
                    _1982 = *(&raw mut *((*(&mut _1246 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1983__PHI_TEMPORARY = _1982;
                }
                _1983 = _1983__PHI_TEMPORARY;
                _1984 = *(&raw mut *((*(&mut _1248 as *mut l_struct_struct_OC_vmask4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                if _1984 & 2147483648 as core::ffi::c_uint != 0 as core::ffi::c_uint {
                    _1985 = *(&raw mut *((*(&mut _1247 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1987__PHI_TEMPORARY = _1985;
                } else {
                    _1986 = *(&raw mut *((*(&mut _1246 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1987__PHI_TEMPORARY = _1986;
                }
                _1987 = _1987__PHI_TEMPORARY;
                _1240 = &mut _1245 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _1241 = _1975;
                _1242 = _1979;
                _1243 = _1983;
                _1244 = _1987;
                _1988 = _1240;
                _1989 = _1241;
                *(_1988 as *mut core::ffi::c_float) = _1989;
                _1990 = _1242;
                *(&raw mut *((*(_1988 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1990;
                _1991 = _1243;
                *(&raw mut *((*(_1988 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1991;
                _1992 = _1244;
                *(&raw mut *((*(_1988 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _1992;
                _1993 = _1245;
                *(&mut _1262 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1993.field0;
                _1994 = memcpy(
                    &mut _1268 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _1252 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _1995 =
                    *(&mut _1262 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                _1996 =
                    *(&mut _1268 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                *(&mut _1009 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1995;
                *(&mut _1010 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1996;
                _1997 = memcpy(
                    &mut _1012 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _1009 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _1998 =
                    *(&mut _1012 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                *(&mut _979 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _1998;
                _1999 = memcpy(
                    &mut _978 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void,
                    &mut _979 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _2000 =
                    *(&mut _978 as *mut l_struct_struct_OC_vint4 as *mut crate::l_array_2_uint64_t);
                *(&mut _1011 as *mut l_struct_struct_OC_vint4 as *mut crate::l_array_2_uint64_t) =
                    _2000;
                _2001 = memcpy(
                    &mut _1014 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _1010 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _2002 =
                    *(&mut _1014 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                *(&mut _981 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _2002;
                _2003 = memcpy(
                    &mut _980 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void,
                    &mut _981 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _2004 =
                    *(&mut _980 as *mut l_struct_struct_OC_vint4 as *mut crate::l_array_2_uint64_t);
                *(&mut _1013 as *mut l_struct_struct_OC_vint4 as *mut crate::l_array_2_uint64_t) =
                    _2004;
                _972 = &mut _1015 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void;
                _973 = 2147483648 as core::ffi::c_uint;
                _2005 = _972;
                _2006 = _973;
                *(_2005 as *mut uint32_t) = _2006;
                _2007 = _973;
                *(&raw mut *((*(_2005 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _2007;
                _2008 = _973;
                *(&raw mut *((*(_2005 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _2008;
                _2009 = _973;
                *(&raw mut *((*(_2005 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _2009;
                _2010 = memcpy(
                    &mut _1017 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void,
                    &mut _1011 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void,
                    16,
                );
                _2011 = memcpy(
                    &mut _1019 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void,
                    &mut _1013 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void,
                    16,
                );
                _2012 = memcpy(
                    &mut _1020 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void,
                    &mut _1015 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void,
                    16,
                );
                _2013 = *(&mut _1019 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_2_uint64_t);
                _2014 = *(&mut _1020 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_2_uint64_t);
                *(&mut _952 as *mut l_struct_struct_OC_vint4 as *mut crate::l_array_2_uint64_t) =
                    _2013;
                *(&mut _953 as *mut l_struct_struct_OC_vint4 as *mut crate::l_array_2_uint64_t) =
                    _2014;
                _2015 = *(&mut _952 as *mut l_struct_struct_OC_vint4 as *mut uint32_t);
                _2016 = *(&mut _953 as *mut l_struct_struct_OC_vint4 as *mut uint32_t);
                _2017 = *(&raw mut *((*(&mut _952 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _2018 = *(&raw mut *((*(&mut _953 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _2019 = *(&raw mut *((*(&mut _952 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _2020 = *(&raw mut *((*(&mut _953 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _2021 = *(&raw mut *((*(&mut _952 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _2022 = *(&raw mut *((*(&mut _953 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _946 = &mut _951 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void;
                _947 = _2015 & _2016;
                _948 = _2017 & _2018;
                _949 = _2019 & _2020;
                _950 = _2021 & _2022;
                _2023 = _946;
                _2024 = _947;
                *(_2023 as *mut uint32_t) = _2024;
                _2025 = _948;
                *(&raw mut *((*(_2023 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _2025;
                _2026 = _949;
                *(&raw mut *((*(_2023 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _2026;
                _2027 = _950;
                *(&raw mut *((*(_2023 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _2027;
                _2028 =
                    *(&mut _951 as *mut l_struct_struct_OC_vint4 as *mut crate::l_array_2_uint64_t);
                *(&mut _1018 as *mut l_struct_struct_OC_vint4 as *mut crate::l_array_2_uint64_t) =
                    _2028;
                _2029 = *(&mut _1017 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_2_uint64_t);
                _2030 = *(&mut _1018 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_2_uint64_t);
                *(&mut _968 as *mut l_struct_struct_OC_vint4 as *mut crate::l_array_2_uint64_t) =
                    _2029;
                *(&mut _969 as *mut l_struct_struct_OC_vint4 as *mut crate::l_array_2_uint64_t) =
                    _2030;
                _2031 = *(&mut _968 as *mut l_struct_struct_OC_vint4 as *mut uint32_t);
                _2032 = *(&mut _969 as *mut l_struct_struct_OC_vint4 as *mut uint32_t);
                _2033 = *(&raw mut *((*(&mut _968 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _2034 = *(&raw mut *((*(&mut _969 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _2035 = *(&raw mut *((*(&mut _968 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _2036 = *(&raw mut *((*(&mut _969 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _2037 = *(&raw mut *((*(&mut _968 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _2038 = *(&raw mut *((*(&mut _969 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _962 = &mut _967 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void;
                _963 = _2031 ^ _2032;
                _964 = _2033 ^ _2034;
                _965 = _2035 ^ _2036;
                _966 = _2037 ^ _2038;
                _2039 = _962;
                _2040 = _963;
                *(_2039 as *mut uint32_t) = _2040;
                _2041 = _964;
                *(&raw mut *((*(_2039 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _2041;
                _2042 = _965;
                *(&raw mut *((*(_2039 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _2042;
                _2043 = _966;
                *(&raw mut *((*(_2039 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _2043;
                _2044 =
                    *(&mut _967 as *mut l_struct_struct_OC_vint4 as *mut crate::l_array_2_uint64_t);
                *(&mut _1016 as *mut l_struct_struct_OC_vint4 as *mut crate::l_array_2_uint64_t) =
                    _2044;
                _2045 = memcpy(
                    &mut _1021 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void,
                    &mut _1016 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void,
                    16,
                );
                _2046 = *(&mut _1021 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_2_uint64_t);
                *(&mut _937 as *mut l_struct_struct_OC_vint4 as *mut crate::l_array_2_uint64_t) =
                    _2046;
                _2047 = memcpy(
                    &mut _936 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _937 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void,
                    16,
                );
                _2048 = _936;
                *(&mut _1008 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _2048.field0;
                _2049 = _1008;
                *(&mut _1251 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _2049.field0;
                _2050 = _1251;
                *(&mut _1338.field0 as *mut crate::l_array_4_float) = _2050.field0;
                _2051 = memcpy(
                    &mut _1342 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _1338 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _2052 = memcpy(
                    &mut _1343 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _1330 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _2053 = *(&mut _1342.field0 as *mut crate::l_array_4_float);
                _2054 = *(&mut _1343.field0 as *mut crate::l_array_4_float);
                *(&mut _1227 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _2053;
                *(&mut _1228 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _2054;
                _2055 = *(&mut _1227 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _2056 = *(&mut _1228 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _2057 = *(&raw mut *((*(&mut _1227 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _2058 = *(&raw mut *((*(&mut _1228 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _2059 = *(&raw mut *((*(&mut _1227 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _2060 = *(&raw mut *((*(&mut _1228 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _2061 = *(&raw mut *((*(&mut _1227 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _2062 = *(&raw mut *((*(&mut _1228 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1221 = &mut _1226 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _1222 = llvm_fmul_f32(_2055, _2056);
                _1223 = llvm_fmul_f32(_2057, _2058);
                _1224 = llvm_fmul_f32(_2059, _2060);
                _1225 = llvm_fmul_f32(_2061, _2062);
                _2063 = _1221;
                _2064 = _1222;
                *(_2063 as *mut core::ffi::c_float) = _2064;
                _2065 = _1223;
                *(&raw mut *((*(_2063 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _2065;
                _2066 = _1224;
                *(&raw mut *((*(_2063 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _2066;
                _2067 = _1225;
                *(&raw mut *((*(_2063 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _2067;
                _2068 = _1226;
                *(&mut _1341.field0 as *mut crate::l_array_4_float) = _2068.field0;
                _2069 = memcpy(
                    &mut _1344 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _1341 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _2070 = _1320;
                _2071 = _1331;
                _2072 = *(&mut _1344.field0 as *mut crate::l_array_4_float);
                *(&mut _1219 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _2072;
                _1220 = &raw mut *(_2070 as *mut core::ffi::c_float)
                    .offset(_2071 as uint64_t as int64_t as isize)
                    as *mut core::ffi::c_float as *mut core::ffi::c_void;
                _2073 = *(&mut _1219 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _2074 = _1220;
                *(_2074 as *mut core::ffi::c_float) = _2073;
                _2075 = *(&raw mut *((*(&mut _1219 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _2076 = _1220;
                *(&raw mut *(_2076 as *mut core::ffi::c_float)
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _2075;
                _2077 = *(&raw mut *((*(&mut _1219 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _2078 = _1220;
                *(&raw mut *(_2078 as *mut core::ffi::c_float)
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _2077;
                _2079 = *(&raw mut *((*(&mut _1219 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _2080 = _1220;
                *(&raw mut *(_2080 as *mut core::ffi::c_float)
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _2079;
                _2081 = _1331;
                _1331 = llvm_add_u32(_2081, 4);
            }
            return;
        } else {
            __assert_fail(
                &_OC_str_OC_6 as *const crate::l_array_22_uint8_t as *mut core::ffi::c_void,
                &_OC_str_OC_1 as *const crate::l_array_51_uint8_t as *mut core::ffi::c_void,
                101,
                &__PRETTY_FUNCTION___OC__ZL23compute_angular_offsetsjPKfjPf
                    as *const crate::l_array_81_uint8_t as *mut core::ffi::c_void,
            );
        }
    } else {
        __assert_fail(
            &_OC_str_OC_7 as *const crate::l_array_17_uint8_t as *mut core::ffi::c_void,
            &_OC_str_OC_1 as *const crate::l_array_51_uint8_t as *mut core::ffi::c_void,
            100,
            &__PRETTY_FUNCTION___OC__ZL23compute_angular_offsetsjPKfjPf
                as *const crate::l_array_81_uint8_t as *mut core::ffi::c_void,
        );
    };
}
#[inline(never)]
unsafe extern "C" fn _ZL33compute_lowest_and_highest_weightjPKfjjS0_PfPiS1_S1_S1_(
    mut _2160: uint32_t,
    mut _2161: *mut core::ffi::c_void,
    mut _2162: uint32_t,
    mut _2163: uint32_t,
    mut _2164: *mut core::ffi::c_void,
    mut _2165: *mut core::ffi::c_void,
    mut _2166: *mut core::ffi::c_void,
    mut _2167: *mut core::ffi::c_void,
    mut _2168: *mut core::ffi::c_void,
    mut _2169: *mut core::ffi::c_void,
) {
    let mut current_block: u64;
    let mut _2170: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2171: uint32_t = 0;
    let mut _2172: uint32_t = 0;
    let mut _2173: uint32_t = 0;
    let mut _2174: uint32_t = 0;
    let mut _2175: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2176: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2177: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2178: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2179: core::ffi::c_float = 0.;
    let mut _2180: core::ffi::c_float = 0.;
    let mut _2181: core::ffi::c_float = 0.;
    let mut _2182: core::ffi::c_float = 0.;
    let mut _2183: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2184: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2185: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2186: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2187: core::ffi::c_float = 0.;
    let mut _2188: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2189: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2190: core::ffi::c_float = 0.;
    let mut _2191: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2192: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2193: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2194: uint32_t = 0;
    let mut _2195: uint32_t = 0;
    let mut _2196: uint32_t = 0;
    let mut _2197: uint32_t = 0;
    let mut _2198: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2199: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2200: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2201: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2202: uint32_t = 0;
    let mut _2203: uint32_t = 0;
    let mut _2204: uint32_t = 0;
    let mut _2205: uint32_t = 0;
    let mut _2206: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2207: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2208: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2209: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2210: uint32_t = 0;
    let mut _2211: uint32_t = 0;
    let mut _2212: uint32_t = 0;
    let mut _2213: uint32_t = 0;
    let mut _2214: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2215: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2216: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2217: uint8_t = 0;
    let mut _2218: uint8_t = 0;
    let mut _2219: uint8_t = 0;
    let mut _2220: uint8_t = 0;
    let mut _2221: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2222: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2223: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2224: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2225: uint8_t = 0;
    let mut _2226: uint8_t = 0;
    let mut _2227: uint8_t = 0;
    let mut _2228: uint8_t = 0;
    let mut _2229: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2230: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2231: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2232: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2233: core::ffi::c_float = 0.;
    let mut _2234: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2235: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2236: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2237: core::ffi::c_float = 0.;
    let mut _2238: core::ffi::c_float = 0.;
    let mut _2239: core::ffi::c_float = 0.;
    let mut _2240: core::ffi::c_float = 0.;
    let mut _2241: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2242: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2243: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2244: core::ffi::c_float = 0.;
    let mut _2245: core::ffi::c_float = 0.;
    let mut _2246: core::ffi::c_float = 0.;
    let mut _2247: core::ffi::c_float = 0.;
    let mut _2248: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2249: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2250: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2251: core::ffi::c_float = 0.;
    let mut _2252: core::ffi::c_float = 0.;
    let mut _2253: core::ffi::c_float = 0.;
    let mut _2254: core::ffi::c_float = 0.;
    let mut _2255: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2256: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2257: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2258: core::ffi::c_float = 0.;
    let mut _2259: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2260: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2261: core::ffi::c_float = 0.;
    let mut _2262: core::ffi::c_float = 0.;
    let mut _2263: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2264: core::ffi::c_float = 0.;
    let mut _2265: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2266: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2267: core::ffi::c_float = 0.;
    let mut _2268: core::ffi::c_float = 0.;
    let mut _2269: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2270: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2271: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2272: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2273: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2274: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2275: uint8_t = 0;
    let mut _2276: uint8_t = 0;
    let mut _2277: uint8_t = 0;
    let mut _2278: uint8_t = 0;
    let mut _2279: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2280: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2281: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2282: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2283: uint32_t = 0;
    let mut _2284: uint32_t = 0;
    let mut _2285: uint32_t = 0;
    let mut _2286: uint32_t = 0;
    let mut _2287: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2288: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2289: uint32_t = 0;
    let mut _2290: uint32_t = 0;
    let mut _2291: uint32_t = 0;
    let mut _2292: uint32_t = 0;
    let mut _2293: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2294: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2295: core::ffi::c_float = 0.;
    let mut _2296: core::ffi::c_float = 0.;
    let mut _2297: core::ffi::c_float = 0.;
    let mut _2298: core::ffi::c_float = 0.;
    let mut _2299: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2300: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2301: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2302: uint32_t = 0;
    let mut _2303: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2304: uint32_t = 0;
    let mut _2305: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2306: uint32_t = 0;
    let mut _2307: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2308: uint32_t = 0;
    let mut _2309: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2310: core::ffi::c_float = 0.;
    let mut _2311: core::ffi::c_float = 0.;
    let mut _2312: core::ffi::c_float = 0.;
    let mut _2313: core::ffi::c_float = 0.;
    let mut _2314: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2315: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2316: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2317: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2318: core::ffi::c_float = 0.;
    let mut _2319: core::ffi::c_float = 0.;
    let mut _2320: core::ffi::c_float = 0.;
    let mut _2321: core::ffi::c_float = 0.;
    let mut _2322: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2323: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2324: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2325: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2326: core::ffi::c_float = 0.;
    let mut _2327: core::ffi::c_float = 0.;
    let mut _2328: core::ffi::c_float = 0.;
    let mut _2329: core::ffi::c_float = 0.;
    let mut _2330: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2331: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2332: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2333: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2334: core::ffi::c_float = 0.;
    let mut _2335: core::ffi::c_float = 0.;
    let mut _2336: core::ffi::c_float = 0.;
    let mut _2337: core::ffi::c_float = 0.;
    let mut _2338: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2339: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2340: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2341: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2342: core::ffi::c_float = 0.;
    let mut _2343: core::ffi::c_float = 0.;
    let mut _2344: core::ffi::c_float = 0.;
    let mut _2345: core::ffi::c_float = 0.;
    let mut _2346: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2347: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2348: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2349: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2350: core::ffi::c_float = 0.;
    let mut _2351: core::ffi::c_float = 0.;
    let mut _2352: core::ffi::c_float = 0.;
    let mut _2353: core::ffi::c_float = 0.;
    let mut _2354: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2355: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2356: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2357: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2358: core::ffi::c_float = 0.;
    let mut _2359: core::ffi::c_float = 0.;
    let mut _2360: core::ffi::c_float = 0.;
    let mut _2361: core::ffi::c_float = 0.;
    let mut _2362: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2363: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2364: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2365: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2366: core::ffi::c_float = 0.;
    let mut _2367: core::ffi::c_float = 0.;
    let mut _2368: core::ffi::c_float = 0.;
    let mut _2369: core::ffi::c_float = 0.;
    let mut _2370: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2371: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2372: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2373: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2374: core::ffi::c_float = 0.;
    let mut _2375: core::ffi::c_float = 0.;
    let mut _2376: core::ffi::c_float = 0.;
    let mut _2377: core::ffi::c_float = 0.;
    let mut _2378: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2379: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2380: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2381: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2382: core::ffi::c_float = 0.;
    let mut _2383: core::ffi::c_float = 0.;
    let mut _2384: core::ffi::c_float = 0.;
    let mut _2385: core::ffi::c_float = 0.;
    let mut _2386: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2387: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2388: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2389: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2390: core::ffi::c_float = 0.;
    let mut _2391: core::ffi::c_float = 0.;
    let mut _2392: core::ffi::c_float = 0.;
    let mut _2393: core::ffi::c_float = 0.;
    let mut _2394: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2395: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2396: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2397: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2398: core::ffi::c_float = 0.;
    let mut _2399: core::ffi::c_float = 0.;
    let mut _2400: core::ffi::c_float = 0.;
    let mut _2401: core::ffi::c_float = 0.;
    let mut _2402: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2403: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2404: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2405: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2406: core::ffi::c_float = 0.;
    let mut _2407: core::ffi::c_float = 0.;
    let mut _2408: core::ffi::c_float = 0.;
    let mut _2409: core::ffi::c_float = 0.;
    let mut _2410: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2411: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2412: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2413: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2414: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2415: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2416: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2417: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2418: core::ffi::c_float = 0.;
    let mut _2419: core::ffi::c_float = 0.;
    let mut _2420: core::ffi::c_float = 0.;
    let mut _2421: core::ffi::c_float = 0.;
    let mut _2422: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2423: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2424: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2425: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2426: core::ffi::c_float = 0.;
    let mut _2427: core::ffi::c_float = 0.;
    let mut _2428: core::ffi::c_float = 0.;
    let mut _2429: core::ffi::c_float = 0.;
    let mut _2430: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2431: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2432: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2433: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2434: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2435: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2436: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2437: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2438: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2439: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2440: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2441: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2442: core::ffi::c_float = 0.;
    let mut _2443: core::ffi::c_float = 0.;
    let mut _2444: core::ffi::c_float = 0.;
    let mut _2445: core::ffi::c_float = 0.;
    let mut _2446: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2447: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2448: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2449: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2450: core::ffi::c_float = 0.;
    let mut _2451: core::ffi::c_float = 0.;
    let mut _2452: core::ffi::c_float = 0.;
    let mut _2453: core::ffi::c_float = 0.;
    let mut _2454: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2455: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2456: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2457: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2458: core::ffi::c_float = 0.;
    let mut _2459: core::ffi::c_float = 0.;
    let mut _2460: core::ffi::c_float = 0.;
    let mut _2461: core::ffi::c_float = 0.;
    let mut _2462: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2463: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2464: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2465: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2466: core::ffi::c_float = 0.;
    let mut _2467: core::ffi::c_float = 0.;
    let mut _2468: core::ffi::c_float = 0.;
    let mut _2469: core::ffi::c_float = 0.;
    let mut _2470: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2471: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2472: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2473: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2474: core::ffi::c_float = 0.;
    let mut _2475: core::ffi::c_float = 0.;
    let mut _2476: core::ffi::c_float = 0.;
    let mut _2477: core::ffi::c_float = 0.;
    let mut _2478: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2479: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2480: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2481: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2482: core::ffi::c_float = 0.;
    let mut _2483: core::ffi::c_float = 0.;
    let mut _2484: core::ffi::c_float = 0.;
    let mut _2485: core::ffi::c_float = 0.;
    let mut _2486: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2487: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2488: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2489: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2490: core::ffi::c_float = 0.;
    let mut _2491: core::ffi::c_float = 0.;
    let mut _2492: core::ffi::c_float = 0.;
    let mut _2493: core::ffi::c_float = 0.;
    let mut _2494: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2495: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2496: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2497: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2498: core::ffi::c_float = 0.;
    let mut _2499: core::ffi::c_float = 0.;
    let mut _2500: core::ffi::c_float = 0.;
    let mut _2501: core::ffi::c_float = 0.;
    let mut _2502: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2503: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2504: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2505: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2506: core::ffi::c_float = 0.;
    let mut _2507: core::ffi::c_float = 0.;
    let mut _2508: core::ffi::c_float = 0.;
    let mut _2509: core::ffi::c_float = 0.;
    let mut _2510: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2511: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2512: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2513: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2514: core::ffi::c_float = 0.;
    let mut _2515: core::ffi::c_float = 0.;
    let mut _2516: core::ffi::c_float = 0.;
    let mut _2517: core::ffi::c_float = 0.;
    let mut _2518: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2519: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2520: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2521: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2522: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2523: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2524: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2525: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2526: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2527: core::ffi::c_float = 0.;
    let mut _2528: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2529: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2530: core::ffi::c_float = 0.;
    let mut _2531: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2532: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2533: core::ffi::c_float = 0.;
    let mut _2534: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2535: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2536: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2537: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2538: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2539: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2540: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2541: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2542: core::ffi::c_float = 0.;
    let mut _2543: core::ffi::c_float = 0.;
    let mut _2544: core::ffi::c_float = 0.;
    let mut _2545: core::ffi::c_float = 0.;
    let mut _2546: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2547: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2548: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2549: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2550: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2551: core::ffi::c_float = 0.;
    let mut _2552: core::ffi::c_float = 0.;
    let mut _2553: core::ffi::c_float = 0.;
    let mut _2554: core::ffi::c_float = 0.;
    let mut _2555: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2556: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2557: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2558: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2559: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2560: core::ffi::c_float = 0.;
    let mut _2561: core::ffi::c_float = 0.;
    let mut _2562: core::ffi::c_float = 0.;
    let mut _2563: core::ffi::c_float = 0.;
    let mut _2564: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2565: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2566: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2567: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2568: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2569: core::ffi::c_float = 0.;
    let mut _2570: core::ffi::c_float = 0.;
    let mut _2571: core::ffi::c_float = 0.;
    let mut _2572: core::ffi::c_float = 0.;
    let mut _2573: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2574: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2575: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2576: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2577: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2578: core::ffi::c_float = 0.;
    let mut _2579: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2580: core::ffi::c_float = 0.;
    let mut _2581: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2582: core::ffi::c_float = 0.;
    let mut _2583: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2584: core::ffi::c_float = 0.;
    let mut _2585: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2586: core::ffi::c_float = 0.;
    let mut _2587: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2588: core::ffi::c_float = 0.;
    let mut _2589: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2590: core::ffi::c_float = 0.;
    let mut _2591: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2592: core::ffi::c_float = 0.;
    let mut _2593: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2594: core::ffi::c_float = 0.;
    let mut _2595: uint32_t = 0;
    let mut _2596: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2597: uint32_t = 0;
    let mut _2598: uint32_t = 0;
    let mut _2599: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2600: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2601: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2602: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2603: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2604: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2605: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2606: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2607: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2608: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2609: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2610: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2611: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2612: uint32_t = 0;
    let mut _2613: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2614: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2615: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2616: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2617: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2618: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2619: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2620: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2621: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2622: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2623: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2624: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2625: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2626: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2627: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2628: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2629: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2630: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2631: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2632: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2633: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2634: uint32_t = 0;
    let mut _2635: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2636: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2637: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2638: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2639: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2640: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2641: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2642: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2643: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2644: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2645: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2646: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2647: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2648: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2649: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2650: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2651: uint32_t = 0;
    let mut _2652: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2653: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2654: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2655: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2656: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2657: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2658: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2659: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2660: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2661: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2662: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2663: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2664: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2665: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2666: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2667: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2668: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2669: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2670: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2671: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2672: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2673: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2674: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2675: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2676: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2677: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2678: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2679: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2680: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2681: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2682: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2683: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2684: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2685: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2686: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2687: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2688: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2689: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2690: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2691: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2692: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2693: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2694: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2695: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2696: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2697: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2698: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2699: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2700: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2701: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2702: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2703: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2704: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2705: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2706: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: crate::l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _2707: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2708: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2709: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2710: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2711: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2712: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2713: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2714: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2715: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2716: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2717: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2718: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2719: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2720: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2721: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2722: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2723: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2724: uint32_t = 0;
    let mut _2725: uint32_t = 0;
    let mut _2726: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2727: uint32_t = 0;
    let mut _2728: uint32_t = 0;
    let mut _2729: uint32_t = 0;
    let mut _2730: uint32_t = 0;
    let mut _2731: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _2732: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _2733: uint32_t = 0;
    let mut _2734: uint32_t = 0;
    let mut _2735: uint32_t = 0;
    let mut _2736: uint32_t = 0;
    let mut _2737: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2738: core::ffi::c_float = 0.;
    let mut _2739: core::ffi::c_float = 0.;
    let mut _2740: core::ffi::c_float = 0.;
    let mut _2741: core::ffi::c_float = 0.;
    let mut _2742: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2743: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2744: core::ffi::c_float = 0.;
    let mut _2745: core::ffi::c_float = 0.;
    let mut _2746: core::ffi::c_float = 0.;
    let mut _2747: core::ffi::c_float = 0.;
    let mut _2748: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _2749: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _2750: core::ffi::c_float = 0.;
    let mut _2751: core::ffi::c_float = 0.;
    let mut _2752: core::ffi::c_float = 0.;
    let mut _2753: core::ffi::c_float = 0.;
    let mut _2754: core::ffi::c_float = 0.;
    let mut _2755: core::ffi::c_float = 0.;
    let mut _2756: core::ffi::c_float = 0.;
    let mut _2757: core::ffi::c_float = 0.;
    let mut _2758: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2759: core::ffi::c_float = 0.;
    let mut _2760: core::ffi::c_float = 0.;
    let mut _2761: core::ffi::c_float = 0.;
    let mut _2762: core::ffi::c_float = 0.;
    let mut _2763: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2764: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2765: core::ffi::c_float = 0.;
    let mut _2766: core::ffi::c_float = 0.;
    let mut _2767: core::ffi::c_float = 0.;
    let mut _2768: core::ffi::c_float = 0.;
    let mut _2769: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2770: core::ffi::c_float = 0.;
    let mut _2771: core::ffi::c_float = 0.;
    let mut _2772: core::ffi::c_float = 0.;
    let mut _2773: core::ffi::c_float = 0.;
    let mut _2774: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2775: uint32_t = 0;
    let mut _2776: uint32_t = 0;
    let mut _2777: uint32_t = 0;
    let mut _2778: uint32_t = 0;
    let mut _2779: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _2780: uint32_t = 0;
    let mut _2781: uint32_t = 0;
    let mut _2782: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2783: uint32_t = 0;
    let mut _2784: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2785: uint32_t = 0;
    let mut _2786: uint32_t = 0;
    let mut _2787: uint32_t = 0;
    let mut _2788: uint32_t = 0;
    let mut _2789: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _2790: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _2791: uint32_t = 0;
    let mut _2792: uint32_t = 0;
    let mut _2793: uint32_t = 0;
    let mut _2794: uint32_t = 0;
    let mut _2795: uint32_t = 0;
    let mut _2796: uint32_t = 0;
    let mut _2797: uint32_t = 0;
    let mut _2798: uint32_t = 0;
    let mut _2799: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2800: uint8_t = 0;
    let mut _2801: uint64_t = 0;
    let mut _2802: uint8_t = 0;
    let mut _2803: uint64_t = 0;
    let mut _2804: uint8_t = 0;
    let mut _2805: uint64_t = 0;
    let mut _2806: uint8_t = 0;
    let mut _2807: uint64_t = 0;
    let mut _2808: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _2809: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2810: uint32_t = 0;
    let mut _2811: uint32_t = 0;
    let mut _2812: uint32_t = 0;
    let mut _2813: uint32_t = 0;
    let mut _2814: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2815: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2816: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2817: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2818: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _2819: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _2820: uint32_t = 0;
    let mut _2821: uint32_t = 0;
    let mut _2822: uint32_t = 0;
    let mut _2823: uint32_t = 0;
    let mut _2824: uint32_t = 0;
    let mut _2825: uint32_t = 0;
    let mut _2826: uint32_t = 0;
    let mut _2827: uint32_t = 0;
    let mut _2828: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2829: uint32_t = 0;
    let mut _2830: uint32_t = 0;
    let mut _2831: uint32_t = 0;
    let mut _2832: uint32_t = 0;
    let mut _2833: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _2834: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2835: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2836: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2837: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2838: uint32_t = 0;
    let mut _2839: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2840: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2841: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2842: core::ffi::c_float = 0.;
    let mut _2843: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2844: core::ffi::c_float = 0.;
    let mut _2845: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2846: core::ffi::c_float = 0.;
    let mut _2847: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2848: core::ffi::c_float = 0.;
    let mut _2849: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2850: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2851: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2852: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2853: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2854: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _2855: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _2856: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _2857: uint32_t = 0;
    let mut _2858: core::ffi::c_float = 0.;
    let mut _2859: core::ffi::c_float = 0.;
    let mut _2860: core::ffi::c_float = 0.;
    let mut _2860__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _2861: uint32_t = 0;
    let mut _2862: core::ffi::c_float = 0.;
    let mut _2863: core::ffi::c_float = 0.;
    let mut _2864: core::ffi::c_float = 0.;
    let mut _2864__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _2865: uint32_t = 0;
    let mut _2866: core::ffi::c_float = 0.;
    let mut _2867: core::ffi::c_float = 0.;
    let mut _2868: core::ffi::c_float = 0.;
    let mut _2868__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _2869: uint32_t = 0;
    let mut _2870: core::ffi::c_float = 0.;
    let mut _2871: core::ffi::c_float = 0.;
    let mut _2872: core::ffi::c_float = 0.;
    let mut _2872__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _2873: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2874: core::ffi::c_float = 0.;
    let mut _2875: core::ffi::c_float = 0.;
    let mut _2876: core::ffi::c_float = 0.;
    let mut _2877: core::ffi::c_float = 0.;
    let mut _2878: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2879: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _2880: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _2881: core::ffi::c_float = 0.;
    let mut _2882: core::ffi::c_float = 0.;
    let mut _2883: core::ffi::c_float = 0.;
    let mut _2884: core::ffi::c_float = 0.;
    let mut _2885: core::ffi::c_float = 0.;
    let mut _2885__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _2886: core::ffi::c_float = 0.;
    let mut _2887: core::ffi::c_float = 0.;
    let mut _2888: core::ffi::c_float = 0.;
    let mut _2889: core::ffi::c_float = 0.;
    let mut _2890: core::ffi::c_float = 0.;
    let mut _2890__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _2891: core::ffi::c_float = 0.;
    let mut _2892: core::ffi::c_float = 0.;
    let mut _2893: core::ffi::c_float = 0.;
    let mut _2894: core::ffi::c_float = 0.;
    let mut _2895: core::ffi::c_float = 0.;
    let mut _2895__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _2896: core::ffi::c_float = 0.;
    let mut _2897: core::ffi::c_float = 0.;
    let mut _2898: core::ffi::c_float = 0.;
    let mut _2899: core::ffi::c_float = 0.;
    let mut _2900: core::ffi::c_float = 0.;
    let mut _2900__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _2901: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2902: core::ffi::c_float = 0.;
    let mut _2903: core::ffi::c_float = 0.;
    let mut _2904: core::ffi::c_float = 0.;
    let mut _2905: core::ffi::c_float = 0.;
    let mut _2906: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2907: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2908: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2909: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2910: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2911: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2912: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _2913: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _2914: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _2915: uint32_t = 0;
    let mut _2916: core::ffi::c_float = 0.;
    let mut _2917: core::ffi::c_float = 0.;
    let mut _2918: core::ffi::c_float = 0.;
    let mut _2918__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _2919: uint32_t = 0;
    let mut _2920: core::ffi::c_float = 0.;
    let mut _2921: core::ffi::c_float = 0.;
    let mut _2922: core::ffi::c_float = 0.;
    let mut _2922__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _2923: uint32_t = 0;
    let mut _2924: core::ffi::c_float = 0.;
    let mut _2925: core::ffi::c_float = 0.;
    let mut _2926: core::ffi::c_float = 0.;
    let mut _2926__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _2927: uint32_t = 0;
    let mut _2928: core::ffi::c_float = 0.;
    let mut _2929: core::ffi::c_float = 0.;
    let mut _2930: core::ffi::c_float = 0.;
    let mut _2930__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _2931: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2932: core::ffi::c_float = 0.;
    let mut _2933: core::ffi::c_float = 0.;
    let mut _2934: core::ffi::c_float = 0.;
    let mut _2935: core::ffi::c_float = 0.;
    let mut _2936: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2937: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _2938: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _2939: core::ffi::c_float = 0.;
    let mut _2940: core::ffi::c_float = 0.;
    let mut _2941: core::ffi::c_float = 0.;
    let mut _2942: core::ffi::c_float = 0.;
    let mut _2943: core::ffi::c_float = 0.;
    let mut _2943__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _2944: core::ffi::c_float = 0.;
    let mut _2945: core::ffi::c_float = 0.;
    let mut _2946: core::ffi::c_float = 0.;
    let mut _2947: core::ffi::c_float = 0.;
    let mut _2948: core::ffi::c_float = 0.;
    let mut _2948__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _2949: core::ffi::c_float = 0.;
    let mut _2950: core::ffi::c_float = 0.;
    let mut _2951: core::ffi::c_float = 0.;
    let mut _2952: core::ffi::c_float = 0.;
    let mut _2953: core::ffi::c_float = 0.;
    let mut _2953__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _2954: core::ffi::c_float = 0.;
    let mut _2955: core::ffi::c_float = 0.;
    let mut _2956: core::ffi::c_float = 0.;
    let mut _2957: core::ffi::c_float = 0.;
    let mut _2958: core::ffi::c_float = 0.;
    let mut _2958__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _2959: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2960: core::ffi::c_float = 0.;
    let mut _2961: core::ffi::c_float = 0.;
    let mut _2962: core::ffi::c_float = 0.;
    let mut _2963: core::ffi::c_float = 0.;
    let mut _2964: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2965: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2966: uint32_t = 0;
    let mut _2967: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2968: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _2969: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2970: core::ffi::c_float = 0.;
    let mut _2971: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2972: core::ffi::c_float = 0.;
    let mut _2973: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2974: core::ffi::c_float = 0.;
    let mut _2975: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2976: core::ffi::c_float = 0.;
    let mut _2977: core::ffi::c_float = 0.;
    let mut _2978: core::ffi::c_float = 0.;
    let mut _2979: core::ffi::c_float = 0.;
    let mut _2980: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2981: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2982: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2983: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _2984: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2985: core::ffi::c_float = 0.;
    let mut _2986: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2987: core::ffi::c_float = 0.;
    let mut _2988: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2989: core::ffi::c_float = 0.;
    let mut _2990: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2991: core::ffi::c_float = 0.;
    let mut _2992: core::ffi::c_float = 0.;
    let mut _2993: core::ffi::c_float = 0.;
    let mut _2994: core::ffi::c_float = 0.;
    let mut _2995: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _2996: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _2997: uint32_t = 0;
    let mut _2998: uint32_t = 0;
    let mut _2999: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3000: core::ffi::c_float = 0.;
    let mut _3001: core::ffi::c_float = 0.;
    let mut _3002: core::ffi::c_float = 0.;
    let mut _3003: core::ffi::c_float = 0.;
    let mut _3004: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _3005: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3006: core::ffi::c_float = 0.;
    let mut _3007: core::ffi::c_float = 0.;
    let mut _3008: core::ffi::c_float = 0.;
    let mut _3009: core::ffi::c_float = 0.;
    let mut _3010: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _3011: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3012: core::ffi::c_float = 0.;
    let mut _3013: core::ffi::c_float = 0.;
    let mut _3014: core::ffi::c_float = 0.;
    let mut _3015: core::ffi::c_float = 0.;
    let mut _3016: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _3017: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3018: uint32_t = 0;
    let mut _3019: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3020: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3021: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3022: core::ffi::c_float = 0.;
    let mut _3023: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3024: core::ffi::c_float = 0.;
    let mut _3025: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3026: core::ffi::c_float = 0.;
    let mut _3027: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3028: core::ffi::c_float = 0.;
    let mut _3029: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _3030: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3031: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3032: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3033: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3034: core::ffi::c_float = 0.;
    let mut _3035: core::ffi::c_float = 0.;
    let mut _3036: core::ffi::c_float = 0.;
    let mut _3037: core::ffi::c_float = 0.;
    let mut _3038: core::ffi::c_float = 0.;
    let mut _3039: core::ffi::c_float = 0.;
    let mut _3040: core::ffi::c_float = 0.;
    let mut _3041: core::ffi::c_float = 0.;
    let mut _3042: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3043: core::ffi::c_float = 0.;
    let mut _3044: core::ffi::c_float = 0.;
    let mut _3045: core::ffi::c_float = 0.;
    let mut _3046: core::ffi::c_float = 0.;
    let mut _3047: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _3048: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3049: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3050: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3051: core::ffi::c_float = 0.;
    let mut _3052: core::ffi::c_float = 0.;
    let mut _3053: core::ffi::c_float = 0.;
    let mut _3054: core::ffi::c_float = 0.;
    let mut _3055: core::ffi::c_float = 0.;
    let mut _3056: core::ffi::c_float = 0.;
    let mut _3057: core::ffi::c_float = 0.;
    let mut _3058: core::ffi::c_float = 0.;
    let mut _3059: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3060: core::ffi::c_float = 0.;
    let mut _3061: core::ffi::c_float = 0.;
    let mut _3062: core::ffi::c_float = 0.;
    let mut _3063: core::ffi::c_float = 0.;
    let mut _3064: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _3065: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3066: uint32_t = 0;
    let mut _3067: core::ffi::c_float = 0.;
    let mut _3068: core::ffi::c_float = 0.;
    let mut _3069: core::ffi::c_float = 0.;
    let mut _3070: core::ffi::c_float = 0.;
    let mut _3071: core::ffi::c_float = 0.;
    let mut _3072: core::ffi::c_float = 0.;
    let mut _3073: core::ffi::c_float = 0.;
    let mut _3074: core::ffi::c_float = 0.;
    let mut _3075: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3076: core::ffi::c_float = 0.;
    let mut _3077: core::ffi::c_float = 0.;
    let mut _3078: core::ffi::c_float = 0.;
    let mut _3079: core::ffi::c_float = 0.;
    let mut _3080: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _3081: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3082: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3083: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3084: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3085: core::ffi::c_float = 0.;
    let mut _3086: core::ffi::c_float = 0.;
    let mut _3087: core::ffi::c_float = 0.;
    let mut _3088: core::ffi::c_float = 0.;
    let mut _3089: core::ffi::c_float = 0.;
    let mut _3090: core::ffi::c_float = 0.;
    let mut _3091: core::ffi::c_float = 0.;
    let mut _3092: core::ffi::c_float = 0.;
    let mut _3093: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3094: core::ffi::c_float = 0.;
    let mut _3095: core::ffi::c_float = 0.;
    let mut _3096: core::ffi::c_float = 0.;
    let mut _3097: core::ffi::c_float = 0.;
    let mut _3098: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _3099: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3100: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3101: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3102: core::ffi::c_float = 0.;
    let mut _3103: core::ffi::c_float = 0.;
    let mut _3104: core::ffi::c_float = 0.;
    let mut _3105: core::ffi::c_float = 0.;
    let mut _3106: core::ffi::c_float = 0.;
    let mut _3107: core::ffi::c_float = 0.;
    let mut _3108: core::ffi::c_float = 0.;
    let mut _3109: core::ffi::c_float = 0.;
    let mut _3110: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3111: core::ffi::c_float = 0.;
    let mut _3112: core::ffi::c_float = 0.;
    let mut _3113: core::ffi::c_float = 0.;
    let mut _3114: core::ffi::c_float = 0.;
    let mut _3115: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _3116: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3117: uint32_t = 0;
    let mut _3118: core::ffi::c_float = 0.;
    let mut _3119: core::ffi::c_float = 0.;
    let mut _3120: core::ffi::c_float = 0.;
    let mut _3121: core::ffi::c_float = 0.;
    let mut _3122: core::ffi::c_float = 0.;
    let mut _3123: core::ffi::c_float = 0.;
    let mut _3124: core::ffi::c_float = 0.;
    let mut _3125: core::ffi::c_float = 0.;
    let mut _3126: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3127: core::ffi::c_float = 0.;
    let mut _3128: core::ffi::c_float = 0.;
    let mut _3129: core::ffi::c_float = 0.;
    let mut _3130: core::ffi::c_float = 0.;
    let mut _3131: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _3132: uint32_t = 0;
    let mut _3133: uint32_t = 0;
    let mut _3134: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3135: uint32_t = 0;
    let mut _3136: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3137: core::ffi::c_float = 0.;
    let mut _3138: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3139: core::ffi::c_float = 0.;
    let mut _3140: core::ffi::c_float = 0.;
    let mut _3141: core::ffi::c_float = 0.;
    let mut _3142: core::ffi::c_float = 0.;
    let mut _3143: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _3144: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3145: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3146: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3147: core::ffi::c_float = 0.;
    let mut _3148: core::ffi::c_float = 0.;
    let mut _3149: core::ffi::c_float = 0.;
    let mut _3150: core::ffi::c_float = 0.;
    let mut _3151: core::ffi::c_float = 0.;
    let mut _3152: core::ffi::c_float = 0.;
    let mut _3153: core::ffi::c_float = 0.;
    let mut _3154: core::ffi::c_float = 0.;
    let mut _3155: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3156: core::ffi::c_float = 0.;
    let mut _3157: core::ffi::c_float = 0.;
    let mut _3158: core::ffi::c_float = 0.;
    let mut _3159: core::ffi::c_float = 0.;
    let mut _3160: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _3161: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3162: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3163: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3164: core::ffi::c_float = 0.;
    let mut _3165: core::ffi::c_float = 0.;
    let mut _3166: core::ffi::c_float = 0.;
    let mut _3167: core::ffi::c_float = 0.;
    let mut _3168: core::ffi::c_float = 0.;
    let mut _3169: core::ffi::c_float = 0.;
    let mut _3170: core::ffi::c_float = 0.;
    let mut _3171: core::ffi::c_float = 0.;
    let mut _3172: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3173: core::ffi::c_float = 0.;
    let mut _3174: core::ffi::c_float = 0.;
    let mut _3175: core::ffi::c_float = 0.;
    let mut _3176: core::ffi::c_float = 0.;
    let mut _3177: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _3178: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3179: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3180: uint32_t = 0;
    let mut _3181: core::ffi::c_float = 0.;
    let mut _3182: core::ffi::c_float = 0.;
    let mut _3183: core::ffi::c_float = 0.;
    let mut _3184: core::ffi::c_float = 0.;
    let mut _3185: core::ffi::c_float = 0.;
    let mut _3186: core::ffi::c_float = 0.;
    let mut _3187: core::ffi::c_float = 0.;
    let mut _3188: core::ffi::c_float = 0.;
    let mut _3189: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3190: core::ffi::c_float = 0.;
    let mut _3191: core::ffi::c_float = 0.;
    let mut _3192: core::ffi::c_float = 0.;
    let mut _3193: core::ffi::c_float = 0.;
    let mut _3194: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _3195: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3196: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3197: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3198: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3199: core::ffi::c_float = 0.;
    let mut _3200: core::ffi::c_float = 0.;
    let mut _3201: core::ffi::c_float = 0.;
    let mut _3202: core::ffi::c_float = 0.;
    let mut _3203: core::ffi::c_float = 0.;
    let mut _3204: core::ffi::c_float = 0.;
    let mut _3205: core::ffi::c_float = 0.;
    let mut _3206: core::ffi::c_float = 0.;
    let mut _3207: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3208: core::ffi::c_float = 0.;
    let mut _3209: core::ffi::c_float = 0.;
    let mut _3210: core::ffi::c_float = 0.;
    let mut _3211: core::ffi::c_float = 0.;
    let mut _3212: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _3213: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3214: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3215: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3216: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3217: core::ffi::c_float = 0.;
    let mut _3218: core::ffi::c_float = 0.;
    let mut _3219: core::ffi::c_float = 0.;
    let mut _3220: core::ffi::c_float = 0.;
    let mut _3221: core::ffi::c_float = 0.;
    let mut _3222: core::ffi::c_float = 0.;
    let mut _3223: core::ffi::c_float = 0.;
    let mut _3224: core::ffi::c_float = 0.;
    let mut _3225: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3226: core::ffi::c_float = 0.;
    let mut _3227: core::ffi::c_float = 0.;
    let mut _3228: core::ffi::c_float = 0.;
    let mut _3229: core::ffi::c_float = 0.;
    let mut _3230: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _3231: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3232: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3233: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3234: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3235: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3236: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3237: core::ffi::c_float = 0.;
    let mut _3238: core::ffi::c_float = 0.;
    let mut _3239: core::ffi::c_float = 0.;
    let mut _3240: core::ffi::c_float = 0.;
    let mut _3241: core::ffi::c_float = 0.;
    let mut _3242: core::ffi::c_float = 0.;
    let mut _3243: core::ffi::c_float = 0.;
    let mut _3244: core::ffi::c_float = 0.;
    let mut _3245: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3246: core::ffi::c_float = 0.;
    let mut _3247: core::ffi::c_float = 0.;
    let mut _3248: core::ffi::c_float = 0.;
    let mut _3249: core::ffi::c_float = 0.;
    let mut _3250: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _3251: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3252: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3253: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3254: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3255: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3256: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3257: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3258: core::ffi::c_float = 0.;
    let mut _3259: core::ffi::c_float = 0.;
    let mut _3260: core::ffi::c_float = 0.;
    let mut _3261: core::ffi::c_float = 0.;
    let mut _3262: core::ffi::c_float = 0.;
    let mut _3263: core::ffi::c_float = 0.;
    let mut _3264: core::ffi::c_float = 0.;
    let mut _3265: core::ffi::c_float = 0.;
    let mut _3266: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3267: uint8_t = 0;
    let mut _3268: uint64_t = 0;
    let mut _3269: uint8_t = 0;
    let mut _3270: uint64_t = 0;
    let mut _3271: uint8_t = 0;
    let mut _3272: uint64_t = 0;
    let mut _3273: uint8_t = 0;
    let mut _3274: uint64_t = 0;
    let mut _3275: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _3276: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3277: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3278: core::ffi::c_float = 0.;
    let mut _3279: core::ffi::c_float = 0.;
    let mut _3280: core::ffi::c_float = 0.;
    let mut _3281: core::ffi::c_float = 0.;
    let mut _3282: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3283: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3284: core::ffi::c_float = 0.;
    let mut _3285: core::ffi::c_float = 0.;
    let mut _3286: core::ffi::c_float = 0.;
    let mut _3287: core::ffi::c_float = 0.;
    let mut _3288: core::ffi::c_float = 0.;
    let mut _3289: core::ffi::c_float = 0.;
    let mut _3290: core::ffi::c_float = 0.;
    let mut _3291: core::ffi::c_float = 0.;
    let mut _3292: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3293: core::ffi::c_float = 0.;
    let mut _3294: core::ffi::c_float = 0.;
    let mut _3295: core::ffi::c_float = 0.;
    let mut _3296: core::ffi::c_float = 0.;
    let mut _3297: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _3298: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3299: core::ffi::c_float = 0.;
    let mut _3300: core::ffi::c_float = 0.;
    let mut _3301: core::ffi::c_float = 0.;
    let mut _3302: core::ffi::c_float = 0.;
    let mut _3303: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3304: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3305: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3306: core::ffi::c_float = 0.;
    let mut _3307: core::ffi::c_float = 0.;
    let mut _3308: core::ffi::c_float = 0.;
    let mut _3309: core::ffi::c_float = 0.;
    let mut _3310: core::ffi::c_float = 0.;
    let mut _3311: core::ffi::c_float = 0.;
    let mut _3312: core::ffi::c_float = 0.;
    let mut _3313: core::ffi::c_float = 0.;
    let mut _3314: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3315: core::ffi::c_float = 0.;
    let mut _3316: core::ffi::c_float = 0.;
    let mut _3317: core::ffi::c_float = 0.;
    let mut _3318: core::ffi::c_float = 0.;
    let mut _3319: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _3320: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3321: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3322: core::ffi::c_float = 0.;
    let mut _3323: core::ffi::c_float = 0.;
    let mut _3324: core::ffi::c_float = 0.;
    let mut _3325: core::ffi::c_float = 0.;
    let mut _3326: core::ffi::c_float = 0.;
    let mut _3327: core::ffi::c_float = 0.;
    let mut _3328: core::ffi::c_float = 0.;
    let mut _3329: core::ffi::c_float = 0.;
    let mut _3330: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3331: core::ffi::c_float = 0.;
    let mut _3332: core::ffi::c_float = 0.;
    let mut _3333: core::ffi::c_float = 0.;
    let mut _3334: core::ffi::c_float = 0.;
    let mut _3335: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _3336: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3337: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3338: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3339: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3340: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3341: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _3342: uint32_t = 0;
    let mut _3343: core::ffi::c_float = 0.;
    let mut _3344: core::ffi::c_float = 0.;
    let mut _3345: core::ffi::c_float = 0.;
    let mut _3345__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _3346: uint32_t = 0;
    let mut _3347: core::ffi::c_float = 0.;
    let mut _3348: core::ffi::c_float = 0.;
    let mut _3349: core::ffi::c_float = 0.;
    let mut _3349__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _3350: uint32_t = 0;
    let mut _3351: core::ffi::c_float = 0.;
    let mut _3352: core::ffi::c_float = 0.;
    let mut _3353: core::ffi::c_float = 0.;
    let mut _3353__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _3354: uint32_t = 0;
    let mut _3355: core::ffi::c_float = 0.;
    let mut _3356: core::ffi::c_float = 0.;
    let mut _3357: core::ffi::c_float = 0.;
    let mut _3357__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _3358: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3359: core::ffi::c_float = 0.;
    let mut _3360: core::ffi::c_float = 0.;
    let mut _3361: core::ffi::c_float = 0.;
    let mut _3362: core::ffi::c_float = 0.;
    let mut _3363: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _3364: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3365: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3366: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3367: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3368: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3369: core::ffi::c_float = 0.;
    let mut _3370: core::ffi::c_float = 0.;
    let mut _3371: core::ffi::c_float = 0.;
    let mut _3372: core::ffi::c_float = 0.;
    let mut _3373: core::ffi::c_float = 0.;
    let mut _3374: core::ffi::c_float = 0.;
    let mut _3375: core::ffi::c_float = 0.;
    let mut _3376: core::ffi::c_float = 0.;
    let mut _3377: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3378: uint8_t = 0;
    let mut _3379: uint64_t = 0;
    let mut _3380: uint8_t = 0;
    let mut _3381: uint64_t = 0;
    let mut _3382: uint8_t = 0;
    let mut _3383: uint64_t = 0;
    let mut _3384: uint8_t = 0;
    let mut _3385: uint64_t = 0;
    let mut _3386: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _3387: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3388: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3389: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3390: core::ffi::c_float = 0.;
    let mut _3391: core::ffi::c_float = 0.;
    let mut _3392: core::ffi::c_float = 0.;
    let mut _3393: core::ffi::c_float = 0.;
    let mut _3394: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3395: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3396: core::ffi::c_float = 0.;
    let mut _3397: core::ffi::c_float = 0.;
    let mut _3398: core::ffi::c_float = 0.;
    let mut _3399: core::ffi::c_float = 0.;
    let mut _3400: core::ffi::c_float = 0.;
    let mut _3401: core::ffi::c_float = 0.;
    let mut _3402: core::ffi::c_float = 0.;
    let mut _3403: core::ffi::c_float = 0.;
    let mut _3404: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3405: core::ffi::c_float = 0.;
    let mut _3406: core::ffi::c_float = 0.;
    let mut _3407: core::ffi::c_float = 0.;
    let mut _3408: core::ffi::c_float = 0.;
    let mut _3409: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _3410: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3411: core::ffi::c_float = 0.;
    let mut _3412: core::ffi::c_float = 0.;
    let mut _3413: core::ffi::c_float = 0.;
    let mut _3414: core::ffi::c_float = 0.;
    let mut _3415: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3416: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3417: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3418: core::ffi::c_float = 0.;
    let mut _3419: core::ffi::c_float = 0.;
    let mut _3420: core::ffi::c_float = 0.;
    let mut _3421: core::ffi::c_float = 0.;
    let mut _3422: core::ffi::c_float = 0.;
    let mut _3423: core::ffi::c_float = 0.;
    let mut _3424: core::ffi::c_float = 0.;
    let mut _3425: core::ffi::c_float = 0.;
    let mut _3426: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3427: core::ffi::c_float = 0.;
    let mut _3428: core::ffi::c_float = 0.;
    let mut _3429: core::ffi::c_float = 0.;
    let mut _3430: core::ffi::c_float = 0.;
    let mut _3431: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _3432: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3433: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3434: core::ffi::c_float = 0.;
    let mut _3435: core::ffi::c_float = 0.;
    let mut _3436: core::ffi::c_float = 0.;
    let mut _3437: core::ffi::c_float = 0.;
    let mut _3438: core::ffi::c_float = 0.;
    let mut _3439: core::ffi::c_float = 0.;
    let mut _3440: core::ffi::c_float = 0.;
    let mut _3441: core::ffi::c_float = 0.;
    let mut _3442: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3443: core::ffi::c_float = 0.;
    let mut _3444: core::ffi::c_float = 0.;
    let mut _3445: core::ffi::c_float = 0.;
    let mut _3446: core::ffi::c_float = 0.;
    let mut _3447: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _3448: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3449: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3450: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3451: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3452: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3453: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3454: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _3455: uint32_t = 0;
    let mut _3456: core::ffi::c_float = 0.;
    let mut _3457: core::ffi::c_float = 0.;
    let mut _3458: core::ffi::c_float = 0.;
    let mut _3458__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _3459: uint32_t = 0;
    let mut _3460: core::ffi::c_float = 0.;
    let mut _3461: core::ffi::c_float = 0.;
    let mut _3462: core::ffi::c_float = 0.;
    let mut _3462__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _3463: uint32_t = 0;
    let mut _3464: core::ffi::c_float = 0.;
    let mut _3465: core::ffi::c_float = 0.;
    let mut _3466: core::ffi::c_float = 0.;
    let mut _3466__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _3467: uint32_t = 0;
    let mut _3468: core::ffi::c_float = 0.;
    let mut _3469: core::ffi::c_float = 0.;
    let mut _3470: core::ffi::c_float = 0.;
    let mut _3470__PHI_TEMPORARY: core::ffi::c_float = 0.;
    let mut _3471: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3472: core::ffi::c_float = 0.;
    let mut _3473: core::ffi::c_float = 0.;
    let mut _3474: core::ffi::c_float = 0.;
    let mut _3475: core::ffi::c_float = 0.;
    let mut _3476: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _3477: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3478: uint32_t = 0;
    let mut _3479: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3480: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3481: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3482: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3483: core::ffi::c_float = 0.;
    let mut _3484: core::ffi::c_float = 0.;
    let mut _3485: core::ffi::c_float = 0.;
    let mut _3486: core::ffi::c_float = 0.;
    let mut _3487: core::ffi::c_float = 0.;
    let mut _3488: core::ffi::c_float = 0.;
    let mut _3489: core::ffi::c_float = 0.;
    let mut _3490: core::ffi::c_float = 0.;
    let mut _3491: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3492: core::ffi::c_float = 0.;
    let mut _3493: core::ffi::c_float = 0.;
    let mut _3494: core::ffi::c_float = 0.;
    let mut _3495: core::ffi::c_float = 0.;
    let mut _3496: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _3497: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3498: core::ffi::c_float = 0.;
    let mut _3499: core::ffi::c_float = 0.;
    let mut _3500: core::ffi::c_float = 0.;
    let mut _3501: core::ffi::c_float = 0.;
    let mut _3502: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3503: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3504: core::ffi::c_float = 0.;
    let mut _3505: core::ffi::c_float = 0.;
    let mut _3506: core::ffi::c_float = 0.;
    let mut _3507: core::ffi::c_float = 0.;
    let mut _3508: core::ffi::c_float = 0.;
    let mut _3509: core::ffi::c_float = 0.;
    let mut _3510: core::ffi::c_float = 0.;
    let mut _3511: core::ffi::c_float = 0.;
    let mut _3512: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3513: core::ffi::c_float = 0.;
    let mut _3514: core::ffi::c_float = 0.;
    let mut _3515: core::ffi::c_float = 0.;
    let mut _3516: core::ffi::c_float = 0.;
    let mut _3517: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _3518: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3519: core::ffi::c_float = 0.;
    let mut _3520: core::ffi::c_float = 0.;
    let mut _3521: core::ffi::c_float = 0.;
    let mut _3522: core::ffi::c_float = 0.;
    let mut _3523: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3524: uint32_t = 0;
    let mut _3525: uint32_t = 0;
    let mut _3526: uint32_t = 0;
    let mut _3527: uint32_t = 0;
    let mut _3528: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _3529: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3530: uint32_t = 0;
    let mut _3531: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3532: uint32_t = 0;
    let mut _3533: uint32_t = 0;
    let mut _3534: uint32_t = 0;
    let mut _3535: uint32_t = 0;
    let mut _3536: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _3537: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _3538: uint32_t = 0;
    let mut _3539: uint32_t = 0;
    let mut _3540: uint32_t = 0;
    let mut _3541: uint32_t = 0;
    let mut _3542: uint32_t = 0;
    let mut _3542__PHI_TEMPORARY: uint32_t = 0;
    let mut _3543: uint32_t = 0;
    let mut _3544: uint32_t = 0;
    let mut _3545: uint32_t = 0;
    let mut _3546: uint32_t = 0;
    let mut _3547: uint32_t = 0;
    let mut _3547__PHI_TEMPORARY: uint32_t = 0;
    let mut _3548: uint32_t = 0;
    let mut _3549: uint32_t = 0;
    let mut _3550: uint32_t = 0;
    let mut _3551: uint32_t = 0;
    let mut _3552: uint32_t = 0;
    let mut _3552__PHI_TEMPORARY: uint32_t = 0;
    let mut _3553: uint32_t = 0;
    let mut _3554: uint32_t = 0;
    let mut _3555: uint32_t = 0;
    let mut _3556: uint32_t = 0;
    let mut _3557: uint32_t = 0;
    let mut _3557__PHI_TEMPORARY: uint32_t = 0;
    let mut _3558: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3559: uint32_t = 0;
    let mut _3560: uint32_t = 0;
    let mut _3561: uint32_t = 0;
    let mut _3562: uint32_t = 0;
    let mut _3563: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _3564: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3565: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3566: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3567: uint32_t = 0;
    let mut _3568: uint32_t = 0;
    let mut _3569: uint32_t = 0;
    let mut _3570: uint32_t = 0;
    let mut _3571: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _3572: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _3573: uint32_t = 0;
    let mut _3574: uint32_t = 0;
    let mut _3575: uint32_t = 0;
    let mut _3576: uint32_t = 0;
    let mut _3577: uint32_t = 0;
    let mut _3577__PHI_TEMPORARY: uint32_t = 0;
    let mut _3578: uint32_t = 0;
    let mut _3579: uint32_t = 0;
    let mut _3580: uint32_t = 0;
    let mut _3581: uint32_t = 0;
    let mut _3582: uint32_t = 0;
    let mut _3582__PHI_TEMPORARY: uint32_t = 0;
    let mut _3583: uint32_t = 0;
    let mut _3584: uint32_t = 0;
    let mut _3585: uint32_t = 0;
    let mut _3586: uint32_t = 0;
    let mut _3587: uint32_t = 0;
    let mut _3587__PHI_TEMPORARY: uint32_t = 0;
    let mut _3588: uint32_t = 0;
    let mut _3589: uint32_t = 0;
    let mut _3590: uint32_t = 0;
    let mut _3591: uint32_t = 0;
    let mut _3592: uint32_t = 0;
    let mut _3592__PHI_TEMPORARY: uint32_t = 0;
    let mut _3593: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3594: uint32_t = 0;
    let mut _3595: uint32_t = 0;
    let mut _3596: uint32_t = 0;
    let mut _3597: uint32_t = 0;
    let mut _3598: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _3599: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3600: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3601: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3602: uint32_t = 0;
    let mut _3603: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3604: core::ffi::c_float = 0.;
    let mut _3605: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3606: core::ffi::c_float = 0.;
    let mut _3607: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3608: core::ffi::c_float = 0.;
    let mut _3609: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3610: core::ffi::c_float = 0.;
    let mut _3611: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3612: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3613: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3614: uint32_t = 0;
    let mut _3615: crate::l_array_2_uint64_t = crate::l_array_2_uint64_t { array: [0; 2] };
    let mut _3616: uint32_t = 0;
    let mut _3617: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3618: uint32_t = 0;
    let mut _3619: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3620: uint32_t = 0;
    let mut _3621: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3622: uint32_t = 0;
    let mut _3623: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3624: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3625: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3626: core::ffi::c_float = 0.;
    let mut _3627: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3628: core::ffi::c_float = 0.;
    let mut _3629: core::ffi::c_float = 0.;
    let mut _3630: core::ffi::c_float = 0.;
    let mut _3631: core::ffi::c_float = 0.;
    let mut _3632: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3633: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3634: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3635: core::ffi::c_float = 0.;
    let mut _3636: core::ffi::c_float = 0.;
    let mut _3637: core::ffi::c_float = 0.;
    let mut _3638: core::ffi::c_float = 0.;
    let mut _3639: core::ffi::c_float = 0.;
    let mut _3640: core::ffi::c_float = 0.;
    let mut _3641: core::ffi::c_float = 0.;
    let mut _3642: core::ffi::c_float = 0.;
    let mut _3643: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3644: core::ffi::c_float = 0.;
    let mut _3645: core::ffi::c_float = 0.;
    let mut _3646: core::ffi::c_float = 0.;
    let mut _3647: core::ffi::c_float = 0.;
    let mut _3648: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _3649: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _3650: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3651: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3652: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3653: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3654: core::ffi::c_float = 0.;
    let mut _3655: core::ffi::c_float = 0.;
    let mut _3656: core::ffi::c_float = 0.;
    let mut _3657: core::ffi::c_float = 0.;
    let mut _3658: core::ffi::c_float = 0.;
    let mut _3659: core::ffi::c_float = 0.;
    let mut _3660: core::ffi::c_float = 0.;
    let mut _3661: core::ffi::c_float = 0.;
    let mut _3662: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3663: core::ffi::c_float = 0.;
    let mut _3664: core::ffi::c_float = 0.;
    let mut _3665: core::ffi::c_float = 0.;
    let mut _3666: core::ffi::c_float = 0.;
    let mut _3667: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _3668: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3669: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3670: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3671: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3672: core::ffi::c_float = 0.;
    let mut _3673: core::ffi::c_float = 0.;
    let mut _3674: core::ffi::c_float = 0.;
    let mut _3675: core::ffi::c_float = 0.;
    let mut _3676: core::ffi::c_float = 0.;
    let mut _3677: core::ffi::c_float = 0.;
    let mut _3678: core::ffi::c_float = 0.;
    let mut _3679: core::ffi::c_float = 0.;
    let mut _3680: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3681: core::ffi::c_float = 0.;
    let mut _3682: core::ffi::c_float = 0.;
    let mut _3683: core::ffi::c_float = 0.;
    let mut _3684: core::ffi::c_float = 0.;
    let mut _3685: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _3686: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3687: uint32_t = 0;
    let mut _3688: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3689: core::ffi::c_float = 0.;
    let mut _3690: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3691: core::ffi::c_float = 0.;
    let mut _3692: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3693: core::ffi::c_float = 0.;
    let mut _3694: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3695: core::ffi::c_float = 0.;
    let mut _3696: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3697: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3698: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3699: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3700: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3701: core::ffi::c_float = 0.;
    let mut _3702: core::ffi::c_float = 0.;
    let mut _3703: core::ffi::c_float = 0.;
    let mut _3704: core::ffi::c_float = 0.;
    let mut _3705: core::ffi::c_float = 0.;
    let mut _3706: core::ffi::c_float = 0.;
    let mut _3707: core::ffi::c_float = 0.;
    let mut _3708: core::ffi::c_float = 0.;
    let mut _3709: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3710: core::ffi::c_float = 0.;
    let mut _3711: core::ffi::c_float = 0.;
    let mut _3712: core::ffi::c_float = 0.;
    let mut _3713: core::ffi::c_float = 0.;
    let mut _3714: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _3715: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3716: uint32_t = 0;
    let mut _3717: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3718: core::ffi::c_float = 0.;
    let mut _3719: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3720: core::ffi::c_float = 0.;
    let mut _3721: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3722: core::ffi::c_float = 0.;
    let mut _3723: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3724: core::ffi::c_float = 0.;
    let mut _3725: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3726: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3727: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3728: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3729: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3730: core::ffi::c_float = 0.;
    let mut _3731: core::ffi::c_float = 0.;
    let mut _3732: core::ffi::c_float = 0.;
    let mut _3733: core::ffi::c_float = 0.;
    let mut _3734: core::ffi::c_float = 0.;
    let mut _3735: core::ffi::c_float = 0.;
    let mut _3736: core::ffi::c_float = 0.;
    let mut _3737: core::ffi::c_float = 0.;
    let mut _3738: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3739: core::ffi::c_float = 0.;
    let mut _3740: core::ffi::c_float = 0.;
    let mut _3741: core::ffi::c_float = 0.;
    let mut _3742: core::ffi::c_float = 0.;
    let mut _3743: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _3744: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3745: uint32_t = 0;
    let mut _3746: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3747: core::ffi::c_float = 0.;
    let mut _3748: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3749: core::ffi::c_float = 0.;
    let mut _3750: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3751: core::ffi::c_float = 0.;
    let mut _3752: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3753: core::ffi::c_float = 0.;
    let mut _3754: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3755: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3756: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3757: core::ffi::c_float = 0.;
    let mut _3758: core::ffi::c_float = 0.;
    let mut _3759: core::ffi::c_float = 0.;
    let mut _3760: core::ffi::c_float = 0.;
    let mut _3761: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3762: crate::l_array_4_float = crate::l_array_4_float { array: [0.; 4] };
    let mut _3763: core::ffi::c_float = 0.;
    let mut _3764: core::ffi::c_float = 0.;
    let mut _3765: core::ffi::c_float = 0.;
    let mut _3766: core::ffi::c_float = 0.;
    let mut _3767: core::ffi::c_float = 0.;
    let mut _3768: core::ffi::c_float = 0.;
    let mut _3769: core::ffi::c_float = 0.;
    let mut _3770: core::ffi::c_float = 0.;
    let mut _3771: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3772: core::ffi::c_float = 0.;
    let mut _3773: core::ffi::c_float = 0.;
    let mut _3774: core::ffi::c_float = 0.;
    let mut _3775: core::ffi::c_float = 0.;
    let mut _3776: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: crate::l_array_4_float { array: [0.; 4] },
    };
    let mut _3777: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3778: uint32_t = 0;
    _2595 = _2160;
    _2596 = _2161;
    _2597 = _2162;
    _2598 = _2163;
    _2599 = _2164;
    _2600 = _2165;
    _2601 = _2166;
    _2602 = _2167;
    _2603 = _2168;
    _2604 = _2169;
    _2724 = _2595;
    if _2724 > 0 as core::ffi::c_uint {
        _2725 = _2597;
        if _2725 > 0 as core::ffi::c_uint {
            _2282 = &mut _2287 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void;
            _2283 = 0;
            _2284 = 1;
            _2285 = 2;
            _2286 = 3;
            _2726 = _2282;
            _2727 = _2283;
            *(_2726 as *mut uint32_t) = _2727;
            _2728 = _2284;
            *(&raw mut *((*(_2726 as *mut crate::l_array_4_uint32_t)).array)
                .as_mut_ptr()
                .offset(1 as core::ffi::c_int as int64_t as isize)
                as *mut uint32_t) = _2728;
            _2729 = _2285;
            *(&raw mut *((*(_2726 as *mut crate::l_array_4_uint32_t)).array)
                .as_mut_ptr()
                .offset(2 as core::ffi::c_int as int64_t as isize)
                as *mut uint32_t) = _2729;
            _2730 = _2286;
            *(&raw mut *((*(_2726 as *mut crate::l_array_4_uint32_t)).array)
                .as_mut_ptr()
                .offset(3 as core::ffi::c_int as int64_t as isize)
                as *mut uint32_t) = _2730;
            _2731 =
                *(&mut _2287 as *mut l_struct_struct_OC_vint4 as *mut crate::l_array_2_uint64_t);
            (*(&mut _2607.field0 as *mut crate::l_array_4_uint32_t as *mut C2RustUnnamed_26))
                .data = _2731;
            _2732 = (*(&mut _2607.field0 as *mut crate::l_array_4_uint32_t
                as *mut C2RustUnnamed_25))
                .data;
            *(&mut _2300 as *mut l_struct_struct_OC_vint4 as *mut crate::l_array_2_uint64_t) =
                _2732;
            _2733 = *(&mut _2300 as *mut l_struct_struct_OC_vint4 as *mut uint32_t);
            _2734 = *(&raw mut *((*(&mut _2300 as *mut l_struct_struct_OC_vint4
                as *mut crate::l_array_4_uint32_t))
                .array)
                .as_mut_ptr()
                .offset(1 as core::ffi::c_int as int64_t as isize)
                as *mut uint32_t);
            _2735 = *(&raw mut *((*(&mut _2300 as *mut l_struct_struct_OC_vint4
                as *mut crate::l_array_4_uint32_t))
                .array)
                .as_mut_ptr()
                .offset(2 as core::ffi::c_int as int64_t as isize)
                as *mut uint32_t);
            _2736 = *(&raw mut *((*(&mut _2300 as *mut l_struct_struct_OC_vint4
                as *mut crate::l_array_4_uint32_t))
                .array)
                .as_mut_ptr()
                .offset(3 as core::ffi::c_int as int64_t as isize)
                as *mut uint32_t);
            _2294 = &mut _2299 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
            _2295 = _2733 as int32_t as core::ffi::c_float;
            _2296 = _2734 as int32_t as core::ffi::c_float;
            _2297 = _2735 as int32_t as core::ffi::c_float;
            _2298 = _2736 as int32_t as core::ffi::c_float;
            _2737 = _2294;
            _2738 = _2295;
            *(_2737 as *mut core::ffi::c_float) = _2738;
            _2739 = _2296;
            *(&raw mut *((*(_2737 as *mut crate::l_array_4_float)).array)
                .as_mut_ptr()
                .offset(1 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float) = _2739;
            _2740 = _2297;
            *(&raw mut *((*(_2737 as *mut crate::l_array_4_float)).array)
                .as_mut_ptr()
                .offset(2 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float) = _2740;
            _2741 = _2298;
            *(&raw mut *((*(_2737 as *mut crate::l_array_4_float)).array)
                .as_mut_ptr()
                .offset(3 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float) = _2741;
            _2742 = _2299;
            *(&mut _2606.field0 as *mut crate::l_array_4_float) = _2742.field0;
            _2593 = &mut _2608 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
            _2594 = 1 as core::ffi::c_int as core::ffi::c_float;
            _2743 = _2593;
            _2744 = _2594;
            *(_2743 as *mut core::ffi::c_float) = _2744;
            _2745 = _2594;
            *(&raw mut *((*(_2743 as *mut crate::l_array_4_float)).array)
                .as_mut_ptr()
                .offset(1 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float) = _2745;
            _2746 = _2594;
            *(&raw mut *((*(_2743 as *mut crate::l_array_4_float)).array)
                .as_mut_ptr()
                .offset(2 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float) = _2746;
            _2747 = _2594;
            *(&raw mut *((*(_2743 as *mut crate::l_array_4_float)).array)
                .as_mut_ptr()
                .offset(3 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float) = _2747;
            _2748 = *(&mut _2606.field0 as *mut crate::l_array_4_float);
            _2749 = *(&mut _2608.field0 as *mut crate::l_array_4_float);
            *(&mut _2363 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) = _2748;
            *(&mut _2364 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) = _2749;
            _2750 = *(&mut _2363 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
            _2751 = *(&mut _2364 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
            _2752 = *(&raw mut *((*(&mut _2363 as *mut l_struct_struct_OC_vfloat4
                as *mut crate::l_array_4_float))
                .array)
                .as_mut_ptr()
                .offset(1 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float);
            _2753 = *(&raw mut *((*(&mut _2364 as *mut l_struct_struct_OC_vfloat4
                as *mut crate::l_array_4_float))
                .array)
                .as_mut_ptr()
                .offset(1 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float);
            _2754 = *(&raw mut *((*(&mut _2363 as *mut l_struct_struct_OC_vfloat4
                as *mut crate::l_array_4_float))
                .array)
                .as_mut_ptr()
                .offset(2 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float);
            _2755 = *(&raw mut *((*(&mut _2364 as *mut l_struct_struct_OC_vfloat4
                as *mut crate::l_array_4_float))
                .array)
                .as_mut_ptr()
                .offset(2 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float);
            _2756 = *(&raw mut *((*(&mut _2363 as *mut l_struct_struct_OC_vfloat4
                as *mut crate::l_array_4_float))
                .array)
                .as_mut_ptr()
                .offset(3 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float);
            _2757 = *(&raw mut *((*(&mut _2364 as *mut l_struct_struct_OC_vfloat4
                as *mut crate::l_array_4_float))
                .array)
                .as_mut_ptr()
                .offset(3 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float);
            _2357 = &mut _2362 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
            _2358 = llvm_fadd_f32(_2750, _2751);
            _2359 = llvm_fadd_f32(_2752, _2753);
            _2360 = llvm_fadd_f32(_2754, _2755);
            _2361 = llvm_fadd_f32(_2756, _2757);
            _2758 = _2357;
            _2759 = _2358;
            *(_2758 as *mut core::ffi::c_float) = _2759;
            _2760 = _2359;
            *(&raw mut *((*(_2758 as *mut crate::l_array_4_float)).array)
                .as_mut_ptr()
                .offset(1 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float) = _2760;
            _2761 = _2360;
            *(&raw mut *((*(_2758 as *mut crate::l_array_4_float)).array)
                .as_mut_ptr()
                .offset(2 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float) = _2761;
            _2762 = _2361;
            *(&raw mut *((*(_2758 as *mut crate::l_array_4_float)).array)
                .as_mut_ptr()
                .offset(3 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float) = _2762;
            _2763 = _2362;
            *(&mut _2605.field0 as *mut crate::l_array_4_float) = _2763.field0;
            _2591 = &mut _2609 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
            _2592 = 3.40282347E+38f64 as core::ffi::c_float;
            _2764 = _2591;
            _2765 = _2592;
            *(_2764 as *mut core::ffi::c_float) = _2765;
            _2766 = _2592;
            *(&raw mut *((*(_2764 as *mut crate::l_array_4_float)).array)
                .as_mut_ptr()
                .offset(1 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float) = _2766;
            _2767 = _2592;
            *(&raw mut *((*(_2764 as *mut crate::l_array_4_float)).array)
                .as_mut_ptr()
                .offset(2 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float) = _2767;
            _2768 = _2592;
            *(&raw mut *((*(_2764 as *mut crate::l_array_4_float)).array)
                .as_mut_ptr()
                .offset(3 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float) = _2768;
            _2589 = &mut _2610 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
            _2590 = -3.40282347E+38f64 as core::ffi::c_float;
            _2769 = _2589;
            _2770 = _2590;
            *(_2769 as *mut core::ffi::c_float) = _2770;
            _2771 = _2590;
            *(&raw mut *((*(_2769 as *mut crate::l_array_4_float)).array)
                .as_mut_ptr()
                .offset(1 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float) = _2771;
            _2772 = _2590;
            *(&raw mut *((*(_2769 as *mut crate::l_array_4_float)).array)
                .as_mut_ptr()
                .offset(2 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float) = _2772;
            _2773 = _2590;
            *(&raw mut *((*(_2769 as *mut crate::l_array_4_float)).array)
                .as_mut_ptr()
                .offset(3 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float) = _2773;
            _2288 = &mut _2293 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void;
            _2289 = 0;
            _2290 = 1;
            _2291 = 2;
            _2292 = 3;
            _2774 = _2288;
            _2775 = _2289;
            *(_2774 as *mut uint32_t) = _2775;
            _2776 = _2290;
            *(&raw mut *((*(_2774 as *mut crate::l_array_4_uint32_t)).array)
                .as_mut_ptr()
                .offset(1 as core::ffi::c_int as int64_t as isize)
                as *mut uint32_t) = _2776;
            _2777 = _2291;
            *(&raw mut *((*(_2774 as *mut crate::l_array_4_uint32_t)).array)
                .as_mut_ptr()
                .offset(2 as core::ffi::c_int as int64_t as isize)
                as *mut uint32_t) = _2777;
            _2778 = _2292;
            *(&raw mut *((*(_2774 as *mut crate::l_array_4_uint32_t)).array)
                .as_mut_ptr()
                .offset(3 as core::ffi::c_int as int64_t as isize)
                as *mut uint32_t) = _2778;
            _2779 =
                *(&mut _2293 as *mut l_struct_struct_OC_vint4 as *mut crate::l_array_2_uint64_t);
            (*(&mut _2611.field0 as *mut crate::l_array_4_uint32_t as *mut C2RustUnnamed_24))
                .data = _2779;
            _2612 = 0;
            loop {
                _2780 = _2612;
                _2781 = _2595;
                if !(_2780 < _2781) {
                    break;
                }
                _2782 = memcpy(
                    &mut _2614 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void,
                    &mut _2611 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void,
                    16,
                );
                _2783 = _2595;
                _2307 = &mut _2615 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void;
                _2308 = _2783;
                _2784 = _2307;
                _2785 = _2308;
                *(_2784 as *mut uint32_t) = _2785;
                _2786 = _2308;
                *(&raw mut *((*(_2784 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _2786;
                _2787 = _2308;
                *(&raw mut *((*(_2784 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _2787;
                _2788 = _2308;
                *(&raw mut *((*(_2784 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _2788;
                _2789 = (*(&mut _2614.field0 as *mut crate::l_array_4_uint32_t
                    as *mut C2RustUnnamed_23))
                    .data;
                _2790 = (*(&mut _2615.field0 as *mut crate::l_array_4_uint32_t
                    as *mut C2RustUnnamed_22))
                    .data;
                *(&mut _2280 as *mut l_struct_struct_OC_vint4 as *mut crate::l_array_2_uint64_t) =
                    _2789;
                *(&mut _2281 as *mut l_struct_struct_OC_vint4 as *mut crate::l_array_2_uint64_t) =
                    _2790;
                _2791 = *(&mut _2280 as *mut l_struct_struct_OC_vint4 as *mut uint32_t);
                _2792 = *(&mut _2281 as *mut l_struct_struct_OC_vint4 as *mut uint32_t);
                _2793 = *(&raw mut *((*(&mut _2280 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _2794 = *(&raw mut *((*(&mut _2281 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _2795 = *(&raw mut *((*(&mut _2280 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _2796 = *(&raw mut *((*(&mut _2281 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _2797 = *(&raw mut *((*(&mut _2280 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _2798 = *(&raw mut *((*(&mut _2281 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _2274 = &mut _2279 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void;
                _2275 = ((_2791 as int32_t) < _2792 as int32_t) as core::ffi::c_int as bool_0;
                _2276 = ((_2793 as int32_t) < _2794 as int32_t) as core::ffi::c_int as bool_0;
                _2277 = ((_2795 as int32_t) < _2796 as int32_t) as core::ffi::c_int as bool_0;
                _2278 = ((_2797 as int32_t) < _2798 as int32_t) as core::ffi::c_int as bool_0;
                _2799 = _2274;
                _2800 = _2275;
                _2801 = ((_2800 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0
                    as uint32_t
                    == 0 as core::ffi::c_uint) as core::ffi::c_int as bool_0
                    as uint64_t;
                *(_2799 as *mut uint32_t) = llvm_select_u32(
                    ((_2800 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0 as uint32_t
                        == 0 as core::ffi::c_uint) as core::ffi::c_int
                        as bool_0,
                    0,
                    -(1 as core::ffi::c_int) as uint32_t,
                );
                _2802 = _2276;
                _2803 = ((_2802 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0
                    as uint32_t
                    == 0 as core::ffi::c_uint) as core::ffi::c_int as bool_0
                    as uint64_t;
                *(&raw mut *((*(_2799 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = llvm_select_u32(
                    ((_2802 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0 as uint32_t
                        == 0 as core::ffi::c_uint) as core::ffi::c_int
                        as bool_0,
                    0,
                    -(1 as core::ffi::c_int) as uint32_t,
                );
                _2804 = _2277;
                _2805 = ((_2804 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0
                    as uint32_t
                    == 0 as core::ffi::c_uint) as core::ffi::c_int as bool_0
                    as uint64_t;
                *(&raw mut *((*(_2799 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = llvm_select_u32(
                    ((_2804 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0 as uint32_t
                        == 0 as core::ffi::c_uint) as core::ffi::c_int
                        as bool_0,
                    0,
                    -(1 as core::ffi::c_int) as uint32_t,
                );
                _2806 = _2278;
                _2807 = ((_2806 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0
                    as uint32_t
                    == 0 as core::ffi::c_uint) as core::ffi::c_int as bool_0
                    as uint64_t;
                *(&raw mut *((*(_2799 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = llvm_select_u32(
                    ((_2806 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0 as uint32_t
                        == 0 as core::ffi::c_uint) as core::ffi::c_int
                        as bool_0,
                    0,
                    -(1 as core::ffi::c_int) as uint32_t,
                );
                _2808 = *(&mut _2279 as *mut l_struct_struct_OC_vmask4
                    as *mut crate::l_array_2_uint64_t);
                (*(&mut _2613.field0 as *mut crate::l_array_4_uint32_t as *mut C2RustUnnamed_21))
                    .data = _2808;
                _2305 = &mut _2616 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void;
                _2306 = 4;
                _2809 = _2305;
                _2810 = _2306;
                *(_2809 as *mut uint32_t) = _2810;
                _2811 = _2306;
                *(&raw mut *((*(_2809 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _2811;
                _2812 = _2306;
                *(&raw mut *((*(_2809 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _2812;
                _2813 = _2306;
                *(&raw mut *((*(_2809 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _2813;
                _2269 = &mut _2611 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void;
                _2270 = &mut _2616 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void;
                _2814 = _2269;
                _2815 = memcpy(
                    &mut _2272 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void,
                    _2814,
                    16,
                );
                _2816 = _2270;
                _2817 = memcpy(
                    &mut _2273 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void,
                    _2816,
                    16,
                );
                _2818 = *(&mut _2272 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_2_uint64_t);
                _2819 = *(&mut _2273 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_2_uint64_t);
                *(&mut _2176 as *mut l_struct_struct_OC_vint4 as *mut crate::l_array_2_uint64_t) =
                    _2818;
                *(&mut _2177 as *mut l_struct_struct_OC_vint4 as *mut crate::l_array_2_uint64_t) =
                    _2819;
                _2820 = *(&mut _2176 as *mut l_struct_struct_OC_vint4 as *mut uint32_t);
                _2821 = *(&mut _2177 as *mut l_struct_struct_OC_vint4 as *mut uint32_t);
                _2822 = *(&raw mut *((*(&mut _2176 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _2823 = *(&raw mut *((*(&mut _2177 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _2824 = *(&raw mut *((*(&mut _2176 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _2825 = *(&raw mut *((*(&mut _2177 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _2826 = *(&raw mut *((*(&mut _2176 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _2827 = *(&raw mut *((*(&mut _2177 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _2170 = &mut _2175 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void;
                _2171 = llvm_add_u32(_2820, _2821);
                _2172 = llvm_add_u32(_2822, _2823);
                _2173 = llvm_add_u32(_2824, _2825);
                _2174 = llvm_add_u32(_2826, _2827);
                _2828 = _2170;
                _2829 = _2171;
                *(_2828 as *mut uint32_t) = _2829;
                _2830 = _2172;
                *(&raw mut *((*(_2828 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _2830;
                _2831 = _2173;
                *(&raw mut *((*(_2828 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _2831;
                _2832 = _2174;
                *(&raw mut *((*(_2828 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _2832;
                _2833 = *(&mut _2175 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_2_uint64_t);
                *(&mut _2271 as *mut l_struct_struct_OC_vint4 as *mut crate::l_array_2_uint64_t) =
                    _2833;
                _2834 = _2269;
                _2835 = memcpy(
                    _2834,
                    &mut _2271 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void,
                    16,
                );
                _2836 = _2269;
                _2837 = _2596;
                _2838 = _2612;
                _2538 = &raw mut *(_2837 as *mut core::ffi::c_float)
                    .offset(_2838 as uint64_t as int64_t as isize)
                    as *mut core::ffi::c_float as *mut core::ffi::c_void;
                _2839 = _2538;
                _2415 = &mut _2537 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _2416 = _2839;
                _2840 = _2415;
                _2841 = _2416;
                _2842 = *(_2841 as *mut core::ffi::c_float);
                *(_2840 as *mut core::ffi::c_float) = _2842;
                _2843 = _2416;
                _2844 = *(&raw mut *(_2843 as *mut core::ffi::c_float)
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                *(&raw mut *((*(_2840 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _2844;
                _2845 = _2416;
                _2846 = *(&raw mut *(_2845 as *mut core::ffi::c_float)
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                *(&raw mut *((*(_2840 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _2846;
                _2847 = _2416;
                _2848 = *(&raw mut *(_2847 as *mut core::ffi::c_float)
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                *(&raw mut *((*(_2840 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _2848;
                _2849 = _2537;
                *(&mut _2617.field0 as *mut crate::l_array_4_float) = _2849.field0;
                _2850 = memcpy(
                    &mut _2619 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _2609 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _2851 = memcpy(
                    &mut _2621 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _2609 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _2852 = memcpy(
                    &mut _2622 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _2617 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _2853 = memcpy(
                    &mut _2623 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void,
                    &mut _2613 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void,
                    16,
                );
                _2854 = *(&mut _2621.field0 as *mut crate::l_array_4_float);
                _2855 = *(&mut _2622.field0 as *mut crate::l_array_4_float);
                _2856 = (*(&mut _2623.field0 as *mut crate::l_array_4_uint32_t
                    as *mut C2RustUnnamed_20))
                    .data;
                *(&mut _2547 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _2854;
                *(&mut _2548 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _2855;
                *(&mut _2549 as *mut l_struct_struct_OC_vmask4 as *mut crate::l_array_2_uint64_t) =
                    _2856;
                _2857 = *(&mut _2549 as *mut l_struct_struct_OC_vmask4 as *mut uint32_t);
                if _2857 & 2147483648 as core::ffi::c_uint != 0 as core::ffi::c_uint {
                    _2858 =
                        *(&mut _2548 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _2860__PHI_TEMPORARY = _2858;
                } else {
                    _2859 =
                        *(&mut _2547 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _2860__PHI_TEMPORARY = _2859;
                }
                _2860 = _2860__PHI_TEMPORARY;
                _2861 = *(&raw mut *((*(&mut _2549 as *mut l_struct_struct_OC_vmask4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                if _2861 & 2147483648 as core::ffi::c_uint != 0 as core::ffi::c_uint {
                    _2862 = *(&raw mut *((*(&mut _2548 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2864__PHI_TEMPORARY = _2862;
                } else {
                    _2863 = *(&raw mut *((*(&mut _2547 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2864__PHI_TEMPORARY = _2863;
                }
                _2864 = _2864__PHI_TEMPORARY;
                _2865 = *(&raw mut *((*(&mut _2549 as *mut l_struct_struct_OC_vmask4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                if _2865 & 2147483648 as core::ffi::c_uint != 0 as core::ffi::c_uint {
                    _2866 = *(&raw mut *((*(&mut _2548 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2868__PHI_TEMPORARY = _2866;
                } else {
                    _2867 = *(&raw mut *((*(&mut _2547 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2868__PHI_TEMPORARY = _2867;
                }
                _2868 = _2868__PHI_TEMPORARY;
                _2869 = *(&raw mut *((*(&mut _2549 as *mut l_struct_struct_OC_vmask4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                if _2869 & 2147483648 as core::ffi::c_uint != 0 as core::ffi::c_uint {
                    _2870 = *(&raw mut *((*(&mut _2548 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2872__PHI_TEMPORARY = _2870;
                } else {
                    _2871 = *(&raw mut *((*(&mut _2547 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2872__PHI_TEMPORARY = _2871;
                }
                _2872 = _2872__PHI_TEMPORARY;
                _2541 = &mut _2546 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _2542 = _2860;
                _2543 = _2864;
                _2544 = _2868;
                _2545 = _2872;
                _2873 = _2541;
                _2874 = _2542;
                *(_2873 as *mut core::ffi::c_float) = _2874;
                _2875 = _2543;
                *(&raw mut *((*(_2873 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _2875;
                _2876 = _2544;
                *(&raw mut *((*(_2873 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _2876;
                _2877 = _2545;
                *(&raw mut *((*(_2873 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _2877;
                _2878 = _2546;
                *(&mut _2620.field0 as *mut crate::l_array_4_float) = _2878.field0;
                _2879 = *(&mut _2619.field0 as *mut crate::l_array_4_float);
                _2880 = *(&mut _2620.field0 as *mut crate::l_array_4_float);
                *(&mut _2423 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _2879;
                *(&mut _2424 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _2880;
                _2881 = *(&mut _2423 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _2882 = *(&mut _2424 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                if llvm_fcmp_olt(_2881 as core::ffi::c_double, _2882 as core::ffi::c_double) != 0 {
                    _2883 =
                        *(&mut _2423 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _2885__PHI_TEMPORARY = _2883;
                } else {
                    _2884 =
                        *(&mut _2424 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _2885__PHI_TEMPORARY = _2884;
                }
                _2885 = _2885__PHI_TEMPORARY;
                _2886 = *(&raw mut *((*(&mut _2423 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _2887 = *(&raw mut *((*(&mut _2424 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                if llvm_fcmp_olt(_2886 as core::ffi::c_double, _2887 as core::ffi::c_double) != 0 {
                    _2888 = *(&raw mut *((*(&mut _2423 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2890__PHI_TEMPORARY = _2888;
                } else {
                    _2889 = *(&raw mut *((*(&mut _2424 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2890__PHI_TEMPORARY = _2889;
                }
                _2890 = _2890__PHI_TEMPORARY;
                _2891 = *(&raw mut *((*(&mut _2423 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _2892 = *(&raw mut *((*(&mut _2424 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                if llvm_fcmp_olt(_2891 as core::ffi::c_double, _2892 as core::ffi::c_double) != 0 {
                    _2893 = *(&raw mut *((*(&mut _2423 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2895__PHI_TEMPORARY = _2893;
                } else {
                    _2894 = *(&raw mut *((*(&mut _2424 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2895__PHI_TEMPORARY = _2894;
                }
                _2895 = _2895__PHI_TEMPORARY;
                _2896 = *(&raw mut *((*(&mut _2423 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _2897 = *(&raw mut *((*(&mut _2424 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                if llvm_fcmp_olt(_2896 as core::ffi::c_double, _2897 as core::ffi::c_double) != 0 {
                    _2898 = *(&raw mut *((*(&mut _2423 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2900__PHI_TEMPORARY = _2898;
                } else {
                    _2899 = *(&raw mut *((*(&mut _2424 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2900__PHI_TEMPORARY = _2899;
                }
                _2900 = _2900__PHI_TEMPORARY;
                _2417 = &mut _2422 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _2418 = _2885;
                _2419 = _2890;
                _2420 = _2895;
                _2421 = _2900;
                _2901 = _2417;
                _2902 = _2418;
                *(_2901 as *mut core::ffi::c_float) = _2902;
                _2903 = _2419;
                *(&raw mut *((*(_2901 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _2903;
                _2904 = _2420;
                *(&raw mut *((*(_2901 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _2904;
                _2905 = _2421;
                *(&raw mut *((*(_2901 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _2905;
                _2906 = _2422;
                *(&mut _2618.field0 as *mut crate::l_array_4_float) = _2906.field0;
                _2907 = memcpy(
                    &mut _2609 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _2618 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _2908 = memcpy(
                    &mut _2625 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _2610 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _2909 = memcpy(
                    &mut _2627 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _2610 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _2910 = memcpy(
                    &mut _2628 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _2617 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _2911 = memcpy(
                    &mut _2629 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void,
                    &mut _2613 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void,
                    16,
                );
                _2912 = *(&mut _2627.field0 as *mut crate::l_array_4_float);
                _2913 = *(&mut _2628.field0 as *mut crate::l_array_4_float);
                _2914 = (*(&mut _2629.field0 as *mut crate::l_array_4_uint32_t
                    as *mut C2RustUnnamed_19))
                    .data;
                *(&mut _2556 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _2912;
                *(&mut _2557 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _2913;
                *(&mut _2558 as *mut l_struct_struct_OC_vmask4 as *mut crate::l_array_2_uint64_t) =
                    _2914;
                _2915 = *(&mut _2558 as *mut l_struct_struct_OC_vmask4 as *mut uint32_t);
                if _2915 & 2147483648 as core::ffi::c_uint != 0 as core::ffi::c_uint {
                    _2916 =
                        *(&mut _2557 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _2918__PHI_TEMPORARY = _2916;
                } else {
                    _2917 =
                        *(&mut _2556 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _2918__PHI_TEMPORARY = _2917;
                }
                _2918 = _2918__PHI_TEMPORARY;
                _2919 = *(&raw mut *((*(&mut _2558 as *mut l_struct_struct_OC_vmask4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                if _2919 & 2147483648 as core::ffi::c_uint != 0 as core::ffi::c_uint {
                    _2920 = *(&raw mut *((*(&mut _2557 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2922__PHI_TEMPORARY = _2920;
                } else {
                    _2921 = *(&raw mut *((*(&mut _2556 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2922__PHI_TEMPORARY = _2921;
                }
                _2922 = _2922__PHI_TEMPORARY;
                _2923 = *(&raw mut *((*(&mut _2558 as *mut l_struct_struct_OC_vmask4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                if _2923 & 2147483648 as core::ffi::c_uint != 0 as core::ffi::c_uint {
                    _2924 = *(&raw mut *((*(&mut _2557 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2926__PHI_TEMPORARY = _2924;
                } else {
                    _2925 = *(&raw mut *((*(&mut _2556 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2926__PHI_TEMPORARY = _2925;
                }
                _2926 = _2926__PHI_TEMPORARY;
                _2927 = *(&raw mut *((*(&mut _2558 as *mut l_struct_struct_OC_vmask4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                if _2927 & 2147483648 as core::ffi::c_uint != 0 as core::ffi::c_uint {
                    _2928 = *(&raw mut *((*(&mut _2557 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2930__PHI_TEMPORARY = _2928;
                } else {
                    _2929 = *(&raw mut *((*(&mut _2556 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2930__PHI_TEMPORARY = _2929;
                }
                _2930 = _2930__PHI_TEMPORARY;
                _2550 = &mut _2555 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _2551 = _2918;
                _2552 = _2922;
                _2553 = _2926;
                _2554 = _2930;
                _2931 = _2550;
                _2932 = _2551;
                *(_2931 as *mut core::ffi::c_float) = _2932;
                _2933 = _2552;
                *(&raw mut *((*(_2931 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _2933;
                _2934 = _2553;
                *(&raw mut *((*(_2931 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _2934;
                _2935 = _2554;
                *(&raw mut *((*(_2931 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _2935;
                _2936 = _2555;
                *(&mut _2626.field0 as *mut crate::l_array_4_float) = _2936.field0;
                _2937 = *(&mut _2625.field0 as *mut crate::l_array_4_float);
                _2938 = *(&mut _2626.field0 as *mut crate::l_array_4_float);
                *(&mut _2431 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _2937;
                *(&mut _2432 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _2938;
                _2939 = *(&mut _2431 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _2940 = *(&mut _2432 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                if llvm_fcmp_ogt(_2939 as core::ffi::c_double, _2940 as core::ffi::c_double) != 0 {
                    _2941 =
                        *(&mut _2431 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _2943__PHI_TEMPORARY = _2941;
                } else {
                    _2942 =
                        *(&mut _2432 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _2943__PHI_TEMPORARY = _2942;
                }
                _2943 = _2943__PHI_TEMPORARY;
                _2944 = *(&raw mut *((*(&mut _2431 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _2945 = *(&raw mut *((*(&mut _2432 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                if llvm_fcmp_ogt(_2944 as core::ffi::c_double, _2945 as core::ffi::c_double) != 0 {
                    _2946 = *(&raw mut *((*(&mut _2431 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2948__PHI_TEMPORARY = _2946;
                } else {
                    _2947 = *(&raw mut *((*(&mut _2432 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2948__PHI_TEMPORARY = _2947;
                }
                _2948 = _2948__PHI_TEMPORARY;
                _2949 = *(&raw mut *((*(&mut _2431 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _2950 = *(&raw mut *((*(&mut _2432 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                if llvm_fcmp_ogt(_2949 as core::ffi::c_double, _2950 as core::ffi::c_double) != 0 {
                    _2951 = *(&raw mut *((*(&mut _2431 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2953__PHI_TEMPORARY = _2951;
                } else {
                    _2952 = *(&raw mut *((*(&mut _2432 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2953__PHI_TEMPORARY = _2952;
                }
                _2953 = _2953__PHI_TEMPORARY;
                _2954 = *(&raw mut *((*(&mut _2431 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _2955 = *(&raw mut *((*(&mut _2432 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                if llvm_fcmp_ogt(_2954 as core::ffi::c_double, _2955 as core::ffi::c_double) != 0 {
                    _2956 = *(&raw mut *((*(&mut _2431 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2958__PHI_TEMPORARY = _2956;
                } else {
                    _2957 = *(&raw mut *((*(&mut _2432 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2958__PHI_TEMPORARY = _2957;
                }
                _2958 = _2958__PHI_TEMPORARY;
                _2425 = &mut _2430 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _2426 = _2943;
                _2427 = _2948;
                _2428 = _2953;
                _2429 = _2958;
                _2959 = _2425;
                _2960 = _2426;
                *(_2959 as *mut core::ffi::c_float) = _2960;
                _2961 = _2427;
                *(&raw mut *((*(_2959 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _2961;
                _2962 = _2428;
                *(&raw mut *((*(_2959 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _2962;
                _2963 = _2429;
                *(&raw mut *((*(_2959 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _2963;
                _2964 = _2430;
                *(&mut _2624.field0 as *mut crate::l_array_4_float) = _2964.field0;
                _2965 = memcpy(
                    &mut _2610 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _2624 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _2966 = _2612;
                _2612 = llvm_add_u32(_2966, 4);
            }
            _2967 = memcpy(
                &mut _2631 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                &mut _2609 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                16,
            );
            _2968 = *(&mut _2631.field0 as *mut crate::l_array_4_float);
            *(&mut _2266 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) = _2968;
            _2969 = _ZSt3minIfERKT_S2_S2_(
                &mut _2266 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                &raw mut *((*(&mut _2266 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float as *mut core::ffi::c_void,
            );
            _2970 = *(_2969 as *mut core::ffi::c_float);
            _2267 = _2970;
            _2971 = _ZSt3minIfERKT_S2_S2_(
                &raw mut *((*(&mut _2266 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float as *mut core::ffi::c_void,
                &raw mut *((*(&mut _2266 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float as *mut core::ffi::c_void,
            );
            _2972 = *(_2971 as *mut core::ffi::c_float);
            _2268 = _2972;
            _2973 = _ZSt3minIfERKT_S2_S2_(
                &mut _2267 as *mut core::ffi::c_float as *mut core::ffi::c_void,
                &mut _2268 as *mut core::ffi::c_float as *mut core::ffi::c_void,
            );
            _2974 = *(_2973 as *mut core::ffi::c_float);
            _2263 = &mut _2265 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
            _2264 = _2974;
            _2975 = _2263;
            _2976 = _2264;
            *(_2975 as *mut core::ffi::c_float) = _2976;
            _2977 = _2264;
            *(&raw mut *((*(_2975 as *mut crate::l_array_4_float)).array)
                .as_mut_ptr()
                .offset(1 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float) = _2977;
            _2978 = _2264;
            *(&raw mut *((*(_2975 as *mut crate::l_array_4_float)).array)
                .as_mut_ptr()
                .offset(2 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float) = _2978;
            _2979 = _2264;
            *(&raw mut *((*(_2975 as *mut crate::l_array_4_float)).array)
                .as_mut_ptr()
                .offset(3 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float) = _2979;
            _2980 = _2265;
            *(&mut _2630.field0 as *mut crate::l_array_4_float) = _2980.field0;
            _2981 = memcpy(
                &mut _2609 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                &mut _2630 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                16,
            );
            _2982 = memcpy(
                &mut _2633 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                &mut _2610 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                16,
            );
            _2983 = *(&mut _2633.field0 as *mut crate::l_array_4_float);
            *(&mut _2260 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) = _2983;
            _2984 = _ZSt3maxIfERKT_S2_S2_(
                &mut _2260 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                &raw mut *((*(&mut _2260 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float as *mut core::ffi::c_void,
            );
            _2985 = *(_2984 as *mut core::ffi::c_float);
            _2261 = _2985;
            _2986 = _ZSt3maxIfERKT_S2_S2_(
                &raw mut *((*(&mut _2260 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float as *mut core::ffi::c_void,
                &raw mut *((*(&mut _2260 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float as *mut core::ffi::c_void,
            );
            _2987 = *(_2986 as *mut core::ffi::c_float);
            _2262 = _2987;
            _2988 = _ZSt3maxIfERKT_S2_S2_(
                &mut _2261 as *mut core::ffi::c_float as *mut core::ffi::c_void,
                &mut _2262 as *mut core::ffi::c_float as *mut core::ffi::c_void,
            );
            _2989 = *(_2988 as *mut core::ffi::c_float);
            _2257 = &mut _2259 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
            _2258 = _2989;
            _2990 = _2257;
            _2991 = _2258;
            *(_2990 as *mut core::ffi::c_float) = _2991;
            _2992 = _2258;
            *(&raw mut *((*(_2990 as *mut crate::l_array_4_float)).array)
                .as_mut_ptr()
                .offset(1 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float) = _2992;
            _2993 = _2258;
            *(&raw mut *((*(_2990 as *mut crate::l_array_4_float)).array)
                .as_mut_ptr()
                .offset(2 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float) = _2993;
            _2994 = _2258;
            *(&raw mut *((*(_2990 as *mut crate::l_array_4_float)).array)
                .as_mut_ptr()
                .offset(3 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float) = _2994;
            _2995 = _2259;
            *(&mut _2632.field0 as *mut crate::l_array_4_float) = _2995.field0;
            _2996 = memcpy(
                &mut _2610 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                &mut _2632 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                16,
            );
            _2634 = 0;
            '__3837: loop {
                _2997 = _2634;
                _2998 = _2597;
                if !(_2997 < _2998) {
                    current_block = 499966189669160781;
                    break;
                }
                _2526 = &mut _2528 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _2527 = 0 as core::ffi::c_int as core::ffi::c_float;
                _2999 = _2526;
                _3000 = _2527;
                *(_2999 as *mut core::ffi::c_float) = _3000;
                _3001 = _2527;
                *(&raw mut *((*(_2999 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3001;
                _3002 = _2527;
                *(&raw mut *((*(_2999 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3002;
                _3003 = _2527;
                *(&raw mut *((*(_2999 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3003;
                _3004 = _2528;
                *(&mut _2635.field0 as *mut crate::l_array_4_float) = _3004.field0;
                _2529 = &mut _2531 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _2530 = 0 as core::ffi::c_int as core::ffi::c_float;
                _3005 = _2529;
                _3006 = _2530;
                *(_3005 as *mut core::ffi::c_float) = _3006;
                _3007 = _2530;
                *(&raw mut *((*(_3005 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3007;
                _3008 = _2530;
                *(&raw mut *((*(_3005 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3008;
                _3009 = _2530;
                *(&raw mut *((*(_3005 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3009;
                _3010 = _2531;
                *(&mut _2636.field0 as *mut crate::l_array_4_float) = _3010.field0;
                _2532 = &mut _2534 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _2533 = 0 as core::ffi::c_int as core::ffi::c_float;
                _3011 = _2532;
                _3012 = _2533;
                *(_3011 as *mut core::ffi::c_float) = _3012;
                _3013 = _2533;
                *(&raw mut *((*(_3011 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3013;
                _3014 = _2533;
                *(&raw mut *((*(_3011 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3014;
                _3015 = _2533;
                *(&raw mut *((*(_3011 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3015;
                _3016 = _2534;
                *(&mut _2637.field0 as *mut crate::l_array_4_float) = _3016.field0;
                _3017 = _2599;
                _3018 = _2634;
                _2540 = &raw mut *(_3017 as *mut core::ffi::c_float)
                    .offset(_3018 as uint64_t as int64_t as isize)
                    as *mut core::ffi::c_float as *mut core::ffi::c_void;
                _3019 = _2540;
                _2413 = &mut _2539 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _2414 = _3019;
                _3020 = _2413;
                _3021 = _2414;
                _3022 = *(_3021 as *mut core::ffi::c_float);
                *(_3020 as *mut core::ffi::c_float) = _3022;
                _3023 = _2414;
                _3024 = *(&raw mut *(_3023 as *mut core::ffi::c_float)
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                *(&raw mut *((*(_3020 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3024;
                _3025 = _2414;
                _3026 = *(&raw mut *(_3025 as *mut core::ffi::c_float)
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                *(&raw mut *((*(_3020 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3026;
                _3027 = _2414;
                _3028 = *(&raw mut *(_3027 as *mut core::ffi::c_float)
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                *(&raw mut *((*(_3020 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3028;
                _3029 = _2539;
                *(&mut _2638.field0 as *mut crate::l_array_4_float) = _3029.field0;
                _3030 = memcpy(
                    &mut _2642 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _2609 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _3031 = memcpy(
                    &mut _2643 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _2605 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _3032 = *(&mut _2642.field0 as *mut crate::l_array_4_float);
                _3033 = *(&mut _2643.field0 as *mut crate::l_array_4_float);
                *(&mut _2447 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _3032;
                *(&mut _2448 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _3033;
                _3034 = *(&mut _2447 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _3035 = *(&mut _2448 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _3036 = *(&raw mut *((*(&mut _2447 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3037 = *(&raw mut *((*(&mut _2448 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3038 = *(&raw mut *((*(&mut _2447 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3039 = *(&raw mut *((*(&mut _2448 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3040 = *(&raw mut *((*(&mut _2447 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3041 = *(&raw mut *((*(&mut _2448 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _2441 = &mut _2446 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _2442 = llvm_fmul_f32(_3034, _3035);
                _2443 = llvm_fmul_f32(_3036, _3037);
                _2444 = llvm_fmul_f32(_3038, _3039);
                _2445 = llvm_fmul_f32(_3040, _3041);
                _3042 = _2441;
                _3043 = _2442;
                *(_3042 as *mut core::ffi::c_float) = _3043;
                _3044 = _2443;
                *(&raw mut *((*(_3042 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3044;
                _3045 = _2444;
                *(&raw mut *((*(_3042 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3045;
                _3046 = _2445;
                *(&raw mut *((*(_3042 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3046;
                _3047 = _2446;
                *(&mut _2641.field0 as *mut crate::l_array_4_float) = _3047.field0;
                _3048 = memcpy(
                    &mut _2644 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _2638 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _3049 = *(&mut _2641.field0 as *mut crate::l_array_4_float);
                _3050 = *(&mut _2644.field0 as *mut crate::l_array_4_float);
                *(&mut _2315 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _3049;
                *(&mut _2316 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _3050;
                _3051 = *(&mut _2315 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _3052 = *(&mut _2316 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _3053 = *(&raw mut *((*(&mut _2315 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3054 = *(&raw mut *((*(&mut _2316 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3055 = *(&raw mut *((*(&mut _2315 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3056 = *(&raw mut *((*(&mut _2316 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3057 = *(&raw mut *((*(&mut _2315 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3058 = *(&raw mut *((*(&mut _2316 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _2309 = &mut _2314 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _2310 = llvm_fsub_f32(_3051, _3052);
                _2311 = llvm_fsub_f32(_3053, _3054);
                _2312 = llvm_fsub_f32(_3055, _3056);
                _2313 = llvm_fsub_f32(_3057, _3058);
                _3059 = _2309;
                _3060 = _2310;
                *(_3059 as *mut core::ffi::c_float) = _3060;
                _3061 = _2311;
                *(&raw mut *((*(_3059 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3061;
                _3062 = _2312;
                *(&raw mut *((*(_3059 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3062;
                _3063 = _2313;
                *(&raw mut *((*(_3059 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3063;
                _3064 = _2314;
                *(&mut _2640.field0 as *mut crate::l_array_4_float) = _3064.field0;
                _3065 = *(&mut _2640.field0 as *mut crate::l_array_4_float);
                *(&mut _2242 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _3065;
                _3066 = fegetround();
                if !(_3066 == 0 as core::ffi::c_uint) {
                    current_block = 13801086798632724361;
                    break;
                }
                _3067 = *(&mut _2242 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _3068 = _ZSt5roundf(_3067);
                _3069 = *(&raw mut *((*(&mut _2242 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3070 = _ZSt5roundf(_3069);
                _3071 = *(&raw mut *((*(&mut _2242 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3072 = _ZSt5roundf(_3071);
                _3073 = *(&raw mut *((*(&mut _2242 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3074 = _ZSt5roundf(_3073);
                _2236 = &mut _2241 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _2237 = _3068;
                _2238 = _3070;
                _2239 = _3072;
                _2240 = _3074;
                _3075 = _2236;
                _3076 = _2237;
                *(_3075 as *mut core::ffi::c_float) = _3076;
                _3077 = _2238;
                *(&raw mut *((*(_3075 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3077;
                _3078 = _2239;
                *(&raw mut *((*(_3075 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3078;
                _3079 = _2240;
                *(&raw mut *((*(_3075 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3079;
                _3080 = _2241;
                *(&mut _2639.field0 as *mut crate::l_array_4_float) = _3080.field0;
                _3081 = memcpy(
                    &mut _2648 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _2610 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _3082 = memcpy(
                    &mut _2649 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _2605 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _3083 = *(&mut _2648.field0 as *mut crate::l_array_4_float);
                _3084 = *(&mut _2649.field0 as *mut crate::l_array_4_float);
                *(&mut _2455 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _3083;
                *(&mut _2456 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _3084;
                _3085 = *(&mut _2455 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _3086 = *(&mut _2456 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _3087 = *(&raw mut *((*(&mut _2455 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3088 = *(&raw mut *((*(&mut _2456 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3089 = *(&raw mut *((*(&mut _2455 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3090 = *(&raw mut *((*(&mut _2456 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3091 = *(&raw mut *((*(&mut _2455 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3092 = *(&raw mut *((*(&mut _2456 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _2449 = &mut _2454 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _2450 = llvm_fmul_f32(_3085, _3086);
                _2451 = llvm_fmul_f32(_3087, _3088);
                _2452 = llvm_fmul_f32(_3089, _3090);
                _2453 = llvm_fmul_f32(_3091, _3092);
                _3093 = _2449;
                _3094 = _2450;
                *(_3093 as *mut core::ffi::c_float) = _3094;
                _3095 = _2451;
                *(&raw mut *((*(_3093 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3095;
                _3096 = _2452;
                *(&raw mut *((*(_3093 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3096;
                _3097 = _2453;
                *(&raw mut *((*(_3093 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3097;
                _3098 = _2454;
                *(&mut _2647.field0 as *mut crate::l_array_4_float) = _3098.field0;
                _3099 = memcpy(
                    &mut _2650 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _2638 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _3100 = *(&mut _2647.field0 as *mut crate::l_array_4_float);
                _3101 = *(&mut _2650.field0 as *mut crate::l_array_4_float);
                *(&mut _2323 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _3100;
                *(&mut _2324 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _3101;
                _3102 = *(&mut _2323 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _3103 = *(&mut _2324 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _3104 = *(&raw mut *((*(&mut _2323 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3105 = *(&raw mut *((*(&mut _2324 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3106 = *(&raw mut *((*(&mut _2323 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3107 = *(&raw mut *((*(&mut _2324 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3108 = *(&raw mut *((*(&mut _2323 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3109 = *(&raw mut *((*(&mut _2324 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _2317 = &mut _2322 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _2318 = llvm_fsub_f32(_3102, _3103);
                _2319 = llvm_fsub_f32(_3104, _3105);
                _2320 = llvm_fsub_f32(_3106, _3107);
                _2321 = llvm_fsub_f32(_3108, _3109);
                _3110 = _2317;
                _3111 = _2318;
                *(_3110 as *mut core::ffi::c_float) = _3111;
                _3112 = _2319;
                *(&raw mut *((*(_3110 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3112;
                _3113 = _2320;
                *(&raw mut *((*(_3110 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3113;
                _3114 = _2321;
                *(&raw mut *((*(_3110 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3114;
                _3115 = _2322;
                *(&mut _2646.field0 as *mut crate::l_array_4_float) = _3115.field0;
                _3116 = *(&mut _2646.field0 as *mut crate::l_array_4_float);
                *(&mut _2249 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _3116;
                _3117 = fegetround();
                if !(_3117 == 0 as core::ffi::c_uint) {
                    current_block = 16207723282426863017;
                    break;
                }
                _3118 = *(&mut _2249 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _3119 = _ZSt5roundf(_3118);
                _3120 = *(&raw mut *((*(&mut _2249 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3121 = _ZSt5roundf(_3120);
                _3122 = *(&raw mut *((*(&mut _2249 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3123 = _ZSt5roundf(_3122);
                _3124 = *(&raw mut *((*(&mut _2249 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3125 = _ZSt5roundf(_3124);
                _2243 = &mut _2248 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _2244 = _3119;
                _2245 = _3121;
                _2246 = _3123;
                _2247 = _3125;
                _3126 = _2243;
                _3127 = _2244;
                *(_3126 as *mut core::ffi::c_float) = _3127;
                _3128 = _2245;
                *(&raw mut *((*(_3126 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3128;
                _3129 = _2246;
                *(&raw mut *((*(_3126 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3129;
                _3130 = _2247;
                *(&raw mut *((*(_3126 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3130;
                _3131 = _2248;
                *(&mut _2645.field0 as *mut crate::l_array_4_float) = _3131.field0;
                _2651 = 0;
                loop {
                    _3132 = _2651;
                    _3133 = _2595;
                    if !(_3132 < _3133) {
                        break;
                    }
                    _3134 = _2596;
                    _3135 = _2651;
                    _2235 = &raw mut *(_3134 as *mut core::ffi::c_float)
                        .offset(_3135 as uint64_t as int64_t as isize)
                        as *mut core::ffi::c_float
                        as *mut core::ffi::c_void;
                    _3136 = _2235;
                    _3137 = *(_3136 as *mut core::ffi::c_float);
                    _2232 = &mut _2234 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                    _2233 = _3137;
                    _3138 = _2232;
                    _3139 = _2233;
                    *(_3138 as *mut core::ffi::c_float) = _3139;
                    _3140 = _2233;
                    *(&raw mut *((*(_3138 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3140;
                    _3141 = _2233;
                    *(&raw mut *((*(_3138 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3141;
                    _3142 = _2233;
                    *(&raw mut *((*(_3138 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3142;
                    _3143 = _2234;
                    *(&mut _2654.field0 as *mut crate::l_array_4_float) = _3143.field0;
                    _3144 = memcpy(
                        &mut _2655 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        &mut _2605 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        16,
                    );
                    _3145 = *(&mut _2654.field0 as *mut crate::l_array_4_float);
                    _3146 = *(&mut _2655.field0 as *mut crate::l_array_4_float);
                    *(&mut _2463 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float) = _3145;
                    *(&mut _2464 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float) = _3146;
                    _3147 =
                        *(&mut _2463 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _3148 =
                        *(&mut _2464 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _3149 = *(&raw mut *((*(&mut _2463 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3150 = *(&raw mut *((*(&mut _2464 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3151 = *(&raw mut *((*(&mut _2463 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3152 = *(&raw mut *((*(&mut _2464 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3153 = *(&raw mut *((*(&mut _2463 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3154 = *(&raw mut *((*(&mut _2464 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2457 = &mut _2462 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                    _2458 = llvm_fmul_f32(_3147, _3148);
                    _2459 = llvm_fmul_f32(_3149, _3150);
                    _2460 = llvm_fmul_f32(_3151, _3152);
                    _2461 = llvm_fmul_f32(_3153, _3154);
                    _3155 = _2457;
                    _3156 = _2458;
                    *(_3155 as *mut core::ffi::c_float) = _3156;
                    _3157 = _2459;
                    *(&raw mut *((*(_3155 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3157;
                    _3158 = _2460;
                    *(&raw mut *((*(_3155 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3158;
                    _3159 = _2461;
                    *(&raw mut *((*(_3155 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3159;
                    _3160 = _2462;
                    *(&mut _2653.field0 as *mut crate::l_array_4_float) = _3160.field0;
                    _3161 = memcpy(
                        &mut _2656 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        &mut _2638 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        16,
                    );
                    _3162 = *(&mut _2653.field0 as *mut crate::l_array_4_float);
                    _3163 = *(&mut _2656.field0 as *mut crate::l_array_4_float);
                    *(&mut _2331 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float) = _3162;
                    *(&mut _2332 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float) = _3163;
                    _3164 =
                        *(&mut _2331 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _3165 =
                        *(&mut _2332 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _3166 = *(&raw mut *((*(&mut _2331 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3167 = *(&raw mut *((*(&mut _2332 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3168 = *(&raw mut *((*(&mut _2331 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3169 = *(&raw mut *((*(&mut _2332 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3170 = *(&raw mut *((*(&mut _2331 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3171 = *(&raw mut *((*(&mut _2332 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2325 = &mut _2330 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                    _2326 = llvm_fsub_f32(_3164, _3165);
                    _2327 = llvm_fsub_f32(_3166, _3167);
                    _2328 = llvm_fsub_f32(_3168, _3169);
                    _2329 = llvm_fsub_f32(_3170, _3171);
                    _3172 = _2325;
                    _3173 = _2326;
                    *(_3172 as *mut core::ffi::c_float) = _3173;
                    _3174 = _2327;
                    *(&raw mut *((*(_3172 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3174;
                    _3175 = _2328;
                    *(&raw mut *((*(_3172 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3175;
                    _3176 = _2329;
                    *(&raw mut *((*(_3172 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3176;
                    _3177 = _2330;
                    *(&mut _2652.field0 as *mut crate::l_array_4_float) = _3177.field0;
                    _3178 = memcpy(
                        &mut _2658 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        &mut _2652 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        16,
                    );
                    _3179 = *(&mut _2658.field0 as *mut crate::l_array_4_float);
                    *(&mut _2256 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float) = _3179;
                    _3180 = fegetround();
                    if !(_3180 == 0 as core::ffi::c_uint) {
                        current_block = 4110532860816618439;
                        break '__3837;
                    }
                    _3181 =
                        *(&mut _2256 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _3182 = _ZSt5roundf(_3181);
                    _3183 = *(&raw mut *((*(&mut _2256 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3184 = _ZSt5roundf(_3183);
                    _3185 = *(&raw mut *((*(&mut _2256 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3186 = _ZSt5roundf(_3185);
                    _3187 = *(&raw mut *((*(&mut _2256 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3188 = _ZSt5roundf(_3187);
                    _2250 = &mut _2255 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                    _2251 = _3182;
                    _2252 = _3184;
                    _2253 = _3186;
                    _2254 = _3188;
                    _3189 = _2250;
                    _3190 = _2251;
                    *(_3189 as *mut core::ffi::c_float) = _3190;
                    _3191 = _2252;
                    *(&raw mut *((*(_3189 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3191;
                    _3192 = _2253;
                    *(&raw mut *((*(_3189 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3192;
                    _3193 = _2254;
                    *(&raw mut *((*(_3189 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3193;
                    _3194 = _2255;
                    *(&mut _2657.field0 as *mut crate::l_array_4_float) = _3194.field0;
                    _3195 = memcpy(
                        &mut _2660 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        &mut _2652 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        16,
                    );
                    _3196 = memcpy(
                        &mut _2661 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        &mut _2657 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        16,
                    );
                    _3197 = *(&mut _2660.field0 as *mut crate::l_array_4_float);
                    _3198 = *(&mut _2661.field0 as *mut crate::l_array_4_float);
                    *(&mut _2339 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float) = _3197;
                    *(&mut _2340 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float) = _3198;
                    _3199 =
                        *(&mut _2339 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _3200 =
                        *(&mut _2340 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _3201 = *(&raw mut *((*(&mut _2339 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3202 = *(&raw mut *((*(&mut _2340 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3203 = *(&raw mut *((*(&mut _2339 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3204 = *(&raw mut *((*(&mut _2340 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3205 = *(&raw mut *((*(&mut _2339 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3206 = *(&raw mut *((*(&mut _2340 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2333 = &mut _2338 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                    _2334 = llvm_fsub_f32(_3199, _3200);
                    _2335 = llvm_fsub_f32(_3201, _3202);
                    _2336 = llvm_fsub_f32(_3203, _3204);
                    _2337 = llvm_fsub_f32(_3205, _3206);
                    _3207 = _2333;
                    _3208 = _2334;
                    *(_3207 as *mut core::ffi::c_float) = _3208;
                    _3209 = _2335;
                    *(&raw mut *((*(_3207 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3209;
                    _3210 = _2336;
                    *(&raw mut *((*(_3207 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3210;
                    _3211 = _2337;
                    *(&raw mut *((*(_3207 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3211;
                    _3212 = _2338;
                    *(&mut _2659.field0 as *mut crate::l_array_4_float) = _3212.field0;
                    _3213 = memcpy(
                        &mut _2663 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        &mut _2659 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        16,
                    );
                    _3214 = memcpy(
                        &mut _2664 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        &mut _2659 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        16,
                    );
                    _3215 = *(&mut _2663.field0 as *mut crate::l_array_4_float);
                    _3216 = *(&mut _2664.field0 as *mut crate::l_array_4_float);
                    *(&mut _2471 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float) = _3215;
                    *(&mut _2472 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float) = _3216;
                    _3217 =
                        *(&mut _2471 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _3218 =
                        *(&mut _2472 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _3219 = *(&raw mut *((*(&mut _2471 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3220 = *(&raw mut *((*(&mut _2472 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3221 = *(&raw mut *((*(&mut _2471 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3222 = *(&raw mut *((*(&mut _2472 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3223 = *(&raw mut *((*(&mut _2471 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3224 = *(&raw mut *((*(&mut _2472 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2465 = &mut _2470 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                    _2466 = llvm_fmul_f32(_3217, _3218);
                    _2467 = llvm_fmul_f32(_3219, _3220);
                    _2468 = llvm_fmul_f32(_3221, _3222);
                    _2469 = llvm_fmul_f32(_3223, _3224);
                    _3225 = _2465;
                    _3226 = _2466;
                    *(_3225 as *mut core::ffi::c_float) = _3226;
                    _3227 = _2467;
                    *(&raw mut *((*(_3225 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3227;
                    _3228 = _2468;
                    *(&raw mut *((*(_3225 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3228;
                    _3229 = _2469;
                    *(&raw mut *((*(_3225 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3229;
                    _3230 = _2470;
                    *(&mut _2662.field0 as *mut crate::l_array_4_float) = _3230.field0;
                    _2521 = &mut _2635 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                    _2522 = &mut _2662 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                    _3231 = _2521;
                    _3232 = memcpy(
                        &mut _2524 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        _3231,
                        16,
                    );
                    _3233 = _2522;
                    _3234 = memcpy(
                        &mut _2525 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        _3233,
                        16,
                    );
                    _3235 = *(&mut _2524 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float);
                    _3236 = *(&mut _2525 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float);
                    *(&mut _2411 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float) = _3235;
                    *(&mut _2412 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float) = _3236;
                    _3237 =
                        *(&mut _2411 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _3238 =
                        *(&mut _2412 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _3239 = *(&raw mut *((*(&mut _2411 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3240 = *(&raw mut *((*(&mut _2412 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3241 = *(&raw mut *((*(&mut _2411 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3242 = *(&raw mut *((*(&mut _2412 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3243 = *(&raw mut *((*(&mut _2411 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3244 = *(&raw mut *((*(&mut _2412 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2405 = &mut _2410 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                    _2406 = llvm_fadd_f32(_3237, _3238);
                    _2407 = llvm_fadd_f32(_3239, _3240);
                    _2408 = llvm_fadd_f32(_3241, _3242);
                    _2409 = llvm_fadd_f32(_3243, _3244);
                    _3245 = _2405;
                    _3246 = _2406;
                    *(_3245 as *mut core::ffi::c_float) = _3246;
                    _3247 = _2407;
                    *(&raw mut *((*(_3245 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3247;
                    _3248 = _2408;
                    *(&raw mut *((*(_3245 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3248;
                    _3249 = _2409;
                    *(&raw mut *((*(_3245 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3249;
                    _3250 = _2410;
                    *(&mut _2523 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float) = _3250.field0;
                    _3251 = _2521;
                    _3252 = memcpy(
                        _3251,
                        &mut _2523 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        16,
                    );
                    _3253 = _2521;
                    _3254 = memcpy(
                        &mut _2666 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        &mut _2657 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        16,
                    );
                    _3255 = memcpy(
                        &mut _2667 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        &mut _2639 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        16,
                    );
                    _3256 = *(&mut _2666.field0 as *mut crate::l_array_4_float);
                    _3257 = *(&mut _2667.field0 as *mut crate::l_array_4_float);
                    *(&mut _2222 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float) = _3256;
                    *(&mut _2223 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float) = _3257;
                    _3258 =
                        *(&mut _2222 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _3259 =
                        *(&mut _2223 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _3260 = *(&raw mut *((*(&mut _2222 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3261 = *(&raw mut *((*(&mut _2223 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3262 = *(&raw mut *((*(&mut _2222 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3263 = *(&raw mut *((*(&mut _2223 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3264 = *(&raw mut *((*(&mut _2222 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3265 = *(&raw mut *((*(&mut _2223 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2216 = &mut _2221 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void;
                    _2217 =
                        llvm_fcmp_oeq(_3258 as core::ffi::c_double, _3259 as core::ffi::c_double)
                            as bool_0;
                    _2218 =
                        llvm_fcmp_oeq(_3260 as core::ffi::c_double, _3261 as core::ffi::c_double)
                            as bool_0;
                    _2219 =
                        llvm_fcmp_oeq(_3262 as core::ffi::c_double, _3263 as core::ffi::c_double)
                            as bool_0;
                    _2220 =
                        llvm_fcmp_oeq(_3264 as core::ffi::c_double, _3265 as core::ffi::c_double)
                            as bool_0;
                    _3266 = _2216;
                    _3267 = _2217;
                    _3268 = ((_3267 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0
                        as uint32_t
                        == 0 as core::ffi::c_uint) as core::ffi::c_int
                        as bool_0 as uint64_t;
                    *(_3266 as *mut uint32_t) = llvm_select_u32(
                        ((_3267 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0
                            as uint32_t
                            == 0 as core::ffi::c_uint) as core::ffi::c_int
                            as bool_0,
                        0,
                        -(1 as core::ffi::c_int) as uint32_t,
                    );
                    _3269 = _2218;
                    _3270 = ((_3269 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0
                        as uint32_t
                        == 0 as core::ffi::c_uint) as core::ffi::c_int
                        as bool_0 as uint64_t;
                    *(&raw mut *((*(_3266 as *mut crate::l_array_4_uint32_t)).array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut uint32_t) = llvm_select_u32(
                        ((_3269 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0
                            as uint32_t
                            == 0 as core::ffi::c_uint) as core::ffi::c_int
                            as bool_0,
                        0,
                        -(1 as core::ffi::c_int) as uint32_t,
                    );
                    _3271 = _2219;
                    _3272 = ((_3271 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0
                        as uint32_t
                        == 0 as core::ffi::c_uint) as core::ffi::c_int
                        as bool_0 as uint64_t;
                    *(&raw mut *((*(_3266 as *mut crate::l_array_4_uint32_t)).array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut uint32_t) = llvm_select_u32(
                        ((_3271 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0
                            as uint32_t
                            == 0 as core::ffi::c_uint) as core::ffi::c_int
                            as bool_0,
                        0,
                        -(1 as core::ffi::c_int) as uint32_t,
                    );
                    _3273 = _2220;
                    _3274 = ((_3273 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0
                        as uint32_t
                        == 0 as core::ffi::c_uint) as core::ffi::c_int
                        as bool_0 as uint64_t;
                    *(&raw mut *((*(_3266 as *mut crate::l_array_4_uint32_t)).array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut uint32_t) = llvm_select_u32(
                        ((_3273 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0
                            as uint32_t
                            == 0 as core::ffi::c_uint) as core::ffi::c_int
                            as bool_0,
                        0,
                        -(1 as core::ffi::c_int) as uint32_t,
                    );
                    _3275 = *(&mut _2221 as *mut l_struct_struct_OC_vmask4
                        as *mut crate::l_array_2_uint64_t);
                    (*(&mut _2665.field0 as *mut crate::l_array_4_uint32_t
                        as *mut C2RustUnnamed_18))
                        .data = _3275;
                    _3276 = memcpy(
                        &mut _2670 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        &mut _2636 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        16,
                    );
                    _2587 = &mut _2671 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                    _2588 = 1 as core::ffi::c_int as core::ffi::c_float;
                    _3277 = _2587;
                    _3278 = _2588;
                    *(_3277 as *mut core::ffi::c_float) = _3278;
                    _3279 = _2588;
                    *(&raw mut *((*(_3277 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3279;
                    _3280 = _2588;
                    *(&raw mut *((*(_3277 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3280;
                    _3281 = _2588;
                    *(&raw mut *((*(_3277 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3281;
                    _3282 = *(&mut _2670.field0 as *mut crate::l_array_4_float);
                    _3283 = *(&mut _2671.field0 as *mut crate::l_array_4_float);
                    *(&mut _2371 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float) = _3282;
                    *(&mut _2372 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float) = _3283;
                    _3284 =
                        *(&mut _2371 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _3285 =
                        *(&mut _2372 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _3286 = *(&raw mut *((*(&mut _2371 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3287 = *(&raw mut *((*(&mut _2372 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3288 = *(&raw mut *((*(&mut _2371 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3289 = *(&raw mut *((*(&mut _2372 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3290 = *(&raw mut *((*(&mut _2371 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3291 = *(&raw mut *((*(&mut _2372 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2365 = &mut _2370 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                    _2366 = llvm_fadd_f32(_3284, _3285);
                    _2367 = llvm_fadd_f32(_3286, _3287);
                    _2368 = llvm_fadd_f32(_3288, _3289);
                    _2369 = llvm_fadd_f32(_3290, _3291);
                    _3292 = _2365;
                    _3293 = _2366;
                    *(_3292 as *mut core::ffi::c_float) = _3293;
                    _3294 = _2367;
                    *(&raw mut *((*(_3292 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3294;
                    _3295 = _2368;
                    *(&raw mut *((*(_3292 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3295;
                    _3296 = _2369;
                    *(&raw mut *((*(_3292 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3296;
                    _3297 = _2370;
                    *(&mut _2669.field0 as *mut crate::l_array_4_float) = _3297.field0;
                    _2585 = &mut _2673 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                    _2586 = 2 as core::ffi::c_int as core::ffi::c_float;
                    _3298 = _2585;
                    _3299 = _2586;
                    *(_3298 as *mut core::ffi::c_float) = _3299;
                    _3300 = _2586;
                    *(&raw mut *((*(_3298 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3300;
                    _3301 = _2586;
                    *(&raw mut *((*(_3298 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3301;
                    _3302 = _2586;
                    *(&raw mut *((*(_3298 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3302;
                    _3303 = memcpy(
                        &mut _2674 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        &mut _2659 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        16,
                    );
                    _3304 = *(&mut _2673.field0 as *mut crate::l_array_4_float);
                    _3305 = *(&mut _2674.field0 as *mut crate::l_array_4_float);
                    *(&mut _2479 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float) = _3304;
                    *(&mut _2480 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float) = _3305;
                    _3306 =
                        *(&mut _2479 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _3307 =
                        *(&mut _2480 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _3308 = *(&raw mut *((*(&mut _2479 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3309 = *(&raw mut *((*(&mut _2480 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3310 = *(&raw mut *((*(&mut _2479 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3311 = *(&raw mut *((*(&mut _2480 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3312 = *(&raw mut *((*(&mut _2479 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3313 = *(&raw mut *((*(&mut _2480 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2473 = &mut _2478 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                    _2474 = llvm_fmul_f32(_3306, _3307);
                    _2475 = llvm_fmul_f32(_3308, _3309);
                    _2476 = llvm_fmul_f32(_3310, _3311);
                    _2477 = llvm_fmul_f32(_3312, _3313);
                    _3314 = _2473;
                    _3315 = _2474;
                    *(_3314 as *mut core::ffi::c_float) = _3315;
                    _3316 = _2475;
                    *(&raw mut *((*(_3314 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3316;
                    _3317 = _2476;
                    *(&raw mut *((*(_3314 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3317;
                    _3318 = _2477;
                    *(&raw mut *((*(_3314 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3318;
                    _3319 = _2478;
                    *(&mut _2672.field0 as *mut crate::l_array_4_float) = _3319.field0;
                    _3320 = *(&mut _2669.field0 as *mut crate::l_array_4_float);
                    _3321 = *(&mut _2672.field0 as *mut crate::l_array_4_float);
                    *(&mut _2347 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float) = _3320;
                    *(&mut _2348 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float) = _3321;
                    _3322 =
                        *(&mut _2347 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _3323 =
                        *(&mut _2348 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _3324 = *(&raw mut *((*(&mut _2347 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3325 = *(&raw mut *((*(&mut _2348 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3326 = *(&raw mut *((*(&mut _2347 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3327 = *(&raw mut *((*(&mut _2348 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3328 = *(&raw mut *((*(&mut _2347 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3329 = *(&raw mut *((*(&mut _2348 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2341 = &mut _2346 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                    _2342 = llvm_fsub_f32(_3322, _3323);
                    _2343 = llvm_fsub_f32(_3324, _3325);
                    _2344 = llvm_fsub_f32(_3326, _3327);
                    _2345 = llvm_fsub_f32(_3328, _3329);
                    _3330 = _2341;
                    _3331 = _2342;
                    *(_3330 as *mut core::ffi::c_float) = _3331;
                    _3332 = _2343;
                    *(&raw mut *((*(_3330 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3332;
                    _3333 = _2344;
                    *(&raw mut *((*(_3330 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3333;
                    _3334 = _2345;
                    *(&raw mut *((*(_3330 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3334;
                    _3335 = _2346;
                    *(&mut _2668.field0 as *mut crate::l_array_4_float) = _3335.field0;
                    _3336 = memcpy(
                        &mut _2676 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        &mut _2636 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        16,
                    );
                    _3337 = memcpy(
                        &mut _2677 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        &mut _2668 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        16,
                    );
                    _3338 = memcpy(
                        &mut _2678 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void,
                        &mut _2665 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void,
                        16,
                    );
                    _3339 = *(&mut _2676.field0 as *mut crate::l_array_4_float);
                    _3340 = *(&mut _2677.field0 as *mut crate::l_array_4_float);
                    _3341 = (*(&mut _2678.field0 as *mut crate::l_array_4_uint32_t
                        as *mut C2RustUnnamed_17))
                        .data;
                    *(&mut _2565 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float) = _3339;
                    *(&mut _2566 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float) = _3340;
                    *(&mut _2567 as *mut l_struct_struct_OC_vmask4
                        as *mut crate::l_array_2_uint64_t) = _3341;
                    _3342 = *(&mut _2567 as *mut l_struct_struct_OC_vmask4 as *mut uint32_t);
                    if _3342 & 2147483648 as core::ffi::c_uint != 0 as core::ffi::c_uint {
                        _3343 = *(&mut _2566 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _3345__PHI_TEMPORARY = _3343;
                    } else {
                        _3344 = *(&mut _2565 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _3345__PHI_TEMPORARY = _3344;
                    }
                    _3345 = _3345__PHI_TEMPORARY;
                    _3346 = *(&raw mut *((*(&mut _2567 as *mut l_struct_struct_OC_vmask4
                        as *mut crate::l_array_4_uint32_t))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut uint32_t);
                    if _3346 & 2147483648 as core::ffi::c_uint != 0 as core::ffi::c_uint {
                        _3347 = *(&raw mut *((*(&mut _2566 as *mut l_struct_struct_OC_vfloat4
                            as *mut crate::l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _3349__PHI_TEMPORARY = _3347;
                    } else {
                        _3348 = *(&raw mut *((*(&mut _2565 as *mut l_struct_struct_OC_vfloat4
                            as *mut crate::l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _3349__PHI_TEMPORARY = _3348;
                    }
                    _3349 = _3349__PHI_TEMPORARY;
                    _3350 = *(&raw mut *((*(&mut _2567 as *mut l_struct_struct_OC_vmask4
                        as *mut crate::l_array_4_uint32_t))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut uint32_t);
                    if _3350 & 2147483648 as core::ffi::c_uint != 0 as core::ffi::c_uint {
                        _3351 = *(&raw mut *((*(&mut _2566 as *mut l_struct_struct_OC_vfloat4
                            as *mut crate::l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _3353__PHI_TEMPORARY = _3351;
                    } else {
                        _3352 = *(&raw mut *((*(&mut _2565 as *mut l_struct_struct_OC_vfloat4
                            as *mut crate::l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _3353__PHI_TEMPORARY = _3352;
                    }
                    _3353 = _3353__PHI_TEMPORARY;
                    _3354 = *(&raw mut *((*(&mut _2567 as *mut l_struct_struct_OC_vmask4
                        as *mut crate::l_array_4_uint32_t))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut uint32_t);
                    if _3354 & 2147483648 as core::ffi::c_uint != 0 as core::ffi::c_uint {
                        _3355 = *(&raw mut *((*(&mut _2566 as *mut l_struct_struct_OC_vfloat4
                            as *mut crate::l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _3357__PHI_TEMPORARY = _3355;
                    } else {
                        _3356 = *(&raw mut *((*(&mut _2565 as *mut l_struct_struct_OC_vfloat4
                            as *mut crate::l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _3357__PHI_TEMPORARY = _3356;
                    }
                    _3357 = _3357__PHI_TEMPORARY;
                    _2559 = &mut _2564 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                    _2560 = _3345;
                    _2561 = _3349;
                    _2562 = _3353;
                    _2563 = _3357;
                    _3358 = _2559;
                    _3359 = _2560;
                    *(_3358 as *mut core::ffi::c_float) = _3359;
                    _3360 = _2561;
                    *(&raw mut *((*(_3358 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3360;
                    _3361 = _2562;
                    *(&raw mut *((*(_3358 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3361;
                    _3362 = _2563;
                    *(&raw mut *((*(_3358 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3362;
                    _3363 = _2564;
                    *(&mut _2675.field0 as *mut crate::l_array_4_float) = _3363.field0;
                    _3364 = memcpy(
                        &mut _2636 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        &mut _2675 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        16,
                    );
                    _3365 = memcpy(
                        &mut _2680 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        &mut _2657 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        16,
                    );
                    _3366 = memcpy(
                        &mut _2681 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        &mut _2645 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        16,
                    );
                    _3367 = *(&mut _2680.field0 as *mut crate::l_array_4_float);
                    _3368 = *(&mut _2681.field0 as *mut crate::l_array_4_float);
                    *(&mut _2230 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float) = _3367;
                    *(&mut _2231 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float) = _3368;
                    _3369 =
                        *(&mut _2230 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _3370 =
                        *(&mut _2231 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _3371 = *(&raw mut *((*(&mut _2230 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3372 = *(&raw mut *((*(&mut _2231 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3373 = *(&raw mut *((*(&mut _2230 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3374 = *(&raw mut *((*(&mut _2231 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3375 = *(&raw mut *((*(&mut _2230 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3376 = *(&raw mut *((*(&mut _2231 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2224 = &mut _2229 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void;
                    _2225 =
                        llvm_fcmp_oeq(_3369 as core::ffi::c_double, _3370 as core::ffi::c_double)
                            as bool_0;
                    _2226 =
                        llvm_fcmp_oeq(_3371 as core::ffi::c_double, _3372 as core::ffi::c_double)
                            as bool_0;
                    _2227 =
                        llvm_fcmp_oeq(_3373 as core::ffi::c_double, _3374 as core::ffi::c_double)
                            as bool_0;
                    _2228 =
                        llvm_fcmp_oeq(_3375 as core::ffi::c_double, _3376 as core::ffi::c_double)
                            as bool_0;
                    _3377 = _2224;
                    _3378 = _2225;
                    _3379 = ((_3378 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0
                        as uint32_t
                        == 0 as core::ffi::c_uint) as core::ffi::c_int
                        as bool_0 as uint64_t;
                    *(_3377 as *mut uint32_t) = llvm_select_u32(
                        ((_3378 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0
                            as uint32_t
                            == 0 as core::ffi::c_uint) as core::ffi::c_int
                            as bool_0,
                        0,
                        -(1 as core::ffi::c_int) as uint32_t,
                    );
                    _3380 = _2226;
                    _3381 = ((_3380 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0
                        as uint32_t
                        == 0 as core::ffi::c_uint) as core::ffi::c_int
                        as bool_0 as uint64_t;
                    *(&raw mut *((*(_3377 as *mut crate::l_array_4_uint32_t)).array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut uint32_t) = llvm_select_u32(
                        ((_3380 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0
                            as uint32_t
                            == 0 as core::ffi::c_uint) as core::ffi::c_int
                            as bool_0,
                        0,
                        -(1 as core::ffi::c_int) as uint32_t,
                    );
                    _3382 = _2227;
                    _3383 = ((_3382 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0
                        as uint32_t
                        == 0 as core::ffi::c_uint) as core::ffi::c_int
                        as bool_0 as uint64_t;
                    *(&raw mut *((*(_3377 as *mut crate::l_array_4_uint32_t)).array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut uint32_t) = llvm_select_u32(
                        ((_3382 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0
                            as uint32_t
                            == 0 as core::ffi::c_uint) as core::ffi::c_int
                            as bool_0,
                        0,
                        -(1 as core::ffi::c_int) as uint32_t,
                    );
                    _3384 = _2228;
                    _3385 = ((_3384 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0
                        as uint32_t
                        == 0 as core::ffi::c_uint) as core::ffi::c_int
                        as bool_0 as uint64_t;
                    *(&raw mut *((*(_3377 as *mut crate::l_array_4_uint32_t)).array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut uint32_t) = llvm_select_u32(
                        ((_3384 as core::ffi::c_uint & 1 as core::ffi::c_uint) as bool_0
                            as uint32_t
                            == 0 as core::ffi::c_uint) as core::ffi::c_int
                            as bool_0,
                        0,
                        -(1 as core::ffi::c_int) as uint32_t,
                    );
                    _3386 = *(&mut _2229 as *mut l_struct_struct_OC_vmask4
                        as *mut crate::l_array_2_uint64_t);
                    (*(&mut _2679.field0 as *mut crate::l_array_4_uint32_t
                        as *mut C2RustUnnamed_16))
                        .data = _3386;
                    _3387 = memcpy(
                        &mut _2665 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void,
                        &mut _2679 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void,
                        16,
                    );
                    _3388 = memcpy(
                        &mut _2684 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        &mut _2637 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        16,
                    );
                    _2583 = &mut _2685 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                    _2584 = 1 as core::ffi::c_int as core::ffi::c_float;
                    _3389 = _2583;
                    _3390 = _2584;
                    *(_3389 as *mut core::ffi::c_float) = _3390;
                    _3391 = _2584;
                    *(&raw mut *((*(_3389 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3391;
                    _3392 = _2584;
                    *(&raw mut *((*(_3389 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3392;
                    _3393 = _2584;
                    *(&raw mut *((*(_3389 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3393;
                    _3394 = *(&mut _2684.field0 as *mut crate::l_array_4_float);
                    _3395 = *(&mut _2685.field0 as *mut crate::l_array_4_float);
                    *(&mut _2379 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float) = _3394;
                    *(&mut _2380 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float) = _3395;
                    _3396 =
                        *(&mut _2379 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _3397 =
                        *(&mut _2380 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _3398 = *(&raw mut *((*(&mut _2379 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3399 = *(&raw mut *((*(&mut _2380 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3400 = *(&raw mut *((*(&mut _2379 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3401 = *(&raw mut *((*(&mut _2380 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3402 = *(&raw mut *((*(&mut _2379 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3403 = *(&raw mut *((*(&mut _2380 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2373 = &mut _2378 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                    _2374 = llvm_fadd_f32(_3396, _3397);
                    _2375 = llvm_fadd_f32(_3398, _3399);
                    _2376 = llvm_fadd_f32(_3400, _3401);
                    _2377 = llvm_fadd_f32(_3402, _3403);
                    _3404 = _2373;
                    _3405 = _2374;
                    *(_3404 as *mut core::ffi::c_float) = _3405;
                    _3406 = _2375;
                    *(&raw mut *((*(_3404 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3406;
                    _3407 = _2376;
                    *(&raw mut *((*(_3404 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3407;
                    _3408 = _2377;
                    *(&raw mut *((*(_3404 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3408;
                    _3409 = _2378;
                    *(&mut _2683.field0 as *mut crate::l_array_4_float) = _3409.field0;
                    _2581 = &mut _2687 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                    _2582 = 2 as core::ffi::c_int as core::ffi::c_float;
                    _3410 = _2581;
                    _3411 = _2582;
                    *(_3410 as *mut core::ffi::c_float) = _3411;
                    _3412 = _2582;
                    *(&raw mut *((*(_3410 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3412;
                    _3413 = _2582;
                    *(&raw mut *((*(_3410 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3413;
                    _3414 = _2582;
                    *(&raw mut *((*(_3410 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3414;
                    _3415 = memcpy(
                        &mut _2688 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        &mut _2659 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        16,
                    );
                    _3416 = *(&mut _2687.field0 as *mut crate::l_array_4_float);
                    _3417 = *(&mut _2688.field0 as *mut crate::l_array_4_float);
                    *(&mut _2487 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float) = _3416;
                    *(&mut _2488 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float) = _3417;
                    _3418 =
                        *(&mut _2487 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _3419 =
                        *(&mut _2488 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _3420 = *(&raw mut *((*(&mut _2487 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3421 = *(&raw mut *((*(&mut _2488 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3422 = *(&raw mut *((*(&mut _2487 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3423 = *(&raw mut *((*(&mut _2488 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3424 = *(&raw mut *((*(&mut _2487 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3425 = *(&raw mut *((*(&mut _2488 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2481 = &mut _2486 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                    _2482 = llvm_fmul_f32(_3418, _3419);
                    _2483 = llvm_fmul_f32(_3420, _3421);
                    _2484 = llvm_fmul_f32(_3422, _3423);
                    _2485 = llvm_fmul_f32(_3424, _3425);
                    _3426 = _2481;
                    _3427 = _2482;
                    *(_3426 as *mut core::ffi::c_float) = _3427;
                    _3428 = _2483;
                    *(&raw mut *((*(_3426 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3428;
                    _3429 = _2484;
                    *(&raw mut *((*(_3426 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3429;
                    _3430 = _2485;
                    *(&raw mut *((*(_3426 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3430;
                    _3431 = _2486;
                    *(&mut _2686.field0 as *mut crate::l_array_4_float) = _3431.field0;
                    _3432 = *(&mut _2683.field0 as *mut crate::l_array_4_float);
                    _3433 = *(&mut _2686.field0 as *mut crate::l_array_4_float);
                    *(&mut _2387 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float) = _3432;
                    *(&mut _2388 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float) = _3433;
                    _3434 =
                        *(&mut _2387 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _3435 =
                        *(&mut _2388 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _3436 = *(&raw mut *((*(&mut _2387 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3437 = *(&raw mut *((*(&mut _2388 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3438 = *(&raw mut *((*(&mut _2387 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3439 = *(&raw mut *((*(&mut _2388 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3440 = *(&raw mut *((*(&mut _2387 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _3441 = *(&raw mut *((*(&mut _2388 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2381 = &mut _2386 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                    _2382 = llvm_fadd_f32(_3434, _3435);
                    _2383 = llvm_fadd_f32(_3436, _3437);
                    _2384 = llvm_fadd_f32(_3438, _3439);
                    _2385 = llvm_fadd_f32(_3440, _3441);
                    _3442 = _2381;
                    _3443 = _2382;
                    *(_3442 as *mut core::ffi::c_float) = _3443;
                    _3444 = _2383;
                    *(&raw mut *((*(_3442 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3444;
                    _3445 = _2384;
                    *(&raw mut *((*(_3442 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3445;
                    _3446 = _2385;
                    *(&raw mut *((*(_3442 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3446;
                    _3447 = _2386;
                    *(&mut _2682.field0 as *mut crate::l_array_4_float) = _3447.field0;
                    _3448 = memcpy(
                        &mut _2668 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        &mut _2682 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        16,
                    );
                    _3449 = memcpy(
                        &mut _2690 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        &mut _2637 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        16,
                    );
                    _3450 = memcpy(
                        &mut _2691 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        &mut _2668 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        16,
                    );
                    _3451 = memcpy(
                        &mut _2692 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void,
                        &mut _2665 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void,
                        16,
                    );
                    _3452 = *(&mut _2690.field0 as *mut crate::l_array_4_float);
                    _3453 = *(&mut _2691.field0 as *mut crate::l_array_4_float);
                    _3454 = (*(&mut _2692.field0 as *mut crate::l_array_4_uint32_t
                        as *mut C2RustUnnamed_15))
                        .data;
                    *(&mut _2574 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float) = _3452;
                    *(&mut _2575 as *mut l_struct_struct_OC_vfloat4
                        as *mut crate::l_array_4_float) = _3453;
                    *(&mut _2576 as *mut l_struct_struct_OC_vmask4
                        as *mut crate::l_array_2_uint64_t) = _3454;
                    _3455 = *(&mut _2576 as *mut l_struct_struct_OC_vmask4 as *mut uint32_t);
                    if _3455 & 2147483648 as core::ffi::c_uint != 0 as core::ffi::c_uint {
                        _3456 = *(&mut _2575 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _3458__PHI_TEMPORARY = _3456;
                    } else {
                        _3457 = *(&mut _2574 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _3458__PHI_TEMPORARY = _3457;
                    }
                    _3458 = _3458__PHI_TEMPORARY;
                    _3459 = *(&raw mut *((*(&mut _2576 as *mut l_struct_struct_OC_vmask4
                        as *mut crate::l_array_4_uint32_t))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut uint32_t);
                    if _3459 & 2147483648 as core::ffi::c_uint != 0 as core::ffi::c_uint {
                        _3460 = *(&raw mut *((*(&mut _2575 as *mut l_struct_struct_OC_vfloat4
                            as *mut crate::l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _3462__PHI_TEMPORARY = _3460;
                    } else {
                        _3461 = *(&raw mut *((*(&mut _2574 as *mut l_struct_struct_OC_vfloat4
                            as *mut crate::l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _3462__PHI_TEMPORARY = _3461;
                    }
                    _3462 = _3462__PHI_TEMPORARY;
                    _3463 = *(&raw mut *((*(&mut _2576 as *mut l_struct_struct_OC_vmask4
                        as *mut crate::l_array_4_uint32_t))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut uint32_t);
                    if _3463 & 2147483648 as core::ffi::c_uint != 0 as core::ffi::c_uint {
                        _3464 = *(&raw mut *((*(&mut _2575 as *mut l_struct_struct_OC_vfloat4
                            as *mut crate::l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _3466__PHI_TEMPORARY = _3464;
                    } else {
                        _3465 = *(&raw mut *((*(&mut _2574 as *mut l_struct_struct_OC_vfloat4
                            as *mut crate::l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _3466__PHI_TEMPORARY = _3465;
                    }
                    _3466 = _3466__PHI_TEMPORARY;
                    _3467 = *(&raw mut *((*(&mut _2576 as *mut l_struct_struct_OC_vmask4
                        as *mut crate::l_array_4_uint32_t))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut uint32_t);
                    if _3467 & 2147483648 as core::ffi::c_uint != 0 as core::ffi::c_uint {
                        _3468 = *(&raw mut *((*(&mut _2575 as *mut l_struct_struct_OC_vfloat4
                            as *mut crate::l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _3470__PHI_TEMPORARY = _3468;
                    } else {
                        _3469 = *(&raw mut *((*(&mut _2574 as *mut l_struct_struct_OC_vfloat4
                            as *mut crate::l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _3470__PHI_TEMPORARY = _3469;
                    }
                    _3470 = _3470__PHI_TEMPORARY;
                    _2568 = &mut _2573 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                    _2569 = _3458;
                    _2570 = _3462;
                    _2571 = _3466;
                    _2572 = _3470;
                    _3471 = _2568;
                    _3472 = _2569;
                    *(_3471 as *mut core::ffi::c_float) = _3472;
                    _3473 = _2570;
                    *(&raw mut *((*(_3471 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3473;
                    _3474 = _2571;
                    *(&raw mut *((*(_3471 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3474;
                    _3475 = _2572;
                    *(&raw mut *((*(_3471 as *mut crate::l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _3475;
                    _3476 = _2573;
                    *(&mut _2689.field0 as *mut crate::l_array_4_float) = _3476.field0;
                    _3477 = memcpy(
                        &mut _2637 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        &mut _2689 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        16,
                    );
                    _3478 = _2651;
                    _2651 = llvm_add_u32(_3478, 1);
                }
                _3479 = memcpy(
                    &mut _2696 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _2645 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _3480 = memcpy(
                    &mut _2697 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _2639 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _3481 = *(&mut _2696.field0 as *mut crate::l_array_4_float);
                _3482 = *(&mut _2697.field0 as *mut crate::l_array_4_float);
                *(&mut _2355 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _3481;
                *(&mut _2356 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _3482;
                _3483 = *(&mut _2355 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _3484 = *(&mut _2356 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _3485 = *(&raw mut *((*(&mut _2355 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3486 = *(&raw mut *((*(&mut _2356 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3487 = *(&raw mut *((*(&mut _2355 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3488 = *(&raw mut *((*(&mut _2356 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3489 = *(&raw mut *((*(&mut _2355 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3490 = *(&raw mut *((*(&mut _2356 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _2349 = &mut _2354 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _2350 = llvm_fsub_f32(_3483, _3484);
                _2351 = llvm_fsub_f32(_3485, _3486);
                _2352 = llvm_fsub_f32(_3487, _3488);
                _2353 = llvm_fsub_f32(_3489, _3490);
                _3491 = _2349;
                _3492 = _2350;
                *(_3491 as *mut core::ffi::c_float) = _3492;
                _3493 = _2351;
                *(&raw mut *((*(_3491 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3493;
                _3494 = _2352;
                *(&raw mut *((*(_3491 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3494;
                _3495 = _2353;
                *(&raw mut *((*(_3491 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3495;
                _3496 = _2354;
                *(&mut _2695.field0 as *mut crate::l_array_4_float) = _3496.field0;
                _2579 = &mut _2698 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _2580 = 1 as core::ffi::c_int as core::ffi::c_float;
                _3497 = _2579;
                _3498 = _2580;
                *(_3497 as *mut core::ffi::c_float) = _3498;
                _3499 = _2580;
                *(&raw mut *((*(_3497 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3499;
                _3500 = _2580;
                *(&raw mut *((*(_3497 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3500;
                _3501 = _2580;
                *(&raw mut *((*(_3497 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3501;
                _3502 = *(&mut _2695.field0 as *mut crate::l_array_4_float);
                _3503 = *(&mut _2698.field0 as *mut crate::l_array_4_float);
                *(&mut _2395 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _3502;
                *(&mut _2396 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _3503;
                _3504 = *(&mut _2395 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _3505 = *(&mut _2396 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _3506 = *(&raw mut *((*(&mut _2395 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3507 = *(&raw mut *((*(&mut _2396 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3508 = *(&raw mut *((*(&mut _2395 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3509 = *(&raw mut *((*(&mut _2396 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3510 = *(&raw mut *((*(&mut _2395 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3511 = *(&raw mut *((*(&mut _2396 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _2389 = &mut _2394 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _2390 = llvm_fadd_f32(_3504, _3505);
                _2391 = llvm_fadd_f32(_3506, _3507);
                _2392 = llvm_fadd_f32(_3508, _3509);
                _2393 = llvm_fadd_f32(_3510, _3511);
                _3512 = _2389;
                _3513 = _2390;
                *(_3512 as *mut core::ffi::c_float) = _3513;
                _3514 = _2391;
                *(&raw mut *((*(_3512 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3514;
                _3515 = _2392;
                *(&raw mut *((*(_3512 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3515;
                _3516 = _2393;
                *(&raw mut *((*(_3512 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3516;
                _3517 = _2394;
                *(&mut _2694.field0 as *mut crate::l_array_4_float) = _3517.field0;
                _3518 = *(&mut _2694.field0 as *mut crate::l_array_4_float);
                *(&mut _2215 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _3518;
                _3519 = *(&mut _2215 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _3520 = *(&raw mut *((*(&mut _2215 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3521 = *(&raw mut *((*(&mut _2215 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3522 = *(&raw mut *((*(&mut _2215 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _2209 = &mut _2214 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void;
                _2210 = _3519 as int32_t as uint32_t;
                _2211 = _3520 as int32_t as uint32_t;
                _2212 = _3521 as int32_t as uint32_t;
                _2213 = _3522 as int32_t as uint32_t;
                _3523 = _2209;
                _3524 = _2210;
                *(_3523 as *mut uint32_t) = _3524;
                _3525 = _2211;
                *(&raw mut *((*(_3523 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _3525;
                _3526 = _2212;
                *(&raw mut *((*(_3523 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _3526;
                _3527 = _2213;
                *(&raw mut *((*(_3523 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _3527;
                _3528 = *(&mut _2214 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_2_uint64_t);
                (*(&mut _2693.field0 as *mut crate::l_array_4_uint32_t as *mut C2RustUnnamed_14))
                    .data = _3528;
                _3529 = memcpy(
                    &mut _2700 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void,
                    &mut _2693 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void,
                    16,
                );
                _3530 = _2598;
                _2303 = &mut _2701 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void;
                _2304 = llvm_add_u32(_3530, 3);
                _3531 = _2303;
                _3532 = _2304;
                *(_3531 as *mut uint32_t) = _3532;
                _3533 = _2304;
                *(&raw mut *((*(_3531 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _3533;
                _3534 = _2304;
                *(&raw mut *((*(_3531 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _3534;
                _3535 = _2304;
                *(&raw mut *((*(_3531 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _3535;
                _3536 = (*(&mut _2700.field0 as *mut crate::l_array_4_uint32_t
                    as *mut C2RustUnnamed_13))
                    .data;
                _3537 = (*(&mut _2701.field0 as *mut crate::l_array_4_uint32_t
                    as *mut C2RustUnnamed_12))
                    .data;
                *(&mut _2207 as *mut l_struct_struct_OC_vint4 as *mut crate::l_array_2_uint64_t) =
                    _3536;
                *(&mut _2208 as *mut l_struct_struct_OC_vint4 as *mut crate::l_array_2_uint64_t) =
                    _3537;
                _3538 = *(&mut _2207 as *mut l_struct_struct_OC_vint4 as *mut uint32_t);
                _3539 = *(&mut _2208 as *mut l_struct_struct_OC_vint4 as *mut uint32_t);
                if (_3538 as int32_t) < _3539 as int32_t {
                    _3540 = *(&mut _2207 as *mut l_struct_struct_OC_vint4 as *mut uint32_t);
                    _3542__PHI_TEMPORARY = _3540;
                } else {
                    _3541 = *(&mut _2208 as *mut l_struct_struct_OC_vint4 as *mut uint32_t);
                    _3542__PHI_TEMPORARY = _3541;
                }
                _3542 = _3542__PHI_TEMPORARY;
                _3543 = *(&raw mut *((*(&mut _2207 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _3544 = *(&raw mut *((*(&mut _2208 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                if (_3543 as int32_t) < _3544 as int32_t {
                    _3545 = *(&raw mut *((*(&mut _2207 as *mut l_struct_struct_OC_vint4
                        as *mut crate::l_array_4_uint32_t))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut uint32_t);
                    _3547__PHI_TEMPORARY = _3545;
                } else {
                    _3546 = *(&raw mut *((*(&mut _2208 as *mut l_struct_struct_OC_vint4
                        as *mut crate::l_array_4_uint32_t))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut uint32_t);
                    _3547__PHI_TEMPORARY = _3546;
                }
                _3547 = _3547__PHI_TEMPORARY;
                _3548 = *(&raw mut *((*(&mut _2207 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _3549 = *(&raw mut *((*(&mut _2208 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                if (_3548 as int32_t) < _3549 as int32_t {
                    _3550 = *(&raw mut *((*(&mut _2207 as *mut l_struct_struct_OC_vint4
                        as *mut crate::l_array_4_uint32_t))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut uint32_t);
                    _3552__PHI_TEMPORARY = _3550;
                } else {
                    _3551 = *(&raw mut *((*(&mut _2208 as *mut l_struct_struct_OC_vint4
                        as *mut crate::l_array_4_uint32_t))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut uint32_t);
                    _3552__PHI_TEMPORARY = _3551;
                }
                _3552 = _3552__PHI_TEMPORARY;
                _3553 = *(&raw mut *((*(&mut _2207 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _3554 = *(&raw mut *((*(&mut _2208 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                if (_3553 as int32_t) < _3554 as int32_t {
                    _3555 = *(&raw mut *((*(&mut _2207 as *mut l_struct_struct_OC_vint4
                        as *mut crate::l_array_4_uint32_t))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut uint32_t);
                    _3557__PHI_TEMPORARY = _3555;
                } else {
                    _3556 = *(&raw mut *((*(&mut _2208 as *mut l_struct_struct_OC_vint4
                        as *mut crate::l_array_4_uint32_t))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut uint32_t);
                    _3557__PHI_TEMPORARY = _3556;
                }
                _3557 = _3557__PHI_TEMPORARY;
                _2201 = &mut _2206 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void;
                _2202 = _3542;
                _2203 = _3547;
                _2204 = _3552;
                _2205 = _3557;
                _3558 = _2201;
                _3559 = _2202;
                *(_3558 as *mut uint32_t) = _3559;
                _3560 = _2203;
                *(&raw mut *((*(_3558 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _3560;
                _3561 = _2204;
                *(&raw mut *((*(_3558 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _3561;
                _3562 = _2205;
                *(&raw mut *((*(_3558 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _3562;
                _3563 = *(&mut _2206 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_2_uint64_t);
                (*(&mut _2699.field0 as *mut crate::l_array_4_uint32_t as *mut C2RustUnnamed_11))
                    .data = _3563;
                _3564 = memcpy(
                    &mut _2693 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void,
                    &mut _2699 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void,
                    16,
                );
                _3565 = memcpy(
                    &mut _2703 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void,
                    &mut _2693 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void,
                    16,
                );
                _2301 = &mut _2704 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void;
                _2302 = 2;
                _3566 = _2301;
                _3567 = _2302;
                *(_3566 as *mut uint32_t) = _3567;
                _3568 = _2302;
                *(&raw mut *((*(_3566 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _3568;
                _3569 = _2302;
                *(&raw mut *((*(_3566 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _3569;
                _3570 = _2302;
                *(&raw mut *((*(_3566 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _3570;
                _3571 = (*(&mut _2703.field0 as *mut crate::l_array_4_uint32_t
                    as *mut C2RustUnnamed_10))
                    .data;
                _3572 = (*(&mut _2704.field0 as *mut crate::l_array_4_uint32_t
                    as *mut C2RustUnnamed_9))
                    .data;
                *(&mut _2199 as *mut l_struct_struct_OC_vint4 as *mut crate::l_array_2_uint64_t) =
                    _3571;
                *(&mut _2200 as *mut l_struct_struct_OC_vint4 as *mut crate::l_array_2_uint64_t) =
                    _3572;
                _3573 = *(&mut _2199 as *mut l_struct_struct_OC_vint4 as *mut uint32_t);
                _3574 = *(&mut _2200 as *mut l_struct_struct_OC_vint4 as *mut uint32_t);
                if _3573 as int32_t > _3574 as int32_t {
                    _3575 = *(&mut _2199 as *mut l_struct_struct_OC_vint4 as *mut uint32_t);
                    _3577__PHI_TEMPORARY = _3575;
                } else {
                    _3576 = *(&mut _2200 as *mut l_struct_struct_OC_vint4 as *mut uint32_t);
                    _3577__PHI_TEMPORARY = _3576;
                }
                _3577 = _3577__PHI_TEMPORARY;
                _3578 = *(&raw mut *((*(&mut _2199 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _3579 = *(&raw mut *((*(&mut _2200 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                if _3578 as int32_t > _3579 as int32_t {
                    _3580 = *(&raw mut *((*(&mut _2199 as *mut l_struct_struct_OC_vint4
                        as *mut crate::l_array_4_uint32_t))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut uint32_t);
                    _3582__PHI_TEMPORARY = _3580;
                } else {
                    _3581 = *(&raw mut *((*(&mut _2200 as *mut l_struct_struct_OC_vint4
                        as *mut crate::l_array_4_uint32_t))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut uint32_t);
                    _3582__PHI_TEMPORARY = _3581;
                }
                _3582 = _3582__PHI_TEMPORARY;
                _3583 = *(&raw mut *((*(&mut _2199 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _3584 = *(&raw mut *((*(&mut _2200 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                if _3583 as int32_t > _3584 as int32_t {
                    _3585 = *(&raw mut *((*(&mut _2199 as *mut l_struct_struct_OC_vint4
                        as *mut crate::l_array_4_uint32_t))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut uint32_t);
                    _3587__PHI_TEMPORARY = _3585;
                } else {
                    _3586 = *(&raw mut *((*(&mut _2200 as *mut l_struct_struct_OC_vint4
                        as *mut crate::l_array_4_uint32_t))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut uint32_t);
                    _3587__PHI_TEMPORARY = _3586;
                }
                _3587 = _3587__PHI_TEMPORARY;
                _3588 = *(&raw mut *((*(&mut _2199 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _3589 = *(&raw mut *((*(&mut _2200 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                if _3588 as int32_t > _3589 as int32_t {
                    _3590 = *(&raw mut *((*(&mut _2199 as *mut l_struct_struct_OC_vint4
                        as *mut crate::l_array_4_uint32_t))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut uint32_t);
                    _3592__PHI_TEMPORARY = _3590;
                } else {
                    _3591 = *(&raw mut *((*(&mut _2200 as *mut l_struct_struct_OC_vint4
                        as *mut crate::l_array_4_uint32_t))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut uint32_t);
                    _3592__PHI_TEMPORARY = _3591;
                }
                _3592 = _3592__PHI_TEMPORARY;
                _2193 = &mut _2198 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void;
                _2194 = _3577;
                _2195 = _3582;
                _2196 = _3587;
                _2197 = _3592;
                _3593 = _2193;
                _3594 = _2194;
                *(_3593 as *mut uint32_t) = _3594;
                _3595 = _2195;
                *(&raw mut *((*(_3593 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _3595;
                _3596 = _2196;
                *(&raw mut *((*(_3593 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _3596;
                _3597 = _2197;
                *(&raw mut *((*(_3593 as *mut crate::l_array_4_uint32_t)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _3597;
                _3598 = *(&mut _2198 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_2_uint64_t);
                (*(&mut _2702.field0 as *mut crate::l_array_4_uint32_t as *mut C2RustUnnamed_8))
                    .data = _3598;
                _3599 = memcpy(
                    &mut _2693 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void,
                    &mut _2702 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void,
                    16,
                );
                _3600 = memcpy(
                    &mut _2705 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _2639 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _3601 = _2600;
                _3602 = _2634;
                _3603 = *(&mut _2705.field0 as *mut crate::l_array_4_float);
                *(&mut _2433 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _3603;
                _2434 = &raw mut *(_3601 as *mut core::ffi::c_float)
                    .offset(_3602 as uint64_t as int64_t as isize)
                    as *mut core::ffi::c_float as *mut core::ffi::c_void;
                _3604 = *(&mut _2433 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _3605 = _2434;
                *(_3605 as *mut core::ffi::c_float) = _3604;
                _3606 = *(&raw mut *((*(&mut _2433 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3607 = _2434;
                *(&raw mut *(_3607 as *mut core::ffi::c_float)
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3606;
                _3608 = *(&raw mut *((*(&mut _2433 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3609 = _2434;
                *(&raw mut *(_3609 as *mut core::ffi::c_float)
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3608;
                _3610 = *(&raw mut *((*(&mut _2433 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3611 = _2434;
                *(&raw mut *(_3611 as *mut core::ffi::c_float)
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3610;
                _3612 = memcpy(
                    &mut _2706 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void,
                    &mut _2693 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void,
                    16,
                );
                _3613 = _2601;
                _3614 = _2634;
                _3615 = (*(&mut _2706.field0 as *mut crate::l_array_4_uint32_t
                    as *mut C2RustUnnamed_7))
                    .data;
                *(&mut _2535 as *mut l_struct_struct_OC_vint4 as *mut crate::l_array_2_uint64_t) =
                    _3615;
                _2536 = &raw mut *(_3613 as *mut uint32_t)
                    .offset(_3614 as uint64_t as int64_t as isize)
                    as *mut uint32_t as *mut core::ffi::c_void;
                _3616 = *(&mut _2535 as *mut l_struct_struct_OC_vint4 as *mut uint32_t);
                _3617 = _2536;
                *(_3617 as *mut uint32_t) = _3616;
                _3618 = *(&raw mut *((*(&mut _2535 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _3619 = _2536;
                *(&raw mut *(_3619 as *mut uint32_t)
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _3618;
                _3620 = *(&raw mut *((*(&mut _2535 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _3621 = _2536;
                *(&raw mut *(_3621 as *mut uint32_t)
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _3620;
                _3622 = *(&raw mut *((*(&mut _2535 as *mut l_struct_struct_OC_vint4
                    as *mut crate::l_array_4_uint32_t))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t);
                _3623 = _2536;
                *(&raw mut *(_3623 as *mut uint32_t)
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut uint32_t) = _3622;
                _3624 = memcpy(
                    &mut _2708 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _2605 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _3625 = *(&mut _2708.field0 as *mut crate::l_array_4_float);
                *(&mut _2189 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _3625;
                _2190 = 1 as core::ffi::c_int as core::ffi::c_float;
                _3626 = _2190;
                _2186 = &mut _2191 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _2187 = _3626;
                _3627 = _2186;
                _3628 = _2187;
                *(_3627 as *mut core::ffi::c_float) = _3628;
                _3629 = _2187;
                *(&raw mut *((*(_3627 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3629;
                _3630 = _2187;
                *(&raw mut *((*(_3627 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3630;
                _3631 = _2187;
                *(&raw mut *((*(_3627 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3631;
                _3632 = memcpy(
                    &mut _2192 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _2189 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _3633 =
                    *(&mut _2191 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                _3634 =
                    *(&mut _2192 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float);
                *(&mut _2184 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _3633;
                *(&mut _2185 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _3634;
                _3635 = *(&mut _2184 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _3636 = *(&mut _2185 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _3637 = *(&raw mut *((*(&mut _2184 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3638 = *(&raw mut *((*(&mut _2185 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3639 = *(&raw mut *((*(&mut _2184 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3640 = *(&raw mut *((*(&mut _2185 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3641 = *(&raw mut *((*(&mut _2184 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3642 = *(&raw mut *((*(&mut _2185 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _2178 = &mut _2183 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _2179 = llvm_fdiv_f32(_3635, _3636);
                _2180 = llvm_fdiv_f32(_3637, _3638);
                _2181 = llvm_fdiv_f32(_3639, _3640);
                _2182 = llvm_fdiv_f32(_3641, _3642);
                _3643 = _2178;
                _3644 = _2179;
                *(_3643 as *mut core::ffi::c_float) = _3644;
                _3645 = _2180;
                *(&raw mut *((*(_3643 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3645;
                _3646 = _2181;
                *(&raw mut *((*(_3643 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3646;
                _3647 = _2182;
                *(&raw mut *((*(_3643 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3647;
                _3648 = _2183;
                *(&mut _2188 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _3648.field0;
                _3649 = _2188;
                *(&mut _2707.field0 as *mut crate::l_array_4_float) = _3649.field0;
                _3650 = memcpy(
                    &mut _2710 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _2707 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _3651 = memcpy(
                    &mut _2711 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _2707 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _3652 = *(&mut _2710.field0 as *mut crate::l_array_4_float);
                _3653 = *(&mut _2711.field0 as *mut crate::l_array_4_float);
                *(&mut _2495 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _3652;
                *(&mut _2496 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _3653;
                _3654 = *(&mut _2495 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _3655 = *(&mut _2496 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _3656 = *(&raw mut *((*(&mut _2495 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3657 = *(&raw mut *((*(&mut _2496 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3658 = *(&raw mut *((*(&mut _2495 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3659 = *(&raw mut *((*(&mut _2496 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3660 = *(&raw mut *((*(&mut _2495 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3661 = *(&raw mut *((*(&mut _2496 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _2489 = &mut _2494 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _2490 = llvm_fmul_f32(_3654, _3655);
                _2491 = llvm_fmul_f32(_3656, _3657);
                _2492 = llvm_fmul_f32(_3658, _3659);
                _2493 = llvm_fmul_f32(_3660, _3661);
                _3662 = _2489;
                _3663 = _2490;
                *(_3662 as *mut core::ffi::c_float) = _3663;
                _3664 = _2491;
                *(&raw mut *((*(_3662 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3664;
                _3665 = _2492;
                *(&raw mut *((*(_3662 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3665;
                _3666 = _2493;
                *(&raw mut *((*(_3662 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3666;
                _3667 = _2494;
                *(&mut _2709.field0 as *mut crate::l_array_4_float) = _3667.field0;
                _3668 = memcpy(
                    &mut _2713 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _2635 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _3669 = memcpy(
                    &mut _2714 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _2709 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _3670 = *(&mut _2713.field0 as *mut crate::l_array_4_float);
                _3671 = *(&mut _2714.field0 as *mut crate::l_array_4_float);
                *(&mut _2503 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _3670;
                *(&mut _2504 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _3671;
                _3672 = *(&mut _2503 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _3673 = *(&mut _2504 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _3674 = *(&raw mut *((*(&mut _2503 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3675 = *(&raw mut *((*(&mut _2504 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3676 = *(&raw mut *((*(&mut _2503 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3677 = *(&raw mut *((*(&mut _2504 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3678 = *(&raw mut *((*(&mut _2503 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3679 = *(&raw mut *((*(&mut _2504 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _2497 = &mut _2502 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _2498 = llvm_fmul_f32(_3672, _3673);
                _2499 = llvm_fmul_f32(_3674, _3675);
                _2500 = llvm_fmul_f32(_3676, _3677);
                _2501 = llvm_fmul_f32(_3678, _3679);
                _3680 = _2497;
                _3681 = _2498;
                *(_3680 as *mut core::ffi::c_float) = _3681;
                _3682 = _2499;
                *(&raw mut *((*(_3680 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3682;
                _3683 = _2500;
                *(&raw mut *((*(_3680 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3683;
                _3684 = _2501;
                *(&raw mut *((*(_3680 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3684;
                _3685 = _2502;
                *(&mut _2712.field0 as *mut crate::l_array_4_float) = _3685.field0;
                _3686 = _2602;
                _3687 = _2634;
                _3688 = *(&mut _2712.field0 as *mut crate::l_array_4_float);
                *(&mut _2435 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _3688;
                _2436 = &raw mut *(_3686 as *mut core::ffi::c_float)
                    .offset(_3687 as uint64_t as int64_t as isize)
                    as *mut core::ffi::c_float as *mut core::ffi::c_void;
                _3689 = *(&mut _2435 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _3690 = _2436;
                *(_3690 as *mut core::ffi::c_float) = _3689;
                _3691 = *(&raw mut *((*(&mut _2435 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3692 = _2436;
                *(&raw mut *(_3692 as *mut core::ffi::c_float)
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3691;
                _3693 = *(&raw mut *((*(&mut _2435 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3694 = _2436;
                *(&raw mut *(_3694 as *mut core::ffi::c_float)
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3693;
                _3695 = *(&raw mut *((*(&mut _2435 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3696 = _2436;
                *(&raw mut *(_3696 as *mut core::ffi::c_float)
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3695;
                _3697 = memcpy(
                    &mut _2716 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _2636 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _3698 = memcpy(
                    &mut _2717 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _2709 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _3699 = *(&mut _2716.field0 as *mut crate::l_array_4_float);
                _3700 = *(&mut _2717.field0 as *mut crate::l_array_4_float);
                *(&mut _2511 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _3699;
                *(&mut _2512 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _3700;
                _3701 = *(&mut _2511 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _3702 = *(&mut _2512 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _3703 = *(&raw mut *((*(&mut _2511 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3704 = *(&raw mut *((*(&mut _2512 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3705 = *(&raw mut *((*(&mut _2511 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3706 = *(&raw mut *((*(&mut _2512 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3707 = *(&raw mut *((*(&mut _2511 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3708 = *(&raw mut *((*(&mut _2512 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _2505 = &mut _2510 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _2506 = llvm_fmul_f32(_3701, _3702);
                _2507 = llvm_fmul_f32(_3703, _3704);
                _2508 = llvm_fmul_f32(_3705, _3706);
                _2509 = llvm_fmul_f32(_3707, _3708);
                _3709 = _2505;
                _3710 = _2506;
                *(_3709 as *mut core::ffi::c_float) = _3710;
                _3711 = _2507;
                *(&raw mut *((*(_3709 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3711;
                _3712 = _2508;
                *(&raw mut *((*(_3709 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3712;
                _3713 = _2509;
                *(&raw mut *((*(_3709 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3713;
                _3714 = _2510;
                *(&mut _2715.field0 as *mut crate::l_array_4_float) = _3714.field0;
                _3715 = _2603;
                _3716 = _2634;
                _3717 = *(&mut _2715.field0 as *mut crate::l_array_4_float);
                *(&mut _2437 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _3717;
                _2438 = &raw mut *(_3715 as *mut core::ffi::c_float)
                    .offset(_3716 as uint64_t as int64_t as isize)
                    as *mut core::ffi::c_float as *mut core::ffi::c_void;
                _3718 = *(&mut _2437 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _3719 = _2438;
                *(_3719 as *mut core::ffi::c_float) = _3718;
                _3720 = *(&raw mut *((*(&mut _2437 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3721 = _2438;
                *(&raw mut *(_3721 as *mut core::ffi::c_float)
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3720;
                _3722 = *(&raw mut *((*(&mut _2437 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3723 = _2438;
                *(&raw mut *(_3723 as *mut core::ffi::c_float)
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3722;
                _3724 = *(&raw mut *((*(&mut _2437 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3725 = _2438;
                *(&raw mut *(_3725 as *mut core::ffi::c_float)
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3724;
                _3726 = memcpy(
                    &mut _2719 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _2637 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _3727 = memcpy(
                    &mut _2720 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _2709 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _3728 = *(&mut _2719.field0 as *mut crate::l_array_4_float);
                _3729 = *(&mut _2720.field0 as *mut crate::l_array_4_float);
                *(&mut _2519 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _3728;
                *(&mut _2520 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _3729;
                _3730 = *(&mut _2519 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _3731 = *(&mut _2520 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _3732 = *(&raw mut *((*(&mut _2519 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3733 = *(&raw mut *((*(&mut _2520 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3734 = *(&raw mut *((*(&mut _2519 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3735 = *(&raw mut *((*(&mut _2520 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3736 = *(&raw mut *((*(&mut _2519 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3737 = *(&raw mut *((*(&mut _2520 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _2513 = &mut _2518 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _2514 = llvm_fmul_f32(_3730, _3731);
                _2515 = llvm_fmul_f32(_3732, _3733);
                _2516 = llvm_fmul_f32(_3734, _3735);
                _2517 = llvm_fmul_f32(_3736, _3737);
                _3738 = _2513;
                _3739 = _2514;
                *(_3738 as *mut core::ffi::c_float) = _3739;
                _3740 = _2515;
                *(&raw mut *((*(_3738 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3740;
                _3741 = _2516;
                *(&raw mut *((*(_3738 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3741;
                _3742 = _2517;
                *(&raw mut *((*(_3738 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3742;
                _3743 = _2518;
                *(&mut _2718.field0 as *mut crate::l_array_4_float) = _3743.field0;
                _3744 = _2604;
                _3745 = _2634;
                _3746 = *(&mut _2718.field0 as *mut crate::l_array_4_float);
                *(&mut _2439 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _3746;
                _2440 = &raw mut *(_3744 as *mut core::ffi::c_float)
                    .offset(_3745 as uint64_t as int64_t as isize)
                    as *mut core::ffi::c_float as *mut core::ffi::c_void;
                _3747 = *(&mut _2439 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _3748 = _2440;
                *(_3748 as *mut core::ffi::c_float) = _3747;
                _3749 = *(&raw mut *((*(&mut _2439 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3750 = _2440;
                *(&raw mut *(_3750 as *mut core::ffi::c_float)
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3749;
                _3751 = *(&raw mut *((*(&mut _2439 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3752 = _2440;
                *(&raw mut *(_3752 as *mut core::ffi::c_float)
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3751;
                _3753 = *(&raw mut *((*(&mut _2439 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3754 = _2440;
                *(&raw mut *(_3754 as *mut core::ffi::c_float)
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3753;
                _3755 = memcpy(
                    &mut _2722 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _2605 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _2577 = &mut _2723 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _2578 = 4 as core::ffi::c_int as core::ffi::c_float;
                _3756 = _2577;
                _3757 = _2578;
                *(_3756 as *mut core::ffi::c_float) = _3757;
                _3758 = _2578;
                *(&raw mut *((*(_3756 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3758;
                _3759 = _2578;
                *(&raw mut *((*(_3756 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3759;
                _3760 = _2578;
                *(&raw mut *((*(_3756 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3760;
                _3761 = *(&mut _2722.field0 as *mut crate::l_array_4_float);
                _3762 = *(&mut _2723.field0 as *mut crate::l_array_4_float);
                *(&mut _2403 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _3761;
                *(&mut _2404 as *mut l_struct_struct_OC_vfloat4 as *mut crate::l_array_4_float) =
                    _3762;
                _3763 = *(&mut _2403 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _3764 = *(&mut _2404 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _3765 = *(&raw mut *((*(&mut _2403 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3766 = *(&raw mut *((*(&mut _2404 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3767 = *(&raw mut *((*(&mut _2403 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3768 = *(&raw mut *((*(&mut _2404 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3769 = *(&raw mut *((*(&mut _2403 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _3770 = *(&raw mut *((*(&mut _2404 as *mut l_struct_struct_OC_vfloat4
                    as *mut crate::l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float);
                _2397 = &mut _2402 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _2398 = llvm_fadd_f32(_3763, _3764);
                _2399 = llvm_fadd_f32(_3765, _3766);
                _2400 = llvm_fadd_f32(_3767, _3768);
                _2401 = llvm_fadd_f32(_3769, _3770);
                _3771 = _2397;
                _3772 = _2398;
                *(_3771 as *mut core::ffi::c_float) = _3772;
                _3773 = _2399;
                *(&raw mut *((*(_3771 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3773;
                _3774 = _2400;
                *(&raw mut *((*(_3771 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3774;
                _3775 = _2401;
                *(&raw mut *((*(_3771 as *mut crate::l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _3775;
                _3776 = _2402;
                *(&mut _2721.field0 as *mut crate::l_array_4_float) = _3776.field0;
                _3777 = memcpy(
                    &mut _2605 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _2721 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16,
                );
                _3778 = _2634;
                _2634 = llvm_add_u32(_3778, 4);
            }
            match current_block {
                499966189669160781 => return,
                16207723282426863017 => {
                    __assert_fail(
                        &_OC_str_OC_8 as *const crate::l_array_34_uint8_t as *mut core::ffi::c_void,
                        &_OC_str_OC_9 as *const crate::l_array_54_uint8_t as *mut core::ffi::c_void,
                        869,
                        &__PRETTY_FUNCTION___OC__Z5round7vfloat4 as *const crate::l_array_23_uint8_t
                            as *mut core::ffi::c_void,
                    );
                }
                13801086798632724361 => {
                    __assert_fail(
                        &_OC_str_OC_8 as *const crate::l_array_34_uint8_t as *mut core::ffi::c_void,
                        &_OC_str_OC_9 as *const crate::l_array_54_uint8_t as *mut core::ffi::c_void,
                        869,
                        &__PRETTY_FUNCTION___OC__Z5round7vfloat4 as *const crate::l_array_23_uint8_t
                            as *mut core::ffi::c_void,
                    );
                }
                _ => {
                    __assert_fail(
                        &_OC_str_OC_8 as *const crate::l_array_34_uint8_t as *mut core::ffi::c_void,
                        &_OC_str_OC_9 as *const crate::l_array_54_uint8_t as *mut core::ffi::c_void,
                        869,
                        &__PRETTY_FUNCTION___OC__Z5round7vfloat4 as *const crate::l_array_23_uint8_t
                            as *mut core::ffi::c_void,
                    );
                }
            }
        } else {
            __assert_fail(
                &_OC_str_OC_6 as *const crate::l_array_22_uint8_t as *mut core::ffi::c_void,
                &_OC_str_OC_1 as *const crate::l_array_51_uint8_t as *mut core::ffi::c_void,
                169,
                &__PRETTY_FUNCTION___OC__ZL33compute_lowest_and_highest_weightjPKfjjS0_PfPiS1_S1_S1_
                    as *const crate::l_array_154_uint8_t as *mut core::ffi::c_void,
            );
        }
    } else {
        __assert_fail(
            &_OC_str_OC_7 as *const crate::l_array_17_uint8_t as *mut core::ffi::c_void,
            &_OC_str_OC_1 as *const crate::l_array_51_uint8_t as *mut core::ffi::c_void,
            168,
            &__PRETTY_FUNCTION___OC__ZL33compute_lowest_and_highest_weightjPKfjjS0_PfPiS1_S1_S1_
                as *const crate::l_array_154_uint8_t as *mut core::ffi::c_void,
        );
    };
}
#[inline(never)]
unsafe extern "C" fn _ZN4astcL3maxIiEET_S1_S1_(
    mut _3899: uint32_t,
    mut _3900: uint32_t,
) -> uint32_t {
    let mut _3901: uint32_t = 0;
    let mut _3902: uint32_t = 0;
    let mut _3903: uint32_t = 0;
    let mut _3904: uint32_t = 0;
    let mut _3905: uint32_t = 0;
    let mut _3906: uint32_t = 0;
    let mut _3907: uint32_t = 0;
    let mut _3907__PHI_TEMPORARY: uint32_t = 0;
    _3901 = _3899;
    _3902 = _3900;
    _3903 = _3901;
    _3904 = _3902;
    if _3903 as int32_t > _3904 as int32_t {
        _3905 = _3901;
        _3907__PHI_TEMPORARY = _3905;
    } else {
        _3906 = _3902;
        _3907__PHI_TEMPORARY = _3906;
    }
    _3907 = _3907__PHI_TEMPORARY;
    return _3907;
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn _ZSt3absf(mut _3911: core::ffi::c_float) -> core::ffi::c_float {
    let mut _3912: core::ffi::c_float = 0.;
    let mut _3913: core::ffi::c_float = 0.;
    let mut _3914: core::ffi::c_float = 0.;
    _3912 = _3911;
    _3913 = _3912;
    _3914 = llvm_OC_fabs_OC_f32(_3913);
    return _3914;
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn _ZSt3minIfERKT_S2_S2_(
    mut _3915: *mut core::ffi::c_void,
    mut _3916: *mut core::ffi::c_void,
) -> *mut core::ffi::c_void {
    let mut _3917: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3918: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3919: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3920: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3921: core::ffi::c_float = 0.;
    let mut _3922: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3923: core::ffi::c_float = 0.;
    let mut _3924: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3925: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3926: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    _3918 = _3915;
    _3919 = _3916;
    _3920 = _3919;
    _3921 = *(_3920 as *mut core::ffi::c_float);
    _3922 = _3918;
    _3923 = *(_3922 as *mut core::ffi::c_float);
    if llvm_fcmp_olt(_3921 as core::ffi::c_double, _3923 as core::ffi::c_double) != 0 {
        _3924 = _3919;
        _3917 = _3924;
    } else {
        _3925 = _3918;
        _3917 = _3925;
    }
    _3926 = _3917;
    return _3926;
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn _ZSt3maxIfERKT_S2_S2_(
    mut _3930: *mut core::ffi::c_void,
    mut _3931: *mut core::ffi::c_void,
) -> *mut core::ffi::c_void {
    let mut _3932: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3933: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3934: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3935: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3936: core::ffi::c_float = 0.;
    let mut _3937: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3938: core::ffi::c_float = 0.;
    let mut _3939: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3940: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _3941: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    _3933 = _3930;
    _3934 = _3931;
    _3935 = _3933;
    _3936 = *(_3935 as *mut core::ffi::c_float);
    _3937 = _3934;
    _3938 = *(_3937 as *mut core::ffi::c_float);
    if llvm_fcmp_olt(_3936 as core::ffi::c_double, _3938 as core::ffi::c_double) != 0 {
        _3939 = _3934;
        _3932 = _3939;
    } else {
        _3940 = _3933;
        _3932 = _3940;
    }
    _3941 = _3932;
    return _3941;
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn _ZSt5roundf(mut _3945: core::ffi::c_float) -> core::ffi::c_float {
    let mut _3946: core::ffi::c_float = 0.;
    let mut _3947: core::ffi::c_float = 0.;
    let mut _3948: core::ffi::c_float = 0.;
    _3946 = _3945;
    _3947 = _3946;
    _3948 = roundf(_3947);
    return _3948;
}
