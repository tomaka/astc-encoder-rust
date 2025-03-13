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
pub struct l_array_4_float {
    pub array: [core::ffi::c_float; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_struct_struct_OC_vfloat4 {
    pub field0: l_array_4_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_struct_struct_OC_partition_metrics {
    pub field0: l_struct_struct_OC_vfloat4,
    pub field1: l_struct_struct_OC_vfloat4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_struct_struct_OC_line4 {
    pub field0: l_struct_struct_OC_vfloat4,
    pub field1: l_struct_struct_OC_vfloat4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_struct_struct_OC_processed_line4 {
    pub field0: l_struct_struct_OC_vfloat4,
    pub field1: l_struct_struct_OC_vfloat4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_struct_struct_OC_line3 {
    pub field0: l_struct_struct_OC_vfloat4,
    pub field1: l_struct_struct_OC_vfloat4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_struct_struct_OC_processed_line3 {
    pub field0: l_struct_struct_OC_vfloat4,
    pub field1: l_struct_struct_OC_vfloat4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_struct_struct_OC_partition_lines3 {
    pub field0: l_struct_struct_OC_line3,
    pub field1: l_struct_struct_OC_line3,
    pub field2: l_struct_struct_OC_processed_line3,
    pub field3: l_struct_struct_OC_processed_line3,
    pub field4: core::ffi::c_float,
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
    pub field6: l_array_216_uint8_t,
    pub field7: l_array_4_struct_AC_l_array_216_uint8_t,
    pub field8: l_array_4_struct_AC_l_array_216_uint8_t,
    pub field9: l_array_4_struct_AC_l_array_216_float,
    pub field10: l_array_64_uint8_t,
    pub field11: l_array_216_struct_AC_l_array_64_uint8_t,
    pub field12: l_array_216_struct_AC_l_array_64_float,
    pub field13: l_array_216_struct_AC_l_array_64_float,
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
    pub field2: l_array_4_uint8_t,
    pub field3: l_array_216_uint8_t,
    pub field4: l_array_4_struct_AC_l_array_216_uint8_t,
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
    pub field11: l_array_4_uint32_t,
    pub field12: l_array_4_uint32_t,
    pub field13: l_array_87_struct_AC_l_struct_struct_OC_decimation_mode,
    pub field14: l_array_87_struct_AC_l_struct_struct_OC_decimation_info,
    pub field15: l_array_2048_uint16_t,
    pub field16: l_array_2048_struct_AC_l_struct_struct_OC_block_mode,
    pub field17: l_array_3073_struct_AC_l_struct_struct_OC_partition_info,
    pub field18: l_array_3_struct_AC_l_array_1024_uint16_t,
    pub field19: l_array_64_uint8_t,
    pub field20: l_array_1024_struct_AC_l_array_2_uint64_t,
    pub field21: l_array_1024_struct_AC_l_array_3_uint64_t,
    pub field22: l_array_1024_struct_AC_l_array_4_uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_struct_struct_OC_vmask4 {
    pub field0: l_array_4_uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_struct_struct_OC_vint4 {
    pub field0: l_array_4_uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_219_float {
    pub array: [core::ffi::c_float; 219],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_struct_struct_OC_image_block {
    pub field0: l_array_219_float,
    pub field1: l_array_219_float,
    pub field2: l_array_219_float,
    pub field3: l_array_219_float,
    pub field4: uint8_t,
    pub field5: l_struct_struct_OC_vfloat4,
    pub field6: l_struct_struct_OC_vfloat4,
    pub field7: l_struct_struct_OC_vfloat4,
    pub field8: l_struct_struct_OC_vfloat4,
    pub field9: l_struct_struct_OC_vfloat4,
    pub field10: uint8_t,
    pub field11: uint8_t,
    pub field12: l_array_216_uint8_t,
    pub field13: l_array_216_uint8_t,
    pub field14: uint32_t,
    pub field15: uint32_t,
    pub field16: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_20_uint8_t {
    pub array: [uint8_t; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_61_uint8_t {
    pub array: [uint8_t; 61],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_154_uint8_t {
    pub array: [uint8_t; 154],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_27_uint8_t {
    pub array: [uint8_t; 27],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_22_uint8_t {
    pub array: [uint8_t; 22],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_125_uint8_t {
    pub array: [uint8_t; 125],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_16_uint8_t {
    pub array: [uint8_t; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_77_uint8_t {
    pub array: [uint8_t; 77],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_9_float {
    pub array: [core::ffi::c_float; 9],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_96_uint8_t {
    pub array: [uint8_t; 96],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_17_uint8_t {
    pub array: [uint8_t; 17],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_109_uint8_t {
    pub array: [uint8_t; 109],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_45_uint8_t {
    pub array: [uint8_t; 45],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_37_uint8_t {
    pub array: [uint8_t; 37],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_23_uint8_t {
    pub array: [uint8_t; 23],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_110_uint8_t {
    pub array: [uint8_t; 110],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_107_uint8_t {
    pub array: [uint8_t; 107],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_102_uint8_t {
    pub array: [uint8_t; 102],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_15_uint8_t {
    pub array: [uint8_t; 15],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_79_uint8_t {
    pub array: [uint8_t; 79],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_8_float {
    pub array: [core::ffi::c_float; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_8_uint32_t {
    pub array: [uint32_t; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_4_struct_AC_l_struct_struct_OC_partition_metrics {
    pub array: [l_struct_struct_OC_partition_metrics; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_4_struct_AC_l_struct_struct_OC_line4 {
    pub array: [l_struct_struct_OC_line4; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_4_struct_AC_l_struct_struct_OC_processed_line4 {
    pub array: [l_struct_struct_OC_processed_line4; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_4_struct_AC_l_struct_struct_OC_partition_lines3 {
    pub array: [l_struct_struct_OC_partition_lines3; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_16_uint32_t {
    pub array: [uint32_t; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_16_uint64_t {
    pub array: [uint64_t; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_4_struct_AC_l_struct_struct_OC_vfloat4 {
    pub array: [l_struct_struct_OC_vfloat4; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_1024_uint8_t {
    pub array: [uint8_t; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_64_uint16_t {
    pub array: [uint16_t; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub data: l_array_2_uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub data: l_array_2_uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub data: l_array_2_uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub data: l_array_2_uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub data: l_array_2_uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub data: l_array_2_uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub data: l_array_2_uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub data: l_array_2_uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub data: l_array_2_uint64_t,
}
#[inline(always)]
unsafe extern "C" fn llvm_fcmp_une(
    mut X: core::ffi::c_double,
    mut Y: core::ffi::c_double,
) -> core::ffi::c_int {
    return (X != Y) as core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn llvm_fcmp_oeq(
    mut X: core::ffi::c_double,
    mut Y: core::ffi::c_double,
) -> core::ffi::c_int {
    return (X == Y) as core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn llvm_fcmp_oge(
    mut X: core::ffi::c_double,
    mut Y: core::ffi::c_double,
) -> core::ffi::c_int {
    return (X >= Y) as core::ffi::c_int;
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
static mut _OC_str: l_array_20_uint8_t = unsafe {
    {
        let mut init = l_array_20_uint8_t {
            array: *::core::mem::transmute::<&[u8; 20], &mut [uint8_t; 20]>(
                b"partition_count > 0\0",
            ),
        };
        init
    }
};
static mut _OC_str_OC_1: l_array_61_uint8_t = unsafe {
    {
        let mut init = l_array_61_uint8_t {
            array: *::core::mem::transmute::<&[u8; 61], &mut [uint8_t; 61]>(
                b"/root/astc-encoder/Source/astcenc_find_best_partitioning.cpp\0",
            ),
        };
        init
    }
};
static mut __PRETTY_FUNCTION___OC__Z30find_best_partition_candidatesRK21block_size_descriptorRK11image_blockjjPjj: l_array_154_uint8_t = unsafe {
    {
        let mut init = l_array_154_uint8_t {
            array: *::core::mem::transmute::<
                &[u8; 154],
                &mut [uint8_t; 154],
            >(
                b"unsigned int find_best_partition_candidates(const block_size_descriptor &, const image_block &, unsigned int, unsigned int, unsigned int *, unsigned int)\0",
            ),
        };
        init
    }
};
static mut _OC_str_OC_2: l_array_27_uint8_t = unsafe {
    {
        let mut init = l_array_27_uint8_t {
            array: *::core::mem::transmute::<&[u8; 27], &mut [uint8_t; 27]>(
                b"partition_search_limit > 0\0",
            ),
        };
        init
    }
};
static mut _OC_str_OC_3: l_array_22_uint8_t = unsafe {
    {
        let mut init = l_array_22_uint8_t {
            array: *::core::mem::transmute::<&[u8; 22], &mut [uint8_t; 22]>(
                b"texels_to_process > 0\0",
            ),
        };
        init
    }
};
static mut __PRETTY_FUNCTION___OC__ZL33compute_kmeans_partition_orderingRK21block_size_descriptorRK11image_blockjPt: l_array_125_uint8_t = unsafe {
    {
        let mut init = l_array_125_uint8_t {
            array: *::core::mem::transmute::<
                &[u8; 125],
                &mut [uint8_t; 125],
            >(
                b"unsigned int compute_kmeans_partition_ordering(const block_size_descriptor &, const image_block &, unsigned int, uint16_t *)\0",
            ),
        };
        init
    }
};
static mut _OC_str_OC_4: l_array_16_uint8_t = unsafe {
    {
        let mut init = l_array_16_uint8_t {
            array: *::core::mem::transmute::<&[u8; 16], &mut [uint8_t; 16]>(b"texel_count > 0\0"),
        };
        init
    }
};
static mut __PRETTY_FUNCTION___OC__ZL11kmeans_initRK11image_blockjjP7vfloat4: l_array_77_uint8_t = unsafe {
    {
        let mut init = l_array_77_uint8_t {
            array: *::core::mem::transmute::<&[u8; 77], &mut [uint8_t; 77]>(
                b"void kmeans_init(const image_block &, unsigned int, unsigned int, vfloat4 *)\0",
            ),
        };
        init
    }
};
static mut __const_OC__ZL11kmeans_initRK11image_blockjjP7vfloat4_OC_cluster_cutoffs:
    l_array_9_float = {
    let mut init = l_array_9_float {
        array: [
            0.626219988f64 as core::ffi::c_float,
            0.932770014f64 as core::ffi::c_float,
            0.275454015f64 as core::ffi::c_float,
            0.318558007f64 as core::ffi::c_float,
            0.240113005f64 as core::ffi::c_float,
            0.00918999966f64 as core::ffi::c_float,
            0.347660989f64 as core::ffi::c_float,
            0.731959998f64 as core::ffi::c_float,
            0.156390995f64 as core::ffi::c_float,
        ],
    };
    init
};
static mut __PRETTY_FUNCTION___OC__ZL13kmeans_updateRK11image_blockjjP7vfloat4PKh:
    l_array_96_uint8_t = unsafe {
    {
        let mut init = l_array_96_uint8_t {
            array: *::core::mem::transmute::<
                &[u8; 96],
                &mut [uint8_t; 96],
            >(
                b"void kmeans_update(const image_block &, unsigned int, unsigned int, vfloat4 *, const uint8_t *)\0",
            ),
        };
        init
    }
};
static mut __PRETTY_FUNCTION___OC__ZL13kmeans_assignRK11image_blockjjPK7vfloat4Ph:
    l_array_96_uint8_t = unsafe {
    {
        let mut init = l_array_96_uint8_t {
            array: *::core::mem::transmute::<
                &[u8; 96],
                &mut [uint8_t; 96],
            >(
                b"void kmeans_assign(const image_block &, unsigned int, unsigned int, const vfloat4 *, uint8_t *)\0",
            ),
        };
        init
    }
};
static mut _OC_str_OC_5: l_array_17_uint8_t = unsafe {
    {
        let mut init = l_array_17_uint8_t {
            array: *::core::mem::transmute::<&[u8; 17], &mut [uint8_t; 17]>(b"active_count > 0\0"),
        };
        init
    }
};
static mut __PRETTY_FUNCTION___OC__ZL29count_partition_mismatch_bitsRK21block_size_descriptorjPKmPh: l_array_109_uint8_t = unsafe {
    {
        let mut init = l_array_109_uint8_t {
            array: *::core::mem::transmute::<
                &[u8; 109],
                &mut [uint8_t; 109],
            >(
                b"void count_partition_mismatch_bits(const block_size_descriptor &, unsigned int, const uint64_t *, uint8_t *)\0",
            ),
        };
        init
    }
};
static mut _OC_str_OC_6: l_array_45_uint8_t = unsafe {
    {
        let mut init = l_array_45_uint8_t {
            array: *::core::mem::transmute::<&[u8; 45], &mut [uint8_t; 45]>(
                b"mismatch_counts[i] < BLOCK_MAX_KMEANS_TEXELS\0",
            ),
        };
        init
    }
};
static mut _OC_str_OC_7: l_array_37_uint8_t = unsafe {
    {
        let mut init = l_array_37_uint8_t {
            array: *::core::mem::transmute::<&[u8; 37], &mut [uint8_t; 37]>(
                b"mismatch_counts[i] < bsd.texel_count\0",
            ),
        };
        init
    }
};
static mut _OC_str_OC_8: l_array_23_uint8_t = unsafe {
    {
        let mut init = l_array_23_uint8_t {
            array: *::core::mem::transmute::<&[u8; 23], &mut [uint8_t; 23]>(
                b"partitioning_count > 0\0",
            ),
        };
        init
    }
};
static mut __PRETTY_FUNCTION___OC__ZL39get_partition_ordering_by_mismatch_bitsjjPKhPt:
    l_array_110_uint8_t = unsafe {
    {
        let mut init = l_array_110_uint8_t {
            array: *::core::mem::transmute::<
                &[u8; 110],
                &mut [uint8_t; 110],
            >(
                b"unsigned int get_partition_ordering_by_mismatch_bits(unsigned int, unsigned int, const uint8_t *, uint16_t *)\0",
            ),
        };
        init
    }
};
static mut _OC_str_OC_9: l_array_107_uint8_t = unsafe {
    {
        let mut init = l_array_107_uint8_t {
            array: *::core::mem::transmute::<
                &[u8; 107],
                &mut [uint8_t; 107],
            >(
                b"packed_index != BLOCK_BAD_PARTITIONING && packed_index < this->partitioning_count_all[partition_count - 1]\0",
            ),
        };
        init
    }
};
static mut _OC_str_OC_10: l_array_45_uint8_t = unsafe {
    {
        let mut init = l_array_45_uint8_t {
            array: *::core::mem::transmute::<&[u8; 45], &mut [uint8_t; 45]>(
                b"/root/astc-encoder/Source/astcenc_internal.h\0",
            ),
        };
        init
    }
};
static mut __PRETTY_FUNCTION___OC__ZNK21block_size_descriptor22get_raw_partition_infoEjj:
    l_array_102_uint8_t = unsafe {
    {
        let mut init = l_array_102_uint8_t {
            array: *::core::mem::transmute::<
                &[u8; 102],
                &mut [uint8_t; 102],
            >(
                b"const partition_info &block_size_descriptor::get_raw_partition_info(unsigned int, unsigned int) const\0",
            ),
        };
        init
    }
};
static mut _OC_str_OC_11: l_array_15_uint8_t = unsafe {
    {
        let mut init = l_array_15_uint8_t {
            array: *::core::mem::transmute::<&[u8; 15], &mut [uint8_t; 15]>(b"max_values > 0\0"),
        };
        init
    }
};
static mut __PRETTY_FUNCTION___OC__ZL13insert_resultjfjPfPj: l_array_79_uint8_t = unsafe {
    {
        let mut init = l_array_79_uint8_t {
            array: *::core::mem::transmute::<&[u8; 79], &mut [uint8_t; 79]>(
                b"void insert_result(unsigned int, float, unsigned int, float *, unsigned int *)\0",
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
unsafe extern "C" fn llvm_add_u8(mut a: uint8_t, mut b: uint8_t) -> uint8_t {
    let mut r: uint8_t = (a as core::ffi::c_int + b as libc::c_int) as uint8_t;
    return r;
}
#[inline(always)]
unsafe extern "C" fn llvm_add_u16(mut a: uint16_t, mut b: uint16_t) -> uint16_t {
    let mut r: uint16_t = (a as core::ffi::c_int + b as libc::c_int) as uint16_t;
    return r;
}
#[inline(always)]
unsafe extern "C" fn llvm_add_u32(mut a: uint32_t, mut b: uint32_t) -> uint32_t {
    let mut r: uint32_t = a.wrapping_add(b);
    return r;
}
#[inline(always)]
unsafe extern "C" fn llvm_add_u64(mut a: uint64_t, mut b: uint64_t) -> uint64_t {
    let mut r: uint64_t = a.wrapping_add(b);
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
unsafe extern "C" fn llvm_sub_u64(mut a: uint64_t, mut b: uint64_t) -> uint64_t {
    let mut r: uint64_t = a.wrapping_sub(b);
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
unsafe extern "C" fn llvm_mul_u64(mut a: uint64_t, mut b: uint64_t) -> uint64_t {
    let mut r: uint64_t = a * b;
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
unsafe extern "C" fn llvm_sdiv_u32(mut a: int32_t, mut b: int32_t) -> uint32_t {
    let mut r: uint32_t = (a / b) as uint32_t;
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
unsafe extern "C" fn llvm_urem_u32(mut a: uint32_t, mut b: uint32_t) -> uint32_t {
    let mut r: uint32_t = a % b;
    return r;
}
#[inline(always)]
unsafe extern "C" fn llvm_lshr_u64(mut a: uint64_t, mut b: uint64_t) -> uint64_t {
    let mut r: uint64_t = a >> b;
    return r;
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn _Z30find_best_partition_candidatesRK21block_size_descriptorRK11image_blockjjPjj(
    mut _1: *mut core::ffi::c_void,
    mut _2: *mut core::ffi::c_void,
    mut _3: uint32_t,
    mut _4: uint32_t,
    mut _5: *mut core::ffi::c_void,
    mut _6: uint32_t,
) -> uint32_t {
    let mut _7: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _8: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _9: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _10: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _11: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _12: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _13: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _14: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _15: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _16: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _17: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _18: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _19: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _20: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _21: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _22: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _23: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _24: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _25: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _26: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _27: core::ffi::c_float = 0.;
    let mut _28: core::ffi::c_float = 0.;
    let mut _29: core::ffi::c_float = 0.;
    let mut _30: core::ffi::c_float = 0.;
    let mut _31: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _32: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _33: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _34: core::ffi::c_float = 0.;
    let mut _35: core::ffi::c_float = 0.;
    let mut _36: core::ffi::c_float = 0.;
    let mut _37: core::ffi::c_float = 0.;
    let mut _38: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _39: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _40: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _41: core::ffi::c_float = 0.;
    let mut _42: core::ffi::c_float = 0.;
    let mut _43: core::ffi::c_float = 0.;
    let mut _44: core::ffi::c_float = 0.;
    let mut _45: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _46: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _47: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _48: core::ffi::c_float = 0.;
    let mut _49: core::ffi::c_float = 0.;
    let mut _50: core::ffi::c_float = 0.;
    let mut _51: core::ffi::c_float = 0.;
    let mut _52: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _53: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _54: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _55: core::ffi::c_float = 0.;
    let mut _56: core::ffi::c_float = 0.;
    let mut _57: core::ffi::c_float = 0.;
    let mut _58: core::ffi::c_float = 0.;
    let mut _59: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _60: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _61: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _62: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _63: core::ffi::c_float = 0.;
    let mut _64: core::ffi::c_float = 0.;
    let mut _65: core::ffi::c_float = 0.;
    let mut _66: core::ffi::c_float = 0.;
    let mut _67: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _68: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _69: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _70: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _71: core::ffi::c_float = 0.;
    let mut _72: core::ffi::c_float = 0.;
    let mut _73: core::ffi::c_float = 0.;
    let mut _74: core::ffi::c_float = 0.;
    let mut _75: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _76: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _77: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _78: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _79: core::ffi::c_float = 0.;
    let mut _80: core::ffi::c_float = 0.;
    let mut _81: core::ffi::c_float = 0.;
    let mut _82: core::ffi::c_float = 0.;
    let mut _83: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _84: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _85: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _86: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _87: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _88: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _89: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _90: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _91: core::ffi::c_float = 0.;
    let mut _92: core::ffi::c_float = 0.;
    let mut _93: core::ffi::c_float = 0.;
    let mut _94: core::ffi::c_float = 0.;
    let mut _95: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _96: core::ffi::c_float = 0.;
    let mut _97: core::ffi::c_float = 0.;
    let mut _98: core::ffi::c_float = 0.;
    let mut _99: core::ffi::c_float = 0.;
    let mut _100: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _101: core::ffi::c_float = 0.;
    let mut _102: core::ffi::c_float = 0.;
    let mut _103: core::ffi::c_float = 0.;
    let mut _104: core::ffi::c_float = 0.;
    let mut _105: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _106: core::ffi::c_float = 0.;
    let mut _107: core::ffi::c_float = 0.;
    let mut _108: core::ffi::c_float = 0.;
    let mut _109: core::ffi::c_float = 0.;
    let mut _110: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _111: core::ffi::c_float = 0.;
    let mut _112: core::ffi::c_float = 0.;
    let mut _113: core::ffi::c_float = 0.;
    let mut _114: core::ffi::c_float = 0.;
    let mut _115: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _116: core::ffi::c_float = 0.;
    let mut _117: core::ffi::c_float = 0.;
    let mut _118: core::ffi::c_float = 0.;
    let mut _119: core::ffi::c_float = 0.;
    let mut _120: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _121: core::ffi::c_float = 0.;
    let mut _122: core::ffi::c_float = 0.;
    let mut _123: core::ffi::c_float = 0.;
    let mut _124: core::ffi::c_float = 0.;
    let mut _125: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _126: core::ffi::c_float = 0.;
    let mut _127: core::ffi::c_float = 0.;
    let mut _128: core::ffi::c_float = 0.;
    let mut _129: core::ffi::c_float = 0.;
    let mut _130: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _131: core::ffi::c_float = 0.;
    let mut _132: core::ffi::c_float = 0.;
    let mut _133: core::ffi::c_float = 0.;
    let mut _134: core::ffi::c_float = 0.;
    let mut _135: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _136: core::ffi::c_float = 0.;
    let mut _137: core::ffi::c_float = 0.;
    let mut _138: core::ffi::c_float = 0.;
    let mut _139: core::ffi::c_float = 0.;
    let mut _140: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _141: core::ffi::c_float = 0.;
    let mut _142: core::ffi::c_float = 0.;
    let mut _143: core::ffi::c_float = 0.;
    let mut _144: core::ffi::c_float = 0.;
    let mut _145: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _146: core::ffi::c_float = 0.;
    let mut _147: core::ffi::c_float = 0.;
    let mut _148: core::ffi::c_float = 0.;
    let mut _149: core::ffi::c_float = 0.;
    let mut _150: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _151: core::ffi::c_float = 0.;
    let mut _152: core::ffi::c_float = 0.;
    let mut _153: core::ffi::c_float = 0.;
    let mut _154: core::ffi::c_float = 0.;
    let mut _155: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _156: core::ffi::c_float = 0.;
    let mut _157: core::ffi::c_float = 0.;
    let mut _158: core::ffi::c_float = 0.;
    let mut _159: core::ffi::c_float = 0.;
    let mut _160: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _161: core::ffi::c_float = 0.;
    let mut _162: core::ffi::c_float = 0.;
    let mut _163: core::ffi::c_float = 0.;
    let mut _164: core::ffi::c_float = 0.;
    let mut _165: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _166: core::ffi::c_float = 0.;
    let mut _167: core::ffi::c_float = 0.;
    let mut _168: core::ffi::c_float = 0.;
    let mut _169: core::ffi::c_float = 0.;
    let mut _170: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _171: core::ffi::c_float = 0.;
    let mut _172: core::ffi::c_float = 0.;
    let mut _173: core::ffi::c_float = 0.;
    let mut _174: core::ffi::c_float = 0.;
    let mut _175: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _176: core::ffi::c_float = 0.;
    let mut _177: core::ffi::c_float = 0.;
    let mut _178: core::ffi::c_float = 0.;
    let mut _179: core::ffi::c_float = 0.;
    let mut _180: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _181: core::ffi::c_float = 0.;
    let mut _182: core::ffi::c_float = 0.;
    let mut _183: core::ffi::c_float = 0.;
    let mut _184: core::ffi::c_float = 0.;
    let mut _185: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _186: core::ffi::c_float = 0.;
    let mut _187: core::ffi::c_float = 0.;
    let mut _188: core::ffi::c_float = 0.;
    let mut _189: core::ffi::c_float = 0.;
    let mut _190: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _191: core::ffi::c_float = 0.;
    let mut _192: core::ffi::c_float = 0.;
    let mut _193: core::ffi::c_float = 0.;
    let mut _194: core::ffi::c_float = 0.;
    let mut _195: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _196: core::ffi::c_float = 0.;
    let mut _197: core::ffi::c_float = 0.;
    let mut _198: core::ffi::c_float = 0.;
    let mut _199: core::ffi::c_float = 0.;
    let mut _200: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _201: core::ffi::c_float = 0.;
    let mut _202: core::ffi::c_float = 0.;
    let mut _203: core::ffi::c_float = 0.;
    let mut _204: core::ffi::c_float = 0.;
    let mut _205: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _206: core::ffi::c_float = 0.;
    let mut _207: core::ffi::c_float = 0.;
    let mut _208: core::ffi::c_float = 0.;
    let mut _209: core::ffi::c_float = 0.;
    let mut _210: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _211: core::ffi::c_float = 0.;
    let mut _212: core::ffi::c_float = 0.;
    let mut _213: core::ffi::c_float = 0.;
    let mut _214: core::ffi::c_float = 0.;
    let mut _215: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _216: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _217: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _218: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _219: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _220: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _221: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _222: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _223: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _224: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _225: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _226: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _227: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _228: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _229: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _230: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _231: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _232: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _233: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _234: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _235: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _236: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _237: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _238: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _239: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _240: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _241: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _242: core::ffi::c_float = 0.;
    let mut _243: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _244: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _245: core::ffi::c_float = 0.;
    let mut _246: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _247: core::ffi::c_float = 0.;
    let mut _248: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _249: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _250: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _251: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _252: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _253: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _254: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _255: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _256: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _257: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _258: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _259: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _260: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _261: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _262: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _263: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _264: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _265: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _266: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _267: core::ffi::c_float = 0.;
    let mut _268: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _269: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _270: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _271: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _272: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _273: core::ffi::c_float = 0.;
    let mut _274: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _275: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _276: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _277: core::ffi::c_float = 0.;
    let mut _278: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _279: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _280: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _281: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _282: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _283: core::ffi::c_float = 0.;
    let mut _284: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _285: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _286: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _287: core::ffi::c_float = 0.;
    let mut _288: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _289: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _290: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _291: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _292: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _293: core::ffi::c_float = 0.;
    let mut _294: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _295: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _296: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _297: core::ffi::c_float = 0.;
    let mut _298: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _299: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _300: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _301: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _302: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _303: core::ffi::c_float = 0.;
    let mut _304: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _305: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _306: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _307: core::ffi::c_float = 0.;
    let mut _308: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _309: core::ffi::c_float = 0.;
    let mut _310: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _311: core::ffi::c_float = 0.;
    let mut _312: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _313: core::ffi::c_float = 0.;
    let mut _314: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _315: core::ffi::c_float = 0.;
    let mut _316: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _317: core::ffi::c_float = 0.;
    let mut _318: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _319: core::ffi::c_float = 0.;
    let mut _320: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _321: core::ffi::c_float = 0.;
    let mut _322: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _323: core::ffi::c_float = 0.;
    let mut _324: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _325: core::ffi::c_float = 0.;
    let mut _326: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _327: core::ffi::c_float = 0.;
    let mut _328: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _329: core::ffi::c_float = 0.;
    let mut _330: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _331: core::ffi::c_float = 0.;
    let mut _332: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _333: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _334: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _335: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _336: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _337: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _338: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _339: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _340: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _341: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _342: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _343: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _344: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _345: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _346: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _347: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _348: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _349: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _350: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _351: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _352: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _353: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _354: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _355: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _356: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _357: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _358: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _359: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _360: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _361: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _362: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _363: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _364: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _365: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _366: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _367: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _368: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _369: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _370: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _371: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _372: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _373: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _374: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _375: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _376: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _377: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _378: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _379: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _380: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _381: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _382: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _383: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _384: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _385: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _386: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _387: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _388: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _389: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _390: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _391: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _392: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _393: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _394: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _395: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _396: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _397: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _398: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _399: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _400: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _401: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _402: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _403: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _404: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _405: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _406: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _407: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _408: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _409: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _410: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _411: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _412: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _413: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _414: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _415: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _416: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _417: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _418: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _419: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _420: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _421: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _422: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _423: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _424: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _425: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _426: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _427: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _428: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _429: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _430: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _431: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _432: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _433: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _434: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _435: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _436: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _437: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _438: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _439: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _440: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _441: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _442: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _443: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _444: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _445: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _446: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _447: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _448: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _449: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _450: uint32_t = 0;
    let mut _451: uint32_t = 0;
    let mut _452: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _453: uint32_t = 0;
    let mut _454: uint32_t = 0;
    let mut _455: core::ffi::c_float = 0.;
    let mut _456: l_array_1024_uint16_t = l_array_1024_uint16_t { array: [0; 1024] };
    let mut _457: uint32_t = 0;
    let mut _458: uint8_t = 0;
    let mut _459: l_array_8_float = l_array_8_float { array: [0.; 8] };
    let mut _460: l_array_8_uint32_t = l_array_8_uint32_t { array: [0; 8] };
    let mut _461: l_array_8_float = l_array_8_float { array: [0.; 8] };
    let mut _462: l_array_8_uint32_t = l_array_8_uint32_t { array: [0; 8] };
    let mut _463: uint32_t = 0;
    let mut _464: uint32_t = 0;
    let mut _465: uint32_t = 0;
    let mut _466: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _467: l_array_4_struct_AC_l_struct_struct_OC_partition_metrics =
        l_array_4_struct_AC_l_struct_struct_OC_partition_metrics {
            array: [l_struct_struct_OC_partition_metrics {
                field0: l_struct_struct_OC_vfloat4 {
                    field0: l_array_4_float { array: [0.; 4] },
                },
                field1: l_struct_struct_OC_vfloat4 {
                    field0: l_array_4_float { array: [0.; 4] },
                },
            }; 4],
        };
    let mut _468: l_array_4_struct_AC_l_struct_struct_OC_line4 =
        l_array_4_struct_AC_l_struct_struct_OC_line4 {
            array: [l_struct_struct_OC_line4 {
                field0: l_struct_struct_OC_vfloat4 {
                    field0: l_array_4_float { array: [0.; 4] },
                },
                field1: l_struct_struct_OC_vfloat4 {
                    field0: l_array_4_float { array: [0.; 4] },
                },
            }; 4],
        };
    let mut _469: l_array_4_struct_AC_l_struct_struct_OC_line4 =
        l_array_4_struct_AC_l_struct_struct_OC_line4 {
            array: [l_struct_struct_OC_line4 {
                field0: l_struct_struct_OC_vfloat4 {
                    field0: l_array_4_float { array: [0.; 4] },
                },
                field1: l_struct_struct_OC_vfloat4 {
                    field0: l_array_4_float { array: [0.; 4] },
                },
            }; 4],
        };
    let mut _470: l_array_4_struct_AC_l_struct_struct_OC_processed_line4 =
        l_array_4_struct_AC_l_struct_struct_OC_processed_line4 {
            array: [l_struct_struct_OC_processed_line4 {
                field0: l_struct_struct_OC_vfloat4 {
                    field0: l_array_4_float { array: [0.; 4] },
                },
                field1: l_struct_struct_OC_vfloat4 {
                    field0: l_array_4_float { array: [0.; 4] },
                },
            }; 4],
        };
    let mut _471: l_array_4_struct_AC_l_struct_struct_OC_processed_line4 =
        l_array_4_struct_AC_l_struct_struct_OC_processed_line4 {
            array: [l_struct_struct_OC_processed_line4 {
                field0: l_struct_struct_OC_vfloat4 {
                    field0: l_array_4_float { array: [0.; 4] },
                },
                field1: l_struct_struct_OC_vfloat4 {
                    field0: l_array_4_float { array: [0.; 4] },
                },
            }; 4],
        };
    let mut _472: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _473: uint32_t = 0;
    let mut _474: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _475: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _476: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _477: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _478: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _479: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _480: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _481: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _482: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _483: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _484: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _485: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _486: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _487: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _488: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _489: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _490: core::ffi::c_float = 0.;
    let mut _491: core::ffi::c_float = 0.;
    let mut _492: uint32_t = 0;
    let mut _493: core::ffi::c_float = 0.;
    let mut _494: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _495: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _496: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _497: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _498: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _499: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _500: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _501: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _502: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _503: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _504: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _505: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _506: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _507: uint32_t = 0;
    let mut _508: uint32_t = 0;
    let mut _509: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _510: l_array_4_struct_AC_l_struct_struct_OC_partition_metrics =
        l_array_4_struct_AC_l_struct_struct_OC_partition_metrics {
            array: [l_struct_struct_OC_partition_metrics {
                field0: l_struct_struct_OC_vfloat4 {
                    field0: l_array_4_float { array: [0.; 4] },
                },
                field1: l_struct_struct_OC_vfloat4 {
                    field0: l_array_4_float { array: [0.; 4] },
                },
            }; 4],
        };
    let mut _511: l_array_4_struct_AC_l_struct_struct_OC_partition_lines3 =
        l_array_4_struct_AC_l_struct_struct_OC_partition_lines3 {
            array: [l_struct_struct_OC_partition_lines3 {
                field0: l_struct_struct_OC_line3 {
                    field0: l_struct_struct_OC_vfloat4 {
                        field0: l_array_4_float { array: [0.; 4] },
                    },
                    field1: l_struct_struct_OC_vfloat4 {
                        field0: l_array_4_float { array: [0.; 4] },
                    },
                },
                field1: l_struct_struct_OC_line3 {
                    field0: l_struct_struct_OC_vfloat4 {
                        field0: l_array_4_float { array: [0.; 4] },
                    },
                    field1: l_struct_struct_OC_vfloat4 {
                        field0: l_array_4_float { array: [0.; 4] },
                    },
                },
                field2: l_struct_struct_OC_processed_line3 {
                    field0: l_struct_struct_OC_vfloat4 {
                        field0: l_array_4_float { array: [0.; 4] },
                    },
                    field1: l_struct_struct_OC_vfloat4 {
                        field0: l_array_4_float { array: [0.; 4] },
                    },
                },
                field3: l_struct_struct_OC_processed_line3 {
                    field0: l_struct_struct_OC_vfloat4 {
                        field0: l_array_4_float { array: [0.; 4] },
                    },
                    field1: l_struct_struct_OC_vfloat4 {
                        field0: l_array_4_float { array: [0.; 4] },
                    },
                },
                field4: 0.,
            }; 4],
        };
    let mut _512: uint32_t = 0;
    let mut _513: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _514: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _515: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _516: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _517: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _518: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _519: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _520: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _521: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _522: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _523: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _524: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _525: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _526: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _527: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _528: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _529: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _530: core::ffi::c_float = 0.;
    let mut _531: core::ffi::c_float = 0.;
    let mut _532: uint32_t = 0;
    let mut _533: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _534: core::ffi::c_float = 0.;
    let mut _535: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _536: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _537: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _538: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _539: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _540: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _541: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _542: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _543: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _544: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _545: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _546: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _547: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _548: l_array_16_uint32_t = l_array_16_uint32_t { array: [0; 16] };
    let mut _549: uint32_t = 0;
    let mut _550: l_array_16_uint64_t = l_array_16_uint64_t { array: [0; 16] };
    let mut _551: uint32_t = 0;
    let mut _552: uint32_t = 0;
    let mut _553: uint32_t = 0;
    let mut _554: uint32_t = 0;
    let mut _555: uint32_t = 0;
    let mut _556: uint8_t = 0;
    let mut _557: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _558: uint8_t = 0;
    let mut _559: uint32_t = 0;
    let mut _560: uint32_t = 0;
    let mut _561: uint32_t = 0;
    let mut _562: uint32_t = 0;
    let mut _563: uint32_t = 0;
    let mut _564: core::ffi::c_float = 0.;
    let mut _565: core::ffi::c_float = 0.;
    let mut _566: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _567: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _568: uint32_t = 0;
    let mut _569: uint32_t = 0;
    let mut _570: uint32_t = 0;
    let mut _571: uint32_t = 0;
    let mut _572: uint32_t = 0;
    let mut _573: uint32_t = 0;
    let mut _574: uint32_t = 0;
    let mut _575: uint32_t = 0;
    let mut _576: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _577: bool_0 = 0;
    let mut _578: uint32_t = 0;
    let mut _579: uint32_t = 0;
    let mut _580: uint32_t = 0;
    let mut _581: uint32_t = 0;
    let mut _582: uint32_t = 0;
    let mut _583: uint8_t = 0;
    let mut _584: uint32_t = 0;
    let mut _585: uint32_t = 0;
    let mut _586: uint32_t = 0;
    let mut _587: uint16_t = 0;
    let mut _588: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _589: uint32_t = 0;
    let mut _590: uint32_t = 0;
    let mut _591: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _592: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _593: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _594: uint32_t = 0;
    let mut _595: uint32_t = 0;
    let mut _596: uint32_t = 0;
    let mut _597: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _598: uint32_t = 0;
    let mut _599: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _600: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _601: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _602: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _603: core::ffi::c_float = 0.;
    let mut _604: core::ffi::c_float = 0.;
    let mut _605: core::ffi::c_float = 0.;
    let mut _606: core::ffi::c_float = 0.;
    let mut _607: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _608: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _609: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _610: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _611: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _612: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _613: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _614: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _615: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _616: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _617: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _618: core::ffi::c_float = 0.;
    let mut _619: core::ffi::c_float = 0.;
    let mut _620: core::ffi::c_float = 0.;
    let mut _621: core::ffi::c_float = 0.;
    let mut _622: core::ffi::c_float = 0.;
    let mut _623: core::ffi::c_float = 0.;
    let mut _624: core::ffi::c_float = 0.;
    let mut _625: core::ffi::c_float = 0.;
    let mut _626: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _627: core::ffi::c_float = 0.;
    let mut _628: core::ffi::c_float = 0.;
    let mut _629: core::ffi::c_float = 0.;
    let mut _630: core::ffi::c_float = 0.;
    let mut _631: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _632: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _633: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _634: core::ffi::c_float = 0.;
    let mut _635: core::ffi::c_float = 0.;
    let mut _636: core::ffi::c_float = 0.;
    let mut _637: core::ffi::c_float = 0.;
    let mut _638: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _639: core::ffi::c_float = 0.;
    let mut _640: core::ffi::c_float = 0.;
    let mut _641: core::ffi::c_float = 0.;
    let mut _642: core::ffi::c_float = 0.;
    let mut _643: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _644: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _645: core::ffi::c_float = 0.;
    let mut _646: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _647: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _648: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _649: core::ffi::c_float = 0.;
    let mut _650: core::ffi::c_float = 0.;
    let mut _651: core::ffi::c_float = 0.;
    let mut _652: core::ffi::c_float = 0.;
    let mut _653: core::ffi::c_float = 0.;
    let mut _654: core::ffi::c_float = 0.;
    let mut _655: core::ffi::c_float = 0.;
    let mut _656: core::ffi::c_float = 0.;
    let mut _657: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _658: core::ffi::c_float = 0.;
    let mut _659: core::ffi::c_float = 0.;
    let mut _660: core::ffi::c_float = 0.;
    let mut _661: core::ffi::c_float = 0.;
    let mut _662: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _663: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _664: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _665: core::ffi::c_float = 0.;
    let mut _666: core::ffi::c_float = 0.;
    let mut _667: core::ffi::c_float = 0.;
    let mut _668: core::ffi::c_float = 0.;
    let mut _669: core::ffi::c_float = 0.;
    let mut _670: core::ffi::c_float = 0.;
    let mut _671: core::ffi::c_float = 0.;
    let mut _672: core::ffi::c_float = 0.;
    let mut _673: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _674: core::ffi::c_float = 0.;
    let mut _675: core::ffi::c_float = 0.;
    let mut _676: core::ffi::c_float = 0.;
    let mut _677: core::ffi::c_float = 0.;
    let mut _678: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _679: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _680: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _681: uint32_t = 0;
    let mut _682: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _683: uint32_t = 0;
    let mut _684: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _685: uint32_t = 0;
    let mut _686: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _687: uint32_t = 0;
    let mut _688: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _689: uint32_t = 0;
    let mut _690: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _691: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _692: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _693: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _694: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _695: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _696: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _697: core::ffi::c_float = 0.;
    let mut _698: core::ffi::c_float = 0.;
    let mut _699: core::ffi::c_float = 0.;
    let mut _700: core::ffi::c_float = 0.;
    let mut _701: core::ffi::c_float = 0.;
    let mut _702: core::ffi::c_float = 0.;
    let mut _703: core::ffi::c_float = 0.;
    let mut _704: core::ffi::c_float = 0.;
    let mut _705: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _706: core::ffi::c_float = 0.;
    let mut _707: core::ffi::c_float = 0.;
    let mut _708: core::ffi::c_float = 0.;
    let mut _709: core::ffi::c_float = 0.;
    let mut _710: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _711: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _712: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _713: core::ffi::c_float = 0.;
    let mut _714: core::ffi::c_float = 0.;
    let mut _715: core::ffi::c_float = 0.;
    let mut _716: core::ffi::c_float = 0.;
    let mut _717: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _718: core::ffi::c_float = 0.;
    let mut _719: core::ffi::c_float = 0.;
    let mut _720: core::ffi::c_float = 0.;
    let mut _721: core::ffi::c_float = 0.;
    let mut _722: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _723: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _724: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _725: core::ffi::c_float = 0.;
    let mut _726: core::ffi::c_float = 0.;
    let mut _727: core::ffi::c_float = 0.;
    let mut _728: core::ffi::c_float = 0.;
    let mut _729: core::ffi::c_float = 0.;
    let mut _730: core::ffi::c_float = 0.;
    let mut _731: core::ffi::c_float = 0.;
    let mut _732: core::ffi::c_float = 0.;
    let mut _733: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _734: core::ffi::c_float = 0.;
    let mut _735: core::ffi::c_float = 0.;
    let mut _736: core::ffi::c_float = 0.;
    let mut _737: core::ffi::c_float = 0.;
    let mut _738: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _739: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _740: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _741: core::ffi::c_float = 0.;
    let mut _742: core::ffi::c_float = 0.;
    let mut _743: core::ffi::c_float = 0.;
    let mut _744: core::ffi::c_float = 0.;
    let mut _745: core::ffi::c_float = 0.;
    let mut _746: core::ffi::c_float = 0.;
    let mut _747: core::ffi::c_float = 0.;
    let mut _748: core::ffi::c_float = 0.;
    let mut _749: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _750: core::ffi::c_float = 0.;
    let mut _751: core::ffi::c_float = 0.;
    let mut _752: core::ffi::c_float = 0.;
    let mut _753: core::ffi::c_float = 0.;
    let mut _754: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _755: uint32_t = 0;
    let mut _756: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _757: uint32_t = 0;
    let mut _758: uint32_t = 0;
    let mut _759: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _760: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _761: core::ffi::c_float = 0.;
    let mut _762: core::ffi::c_float = 0.;
    let mut _763: core::ffi::c_float = 0.;
    let mut _764: core::ffi::c_float = 0.;
    let mut _765: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _766: uint32_t = 0;
    let mut _767: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _768: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _769: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _770: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _771: core::ffi::c_float = 0.;
    let mut _772: core::ffi::c_float = 0.;
    let mut _773: core::ffi::c_float = 0.;
    let mut _774: core::ffi::c_float = 0.;
    let mut _775: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _776: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _777: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _778: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _779: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _780: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _781: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _782: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _783: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _784: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _785: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _786: core::ffi::c_float = 0.;
    let mut _787: core::ffi::c_float = 0.;
    let mut _788: core::ffi::c_float = 0.;
    let mut _789: core::ffi::c_float = 0.;
    let mut _790: core::ffi::c_float = 0.;
    let mut _791: core::ffi::c_float = 0.;
    let mut _792: core::ffi::c_float = 0.;
    let mut _793: core::ffi::c_float = 0.;
    let mut _794: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _795: core::ffi::c_float = 0.;
    let mut _796: core::ffi::c_float = 0.;
    let mut _797: core::ffi::c_float = 0.;
    let mut _798: core::ffi::c_float = 0.;
    let mut _799: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _800: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _801: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _802: core::ffi::c_float = 0.;
    let mut _803: core::ffi::c_float = 0.;
    let mut _804: core::ffi::c_float = 0.;
    let mut _805: core::ffi::c_float = 0.;
    let mut _806: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _807: core::ffi::c_float = 0.;
    let mut _808: core::ffi::c_float = 0.;
    let mut _809: core::ffi::c_float = 0.;
    let mut _810: core::ffi::c_float = 0.;
    let mut _811: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _812: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _813: core::ffi::c_float = 0.;
    let mut _814: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _815: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _816: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _817: core::ffi::c_float = 0.;
    let mut _818: core::ffi::c_float = 0.;
    let mut _819: core::ffi::c_float = 0.;
    let mut _820: core::ffi::c_float = 0.;
    let mut _821: core::ffi::c_float = 0.;
    let mut _822: core::ffi::c_float = 0.;
    let mut _823: core::ffi::c_float = 0.;
    let mut _824: core::ffi::c_float = 0.;
    let mut _825: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _826: core::ffi::c_float = 0.;
    let mut _827: core::ffi::c_float = 0.;
    let mut _828: core::ffi::c_float = 0.;
    let mut _829: core::ffi::c_float = 0.;
    let mut _830: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _831: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _832: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _833: core::ffi::c_float = 0.;
    let mut _834: core::ffi::c_float = 0.;
    let mut _835: core::ffi::c_float = 0.;
    let mut _836: core::ffi::c_float = 0.;
    let mut _837: core::ffi::c_float = 0.;
    let mut _838: core::ffi::c_float = 0.;
    let mut _839: core::ffi::c_float = 0.;
    let mut _840: core::ffi::c_float = 0.;
    let mut _841: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _842: core::ffi::c_float = 0.;
    let mut _843: core::ffi::c_float = 0.;
    let mut _844: core::ffi::c_float = 0.;
    let mut _845: core::ffi::c_float = 0.;
    let mut _846: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _847: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _848: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _849: uint32_t = 0;
    let mut _850: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _851: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _852: core::ffi::c_float = 0.;
    let mut _853: core::ffi::c_float = 0.;
    let mut _854: core::ffi::c_float = 0.;
    let mut _855: core::ffi::c_float = 0.;
    let mut _856: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _857: uint32_t = 0;
    let mut _858: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _859: uint32_t = 0;
    let mut _860: uint32_t = 0;
    let mut _861: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _862: uint32_t = 0;
    let mut _863: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _864: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _865: uint32_t = 0;
    let mut _866: uint32_t = 0;
    let mut _867: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _868: uint32_t = 0;
    let mut _869: uint8_t = 0;
    let mut _870: core::ffi::c_float = 0.;
    let mut _871: core::ffi::c_float = 0.;
    let mut _872: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _873: core::ffi::c_float = 0.;
    let mut _874: core::ffi::c_float = 0.;
    let mut _875: core::ffi::c_float = 0.;
    let mut _876: core::ffi::c_float = 0.;
    let mut _877: uint32_t = 0;
    let mut _878: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _879: uint32_t = 0;
    let mut _880: core::ffi::c_float = 0.;
    let mut _881: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _882: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _883: core::ffi::c_float = 0.;
    let mut _884: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _885: core::ffi::c_float = 0.;
    let mut _886: core::ffi::c_float = 0.;
    let mut _887: core::ffi::c_float = 0.;
    let mut _888: core::ffi::c_float = 0.;
    let mut _889: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _890: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _891: core::ffi::c_float = 0.;
    let mut _892: core::ffi::c_float = 0.;
    let mut _893: core::ffi::c_float = 0.;
    let mut _894: core::ffi::c_float = 0.;
    let mut _895: core::ffi::c_float = 0.;
    let mut _896: core::ffi::c_float = 0.;
    let mut _897: core::ffi::c_float = 0.;
    let mut _898: core::ffi::c_float = 0.;
    let mut _899: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _900: core::ffi::c_float = 0.;
    let mut _901: core::ffi::c_float = 0.;
    let mut _902: core::ffi::c_float = 0.;
    let mut _903: core::ffi::c_float = 0.;
    let mut _904: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _905: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _906: uint32_t = 0;
    let mut _907: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _908: uint32_t = 0;
    let mut _909: core::ffi::c_float = 0.;
    let mut _910: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _911: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _912: core::ffi::c_float = 0.;
    let mut _913: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _914: core::ffi::c_float = 0.;
    let mut _915: core::ffi::c_float = 0.;
    let mut _916: core::ffi::c_float = 0.;
    let mut _917: core::ffi::c_float = 0.;
    let mut _918: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _919: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _920: core::ffi::c_float = 0.;
    let mut _921: core::ffi::c_float = 0.;
    let mut _922: core::ffi::c_float = 0.;
    let mut _923: core::ffi::c_float = 0.;
    let mut _924: core::ffi::c_float = 0.;
    let mut _925: core::ffi::c_float = 0.;
    let mut _926: core::ffi::c_float = 0.;
    let mut _927: core::ffi::c_float = 0.;
    let mut _928: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _929: core::ffi::c_float = 0.;
    let mut _930: core::ffi::c_float = 0.;
    let mut _931: core::ffi::c_float = 0.;
    let mut _932: core::ffi::c_float = 0.;
    let mut _933: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _934: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _935: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _936: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _937: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _938: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _939: core::ffi::c_float = 0.;
    let mut _940: core::ffi::c_float = 0.;
    let mut _941: core::ffi::c_float = 0.;
    let mut _942: core::ffi::c_float = 0.;
    let mut _943: core::ffi::c_float = 0.;
    let mut _944: core::ffi::c_float = 0.;
    let mut _945: core::ffi::c_float = 0.;
    let mut _946: core::ffi::c_float = 0.;
    let mut _947: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _948: core::ffi::c_float = 0.;
    let mut _949: core::ffi::c_float = 0.;
    let mut _950: core::ffi::c_float = 0.;
    let mut _951: core::ffi::c_float = 0.;
    let mut _952: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _953: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _954: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _955: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _956: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _957: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _958: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _959: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _960: core::ffi::c_float = 0.;
    let mut _961: core::ffi::c_float = 0.;
    let mut _962: core::ffi::c_float = 0.;
    let mut _963: core::ffi::c_float = 0.;
    let mut _964: core::ffi::c_float = 0.;
    let mut _965: core::ffi::c_float = 0.;
    let mut _966: core::ffi::c_float = 0.;
    let mut _967: core::ffi::c_float = 0.;
    let mut _968: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _969: core::ffi::c_float = 0.;
    let mut _970: core::ffi::c_float = 0.;
    let mut _971: core::ffi::c_float = 0.;
    let mut _972: core::ffi::c_float = 0.;
    let mut _973: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _974: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _975: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _976: core::ffi::c_float = 0.;
    let mut _977: core::ffi::c_float = 0.;
    let mut _978: core::ffi::c_float = 0.;
    let mut _979: core::ffi::c_float = 0.;
    let mut _980: core::ffi::c_float = 0.;
    let mut _981: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _982: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _983: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _984: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _985: core::ffi::c_float = 0.;
    let mut _986: core::ffi::c_float = 0.;
    let mut _987: core::ffi::c_float = 0.;
    let mut _988: core::ffi::c_float = 0.;
    let mut _989: core::ffi::c_float = 0.;
    let mut _990: core::ffi::c_float = 0.;
    let mut _991: core::ffi::c_float = 0.;
    let mut _992: core::ffi::c_float = 0.;
    let mut _993: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _994: core::ffi::c_float = 0.;
    let mut _995: core::ffi::c_float = 0.;
    let mut _996: core::ffi::c_float = 0.;
    let mut _997: core::ffi::c_float = 0.;
    let mut _998: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _999: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1000: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1001: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1002: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1003: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1004: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1005: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1006: core::ffi::c_float = 0.;
    let mut _1007: core::ffi::c_float = 0.;
    let mut _1008: core::ffi::c_float = 0.;
    let mut _1009: core::ffi::c_float = 0.;
    let mut _1010: core::ffi::c_float = 0.;
    let mut _1011: core::ffi::c_float = 0.;
    let mut _1012: core::ffi::c_float = 0.;
    let mut _1013: core::ffi::c_float = 0.;
    let mut _1014: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1015: core::ffi::c_float = 0.;
    let mut _1016: core::ffi::c_float = 0.;
    let mut _1017: core::ffi::c_float = 0.;
    let mut _1018: core::ffi::c_float = 0.;
    let mut _1019: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1020: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1021: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1022: core::ffi::c_float = 0.;
    let mut _1023: core::ffi::c_float = 0.;
    let mut _1024: core::ffi::c_float = 0.;
    let mut _1025: core::ffi::c_float = 0.;
    let mut _1026: core::ffi::c_float = 0.;
    let mut _1027: uint32_t = 0;
    let mut _1028: uint32_t = 0;
    let mut _1029: core::ffi::c_float = 0.;
    let mut _1030: uint32_t = 0;
    let mut _1031: uint32_t = 0;
    let mut _1032: core::ffi::c_float = 0.;
    let mut _1033: uint32_t = 0;
    let mut _1034: uint32_t = 0;
    let mut _1035: uint32_t = 0;
    let mut _1036: uint32_t = 0;
    let mut _1037: uint32_t = 0;
    let mut _1038: uint16_t = 0;
    let mut _1039: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1040: uint32_t = 0;
    let mut _1041: uint32_t = 0;
    let mut _1042: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1043: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1044: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1045: uint32_t = 0;
    let mut _1046: uint32_t = 0;
    let mut _1047: uint32_t = 0;
    let mut _1048: uint32_t = 0;
    let mut _1049: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1050: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1051: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1052: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1053: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1054: core::ffi::c_float = 0.;
    let mut _1055: core::ffi::c_float = 0.;
    let mut _1056: core::ffi::c_float = 0.;
    let mut _1057: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1058: core::ffi::c_float = 0.;
    let mut _1059: core::ffi::c_float = 0.;
    let mut _1060: core::ffi::c_float = 0.;
    let mut _1061: core::ffi::c_float = 0.;
    let mut _1062: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1063: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1064: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1065: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1066: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1067: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1068: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1069: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1070: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1071: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1072: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1073: core::ffi::c_float = 0.;
    let mut _1074: core::ffi::c_float = 0.;
    let mut _1075: core::ffi::c_float = 0.;
    let mut _1076: core::ffi::c_float = 0.;
    let mut _1077: core::ffi::c_float = 0.;
    let mut _1078: core::ffi::c_float = 0.;
    let mut _1079: core::ffi::c_float = 0.;
    let mut _1080: core::ffi::c_float = 0.;
    let mut _1081: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1082: core::ffi::c_float = 0.;
    let mut _1083: core::ffi::c_float = 0.;
    let mut _1084: core::ffi::c_float = 0.;
    let mut _1085: core::ffi::c_float = 0.;
    let mut _1086: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1087: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1088: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1089: core::ffi::c_float = 0.;
    let mut _1090: core::ffi::c_float = 0.;
    let mut _1091: core::ffi::c_float = 0.;
    let mut _1092: core::ffi::c_float = 0.;
    let mut _1093: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1094: core::ffi::c_float = 0.;
    let mut _1095: core::ffi::c_float = 0.;
    let mut _1096: core::ffi::c_float = 0.;
    let mut _1097: core::ffi::c_float = 0.;
    let mut _1098: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1099: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1100: core::ffi::c_float = 0.;
    let mut _1101: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1102: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1103: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1104: core::ffi::c_float = 0.;
    let mut _1105: core::ffi::c_float = 0.;
    let mut _1106: core::ffi::c_float = 0.;
    let mut _1107: core::ffi::c_float = 0.;
    let mut _1108: core::ffi::c_float = 0.;
    let mut _1109: core::ffi::c_float = 0.;
    let mut _1110: core::ffi::c_float = 0.;
    let mut _1111: core::ffi::c_float = 0.;
    let mut _1112: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1113: core::ffi::c_float = 0.;
    let mut _1114: core::ffi::c_float = 0.;
    let mut _1115: core::ffi::c_float = 0.;
    let mut _1116: core::ffi::c_float = 0.;
    let mut _1117: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1118: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1119: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1120: core::ffi::c_float = 0.;
    let mut _1121: core::ffi::c_float = 0.;
    let mut _1122: core::ffi::c_float = 0.;
    let mut _1123: core::ffi::c_float = 0.;
    let mut _1124: core::ffi::c_float = 0.;
    let mut _1125: core::ffi::c_float = 0.;
    let mut _1126: core::ffi::c_float = 0.;
    let mut _1127: core::ffi::c_float = 0.;
    let mut _1128: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1129: core::ffi::c_float = 0.;
    let mut _1130: core::ffi::c_float = 0.;
    let mut _1131: core::ffi::c_float = 0.;
    let mut _1132: core::ffi::c_float = 0.;
    let mut _1133: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1134: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1135: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1136: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1137: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1138: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1139: core::ffi::c_float = 0.;
    let mut _1140: core::ffi::c_float = 0.;
    let mut _1141: core::ffi::c_float = 0.;
    let mut _1142: core::ffi::c_float = 0.;
    let mut _1143: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1144: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1145: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1146: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1147: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1148: core::ffi::c_float = 0.;
    let mut _1149: core::ffi::c_float = 0.;
    let mut _1150: core::ffi::c_float = 0.;
    let mut _1151: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1152: core::ffi::c_float = 0.;
    let mut _1153: core::ffi::c_float = 0.;
    let mut _1154: core::ffi::c_float = 0.;
    let mut _1155: core::ffi::c_float = 0.;
    let mut _1156: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1157: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1158: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1159: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1160: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1161: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1162: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1163: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1164: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1165: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1166: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1167: core::ffi::c_float = 0.;
    let mut _1168: core::ffi::c_float = 0.;
    let mut _1169: core::ffi::c_float = 0.;
    let mut _1170: core::ffi::c_float = 0.;
    let mut _1171: core::ffi::c_float = 0.;
    let mut _1172: core::ffi::c_float = 0.;
    let mut _1173: core::ffi::c_float = 0.;
    let mut _1174: core::ffi::c_float = 0.;
    let mut _1175: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1176: core::ffi::c_float = 0.;
    let mut _1177: core::ffi::c_float = 0.;
    let mut _1178: core::ffi::c_float = 0.;
    let mut _1179: core::ffi::c_float = 0.;
    let mut _1180: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1181: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1182: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1183: core::ffi::c_float = 0.;
    let mut _1184: core::ffi::c_float = 0.;
    let mut _1185: core::ffi::c_float = 0.;
    let mut _1186: core::ffi::c_float = 0.;
    let mut _1187: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1188: core::ffi::c_float = 0.;
    let mut _1189: core::ffi::c_float = 0.;
    let mut _1190: core::ffi::c_float = 0.;
    let mut _1191: core::ffi::c_float = 0.;
    let mut _1192: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1193: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1194: core::ffi::c_float = 0.;
    let mut _1195: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1196: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1197: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1198: core::ffi::c_float = 0.;
    let mut _1199: core::ffi::c_float = 0.;
    let mut _1200: core::ffi::c_float = 0.;
    let mut _1201: core::ffi::c_float = 0.;
    let mut _1202: core::ffi::c_float = 0.;
    let mut _1203: core::ffi::c_float = 0.;
    let mut _1204: core::ffi::c_float = 0.;
    let mut _1205: core::ffi::c_float = 0.;
    let mut _1206: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1207: core::ffi::c_float = 0.;
    let mut _1208: core::ffi::c_float = 0.;
    let mut _1209: core::ffi::c_float = 0.;
    let mut _1210: core::ffi::c_float = 0.;
    let mut _1211: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1212: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1213: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1214: core::ffi::c_float = 0.;
    let mut _1215: core::ffi::c_float = 0.;
    let mut _1216: core::ffi::c_float = 0.;
    let mut _1217: core::ffi::c_float = 0.;
    let mut _1218: core::ffi::c_float = 0.;
    let mut _1219: core::ffi::c_float = 0.;
    let mut _1220: core::ffi::c_float = 0.;
    let mut _1221: core::ffi::c_float = 0.;
    let mut _1222: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1223: core::ffi::c_float = 0.;
    let mut _1224: core::ffi::c_float = 0.;
    let mut _1225: core::ffi::c_float = 0.;
    let mut _1226: core::ffi::c_float = 0.;
    let mut _1227: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1228: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1229: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1230: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1231: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1232: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1233: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1234: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1235: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1236: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1237: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1238: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1239: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1240: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1241: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1242: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1243: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1244: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1245: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1246: core::ffi::c_float = 0.;
    let mut _1247: core::ffi::c_float = 0.;
    let mut _1248: core::ffi::c_float = 0.;
    let mut _1249: core::ffi::c_float = 0.;
    let mut _1250: core::ffi::c_float = 0.;
    let mut _1251: core::ffi::c_float = 0.;
    let mut _1252: core::ffi::c_float = 0.;
    let mut _1253: core::ffi::c_float = 0.;
    let mut _1254: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1255: core::ffi::c_float = 0.;
    let mut _1256: core::ffi::c_float = 0.;
    let mut _1257: core::ffi::c_float = 0.;
    let mut _1258: core::ffi::c_float = 0.;
    let mut _1259: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1260: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1261: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1262: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1263: core::ffi::c_float = 0.;
    let mut _1264: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1265: core::ffi::c_float = 0.;
    let mut _1266: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1267: core::ffi::c_float = 0.;
    let mut _1268: core::ffi::c_float = 0.;
    let mut _1269: core::ffi::c_float = 0.;
    let mut _1270: core::ffi::c_float = 0.;
    let mut _1271: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1272: core::ffi::c_float = 0.;
    let mut _1273: core::ffi::c_float = 0.;
    let mut _1274: core::ffi::c_float = 0.;
    let mut _1275: core::ffi::c_float = 0.;
    let mut _1276: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1277: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1278: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1279: core::ffi::c_float = 0.;
    let mut _1280: core::ffi::c_float = 0.;
    let mut _1281: core::ffi::c_float = 0.;
    let mut _1282: core::ffi::c_float = 0.;
    let mut _1283: core::ffi::c_float = 0.;
    let mut _1284: core::ffi::c_float = 0.;
    let mut _1285: core::ffi::c_float = 0.;
    let mut _1286: core::ffi::c_float = 0.;
    let mut _1287: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1288: core::ffi::c_float = 0.;
    let mut _1289: core::ffi::c_float = 0.;
    let mut _1290: core::ffi::c_float = 0.;
    let mut _1291: core::ffi::c_float = 0.;
    let mut _1292: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1293: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1294: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1295: core::ffi::c_float = 0.;
    let mut _1296: core::ffi::c_float = 0.;
    let mut _1297: core::ffi::c_float = 0.;
    let mut _1298: core::ffi::c_float = 0.;
    let mut _1299: core::ffi::c_float = 0.;
    let mut _1300: core::ffi::c_float = 0.;
    let mut _1301: core::ffi::c_float = 0.;
    let mut _1302: core::ffi::c_float = 0.;
    let mut _1303: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1304: core::ffi::c_float = 0.;
    let mut _1305: core::ffi::c_float = 0.;
    let mut _1306: core::ffi::c_float = 0.;
    let mut _1307: core::ffi::c_float = 0.;
    let mut _1308: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1309: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1310: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1311: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1312: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1313: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1314: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1315: core::ffi::c_float = 0.;
    let mut _1316: core::ffi::c_float = 0.;
    let mut _1317: core::ffi::c_float = 0.;
    let mut _1318: core::ffi::c_float = 0.;
    let mut _1319: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1320: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1321: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1322: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1323: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1324: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1325: uint32_t = 0;
    let mut _1326: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1327: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1328: uint32_t = 0;
    let mut _1329: uint32_t = 0;
    let mut _1330: uint32_t = 0;
    let mut _1331: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1332: uint32_t = 0;
    let mut _1333: uint8_t = 0;
    let mut _1334: core::ffi::c_float = 0.;
    let mut _1335: core::ffi::c_float = 0.;
    let mut _1336: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1337: core::ffi::c_float = 0.;
    let mut _1338: core::ffi::c_float = 0.;
    let mut _1339: core::ffi::c_float = 0.;
    let mut _1340: core::ffi::c_float = 0.;
    let mut _1341: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1342: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1343: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1344: core::ffi::c_float = 0.;
    let mut _1345: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1346: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1347: core::ffi::c_float = 0.;
    let mut _1348: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1349: core::ffi::c_float = 0.;
    let mut _1350: core::ffi::c_float = 0.;
    let mut _1351: core::ffi::c_float = 0.;
    let mut _1352: core::ffi::c_float = 0.;
    let mut _1353: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1354: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1355: core::ffi::c_float = 0.;
    let mut _1356: core::ffi::c_float = 0.;
    let mut _1357: core::ffi::c_float = 0.;
    let mut _1358: core::ffi::c_float = 0.;
    let mut _1359: core::ffi::c_float = 0.;
    let mut _1360: core::ffi::c_float = 0.;
    let mut _1361: core::ffi::c_float = 0.;
    let mut _1362: core::ffi::c_float = 0.;
    let mut _1363: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1364: core::ffi::c_float = 0.;
    let mut _1365: core::ffi::c_float = 0.;
    let mut _1366: core::ffi::c_float = 0.;
    let mut _1367: core::ffi::c_float = 0.;
    let mut _1368: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1369: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1370: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1371: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1372: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1373: core::ffi::c_float = 0.;
    let mut _1374: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1375: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1376: core::ffi::c_float = 0.;
    let mut _1377: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1378: core::ffi::c_float = 0.;
    let mut _1379: core::ffi::c_float = 0.;
    let mut _1380: core::ffi::c_float = 0.;
    let mut _1381: core::ffi::c_float = 0.;
    let mut _1382: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1383: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1384: core::ffi::c_float = 0.;
    let mut _1385: core::ffi::c_float = 0.;
    let mut _1386: core::ffi::c_float = 0.;
    let mut _1387: core::ffi::c_float = 0.;
    let mut _1388: core::ffi::c_float = 0.;
    let mut _1389: core::ffi::c_float = 0.;
    let mut _1390: core::ffi::c_float = 0.;
    let mut _1391: core::ffi::c_float = 0.;
    let mut _1392: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1393: core::ffi::c_float = 0.;
    let mut _1394: core::ffi::c_float = 0.;
    let mut _1395: core::ffi::c_float = 0.;
    let mut _1396: core::ffi::c_float = 0.;
    let mut _1397: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1398: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1399: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1400: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1401: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1402: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1403: core::ffi::c_float = 0.;
    let mut _1404: core::ffi::c_float = 0.;
    let mut _1405: core::ffi::c_float = 0.;
    let mut _1406: core::ffi::c_float = 0.;
    let mut _1407: core::ffi::c_float = 0.;
    let mut _1408: core::ffi::c_float = 0.;
    let mut _1409: core::ffi::c_float = 0.;
    let mut _1410: core::ffi::c_float = 0.;
    let mut _1411: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1412: core::ffi::c_float = 0.;
    let mut _1413: core::ffi::c_float = 0.;
    let mut _1414: core::ffi::c_float = 0.;
    let mut _1415: core::ffi::c_float = 0.;
    let mut _1416: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1417: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1418: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1419: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1420: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1421: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1422: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1423: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1424: core::ffi::c_float = 0.;
    let mut _1425: core::ffi::c_float = 0.;
    let mut _1426: core::ffi::c_float = 0.;
    let mut _1427: core::ffi::c_float = 0.;
    let mut _1428: core::ffi::c_float = 0.;
    let mut _1429: core::ffi::c_float = 0.;
    let mut _1430: core::ffi::c_float = 0.;
    let mut _1431: core::ffi::c_float = 0.;
    let mut _1432: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1433: core::ffi::c_float = 0.;
    let mut _1434: core::ffi::c_float = 0.;
    let mut _1435: core::ffi::c_float = 0.;
    let mut _1436: core::ffi::c_float = 0.;
    let mut _1437: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1438: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1439: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1440: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1441: core::ffi::c_float = 0.;
    let mut _1442: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1443: core::ffi::c_float = 0.;
    let mut _1444: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1445: core::ffi::c_float = 0.;
    let mut _1446: core::ffi::c_float = 0.;
    let mut _1447: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1448: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1449: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1450: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1451: core::ffi::c_float = 0.;
    let mut _1452: core::ffi::c_float = 0.;
    let mut _1453: core::ffi::c_float = 0.;
    let mut _1454: core::ffi::c_float = 0.;
    let mut _1455: core::ffi::c_float = 0.;
    let mut _1456: core::ffi::c_float = 0.;
    let mut _1457: core::ffi::c_float = 0.;
    let mut _1458: core::ffi::c_float = 0.;
    let mut _1459: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1460: core::ffi::c_float = 0.;
    let mut _1461: core::ffi::c_float = 0.;
    let mut _1462: core::ffi::c_float = 0.;
    let mut _1463: core::ffi::c_float = 0.;
    let mut _1464: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1465: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1466: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1467: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1468: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1469: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1470: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1471: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1472: core::ffi::c_float = 0.;
    let mut _1473: core::ffi::c_float = 0.;
    let mut _1474: core::ffi::c_float = 0.;
    let mut _1475: core::ffi::c_float = 0.;
    let mut _1476: core::ffi::c_float = 0.;
    let mut _1477: core::ffi::c_float = 0.;
    let mut _1478: core::ffi::c_float = 0.;
    let mut _1479: core::ffi::c_float = 0.;
    let mut _1480: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1481: core::ffi::c_float = 0.;
    let mut _1482: core::ffi::c_float = 0.;
    let mut _1483: core::ffi::c_float = 0.;
    let mut _1484: core::ffi::c_float = 0.;
    let mut _1485: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1486: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1487: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1488: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1489: core::ffi::c_float = 0.;
    let mut _1490: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1491: core::ffi::c_float = 0.;
    let mut _1492: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1493: core::ffi::c_float = 0.;
    let mut _1494: core::ffi::c_float = 0.;
    let mut _1495: uint32_t = 0;
    let mut _1496: uint32_t = 0;
    let mut _1497: core::ffi::c_float = 0.;
    let mut _1498: uint32_t = 0;
    let mut _1499: uint32_t = 0;
    let mut _1500: core::ffi::c_float = 0.;
    let mut _1501: uint32_t = 0;
    let mut _1502: uint32_t = 0;
    let mut _1503: uint32_t = 0;
    let mut _1504: uint32_t = 0;
    let mut _1505: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1506: uint32_t = 0;
    let mut _1507: uint32_t = 0;
    let mut _1508: uint32_t = 0;
    let mut _1509: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1510: uint16_t = 0;
    let mut _1511: uint32_t = 0;
    let mut _1512: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1513: uint32_t = 0;
    let mut _1514: uint32_t = 0;
    let mut _1515: uint32_t = 0;
    let mut _1516: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1517: uint16_t = 0;
    let mut _1518: uint32_t = 0;
    let mut _1519: uint32_t = 0;
    let mut _1520: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1521: uint32_t = 0;
    let mut _1522: uint32_t = 0;
    let mut _1523: uint32_t = 0;
    let mut _1524: uint32_t = 0;
    let mut _1525: uint32_t = 0;
    let mut _1526: uint32_t = 0;
    let mut _1527: uint32_t = 0;
    let mut _1528: uint64_t = 0;
    let mut _1529: uint32_t = 0;
    let mut _1530: uint8_t = 0;
    let mut _1531: uint32_t = 0;
    let mut _1532: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1533: uint32_t = 0;
    let mut _1534: uint32_t = 0;
    let mut _1535: uint32_t = 0;
    let mut _1536: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1537: uint64_t = 0;
    let mut _1538: uint32_t = 0;
    let mut _1539: uint32_t = 0;
    let mut _1540: uint32_t = 0;
    let mut _1541: uint32_t = 0;
    let mut _1542: uint32_t = 0;
    _448 = _1;
    _449 = _2;
    _450 = _3;
    _451 = _4;
    _452 = _5;
    _453 = _6;
    _557 = _448;
    _558 =
        *(&mut (*(_557 as *mut l_struct_struct_OC_block_size_descriptor)).field3 as *mut uint8_t);
    _454 = _558 as uint32_t;
    _455 = 0.0549999997f64 as core::ffi::c_float;
    _559 = _454;
    if _559 <= 20 as core::ffi::c_uint {
        _455 = 0.0299999993f64 as core::ffi::c_float;
    } else {
        _560 = _454;
        if _560 <= 31 as core::ffi::c_uint {
            _455 = 0.0399999991f64 as core::ffi::c_float;
        } else {
            _561 = _454;
            if _561 <= 41 as core::ffi::c_uint {
                _455 = 0.0500000007f64 as core::ffi::c_float;
            }
        }
    }
    _562 = _450;
    if _562 > 0 as core::ffi::c_uint {
        _563 = _451;
        if _563 > 0 as core::ffi::c_uint {
            _564 = _455;
            _565 = _455;
            _455 = llvm_fmul_f32(_564, _565);
            _566 = _448;
            _567 = _449;
            _568 = _450;
            _569 =
                _ZL33compute_kmeans_partition_orderingRK21block_size_descriptorRK11image_blockjPt(
                    _566,
                    _567,
                    _568,
                    &mut *(_456.array)
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as int64_t as isize)
                        as *mut uint16_t as *mut core::ffi::c_void,
                );
            _457 = _569;
            _570 = _451;
            _571 = _457;
            _572 = _ZN4astcL3minIjEET_S1_S1_(_570, _571);
            _451 = _572;
            _573 = _451;
            _574 = _453;
            _575 = _ZN4astcL3minIjEET_S1_S1_(_573, _574);
            _453 = _575;
            _576 = _449;
            _577 =
                _ZNK11image_block19is_constant_channelEi(_576, 3 as core::ffi::c_int as uint32_t);
            _458 = (_577 as core::ffi::c_int ^ 1 as libc::c_int) as bool_0;
            _463 = 0 as core::ffi::c_int as uint32_t;
            loop {
                _578 = _463;
                _579 = _453;
                if !(_578 < _579) {
                    break;
                }
                _580 = _463;
                *(&mut *(_459.array)
                    .as_mut_ptr()
                    .offset(_580 as uint64_t as int64_t as isize)
                    as *mut core::ffi::c_float) = 1.00000002E+30f64 as libc::c_float;
                _581 = _463;
                *(&mut *(_461.array)
                    .as_mut_ptr()
                    .offset(_581 as uint64_t as int64_t as isize)
                    as *mut core::ffi::c_float) = 1.00000002E+30f64 as libc::c_float;
                _582 = _463;
                _463 = llvm_add_u32(_582, 1 as core::ffi::c_int as uint32_t);
            }
            _583 = _458;
            if _583 as core::ffi::c_uint & 1 as libc::c_uint != 0 {
                _464 = 0 as core::ffi::c_int as uint32_t;
                loop {
                    _584 = _464;
                    _585 = _451;
                    if !(_584 < _585) {
                        break;
                    }
                    _586 = _464;
                    _587 = *(&mut *(_456.array)
                        .as_mut_ptr()
                        .offset(_586 as uint64_t as int64_t as isize)
                        as *mut uint16_t);
                    _465 = _587 as uint32_t;
                    _588 = _448;
                    _589 = _450;
                    _590 = _465;
                    _591 = _ZNK21block_size_descriptor22get_raw_partition_infoEjj(_588, _589, _590);
                    _466 = _591;
                    _592 = _466;
                    _593 = _449;
                    _Z28compute_avgs_and_dirs_4_compRK14partition_infoRK11image_blockP17partition_metrics(
                        _592,
                        _593,
                        &mut *(_467.array)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as int64_t as isize)
                            as *mut l_struct_struct_OC_partition_metrics
                            as *mut core::ffi::c_void,
                    );
                    _473 = 0 as core::ffi::c_int as uint32_t;
                    loop {
                        _594 = _473;
                        _595 = _450;
                        if !(_594 < _595) {
                            break;
                        }
                        _596 = _473;
                        _474 = &mut *(_467.array)
                            .as_mut_ptr()
                            .offset(_596 as uint64_t as int64_t as isize)
                            as *mut l_struct_struct_OC_partition_metrics
                            as *mut core::ffi::c_void;
                        _597 = _474;
                        _598 = _473;
                        _599 = memcpy(
                            &mut (*(&mut *(_468.array)
                                .as_mut_ptr()
                                .offset(_598 as uint64_t as int64_t as isize)
                                as *mut l_struct_struct_OC_line4))
                                .field0
                                as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void,
                            &mut (*(_597 as *mut l_struct_struct_OC_partition_metrics)).field0
                                as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _600 = _474;
                        _601 = memcpy(
                            &mut _476 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut (*(_600 as *mut l_struct_struct_OC_partition_metrics)).field1
                                as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _312 =
                            &mut _410 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _313 = 0.5f64 as core::ffi::c_float;
                        _602 = _312;
                        _603 = _313;
                        *(_602 as *mut core::ffi::c_float) = _603;
                        _604 = _313;
                        *(&mut *((*(_602 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _604;
                        _605 = _313;
                        *(&mut *((*(_602 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _605;
                        _606 = _313;
                        *(&mut *((*(_602 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _606;
                        _607 = _410;
                        *(&mut _477.field0 as *mut l_array_4_float) = _607.field0;
                        _608 = *(&mut _476.field0 as *mut l_array_4_float);
                        _609 = *(&mut _477.field0 as *mut l_array_4_float);
                        *(&mut _413 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _608;
                        *(&mut _414 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _609;
                        _610 = memcpy(
                            &mut _416 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _413 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _611 = memcpy(
                            &mut _417 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _413 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _612 =
                            *(&mut _416 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        _613 =
                            *(&mut _417 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        *(&mut _380 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _612;
                        *(&mut _381 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _613;
                        _614 = memcpy(
                            &mut _383 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _380 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _615 = memcpy(
                            &mut _384 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _381 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _616 =
                            *(&mut _383 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        _617 =
                            *(&mut _384 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        *(&mut _377 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _616;
                        *(&mut _378 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _617;
                        _618 = *(&mut _377 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _619 = *(&mut _378 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _620 = *(&mut *((*(&mut _377 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _621 = *(&mut *((*(&mut _378 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _622 = *(&mut *((*(&mut _377 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _623 = *(&mut *((*(&mut _378 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _624 = *(&mut *((*(&mut _377 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _625 = *(&mut *((*(&mut _378 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _130 =
                            &mut _376 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _131 = llvm_fmul_f32(_618, _619);
                        _132 = llvm_fmul_f32(_620, _621);
                        _133 = llvm_fmul_f32(_622, _623);
                        _134 = llvm_fmul_f32(_624, _625);
                        _626 = _130;
                        _627 = _131;
                        *(_626 as *mut core::ffi::c_float) = _627;
                        _628 = _132;
                        *(&mut *((*(_626 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _628;
                        _629 = _133;
                        *(&mut *((*(_626 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _629;
                        _630 = _134;
                        *(&mut *((*(_626 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _630;
                        _631 = _376;
                        *(&mut _382 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _631.field0;
                        _632 = memcpy(
                            &mut _385 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _382 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _633 =
                            *(&mut _385 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        *(&mut _19 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _633;
                        _634 = *(&mut _19 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _635 = *(&mut *((*(&mut _19 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _636 = *(&mut *((*(&mut _19 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _637 = *(&mut *((*(&mut _19 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _314 =
                            &mut _379 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _315 = llvm_fadd_f32(llvm_fadd_f32(_634, _635), llvm_fadd_f32(_636, _637));
                        _638 = _314;
                        _639 = _315;
                        *(_638 as *mut core::ffi::c_float) = _639;
                        _640 = _315;
                        *(&mut *((*(_638 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _640;
                        _641 = _315;
                        *(&mut *((*(_638 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _641;
                        _642 = _315;
                        *(&mut *((*(_638 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _642;
                        _643 = _379;
                        *(&mut _415 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _643.field0;
                        _89 =
                            &mut _415 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _644 = _89;
                        _645 = *(_644 as *mut core::ffi::c_float);
                        if llvm_fcmp_une(
                            _645 as core::ffi::c_double,
                            0 as core::ffi::c_int as libc::c_double,
                        ) != 0
                        {
                            _646 = memcpy(
                                &mut _418 as *mut l_struct_struct_OC_vfloat4
                                    as *mut core::ffi::c_void,
                                &mut _413 as *mut l_struct_struct_OC_vfloat4
                                    as *mut core::ffi::c_void,
                                16 as core::ffi::c_int as uint64_t,
                            );
                            _647 = memcpy(
                                &mut _420 as *mut l_struct_struct_OC_vfloat4
                                    as *mut core::ffi::c_void,
                                &mut _415 as *mut l_struct_struct_OC_vfloat4
                                    as *mut core::ffi::c_void,
                                16 as core::ffi::c_int as uint64_t,
                            );
                            _648 = *(&mut _420 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float);
                            *(&mut _53 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float) = _648;
                            _649 = *(&mut _53 as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_float);
                            _650 = _ZSt4sqrtf(_649);
                            _651 = *(&mut *((*(&mut _53 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float))
                                .array)
                                .as_mut_ptr()
                                .offset(1 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float);
                            _652 = _ZSt4sqrtf(_651);
                            _653 = *(&mut *((*(&mut _53 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float))
                                .array)
                                .as_mut_ptr()
                                .offset(2 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float);
                            _654 = _ZSt4sqrtf(_653);
                            _655 = *(&mut *((*(&mut _53 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float))
                                .array)
                                .as_mut_ptr()
                                .offset(3 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float);
                            _656 = _ZSt4sqrtf(_655);
                            _47 = &mut _52 as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void;
                            _48 = _650;
                            _49 = _652;
                            _50 = _654;
                            _51 = _656;
                            _657 = _47;
                            _658 = _48;
                            *(_657 as *mut core::ffi::c_float) = _658;
                            _659 = _49;
                            *(&mut *((*(_657 as *mut l_array_4_float)).array)
                                .as_mut_ptr()
                                .offset(1 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float) = _659;
                            _660 = _50;
                            *(&mut *((*(_657 as *mut l_array_4_float)).array)
                                .as_mut_ptr()
                                .offset(2 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float) = _660;
                            _661 = _51;
                            *(&mut *((*(_657 as *mut l_array_4_float)).array)
                                .as_mut_ptr()
                                .offset(3 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float) = _661;
                            _662 = _52;
                            *(&mut _419 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float) = _662.field0;
                            _663 = *(&mut _418 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float);
                            _664 = *(&mut _419 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float);
                            *(&mut _84 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float) = _663;
                            *(&mut _85 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float) = _664;
                            _665 = *(&mut _84 as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_float);
                            _666 = *(&mut _85 as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_float);
                            _667 = *(&mut *((*(&mut _84 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float))
                                .array)
                                .as_mut_ptr()
                                .offset(1 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float);
                            _668 = *(&mut *((*(&mut _85 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float))
                                .array)
                                .as_mut_ptr()
                                .offset(1 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float);
                            _669 = *(&mut *((*(&mut _84 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float))
                                .array)
                                .as_mut_ptr()
                                .offset(2 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float);
                            _670 = *(&mut *((*(&mut _85 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float))
                                .array)
                                .as_mut_ptr()
                                .offset(2 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float);
                            _671 = *(&mut *((*(&mut _84 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float))
                                .array)
                                .as_mut_ptr()
                                .offset(3 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float);
                            _672 = *(&mut *((*(&mut _85 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float))
                                .array)
                                .as_mut_ptr()
                                .offset(3 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float);
                            _78 = &mut _83 as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void;
                            _79 = llvm_fdiv_f32(_665, _666);
                            _80 = llvm_fdiv_f32(_667, _668);
                            _81 = llvm_fdiv_f32(_669, _670);
                            _82 = llvm_fdiv_f32(_671, _672);
                            _673 = _78;
                            _674 = _79;
                            *(_673 as *mut core::ffi::c_float) = _674;
                            _675 = _80;
                            *(&mut *((*(_673 as *mut l_array_4_float)).array)
                                .as_mut_ptr()
                                .offset(1 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float) = _675;
                            _676 = _81;
                            *(&mut *((*(_673 as *mut l_array_4_float)).array)
                                .as_mut_ptr()
                                .offset(2 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float) = _676;
                            _677 = _82;
                            *(&mut *((*(_673 as *mut l_array_4_float)).array)
                                .as_mut_ptr()
                                .offset(3 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float) = _677;
                            _678 = _83;
                            *(&mut _412 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float) = _678.field0;
                        } else {
                            _679 = memcpy(
                                &mut _412 as *mut l_struct_struct_OC_vfloat4
                                    as *mut core::ffi::c_void,
                                &mut _414 as *mut l_struct_struct_OC_vfloat4
                                    as *mut core::ffi::c_void,
                                16 as core::ffi::c_int as uint64_t,
                            );
                        }
                        _680 = _412;
                        *(&mut _475.field0 as *mut l_array_4_float) = _680.field0;
                        _681 = _473;
                        _682 = memcpy(
                            &mut (*(&mut *(_468.array)
                                .as_mut_ptr()
                                .offset(_681 as uint64_t as int64_t as isize)
                                as *mut l_struct_struct_OC_line4))
                                .field1
                                as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void,
                            &mut _475 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _683 = _473;
                        _684 = memcpy(
                            &mut _479 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut (*(&mut *(_468.array)
                                .as_mut_ptr()
                                .offset(_683 as uint64_t as int64_t as isize)
                                as *mut l_struct_struct_OC_line4))
                                .field0
                                as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _685 = _473;
                        _686 = memcpy(
                            &mut _481 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut (*(&mut *(_468.array)
                                .as_mut_ptr()
                                .offset(_685 as uint64_t as int64_t as isize)
                                as *mut l_struct_struct_OC_line4))
                                .field1
                                as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _687 = _473;
                        _688 = memcpy(
                            &mut _483 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut (*(&mut *(_468.array)
                                .as_mut_ptr()
                                .offset(_687 as uint64_t as int64_t as isize)
                                as *mut l_struct_struct_OC_line4))
                                .field0
                                as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _689 = _473;
                        _690 = memcpy(
                            &mut _484 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut (*(&mut *(_468.array)
                                .as_mut_ptr()
                                .offset(_689 as uint64_t as int64_t as isize)
                                as *mut l_struct_struct_OC_line4))
                                .field1
                                as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _691 = *(&mut _483.field0 as *mut l_array_4_float);
                        _692 = *(&mut _484.field0 as *mut l_array_4_float);
                        *(&mut _340 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _691;
                        *(&mut _341 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _692;
                        _693 = memcpy(
                            &mut _343 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _340 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _694 = memcpy(
                            &mut _344 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _341 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _695 =
                            *(&mut _343 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        _696 =
                            *(&mut _344 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        *(&mut _337 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _695;
                        *(&mut _338 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _696;
                        _697 = *(&mut _337 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _698 = *(&mut _338 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _699 = *(&mut *((*(&mut _337 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _700 = *(&mut *((*(&mut _338 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _701 = *(&mut *((*(&mut _337 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _702 = *(&mut *((*(&mut _338 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _703 = *(&mut *((*(&mut _337 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _704 = *(&mut *((*(&mut _338 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _150 =
                            &mut _336 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _151 = llvm_fmul_f32(_697, _698);
                        _152 = llvm_fmul_f32(_699, _700);
                        _153 = llvm_fmul_f32(_701, _702);
                        _154 = llvm_fmul_f32(_703, _704);
                        _705 = _150;
                        _706 = _151;
                        *(_705 as *mut core::ffi::c_float) = _706;
                        _707 = _152;
                        *(&mut *((*(_705 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _707;
                        _708 = _153;
                        *(&mut *((*(_705 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _708;
                        _709 = _154;
                        *(&mut *((*(_705 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _709;
                        _710 = _336;
                        *(&mut _342 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _710.field0;
                        _711 = memcpy(
                            &mut _345 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _342 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _712 =
                            *(&mut _345 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        *(&mut _23 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _712;
                        _713 = *(&mut _23 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _714 = *(&mut *((*(&mut _23 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _715 = *(&mut *((*(&mut _23 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _716 = *(&mut *((*(&mut _23 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _322 =
                            &mut _339 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _323 = llvm_fadd_f32(llvm_fadd_f32(_713, _714), llvm_fadd_f32(_715, _716));
                        _717 = _322;
                        _718 = _323;
                        *(_717 as *mut core::ffi::c_float) = _718;
                        _719 = _323;
                        *(&mut *((*(_717 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _719;
                        _720 = _323;
                        *(&mut *((*(_717 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _720;
                        _721 = _323;
                        *(&mut *((*(_717 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _721;
                        _722 = _339;
                        *(&mut _482.field0 as *mut l_array_4_float) = _722.field0;
                        _723 = *(&mut _481.field0 as *mut l_array_4_float);
                        _724 = *(&mut _482.field0 as *mut l_array_4_float);
                        *(&mut _387 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _723;
                        *(&mut _388 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _724;
                        _725 = *(&mut _387 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _726 = *(&mut _388 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _727 = *(&mut *((*(&mut _387 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _728 = *(&mut *((*(&mut _388 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _729 = *(&mut *((*(&mut _387 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _730 = *(&mut *((*(&mut _388 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _731 = *(&mut *((*(&mut _387 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _732 = *(&mut *((*(&mut _388 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _125 =
                            &mut _386 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _126 = llvm_fmul_f32(_725, _726);
                        _127 = llvm_fmul_f32(_727, _728);
                        _128 = llvm_fmul_f32(_729, _730);
                        _129 = llvm_fmul_f32(_731, _732);
                        _733 = _125;
                        _734 = _126;
                        *(_733 as *mut core::ffi::c_float) = _734;
                        _735 = _127;
                        *(&mut *((*(_733 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _735;
                        _736 = _128;
                        *(&mut *((*(_733 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _736;
                        _737 = _129;
                        *(&mut *((*(_733 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _737;
                        _738 = _386;
                        *(&mut _480.field0 as *mut l_array_4_float) = _738.field0;
                        _739 = *(&mut _479.field0 as *mut l_array_4_float);
                        _740 = *(&mut _480.field0 as *mut l_array_4_float);
                        *(&mut _405 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _739;
                        *(&mut _406 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _740;
                        _741 = *(&mut _405 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _742 = *(&mut _406 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _743 = *(&mut *((*(&mut _405 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _744 = *(&mut *((*(&mut _406 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _745 = *(&mut *((*(&mut _405 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _746 = *(&mut *((*(&mut _406 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _747 = *(&mut *((*(&mut _405 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _748 = *(&mut *((*(&mut _406 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _95 =
                            &mut _404 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _96 = llvm_fsub_f32(_741, _742);
                        _97 = llvm_fsub_f32(_743, _744);
                        _98 = llvm_fsub_f32(_745, _746);
                        _99 = llvm_fsub_f32(_747, _748);
                        _749 = _95;
                        _750 = _96;
                        *(_749 as *mut core::ffi::c_float) = _750;
                        _751 = _97;
                        *(&mut *((*(_749 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _751;
                        _752 = _98;
                        *(&mut *((*(_749 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _752;
                        _753 = _99;
                        *(&mut *((*(_749 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _753;
                        _754 = _404;
                        *(&mut _478.field0 as *mut l_array_4_float) = _754.field0;
                        _755 = _473;
                        _756 = memcpy(
                            &mut (*(&mut *(_470.array)
                                .as_mut_ptr()
                                .offset(_755 as uint64_t as int64_t as isize)
                                as *mut l_struct_struct_OC_processed_line4))
                                .field0
                                as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void,
                            &mut _478 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _757 = _473;
                        _758 = _473;
                        _759 = memcpy(
                            &mut (*(&mut *(_470.array)
                                .as_mut_ptr()
                                .offset(_758 as uint64_t as int64_t as isize)
                                as *mut l_struct_struct_OC_processed_line4))
                                .field1
                                as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void,
                            &mut (*(&mut *(_468.array)
                                .as_mut_ptr()
                                .offset(_757 as uint64_t as int64_t as isize)
                                as *mut l_struct_struct_OC_line4))
                                .field1
                                as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _330 =
                            &mut _332 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _331 = 0 as core::ffi::c_int as libc::c_float;
                        _760 = _330;
                        _761 = _331;
                        *(_760 as *mut core::ffi::c_float) = _761;
                        _762 = _331;
                        *(&mut *((*(_760 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _762;
                        _763 = _331;
                        *(&mut *((*(_760 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _763;
                        _764 = _331;
                        *(&mut *((*(_760 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _764;
                        _765 = _332;
                        *(&mut _485.field0 as *mut l_array_4_float) = _765.field0;
                        _766 = _473;
                        _767 = memcpy(
                            &mut (*(&mut *(_469.array)
                                .as_mut_ptr()
                                .offset(_766 as uint64_t as int64_t as isize)
                                as *mut l_struct_struct_OC_line4))
                                .field0
                                as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void,
                            &mut _485 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _768 = _474;
                        _769 = memcpy(
                            &mut _487 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut (*(_768 as *mut l_struct_struct_OC_partition_metrics)).field0
                                as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _310 =
                            &mut _411 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _311 = 0.5f64 as core::ffi::c_float;
                        _770 = _310;
                        _771 = _311;
                        *(_770 as *mut core::ffi::c_float) = _771;
                        _772 = _311;
                        *(&mut *((*(_770 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _772;
                        _773 = _311;
                        *(&mut *((*(_770 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _773;
                        _774 = _311;
                        *(&mut *((*(_770 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _774;
                        _775 = _411;
                        *(&mut _488.field0 as *mut l_array_4_float) = _775.field0;
                        _776 = *(&mut _487.field0 as *mut l_array_4_float);
                        _777 = *(&mut _488.field0 as *mut l_array_4_float);
                        *(&mut _422 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _776;
                        *(&mut _423 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _777;
                        _778 = memcpy(
                            &mut _425 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _422 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _779 = memcpy(
                            &mut _426 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _422 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _780 =
                            *(&mut _425 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        _781 =
                            *(&mut _426 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        *(&mut _370 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _780;
                        *(&mut _371 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _781;
                        _782 = memcpy(
                            &mut _373 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _370 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _783 = memcpy(
                            &mut _374 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _371 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _784 =
                            *(&mut _373 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        _785 =
                            *(&mut _374 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        *(&mut _367 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _784;
                        *(&mut _368 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _785;
                        _786 = *(&mut _367 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _787 = *(&mut _368 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _788 = *(&mut *((*(&mut _367 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _789 = *(&mut *((*(&mut _368 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _790 = *(&mut *((*(&mut _367 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _791 = *(&mut *((*(&mut _368 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _792 = *(&mut *((*(&mut _367 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _793 = *(&mut *((*(&mut _368 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _135 =
                            &mut _366 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _136 = llvm_fmul_f32(_786, _787);
                        _137 = llvm_fmul_f32(_788, _789);
                        _138 = llvm_fmul_f32(_790, _791);
                        _139 = llvm_fmul_f32(_792, _793);
                        _794 = _135;
                        _795 = _136;
                        *(_794 as *mut core::ffi::c_float) = _795;
                        _796 = _137;
                        *(&mut *((*(_794 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _796;
                        _797 = _138;
                        *(&mut *((*(_794 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _797;
                        _798 = _139;
                        *(&mut *((*(_794 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _798;
                        _799 = _366;
                        *(&mut _372 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _799.field0;
                        _800 = memcpy(
                            &mut _375 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _372 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _801 =
                            *(&mut _375 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        *(&mut _20 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _801;
                        _802 = *(&mut _20 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _803 = *(&mut *((*(&mut _20 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _804 = *(&mut *((*(&mut _20 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _805 = *(&mut *((*(&mut _20 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _316 =
                            &mut _369 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _317 = llvm_fadd_f32(llvm_fadd_f32(_802, _803), llvm_fadd_f32(_804, _805));
                        _806 = _316;
                        _807 = _317;
                        *(_806 as *mut core::ffi::c_float) = _807;
                        _808 = _317;
                        *(&mut *((*(_806 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _808;
                        _809 = _317;
                        *(&mut *((*(_806 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _809;
                        _810 = _317;
                        *(&mut *((*(_806 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _810;
                        _811 = _369;
                        *(&mut _424 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _811.field0;
                        _88 =
                            &mut _424 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _812 = _88;
                        _813 = *(_812 as *mut core::ffi::c_float);
                        if llvm_fcmp_une(
                            _813 as core::ffi::c_double,
                            0 as core::ffi::c_int as libc::c_double,
                        ) != 0
                        {
                            _814 = memcpy(
                                &mut _427 as *mut l_struct_struct_OC_vfloat4
                                    as *mut core::ffi::c_void,
                                &mut _422 as *mut l_struct_struct_OC_vfloat4
                                    as *mut core::ffi::c_void,
                                16 as core::ffi::c_int as uint64_t,
                            );
                            _815 = memcpy(
                                &mut _429 as *mut l_struct_struct_OC_vfloat4
                                    as *mut core::ffi::c_void,
                                &mut _424 as *mut l_struct_struct_OC_vfloat4
                                    as *mut core::ffi::c_void,
                                16 as core::ffi::c_int as uint64_t,
                            );
                            _816 = *(&mut _429 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float);
                            *(&mut _46 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float) = _816;
                            _817 = *(&mut _46 as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_float);
                            _818 = _ZSt4sqrtf(_817);
                            _819 = *(&mut *((*(&mut _46 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float))
                                .array)
                                .as_mut_ptr()
                                .offset(1 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float);
                            _820 = _ZSt4sqrtf(_819);
                            _821 = *(&mut *((*(&mut _46 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float))
                                .array)
                                .as_mut_ptr()
                                .offset(2 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float);
                            _822 = _ZSt4sqrtf(_821);
                            _823 = *(&mut *((*(&mut _46 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float))
                                .array)
                                .as_mut_ptr()
                                .offset(3 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float);
                            _824 = _ZSt4sqrtf(_823);
                            _40 = &mut _45 as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void;
                            _41 = _818;
                            _42 = _820;
                            _43 = _822;
                            _44 = _824;
                            _825 = _40;
                            _826 = _41;
                            *(_825 as *mut core::ffi::c_float) = _826;
                            _827 = _42;
                            *(&mut *((*(_825 as *mut l_array_4_float)).array)
                                .as_mut_ptr()
                                .offset(1 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float) = _827;
                            _828 = _43;
                            *(&mut *((*(_825 as *mut l_array_4_float)).array)
                                .as_mut_ptr()
                                .offset(2 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float) = _828;
                            _829 = _44;
                            *(&mut *((*(_825 as *mut l_array_4_float)).array)
                                .as_mut_ptr()
                                .offset(3 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float) = _829;
                            _830 = _45;
                            *(&mut _428 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float) = _830.field0;
                            _831 = *(&mut _427 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float);
                            _832 = *(&mut _428 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float);
                            *(&mut _76 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float) = _831;
                            *(&mut _77 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float) = _832;
                            _833 = *(&mut _76 as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_float);
                            _834 = *(&mut _77 as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_float);
                            _835 = *(&mut *((*(&mut _76 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float))
                                .array)
                                .as_mut_ptr()
                                .offset(1 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float);
                            _836 = *(&mut *((*(&mut _77 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float))
                                .array)
                                .as_mut_ptr()
                                .offset(1 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float);
                            _837 = *(&mut *((*(&mut _76 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float))
                                .array)
                                .as_mut_ptr()
                                .offset(2 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float);
                            _838 = *(&mut *((*(&mut _77 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float))
                                .array)
                                .as_mut_ptr()
                                .offset(2 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float);
                            _839 = *(&mut *((*(&mut _76 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float))
                                .array)
                                .as_mut_ptr()
                                .offset(3 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float);
                            _840 = *(&mut *((*(&mut _77 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float))
                                .array)
                                .as_mut_ptr()
                                .offset(3 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float);
                            _70 = &mut _75 as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void;
                            _71 = llvm_fdiv_f32(_833, _834);
                            _72 = llvm_fdiv_f32(_835, _836);
                            _73 = llvm_fdiv_f32(_837, _838);
                            _74 = llvm_fdiv_f32(_839, _840);
                            _841 = _70;
                            _842 = _71;
                            *(_841 as *mut core::ffi::c_float) = _842;
                            _843 = _72;
                            *(&mut *((*(_841 as *mut l_array_4_float)).array)
                                .as_mut_ptr()
                                .offset(1 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float) = _843;
                            _844 = _73;
                            *(&mut *((*(_841 as *mut l_array_4_float)).array)
                                .as_mut_ptr()
                                .offset(2 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float) = _844;
                            _845 = _74;
                            *(&mut *((*(_841 as *mut l_array_4_float)).array)
                                .as_mut_ptr()
                                .offset(3 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float) = _845;
                            _846 = _75;
                            *(&mut _421 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float) = _846.field0;
                        } else {
                            _847 = memcpy(
                                &mut _421 as *mut l_struct_struct_OC_vfloat4
                                    as *mut core::ffi::c_void,
                                &mut _423 as *mut l_struct_struct_OC_vfloat4
                                    as *mut core::ffi::c_void,
                                16 as core::ffi::c_int as uint64_t,
                            );
                        }
                        _848 = _421;
                        *(&mut _486.field0 as *mut l_array_4_float) = _848.field0;
                        _849 = _473;
                        _850 = memcpy(
                            &mut (*(&mut *(_469.array)
                                .as_mut_ptr()
                                .offset(_849 as uint64_t as int64_t as isize)
                                as *mut l_struct_struct_OC_line4))
                                .field1
                                as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void,
                            &mut _486 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _328 =
                            &mut _333 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _329 = 0 as core::ffi::c_int as libc::c_float;
                        _851 = _328;
                        _852 = _329;
                        *(_851 as *mut core::ffi::c_float) = _852;
                        _853 = _329;
                        *(&mut *((*(_851 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _853;
                        _854 = _329;
                        *(&mut *((*(_851 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _854;
                        _855 = _329;
                        *(&mut *((*(_851 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _855;
                        _856 = _333;
                        *(&mut _489.field0 as *mut l_array_4_float) = _856.field0;
                        _857 = _473;
                        _858 = memcpy(
                            &mut (*(&mut *(_471.array)
                                .as_mut_ptr()
                                .offset(_857 as uint64_t as int64_t as isize)
                                as *mut l_struct_struct_OC_processed_line4))
                                .field0
                                as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void,
                            &mut _489 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _859 = _473;
                        _860 = _473;
                        _861 = memcpy(
                            &mut (*(&mut *(_471.array)
                                .as_mut_ptr()
                                .offset(_860 as uint64_t as int64_t as isize)
                                as *mut l_struct_struct_OC_processed_line4))
                                .field1
                                as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void,
                            &mut (*(&mut *(_469.array)
                                .as_mut_ptr()
                                .offset(_859 as uint64_t as int64_t as isize)
                                as *mut l_struct_struct_OC_line4))
                                .field1
                                as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _862 = _473;
                        _473 = llvm_add_u32(_862, 1 as core::ffi::c_int as uint32_t);
                    }
                    _490 = 0 as core::ffi::c_int as libc::c_float;
                    _491 = 0 as core::ffi::c_int as libc::c_float;
                    _863 = _466;
                    _864 = _449;
                    _Z26compute_error_squared_rgbaRK14partition_infoRK11image_blockPK15processed_line4S7_PfRfS9_(
                        _863,
                        _864,
                        &mut *(_470.array)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as int64_t as isize)
                            as *mut l_struct_struct_OC_processed_line4
                            as *mut core::ffi::c_void,
                        &mut *(_471.array)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as int64_t as isize)
                            as *mut l_struct_struct_OC_processed_line4
                            as *mut core::ffi::c_void,
                        &mut *(_472.array)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float as *mut libc::c_void,
                        &mut _490 as *mut core::ffi::c_float as *mut libc::c_void,
                        &mut _491 as *mut core::ffi::c_float as *mut libc::c_void,
                    );
                    _492 = 0 as core::ffi::c_int as uint32_t;
                    loop {
                        _865 = _492;
                        _866 = _450;
                        if !(_865 < _866) {
                            break;
                        }
                        _867 = _466;
                        _868 = _492;
                        _869 = *(&mut *((*(&mut (*(_867 as *mut l_struct_struct_OC_partition_info))
                            .field2
                            as *mut l_array_4_uint8_t))
                            .array)
                            .as_mut_ptr()
                            .offset(_868 as uint64_t as int64_t as isize)
                            as *mut uint8_t);
                        _493 = _869 as core::ffi::c_float;
                        _870 = _493;
                        _871 = _455;
                        _308 =
                            &mut _494 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _309 = llvm_fmul_f32(_870, _871);
                        _872 = _308;
                        _873 = _309;
                        *(_872 as *mut core::ffi::c_float) = _873;
                        _874 = _309;
                        *(&mut *((*(_872 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _874;
                        _875 = _309;
                        *(&mut *((*(_872 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _875;
                        _876 = _309;
                        *(&mut *((*(_872 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _876;
                        _877 = _492;
                        _878 = memcpy(
                            &mut _496 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut (*(&mut *(_468.array)
                                .as_mut_ptr()
                                .offset(_877 as uint64_t as int64_t as isize)
                                as *mut l_struct_struct_OC_line4))
                                .field1
                                as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _879 = _492;
                        _880 = *(&mut *(_472.array)
                            .as_mut_ptr()
                            .offset(_879 as uint64_t as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _881 = *(&mut _496.field0 as *mut l_array_4_float);
                        *(&mut _272 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _881;
                        _273 = _880;
                        _882 = memcpy(
                            &mut _274 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _272 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _883 = _273;
                        _266 =
                            &mut _275 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _267 = _883;
                        _884 = _266;
                        _885 = _267;
                        *(_884 as *mut core::ffi::c_float) = _885;
                        _886 = _267;
                        *(&mut *((*(_884 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _886;
                        _887 = _267;
                        *(&mut *((*(_884 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _887;
                        _888 = _267;
                        *(&mut *((*(_884 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _888;
                        _889 =
                            *(&mut _274 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        _890 =
                            *(&mut _275 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        *(&mut _269 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _889;
                        *(&mut _270 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _890;
                        _891 = *(&mut _269 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _892 = *(&mut _270 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _893 = *(&mut *((*(&mut _269 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _894 = *(&mut *((*(&mut _270 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _895 = *(&mut *((*(&mut _269 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _896 = *(&mut *((*(&mut _270 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _897 = *(&mut *((*(&mut _269 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _898 = *(&mut *((*(&mut _270 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _170 =
                            &mut _268 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _171 = llvm_fmul_f32(_891, _892);
                        _172 = llvm_fmul_f32(_893, _894);
                        _173 = llvm_fmul_f32(_895, _896);
                        _174 = llvm_fmul_f32(_897, _898);
                        _899 = _170;
                        _900 = _171;
                        *(_899 as *mut core::ffi::c_float) = _900;
                        _901 = _172;
                        *(&mut *((*(_899 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _901;
                        _902 = _173;
                        *(&mut *((*(_899 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _902;
                        _903 = _174;
                        *(&mut *((*(_899 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _903;
                        _904 = _268;
                        *(&mut _271 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _904.field0;
                        _905 = _271;
                        *(&mut _495.field0 as *mut l_array_4_float) = _905.field0;
                        _906 = _492;
                        _907 = memcpy(
                            &mut _498 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut (*(&mut *(_469.array)
                                .as_mut_ptr()
                                .offset(_906 as uint64_t as int64_t as isize)
                                as *mut l_struct_struct_OC_line4))
                                .field1
                                as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _908 = _492;
                        _909 = *(&mut *(_472.array)
                            .as_mut_ptr()
                            .offset(_908 as uint64_t as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _910 = *(&mut _498.field0 as *mut l_array_4_float);
                        *(&mut _282 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _910;
                        _283 = _909;
                        _911 = memcpy(
                            &mut _284 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _282 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _912 = _283;
                        _276 =
                            &mut _285 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _277 = _912;
                        _913 = _276;
                        _914 = _277;
                        *(_913 as *mut core::ffi::c_float) = _914;
                        _915 = _277;
                        *(&mut *((*(_913 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _915;
                        _916 = _277;
                        *(&mut *((*(_913 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _916;
                        _917 = _277;
                        *(&mut *((*(_913 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _917;
                        _918 =
                            *(&mut _284 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        _919 =
                            *(&mut _285 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        *(&mut _279 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _918;
                        *(&mut _280 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _919;
                        _920 = *(&mut _279 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _921 = *(&mut _280 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _922 = *(&mut *((*(&mut _279 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _923 = *(&mut *((*(&mut _280 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _924 = *(&mut *((*(&mut _279 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _925 = *(&mut *((*(&mut _280 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _926 = *(&mut *((*(&mut _279 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _927 = *(&mut *((*(&mut _280 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _165 =
                            &mut _278 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _166 = llvm_fmul_f32(_920, _921);
                        _167 = llvm_fmul_f32(_922, _923);
                        _168 = llvm_fmul_f32(_924, _925);
                        _169 = llvm_fmul_f32(_926, _927);
                        _928 = _165;
                        _929 = _166;
                        *(_928 as *mut core::ffi::c_float) = _929;
                        _930 = _167;
                        *(&mut *((*(_928 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _930;
                        _931 = _168;
                        *(&mut *((*(_928 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _931;
                        _932 = _169;
                        *(&mut *((*(_928 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _932;
                        _933 = _278;
                        *(&mut _281 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _933.field0;
                        _934 = _281;
                        *(&mut _497.field0 as *mut l_array_4_float) = _934.field0;
                        _935 = memcpy(
                            &mut _500 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _495 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _936 = memcpy(
                            &mut _501 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _495 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _937 = *(&mut _500.field0 as *mut l_array_4_float);
                        _938 = *(&mut _501.field0 as *mut l_array_4_float);
                        *(&mut _390 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _937;
                        *(&mut _391 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _938;
                        _939 = *(&mut _390 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _940 = *(&mut _391 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _941 = *(&mut *((*(&mut _390 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _942 = *(&mut *((*(&mut _391 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _943 = *(&mut *((*(&mut _390 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _944 = *(&mut *((*(&mut _391 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _945 = *(&mut *((*(&mut _390 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _946 = *(&mut *((*(&mut _391 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _120 =
                            &mut _389 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _121 = llvm_fmul_f32(_939, _940);
                        _122 = llvm_fmul_f32(_941, _942);
                        _123 = llvm_fmul_f32(_943, _944);
                        _124 = llvm_fmul_f32(_945, _946);
                        _947 = _120;
                        _948 = _121;
                        *(_947 as *mut core::ffi::c_float) = _948;
                        _949 = _122;
                        *(&mut *((*(_947 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _949;
                        _950 = _123;
                        *(&mut *((*(_947 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _950;
                        _951 = _124;
                        *(&mut *((*(_947 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _951;
                        _952 = _389;
                        *(&mut _499.field0 as *mut l_array_4_float) = _952.field0;
                        _953 = memcpy(
                            &mut _502 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _494 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _954 = *(&mut _499.field0 as *mut l_array_4_float);
                        _955 = *(&mut _502.field0 as *mut l_array_4_float);
                        *(&mut _251 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _954;
                        *(&mut _252 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _955;
                        _956 = memcpy(
                            &mut _254 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _251 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _957 = memcpy(
                            &mut _255 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _252 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _958 =
                            *(&mut _254 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        _959 =
                            *(&mut _255 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        *(&mut _249 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _958;
                        *(&mut _250 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _959;
                        _960 = *(&mut _249 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _961 = *(&mut _250 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _962 = *(&mut *((*(&mut _249 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _963 = *(&mut *((*(&mut _250 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _964 = *(&mut *((*(&mut _249 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _965 = *(&mut *((*(&mut _250 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _966 = *(&mut *((*(&mut _249 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _967 = *(&mut *((*(&mut _250 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _180 =
                            &mut _248 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _181 = llvm_fmul_f32(_960, _961);
                        _182 = llvm_fmul_f32(_962, _963);
                        _183 = llvm_fmul_f32(_964, _965);
                        _184 = llvm_fmul_f32(_966, _967);
                        _968 = _180;
                        _969 = _181;
                        *(_968 as *mut core::ffi::c_float) = _969;
                        _970 = _182;
                        *(&mut *((*(_968 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _970;
                        _971 = _183;
                        *(&mut *((*(_968 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _971;
                        _972 = _184;
                        *(&mut *((*(_968 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _972;
                        _973 = _248;
                        *(&mut _253 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _973.field0;
                        _974 = memcpy(
                            &mut _256 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _253 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _975 =
                            *(&mut _256 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        *(&mut _25 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _975;
                        _976 = *(&mut _25 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _977 = *(&mut *((*(&mut _25 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _978 = *(&mut *((*(&mut _25 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _979 = *(&mut *((*(&mut _25 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _980 = _490;
                        _490 = llvm_fadd_f32(
                            _980,
                            llvm_fadd_f32(llvm_fadd_f32(_976, _977), llvm_fadd_f32(_978, _979)),
                        );
                        _981 = memcpy(
                            &mut _504 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _497 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _982 = memcpy(
                            &mut _505 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _497 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _983 = *(&mut _504.field0 as *mut l_array_4_float);
                        _984 = *(&mut _505.field0 as *mut l_array_4_float);
                        *(&mut _393 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _983;
                        *(&mut _394 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _984;
                        _985 = *(&mut _393 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _986 = *(&mut _394 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _987 = *(&mut *((*(&mut _393 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _988 = *(&mut *((*(&mut _394 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _989 = *(&mut *((*(&mut _393 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _990 = *(&mut *((*(&mut _394 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _991 = *(&mut *((*(&mut _393 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _992 = *(&mut *((*(&mut _394 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _115 =
                            &mut _392 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _116 = llvm_fmul_f32(_985, _986);
                        _117 = llvm_fmul_f32(_987, _988);
                        _118 = llvm_fmul_f32(_989, _990);
                        _119 = llvm_fmul_f32(_991, _992);
                        _993 = _115;
                        _994 = _116;
                        *(_993 as *mut core::ffi::c_float) = _994;
                        _995 = _117;
                        *(&mut *((*(_993 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _995;
                        _996 = _118;
                        *(&mut *((*(_993 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _996;
                        _997 = _119;
                        *(&mut *((*(_993 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _997;
                        _998 = _392;
                        *(&mut _503.field0 as *mut l_array_4_float) = _998.field0;
                        _999 = memcpy(
                            &mut _506 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _494 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _1000 = *(&mut _503.field0 as *mut l_array_4_float);
                        _1001 = *(&mut _506.field0 as *mut l_array_4_float);
                        *(&mut _260 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1000;
                        *(&mut _261 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1001;
                        _1002 = memcpy(
                            &mut _263 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _260 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _1003 = memcpy(
                            &mut _264 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _261 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _1004 =
                            *(&mut _263 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        _1005 =
                            *(&mut _264 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        *(&mut _258 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1004;
                        *(&mut _259 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1005;
                        _1006 = *(&mut _258 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _1007 = *(&mut _259 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _1008 = *(&mut *((*(&mut _258 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1009 = *(&mut *((*(&mut _259 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1010 = *(&mut *((*(&mut _258 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1011 = *(&mut *((*(&mut _259 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1012 = *(&mut *((*(&mut _258 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1013 = *(&mut *((*(&mut _259 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _175 =
                            &mut _257 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _176 = llvm_fmul_f32(_1006, _1007);
                        _177 = llvm_fmul_f32(_1008, _1009);
                        _178 = llvm_fmul_f32(_1010, _1011);
                        _179 = llvm_fmul_f32(_1012, _1013);
                        _1014 = _175;
                        _1015 = _176;
                        *(_1014 as *mut core::ffi::c_float) = _1015;
                        _1016 = _177;
                        *(&mut *((*(_1014 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1016;
                        _1017 = _178;
                        *(&mut *((*(_1014 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1017;
                        _1018 = _179;
                        *(&mut *((*(_1014 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1018;
                        _1019 = _257;
                        *(&mut _262 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1019.field0;
                        _1020 = memcpy(
                            &mut _265 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _262 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _1021 =
                            *(&mut _265 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        *(&mut _24 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1021;
                        _1022 = *(&mut _24 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _1023 = *(&mut *((*(&mut _24 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1024 = *(&mut *((*(&mut _24 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1025 = *(&mut *((*(&mut _24 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1026 = _491;
                        _491 = llvm_fadd_f32(
                            _1026,
                            llvm_fadd_f32(llvm_fadd_f32(_1022, _1023), llvm_fadd_f32(_1024, _1025)),
                        );
                        _1027 = _492;
                        _492 = llvm_add_u32(_1027, 1 as core::ffi::c_int as uint32_t);
                    }
                    _1028 = _453;
                    _1029 = _490;
                    _1030 = _465;
                    _ZL13insert_resultjfjPfPj(
                        _1028,
                        _1029,
                        _1030,
                        &mut *(_459.array)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float as *mut libc::c_void,
                        &mut *(_460.array)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as int64_t as isize)
                            as *mut uint32_t as *mut core::ffi::c_void,
                    );
                    _1031 = _453;
                    _1032 = _491;
                    _1033 = _465;
                    _ZL13insert_resultjfjPfPj(
                        _1031,
                        _1032,
                        _1033,
                        &mut *(_461.array)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float as *mut libc::c_void,
                        &mut *(_462.array)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as int64_t as isize)
                            as *mut uint32_t as *mut core::ffi::c_void,
                    );
                    _1034 = _464;
                    _464 = llvm_add_u32(_1034, 1 as core::ffi::c_int as uint32_t);
                }
            } else {
                _507 = 0 as core::ffi::c_int as uint32_t;
                loop {
                    _1035 = _507;
                    _1036 = _451;
                    if !(_1035 < _1036) {
                        break;
                    }
                    _1037 = _507;
                    _1038 = *(&mut *(_456.array)
                        .as_mut_ptr()
                        .offset(_1037 as uint64_t as int64_t as isize)
                        as *mut uint16_t);
                    _508 = _1038 as uint32_t;
                    _1039 = _448;
                    _1040 = _450;
                    _1041 = _508;
                    _1042 =
                        _ZNK21block_size_descriptor22get_raw_partition_infoEjj(_1039, _1040, _1041);
                    _509 = _1042;
                    _1043 = _509;
                    _1044 = _449;
                    _Z32compute_avgs_and_dirs_3_comp_rgbRK14partition_infoRK11image_blockP17partition_metrics(
                        _1043,
                        _1044,
                        &mut *(_510.array)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as int64_t as isize)
                            as *mut l_struct_struct_OC_partition_metrics
                            as *mut core::ffi::c_void,
                    );
                    _512 = 0 as core::ffi::c_int as uint32_t;
                    loop {
                        _1045 = _512;
                        _1046 = _450;
                        if !(_1045 < _1046) {
                            break;
                        }
                        _1047 = _512;
                        _513 = &mut *(_510.array)
                            .as_mut_ptr()
                            .offset(_1047 as uint64_t as int64_t as isize)
                            as *mut l_struct_struct_OC_partition_metrics
                            as *mut core::ffi::c_void;
                        _1048 = _512;
                        _514 = &mut *(_511.array)
                            .as_mut_ptr()
                            .offset(_1048 as uint64_t as int64_t as isize)
                            as *mut l_struct_struct_OC_partition_lines3
                            as *mut core::ffi::c_void;
                        _1049 = _513;
                        _1050 = _514;
                        _1051 = memcpy(
                            &mut (*(&mut (*(_1050 as *mut l_struct_struct_OC_partition_lines3))
                                .field0
                                as *mut l_struct_struct_OC_line3))
                                .field0
                                as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void,
                            &mut (*(_1049 as *mut l_struct_struct_OC_partition_metrics)).field0
                                as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _1052 = _513;
                        _1053 = memcpy(
                            &mut _516 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut (*(_1052 as *mut l_struct_struct_OC_partition_metrics)).field1
                                as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _245 = 0.577350259f64 as core::ffi::c_float;
                        _1054 = _245;
                        _1055 = _245;
                        _1056 = _245;
                        _190 =
                            &mut _244 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _191 = _1054;
                        _192 = _1055;
                        _193 = _1056;
                        _194 = 0 as core::ffi::c_int as libc::c_float;
                        _1057 = _190;
                        _1058 = _191;
                        *(_1057 as *mut core::ffi::c_float) = _1058;
                        _1059 = _192;
                        *(&mut *((*(_1057 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1059;
                        _1060 = _193;
                        *(&mut *((*(_1057 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1060;
                        _1061 = _194;
                        *(&mut *((*(_1057 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1061;
                        _1062 = _244;
                        *(&mut _517.field0 as *mut l_array_4_float) = _1062.field0;
                        _1063 = *(&mut _516.field0 as *mut l_array_4_float);
                        _1064 = *(&mut _517.field0 as *mut l_array_4_float);
                        *(&mut _431 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1063;
                        *(&mut _432 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1064;
                        _1065 = memcpy(
                            &mut _434 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _431 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _1066 = memcpy(
                            &mut _435 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _431 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _1067 =
                            *(&mut _434 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        _1068 =
                            *(&mut _435 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        *(&mut _360 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1067;
                        *(&mut _361 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1068;
                        _1069 = memcpy(
                            &mut _363 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _360 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _1070 = memcpy(
                            &mut _364 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _361 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _1071 =
                            *(&mut _363 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        _1072 =
                            *(&mut _364 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        *(&mut _357 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1071;
                        *(&mut _358 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1072;
                        _1073 = *(&mut _357 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _1074 = *(&mut _358 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _1075 = *(&mut *((*(&mut _357 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1076 = *(&mut *((*(&mut _358 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1077 = *(&mut *((*(&mut _357 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1078 = *(&mut *((*(&mut _358 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1079 = *(&mut *((*(&mut _357 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1080 = *(&mut *((*(&mut _358 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _140 =
                            &mut _356 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _141 = llvm_fmul_f32(_1073, _1074);
                        _142 = llvm_fmul_f32(_1075, _1076);
                        _143 = llvm_fmul_f32(_1077, _1078);
                        _144 = llvm_fmul_f32(_1079, _1080);
                        _1081 = _140;
                        _1082 = _141;
                        *(_1081 as *mut core::ffi::c_float) = _1082;
                        _1083 = _142;
                        *(&mut *((*(_1081 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1083;
                        _1084 = _143;
                        *(&mut *((*(_1081 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1084;
                        _1085 = _144;
                        *(&mut *((*(_1081 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1085;
                        _1086 = _356;
                        *(&mut _362 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1086.field0;
                        _1087 = memcpy(
                            &mut _365 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _362 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _1088 =
                            *(&mut _365 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        *(&mut _21 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1088;
                        _1089 = *(&mut _21 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _1090 = *(&mut *((*(&mut _21 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1091 = *(&mut *((*(&mut _21 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1092 = *(&mut *((*(&mut _21 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _318 =
                            &mut _359 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _319 =
                            llvm_fadd_f32(llvm_fadd_f32(_1089, _1090), llvm_fadd_f32(_1091, _1092));
                        _1093 = _318;
                        _1094 = _319;
                        *(_1093 as *mut core::ffi::c_float) = _1094;
                        _1095 = _319;
                        *(&mut *((*(_1093 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1095;
                        _1096 = _319;
                        *(&mut *((*(_1093 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1096;
                        _1097 = _319;
                        *(&mut *((*(_1093 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1097;
                        _1098 = _359;
                        *(&mut _433 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1098.field0;
                        _87 =
                            &mut _433 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _1099 = _87;
                        _1100 = *(_1099 as *mut core::ffi::c_float);
                        if llvm_fcmp_une(
                            _1100 as core::ffi::c_double,
                            0 as core::ffi::c_int as libc::c_double,
                        ) != 0
                        {
                            _1101 = memcpy(
                                &mut _436 as *mut l_struct_struct_OC_vfloat4
                                    as *mut core::ffi::c_void,
                                &mut _431 as *mut l_struct_struct_OC_vfloat4
                                    as *mut core::ffi::c_void,
                                16 as core::ffi::c_int as uint64_t,
                            );
                            _1102 = memcpy(
                                &mut _438 as *mut l_struct_struct_OC_vfloat4
                                    as *mut core::ffi::c_void,
                                &mut _433 as *mut l_struct_struct_OC_vfloat4
                                    as *mut core::ffi::c_void,
                                16 as core::ffi::c_int as uint64_t,
                            );
                            _1103 = *(&mut _438 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float);
                            *(&mut _39 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float) = _1103;
                            _1104 = *(&mut _39 as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_float);
                            _1105 = _ZSt4sqrtf(_1104);
                            _1106 = *(&mut *((*(&mut _39 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float))
                                .array)
                                .as_mut_ptr()
                                .offset(1 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float);
                            _1107 = _ZSt4sqrtf(_1106);
                            _1108 = *(&mut *((*(&mut _39 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float))
                                .array)
                                .as_mut_ptr()
                                .offset(2 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float);
                            _1109 = _ZSt4sqrtf(_1108);
                            _1110 = *(&mut *((*(&mut _39 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float))
                                .array)
                                .as_mut_ptr()
                                .offset(3 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float);
                            _1111 = _ZSt4sqrtf(_1110);
                            _33 = &mut _38 as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void;
                            _34 = _1105;
                            _35 = _1107;
                            _36 = _1109;
                            _37 = _1111;
                            _1112 = _33;
                            _1113 = _34;
                            *(_1112 as *mut core::ffi::c_float) = _1113;
                            _1114 = _35;
                            *(&mut *((*(_1112 as *mut l_array_4_float)).array)
                                .as_mut_ptr()
                                .offset(1 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float) = _1114;
                            _1115 = _36;
                            *(&mut *((*(_1112 as *mut l_array_4_float)).array)
                                .as_mut_ptr()
                                .offset(2 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float) = _1115;
                            _1116 = _37;
                            *(&mut *((*(_1112 as *mut l_array_4_float)).array)
                                .as_mut_ptr()
                                .offset(3 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float) = _1116;
                            _1117 = _38;
                            *(&mut _437 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float) = _1117.field0;
                            _1118 = *(&mut _436 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float);
                            _1119 = *(&mut _437 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float);
                            *(&mut _68 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float) = _1118;
                            *(&mut _69 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float) = _1119;
                            _1120 = *(&mut _68 as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_float);
                            _1121 = *(&mut _69 as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_float);
                            _1122 = *(&mut *((*(&mut _68 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float))
                                .array)
                                .as_mut_ptr()
                                .offset(1 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float);
                            _1123 = *(&mut *((*(&mut _69 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float))
                                .array)
                                .as_mut_ptr()
                                .offset(1 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float);
                            _1124 = *(&mut *((*(&mut _68 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float))
                                .array)
                                .as_mut_ptr()
                                .offset(2 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float);
                            _1125 = *(&mut *((*(&mut _69 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float))
                                .array)
                                .as_mut_ptr()
                                .offset(2 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float);
                            _1126 = *(&mut *((*(&mut _68 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float))
                                .array)
                                .as_mut_ptr()
                                .offset(3 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float);
                            _1127 = *(&mut *((*(&mut _69 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float))
                                .array)
                                .as_mut_ptr()
                                .offset(3 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float);
                            _62 = &mut _67 as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void;
                            _63 = llvm_fdiv_f32(_1120, _1121);
                            _64 = llvm_fdiv_f32(_1122, _1123);
                            _65 = llvm_fdiv_f32(_1124, _1125);
                            _66 = llvm_fdiv_f32(_1126, _1127);
                            _1128 = _62;
                            _1129 = _63;
                            *(_1128 as *mut core::ffi::c_float) = _1129;
                            _1130 = _64;
                            *(&mut *((*(_1128 as *mut l_array_4_float)).array)
                                .as_mut_ptr()
                                .offset(1 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float) = _1130;
                            _1131 = _65;
                            *(&mut *((*(_1128 as *mut l_array_4_float)).array)
                                .as_mut_ptr()
                                .offset(2 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float) = _1131;
                            _1132 = _66;
                            *(&mut *((*(_1128 as *mut l_array_4_float)).array)
                                .as_mut_ptr()
                                .offset(3 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float) = _1132;
                            _1133 = _67;
                            *(&mut _430 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float) = _1133.field0;
                        } else {
                            _1134 = memcpy(
                                &mut _430 as *mut l_struct_struct_OC_vfloat4
                                    as *mut core::ffi::c_void,
                                &mut _432 as *mut l_struct_struct_OC_vfloat4
                                    as *mut core::ffi::c_void,
                                16 as core::ffi::c_int as uint64_t,
                            );
                        }
                        _1135 = _430;
                        *(&mut _515.field0 as *mut l_array_4_float) = _1135.field0;
                        _1136 = _514;
                        _1137 = memcpy(
                            &mut (*(&mut (*(_1136 as *mut l_struct_struct_OC_partition_lines3))
                                .field0
                                as *mut l_struct_struct_OC_line3))
                                .field1
                                as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void,
                            &mut _515 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _326 =
                            &mut _334 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _327 = 0 as core::ffi::c_int as libc::c_float;
                        _1138 = _326;
                        _1139 = _327;
                        *(_1138 as *mut core::ffi::c_float) = _1139;
                        _1140 = _327;
                        *(&mut *((*(_1138 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1140;
                        _1141 = _327;
                        *(&mut *((*(_1138 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1141;
                        _1142 = _327;
                        *(&mut *((*(_1138 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1142;
                        _1143 = _334;
                        *(&mut _518.field0 as *mut l_array_4_float) = _1143.field0;
                        _1144 = _514;
                        _1145 = memcpy(
                            &mut (*(&mut (*(_1144 as *mut l_struct_struct_OC_partition_lines3))
                                .field1
                                as *mut l_struct_struct_OC_line3))
                                .field0
                                as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void,
                            &mut _518 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _1146 = _513;
                        _1147 = memcpy(
                            &mut _520 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut (*(_1146 as *mut l_struct_struct_OC_partition_metrics)).field0
                                as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _247 = 0.577350259f64 as core::ffi::c_float;
                        _1148 = _247;
                        _1149 = _247;
                        _1150 = _247;
                        _185 =
                            &mut _246 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _186 = _1148;
                        _187 = _1149;
                        _188 = _1150;
                        _189 = 0 as core::ffi::c_int as libc::c_float;
                        _1151 = _185;
                        _1152 = _186;
                        *(_1151 as *mut core::ffi::c_float) = _1152;
                        _1153 = _187;
                        *(&mut *((*(_1151 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1153;
                        _1154 = _188;
                        *(&mut *((*(_1151 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1154;
                        _1155 = _189;
                        *(&mut *((*(_1151 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1155;
                        _1156 = _246;
                        *(&mut _521.field0 as *mut l_array_4_float) = _1156.field0;
                        _1157 = *(&mut _520.field0 as *mut l_array_4_float);
                        _1158 = *(&mut _521.field0 as *mut l_array_4_float);
                        *(&mut _440 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1157;
                        *(&mut _441 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1158;
                        _1159 = memcpy(
                            &mut _443 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _440 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _1160 = memcpy(
                            &mut _444 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _440 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _1161 =
                            *(&mut _443 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        _1162 =
                            *(&mut _444 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        *(&mut _350 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1161;
                        *(&mut _351 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1162;
                        _1163 = memcpy(
                            &mut _353 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _350 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _1164 = memcpy(
                            &mut _354 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _351 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _1165 =
                            *(&mut _353 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        _1166 =
                            *(&mut _354 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        *(&mut _347 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1165;
                        *(&mut _348 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1166;
                        _1167 = *(&mut _347 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _1168 = *(&mut _348 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _1169 = *(&mut *((*(&mut _347 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1170 = *(&mut *((*(&mut _348 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1171 = *(&mut *((*(&mut _347 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1172 = *(&mut *((*(&mut _348 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1173 = *(&mut *((*(&mut _347 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1174 = *(&mut *((*(&mut _348 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _145 =
                            &mut _346 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _146 = llvm_fmul_f32(_1167, _1168);
                        _147 = llvm_fmul_f32(_1169, _1170);
                        _148 = llvm_fmul_f32(_1171, _1172);
                        _149 = llvm_fmul_f32(_1173, _1174);
                        _1175 = _145;
                        _1176 = _146;
                        *(_1175 as *mut core::ffi::c_float) = _1176;
                        _1177 = _147;
                        *(&mut *((*(_1175 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1177;
                        _1178 = _148;
                        *(&mut *((*(_1175 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1178;
                        _1179 = _149;
                        *(&mut *((*(_1175 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1179;
                        _1180 = _346;
                        *(&mut _352 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1180.field0;
                        _1181 = memcpy(
                            &mut _355 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _352 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _1182 =
                            *(&mut _355 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        *(&mut _22 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1182;
                        _1183 = *(&mut _22 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _1184 = *(&mut *((*(&mut _22 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1185 = *(&mut *((*(&mut _22 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1186 = *(&mut *((*(&mut _22 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _320 =
                            &mut _349 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _321 =
                            llvm_fadd_f32(llvm_fadd_f32(_1183, _1184), llvm_fadd_f32(_1185, _1186));
                        _1187 = _320;
                        _1188 = _321;
                        *(_1187 as *mut core::ffi::c_float) = _1188;
                        _1189 = _321;
                        *(&mut *((*(_1187 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1189;
                        _1190 = _321;
                        *(&mut *((*(_1187 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1190;
                        _1191 = _321;
                        *(&mut *((*(_1187 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1191;
                        _1192 = _349;
                        *(&mut _442 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1192.field0;
                        _86 =
                            &mut _442 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _1193 = _86;
                        _1194 = *(_1193 as *mut core::ffi::c_float);
                        if llvm_fcmp_une(
                            _1194 as core::ffi::c_double,
                            0 as core::ffi::c_int as libc::c_double,
                        ) != 0
                        {
                            _1195 = memcpy(
                                &mut _445 as *mut l_struct_struct_OC_vfloat4
                                    as *mut core::ffi::c_void,
                                &mut _440 as *mut l_struct_struct_OC_vfloat4
                                    as *mut core::ffi::c_void,
                                16 as core::ffi::c_int as uint64_t,
                            );
                            _1196 = memcpy(
                                &mut _447 as *mut l_struct_struct_OC_vfloat4
                                    as *mut core::ffi::c_void,
                                &mut _442 as *mut l_struct_struct_OC_vfloat4
                                    as *mut core::ffi::c_void,
                                16 as core::ffi::c_int as uint64_t,
                            );
                            _1197 = *(&mut _447 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float);
                            *(&mut _32 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float) = _1197;
                            _1198 = *(&mut _32 as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_float);
                            _1199 = _ZSt4sqrtf(_1198);
                            _1200 = *(&mut *((*(&mut _32 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float))
                                .array)
                                .as_mut_ptr()
                                .offset(1 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float);
                            _1201 = _ZSt4sqrtf(_1200);
                            _1202 = *(&mut *((*(&mut _32 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float))
                                .array)
                                .as_mut_ptr()
                                .offset(2 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float);
                            _1203 = _ZSt4sqrtf(_1202);
                            _1204 = *(&mut *((*(&mut _32 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float))
                                .array)
                                .as_mut_ptr()
                                .offset(3 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float);
                            _1205 = _ZSt4sqrtf(_1204);
                            _26 = &mut _31 as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void;
                            _27 = _1199;
                            _28 = _1201;
                            _29 = _1203;
                            _30 = _1205;
                            _1206 = _26;
                            _1207 = _27;
                            *(_1206 as *mut core::ffi::c_float) = _1207;
                            _1208 = _28;
                            *(&mut *((*(_1206 as *mut l_array_4_float)).array)
                                .as_mut_ptr()
                                .offset(1 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float) = _1208;
                            _1209 = _29;
                            *(&mut *((*(_1206 as *mut l_array_4_float)).array)
                                .as_mut_ptr()
                                .offset(2 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float) = _1209;
                            _1210 = _30;
                            *(&mut *((*(_1206 as *mut l_array_4_float)).array)
                                .as_mut_ptr()
                                .offset(3 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float) = _1210;
                            _1211 = _31;
                            *(&mut _446 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float) = _1211.field0;
                            _1212 = *(&mut _445 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float);
                            _1213 = *(&mut _446 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float);
                            *(&mut _60 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float) = _1212;
                            *(&mut _61 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float) = _1213;
                            _1214 = *(&mut _60 as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_float);
                            _1215 = *(&mut _61 as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_float);
                            _1216 = *(&mut *((*(&mut _60 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float))
                                .array)
                                .as_mut_ptr()
                                .offset(1 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float);
                            _1217 = *(&mut *((*(&mut _61 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float))
                                .array)
                                .as_mut_ptr()
                                .offset(1 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float);
                            _1218 = *(&mut *((*(&mut _60 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float))
                                .array)
                                .as_mut_ptr()
                                .offset(2 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float);
                            _1219 = *(&mut *((*(&mut _61 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float))
                                .array)
                                .as_mut_ptr()
                                .offset(2 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float);
                            _1220 = *(&mut *((*(&mut _60 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float))
                                .array)
                                .as_mut_ptr()
                                .offset(3 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float);
                            _1221 = *(&mut *((*(&mut _61 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float))
                                .array)
                                .as_mut_ptr()
                                .offset(3 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float);
                            _54 = &mut _59 as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void;
                            _55 = llvm_fdiv_f32(_1214, _1215);
                            _56 = llvm_fdiv_f32(_1216, _1217);
                            _57 = llvm_fdiv_f32(_1218, _1219);
                            _58 = llvm_fdiv_f32(_1220, _1221);
                            _1222 = _54;
                            _1223 = _55;
                            *(_1222 as *mut core::ffi::c_float) = _1223;
                            _1224 = _56;
                            *(&mut *((*(_1222 as *mut l_array_4_float)).array)
                                .as_mut_ptr()
                                .offset(1 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float) = _1224;
                            _1225 = _57;
                            *(&mut *((*(_1222 as *mut l_array_4_float)).array)
                                .as_mut_ptr()
                                .offset(2 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float) = _1225;
                            _1226 = _58;
                            *(&mut *((*(_1222 as *mut l_array_4_float)).array)
                                .as_mut_ptr()
                                .offset(3 as core::ffi::c_int as int64_t as isize)
                                as *mut core::ffi::c_float) = _1226;
                            _1227 = _59;
                            *(&mut _439 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float) = _1227.field0;
                        } else {
                            _1228 = memcpy(
                                &mut _439 as *mut l_struct_struct_OC_vfloat4
                                    as *mut core::ffi::c_void,
                                &mut _441 as *mut l_struct_struct_OC_vfloat4
                                    as *mut core::ffi::c_void,
                                16 as core::ffi::c_int as uint64_t,
                            );
                        }
                        _1229 = _439;
                        *(&mut _519.field0 as *mut l_array_4_float) = _1229.field0;
                        _1230 = _514;
                        _1231 = memcpy(
                            &mut (*(&mut (*(_1230 as *mut l_struct_struct_OC_partition_lines3))
                                .field1
                                as *mut l_struct_struct_OC_line3))
                                .field1
                                as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void,
                            &mut _519 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _1232 = _514;
                        _1233 = memcpy(
                            &mut _523 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut (*(&mut (*(_1232 as *mut l_struct_struct_OC_partition_lines3))
                                .field0
                                as *mut l_struct_struct_OC_line3))
                                .field0
                                as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _1234 = _514;
                        _1235 = memcpy(
                            &mut _525 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut (*(&mut (*(_1234 as *mut l_struct_struct_OC_partition_lines3))
                                .field0
                                as *mut l_struct_struct_OC_line3))
                                .field1
                                as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _1236 = _514;
                        _1237 = memcpy(
                            &mut _527 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut (*(&mut (*(_1236 as *mut l_struct_struct_OC_partition_lines3))
                                .field0
                                as *mut l_struct_struct_OC_line3))
                                .field0
                                as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _1238 = _514;
                        _1239 = memcpy(
                            &mut _528 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut (*(&mut (*(_1238 as *mut l_struct_struct_OC_partition_lines3))
                                .field0
                                as *mut l_struct_struct_OC_line3))
                                .field1
                                as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _1240 = *(&mut _527.field0 as *mut l_array_4_float);
                        _1241 = *(&mut _528.field0 as *mut l_array_4_float);
                        *(&mut _237 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1240;
                        *(&mut _238 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1241;
                        _1242 = memcpy(
                            &mut _240 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _237 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _1243 = memcpy(
                            &mut _241 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _238 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _1244 =
                            *(&mut _240 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        _1245 =
                            *(&mut _241 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        *(&mut _234 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1244;
                        *(&mut _235 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1245;
                        _1246 = *(&mut _234 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _1247 = *(&mut _235 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _1248 = *(&mut *((*(&mut _234 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1249 = *(&mut *((*(&mut _235 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1250 = *(&mut *((*(&mut _234 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1251 = *(&mut *((*(&mut _235 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1252 = *(&mut *((*(&mut _234 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1253 = *(&mut *((*(&mut _235 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _195 =
                            &mut _233 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _196 = llvm_fmul_f32(_1246, _1247);
                        _197 = llvm_fmul_f32(_1248, _1249);
                        _198 = llvm_fmul_f32(_1250, _1251);
                        _199 = llvm_fmul_f32(_1252, _1253);
                        _1254 = _195;
                        _1255 = _196;
                        *(_1254 as *mut core::ffi::c_float) = _1255;
                        _1256 = _197;
                        *(&mut *((*(_1254 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1256;
                        _1257 = _198;
                        *(&mut *((*(_1254 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1257;
                        _1258 = _199;
                        *(&mut *((*(_1254 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1258;
                        _1259 = _233;
                        *(&mut _239 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1259.field0;
                        _1260 = memcpy(
                            &mut _243 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _239 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _1261 =
                            *(&mut _243 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        *(&mut _14 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1261;
                        _13 = &mut _14 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _1262 = _13;
                        _1263 = *(_1262 as *mut core::ffi::c_float);
                        _12 = &mut _14 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _1264 = _12;
                        _1265 = *(&mut *((*(_1264 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _9 = &mut _14 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _1266 = _9;
                        _1267 = *(&mut *((*(_1266 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _242 = llvm_fadd_f32(llvm_fadd_f32(_1263, _1265), _1267);
                        _1268 = _242;
                        _1269 = _242;
                        _1270 = _242;
                        _200 =
                            &mut _236 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _201 = _1268;
                        _202 = _1269;
                        _203 = _1270;
                        _204 = 0 as core::ffi::c_int as libc::c_float;
                        _1271 = _200;
                        _1272 = _201;
                        *(_1271 as *mut core::ffi::c_float) = _1272;
                        _1273 = _202;
                        *(&mut *((*(_1271 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1273;
                        _1274 = _203;
                        *(&mut *((*(_1271 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1274;
                        _1275 = _204;
                        *(&mut *((*(_1271 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1275;
                        _1276 = _236;
                        *(&mut _526.field0 as *mut l_array_4_float) = _1276.field0;
                        _1277 = *(&mut _525.field0 as *mut l_array_4_float);
                        _1278 = *(&mut _526.field0 as *mut l_array_4_float);
                        *(&mut _396 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1277;
                        *(&mut _397 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1278;
                        _1279 = *(&mut _396 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _1280 = *(&mut _397 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _1281 = *(&mut *((*(&mut _396 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1282 = *(&mut *((*(&mut _397 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1283 = *(&mut *((*(&mut _396 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1284 = *(&mut *((*(&mut _397 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1285 = *(&mut *((*(&mut _396 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1286 = *(&mut *((*(&mut _397 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _110 =
                            &mut _395 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _111 = llvm_fmul_f32(_1279, _1280);
                        _112 = llvm_fmul_f32(_1281, _1282);
                        _113 = llvm_fmul_f32(_1283, _1284);
                        _114 = llvm_fmul_f32(_1285, _1286);
                        _1287 = _110;
                        _1288 = _111;
                        *(_1287 as *mut core::ffi::c_float) = _1288;
                        _1289 = _112;
                        *(&mut *((*(_1287 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1289;
                        _1290 = _113;
                        *(&mut *((*(_1287 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1290;
                        _1291 = _114;
                        *(&mut *((*(_1287 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1291;
                        _1292 = _395;
                        *(&mut _524.field0 as *mut l_array_4_float) = _1292.field0;
                        _1293 = *(&mut _523.field0 as *mut l_array_4_float);
                        _1294 = *(&mut _524.field0 as *mut l_array_4_float);
                        *(&mut _408 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1293;
                        *(&mut _409 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1294;
                        _1295 = *(&mut _408 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _1296 = *(&mut _409 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _1297 = *(&mut *((*(&mut _408 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1298 = *(&mut *((*(&mut _409 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1299 = *(&mut *((*(&mut _408 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1300 = *(&mut *((*(&mut _409 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1301 = *(&mut *((*(&mut _408 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1302 = *(&mut *((*(&mut _409 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _90 =
                            &mut _407 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _91 = llvm_fsub_f32(_1295, _1296);
                        _92 = llvm_fsub_f32(_1297, _1298);
                        _93 = llvm_fsub_f32(_1299, _1300);
                        _94 = llvm_fsub_f32(_1301, _1302);
                        _1303 = _90;
                        _1304 = _91;
                        *(_1303 as *mut core::ffi::c_float) = _1304;
                        _1305 = _92;
                        *(&mut *((*(_1303 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1305;
                        _1306 = _93;
                        *(&mut *((*(_1303 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1306;
                        _1307 = _94;
                        *(&mut *((*(_1303 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1307;
                        _1308 = _407;
                        *(&mut _522.field0 as *mut l_array_4_float) = _1308.field0;
                        _1309 = _514;
                        _1310 = memcpy(
                            &mut (*(&mut (*(_1309 as *mut l_struct_struct_OC_partition_lines3))
                                .field2
                                as *mut l_struct_struct_OC_processed_line3))
                                .field0
                                as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void,
                            &mut _522 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _1311 = _514;
                        _1312 = _514;
                        _1313 = memcpy(
                            &mut (*(&mut (*(_1312 as *mut l_struct_struct_OC_partition_lines3))
                                .field2
                                as *mut l_struct_struct_OC_processed_line3))
                                .field1
                                as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void,
                            &mut (*(&mut (*(_1311 as *mut l_struct_struct_OC_partition_lines3))
                                .field0
                                as *mut l_struct_struct_OC_line3))
                                .field1
                                as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _324 =
                            &mut _335 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _325 = 0 as core::ffi::c_int as libc::c_float;
                        _1314 = _324;
                        _1315 = _325;
                        *(_1314 as *mut core::ffi::c_float) = _1315;
                        _1316 = _325;
                        *(&mut *((*(_1314 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1316;
                        _1317 = _325;
                        *(&mut *((*(_1314 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1317;
                        _1318 = _325;
                        *(&mut *((*(_1314 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1318;
                        _1319 = _335;
                        *(&mut _529.field0 as *mut l_array_4_float) = _1319.field0;
                        _1320 = _514;
                        _1321 = memcpy(
                            &mut (*(&mut (*(_1320 as *mut l_struct_struct_OC_partition_lines3))
                                .field3
                                as *mut l_struct_struct_OC_processed_line3))
                                .field0
                                as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void,
                            &mut _529 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _1322 = _514;
                        _1323 = _514;
                        _1324 = memcpy(
                            &mut (*(&mut (*(_1323 as *mut l_struct_struct_OC_partition_lines3))
                                .field3
                                as *mut l_struct_struct_OC_processed_line3))
                                .field1
                                as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void,
                            &mut (*(&mut (*(_1322 as *mut l_struct_struct_OC_partition_lines3))
                                .field1
                                as *mut l_struct_struct_OC_line3))
                                .field1
                                as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _1325 = _512;
                        _512 = llvm_add_u32(_1325, 1 as core::ffi::c_int as uint32_t);
                    }
                    _530 = 0 as core::ffi::c_int as libc::c_float;
                    _531 = 0 as core::ffi::c_int as libc::c_float;
                    _1326 = _509;
                    _1327 = _449;
                    _Z25compute_error_squared_rgbRK14partition_infoRK11image_blockP16partition_lines3RfS7_(
                        _1326,
                        _1327,
                        &mut *(_511.array)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as int64_t as isize)
                            as *mut l_struct_struct_OC_partition_lines3
                            as *mut core::ffi::c_void,
                        &mut _530 as *mut core::ffi::c_float as *mut libc::c_void,
                        &mut _531 as *mut core::ffi::c_float as *mut libc::c_void,
                    );
                    _532 = 0 as core::ffi::c_int as uint32_t;
                    loop {
                        _1328 = _532;
                        _1329 = _450;
                        if !(_1328 < _1329) {
                            break;
                        }
                        _1330 = _532;
                        _533 = &mut *(_511.array)
                            .as_mut_ptr()
                            .offset(_1330 as uint64_t as int64_t as isize)
                            as *mut l_struct_struct_OC_partition_lines3
                            as *mut core::ffi::c_void;
                        _1331 = _509;
                        _1332 = _532;
                        _1333 = *(&mut *((*(&mut (*(_1331
                            as *mut l_struct_struct_OC_partition_info))
                            .field2
                            as *mut l_array_4_uint8_t))
                            .array)
                            .as_mut_ptr()
                            .offset(_1332 as uint64_t as int64_t as isize)
                            as *mut uint8_t);
                        _534 = _1333 as core::ffi::c_float;
                        _1334 = _534;
                        _1335 = _455;
                        _306 =
                            &mut _535 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _307 = llvm_fmul_f32(_1334, _1335);
                        _1336 = _306;
                        _1337 = _307;
                        *(_1336 as *mut core::ffi::c_float) = _1337;
                        _1338 = _307;
                        *(&mut *((*(_1336 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1338;
                        _1339 = _307;
                        *(&mut *((*(_1336 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1339;
                        _1340 = _307;
                        *(&mut *((*(_1336 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1340;
                        _1341 = _533;
                        _1342 = memcpy(
                            &mut _537 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut (*(&mut (*(_1341 as *mut l_struct_struct_OC_partition_lines3))
                                .field0
                                as *mut l_struct_struct_OC_line3))
                                .field1
                                as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _1343 = _533;
                        _1344 = *(&mut (*(_1343 as *mut l_struct_struct_OC_partition_lines3)).field4
                            as *mut core::ffi::c_float);
                        _1345 = *(&mut _537.field0 as *mut l_array_4_float);
                        *(&mut _292 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1345;
                        _293 = _1344;
                        _1346 = memcpy(
                            &mut _294 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _292 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _1347 = _293;
                        _286 =
                            &mut _295 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _287 = _1347;
                        _1348 = _286;
                        _1349 = _287;
                        *(_1348 as *mut core::ffi::c_float) = _1349;
                        _1350 = _287;
                        *(&mut *((*(_1348 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1350;
                        _1351 = _287;
                        *(&mut *((*(_1348 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1351;
                        _1352 = _287;
                        *(&mut *((*(_1348 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1352;
                        _1353 =
                            *(&mut _294 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        _1354 =
                            *(&mut _295 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        *(&mut _289 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1353;
                        *(&mut _290 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1354;
                        _1355 = *(&mut _289 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _1356 = *(&mut _290 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _1357 = *(&mut *((*(&mut _289 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1358 = *(&mut *((*(&mut _290 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1359 = *(&mut *((*(&mut _289 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1360 = *(&mut *((*(&mut _290 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1361 = *(&mut *((*(&mut _289 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1362 = *(&mut *((*(&mut _290 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _160 =
                            &mut _288 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _161 = llvm_fmul_f32(_1355, _1356);
                        _162 = llvm_fmul_f32(_1357, _1358);
                        _163 = llvm_fmul_f32(_1359, _1360);
                        _164 = llvm_fmul_f32(_1361, _1362);
                        _1363 = _160;
                        _1364 = _161;
                        *(_1363 as *mut core::ffi::c_float) = _1364;
                        _1365 = _162;
                        *(&mut *((*(_1363 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1365;
                        _1366 = _163;
                        *(&mut *((*(_1363 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1366;
                        _1367 = _164;
                        *(&mut *((*(_1363 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1367;
                        _1368 = _288;
                        *(&mut _291 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1368.field0;
                        _1369 = _291;
                        *(&mut _536.field0 as *mut l_array_4_float) = _1369.field0;
                        _1370 = _533;
                        _1371 = memcpy(
                            &mut _539 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut (*(&mut (*(_1370 as *mut l_struct_struct_OC_partition_lines3))
                                .field1
                                as *mut l_struct_struct_OC_line3))
                                .field1
                                as *mut l_struct_struct_OC_vfloat4
                                as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _1372 = _533;
                        _1373 = *(&mut (*(_1372 as *mut l_struct_struct_OC_partition_lines3)).field4
                            as *mut core::ffi::c_float);
                        _1374 = *(&mut _539.field0 as *mut l_array_4_float);
                        *(&mut _302 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1374;
                        _303 = _1373;
                        _1375 = memcpy(
                            &mut _304 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _302 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _1376 = _303;
                        _296 =
                            &mut _305 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _297 = _1376;
                        _1377 = _296;
                        _1378 = _297;
                        *(_1377 as *mut core::ffi::c_float) = _1378;
                        _1379 = _297;
                        *(&mut *((*(_1377 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1379;
                        _1380 = _297;
                        *(&mut *((*(_1377 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1380;
                        _1381 = _297;
                        *(&mut *((*(_1377 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1381;
                        _1382 =
                            *(&mut _304 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        _1383 =
                            *(&mut _305 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        *(&mut _299 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1382;
                        *(&mut _300 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1383;
                        _1384 = *(&mut _299 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _1385 = *(&mut _300 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _1386 = *(&mut *((*(&mut _299 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1387 = *(&mut *((*(&mut _300 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1388 = *(&mut *((*(&mut _299 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1389 = *(&mut *((*(&mut _300 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1390 = *(&mut *((*(&mut _299 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1391 = *(&mut *((*(&mut _300 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _155 =
                            &mut _298 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _156 = llvm_fmul_f32(_1384, _1385);
                        _157 = llvm_fmul_f32(_1386, _1387);
                        _158 = llvm_fmul_f32(_1388, _1389);
                        _159 = llvm_fmul_f32(_1390, _1391);
                        _1392 = _155;
                        _1393 = _156;
                        *(_1392 as *mut core::ffi::c_float) = _1393;
                        _1394 = _157;
                        *(&mut *((*(_1392 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1394;
                        _1395 = _158;
                        *(&mut *((*(_1392 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1395;
                        _1396 = _159;
                        *(&mut *((*(_1392 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1396;
                        _1397 = _298;
                        *(&mut _301 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1397.field0;
                        _1398 = _301;
                        *(&mut _538.field0 as *mut l_array_4_float) = _1398.field0;
                        _1399 = memcpy(
                            &mut _541 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _536 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _1400 = memcpy(
                            &mut _542 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _536 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _1401 = *(&mut _541.field0 as *mut l_array_4_float);
                        _1402 = *(&mut _542.field0 as *mut l_array_4_float);
                        *(&mut _399 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1401;
                        *(&mut _400 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1402;
                        _1403 = *(&mut _399 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _1404 = *(&mut _400 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _1405 = *(&mut *((*(&mut _399 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1406 = *(&mut *((*(&mut _400 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1407 = *(&mut *((*(&mut _399 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1408 = *(&mut *((*(&mut _400 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1409 = *(&mut *((*(&mut _399 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1410 = *(&mut *((*(&mut _400 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _105 =
                            &mut _398 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _106 = llvm_fmul_f32(_1403, _1404);
                        _107 = llvm_fmul_f32(_1405, _1406);
                        _108 = llvm_fmul_f32(_1407, _1408);
                        _109 = llvm_fmul_f32(_1409, _1410);
                        _1411 = _105;
                        _1412 = _106;
                        *(_1411 as *mut core::ffi::c_float) = _1412;
                        _1413 = _107;
                        *(&mut *((*(_1411 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1413;
                        _1414 = _108;
                        *(&mut *((*(_1411 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1414;
                        _1415 = _109;
                        *(&mut *((*(_1411 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1415;
                        _1416 = _398;
                        *(&mut _540.field0 as *mut l_array_4_float) = _1416.field0;
                        _1417 = memcpy(
                            &mut _543 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _535 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _1418 = *(&mut _540.field0 as *mut l_array_4_float);
                        _1419 = *(&mut _543.field0 as *mut l_array_4_float);
                        *(&mut _218 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1418;
                        *(&mut _219 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1419;
                        _1420 = memcpy(
                            &mut _221 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _218 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _1421 = memcpy(
                            &mut _222 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _219 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _1422 =
                            *(&mut _221 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        _1423 =
                            *(&mut _222 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        *(&mut _216 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1422;
                        *(&mut _217 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1423;
                        _1424 = *(&mut _216 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _1425 = *(&mut _217 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _1426 = *(&mut *((*(&mut _216 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1427 = *(&mut *((*(&mut _217 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1428 = *(&mut *((*(&mut _216 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1429 = *(&mut *((*(&mut _217 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1430 = *(&mut *((*(&mut _216 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1431 = *(&mut *((*(&mut _217 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _210 =
                            &mut _215 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _211 = llvm_fmul_f32(_1424, _1425);
                        _212 = llvm_fmul_f32(_1426, _1427);
                        _213 = llvm_fmul_f32(_1428, _1429);
                        _214 = llvm_fmul_f32(_1430, _1431);
                        _1432 = _210;
                        _1433 = _211;
                        *(_1432 as *mut core::ffi::c_float) = _1433;
                        _1434 = _212;
                        *(&mut *((*(_1432 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1434;
                        _1435 = _213;
                        *(&mut *((*(_1432 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1435;
                        _1436 = _214;
                        *(&mut *((*(_1432 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1436;
                        _1437 = _215;
                        *(&mut _220 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1437.field0;
                        _1438 = memcpy(
                            &mut _223 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _220 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _1439 =
                            *(&mut _223 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        *(&mut _18 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1439;
                        _17 = &mut _18 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _1440 = _17;
                        _1441 = *(_1440 as *mut core::ffi::c_float);
                        _10 = &mut _18 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _1442 = _10;
                        _1443 = *(&mut *((*(_1442 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _7 = &mut _18 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _1444 = _7;
                        _1445 = *(&mut *((*(_1444 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1446 = _530;
                        _530 =
                            llvm_fadd_f32(_1446, llvm_fadd_f32(llvm_fadd_f32(_1441, _1443), _1445));
                        _1447 = memcpy(
                            &mut _545 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _538 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _1448 = memcpy(
                            &mut _546 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _538 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _1449 = *(&mut _545.field0 as *mut l_array_4_float);
                        _1450 = *(&mut _546.field0 as *mut l_array_4_float);
                        *(&mut _402 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1449;
                        *(&mut _403 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1450;
                        _1451 = *(&mut _402 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _1452 = *(&mut _403 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _1453 = *(&mut *((*(&mut _402 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1454 = *(&mut *((*(&mut _403 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1455 = *(&mut *((*(&mut _402 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1456 = *(&mut *((*(&mut _403 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1457 = *(&mut *((*(&mut _402 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1458 = *(&mut *((*(&mut _403 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _100 =
                            &mut _401 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _101 = llvm_fmul_f32(_1451, _1452);
                        _102 = llvm_fmul_f32(_1453, _1454);
                        _103 = llvm_fmul_f32(_1455, _1456);
                        _104 = llvm_fmul_f32(_1457, _1458);
                        _1459 = _100;
                        _1460 = _101;
                        *(_1459 as *mut core::ffi::c_float) = _1460;
                        _1461 = _102;
                        *(&mut *((*(_1459 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1461;
                        _1462 = _103;
                        *(&mut *((*(_1459 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1462;
                        _1463 = _104;
                        *(&mut *((*(_1459 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1463;
                        _1464 = _401;
                        *(&mut _544.field0 as *mut l_array_4_float) = _1464.field0;
                        _1465 = memcpy(
                            &mut _547 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _535 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _1466 = *(&mut _544.field0 as *mut l_array_4_float);
                        _1467 = *(&mut _547.field0 as *mut l_array_4_float);
                        *(&mut _227 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1466;
                        *(&mut _228 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1467;
                        _1468 = memcpy(
                            &mut _230 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _227 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _1469 = memcpy(
                            &mut _231 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _228 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _1470 =
                            *(&mut _230 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        _1471 =
                            *(&mut _231 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        *(&mut _225 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1470;
                        *(&mut _226 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1471;
                        _1472 = *(&mut _225 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _1473 = *(&mut _226 as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_float);
                        _1474 = *(&mut *((*(&mut _225 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1475 = *(&mut *((*(&mut _226 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1476 = *(&mut *((*(&mut _225 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1477 = *(&mut *((*(&mut _226 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1478 = *(&mut *((*(&mut _225 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1479 = *(&mut *((*(&mut _226 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _205 =
                            &mut _224 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _206 = llvm_fmul_f32(_1472, _1473);
                        _207 = llvm_fmul_f32(_1474, _1475);
                        _208 = llvm_fmul_f32(_1476, _1477);
                        _209 = llvm_fmul_f32(_1478, _1479);
                        _1480 = _205;
                        _1481 = _206;
                        *(_1480 as *mut core::ffi::c_float) = _1481;
                        _1482 = _207;
                        *(&mut *((*(_1480 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1482;
                        _1483 = _208;
                        *(&mut *((*(_1480 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1483;
                        _1484 = _209;
                        *(&mut *((*(_1480 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(3 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float) = _1484;
                        _1485 = _224;
                        *(&mut _229 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1485.field0;
                        _1486 = memcpy(
                            &mut _232 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            &mut _229 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                            16 as core::ffi::c_int as uint64_t,
                        );
                        _1487 =
                            *(&mut _232 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                        *(&mut _16 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _1487;
                        _15 = &mut _16 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _1488 = _15;
                        _1489 = *(_1488 as *mut core::ffi::c_float);
                        _11 = &mut _16 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _1490 = _11;
                        _1491 = *(&mut *((*(_1490 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _8 = &mut _16 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                        _1492 = _8;
                        _1493 = *(&mut *((*(_1492 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float);
                        _1494 = _531;
                        _531 =
                            llvm_fadd_f32(_1494, llvm_fadd_f32(llvm_fadd_f32(_1489, _1491), _1493));
                        _1495 = _532;
                        _532 = llvm_add_u32(_1495, 1 as core::ffi::c_int as uint32_t);
                    }
                    _1496 = _453;
                    _1497 = _530;
                    _1498 = _508;
                    _ZL13insert_resultjfjPfPj(
                        _1496,
                        _1497,
                        _1498,
                        &mut *(_459.array)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float as *mut libc::c_void,
                        &mut *(_460.array)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as int64_t as isize)
                            as *mut uint32_t as *mut core::ffi::c_void,
                    );
                    _1499 = _453;
                    _1500 = _531;
                    _1501 = _508;
                    _ZL13insert_resultjfjPfPj(
                        _1499,
                        _1500,
                        _1501,
                        &mut *(_461.array)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as int64_t as isize)
                            as *mut core::ffi::c_float as *mut libc::c_void,
                        &mut *(_462.array)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as int64_t as isize)
                            as *mut uint32_t as *mut core::ffi::c_void,
                    );
                    _1502 = _507;
                    _507 = llvm_add_u32(_1502, 1 as core::ffi::c_int as uint32_t);
                }
            }
            _549 = 0 as core::ffi::c_int as uint32_t;
            loop {
                _1503 = _549;
                _1504 = _453;
                if !(_1503 < _1504) {
                    break;
                }
                _1505 = _448;
                _1506 = _450;
                _1507 = _549;
                _1508 = *(&mut *(_460.array)
                    .as_mut_ptr()
                    .offset(_1507 as uint64_t as int64_t as isize)
                    as *mut uint32_t);
                _1509 = _ZNK21block_size_descriptor22get_raw_partition_infoEjj(_1505, _1506, _1508);
                _1510 = *(&mut (*(_1509 as *mut l_struct_struct_OC_partition_info)).field1
                    as *mut uint16_t);
                _1511 = _549;
                *(&mut *(_548.array).as_mut_ptr().offset((llvm_mul_u32
                    as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                    2 as core::ffi::c_int as uint32_t,
                    _1511,
                ) as uint64_t as int64_t
                    as isize) as *mut uint32_t) = _1510 as uint32_t;
                _1512 = _448;
                _1513 = _450;
                _1514 = _549;
                _1515 = *(&mut *(_462.array)
                    .as_mut_ptr()
                    .offset(_1514 as uint64_t as int64_t as isize)
                    as *mut uint32_t);
                _1516 = _ZNK21block_size_descriptor22get_raw_partition_infoEjj(_1512, _1513, _1515);
                _1517 = *(&mut (*(_1516 as *mut l_struct_struct_OC_partition_info)).field1
                    as *mut uint16_t);
                _1518 = _549;
                *(&mut *(_548.array).as_mut_ptr().offset((llvm_add_u32
                    as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                    (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        2 as core::ffi::c_int as uint32_t,
                        _1518,
                    ),
                    1 as core::ffi::c_int as uint32_t,
                ) as uint64_t as int64_t
                    as isize) as *mut uint32_t) = _1517 as uint32_t;
                _1519 = _549;
                _549 = llvm_add_u32(_1519, 1 as core::ffi::c_int as uint32_t);
            }
            _1520 = memset(
                &mut _550 as *mut l_array_16_uint64_t as *mut core::ffi::c_void,
                0 as core::ffi::c_int as uint32_t,
                128 as core::ffi::c_int as uint64_t,
            );
            _551 = 0 as core::ffi::c_int as uint32_t;
            _552 = 0 as core::ffi::c_int as uint32_t;
            loop {
                _1521 = _552;
                _1522 = _453;
                if !(_1521 < llvm_mul_u32(_1522, 2 as core::ffi::c_int as uint32_t)) {
                    break;
                }
                _1523 = _552;
                _1524 = *(&mut *(_548.array)
                    .as_mut_ptr()
                    .offset(_1523 as uint64_t as int64_t as isize)
                    as *mut uint32_t);
                _553 = _1524;
                _1525 = _553;
                _554 = llvm_udiv_u32(_1525, 64 as core::ffi::c_int as uint32_t);
                _1526 = _553;
                _555 = llvm_urem_u32(_1526, 64 as core::ffi::c_int as uint32_t);
                _1527 = _554;
                _1528 = *(&mut *(_550.array)
                    .as_mut_ptr()
                    .offset(_1527 as uint64_t as int64_t as isize)
                    as *mut uint64_t);
                _1529 = _555;
                _556 = (_1528 & ((1 as core::ffi::c_int) << _1529 as uint64_t) as uint64_t
                    != 0 as core::ffi::c_ulong) as libc::c_int as bool_0;
                _1530 = _556;
                if !(_1530 as core::ffi::c_uint & 1 as libc::c_uint != 0) {
                    _1531 = _553;
                    _1532 = _452;
                    _1533 = _551;
                    *(&mut *(_1532 as *mut uint32_t).offset(_1533 as uint64_t as int64_t as isize)
                        as *mut uint32_t) = _1531;
                    _1534 = _555;
                    _1535 = _554;
                    _1536 = &mut *(_550.array)
                        .as_mut_ptr()
                        .offset(_1535 as uint64_t as int64_t as isize)
                        as *mut uint64_t as *mut core::ffi::c_void;
                    _1537 = *(_1536 as *mut uint64_t);
                    *(_1536 as *mut uint64_t) =
                        _1537 | ((1 as core::ffi::c_int) << _1534 as uint64_t) as uint64_t;
                    _1538 = _551;
                    _551 = llvm_add_u32(_1538, 1 as core::ffi::c_int as uint32_t);
                    _1539 = _551;
                    _1540 = _453;
                    if _1539 == _1540 {
                        break;
                    }
                }
                _1541 = _552;
                _552 = llvm_add_u32(_1541, 1 as core::ffi::c_int as uint32_t);
            }
            _1542 = _551;
            return _1542;
        } else {
            __assert_fail(
                &_OC_str_OC_2 as *const l_array_27_uint8_t as *mut core::ffi::c_void,
                &_OC_str_OC_1 as *const l_array_61_uint8_t as *mut core::ffi::c_void,
                577 as core::ffi::c_int as uint32_t,
                &__PRETTY_FUNCTION___OC__Z30find_best_partition_candidatesRK21block_size_descriptorRK11image_blockjjPjj
                    as *const l_array_154_uint8_t as *mut core::ffi::c_void,
            );
        }
    } else {
        __assert_fail(
            &_OC_str as *const l_array_20_uint8_t as *mut core::ffi::c_void,
            &_OC_str_OC_1 as *const l_array_61_uint8_t as *mut core::ffi::c_void,
            576 as core::ffi::c_int as uint32_t,
            &__PRETTY_FUNCTION___OC__Z30find_best_partition_candidatesRK21block_size_descriptorRK11image_blockjjPjj
                as *const l_array_154_uint8_t as *mut core::ffi::c_void,
        );
    };
}
#[inline(never)]
unsafe extern "C" fn _ZL33compute_kmeans_partition_orderingRK21block_size_descriptorRK11image_blockjPt(
    mut _1612: *mut core::ffi::c_void,
    mut _1613: *mut core::ffi::c_void,
    mut _1614: uint32_t,
    mut _1615: *mut core::ffi::c_void,
) -> uint32_t {
    let mut _1616: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1617: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1618: uint32_t = 0;
    let mut _1619: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1620: l_array_4_struct_AC_l_struct_struct_OC_vfloat4 =
        l_array_4_struct_AC_l_struct_struct_OC_vfloat4 {
            array: [l_struct_struct_OC_vfloat4 {
                field0: l_array_4_float { array: [0.; 4] },
            }; 4],
        };
    let mut _1621: l_array_216_uint8_t = l_array_216_uint8_t { array: [0; 216] };
    let mut _1622: uint32_t = 0;
    let mut _1623: l_array_4_uint64_t = l_array_4_uint64_t { array: [0; 4] };
    let mut _1624: uint32_t = 0;
    let mut _1625: uint32_t = 0;
    let mut _1626: uint32_t = 0;
    let mut _1627: l_array_1024_uint8_t = l_array_1024_uint8_t { array: [0; 1024] };
    let mut _1628: uint32_t = 0;
    let mut _1629: uint32_t = 0;
    let mut _1630: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1631: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1632: uint8_t = 0;
    let mut _1633: uint32_t = 0;
    let mut _1634: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1635: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1636: uint8_t = 0;
    let mut _1637: uint32_t = 0;
    let mut _1638: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1639: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1640: uint8_t = 0;
    let mut _1641: uint32_t = 0;
    let mut _1642: uint32_t = 0;
    let mut _1643: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1644: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1645: uint8_t = 0;
    let mut _1646: uint8_t = 0;
    let mut _1647: uint32_t = 0;
    let mut _1648: uint32_t = 0;
    let mut _1649: uint32_t = 0;
    let mut _1650: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1651: uint32_t = 0;
    let mut _1652: uint8_t = 0;
    let mut _1653: uint32_t = 0;
    let mut _1654: uint32_t = 0;
    let mut _1655: uint8_t = 0;
    let mut _1656: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1657: uint64_t = 0;
    let mut _1658: uint32_t = 0;
    let mut _1659: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1660: uint32_t = 0;
    let mut _1661: uint32_t = 0;
    let mut _1662: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1663: uint32_t = 0;
    let mut _1664: uint32_t = 0;
    let mut _1665: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1666: uint32_t = 0;
    _1616 = _1612;
    _1617 = _1613;
    _1618 = _1614;
    _1619 = _1615;
    _1622 = 0 as core::ffi::c_int as uint32_t;
    loop {
        _1628 = _1622;
        if !(_1628 < 3 as core::ffi::c_uint) {
            break;
        }
        _1629 = _1622;
        if _1629 == 0 as core::ffi::c_uint {
            _1630 = _1617;
            _1631 = _1616;
            _1632 = *(&mut (*(_1631 as *mut l_struct_struct_OC_block_size_descriptor)).field3
                as *mut uint8_t);
            _1633 = _1618;
            _ZL11kmeans_initRK11image_blockjjP7vfloat4(
                _1630,
                _1632 as uint32_t,
                _1633,
                &mut *(_1620.array)
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as int64_t as isize)
                    as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
            );
        } else {
            _1634 = _1617;
            _1635 = _1616;
            _1636 = *(&mut (*(_1635 as *mut l_struct_struct_OC_block_size_descriptor)).field3
                as *mut uint8_t);
            _1637 = _1618;
            _ZL13kmeans_updateRK11image_blockjjP7vfloat4PKh(
                _1634,
                _1636 as uint32_t,
                _1637,
                &mut *(_1620.array)
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as int64_t as isize)
                    as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                &mut *(_1621.array)
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as int64_t as isize)
                    as *mut uint8_t as *mut core::ffi::c_void,
            );
        }
        _1638 = _1617;
        _1639 = _1616;
        _1640 = *(&mut (*(_1639 as *mut l_struct_struct_OC_block_size_descriptor)).field3
            as *mut uint8_t);
        _1641 = _1618;
        _ZL13kmeans_assignRK11image_blockjjPK7vfloat4Ph(
            _1638,
            _1640 as uint32_t,
            _1641,
            &mut *(_1620.array)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as int64_t as isize)
                as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
            &mut *(_1621.array)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as int64_t as isize) as *mut uint8_t
                as *mut core::ffi::c_void,
        );
        _1642 = _1622;
        _1622 = llvm_add_u32(_1642, 1 as core::ffi::c_int as uint32_t);
    }
    _1643 = memset(
        &mut _1623 as *mut l_array_4_uint64_t as *mut core::ffi::c_void,
        0 as core::ffi::c_int as uint32_t,
        32 as core::ffi::c_int as uint64_t,
    );
    _1644 = _1616;
    _1645 =
        *(&mut (*(_1644 as *mut l_struct_struct_OC_block_size_descriptor)).field3 as *mut uint8_t);
    _1646 = _ZN4astcL3minIhEET_S1_S1_(_1645, 64 as core::ffi::c_uint as uint8_t);
    _1624 = _1646 as uint32_t;
    _1647 = _1624;
    if _1647 > 0 as core::ffi::c_uint {
        _1625 = 0 as core::ffi::c_int as uint32_t;
        loop {
            _1648 = _1625;
            _1649 = _1624;
            if !(_1648 < _1649) {
                break;
            }
            _1650 = _1616;
            _1651 = _1625;
            _1652 = *(&mut *((*(&mut (*(_1650 as *mut l_struct_struct_OC_block_size_descriptor))
                .field19 as *mut l_array_64_uint8_t))
                .array)
                .as_mut_ptr()
                .offset(_1651 as uint64_t as int64_t as isize)
                as *mut uint8_t);
            _1626 = _1652 as uint32_t;
            _1653 = _1625;
            _1654 = _1626;
            _1655 = *(&mut *(_1621.array)
                .as_mut_ptr()
                .offset(_1654 as uint64_t as int64_t as isize)
                as *mut uint8_t);
            _1656 = &mut *(_1623.array)
                .as_mut_ptr()
                .offset(_1655 as uint64_t as int64_t as isize) as *mut uint64_t
                as *mut core::ffi::c_void;
            _1657 = *(_1656 as *mut uint64_t);
            *(_1656 as *mut uint64_t) =
                _1657 | ((1 as core::ffi::c_int) << _1653 as uint64_t) as uint64_t;
            _1658 = _1625;
            _1625 = llvm_add_u32(_1658, 1 as core::ffi::c_int as uint32_t);
        }
        _1659 = _1616;
        _1660 = _1618;
        _ZL29count_partition_mismatch_bitsRK21block_size_descriptorjPKmPh(
            _1659,
            _1660,
            &mut *(_1623.array)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as int64_t as isize) as *mut uint64_t
                as *mut core::ffi::c_void,
            &mut *(_1627.array)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as int64_t as isize) as *mut uint8_t
                as *mut core::ffi::c_void,
        );
        _1661 = _1624;
        _1662 = _1616;
        _1663 = _1618;
        _1664 =
            *(&mut *((*(&mut (*(_1662 as *mut l_struct_struct_OC_block_size_descriptor)).field11
                as *mut l_array_4_uint32_t))
                .array)
                .as_mut_ptr()
                .offset(
                    (llvm_sub_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        _1663,
                        1 as core::ffi::c_int as uint32_t,
                    ) as uint64_t as int64_t as isize,
                ) as *mut uint32_t);
        _1665 = _1619;
        _1666 = _ZL39get_partition_ordering_by_mismatch_bitsjjPKhPt(
            _1661,
            _1664,
            &mut *(_1627.array)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as int64_t as isize) as *mut uint8_t
                as *mut core::ffi::c_void,
            _1665,
        );
        return _1666;
    } else {
        __assert_fail(
            &_OC_str_OC_3 as *const l_array_22_uint8_t as *mut core::ffi::c_void,
            &_OC_str_OC_1 as *const l_array_61_uint8_t as *mut core::ffi::c_void,
            485 as core::ffi::c_int as uint32_t,
            &__PRETTY_FUNCTION___OC__ZL33compute_kmeans_partition_orderingRK21block_size_descriptorRK11image_blockjPt
                as *const l_array_125_uint8_t as *mut core::ffi::c_void,
        );
    };
}
#[inline(never)]
unsafe extern "C" fn _ZN4astcL3minIjEET_S1_S1_(
    mut _1681: uint32_t,
    mut _1682: uint32_t,
) -> uint32_t {
    let mut _1683: uint32_t = 0;
    let mut _1684: uint32_t = 0;
    let mut _1685: uint32_t = 0;
    let mut _1686: uint32_t = 0;
    let mut _1687: uint32_t = 0;
    let mut _1688: uint32_t = 0;
    let mut _1689: uint32_t = 0;
    let mut _1689__PHI_TEMPORARY: uint32_t = 0;
    _1683 = _1681;
    _1684 = _1682;
    _1685 = _1683;
    _1686 = _1684;
    if _1685 < _1686 {
        _1687 = _1683;
        _1689__PHI_TEMPORARY = _1687;
    } else {
        _1688 = _1684;
        _1689__PHI_TEMPORARY = _1688;
    }
    _1689 = _1689__PHI_TEMPORARY;
    return _1689;
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn _ZNK11image_block19is_constant_channelEi(
    mut _1693: *mut core::ffi::c_void,
    mut _1694: uint32_t,
) -> bool_0 {
    let mut _1695: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1696: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1697: uint32_t = 0;
    let mut _1698: uint32_t = 0;
    let mut _1699: uint32_t = 0;
    let mut _1700: uint32_t = 0;
    let mut _1701: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1702: uint8_t = 0;
    let mut _1703: uint8_t = 0;
    let mut _1704: uint8_t = 0;
    let mut _1705: uint8_t = 0;
    let mut _1706: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1707: uint8_t = 0;
    let mut _1708: uint8_t = 0;
    let mut _1709: uint8_t = 0;
    let mut _1710: uint8_t = 0;
    let mut _1711: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1712: uint8_t = 0;
    let mut _1713: uint8_t = 0;
    let mut _1714: uint8_t = 0;
    let mut _1715: uint8_t = 0;
    let mut _1716: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1717: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1718: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1719: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1720: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1721: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1722: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1723: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1724: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1725: uint32_t = 0;
    let mut _1726: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1727: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1728: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1729: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1730: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1731: uint32_t = 0;
    let mut _1732: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1733: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1734: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1735: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1736: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1737: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1738: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1739: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1740: l_struct_struct_OC_vmask4 = l_struct_struct_OC_vmask4 {
        field0: l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _1741: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1742: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1743: uint32_t = 0;
    let mut _1744: uint32_t = 0;
    let mut _1745: uint32_t = 0;
    let mut _1746: uint32_t = 0;
    let mut _1747: l_array_2_uint64_t = l_array_2_uint64_t { array: [0; 2] };
    let mut _1748: uint32_t = 0;
    let mut _1749: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1750: uint32_t = 0;
    let mut _1751: uint32_t = 0;
    let mut _1752: uint32_t = 0;
    let mut _1753: uint32_t = 0;
    let mut _1754: l_array_2_uint64_t = l_array_2_uint64_t { array: [0; 2] };
    let mut _1755: l_array_2_uint64_t = l_array_2_uint64_t { array: [0; 2] };
    let mut _1756: uint32_t = 0;
    let mut _1757: uint32_t = 0;
    let mut _1758: uint32_t = 0;
    let mut _1759: uint32_t = 0;
    let mut _1760: uint32_t = 0;
    let mut _1761: uint32_t = 0;
    let mut _1762: uint32_t = 0;
    let mut _1763: uint32_t = 0;
    let mut _1764: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1765: uint8_t = 0;
    let mut _1766: uint64_t = 0;
    let mut _1767: uint8_t = 0;
    let mut _1768: uint64_t = 0;
    let mut _1769: uint8_t = 0;
    let mut _1770: uint64_t = 0;
    let mut _1771: uint8_t = 0;
    let mut _1772: uint64_t = 0;
    let mut _1773: l_array_2_uint64_t = l_array_2_uint64_t { array: [0; 2] };
    let mut _1774: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1775: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1776: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1777: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1778: core::ffi::c_float = 0.;
    let mut _1779: core::ffi::c_float = 0.;
    let mut _1780: core::ffi::c_float = 0.;
    let mut _1781: core::ffi::c_float = 0.;
    let mut _1782: core::ffi::c_float = 0.;
    let mut _1783: core::ffi::c_float = 0.;
    let mut _1784: core::ffi::c_float = 0.;
    let mut _1785: core::ffi::c_float = 0.;
    let mut _1786: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1787: uint8_t = 0;
    let mut _1788: uint64_t = 0;
    let mut _1789: uint8_t = 0;
    let mut _1790: uint64_t = 0;
    let mut _1791: uint8_t = 0;
    let mut _1792: uint64_t = 0;
    let mut _1793: uint8_t = 0;
    let mut _1794: uint64_t = 0;
    let mut _1795: l_array_2_uint64_t = l_array_2_uint64_t { array: [0; 2] };
    let mut _1796: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1797: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1798: l_array_2_uint64_t = l_array_2_uint64_t { array: [0; 2] };
    let mut _1799: l_array_2_uint64_t = l_array_2_uint64_t { array: [0; 2] };
    let mut _1800: uint32_t = 0;
    let mut _1801: uint32_t = 0;
    let mut _1802: uint32_t = 0;
    let mut _1803: uint32_t = 0;
    let mut _1804: uint32_t = 0;
    let mut _1805: uint32_t = 0;
    let mut _1806: uint32_t = 0;
    let mut _1807: uint32_t = 0;
    let mut _1808: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1809: uint8_t = 0;
    let mut _1810: uint64_t = 0;
    let mut _1811: uint8_t = 0;
    let mut _1812: uint64_t = 0;
    let mut _1813: uint8_t = 0;
    let mut _1814: uint64_t = 0;
    let mut _1815: uint8_t = 0;
    let mut _1816: uint64_t = 0;
    let mut _1817: l_array_2_uint64_t = l_array_2_uint64_t { array: [0; 2] };
    let mut _1818: l_array_2_uint64_t = l_array_2_uint64_t { array: [0; 2] };
    let mut _1819: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1820: l_array_2_uint64_t = l_array_2_uint64_t { array: [0; 2] };
    let mut _1821: uint32_t = 0;
    let mut _1822: uint32_t = 0;
    let mut _1823: uint32_t = 0;
    let mut _1824: uint32_t = 0;
    _1730 = _1693;
    _1731 = _1694;
    _1741 = _1730;
    _1696 = &mut _1726 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void;
    _1697 = 0 as core::ffi::c_int as uint32_t;
    _1698 = 1 as core::ffi::c_int as uint32_t;
    _1699 = 2 as core::ffi::c_int as uint32_t;
    _1700 = 3 as core::ffi::c_int as uint32_t;
    _1742 = _1696;
    _1743 = _1697;
    *(_1742 as *mut uint32_t) = _1743;
    _1744 = _1698;
    *(&mut *((*(_1742 as *mut l_array_4_uint32_t)).array)
        .as_mut_ptr()
        .offset(1 as core::ffi::c_int as int64_t as isize) as *mut uint32_t) = _1744;
    _1745 = _1699;
    *(&mut *((*(_1742 as *mut l_array_4_uint32_t)).array)
        .as_mut_ptr()
        .offset(2 as core::ffi::c_int as int64_t as isize) as *mut uint32_t) = _1745;
    _1746 = _1700;
    *(&mut *((*(_1742 as *mut l_array_4_uint32_t)).array)
        .as_mut_ptr()
        .offset(3 as core::ffi::c_int as int64_t as isize) as *mut uint32_t) = _1746;
    _1747 = *(&mut _1726 as *mut l_struct_struct_OC_vint4 as *mut l_array_2_uint64_t);
    (*(&mut _1733.field0 as *mut l_array_4_uint32_t as *mut C2RustUnnamed_7)).data = _1747;
    _1748 = _1731;
    _1724 = &mut _1734 as *mut l_struct_struct_OC_vint4 as *mut core::ffi::c_void;
    _1725 = _1748;
    _1749 = _1724;
    _1750 = _1725;
    *(_1749 as *mut uint32_t) = _1750;
    _1751 = _1725;
    *(&mut *((*(_1749 as *mut l_array_4_uint32_t)).array)
        .as_mut_ptr()
        .offset(1 as core::ffi::c_int as int64_t as isize) as *mut uint32_t) = _1751;
    _1752 = _1725;
    *(&mut *((*(_1749 as *mut l_array_4_uint32_t)).array)
        .as_mut_ptr()
        .offset(2 as core::ffi::c_int as int64_t as isize) as *mut uint32_t) = _1752;
    _1753 = _1725;
    *(&mut *((*(_1749 as *mut l_array_4_uint32_t)).array)
        .as_mut_ptr()
        .offset(3 as core::ffi::c_int as int64_t as isize) as *mut uint32_t) = _1753;
    _1754 = (*(&mut _1733.field0 as *mut l_array_4_uint32_t as *mut C2RustUnnamed_6)).data;
    _1755 = (*(&mut _1734.field0 as *mut l_array_4_uint32_t as *mut C2RustUnnamed_5)).data;
    *(&mut _1728 as *mut l_struct_struct_OC_vint4 as *mut l_array_2_uint64_t) = _1754;
    *(&mut _1729 as *mut l_struct_struct_OC_vint4 as *mut l_array_2_uint64_t) = _1755;
    _1756 = *(&mut _1728 as *mut l_struct_struct_OC_vint4 as *mut uint32_t);
    _1757 = *(&mut _1729 as *mut l_struct_struct_OC_vint4 as *mut uint32_t);
    _1758 = *(&mut *((*(&mut _1728 as *mut l_struct_struct_OC_vint4 as *mut l_array_4_uint32_t))
        .array)
        .as_mut_ptr()
        .offset(1 as core::ffi::c_int as int64_t as isize) as *mut uint32_t);
    _1759 = *(&mut *((*(&mut _1729 as *mut l_struct_struct_OC_vint4 as *mut l_array_4_uint32_t))
        .array)
        .as_mut_ptr()
        .offset(1 as core::ffi::c_int as int64_t as isize) as *mut uint32_t);
    _1760 = *(&mut *((*(&mut _1728 as *mut l_struct_struct_OC_vint4 as *mut l_array_4_uint32_t))
        .array)
        .as_mut_ptr()
        .offset(2 as core::ffi::c_int as int64_t as isize) as *mut uint32_t);
    _1761 = *(&mut *((*(&mut _1729 as *mut l_struct_struct_OC_vint4 as *mut l_array_4_uint32_t))
        .array)
        .as_mut_ptr()
        .offset(2 as core::ffi::c_int as int64_t as isize) as *mut uint32_t);
    _1762 = *(&mut *((*(&mut _1728 as *mut l_struct_struct_OC_vint4 as *mut l_array_4_uint32_t))
        .array)
        .as_mut_ptr()
        .offset(3 as core::ffi::c_int as int64_t as isize) as *mut uint32_t);
    _1763 = *(&mut *((*(&mut _1729 as *mut l_struct_struct_OC_vint4 as *mut l_array_4_uint32_t))
        .array)
        .as_mut_ptr()
        .offset(3 as core::ffi::c_int as int64_t as isize) as *mut uint32_t);
    _1701 = &mut _1727 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void;
    _1702 = (_1756 == _1757) as core::ffi::c_int as bool_0;
    _1703 = (_1758 == _1759) as core::ffi::c_int as bool_0;
    _1704 = (_1760 == _1761) as core::ffi::c_int as bool_0;
    _1705 = (_1762 == _1763) as core::ffi::c_int as bool_0;
    _1764 = _1701;
    _1765 = _1702;
    _1766 = ((_1765 as core::ffi::c_uint & 1 as libc::c_uint) as bool_0 as uint32_t
        == 0 as core::ffi::c_uint) as libc::c_int as bool_0 as uint64_t;
    *(_1764 as *mut uint32_t) = llvm_select_u32(
        ((_1765 as core::ffi::c_uint & 1 as libc::c_uint) as bool_0 as uint32_t
            == 0 as core::ffi::c_uint) as libc::c_int as bool_0,
        0 as core::ffi::c_int as uint32_t,
        -(1 as core::ffi::c_int) as uint32_t,
    );
    _1767 = _1703;
    _1768 = ((_1767 as core::ffi::c_uint & 1 as libc::c_uint) as bool_0 as uint32_t
        == 0 as core::ffi::c_uint) as libc::c_int as bool_0 as uint64_t;
    *(&mut *((*(_1764 as *mut l_array_4_uint32_t)).array)
        .as_mut_ptr()
        .offset(1 as core::ffi::c_int as int64_t as isize) as *mut uint32_t) = llvm_select_u32(
        ((_1767 as core::ffi::c_uint & 1 as libc::c_uint) as bool_0 as uint32_t
            == 0 as core::ffi::c_uint) as libc::c_int as bool_0,
        0 as core::ffi::c_int as uint32_t,
        -(1 as core::ffi::c_int) as uint32_t,
    );
    _1769 = _1704;
    _1770 = ((_1769 as core::ffi::c_uint & 1 as libc::c_uint) as bool_0 as uint32_t
        == 0 as core::ffi::c_uint) as libc::c_int as bool_0 as uint64_t;
    *(&mut *((*(_1764 as *mut l_array_4_uint32_t)).array)
        .as_mut_ptr()
        .offset(2 as core::ffi::c_int as int64_t as isize) as *mut uint32_t) = llvm_select_u32(
        ((_1769 as core::ffi::c_uint & 1 as libc::c_uint) as bool_0 as uint32_t
            == 0 as core::ffi::c_uint) as libc::c_int as bool_0,
        0 as core::ffi::c_int as uint32_t,
        -(1 as core::ffi::c_int) as uint32_t,
    );
    _1771 = _1705;
    _1772 = ((_1771 as core::ffi::c_uint & 1 as libc::c_uint) as bool_0 as uint32_t
        == 0 as core::ffi::c_uint) as libc::c_int as bool_0 as uint64_t;
    *(&mut *((*(_1764 as *mut l_array_4_uint32_t)).array)
        .as_mut_ptr()
        .offset(3 as core::ffi::c_int as int64_t as isize) as *mut uint32_t) = llvm_select_u32(
        ((_1771 as core::ffi::c_uint & 1 as libc::c_uint) as bool_0 as uint32_t
            == 0 as core::ffi::c_uint) as libc::c_int as bool_0,
        0 as core::ffi::c_int as uint32_t,
        -(1 as core::ffi::c_int) as uint32_t,
    );
    _1773 = *(&mut _1727 as *mut l_struct_struct_OC_vmask4 as *mut l_array_2_uint64_t);
    (*(&mut _1732.field0 as *mut l_array_4_uint32_t as *mut C2RustUnnamed_4)).data = _1773;
    _1774 = memcpy(
        &mut _1736 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
        &mut (*(_1741 as *mut l_struct_struct_OC_image_block)).field6
            as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
        16 as core::ffi::c_int as uint64_t,
    );
    _1775 = memcpy(
        &mut _1737 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
        &mut (*(_1741 as *mut l_struct_struct_OC_image_block)).field8
            as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
        16 as core::ffi::c_int as uint64_t,
    );
    _1776 = *(&mut _1736.field0 as *mut l_array_4_float);
    _1777 = *(&mut _1737.field0 as *mut l_array_4_float);
    *(&mut _1722 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) = _1776;
    *(&mut _1723 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) = _1777;
    _1778 = *(&mut _1722 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
    _1779 = *(&mut _1723 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
    _1780 = *(&mut *((*(&mut _1722 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float))
        .array)
        .as_mut_ptr()
        .offset(1 as core::ffi::c_int as int64_t as isize) as *mut libc::c_float);
    _1781 = *(&mut *((*(&mut _1723 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float))
        .array)
        .as_mut_ptr()
        .offset(1 as core::ffi::c_int as int64_t as isize) as *mut libc::c_float);
    _1782 = *(&mut *((*(&mut _1722 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float))
        .array)
        .as_mut_ptr()
        .offset(2 as core::ffi::c_int as int64_t as isize) as *mut libc::c_float);
    _1783 = *(&mut *((*(&mut _1723 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float))
        .array)
        .as_mut_ptr()
        .offset(2 as core::ffi::c_int as int64_t as isize) as *mut libc::c_float);
    _1784 = *(&mut *((*(&mut _1722 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float))
        .array)
        .as_mut_ptr()
        .offset(3 as core::ffi::c_int as int64_t as isize) as *mut libc::c_float);
    _1785 = *(&mut *((*(&mut _1723 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float))
        .array)
        .as_mut_ptr()
        .offset(3 as core::ffi::c_int as int64_t as isize) as *mut libc::c_float);
    _1706 = &mut _1721 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void;
    _1707 = llvm_fcmp_oeq(_1778 as core::ffi::c_double, _1779 as libc::c_double) as bool_0;
    _1708 = llvm_fcmp_oeq(_1780 as core::ffi::c_double, _1781 as libc::c_double) as bool_0;
    _1709 = llvm_fcmp_oeq(_1782 as core::ffi::c_double, _1783 as libc::c_double) as bool_0;
    _1710 = llvm_fcmp_oeq(_1784 as core::ffi::c_double, _1785 as libc::c_double) as bool_0;
    _1786 = _1706;
    _1787 = _1707;
    _1788 = ((_1787 as core::ffi::c_uint & 1 as libc::c_uint) as bool_0 as uint32_t
        == 0 as core::ffi::c_uint) as libc::c_int as bool_0 as uint64_t;
    *(_1786 as *mut uint32_t) = llvm_select_u32(
        ((_1787 as core::ffi::c_uint & 1 as libc::c_uint) as bool_0 as uint32_t
            == 0 as core::ffi::c_uint) as libc::c_int as bool_0,
        0 as core::ffi::c_int as uint32_t,
        -(1 as core::ffi::c_int) as uint32_t,
    );
    _1789 = _1708;
    _1790 = ((_1789 as core::ffi::c_uint & 1 as libc::c_uint) as bool_0 as uint32_t
        == 0 as core::ffi::c_uint) as libc::c_int as bool_0 as uint64_t;
    *(&mut *((*(_1786 as *mut l_array_4_uint32_t)).array)
        .as_mut_ptr()
        .offset(1 as core::ffi::c_int as int64_t as isize) as *mut uint32_t) = llvm_select_u32(
        ((_1789 as core::ffi::c_uint & 1 as libc::c_uint) as bool_0 as uint32_t
            == 0 as core::ffi::c_uint) as libc::c_int as bool_0,
        0 as core::ffi::c_int as uint32_t,
        -(1 as core::ffi::c_int) as uint32_t,
    );
    _1791 = _1709;
    _1792 = ((_1791 as core::ffi::c_uint & 1 as libc::c_uint) as bool_0 as uint32_t
        == 0 as core::ffi::c_uint) as libc::c_int as bool_0 as uint64_t;
    *(&mut *((*(_1786 as *mut l_array_4_uint32_t)).array)
        .as_mut_ptr()
        .offset(2 as core::ffi::c_int as int64_t as isize) as *mut uint32_t) = llvm_select_u32(
        ((_1791 as core::ffi::c_uint & 1 as libc::c_uint) as bool_0 as uint32_t
            == 0 as core::ffi::c_uint) as libc::c_int as bool_0,
        0 as core::ffi::c_int as uint32_t,
        -(1 as core::ffi::c_int) as uint32_t,
    );
    _1793 = _1710;
    _1794 = ((_1793 as core::ffi::c_uint & 1 as libc::c_uint) as bool_0 as uint32_t
        == 0 as core::ffi::c_uint) as libc::c_int as bool_0 as uint64_t;
    *(&mut *((*(_1786 as *mut l_array_4_uint32_t)).array)
        .as_mut_ptr()
        .offset(3 as core::ffi::c_int as int64_t as isize) as *mut uint32_t) = llvm_select_u32(
        ((_1793 as core::ffi::c_uint & 1 as libc::c_uint) as bool_0 as uint32_t
            == 0 as core::ffi::c_uint) as libc::c_int as bool_0,
        0 as core::ffi::c_int as uint32_t,
        -(1 as core::ffi::c_int) as uint32_t,
    );
    _1795 = *(&mut _1721 as *mut l_struct_struct_OC_vmask4 as *mut l_array_2_uint64_t);
    (*(&mut _1735.field0 as *mut l_array_4_uint32_t as *mut C2RustUnnamed_3)).data = _1795;
    _1796 = memcpy(
        &mut _1739 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void,
        &mut _1732 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void,
        16 as core::ffi::c_int as uint64_t,
    );
    _1797 = memcpy(
        &mut _1740 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void,
        &mut _1735 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void,
        16 as core::ffi::c_int as uint64_t,
    );
    _1798 = (*(&mut _1739.field0 as *mut l_array_4_uint32_t as *mut C2RustUnnamed_2)).data;
    _1799 = (*(&mut _1740.field0 as *mut l_array_4_uint32_t as *mut C2RustUnnamed_1)).data;
    *(&mut _1717 as *mut l_struct_struct_OC_vmask4 as *mut l_array_2_uint64_t) = _1798;
    *(&mut _1718 as *mut l_struct_struct_OC_vmask4 as *mut l_array_2_uint64_t) = _1799;
    _1800 = *(&mut _1717 as *mut l_struct_struct_OC_vmask4 as *mut uint32_t);
    _1801 = *(&mut _1718 as *mut l_struct_struct_OC_vmask4 as *mut uint32_t);
    _1802 = *(&mut *((*(&mut _1717 as *mut l_struct_struct_OC_vmask4 as *mut l_array_4_uint32_t))
        .array)
        .as_mut_ptr()
        .offset(1 as core::ffi::c_int as int64_t as isize) as *mut uint32_t);
    _1803 = *(&mut *((*(&mut _1718 as *mut l_struct_struct_OC_vmask4 as *mut l_array_4_uint32_t))
        .array)
        .as_mut_ptr()
        .offset(1 as core::ffi::c_int as int64_t as isize) as *mut uint32_t);
    _1804 = *(&mut *((*(&mut _1717 as *mut l_struct_struct_OC_vmask4 as *mut l_array_4_uint32_t))
        .array)
        .as_mut_ptr()
        .offset(2 as core::ffi::c_int as int64_t as isize) as *mut uint32_t);
    _1805 = *(&mut *((*(&mut _1718 as *mut l_struct_struct_OC_vmask4 as *mut l_array_4_uint32_t))
        .array)
        .as_mut_ptr()
        .offset(2 as core::ffi::c_int as int64_t as isize) as *mut uint32_t);
    _1806 = *(&mut *((*(&mut _1717 as *mut l_struct_struct_OC_vmask4 as *mut l_array_4_uint32_t))
        .array)
        .as_mut_ptr()
        .offset(3 as core::ffi::c_int as int64_t as isize) as *mut uint32_t);
    _1807 = *(&mut *((*(&mut _1718 as *mut l_struct_struct_OC_vmask4 as *mut l_array_4_uint32_t))
        .array)
        .as_mut_ptr()
        .offset(3 as core::ffi::c_int as int64_t as isize) as *mut uint32_t);
    _1711 = &mut _1716 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void;
    _1712 = (_1800 & _1801 != 0 as core::ffi::c_uint) as libc::c_int as bool_0;
    _1713 = (_1802 & _1803 != 0 as core::ffi::c_uint) as libc::c_int as bool_0;
    _1714 = (_1804 & _1805 != 0 as core::ffi::c_uint) as libc::c_int as bool_0;
    _1715 = (_1806 & _1807 != 0 as core::ffi::c_uint) as libc::c_int as bool_0;
    _1808 = _1711;
    _1809 = _1712;
    _1810 = ((_1809 as core::ffi::c_uint & 1 as libc::c_uint) as bool_0 as uint32_t
        == 0 as core::ffi::c_uint) as libc::c_int as bool_0 as uint64_t;
    *(_1808 as *mut uint32_t) = llvm_select_u32(
        ((_1809 as core::ffi::c_uint & 1 as libc::c_uint) as bool_0 as uint32_t
            == 0 as core::ffi::c_uint) as libc::c_int as bool_0,
        0 as core::ffi::c_int as uint32_t,
        -(1 as core::ffi::c_int) as uint32_t,
    );
    _1811 = _1713;
    _1812 = ((_1811 as core::ffi::c_uint & 1 as libc::c_uint) as bool_0 as uint32_t
        == 0 as core::ffi::c_uint) as libc::c_int as bool_0 as uint64_t;
    *(&mut *((*(_1808 as *mut l_array_4_uint32_t)).array)
        .as_mut_ptr()
        .offset(1 as core::ffi::c_int as int64_t as isize) as *mut uint32_t) = llvm_select_u32(
        ((_1811 as core::ffi::c_uint & 1 as libc::c_uint) as bool_0 as uint32_t
            == 0 as core::ffi::c_uint) as libc::c_int as bool_0,
        0 as core::ffi::c_int as uint32_t,
        -(1 as core::ffi::c_int) as uint32_t,
    );
    _1813 = _1714;
    _1814 = ((_1813 as core::ffi::c_uint & 1 as libc::c_uint) as bool_0 as uint32_t
        == 0 as core::ffi::c_uint) as libc::c_int as bool_0 as uint64_t;
    *(&mut *((*(_1808 as *mut l_array_4_uint32_t)).array)
        .as_mut_ptr()
        .offset(2 as core::ffi::c_int as int64_t as isize) as *mut uint32_t) = llvm_select_u32(
        ((_1813 as core::ffi::c_uint & 1 as libc::c_uint) as bool_0 as uint32_t
            == 0 as core::ffi::c_uint) as libc::c_int as bool_0,
        0 as core::ffi::c_int as uint32_t,
        -(1 as core::ffi::c_int) as uint32_t,
    );
    _1815 = _1715;
    _1816 = ((_1815 as core::ffi::c_uint & 1 as libc::c_uint) as bool_0 as uint32_t
        == 0 as core::ffi::c_uint) as libc::c_int as bool_0 as uint64_t;
    *(&mut *((*(_1808 as *mut l_array_4_uint32_t)).array)
        .as_mut_ptr()
        .offset(3 as core::ffi::c_int as int64_t as isize) as *mut uint32_t) = llvm_select_u32(
        ((_1815 as core::ffi::c_uint & 1 as libc::c_uint) as bool_0 as uint32_t
            == 0 as core::ffi::c_uint) as libc::c_int as bool_0,
        0 as core::ffi::c_int as uint32_t,
        -(1 as core::ffi::c_int) as uint32_t,
    );
    _1817 = *(&mut _1716 as *mut l_struct_struct_OC_vmask4 as *mut l_array_2_uint64_t);
    (*(&mut _1738.field0 as *mut l_array_4_uint32_t as *mut C2RustUnnamed_0)).data = _1817;
    _1818 = (*(&mut _1738.field0 as *mut l_array_4_uint32_t as *mut C2RustUnnamed)).data;
    *(&mut _1719 as *mut l_struct_struct_OC_vmask4 as *mut l_array_2_uint64_t) = _1818;
    _1819 = memcpy(
        &mut _1720 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void,
        &mut _1719 as *mut l_struct_struct_OC_vmask4 as *mut core::ffi::c_void,
        16 as core::ffi::c_int as uint64_t,
    );
    _1820 = *(&mut _1720 as *mut l_struct_struct_OC_vmask4 as *mut l_array_2_uint64_t);
    *(&mut _1695 as *mut l_struct_struct_OC_vmask4 as *mut l_array_2_uint64_t) = _1820;
    _1821 = *(&mut _1695 as *mut l_struct_struct_OC_vmask4 as *mut uint32_t);
    _1822 = *(&mut *((*(&mut _1695 as *mut l_struct_struct_OC_vmask4 as *mut l_array_4_uint32_t))
        .array)
        .as_mut_ptr()
        .offset(1 as core::ffi::c_int as int64_t as isize) as *mut uint32_t);
    _1823 = *(&mut *((*(&mut _1695 as *mut l_struct_struct_OC_vmask4 as *mut l_array_4_uint32_t))
        .array)
        .as_mut_ptr()
        .offset(2 as core::ffi::c_int as int64_t as isize) as *mut uint32_t);
    _1824 = *(&mut *((*(&mut _1695 as *mut l_struct_struct_OC_vmask4 as *mut l_array_4_uint32_t))
        .array)
        .as_mut_ptr()
        .offset(3 as core::ffi::c_int as int64_t as isize) as *mut uint32_t);
    return (_1821 & 1 as core::ffi::c_int as uint32_t
        | _1822 & 2 as libc::c_int as uint32_t
        | _1823 & 4 as core::ffi::c_int as uint32_t
        | _1824 & 8 as libc::c_int as uint32_t
        != 0 as core::ffi::c_uint) as libc::c_int as bool_0;
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn _ZNK21block_size_descriptor22get_raw_partition_infoEjj(
    mut _1825: *mut core::ffi::c_void,
    mut _1826: uint32_t,
    mut _1827: uint32_t,
) -> *mut core::ffi::c_void {
    let mut _1828: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1829: uint32_t = 0;
    let mut _1830: uint32_t = 0;
    let mut _1831: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1832: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1833: uint32_t = 0;
    let mut _1834: uint32_t = 0;
    let mut _1835: uint32_t = 0;
    let mut _1836: uint32_t = 0;
    let mut _1837: bool_0 = 0;
    let mut _1837__PHI_TEMPORARY: bool_0 = 0;
    let mut _1838: uint32_t = 0;
    let mut _1839: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1840: uint32_t = 0;
    let mut _1841: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    _1828 = _1825;
    _1829 = _1826;
    _1830 = _1827;
    _1832 = _1828;
    _1833 = _1830;
    if _1833 != 65535 as core::ffi::c_uint {
        _1834 = _1830;
        _1835 = _1829;
        _1836 =
            *(&mut *((*(&mut (*(_1832 as *mut l_struct_struct_OC_block_size_descriptor)).field12
                as *mut l_array_4_uint32_t))
                .array)
                .as_mut_ptr()
                .offset(
                    (llvm_sub_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        _1835,
                        1 as core::ffi::c_int as uint32_t,
                    ) as uint64_t as int64_t as isize,
                ) as *mut uint32_t);
        _1837__PHI_TEMPORARY = (_1834 < _1836) as core::ffi::c_int as bool_0;
    } else {
        _1837__PHI_TEMPORARY = 0 as core::ffi::c_int as bool_0;
    }
    _1837 = _1837__PHI_TEMPORARY;
    if _1837 != 0 {
        _1838 = _1829;
        _1839 = _ZNK21block_size_descriptor19get_partition_tableEj(_1832, _1838);
        _1840 = _1830;
        _1831 = &mut *(_1839 as *mut l_struct_struct_OC_partition_info)
            .offset(_1840 as uint64_t as int64_t as isize)
            as *mut l_struct_struct_OC_partition_info as *mut core::ffi::c_void;
        _1841 = _1831;
        return _1841;
    } else {
        __assert_fail(
            &_OC_str_OC_9 as *const l_array_107_uint8_t as *mut core::ffi::c_void,
            &_OC_str_OC_10 as *const l_array_45_uint8_t as *mut core::ffi::c_void,
            724 as core::ffi::c_int as uint32_t,
            &__PRETTY_FUNCTION___OC__ZNK21block_size_descriptor22get_raw_partition_infoEjj
                as *const l_array_102_uint8_t as *mut core::ffi::c_void,
        );
    };
}
#[inline(never)]
unsafe extern "C" fn _ZL13insert_resultjfjPfPj(
    mut _1847: uint32_t,
    mut _1848: core::ffi::c_float,
    mut _1849: uint32_t,
    mut _1850: *mut core::ffi::c_void,
    mut _1851: *mut core::ffi::c_void,
) {
    let mut current_block: u64;
    let mut _1852: uint32_t = 0;
    let mut _1853: core::ffi::c_float = 0.;
    let mut _1854: uint32_t = 0;
    let mut _1855: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1856: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1857: uint32_t = 0;
    let mut _1858: uint32_t = 0;
    let mut _1859: uint32_t = 0;
    let mut _1860: core::ffi::c_float = 0.;
    let mut _1861: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1862: uint32_t = 0;
    let mut _1863: core::ffi::c_float = 0.;
    let mut _1864: uint32_t = 0;
    let mut _1865: uint32_t = 0;
    let mut _1866: core::ffi::c_float = 0.;
    let mut _1867: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1868: uint32_t = 0;
    let mut _1869: core::ffi::c_float = 0.;
    let mut _1870: uint32_t = 0;
    let mut _1871: uint32_t = 0;
    let mut _1872: uint32_t = 0;
    let mut _1873: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1874: uint32_t = 0;
    let mut _1875: core::ffi::c_float = 0.;
    let mut _1876: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1877: uint32_t = 0;
    let mut _1878: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1879: uint32_t = 0;
    let mut _1880: uint32_t = 0;
    let mut _1881: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1882: uint32_t = 0;
    let mut _1883: uint32_t = 0;
    let mut _1884: core::ffi::c_float = 0.;
    let mut _1885: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1886: uint32_t = 0;
    let mut _1887: uint32_t = 0;
    let mut _1888: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1889: uint32_t = 0;
    let mut _1890: uint32_t = 0;
    _1852 = _1847;
    _1853 = _1848;
    _1854 = _1849;
    _1855 = _1850;
    _1856 = _1851;
    _1859 = _1852;
    if _1859 > 0 as core::ffi::c_uint {
        _1860 = _1853;
        _1861 = _1855;
        _1862 = _1852;
        _1863 = *(&mut *(_1861 as *mut core::ffi::c_float).offset((llvm_sub_u32
            as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
            _1862,
            1 as core::ffi::c_int as uint32_t,
        ) as uint64_t as int64_t
            as isize) as *mut core::ffi::c_float);
        if !(llvm_fcmp_oge(_1860 as core::ffi::c_double, _1863 as libc::c_double) != 0) {
            _1857 = 0 as core::ffi::c_int as uint32_t;
            loop {
                _1864 = _1857;
                _1865 = _1852;
                if !(_1864 < _1865) {
                    current_block = 8080193727413914482;
                    break;
                }
                _1866 = _1853;
                _1867 = _1855;
                _1868 = _1857;
                _1869 = *(&mut *(_1867 as *mut core::ffi::c_float)
                    .offset(_1868 as uint64_t as int64_t as isize)
                    as *mut core::ffi::c_float);
                if !(llvm_fcmp_ogt(_1866 as core::ffi::c_double, _1869 as libc::c_double) != 0) {
                    current_block = 11457568801535137816;
                    break;
                }
                _1890 = _1857;
                _1857 = llvm_add_u32(_1890, 1 as core::ffi::c_int as uint32_t);
            }
            match current_block {
                8080193727413914482 => {}
                _ => {
                    _1870 = _1852;
                    _1858 = llvm_sub_u32(_1870, 1 as core::ffi::c_int as uint32_t);
                    loop {
                        _1871 = _1858;
                        _1872 = _1857;
                        if !(_1871 > _1872) {
                            break;
                        }
                        _1873 = _1855;
                        _1874 = _1858;
                        _1875 = *(&mut *(_1873 as *mut core::ffi::c_float).offset((llvm_sub_u32
                            as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            _1874,
                            1 as core::ffi::c_int as uint32_t,
                        )
                            as uint64_t
                            as int64_t
                            as isize) as *mut core::ffi::c_float);
                        _1876 = _1855;
                        _1877 = _1858;
                        *(&mut *(_1876 as *mut core::ffi::c_float)
                            .offset(_1877 as uint64_t as int64_t as isize)
                            as *mut core::ffi::c_float) = _1875;
                        _1878 = _1856;
                        _1879 = _1858;
                        _1880 = *(&mut *(_1878 as *mut uint32_t).offset((llvm_sub_u32
                            as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            _1879,
                            1 as core::ffi::c_int as uint32_t,
                        )
                            as uint64_t
                            as int64_t
                            as isize) as *mut uint32_t);
                        _1881 = _1856;
                        _1882 = _1858;
                        *(&mut *(_1881 as *mut uint32_t)
                            .offset(_1882 as uint64_t as int64_t as isize)
                            as *mut uint32_t) = _1880;
                        _1883 = _1858;
                        _1858 = llvm_add_u32(_1883, -(1 as core::ffi::c_int) as uint32_t);
                    }
                    _1884 = _1853;
                    _1885 = _1855;
                    _1886 = _1857;
                    *(&mut *(_1885 as *mut core::ffi::c_float)
                        .offset(_1886 as uint64_t as int64_t as isize)
                        as *mut core::ffi::c_float) = _1884;
                    _1887 = _1854;
                    _1888 = _1856;
                    _1889 = _1857;
                    *(&mut *(_1888 as *mut uint32_t).offset(_1889 as uint64_t as int64_t as isize)
                        as *mut uint32_t) = _1887;
                }
            }
        }
        return;
    } else {
        __assert_fail(
            &_OC_str_OC_11 as *const l_array_15_uint8_t as *mut core::ffi::c_void,
            &_OC_str_OC_1 as *const l_array_61_uint8_t as *mut core::ffi::c_void,
            519 as core::ffi::c_int as uint32_t,
            &__PRETTY_FUNCTION___OC__ZL13insert_resultjfjPfPj as *const l_array_79_uint8_t
                as *mut core::ffi::c_void,
        );
    };
}
#[inline(never)]
unsafe extern "C" fn _ZL11kmeans_initRK11image_blockjjP7vfloat4(
    mut _1906: *mut core::ffi::c_void,
    mut _1907: uint32_t,
    mut _1908: uint32_t,
    mut _1909: *mut core::ffi::c_void,
) {
    let mut _1910: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1911: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1912: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1913: core::ffi::c_float = 0.;
    let mut _1914: core::ffi::c_float = 0.;
    let mut _1915: core::ffi::c_float = 0.;
    let mut _1916: core::ffi::c_float = 0.;
    let mut _1917: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1918: core::ffi::c_float = 0.;
    let mut _1919: core::ffi::c_float = 0.;
    let mut _1920: core::ffi::c_float = 0.;
    let mut _1921: core::ffi::c_float = 0.;
    let mut _1922: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1923: core::ffi::c_float = 0.;
    let mut _1924: core::ffi::c_float = 0.;
    let mut _1925: core::ffi::c_float = 0.;
    let mut _1926: core::ffi::c_float = 0.;
    let mut _1927: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1928: core::ffi::c_float = 0.;
    let mut _1929: core::ffi::c_float = 0.;
    let mut _1930: core::ffi::c_float = 0.;
    let mut _1931: core::ffi::c_float = 0.;
    let mut _1932: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1933: core::ffi::c_float = 0.;
    let mut _1934: core::ffi::c_float = 0.;
    let mut _1935: core::ffi::c_float = 0.;
    let mut _1936: core::ffi::c_float = 0.;
    let mut _1937: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1938: core::ffi::c_float = 0.;
    let mut _1939: core::ffi::c_float = 0.;
    let mut _1940: core::ffi::c_float = 0.;
    let mut _1941: core::ffi::c_float = 0.;
    let mut _1942: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1943: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1944: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1945: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1946: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1947: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1948: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1949: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1950: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1951: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1952: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1953: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1954: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1955: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1956: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1957: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1958: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1959: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1960: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1961: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1962: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1963: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1964: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1965: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1966: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1967: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1968: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1969: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1970: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1971: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1972: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1973: uint32_t = 0;
    let mut _1974: uint32_t = 0;
    let mut _1975: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _1976: uint32_t = 0;
    let mut _1977: l_array_216_float = l_array_216_float { array: [0.; 216] };
    let mut _1978: uint32_t = 0;
    let mut _1979: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1980: core::ffi::c_float = 0.;
    let mut _1981: uint32_t = 0;
    let mut _1982: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1983: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1984: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1985: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1986: core::ffi::c_float = 0.;
    let mut _1987: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1988: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1989: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1990: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1991: l_array_9_float = l_array_9_float { array: [0.; 9] };
    let mut _1992: uint32_t = 0;
    let mut _1993: core::ffi::c_float = 0.;
    let mut _1994: core::ffi::c_float = 0.;
    let mut _1995: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1996: uint32_t = 0;
    let mut _1997: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1998: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1999: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2000: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2001: core::ffi::c_float = 0.;
    let mut _2002: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2003: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2004: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2005: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2006: uint32_t = 0;
    let mut _2007: uint32_t = 0;
    let mut _2008: uint32_t = 0;
    let mut _2009: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2010: uint32_t = 0;
    let mut _2011: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2012: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2013: uint32_t = 0;
    let mut _2014: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2015: uint32_t = 0;
    let mut _2016: uint32_t = 0;
    let mut _2017: uint32_t = 0;
    let mut _2018: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2019: uint32_t = 0;
    let mut _2020: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2021: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2022: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2023: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _2024: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _2025: core::ffi::c_float = 0.;
    let mut _2026: core::ffi::c_float = 0.;
    let mut _2027: core::ffi::c_float = 0.;
    let mut _2028: core::ffi::c_float = 0.;
    let mut _2029: core::ffi::c_float = 0.;
    let mut _2030: core::ffi::c_float = 0.;
    let mut _2031: core::ffi::c_float = 0.;
    let mut _2032: core::ffi::c_float = 0.;
    let mut _2033: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2034: core::ffi::c_float = 0.;
    let mut _2035: core::ffi::c_float = 0.;
    let mut _2036: core::ffi::c_float = 0.;
    let mut _2037: core::ffi::c_float = 0.;
    let mut _2038: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2039: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2040: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2041: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _2042: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _2043: core::ffi::c_float = 0.;
    let mut _2044: core::ffi::c_float = 0.;
    let mut _2045: core::ffi::c_float = 0.;
    let mut _2046: core::ffi::c_float = 0.;
    let mut _2047: core::ffi::c_float = 0.;
    let mut _2048: core::ffi::c_float = 0.;
    let mut _2049: core::ffi::c_float = 0.;
    let mut _2050: core::ffi::c_float = 0.;
    let mut _2051: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2052: core::ffi::c_float = 0.;
    let mut _2053: core::ffi::c_float = 0.;
    let mut _2054: core::ffi::c_float = 0.;
    let mut _2055: core::ffi::c_float = 0.;
    let mut _2056: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2057: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2058: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2059: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _2060: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _2061: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2062: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2063: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _2064: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _2065: core::ffi::c_float = 0.;
    let mut _2066: core::ffi::c_float = 0.;
    let mut _2067: core::ffi::c_float = 0.;
    let mut _2068: core::ffi::c_float = 0.;
    let mut _2069: core::ffi::c_float = 0.;
    let mut _2070: core::ffi::c_float = 0.;
    let mut _2071: core::ffi::c_float = 0.;
    let mut _2072: core::ffi::c_float = 0.;
    let mut _2073: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2074: core::ffi::c_float = 0.;
    let mut _2075: core::ffi::c_float = 0.;
    let mut _2076: core::ffi::c_float = 0.;
    let mut _2077: core::ffi::c_float = 0.;
    let mut _2078: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2079: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2080: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _2081: core::ffi::c_float = 0.;
    let mut _2082: core::ffi::c_float = 0.;
    let mut _2083: core::ffi::c_float = 0.;
    let mut _2084: core::ffi::c_float = 0.;
    let mut _2085: core::ffi::c_float = 0.;
    let mut _2086: core::ffi::c_float = 0.;
    let mut _2087: core::ffi::c_float = 0.;
    let mut _2088: uint32_t = 0;
    let mut _2089: uint32_t = 0;
    let mut _2090: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2091: uint32_t = 0;
    let mut _2092: uint32_t = 0;
    let mut _2093: core::ffi::c_float = 0.;
    let mut _2094: uint32_t = 0;
    let mut _2095: core::ffi::c_float = 0.;
    let mut _2096: uint32_t = 0;
    let mut _2097: uint32_t = 0;
    let mut _2098: uint32_t = 0;
    let mut _2099: core::ffi::c_float = 0.;
    let mut _2100: core::ffi::c_float = 0.;
    let mut _2101: core::ffi::c_float = 0.;
    let mut _2102: core::ffi::c_float = 0.;
    let mut _2103: uint32_t = 0;
    let mut _2104: uint32_t = 0;
    let mut _2105: uint32_t = 0;
    let mut _2106: uint32_t = 0;
    let mut _2107: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2108: uint32_t = 0;
    let mut _2109: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2110: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2111: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2112: uint32_t = 0;
    let mut _2113: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2114: uint32_t = 0;
    let mut _2115: uint32_t = 0;
    let mut _2116: uint32_t = 0;
    let mut _2117: uint32_t = 0;
    let mut _2118: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2119: uint32_t = 0;
    let mut _2120: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2121: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2122: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2123: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _2124: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _2125: core::ffi::c_float = 0.;
    let mut _2126: core::ffi::c_float = 0.;
    let mut _2127: core::ffi::c_float = 0.;
    let mut _2128: core::ffi::c_float = 0.;
    let mut _2129: core::ffi::c_float = 0.;
    let mut _2130: core::ffi::c_float = 0.;
    let mut _2131: core::ffi::c_float = 0.;
    let mut _2132: core::ffi::c_float = 0.;
    let mut _2133: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2134: core::ffi::c_float = 0.;
    let mut _2135: core::ffi::c_float = 0.;
    let mut _2136: core::ffi::c_float = 0.;
    let mut _2137: core::ffi::c_float = 0.;
    let mut _2138: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2139: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2140: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2141: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _2142: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _2143: core::ffi::c_float = 0.;
    let mut _2144: core::ffi::c_float = 0.;
    let mut _2145: core::ffi::c_float = 0.;
    let mut _2146: core::ffi::c_float = 0.;
    let mut _2147: core::ffi::c_float = 0.;
    let mut _2148: core::ffi::c_float = 0.;
    let mut _2149: core::ffi::c_float = 0.;
    let mut _2150: core::ffi::c_float = 0.;
    let mut _2151: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2152: core::ffi::c_float = 0.;
    let mut _2153: core::ffi::c_float = 0.;
    let mut _2154: core::ffi::c_float = 0.;
    let mut _2155: core::ffi::c_float = 0.;
    let mut _2156: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2157: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2158: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2159: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _2160: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _2161: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2162: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2163: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _2164: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _2165: core::ffi::c_float = 0.;
    let mut _2166: core::ffi::c_float = 0.;
    let mut _2167: core::ffi::c_float = 0.;
    let mut _2168: core::ffi::c_float = 0.;
    let mut _2169: core::ffi::c_float = 0.;
    let mut _2170: core::ffi::c_float = 0.;
    let mut _2171: core::ffi::c_float = 0.;
    let mut _2172: core::ffi::c_float = 0.;
    let mut _2173: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2174: core::ffi::c_float = 0.;
    let mut _2175: core::ffi::c_float = 0.;
    let mut _2176: core::ffi::c_float = 0.;
    let mut _2177: core::ffi::c_float = 0.;
    let mut _2178: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2179: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2180: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _2181: core::ffi::c_float = 0.;
    let mut _2182: core::ffi::c_float = 0.;
    let mut _2183: core::ffi::c_float = 0.;
    let mut _2184: core::ffi::c_float = 0.;
    let mut _2185: core::ffi::c_float = 0.;
    let mut _2186: uint32_t = 0;
    let mut _2187: core::ffi::c_float = 0.;
    let mut _2188: core::ffi::c_float = 0.;
    let mut _2189: core::ffi::c_float = 0.;
    let mut _2190: core::ffi::c_float = 0.;
    let mut _2191: core::ffi::c_float = 0.;
    let mut _2192: uint32_t = 0;
    let mut _2193: uint32_t = 0;
    _1972 = _1906;
    _1973 = _1907;
    _1974 = _1908;
    _1975 = _1909;
    _2006 = _1973;
    if _2006 > 0 as core::ffi::c_uint {
        _2007 = _1974;
        if _2007 > 0 as core::ffi::c_uint {
            _1976 = 0 as core::ffi::c_int as uint32_t;
            _2008 = _1973;
            _1978 = llvm_urem_u32(145897 as core::ffi::c_int as uint32_t, _2008);
            _2009 = _1972;
            _2010 = _1978;
            _2011 = _ZNK11image_block5texelEj(_2009, _2010);
            *(&mut _1979.field0 as *mut l_array_4_float) = _2011.field0;
            _2012 = _1975;
            _2013 = _1976;
            _2014 = memcpy(
                &mut *(_2012 as *mut l_struct_struct_OC_vfloat4)
                    .offset(_2013 as uint64_t as int64_t as isize)
                    as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                &mut _1979 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                16 as core::ffi::c_int as uint64_t,
            );
            _2015 = _1976;
            _1976 = llvm_add_u32(_2015, 1 as core::ffi::c_int as uint32_t);
            _1980 = 0 as core::ffi::c_int as libc::c_float;
            _1981 = 0 as core::ffi::c_int as uint32_t;
            loop {
                _2016 = _1981;
                _2017 = _1973;
                if !(_2016 < _2017) {
                    break;
                }
                _2018 = _1972;
                _2019 = _1981;
                _2020 = _ZNK11image_block5texelEj(_2018, _2019);
                *(&mut _1982.field0 as *mut l_array_4_float) = _2020.field0;
                _2021 = memcpy(
                    &mut _1984 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _1982 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16 as core::ffi::c_int as uint64_t,
                );
                _2022 = memcpy(
                    &mut _1985 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _1979 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16 as core::ffi::c_int as uint64_t,
                );
                _2023 = *(&mut _1984.field0 as *mut l_array_4_float);
                _2024 = *(&mut _1985.field0 as *mut l_array_4_float);
                *(&mut _1967 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) = _2023;
                *(&mut _1968 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) = _2024;
                _2025 = *(&mut _1967 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _2026 = *(&mut _1968 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _2027 = *(&mut *((*(&mut _1967 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _2028 = *(&mut *((*(&mut _1968 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _2029 = *(&mut *((*(&mut _1967 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _2030 = *(&mut *((*(&mut _1968 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _2031 = *(&mut *((*(&mut _1967 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _2032 = *(&mut *((*(&mut _1968 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _1917 = &mut _1966 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _1918 = llvm_fsub_f32(_2025, _2026);
                _1919 = llvm_fsub_f32(_2027, _2028);
                _1920 = llvm_fsub_f32(_2029, _2030);
                _1921 = llvm_fsub_f32(_2031, _2032);
                _2033 = _1917;
                _2034 = _1918;
                *(_2033 as *mut core::ffi::c_float) = _2034;
                _2035 = _1919;
                *(&mut *((*(_2033 as *mut l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _2035;
                _2036 = _1920;
                *(&mut *((*(_2033 as *mut l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _2036;
                _2037 = _1921;
                *(&mut *((*(_2033 as *mut l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _2037;
                _2038 = _1966;
                *(&mut _1983.field0 as *mut l_array_4_float) = _2038.field0;
                _2039 = memcpy(
                    &mut _1988 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _1983 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16 as core::ffi::c_int as uint64_t,
                );
                _2040 = memcpy(
                    &mut _1989 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _1983 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16 as core::ffi::c_int as uint64_t,
                );
                _2041 = *(&mut _1988.field0 as *mut l_array_4_float);
                _2042 = *(&mut _1989.field0 as *mut l_array_4_float);
                *(&mut _1961 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) = _2041;
                *(&mut _1962 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) = _2042;
                _2043 = *(&mut _1961 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _2044 = *(&mut _1962 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _2045 = *(&mut *((*(&mut _1961 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _2046 = *(&mut *((*(&mut _1962 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _2047 = *(&mut *((*(&mut _1961 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _2048 = *(&mut *((*(&mut _1962 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _2049 = *(&mut *((*(&mut _1961 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _2050 = *(&mut *((*(&mut _1962 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _1927 = &mut _1960 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _1928 = llvm_fmul_f32(_2043, _2044);
                _1929 = llvm_fmul_f32(_2045, _2046);
                _1930 = llvm_fmul_f32(_2047, _2048);
                _1931 = llvm_fmul_f32(_2049, _2050);
                _2051 = _1927;
                _2052 = _1928;
                *(_2051 as *mut core::ffi::c_float) = _2052;
                _2053 = _1929;
                *(&mut *((*(_2051 as *mut l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _2053;
                _2054 = _1930;
                *(&mut *((*(_2051 as *mut l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _2054;
                _2055 = _1931;
                *(&mut *((*(_2051 as *mut l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _2055;
                _2056 = _1960;
                *(&mut _1987.field0 as *mut l_array_4_float) = _2056.field0;
                _2057 = _1972;
                _2058 = memcpy(
                    &mut _1990 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut (*(_2057 as *mut l_struct_struct_OC_image_block)).field9
                        as *mut l_struct_struct_OC_vfloat4
                        as *mut core::ffi::c_void,
                    16 as core::ffi::c_int as uint64_t,
                );
                _2059 = *(&mut _1987.field0 as *mut l_array_4_float);
                _2060 = *(&mut _1990.field0 as *mut l_array_4_float);
                *(&mut _1945 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) = _2059;
                *(&mut _1946 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) = _2060;
                _2061 = memcpy(
                    &mut _1948 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _1945 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16 as core::ffi::c_int as uint64_t,
                );
                _2062 = memcpy(
                    &mut _1949 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _1946 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16 as core::ffi::c_int as uint64_t,
                );
                _2063 = *(&mut _1948 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                _2064 = *(&mut _1949 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                *(&mut _1943 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) = _2063;
                *(&mut _1944 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) = _2064;
                _2065 = *(&mut _1943 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _2066 = *(&mut _1944 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _2067 = *(&mut *((*(&mut _1943 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _2068 = *(&mut *((*(&mut _1944 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _2069 = *(&mut *((*(&mut _1943 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _2070 = *(&mut *((*(&mut _1944 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _2071 = *(&mut *((*(&mut _1943 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _2072 = *(&mut *((*(&mut _1944 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _1937 = &mut _1942 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _1938 = llvm_fmul_f32(_2065, _2066);
                _1939 = llvm_fmul_f32(_2067, _2068);
                _1940 = llvm_fmul_f32(_2069, _2070);
                _1941 = llvm_fmul_f32(_2071, _2072);
                _2073 = _1937;
                _2074 = _1938;
                *(_2073 as *mut core::ffi::c_float) = _2074;
                _2075 = _1939;
                *(&mut *((*(_2073 as *mut l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _2075;
                _2076 = _1940;
                *(&mut *((*(_2073 as *mut l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _2076;
                _2077 = _1941;
                *(&mut *((*(_2073 as *mut l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _2077;
                _2078 = _1942;
                *(&mut _1947 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                    _2078.field0;
                _2079 = memcpy(
                    &mut _1950 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _1947 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16 as core::ffi::c_int as uint64_t,
                );
                _2080 = *(&mut _1950 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                *(&mut _1911 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) = _2080;
                _2081 = *(&mut _1911 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _2082 = *(&mut *((*(&mut _1911 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _2083 = *(&mut *((*(&mut _1911 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _2084 = *(&mut *((*(&mut _1911 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _1986 = llvm_fadd_f32(llvm_fadd_f32(_2081, _2082), llvm_fadd_f32(_2083, _2084));
                _2085 = _1986;
                _2086 = _1980;
                _1980 = llvm_fadd_f32(_2086, _2085);
                _2087 = _1986;
                _2088 = _1981;
                *(&mut *(_1977.array)
                    .as_mut_ptr()
                    .offset(_2088 as uint64_t as int64_t as isize)
                    as *mut core::ffi::c_float) = _2087;
                _2089 = _1981;
                _1981 = llvm_add_u32(_2089, 1 as core::ffi::c_int as uint32_t);
            }
            _2090 = memcpy(
                &mut _1991 as *mut l_array_9_float as *mut core::ffi::c_void,
                &__const_OC__ZL11kmeans_initRK11image_blockjjP7vfloat4_OC_cluster_cutoffs
                    as *const l_array_9_float as *mut core::ffi::c_void,
                36 as core::ffi::c_int as uint64_t,
            );
            _2091 = _1976;
            _2092 = _1974;
            _1992 = llvm_add_u32(
                llvm_sub_u32(_2091, 1 as core::ffi::c_int as uint32_t),
                llvm_mul_u32(
                    3 as core::ffi::c_int as uint32_t,
                    llvm_sub_u32(_2092, 2 as core::ffi::c_int as uint32_t),
                ),
            );
            loop {
                _1993 = 0 as core::ffi::c_int as libc::c_float;
                _2093 = _1980;
                _2094 = _1992;
                _1992 = llvm_add_u32(_2094, 1 as core::ffi::c_int as uint32_t);
                _2095 = *(&mut *(_1991.array)
                    .as_mut_ptr()
                    .offset(_2094 as uint64_t as int64_t as isize)
                    as *mut core::ffi::c_float);
                _1994 = llvm_fmul_f32(_2093, _2095);
                _1978 = 0 as core::ffi::c_int as uint32_t;
                loop {
                    _2096 = _1978;
                    _2097 = _1973;
                    if !(_2096 < _2097) {
                        break;
                    }
                    _2098 = _1978;
                    _2099 = *(&mut *(_1977.array)
                        .as_mut_ptr()
                        .offset(_2098 as uint64_t as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2100 = _1993;
                    _1993 = llvm_fadd_f32(_2100, _2099);
                    _2101 = _1993;
                    _2102 = _1994;
                    if llvm_fcmp_oge(_2101 as core::ffi::c_double, _2102 as libc::c_double) != 0 {
                        break;
                    }
                    _2103 = _1978;
                    _1978 = llvm_add_u32(_2103, 1 as core::ffi::c_int as uint32_t);
                }
                _2104 = _1978;
                _2105 = _1973;
                _2106 = _ZN4astcL3minIjEET_S1_S1_(
                    _2104,
                    llvm_sub_u32(_2105, 1 as core::ffi::c_int as uint32_t),
                );
                _1978 = _2106;
                _2107 = _1972;
                _2108 = _1978;
                _2109 = _ZNK11image_block5texelEj(_2107, _2108);
                *(&mut _1995.field0 as *mut l_array_4_float) = _2109.field0;
                _2110 = memcpy(
                    &mut _1979 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _1995 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16 as core::ffi::c_int as uint64_t,
                );
                _2111 = _1975;
                _2112 = _1976;
                _1976 = llvm_add_u32(_2112, 1 as core::ffi::c_int as uint32_t);
                _2113 = memcpy(
                    &mut *(_2111 as *mut l_struct_struct_OC_vfloat4)
                        .offset(_2112 as uint64_t as int64_t as isize)
                        as *mut l_struct_struct_OC_vfloat4
                        as *mut core::ffi::c_void,
                    &mut _1979 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16 as core::ffi::c_int as uint64_t,
                );
                _2114 = _1976;
                _2115 = _1974;
                if _2114 >= _2115 {
                    break;
                }
                _1980 = 0 as core::ffi::c_int as libc::c_float;
                _1996 = 0 as core::ffi::c_int as uint32_t;
                loop {
                    _2116 = _1996;
                    _2117 = _1973;
                    if !(_2116 < _2117) {
                        break;
                    }
                    _2118 = _1972;
                    _2119 = _1996;
                    _2120 = _ZNK11image_block5texelEj(_2118, _2119);
                    *(&mut _1997.field0 as *mut l_array_4_float) = _2120.field0;
                    _2121 = memcpy(
                        &mut _1999 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        &mut _1997 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        16 as core::ffi::c_int as uint64_t,
                    );
                    _2122 = memcpy(
                        &mut _2000 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        &mut _1979 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        16 as core::ffi::c_int as uint64_t,
                    );
                    _2123 = *(&mut _1999.field0 as *mut l_array_4_float);
                    _2124 = *(&mut _2000.field0 as *mut l_array_4_float);
                    *(&mut _1970 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                        _2123;
                    *(&mut _1971 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                        _2124;
                    _2125 =
                        *(&mut _1970 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _2126 =
                        *(&mut _1971 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _2127 = *(&mut *((*(&mut _1970 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2128 = *(&mut *((*(&mut _1971 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2129 = *(&mut *((*(&mut _1970 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2130 = *(&mut *((*(&mut _1971 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2131 = *(&mut *((*(&mut _1970 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2132 = *(&mut *((*(&mut _1971 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1912 = &mut _1969 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                    _1913 = llvm_fsub_f32(_2125, _2126);
                    _1914 = llvm_fsub_f32(_2127, _2128);
                    _1915 = llvm_fsub_f32(_2129, _2130);
                    _1916 = llvm_fsub_f32(_2131, _2132);
                    _2133 = _1912;
                    _2134 = _1913;
                    *(_2133 as *mut core::ffi::c_float) = _2134;
                    _2135 = _1914;
                    *(&mut *((*(_2133 as *mut l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _2135;
                    _2136 = _1915;
                    *(&mut *((*(_2133 as *mut l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _2136;
                    _2137 = _1916;
                    *(&mut *((*(_2133 as *mut l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _2137;
                    _2138 = _1969;
                    *(&mut _1998.field0 as *mut l_array_4_float) = _2138.field0;
                    _2139 = memcpy(
                        &mut _2003 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        &mut _1998 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        16 as core::ffi::c_int as uint64_t,
                    );
                    _2140 = memcpy(
                        &mut _2004 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        &mut _1998 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        16 as core::ffi::c_int as uint64_t,
                    );
                    _2141 = *(&mut _2003.field0 as *mut l_array_4_float);
                    _2142 = *(&mut _2004.field0 as *mut l_array_4_float);
                    *(&mut _1964 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                        _2141;
                    *(&mut _1965 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                        _2142;
                    _2143 =
                        *(&mut _1964 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _2144 =
                        *(&mut _1965 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _2145 = *(&mut *((*(&mut _1964 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2146 = *(&mut *((*(&mut _1965 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2147 = *(&mut *((*(&mut _1964 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2148 = *(&mut *((*(&mut _1965 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2149 = *(&mut *((*(&mut _1964 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2150 = *(&mut *((*(&mut _1965 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1922 = &mut _1963 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                    _1923 = llvm_fmul_f32(_2143, _2144);
                    _1924 = llvm_fmul_f32(_2145, _2146);
                    _1925 = llvm_fmul_f32(_2147, _2148);
                    _1926 = llvm_fmul_f32(_2149, _2150);
                    _2151 = _1922;
                    _2152 = _1923;
                    *(_2151 as *mut core::ffi::c_float) = _2152;
                    _2153 = _1924;
                    *(&mut *((*(_2151 as *mut l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _2153;
                    _2154 = _1925;
                    *(&mut *((*(_2151 as *mut l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _2154;
                    _2155 = _1926;
                    *(&mut *((*(_2151 as *mut l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _2155;
                    _2156 = _1963;
                    *(&mut _2002.field0 as *mut l_array_4_float) = _2156.field0;
                    _2157 = _1972;
                    _2158 = memcpy(
                        &mut _2005 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        &mut (*(_2157 as *mut l_struct_struct_OC_image_block)).field9
                            as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_void,
                        16 as core::ffi::c_int as uint64_t,
                    );
                    _2159 = *(&mut _2002.field0 as *mut l_array_4_float);
                    _2160 = *(&mut _2005.field0 as *mut l_array_4_float);
                    *(&mut _1954 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                        _2159;
                    *(&mut _1955 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                        _2160;
                    _2161 = memcpy(
                        &mut _1957 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        &mut _1954 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        16 as core::ffi::c_int as uint64_t,
                    );
                    _2162 = memcpy(
                        &mut _1958 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        &mut _1955 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        16 as core::ffi::c_int as uint64_t,
                    );
                    _2163 =
                        *(&mut _1957 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                    _2164 =
                        *(&mut _1958 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                    *(&mut _1952 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                        _2163;
                    *(&mut _1953 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                        _2164;
                    _2165 =
                        *(&mut _1952 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _2166 =
                        *(&mut _1953 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _2167 = *(&mut *((*(&mut _1952 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2168 = *(&mut *((*(&mut _1953 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2169 = *(&mut *((*(&mut _1952 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2170 = *(&mut *((*(&mut _1953 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2171 = *(&mut *((*(&mut _1952 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2172 = *(&mut *((*(&mut _1953 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _1932 = &mut _1951 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                    _1933 = llvm_fmul_f32(_2165, _2166);
                    _1934 = llvm_fmul_f32(_2167, _2168);
                    _1935 = llvm_fmul_f32(_2169, _2170);
                    _1936 = llvm_fmul_f32(_2171, _2172);
                    _2173 = _1932;
                    _2174 = _1933;
                    *(_2173 as *mut core::ffi::c_float) = _2174;
                    _2175 = _1934;
                    *(&mut *((*(_2173 as *mut l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _2175;
                    _2176 = _1935;
                    *(&mut *((*(_2173 as *mut l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _2176;
                    _2177 = _1936;
                    *(&mut *((*(_2173 as *mut l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _2177;
                    _2178 = _1951;
                    *(&mut _1956 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                        _2178.field0;
                    _2179 = memcpy(
                        &mut _1959 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        &mut _1956 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        16 as core::ffi::c_int as uint64_t,
                    );
                    _2180 =
                        *(&mut _1959 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                    *(&mut _1910 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                        _2180;
                    _2181 =
                        *(&mut _1910 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _2182 = *(&mut *((*(&mut _1910 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2183 = *(&mut *((*(&mut _1910 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2184 = *(&mut *((*(&mut _1910 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2001 = llvm_fadd_f32(llvm_fadd_f32(_2181, _2182), llvm_fadd_f32(_2183, _2184));
                    _2185 = _2001;
                    _2186 = _1996;
                    _2187 = *(&mut *(_1977.array)
                        .as_mut_ptr()
                        .offset(_2186 as uint64_t as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2188 = _ZN4astcL3minIfEET_S1_S1_(_2185, _2187);
                    _2001 = _2188;
                    _2189 = _2001;
                    _2190 = _1980;
                    _1980 = llvm_fadd_f32(_2190, _2189);
                    _2191 = _2001;
                    _2192 = _1996;
                    *(&mut *(_1977.array)
                        .as_mut_ptr()
                        .offset(_2192 as uint64_t as int64_t as isize)
                        as *mut core::ffi::c_float) = _2191;
                    _2193 = _1996;
                    _1996 = llvm_add_u32(_2193, 1 as core::ffi::c_int as uint32_t);
                }
            }
            return;
        } else {
            __assert_fail(
                &_OC_str as *const l_array_20_uint8_t as *mut core::ffi::c_void,
                &_OC_str_OC_1 as *const l_array_61_uint8_t as *mut core::ffi::c_void,
                67 as core::ffi::c_int as uint32_t,
                &__PRETTY_FUNCTION___OC__ZL11kmeans_initRK11image_blockjjP7vfloat4
                    as *const l_array_77_uint8_t as *mut core::ffi::c_void,
            );
        }
    } else {
        __assert_fail(
            &_OC_str_OC_4 as *const l_array_16_uint8_t as *mut core::ffi::c_void,
            &_OC_str_OC_1 as *const l_array_61_uint8_t as *mut core::ffi::c_void,
            66 as core::ffi::c_int as uint32_t,
            &__PRETTY_FUNCTION___OC__ZL11kmeans_initRK11image_blockjjP7vfloat4
                as *const l_array_77_uint8_t as *mut core::ffi::c_void,
        );
    };
}
#[inline(never)]
unsafe extern "C" fn _ZL13kmeans_updateRK11image_blockjjP7vfloat4PKh(
    mut _2218: *mut core::ffi::c_void,
    mut _2219: uint32_t,
    mut _2220: uint32_t,
    mut _2221: *mut core::ffi::c_void,
    mut _2222: *mut core::ffi::c_void,
) {
    let mut _2223: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2224: core::ffi::c_float = 0.;
    let mut _2225: core::ffi::c_float = 0.;
    let mut _2226: core::ffi::c_float = 0.;
    let mut _2227: core::ffi::c_float = 0.;
    let mut _2228: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2229: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2230: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2231: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2232: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2233: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2234: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2235: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2236: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2237: core::ffi::c_float = 0.;
    let mut _2238: core::ffi::c_float = 0.;
    let mut _2239: core::ffi::c_float = 0.;
    let mut _2240: core::ffi::c_float = 0.;
    let mut _2241: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2242: core::ffi::c_float = 0.;
    let mut _2243: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2244: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2245: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2246: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2247: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2248: core::ffi::c_float = 0.;
    let mut _2249: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2250: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2251: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2252: core::ffi::c_float = 0.;
    let mut _2253: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2254: core::ffi::c_float = 0.;
    let mut _2255: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2256: core::ffi::c_float = 0.;
    let mut _2257: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2258: core::ffi::c_float = 0.;
    let mut _2259: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2260: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2261: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2262: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2263: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2264: uint32_t = 0;
    let mut _2265: uint32_t = 0;
    let mut _2266: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2267: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2268: l_array_4_struct_AC_l_struct_struct_OC_vfloat4 =
        l_array_4_struct_AC_l_struct_struct_OC_vfloat4 {
            array: [l_struct_struct_OC_vfloat4 {
                field0: l_array_4_float { array: [0.; 4] },
            }; 4],
        };
    let mut _2269: l_array_4_uint8_t = l_array_4_uint8_t { array: [0; 4] };
    let mut _2270: uint32_t = 0;
    let mut _2271: uint8_t = 0;
    let mut _2272: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2273: uint32_t = 0;
    let mut _2274: core::ffi::c_float = 0.;
    let mut _2275: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2276: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2277: uint32_t = 0;
    let mut _2278: uint32_t = 0;
    let mut _2279: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2280: core::ffi::c_float = 0.;
    let mut _2281: core::ffi::c_float = 0.;
    let mut _2282: core::ffi::c_float = 0.;
    let mut _2283: core::ffi::c_float = 0.;
    let mut _2284: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2285: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2286: core::ffi::c_float = 0.;
    let mut _2287: core::ffi::c_float = 0.;
    let mut _2288: core::ffi::c_float = 0.;
    let mut _2289: core::ffi::c_float = 0.;
    let mut _2290: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2291: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2292: core::ffi::c_float = 0.;
    let mut _2293: core::ffi::c_float = 0.;
    let mut _2294: core::ffi::c_float = 0.;
    let mut _2295: core::ffi::c_float = 0.;
    let mut _2296: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2297: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2298: core::ffi::c_float = 0.;
    let mut _2299: core::ffi::c_float = 0.;
    let mut _2300: core::ffi::c_float = 0.;
    let mut _2301: core::ffi::c_float = 0.;
    let mut _2302: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2303: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2304: uint32_t = 0;
    let mut _2305: uint32_t = 0;
    let mut _2306: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2307: uint32_t = 0;
    let mut _2308: uint8_t = 0;
    let mut _2309: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2310: uint32_t = 0;
    let mut _2311: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2312: uint8_t = 0;
    let mut _2313: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2314: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2315: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2316: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2317: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _2318: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _2319: core::ffi::c_float = 0.;
    let mut _2320: core::ffi::c_float = 0.;
    let mut _2321: core::ffi::c_float = 0.;
    let mut _2322: core::ffi::c_float = 0.;
    let mut _2323: core::ffi::c_float = 0.;
    let mut _2324: core::ffi::c_float = 0.;
    let mut _2325: core::ffi::c_float = 0.;
    let mut _2326: core::ffi::c_float = 0.;
    let mut _2327: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2328: core::ffi::c_float = 0.;
    let mut _2329: core::ffi::c_float = 0.;
    let mut _2330: core::ffi::c_float = 0.;
    let mut _2331: core::ffi::c_float = 0.;
    let mut _2332: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2333: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2334: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2335: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2336: uint8_t = 0;
    let mut _2337: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2338: uint8_t = 0;
    let mut _2339: uint32_t = 0;
    let mut _2340: uint32_t = 0;
    let mut _2341: uint32_t = 0;
    let mut _2342: uint32_t = 0;
    let mut _2343: uint8_t = 0;
    let mut _2344: uint32_t = 0;
    let mut _2345: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2346: core::ffi::c_float = 0.;
    let mut _2347: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _2348: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2349: core::ffi::c_float = 0.;
    let mut _2350: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2351: core::ffi::c_float = 0.;
    let mut _2352: core::ffi::c_float = 0.;
    let mut _2353: core::ffi::c_float = 0.;
    let mut _2354: core::ffi::c_float = 0.;
    let mut _2355: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _2356: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _2357: core::ffi::c_float = 0.;
    let mut _2358: core::ffi::c_float = 0.;
    let mut _2359: core::ffi::c_float = 0.;
    let mut _2360: core::ffi::c_float = 0.;
    let mut _2361: core::ffi::c_float = 0.;
    let mut _2362: core::ffi::c_float = 0.;
    let mut _2363: core::ffi::c_float = 0.;
    let mut _2364: core::ffi::c_float = 0.;
    let mut _2365: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2366: core::ffi::c_float = 0.;
    let mut _2367: core::ffi::c_float = 0.;
    let mut _2368: core::ffi::c_float = 0.;
    let mut _2369: core::ffi::c_float = 0.;
    let mut _2370: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2371: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2372: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2373: uint32_t = 0;
    let mut _2374: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2375: uint32_t = 0;
    _2263 = _2218;
    _2264 = _2219;
    _2265 = _2220;
    _2266 = _2221;
    _2267 = _2222;
    _2277 = _2264;
    if _2277 > 0 as core::ffi::c_uint {
        _2278 = _2265;
        if _2278 > 0 as core::ffi::c_uint {
            _2257 = &mut _2259 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
            _2258 = 0 as core::ffi::c_int as libc::c_float;
            _2279 = _2257;
            _2280 = _2258;
            *(_2279 as *mut core::ffi::c_float) = _2280;
            _2281 = _2258;
            *(&mut *((*(_2279 as *mut l_array_4_float)).array)
                .as_mut_ptr()
                .offset(1 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float) = _2281;
            _2282 = _2258;
            *(&mut *((*(_2279 as *mut l_array_4_float)).array)
                .as_mut_ptr()
                .offset(2 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float) = _2282;
            _2283 = _2258;
            *(&mut *((*(_2279 as *mut l_array_4_float)).array)
                .as_mut_ptr()
                .offset(3 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float) = _2283;
            _2284 = _2259;
            *(&mut (*(&mut _2268 as *mut l_array_4_struct_AC_l_struct_struct_OC_vfloat4
                as *mut l_struct_struct_OC_vfloat4))
                .field0 as *mut l_array_4_float) = _2284.field0;
            _2255 = &mut _2260 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
            _2256 = 0 as core::ffi::c_int as libc::c_float;
            _2285 = _2255;
            _2286 = _2256;
            *(_2285 as *mut core::ffi::c_float) = _2286;
            _2287 = _2256;
            *(&mut *((*(_2285 as *mut l_array_4_float)).array)
                .as_mut_ptr()
                .offset(1 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float) = _2287;
            _2288 = _2256;
            *(&mut *((*(_2285 as *mut l_array_4_float)).array)
                .as_mut_ptr()
                .offset(2 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float) = _2288;
            _2289 = _2256;
            *(&mut *((*(_2285 as *mut l_array_4_float)).array)
                .as_mut_ptr()
                .offset(3 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float) = _2289;
            _2290 = _2260;
            *(&mut (*(&mut *(&mut _2268 as *mut l_array_4_struct_AC_l_struct_struct_OC_vfloat4
                as *mut l_struct_struct_OC_vfloat4)
                .offset(1 as core::ffi::c_int as int64_t as isize)
                as *mut l_struct_struct_OC_vfloat4))
                .field0 as *mut l_array_4_float) = _2290.field0;
            _2253 = &mut _2261 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
            _2254 = 0 as core::ffi::c_int as libc::c_float;
            _2291 = _2253;
            _2292 = _2254;
            *(_2291 as *mut core::ffi::c_float) = _2292;
            _2293 = _2254;
            *(&mut *((*(_2291 as *mut l_array_4_float)).array)
                .as_mut_ptr()
                .offset(1 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float) = _2293;
            _2294 = _2254;
            *(&mut *((*(_2291 as *mut l_array_4_float)).array)
                .as_mut_ptr()
                .offset(2 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float) = _2294;
            _2295 = _2254;
            *(&mut *((*(_2291 as *mut l_array_4_float)).array)
                .as_mut_ptr()
                .offset(3 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float) = _2295;
            _2296 = _2261;
            *(&mut (*(&mut *(&mut _2268 as *mut l_array_4_struct_AC_l_struct_struct_OC_vfloat4
                as *mut l_struct_struct_OC_vfloat4)
                .offset(2 as core::ffi::c_int as int64_t as isize)
                as *mut l_struct_struct_OC_vfloat4))
                .field0 as *mut l_array_4_float) = _2296.field0;
            _2251 = &mut _2262 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
            _2252 = 0 as core::ffi::c_int as libc::c_float;
            _2297 = _2251;
            _2298 = _2252;
            *(_2297 as *mut core::ffi::c_float) = _2298;
            _2299 = _2252;
            *(&mut *((*(_2297 as *mut l_array_4_float)).array)
                .as_mut_ptr()
                .offset(1 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float) = _2299;
            _2300 = _2252;
            *(&mut *((*(_2297 as *mut l_array_4_float)).array)
                .as_mut_ptr()
                .offset(2 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float) = _2300;
            _2301 = _2252;
            *(&mut *((*(_2297 as *mut l_array_4_float)).array)
                .as_mut_ptr()
                .offset(3 as core::ffi::c_int as int64_t as isize)
                as *mut core::ffi::c_float) = _2301;
            _2302 = _2262;
            *(&mut (*(&mut *(&mut _2268 as *mut l_array_4_struct_AC_l_struct_struct_OC_vfloat4
                as *mut l_struct_struct_OC_vfloat4)
                .offset(3 as core::ffi::c_int as int64_t as isize)
                as *mut l_struct_struct_OC_vfloat4))
                .field0 as *mut l_array_4_float) = _2302.field0;
            _2303 = memset(
                &mut _2269 as *mut l_array_4_uint8_t as *mut core::ffi::c_void,
                0 as core::ffi::c_int as uint32_t,
                4 as core::ffi::c_int as uint64_t,
            );
            _2270 = 0 as core::ffi::c_int as uint32_t;
            loop {
                _2304 = _2270;
                _2305 = _2264;
                if !(_2304 < _2305) {
                    break;
                }
                _2306 = _2267;
                _2307 = _2270;
                _2308 = *(&mut *(_2306 as *mut uint8_t)
                    .offset(_2307 as uint64_t as int64_t as isize)
                    as *mut uint8_t);
                _2271 = _2308;
                _2309 = _2263;
                _2310 = _2270;
                _2311 = _ZNK11image_block5texelEj(_2309, _2310);
                *(&mut _2272.field0 as *mut l_array_4_float) = _2311.field0;
                _2312 = _2271;
                _2231 = &mut *(_2268.array)
                    .as_mut_ptr()
                    .offset(_2312 as uint64_t as int64_t as isize)
                    as *mut l_struct_struct_OC_vfloat4
                    as *mut core::ffi::c_void;
                _2232 = &mut _2272 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _2313 = _2231;
                _2314 = memcpy(
                    &mut _2234 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    _2313,
                    16 as core::ffi::c_int as uint64_t,
                );
                _2315 = _2232;
                _2316 = memcpy(
                    &mut _2235 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    _2315,
                    16 as core::ffi::c_int as uint64_t,
                );
                _2317 = *(&mut _2234 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                _2318 = *(&mut _2235 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                *(&mut _2229 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) = _2317;
                *(&mut _2230 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) = _2318;
                _2319 = *(&mut _2229 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _2320 = *(&mut _2230 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _2321 = *(&mut *((*(&mut _2229 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _2322 = *(&mut *((*(&mut _2230 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _2323 = *(&mut *((*(&mut _2229 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _2324 = *(&mut *((*(&mut _2230 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _2325 = *(&mut *((*(&mut _2229 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _2326 = *(&mut *((*(&mut _2230 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _2223 = &mut _2228 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _2224 = llvm_fadd_f32(_2319, _2320);
                _2225 = llvm_fadd_f32(_2321, _2322);
                _2226 = llvm_fadd_f32(_2323, _2324);
                _2227 = llvm_fadd_f32(_2325, _2326);
                _2327 = _2223;
                _2328 = _2224;
                *(_2327 as *mut core::ffi::c_float) = _2328;
                _2329 = _2225;
                *(&mut *((*(_2327 as *mut l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _2329;
                _2330 = _2226;
                *(&mut *((*(_2327 as *mut l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _2330;
                _2331 = _2227;
                *(&mut *((*(_2327 as *mut l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _2331;
                _2332 = _2228;
                *(&mut _2233 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                    _2332.field0;
                _2333 = _2231;
                _2334 = memcpy(
                    _2333,
                    &mut _2233 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16 as core::ffi::c_int as uint64_t,
                );
                _2335 = _2231;
                _2336 = _2271;
                _2337 = &mut *(_2269.array)
                    .as_mut_ptr()
                    .offset(_2336 as uint64_t as int64_t as isize)
                    as *mut uint8_t as *mut core::ffi::c_void;
                _2338 = *(_2337 as *mut uint8_t);
                *(_2337 as *mut uint8_t) = llvm_add_u8(_2338, 1 as core::ffi::c_int as uint8_t);
                _2339 = _2270;
                _2270 = llvm_add_u32(_2339, 1 as core::ffi::c_int as uint32_t);
            }
            _2273 = 0 as core::ffi::c_int as uint32_t;
            loop {
                _2340 = _2273;
                _2341 = _2265;
                if !(_2340 < _2341) {
                    break;
                }
                _2342 = _2273;
                _2343 = *(&mut *(_2269.array)
                    .as_mut_ptr()
                    .offset(_2342 as uint64_t as int64_t as isize)
                    as *mut uint8_t);
                _2274 = llvm_fdiv_f32(
                    1 as core::ffi::c_int as libc::c_float,
                    _2343 as core::ffi::c_float,
                );
                _2344 = _2273;
                _2345 = memcpy(
                    &mut _2276 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut *(_2268.array)
                        .as_mut_ptr()
                        .offset(_2344 as uint64_t as int64_t as isize)
                        as *mut l_struct_struct_OC_vfloat4
                        as *mut core::ffi::c_void,
                    16 as core::ffi::c_int as uint64_t,
                );
                _2346 = _2274;
                _2347 = *(&mut _2276.field0 as *mut l_array_4_float);
                *(&mut _2247 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) = _2347;
                _2248 = _2346;
                _2348 = memcpy(
                    &mut _2249 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    &mut _2247 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16 as core::ffi::c_int as uint64_t,
                );
                _2349 = _2248;
                _2241 = &mut _2250 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _2242 = _2349;
                _2350 = _2241;
                _2351 = _2242;
                *(_2350 as *mut core::ffi::c_float) = _2351;
                _2352 = _2242;
                *(&mut *((*(_2350 as *mut l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _2352;
                _2353 = _2242;
                *(&mut *((*(_2350 as *mut l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _2353;
                _2354 = _2242;
                *(&mut *((*(_2350 as *mut l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _2354;
                _2355 = *(&mut _2249 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                _2356 = *(&mut _2250 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                *(&mut _2244 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) = _2355;
                *(&mut _2245 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) = _2356;
                _2357 = *(&mut _2244 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _2358 = *(&mut _2245 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                _2359 = *(&mut *((*(&mut _2244 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _2360 = *(&mut *((*(&mut _2245 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _2361 = *(&mut *((*(&mut _2244 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _2362 = *(&mut *((*(&mut _2245 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _2363 = *(&mut *((*(&mut _2244 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _2364 = *(&mut *((*(&mut _2245 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _2236 = &mut _2243 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                _2237 = llvm_fmul_f32(_2357, _2358);
                _2238 = llvm_fmul_f32(_2359, _2360);
                _2239 = llvm_fmul_f32(_2361, _2362);
                _2240 = llvm_fmul_f32(_2363, _2364);
                _2365 = _2236;
                _2366 = _2237;
                *(_2365 as *mut core::ffi::c_float) = _2366;
                _2367 = _2238;
                *(&mut *((*(_2365 as *mut l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _2367;
                _2368 = _2239;
                *(&mut *((*(_2365 as *mut l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _2368;
                _2369 = _2240;
                *(&mut *((*(_2365 as *mut l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as core::ffi::c_int as int64_t as isize)
                    as *mut core::ffi::c_float) = _2369;
                _2370 = _2243;
                *(&mut _2246 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                    _2370.field0;
                _2371 = _2246;
                *(&mut _2275.field0 as *mut l_array_4_float) = _2371.field0;
                _2372 = _2266;
                _2373 = _2273;
                _2374 = memcpy(
                    &mut *(_2372 as *mut l_struct_struct_OC_vfloat4)
                        .offset(_2373 as uint64_t as int64_t as isize)
                        as *mut l_struct_struct_OC_vfloat4
                        as *mut core::ffi::c_void,
                    &mut _2275 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                    16 as core::ffi::c_int as uint64_t,
                );
                _2375 = _2273;
                _2273 = llvm_add_u32(_2375, 1 as core::ffi::c_int as uint32_t);
            }
            return;
        } else {
            __assert_fail(
                &_OC_str as *const l_array_20_uint8_t as *mut core::ffi::c_void,
                &_OC_str_OC_1 as *const l_array_61_uint8_t as *mut core::ffi::c_void,
                218 as core::ffi::c_int as uint32_t,
                &__PRETTY_FUNCTION___OC__ZL13kmeans_updateRK11image_blockjjP7vfloat4PKh
                    as *const l_array_96_uint8_t as *mut core::ffi::c_void,
            );
        }
    } else {
        __assert_fail(
            &_OC_str_OC_4 as *const l_array_16_uint8_t as *mut core::ffi::c_void,
            &_OC_str_OC_1 as *const l_array_61_uint8_t as *mut core::ffi::c_void,
            217 as core::ffi::c_int as uint32_t,
            &__PRETTY_FUNCTION___OC__ZL13kmeans_updateRK11image_blockjjP7vfloat4PKh
                as *const l_array_96_uint8_t as *mut core::ffi::c_void,
        );
    };
}
#[inline(never)]
unsafe extern "C" fn _ZL13kmeans_assignRK11image_blockjjPK7vfloat4Ph(
    mut _2390: *mut core::ffi::c_void,
    mut _2391: uint32_t,
    mut _2392: uint32_t,
    mut _2393: *mut core::ffi::c_void,
    mut _2394: *mut core::ffi::c_void,
) {
    let mut _2395: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2396: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2397: core::ffi::c_float = 0.;
    let mut _2398: core::ffi::c_float = 0.;
    let mut _2399: core::ffi::c_float = 0.;
    let mut _2400: core::ffi::c_float = 0.;
    let mut _2401: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2402: core::ffi::c_float = 0.;
    let mut _2403: core::ffi::c_float = 0.;
    let mut _2404: core::ffi::c_float = 0.;
    let mut _2405: core::ffi::c_float = 0.;
    let mut _2406: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2407: core::ffi::c_float = 0.;
    let mut _2408: core::ffi::c_float = 0.;
    let mut _2409: core::ffi::c_float = 0.;
    let mut _2410: core::ffi::c_float = 0.;
    let mut _2411: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2412: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2413: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2414: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2415: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2416: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2417: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2418: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2419: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2420: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2421: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2422: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2423: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2424: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2425: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2426: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2427: uint32_t = 0;
    let mut _2428: uint32_t = 0;
    let mut _2429: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2430: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2431: l_array_4_uint8_t = l_array_4_uint8_t { array: [0; 4] };
    let mut _2432: uint32_t = 0;
    let mut _2433: core::ffi::c_float = 0.;
    let mut _2434: uint32_t = 0;
    let mut _2435: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2436: uint32_t = 0;
    let mut _2437: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2438: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2439: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2440: core::ffi::c_float = 0.;
    let mut _2441: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2442: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2443: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2444: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2445: uint8_t = 0;
    let mut _2446: uint32_t = 0;
    let mut _2447: uint32_t = 0;
    let mut _2448: uint32_t = 0;
    let mut _2449: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2450: uint32_t = 0;
    let mut _2451: uint32_t = 0;
    let mut _2452: core::ffi::c_float = 0.;
    let mut _2453: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2454: uint32_t = 0;
    let mut _2455: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2456: uint32_t = 0;
    let mut _2457: uint32_t = 0;
    let mut _2458: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2459: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2460: uint32_t = 0;
    let mut _2461: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2462: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _2463: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _2464: core::ffi::c_float = 0.;
    let mut _2465: core::ffi::c_float = 0.;
    let mut _2466: core::ffi::c_float = 0.;
    let mut _2467: core::ffi::c_float = 0.;
    let mut _2468: core::ffi::c_float = 0.;
    let mut _2469: core::ffi::c_float = 0.;
    let mut _2470: core::ffi::c_float = 0.;
    let mut _2471: core::ffi::c_float = 0.;
    let mut _2472: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2473: core::ffi::c_float = 0.;
    let mut _2474: core::ffi::c_float = 0.;
    let mut _2475: core::ffi::c_float = 0.;
    let mut _2476: core::ffi::c_float = 0.;
    let mut _2477: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2478: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2479: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2480: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _2481: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _2482: core::ffi::c_float = 0.;
    let mut _2483: core::ffi::c_float = 0.;
    let mut _2484: core::ffi::c_float = 0.;
    let mut _2485: core::ffi::c_float = 0.;
    let mut _2486: core::ffi::c_float = 0.;
    let mut _2487: core::ffi::c_float = 0.;
    let mut _2488: core::ffi::c_float = 0.;
    let mut _2489: core::ffi::c_float = 0.;
    let mut _2490: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2491: core::ffi::c_float = 0.;
    let mut _2492: core::ffi::c_float = 0.;
    let mut _2493: core::ffi::c_float = 0.;
    let mut _2494: core::ffi::c_float = 0.;
    let mut _2495: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2496: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2497: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2498: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _2499: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _2500: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2501: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2502: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _2503: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _2504: core::ffi::c_float = 0.;
    let mut _2505: core::ffi::c_float = 0.;
    let mut _2506: core::ffi::c_float = 0.;
    let mut _2507: core::ffi::c_float = 0.;
    let mut _2508: core::ffi::c_float = 0.;
    let mut _2509: core::ffi::c_float = 0.;
    let mut _2510: core::ffi::c_float = 0.;
    let mut _2511: core::ffi::c_float = 0.;
    let mut _2512: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2513: core::ffi::c_float = 0.;
    let mut _2514: core::ffi::c_float = 0.;
    let mut _2515: core::ffi::c_float = 0.;
    let mut _2516: core::ffi::c_float = 0.;
    let mut _2517: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2518: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2519: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _2520: core::ffi::c_float = 0.;
    let mut _2521: core::ffi::c_float = 0.;
    let mut _2522: core::ffi::c_float = 0.;
    let mut _2523: core::ffi::c_float = 0.;
    let mut _2524: core::ffi::c_float = 0.;
    let mut _2525: core::ffi::c_float = 0.;
    let mut _2526: core::ffi::c_float = 0.;
    let mut _2527: uint32_t = 0;
    let mut _2528: uint32_t = 0;
    let mut _2529: uint32_t = 0;
    let mut _2530: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2531: uint32_t = 0;
    let mut _2532: uint32_t = 0;
    let mut _2533: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2534: uint8_t = 0;
    let mut _2535: uint32_t = 0;
    let mut _2536: uint32_t = 0;
    let mut _2537: uint32_t = 0;
    let mut _2538: uint32_t = 0;
    let mut _2539: uint8_t = 0;
    let mut _2540: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2541: uint32_t = 0;
    let mut _2542: uint8_t = 0;
    let mut _2543: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2544: uint8_t = 0;
    let mut _2545: uint32_t = 0;
    let mut _2546: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2547: uint8_t = 0;
    let mut _2548: uint32_t = 0;
    let mut _2549: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2550: uint32_t = 0;
    let mut _2551: uint32_t = 0;
    let mut _2552: uint8_t = 0;
    _2426 = _2390;
    _2427 = _2391;
    _2428 = _2392;
    _2429 = _2393;
    _2430 = _2394;
    _2447 = _2427;
    if _2447 > 0 as core::ffi::c_uint {
        _2448 = _2428;
        if _2448 > 0 as core::ffi::c_uint {
            _2449 = memset(
                &mut _2431 as *mut l_array_4_uint8_t as *mut core::ffi::c_void,
                0 as core::ffi::c_int as uint32_t,
                4 as core::ffi::c_int as uint64_t,
            );
            _2432 = 0 as core::ffi::c_int as uint32_t;
            loop {
                _2450 = _2432;
                _2451 = _2427;
                if !(_2450 < _2451) {
                    break;
                }
                _2452 = _ZNSt14numeric_limitsIfE3maxEv();
                _2433 = _2452;
                _2434 = 0 as core::ffi::c_int as uint32_t;
                _2453 = _2426;
                _2454 = _2432;
                _2455 = _ZNK11image_block5texelEj(_2453, _2454);
                *(&mut _2435.field0 as *mut l_array_4_float) = _2455.field0;
                _2436 = 0 as core::ffi::c_int as uint32_t;
                loop {
                    _2456 = _2436;
                    _2457 = _2428;
                    if !(_2456 < _2457) {
                        break;
                    }
                    _2458 = memcpy(
                        &mut _2438 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        &mut _2435 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        16 as core::ffi::c_int as uint64_t,
                    );
                    _2459 = _2429;
                    _2460 = _2436;
                    _2461 = memcpy(
                        &mut _2439 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        &mut *(_2459 as *mut l_struct_struct_OC_vfloat4)
                            .offset(_2460 as uint64_t as int64_t as isize)
                            as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_void,
                        16 as core::ffi::c_int as uint64_t,
                    );
                    _2462 = *(&mut _2438.field0 as *mut l_array_4_float);
                    _2463 = *(&mut _2439.field0 as *mut l_array_4_float);
                    *(&mut _2424 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                        _2462;
                    *(&mut _2425 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                        _2463;
                    _2464 =
                        *(&mut _2424 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _2465 =
                        *(&mut _2425 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _2466 = *(&mut *((*(&mut _2424 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2467 = *(&mut *((*(&mut _2425 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2468 = *(&mut *((*(&mut _2424 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2469 = *(&mut *((*(&mut _2425 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2470 = *(&mut *((*(&mut _2424 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2471 = *(&mut *((*(&mut _2425 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2396 = &mut _2423 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                    _2397 = llvm_fsub_f32(_2464, _2465);
                    _2398 = llvm_fsub_f32(_2466, _2467);
                    _2399 = llvm_fsub_f32(_2468, _2469);
                    _2400 = llvm_fsub_f32(_2470, _2471);
                    _2472 = _2396;
                    _2473 = _2397;
                    *(_2472 as *mut core::ffi::c_float) = _2473;
                    _2474 = _2398;
                    *(&mut *((*(_2472 as *mut l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _2474;
                    _2475 = _2399;
                    *(&mut *((*(_2472 as *mut l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _2475;
                    _2476 = _2400;
                    *(&mut *((*(_2472 as *mut l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _2476;
                    _2477 = _2423;
                    *(&mut _2437.field0 as *mut l_array_4_float) = _2477.field0;
                    _2478 = memcpy(
                        &mut _2442 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        &mut _2437 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        16 as core::ffi::c_int as uint64_t,
                    );
                    _2479 = memcpy(
                        &mut _2443 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        &mut _2437 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        16 as core::ffi::c_int as uint64_t,
                    );
                    _2480 = *(&mut _2442.field0 as *mut l_array_4_float);
                    _2481 = *(&mut _2443.field0 as *mut l_array_4_float);
                    *(&mut _2421 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                        _2480;
                    *(&mut _2422 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                        _2481;
                    _2482 =
                        *(&mut _2421 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _2483 =
                        *(&mut _2422 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _2484 = *(&mut *((*(&mut _2421 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2485 = *(&mut *((*(&mut _2422 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2486 = *(&mut *((*(&mut _2421 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2487 = *(&mut *((*(&mut _2422 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2488 = *(&mut *((*(&mut _2421 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2489 = *(&mut *((*(&mut _2422 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2401 = &mut _2420 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                    _2402 = llvm_fmul_f32(_2482, _2483);
                    _2403 = llvm_fmul_f32(_2484, _2485);
                    _2404 = llvm_fmul_f32(_2486, _2487);
                    _2405 = llvm_fmul_f32(_2488, _2489);
                    _2490 = _2401;
                    _2491 = _2402;
                    *(_2490 as *mut core::ffi::c_float) = _2491;
                    _2492 = _2403;
                    *(&mut *((*(_2490 as *mut l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _2492;
                    _2493 = _2404;
                    *(&mut *((*(_2490 as *mut l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _2493;
                    _2494 = _2405;
                    *(&mut *((*(_2490 as *mut l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _2494;
                    _2495 = _2420;
                    *(&mut _2441.field0 as *mut l_array_4_float) = _2495.field0;
                    _2496 = _2426;
                    _2497 = memcpy(
                        &mut _2444 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        &mut (*(_2496 as *mut l_struct_struct_OC_image_block)).field9
                            as *mut l_struct_struct_OC_vfloat4
                            as *mut core::ffi::c_void,
                        16 as core::ffi::c_int as uint64_t,
                    );
                    _2498 = *(&mut _2441.field0 as *mut l_array_4_float);
                    _2499 = *(&mut _2444.field0 as *mut l_array_4_float);
                    *(&mut _2414 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                        _2498;
                    *(&mut _2415 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                        _2499;
                    _2500 = memcpy(
                        &mut _2417 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        &mut _2414 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        16 as core::ffi::c_int as uint64_t,
                    );
                    _2501 = memcpy(
                        &mut _2418 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        &mut _2415 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        16 as core::ffi::c_int as uint64_t,
                    );
                    _2502 =
                        *(&mut _2417 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                    _2503 =
                        *(&mut _2418 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                    *(&mut _2412 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                        _2502;
                    *(&mut _2413 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                        _2503;
                    _2504 =
                        *(&mut _2412 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _2505 =
                        *(&mut _2413 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _2506 = *(&mut *((*(&mut _2412 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2507 = *(&mut *((*(&mut _2413 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2508 = *(&mut *((*(&mut _2412 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2509 = *(&mut *((*(&mut _2413 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2510 = *(&mut *((*(&mut _2412 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2511 = *(&mut *((*(&mut _2413 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2406 = &mut _2411 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
                    _2407 = llvm_fmul_f32(_2504, _2505);
                    _2408 = llvm_fmul_f32(_2506, _2507);
                    _2409 = llvm_fmul_f32(_2508, _2509);
                    _2410 = llvm_fmul_f32(_2510, _2511);
                    _2512 = _2406;
                    _2513 = _2407;
                    *(_2512 as *mut core::ffi::c_float) = _2513;
                    _2514 = _2408;
                    *(&mut *((*(_2512 as *mut l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _2514;
                    _2515 = _2409;
                    *(&mut *((*(_2512 as *mut l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _2515;
                    _2516 = _2410;
                    *(&mut *((*(_2512 as *mut l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float) = _2516;
                    _2517 = _2411;
                    *(&mut _2416 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                        _2517.field0;
                    _2518 = memcpy(
                        &mut _2419 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        &mut _2416 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void,
                        16 as core::ffi::c_int as uint64_t,
                    );
                    _2519 =
                        *(&mut _2419 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float);
                    *(&mut _2395 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                        _2519;
                    _2520 =
                        *(&mut _2395 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_float);
                    _2521 = *(&mut *((*(&mut _2395 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2522 = *(&mut *((*(&mut _2395 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2523 = *(&mut *((*(&mut _2395 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut core::ffi::c_float);
                    _2440 = llvm_fadd_f32(llvm_fadd_f32(_2520, _2521), llvm_fadd_f32(_2522, _2523));
                    _2524 = _2440;
                    _2525 = _2433;
                    if llvm_fcmp_olt(_2524 as core::ffi::c_double, _2525 as libc::c_double) != 0 {
                        _2526 = _2440;
                        _2433 = _2526;
                        _2527 = _2436;
                        _2434 = _2527;
                    }
                    _2528 = _2436;
                    _2436 = llvm_add_u32(_2528, 1 as core::ffi::c_int as uint32_t);
                }
                _2529 = _2434;
                _2530 = _2430;
                _2531 = _2432;
                *(&mut *(_2530 as *mut uint8_t).offset(_2531 as uint64_t as int64_t as isize)
                    as *mut uint8_t) = _2529 as uint8_t;
                _2532 = _2434;
                _2533 = &mut *(_2431.array)
                    .as_mut_ptr()
                    .offset(_2532 as uint64_t as int64_t as isize)
                    as *mut uint8_t as *mut core::ffi::c_void;
                _2534 = *(_2533 as *mut uint8_t);
                *(_2533 as *mut uint8_t) = llvm_add_u8(_2534, 1 as core::ffi::c_int as uint8_t);
                _2535 = _2432;
                _2432 = llvm_add_u32(_2535, 1 as core::ffi::c_int as uint32_t);
            }
            loop {
                _2445 = 0 as core::ffi::c_int as uint8_t;
                _2446 = 0 as core::ffi::c_int as uint32_t;
                loop {
                    _2536 = _2446;
                    _2537 = _2428;
                    if !(_2536 < _2537) {
                        break;
                    }
                    _2538 = _2446;
                    _2539 = *(&mut *(_2431.array)
                        .as_mut_ptr()
                        .offset(_2538 as uint64_t as int64_t as isize)
                        as *mut uint8_t);
                    if _2539 as uint32_t == 0 as core::ffi::c_uint {
                        _2540 = _2430;
                        _2541 = _2446;
                        _2542 = *(&mut *(_2540 as *mut uint8_t)
                            .offset(_2541 as uint64_t as int64_t as isize)
                            as *mut uint8_t);
                        _2543 = &mut *(_2431.array)
                            .as_mut_ptr()
                            .offset(_2542 as uint64_t as int64_t as isize)
                            as *mut uint8_t
                            as *mut core::ffi::c_void;
                        _2544 = *(_2543 as *mut uint8_t);
                        *(_2543 as *mut uint8_t) =
                            llvm_add_u8(_2544, -(1 as core::ffi::c_int) as uint8_t);
                        _2545 = _2446;
                        _2546 = &mut *(_2431.array)
                            .as_mut_ptr()
                            .offset(_2545 as uint64_t as int64_t as isize)
                            as *mut uint8_t
                            as *mut core::ffi::c_void;
                        _2547 = *(_2546 as *mut uint8_t);
                        *(_2546 as *mut uint8_t) =
                            llvm_add_u8(_2547, 1 as core::ffi::c_int as uint8_t);
                        _2548 = _2446;
                        _2549 = _2430;
                        _2550 = _2446;
                        *(&mut *(_2549 as *mut uint8_t)
                            .offset(_2550 as uint64_t as int64_t as isize)
                            as *mut uint8_t) = _2548 as uint8_t;
                        _2445 = 1 as core::ffi::c_int as uint8_t;
                    }
                    _2551 = _2446;
                    _2446 = llvm_add_u32(_2551, 1 as core::ffi::c_int as uint32_t);
                }
                _2552 = _2445;
                if !(_2552 as core::ffi::c_uint & 1 as libc::c_uint != 0) {
                    break;
                }
            }
            return;
        } else {
            __assert_fail(
                &_OC_str as *const l_array_20_uint8_t as *mut core::ffi::c_void,
                &_OC_str_OC_1 as *const l_array_61_uint8_t as *mut core::ffi::c_void,
                154 as core::ffi::c_int as uint32_t,
                &__PRETTY_FUNCTION___OC__ZL13kmeans_assignRK11image_blockjjPK7vfloat4Ph
                    as *const l_array_96_uint8_t as *mut core::ffi::c_void,
            );
        }
    } else {
        __assert_fail(
            &_OC_str_OC_4 as *const l_array_16_uint8_t as *mut core::ffi::c_void,
            &_OC_str_OC_1 as *const l_array_61_uint8_t as *mut core::ffi::c_void,
            153 as core::ffi::c_int as uint32_t,
            &__PRETTY_FUNCTION___OC__ZL13kmeans_assignRK11image_blockjjPK7vfloat4Ph
                as *const l_array_96_uint8_t as *mut core::ffi::c_void,
        );
    };
}
#[inline(never)]
unsafe extern "C" fn _ZN4astcL3minIhEET_S1_S1_(mut _2578: uint8_t, mut _2579: uint8_t) -> uint8_t {
    let mut _2580: uint8_t = 0;
    let mut _2581: uint8_t = 0;
    let mut _2582: uint8_t = 0;
    let mut _2583: uint8_t = 0;
    let mut _2584: uint8_t = 0;
    let mut _2585: uint8_t = 0;
    let mut _2586: uint8_t = 0;
    let mut _2586__PHI_TEMPORARY: uint8_t = 0;
    _2580 = _2578;
    _2581 = _2579;
    _2582 = _2580;
    _2583 = _2581;
    if (_2582 as uint32_t as int32_t) < _2583 as uint32_t as int32_t {
        _2584 = _2580;
        _2586__PHI_TEMPORARY = _2584;
    } else {
        _2585 = _2581;
        _2586__PHI_TEMPORARY = _2585;
    }
    _2586 = _2586__PHI_TEMPORARY;
    return _2586;
}
#[inline(never)]
unsafe extern "C" fn _ZL29count_partition_mismatch_bitsRK21block_size_descriptorjPKmPh(
    mut _2590: *mut core::ffi::c_void,
    mut _2591: uint32_t,
    mut _2592: *mut core::ffi::c_void,
    mut _2593: *mut core::ffi::c_void,
) {
    let mut current_block: u64;
    let mut _2594: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2595: uint32_t = 0;
    let mut _2596: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2597: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2598: uint32_t = 0;
    let mut _2599: uint32_t = 0;
    let mut _2600: uint32_t = 0;
    let mut _2601: uint32_t = 0;
    let mut _2602: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2603: uint32_t = 0;
    let mut _2604: uint32_t = 0;
    let mut _2605: uint32_t = 0;
    let mut _2606: uint32_t = 0;
    let mut _2607: uint32_t = 0;
    let mut _2608: uint32_t = 0;
    let mut _2609: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2610: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2611: uint32_t = 0;
    let mut _2612: uint8_t = 0;
    let mut _2613: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2614: uint32_t = 0;
    let mut _2615: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2616: uint32_t = 0;
    let mut _2617: uint8_t = 0;
    let mut _2618: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2619: uint32_t = 0;
    let mut _2620: uint8_t = 0;
    let mut _2621: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2622: uint8_t = 0;
    let mut _2623: uint32_t = 0;
    let mut _2624: uint32_t = 0;
    let mut _2625: uint32_t = 0;
    let mut _2626: uint32_t = 0;
    let mut _2627: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2628: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2629: uint32_t = 0;
    let mut _2630: uint8_t = 0;
    let mut _2631: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2632: uint32_t = 0;
    let mut _2633: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2634: uint32_t = 0;
    let mut _2635: uint8_t = 0;
    let mut _2636: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2637: uint32_t = 0;
    let mut _2638: uint8_t = 0;
    let mut _2639: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2640: uint8_t = 0;
    let mut _2641: uint32_t = 0;
    let mut _2642: uint32_t = 0;
    let mut _2643: uint32_t = 0;
    let mut _2644: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2645: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2646: uint32_t = 0;
    let mut _2647: uint8_t = 0;
    let mut _2648: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2649: uint32_t = 0;
    let mut _2650: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2651: uint32_t = 0;
    let mut _2652: uint8_t = 0;
    let mut _2653: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2654: uint32_t = 0;
    let mut _2655: uint8_t = 0;
    let mut _2656: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2657: uint8_t = 0;
    let mut _2658: uint32_t = 0;
    _2594 = _2590;
    _2595 = _2591;
    _2596 = _2592;
    _2597 = _2593;
    _2602 = _2594;
    _2603 = _2595;
    _2604 = *(&mut *((*(&mut (*(_2602 as *mut l_struct_struct_OC_block_size_descriptor)).field11
        as *mut l_array_4_uint32_t))
        .array)
        .as_mut_ptr()
        .offset(
            (llvm_sub_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                _2603,
                1 as core::ffi::c_int as uint32_t,
            ) as uint64_t as int64_t as isize,
        ) as *mut uint32_t);
    _2598 = _2604;
    _2605 = _2598;
    if _2605 > 0 as core::ffi::c_uint {
        _2606 = _2595;
        if _2606 == 2 as core::ffi::c_uint {
            _2599 = 0 as core::ffi::c_int as uint32_t;
            loop {
                _2607 = _2599;
                _2608 = _2598;
                if !(_2607 < _2608) {
                    current_block = 15636031872310603809;
                    break;
                }
                _2609 = _2596;
                _2610 = _2594;
                _2611 = _2599;
                _2612 = _ZL19partition_mismatch2PKmS0_(
                    _2609,
                    &mut *((*(&mut *((*(&mut (*(_2610
                        as *mut l_struct_struct_OC_block_size_descriptor))
                        .field20
                        as *mut l_array_1024_struct_AC_l_array_2_uint64_t))
                        .array)
                        .as_mut_ptr()
                        .offset(_2611 as uint64_t as int64_t as isize)
                        as *mut l_array_2_uint64_t))
                        .array)
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as int64_t as isize)
                        as *mut uint64_t as *mut core::ffi::c_void,
                );
                _2613 = _2597;
                _2614 = _2599;
                *(&mut *(_2613 as *mut uint8_t).offset(_2614 as uint64_t as int64_t as isize)
                    as *mut uint8_t) = _2612;
                _2615 = _2597;
                _2616 = _2599;
                _2617 = *(&mut *(_2615 as *mut uint8_t)
                    .offset(_2616 as uint64_t as int64_t as isize)
                    as *mut uint8_t);
                if !((_2617 as uint32_t as int32_t) < 64 as core::ffi::c_uint as int32_t) {
                    current_block = 4326507561247619847;
                    break;
                }
                _2618 = _2597;
                _2619 = _2599;
                _2620 = *(&mut *(_2618 as *mut uint8_t)
                    .offset(_2619 as uint64_t as int64_t as isize)
                    as *mut uint8_t);
                _2621 = _2594;
                _2622 = *(&mut (*(_2621 as *mut l_struct_struct_OC_block_size_descriptor)).field3
                    as *mut uint8_t);
                if !((_2620 as uint32_t as int32_t) < _2622 as uint32_t as int32_t) {
                    current_block = 8915113306104255255;
                    break;
                }
                _2623 = _2599;
                _2599 = llvm_add_u32(_2623, 1 as core::ffi::c_int as uint32_t);
            }
            match current_block {
                15636031872310603809 => {}
                _ => match current_block {
                    4326507561247619847 => {
                        __assert_fail(
                                &_OC_str_OC_6 as *const l_array_45_uint8_t
                                    as *mut core::ffi::c_void,
                                &_OC_str_OC_1 as *const l_array_61_uint8_t
                                    as *mut core::ffi::c_void,
                                379 as core::ffi::c_int as uint32_t,
                                &__PRETTY_FUNCTION___OC__ZL29count_partition_mismatch_bitsRK21block_size_descriptorjPKmPh
                                    as *const l_array_109_uint8_t as *mut core::ffi::c_void,
                            );
                    }
                    _ => {
                        __assert_fail(
                                &_OC_str_OC_7 as *const l_array_37_uint8_t
                                    as *mut core::ffi::c_void,
                                &_OC_str_OC_1 as *const l_array_61_uint8_t
                                    as *mut core::ffi::c_void,
                                380 as core::ffi::c_int as uint32_t,
                                &__PRETTY_FUNCTION___OC__ZL29count_partition_mismatch_bitsRK21block_size_descriptorjPKmPh
                                    as *const l_array_109_uint8_t as *mut core::ffi::c_void,
                            );
                    }
                },
            }
        } else {
            _2624 = _2595;
            if _2624 == 3 as core::ffi::c_uint {
                _2600 = 0 as core::ffi::c_int as uint32_t;
                loop {
                    _2625 = _2600;
                    _2626 = _2598;
                    if !(_2625 < _2626) {
                        current_block = 15636031872310603809;
                        break;
                    }
                    _2627 = _2596;
                    _2628 = _2594;
                    _2629 = _2600;
                    _2630 = _ZL19partition_mismatch3PKmS0_(
                        _2627,
                        &mut *((*(&mut *((*(&mut (*(_2628
                            as *mut l_struct_struct_OC_block_size_descriptor))
                            .field21
                            as *mut l_array_1024_struct_AC_l_array_3_uint64_t))
                            .array)
                            .as_mut_ptr()
                            .offset(_2629 as uint64_t as int64_t as isize)
                            as *mut l_array_3_uint64_t))
                            .array)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as int64_t as isize)
                            as *mut uint64_t as *mut core::ffi::c_void,
                    );
                    _2631 = _2597;
                    _2632 = _2600;
                    *(&mut *(_2631 as *mut uint8_t).offset(_2632 as uint64_t as int64_t as isize)
                        as *mut uint8_t) = _2630;
                    _2633 = _2597;
                    _2634 = _2600;
                    _2635 = *(&mut *(_2633 as *mut uint8_t)
                        .offset(_2634 as uint64_t as int64_t as isize)
                        as *mut uint8_t);
                    if !((_2635 as uint32_t as int32_t) < 64 as core::ffi::c_uint as int32_t) {
                        current_block = 14803377394991816690;
                        break;
                    }
                    _2636 = _2597;
                    _2637 = _2600;
                    _2638 = *(&mut *(_2636 as *mut uint8_t)
                        .offset(_2637 as uint64_t as int64_t as isize)
                        as *mut uint8_t);
                    _2639 = _2594;
                    _2640 = *(&mut (*(_2639 as *mut l_struct_struct_OC_block_size_descriptor))
                        .field3 as *mut uint8_t);
                    if !((_2638 as uint32_t as int32_t) < _2640 as uint32_t as int32_t) {
                        current_block = 10783160623487490410;
                        break;
                    }
                    _2641 = _2600;
                    _2600 = llvm_add_u32(_2641, 1 as core::ffi::c_int as uint32_t);
                }
                match current_block {
                    15636031872310603809 => {}
                    _ => match current_block {
                        14803377394991816690 => {
                            __assert_fail(
                                    &_OC_str_OC_6 as *const l_array_45_uint8_t
                                        as *mut core::ffi::c_void,
                                    &_OC_str_OC_1 as *const l_array_61_uint8_t
                                        as *mut core::ffi::c_void,
                                    388 as core::ffi::c_int as uint32_t,
                                    &__PRETTY_FUNCTION___OC__ZL29count_partition_mismatch_bitsRK21block_size_descriptorjPKmPh
                                        as *const l_array_109_uint8_t as *mut core::ffi::c_void,
                                );
                        }
                        _ => {
                            __assert_fail(
                                    &_OC_str_OC_7 as *const l_array_37_uint8_t
                                        as *mut core::ffi::c_void,
                                    &_OC_str_OC_1 as *const l_array_61_uint8_t
                                        as *mut core::ffi::c_void,
                                    389 as core::ffi::c_int as uint32_t,
                                    &__PRETTY_FUNCTION___OC__ZL29count_partition_mismatch_bitsRK21block_size_descriptorjPKmPh
                                        as *const l_array_109_uint8_t as *mut core::ffi::c_void,
                                );
                        }
                    },
                }
            } else {
                _2601 = 0 as core::ffi::c_int as uint32_t;
                loop {
                    _2642 = _2601;
                    _2643 = _2598;
                    if !(_2642 < _2643) {
                        current_block = 15636031872310603809;
                        break;
                    }
                    _2644 = _2596;
                    _2645 = _2594;
                    _2646 = _2601;
                    _2647 = _ZL19partition_mismatch4PKmS0_(
                        _2644,
                        &mut *((*(&mut *((*(&mut (*(_2645
                            as *mut l_struct_struct_OC_block_size_descriptor))
                            .field22
                            as *mut l_array_1024_struct_AC_l_array_4_uint64_t))
                            .array)
                            .as_mut_ptr()
                            .offset(_2646 as uint64_t as int64_t as isize)
                            as *mut l_array_4_uint64_t))
                            .array)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as int64_t as isize)
                            as *mut uint64_t as *mut core::ffi::c_void,
                    );
                    _2648 = _2597;
                    _2649 = _2601;
                    *(&mut *(_2648 as *mut uint8_t).offset(_2649 as uint64_t as int64_t as isize)
                        as *mut uint8_t) = _2647;
                    _2650 = _2597;
                    _2651 = _2601;
                    _2652 = *(&mut *(_2650 as *mut uint8_t)
                        .offset(_2651 as uint64_t as int64_t as isize)
                        as *mut uint8_t);
                    if !((_2652 as uint32_t as int32_t) < 64 as core::ffi::c_uint as int32_t) {
                        current_block = 10557770493418402859;
                        break;
                    }
                    _2653 = _2597;
                    _2654 = _2601;
                    _2655 = *(&mut *(_2653 as *mut uint8_t)
                        .offset(_2654 as uint64_t as int64_t as isize)
                        as *mut uint8_t);
                    _2656 = _2594;
                    _2657 = *(&mut (*(_2656 as *mut l_struct_struct_OC_block_size_descriptor))
                        .field3 as *mut uint8_t);
                    if !((_2655 as uint32_t as int32_t) < _2657 as uint32_t as int32_t) {
                        current_block = 17982576473270685632;
                        break;
                    }
                    _2658 = _2601;
                    _2601 = llvm_add_u32(_2658, 1 as core::ffi::c_int as uint32_t);
                }
                match current_block {
                    15636031872310603809 => {}
                    _ => match current_block {
                        10557770493418402859 => {
                            __assert_fail(
                                    &_OC_str_OC_6 as *const l_array_45_uint8_t
                                        as *mut core::ffi::c_void,
                                    &_OC_str_OC_1 as *const l_array_61_uint8_t
                                        as *mut core::ffi::c_void,
                                    397 as core::ffi::c_int as uint32_t,
                                    &__PRETTY_FUNCTION___OC__ZL29count_partition_mismatch_bitsRK21block_size_descriptorjPKmPh
                                        as *const l_array_109_uint8_t as *mut core::ffi::c_void,
                                );
                        }
                        _ => {
                            __assert_fail(
                                    &_OC_str_OC_7 as *const l_array_37_uint8_t
                                        as *mut core::ffi::c_void,
                                    &_OC_str_OC_1 as *const l_array_61_uint8_t
                                        as *mut core::ffi::c_void,
                                    398 as core::ffi::c_int as uint32_t,
                                    &__PRETTY_FUNCTION___OC__ZL29count_partition_mismatch_bitsRK21block_size_descriptorjPKmPh
                                        as *const l_array_109_uint8_t as *mut core::ffi::c_void,
                                );
                        }
                    },
                }
            }
        }
        return;
    } else {
        __assert_fail(
            &_OC_str_OC_5 as *const l_array_17_uint8_t as *mut core::ffi::c_void,
            &_OC_str_OC_1 as *const l_array_61_uint8_t as *mut core::ffi::c_void,
            372 as core::ffi::c_int as uint32_t,
            &__PRETTY_FUNCTION___OC__ZL29count_partition_mismatch_bitsRK21block_size_descriptorjPKmPh
                as *const l_array_109_uint8_t as *mut core::ffi::c_void,
        );
    };
}
#[inline(never)]
unsafe extern "C" fn _ZL39get_partition_ordering_by_mismatch_bitsjjPKhPt(
    mut _2698: uint32_t,
    mut _2699: uint32_t,
    mut _2700: *mut core::ffi::c_void,
    mut _2701: *mut core::ffi::c_void,
) -> uint32_t {
    let mut _2702: uint32_t = 0;
    let mut _2703: uint32_t = 0;
    let mut _2704: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2705: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2706: l_array_64_uint16_t = l_array_64_uint16_t { array: [0; 64] };
    let mut _2707: uint32_t = 0;
    let mut _2708: uint16_t = 0;
    let mut _2709: uint32_t = 0;
    let mut _2710: uint16_t = 0;
    let mut _2711: uint32_t = 0;
    let mut _2712: uint32_t = 0;
    let mut _2713: uint32_t = 0;
    let mut _2714: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2715: uint32_t = 0;
    let mut _2716: uint32_t = 0;
    let mut _2717: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2718: uint32_t = 0;
    let mut _2719: uint8_t = 0;
    let mut _2720: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2721: uint16_t = 0;
    let mut _2722: uint32_t = 0;
    let mut _2723: uint32_t = 0;
    let mut _2724: uint32_t = 0;
    let mut _2725: uint32_t = 0;
    let mut _2726: uint16_t = 0;
    let mut _2727: uint16_t = 0;
    let mut _2728: uint32_t = 0;
    let mut _2729: uint16_t = 0;
    let mut _2730: uint16_t = 0;
    let mut _2731: uint32_t = 0;
    let mut _2732: uint32_t = 0;
    let mut _2733: uint32_t = 0;
    let mut _2734: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2735: uint32_t = 0;
    let mut _2736: uint8_t = 0;
    let mut _2737: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2738: uint16_t = 0;
    let mut _2739: uint32_t = 0;
    let mut _2740: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2741: uint32_t = 0;
    let mut _2742: uint32_t = 0;
    let mut _2743: uint32_t = 0;
    _2702 = _2698;
    _2703 = _2699;
    _2704 = _2700;
    _2705 = _2701;
    _2713 = _2703;
    if _2713 > 0 as core::ffi::c_uint {
        _2714 = memset(
            &mut _2706 as *mut l_array_64_uint16_t as *mut core::ffi::c_void,
            0 as core::ffi::c_int as uint32_t,
            128 as core::ffi::c_int as uint64_t,
        );
        _2707 = 0 as core::ffi::c_int as uint32_t;
        loop {
            _2715 = _2707;
            _2716 = _2703;
            if !(_2715 < _2716) {
                break;
            }
            _2717 = _2704;
            _2718 = _2707;
            _2719 = *(&mut *(_2717 as *mut uint8_t).offset(_2718 as uint64_t as int64_t as isize)
                as *mut uint8_t);
            _2720 = &mut *(_2706.array)
                .as_mut_ptr()
                .offset(_2719 as uint64_t as int64_t as isize) as *mut uint16_t
                as *mut core::ffi::c_void;
            _2721 = *(_2720 as *mut uint16_t);
            *(_2720 as *mut uint16_t) = llvm_add_u16(_2721, 1 as core::ffi::c_int as uint16_t);
            _2722 = _2707;
            _2707 = llvm_add_u32(_2722, 1 as core::ffi::c_int as uint32_t);
        }
        _2708 = 0 as core::ffi::c_int as uint16_t;
        _2709 = 0 as core::ffi::c_int as uint32_t;
        loop {
            _2723 = _2709;
            _2724 = _2702;
            if !(_2723 < _2724) {
                break;
            }
            _2725 = _2709;
            _2726 = *(&mut *(_2706.array)
                .as_mut_ptr()
                .offset(_2725 as uint64_t as int64_t as isize)
                as *mut uint16_t);
            _2710 = _2726;
            _2727 = _2708;
            _2728 = _2709;
            *(&mut *(_2706.array)
                .as_mut_ptr()
                .offset(_2728 as uint64_t as int64_t as isize) as *mut uint16_t) = _2727;
            _2729 = _2710;
            _2730 = _2708;
            _2708 = llvm_add_u32(_2730 as uint32_t, _2729 as uint32_t) as uint16_t;
            _2731 = _2709;
            _2709 = llvm_add_u32(_2731, 1 as core::ffi::c_int as uint32_t);
        }
        _2711 = 0 as core::ffi::c_int as uint32_t;
        loop {
            _2732 = _2711;
            _2733 = _2703;
            if !(_2732 < _2733) {
                break;
            }
            _2734 = _2704;
            _2735 = _2711;
            _2736 = *(&mut *(_2734 as *mut uint8_t).offset(_2735 as uint64_t as int64_t as isize)
                as *mut uint8_t);
            _2737 = &mut *(_2706.array)
                .as_mut_ptr()
                .offset(_2736 as uint64_t as int64_t as isize) as *mut uint16_t
                as *mut core::ffi::c_void;
            _2738 = *(_2737 as *mut uint16_t);
            *(_2737 as *mut uint16_t) = llvm_add_u16(_2738, 1 as core::ffi::c_int as uint16_t);
            _2712 = _2738 as uint32_t;
            _2739 = _2711;
            _2740 = _2705;
            _2741 = _2712;
            *(&mut *(_2740 as *mut uint16_t).offset(_2741 as uint64_t as int64_t as isize)
                as *mut uint16_t) = _2739 as uint16_t;
            _2742 = _2711;
            _2711 = llvm_add_u32(_2742, 1 as core::ffi::c_int as uint32_t);
        }
        _2743 = _2703;
        return _2743;
    } else {
        __assert_fail(
            &_OC_str_OC_8 as *const l_array_23_uint8_t as *mut core::ffi::c_void,
            &_OC_str_OC_1 as *const l_array_61_uint8_t as *mut core::ffi::c_void,
            418 as core::ffi::c_int as uint32_t,
            &__PRETTY_FUNCTION___OC__ZL39get_partition_ordering_by_mismatch_bitsjjPKhPt
                as *const l_array_110_uint8_t as *mut core::ffi::c_void,
        );
    };
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn _ZNK11image_block5texelEj(
    mut _2759: *mut core::ffi::c_void,
    mut _2760: uint32_t,
) -> l_struct_struct_OC_vfloat4 {
    let mut _2761: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2762: core::ffi::c_float = 0.;
    let mut _2763: core::ffi::c_float = 0.;
    let mut _2764: core::ffi::c_float = 0.;
    let mut _2765: core::ffi::c_float = 0.;
    let mut _2766: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _2767: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2768: uint32_t = 0;
    let mut _2769: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2770: uint32_t = 0;
    let mut _2771: core::ffi::c_float = 0.;
    let mut _2772: uint32_t = 0;
    let mut _2773: core::ffi::c_float = 0.;
    let mut _2774: uint32_t = 0;
    let mut _2775: core::ffi::c_float = 0.;
    let mut _2776: uint32_t = 0;
    let mut _2777: core::ffi::c_float = 0.;
    let mut _2778: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2779: core::ffi::c_float = 0.;
    let mut _2780: core::ffi::c_float = 0.;
    let mut _2781: core::ffi::c_float = 0.;
    let mut _2782: core::ffi::c_float = 0.;
    let mut _2783: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    _2767 = _2759;
    _2768 = _2760;
    _2769 = _2767;
    _2770 = _2768;
    _2771 = *(&mut *((*(&mut (*(_2769 as *mut l_struct_struct_OC_image_block)).field0
        as *mut l_array_219_float))
        .array)
        .as_mut_ptr()
        .offset(_2770 as uint64_t as int64_t as isize) as *mut core::ffi::c_float);
    _2772 = _2768;
    _2773 = *(&mut *((*(&mut (*(_2769 as *mut l_struct_struct_OC_image_block)).field1
        as *mut l_array_219_float))
        .array)
        .as_mut_ptr()
        .offset(_2772 as uint64_t as int64_t as isize) as *mut core::ffi::c_float);
    _2774 = _2768;
    _2775 = *(&mut *((*(&mut (*(_2769 as *mut l_struct_struct_OC_image_block)).field2
        as *mut l_array_219_float))
        .array)
        .as_mut_ptr()
        .offset(_2774 as uint64_t as int64_t as isize) as *mut core::ffi::c_float);
    _2776 = _2768;
    _2777 = *(&mut *((*(&mut (*(_2769 as *mut l_struct_struct_OC_image_block)).field3
        as *mut l_array_219_float))
        .array)
        .as_mut_ptr()
        .offset(_2776 as uint64_t as int64_t as isize) as *mut core::ffi::c_float);
    _2761 = &mut _2766 as *mut l_struct_struct_OC_vfloat4 as *mut core::ffi::c_void;
    _2762 = _2771;
    _2763 = _2773;
    _2764 = _2775;
    _2765 = _2777;
    _2778 = _2761;
    _2779 = _2762;
    *(_2778 as *mut core::ffi::c_float) = _2779;
    _2780 = _2763;
    *(&mut *((*(_2778 as *mut l_array_4_float)).array)
        .as_mut_ptr()
        .offset(1 as core::ffi::c_int as int64_t as isize) as *mut libc::c_float) = _2780;
    _2781 = _2764;
    *(&mut *((*(_2778 as *mut l_array_4_float)).array)
        .as_mut_ptr()
        .offset(2 as core::ffi::c_int as int64_t as isize) as *mut libc::c_float) = _2781;
    _2782 = _2765;
    *(&mut *((*(_2778 as *mut l_array_4_float)).array)
        .as_mut_ptr()
        .offset(3 as core::ffi::c_int as int64_t as isize) as *mut libc::c_float) = _2782;
    _2783 = _2766;
    return _2783;
}
#[inline(never)]
unsafe extern "C" fn _ZN4astcL3minIfEET_S1_S1_(
    mut _2784: core::ffi::c_float,
    mut _2785: core::ffi::c_float,
) -> core::ffi::c_float {
    let mut _2786: core::ffi::c_float = 0.;
    let mut _2787: core::ffi::c_float = 0.;
    let mut _2788: core::ffi::c_float = 0.;
    let mut _2789: core::ffi::c_float = 0.;
    let mut _2790: core::ffi::c_float = 0.;
    let mut _2791: core::ffi::c_float = 0.;
    let mut _2792: core::ffi::c_float = 0.;
    let mut _2792__PHI_TEMPORARY: core::ffi::c_float = 0.;
    _2786 = _2784;
    _2787 = _2785;
    _2788 = _2786;
    _2789 = _2787;
    if llvm_fcmp_olt(_2788 as core::ffi::c_double, _2789 as libc::c_double) != 0 {
        _2790 = _2786;
        _2792__PHI_TEMPORARY = _2790;
    } else {
        _2791 = _2787;
        _2792__PHI_TEMPORARY = _2791;
    }
    _2792 = _2792__PHI_TEMPORARY;
    return _2792;
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn _ZNSt14numeric_limitsIfE3maxEv() -> core::ffi::c_float {
    return 3.40282347E+38f64 as core::ffi::c_float;
}
#[inline(never)]
unsafe extern "C" fn _ZL19partition_mismatch2PKmS0_(
    mut _2796: *mut core::ffi::c_void,
    mut _2797: *mut core::ffi::c_void,
) -> uint8_t {
    let mut _2798: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2799: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2800: uint32_t = 0;
    let mut _2801: uint32_t = 0;
    let mut _2802: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2803: uint64_t = 0;
    let mut _2804: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2805: uint64_t = 0;
    let mut _2806: uint32_t = 0;
    let mut _2807: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2808: uint64_t = 0;
    let mut _2809: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2810: uint64_t = 0;
    let mut _2811: uint32_t = 0;
    let mut _2812: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2813: uint64_t = 0;
    let mut _2814: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2815: uint64_t = 0;
    let mut _2816: uint32_t = 0;
    let mut _2817: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2818: uint64_t = 0;
    let mut _2819: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2820: uint64_t = 0;
    let mut _2821: uint32_t = 0;
    let mut _2822: uint32_t = 0;
    let mut _2823: uint32_t = 0;
    let mut _2824: uint32_t = 0;
    _2798 = _2796;
    _2799 = _2797;
    _2802 = _2798;
    _2803 = *(_2802 as *mut uint64_t);
    _2804 = _2799;
    _2805 = *(_2804 as *mut uint64_t);
    _2806 = _ZL8popcountm(_2803 ^ _2805);
    _2807 = _2798;
    _2808 = *(&mut *(_2807 as *mut uint64_t).offset(1 as core::ffi::c_int as int64_t as isize)
        as *mut uint64_t);
    _2809 = _2799;
    _2810 = *(&mut *(_2809 as *mut uint64_t).offset(1 as core::ffi::c_int as int64_t as isize)
        as *mut uint64_t);
    _2811 = _ZL8popcountm(_2808 ^ _2810);
    _2800 = llvm_add_u32(_2806, _2811);
    _2812 = _2798;
    _2813 = *(_2812 as *mut uint64_t);
    _2814 = _2799;
    _2815 = *(&mut *(_2814 as *mut uint64_t).offset(1 as core::ffi::c_int as int64_t as isize)
        as *mut uint64_t);
    _2816 = _ZL8popcountm(_2813 ^ _2815);
    _2817 = _2798;
    _2818 = *(&mut *(_2817 as *mut uint64_t).offset(1 as core::ffi::c_int as int64_t as isize)
        as *mut uint64_t);
    _2819 = _2799;
    _2820 = *(_2819 as *mut uint64_t);
    _2821 = _ZL8popcountm(_2818 ^ _2820);
    _2801 = llvm_add_u32(_2816, _2821);
    _2822 = _2800;
    _2823 = _2801;
    _2824 = _ZN4astcL3minIiEET_S1_S1_(_2822, _2823);
    return llvm_sdiv_u32(_2824 as int32_t, 2 as core::ffi::c_int) as uint8_t;
}
#[inline(never)]
unsafe extern "C" fn _ZL19partition_mismatch3PKmS0_(
    mut _2825: *mut core::ffi::c_void,
    mut _2826: *mut core::ffi::c_void,
) -> uint8_t {
    let mut _2827: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2828: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2829: uint32_t = 0;
    let mut _2830: uint32_t = 0;
    let mut _2831: uint32_t = 0;
    let mut _2832: uint32_t = 0;
    let mut _2833: uint32_t = 0;
    let mut _2834: uint32_t = 0;
    let mut _2835: uint32_t = 0;
    let mut _2836: uint32_t = 0;
    let mut _2837: uint32_t = 0;
    let mut _2838: uint32_t = 0;
    let mut _2839: uint32_t = 0;
    let mut _2840: uint32_t = 0;
    let mut _2841: uint32_t = 0;
    let mut _2842: uint32_t = 0;
    let mut _2843: uint32_t = 0;
    let mut _2844: uint32_t = 0;
    let mut _2845: uint32_t = 0;
    let mut _2846: uint32_t = 0;
    let mut _2847: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2848: uint64_t = 0;
    let mut _2849: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2850: uint64_t = 0;
    let mut _2851: uint32_t = 0;
    let mut _2852: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2853: uint64_t = 0;
    let mut _2854: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2855: uint64_t = 0;
    let mut _2856: uint32_t = 0;
    let mut _2857: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2858: uint64_t = 0;
    let mut _2859: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2860: uint64_t = 0;
    let mut _2861: uint32_t = 0;
    let mut _2862: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2863: uint64_t = 0;
    let mut _2864: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2865: uint64_t = 0;
    let mut _2866: uint32_t = 0;
    let mut _2867: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2868: uint64_t = 0;
    let mut _2869: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2870: uint64_t = 0;
    let mut _2871: uint32_t = 0;
    let mut _2872: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2873: uint64_t = 0;
    let mut _2874: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2875: uint64_t = 0;
    let mut _2876: uint32_t = 0;
    let mut _2877: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2878: uint64_t = 0;
    let mut _2879: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2880: uint64_t = 0;
    let mut _2881: uint32_t = 0;
    let mut _2882: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2883: uint64_t = 0;
    let mut _2884: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2885: uint64_t = 0;
    let mut _2886: uint32_t = 0;
    let mut _2887: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2888: uint64_t = 0;
    let mut _2889: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2890: uint64_t = 0;
    let mut _2891: uint32_t = 0;
    let mut _2892: uint32_t = 0;
    let mut _2893: uint32_t = 0;
    let mut _2894: uint32_t = 0;
    let mut _2895: uint32_t = 0;
    let mut _2896: uint32_t = 0;
    let mut _2897: uint32_t = 0;
    let mut _2898: uint32_t = 0;
    let mut _2899: uint32_t = 0;
    let mut _2900: uint32_t = 0;
    let mut _2901: uint32_t = 0;
    let mut _2902: uint32_t = 0;
    let mut _2903: uint32_t = 0;
    let mut _2904: uint32_t = 0;
    let mut _2905: uint32_t = 0;
    let mut _2906: uint32_t = 0;
    let mut _2907: uint32_t = 0;
    let mut _2908: uint32_t = 0;
    let mut _2909: uint32_t = 0;
    let mut _2910: uint32_t = 0;
    let mut _2911: uint32_t = 0;
    let mut _2912: uint32_t = 0;
    let mut _2913: uint32_t = 0;
    let mut _2914: uint32_t = 0;
    let mut _2915: uint32_t = 0;
    let mut _2916: uint32_t = 0;
    let mut _2917: uint32_t = 0;
    let mut _2918: uint32_t = 0;
    let mut _2919: uint32_t = 0;
    _2827 = _2825;
    _2828 = _2826;
    _2847 = _2827;
    _2848 = *(_2847 as *mut uint64_t);
    _2849 = _2828;
    _2850 = *(_2849 as *mut uint64_t);
    _2851 = _ZL8popcountm(_2848 ^ _2850);
    _2829 = _2851;
    _2852 = _2827;
    _2853 = *(_2852 as *mut uint64_t);
    _2854 = _2828;
    _2855 = *(&mut *(_2854 as *mut uint64_t).offset(1 as core::ffi::c_int as int64_t as isize)
        as *mut uint64_t);
    _2856 = _ZL8popcountm(_2853 ^ _2855);
    _2830 = _2856;
    _2857 = _2827;
    _2858 = *(_2857 as *mut uint64_t);
    _2859 = _2828;
    _2860 = *(&mut *(_2859 as *mut uint64_t).offset(2 as core::ffi::c_int as int64_t as isize)
        as *mut uint64_t);
    _2861 = _ZL8popcountm(_2858 ^ _2860);
    _2831 = _2861;
    _2862 = _2827;
    _2863 = *(&mut *(_2862 as *mut uint64_t).offset(1 as core::ffi::c_int as int64_t as isize)
        as *mut uint64_t);
    _2864 = _2828;
    _2865 = *(_2864 as *mut uint64_t);
    _2866 = _ZL8popcountm(_2863 ^ _2865);
    _2832 = _2866;
    _2867 = _2827;
    _2868 = *(&mut *(_2867 as *mut uint64_t).offset(1 as core::ffi::c_int as int64_t as isize)
        as *mut uint64_t);
    _2869 = _2828;
    _2870 = *(&mut *(_2869 as *mut uint64_t).offset(1 as core::ffi::c_int as int64_t as isize)
        as *mut uint64_t);
    _2871 = _ZL8popcountm(_2868 ^ _2870);
    _2833 = _2871;
    _2872 = _2827;
    _2873 = *(&mut *(_2872 as *mut uint64_t).offset(1 as core::ffi::c_int as int64_t as isize)
        as *mut uint64_t);
    _2874 = _2828;
    _2875 = *(&mut *(_2874 as *mut uint64_t).offset(2 as core::ffi::c_int as int64_t as isize)
        as *mut uint64_t);
    _2876 = _ZL8popcountm(_2873 ^ _2875);
    _2834 = _2876;
    _2877 = _2827;
    _2878 = *(&mut *(_2877 as *mut uint64_t).offset(2 as core::ffi::c_int as int64_t as isize)
        as *mut uint64_t);
    _2879 = _2828;
    _2880 = *(_2879 as *mut uint64_t);
    _2881 = _ZL8popcountm(_2878 ^ _2880);
    _2835 = _2881;
    _2882 = _2827;
    _2883 = *(&mut *(_2882 as *mut uint64_t).offset(2 as core::ffi::c_int as int64_t as isize)
        as *mut uint64_t);
    _2884 = _2828;
    _2885 = *(&mut *(_2884 as *mut uint64_t).offset(1 as core::ffi::c_int as int64_t as isize)
        as *mut uint64_t);
    _2886 = _ZL8popcountm(_2883 ^ _2885);
    _2836 = _2886;
    _2887 = _2827;
    _2888 = *(&mut *(_2887 as *mut uint64_t).offset(2 as core::ffi::c_int as int64_t as isize)
        as *mut uint64_t);
    _2889 = _2828;
    _2890 = *(&mut *(_2889 as *mut uint64_t).offset(2 as core::ffi::c_int as int64_t as isize)
        as *mut uint64_t);
    _2891 = _ZL8popcountm(_2888 ^ _2890);
    _2837 = _2891;
    _2892 = _2833;
    _2893 = _2837;
    _2838 = llvm_add_u32(_2892, _2893);
    _2894 = _2834;
    _2895 = _2836;
    _2839 = llvm_add_u32(_2894, _2895);
    _2896 = _2838;
    _2897 = _2839;
    _2898 = _ZN4astcL3minIiEET_S1_S1_(_2896, _2897);
    _2899 = _2829;
    _2840 = llvm_add_u32(_2898, _2899);
    _2900 = _2832;
    _2901 = _2837;
    _2841 = llvm_add_u32(_2900, _2901);
    _2902 = _2834;
    _2903 = _2835;
    _2842 = llvm_add_u32(_2902, _2903);
    _2904 = _2841;
    _2905 = _2842;
    _2906 = _ZN4astcL3minIiEET_S1_S1_(_2904, _2905);
    _2907 = _2830;
    _2843 = llvm_add_u32(_2906, _2907);
    _2908 = _2832;
    _2909 = _2836;
    _2844 = llvm_add_u32(_2908, _2909);
    _2910 = _2833;
    _2911 = _2835;
    _2845 = llvm_add_u32(_2910, _2911);
    _2912 = _2844;
    _2913 = _2845;
    _2914 = _ZN4astcL3minIiEET_S1_S1_(_2912, _2913);
    _2915 = _2831;
    _2846 = llvm_add_u32(_2914, _2915);
    _2916 = _2840;
    _2917 = _2843;
    _2918 = _2846;
    _2919 = _ZN4astcL3minIiEET_S1_S1_S1_(_2916, _2917, _2918);
    return llvm_sdiv_u32(_2919 as int32_t, 2 as core::ffi::c_int) as uint8_t;
}
#[inline(never)]
unsafe extern "C" fn _ZL19partition_mismatch4PKmS0_(
    mut _2920: *mut core::ffi::c_void,
    mut _2921: *mut core::ffi::c_void,
) -> uint8_t {
    let mut _2922: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2923: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2924: uint32_t = 0;
    let mut _2925: uint32_t = 0;
    let mut _2926: uint32_t = 0;
    let mut _2927: uint32_t = 0;
    let mut _2928: uint32_t = 0;
    let mut _2929: uint32_t = 0;
    let mut _2930: uint32_t = 0;
    let mut _2931: uint32_t = 0;
    let mut _2932: uint32_t = 0;
    let mut _2933: uint32_t = 0;
    let mut _2934: uint32_t = 0;
    let mut _2935: uint32_t = 0;
    let mut _2936: uint32_t = 0;
    let mut _2937: uint32_t = 0;
    let mut _2938: uint32_t = 0;
    let mut _2939: uint32_t = 0;
    let mut _2940: uint32_t = 0;
    let mut _2941: uint32_t = 0;
    let mut _2942: uint32_t = 0;
    let mut _2943: uint32_t = 0;
    let mut _2944: uint32_t = 0;
    let mut _2945: uint32_t = 0;
    let mut _2946: uint32_t = 0;
    let mut _2947: uint32_t = 0;
    let mut _2948: uint32_t = 0;
    let mut _2949: uint32_t = 0;
    let mut _2950: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2951: uint64_t = 0;
    let mut _2952: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2953: uint64_t = 0;
    let mut _2954: uint32_t = 0;
    let mut _2955: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2956: uint64_t = 0;
    let mut _2957: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2958: uint64_t = 0;
    let mut _2959: uint32_t = 0;
    let mut _2960: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2961: uint64_t = 0;
    let mut _2962: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2963: uint64_t = 0;
    let mut _2964: uint32_t = 0;
    let mut _2965: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2966: uint64_t = 0;
    let mut _2967: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2968: uint64_t = 0;
    let mut _2969: uint32_t = 0;
    let mut _2970: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2971: uint64_t = 0;
    let mut _2972: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2973: uint64_t = 0;
    let mut _2974: uint32_t = 0;
    let mut _2975: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2976: uint64_t = 0;
    let mut _2977: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2978: uint64_t = 0;
    let mut _2979: uint32_t = 0;
    let mut _2980: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2981: uint64_t = 0;
    let mut _2982: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2983: uint64_t = 0;
    let mut _2984: uint32_t = 0;
    let mut _2985: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2986: uint64_t = 0;
    let mut _2987: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2988: uint64_t = 0;
    let mut _2989: uint32_t = 0;
    let mut _2990: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2991: uint64_t = 0;
    let mut _2992: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2993: uint64_t = 0;
    let mut _2994: uint32_t = 0;
    let mut _2995: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2996: uint64_t = 0;
    let mut _2997: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _2998: uint64_t = 0;
    let mut _2999: uint32_t = 0;
    let mut _3000: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _3001: uint64_t = 0;
    let mut _3002: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _3003: uint64_t = 0;
    let mut _3004: uint32_t = 0;
    let mut _3005: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _3006: uint64_t = 0;
    let mut _3007: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _3008: uint64_t = 0;
    let mut _3009: uint32_t = 0;
    let mut _3010: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _3011: uint64_t = 0;
    let mut _3012: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _3013: uint64_t = 0;
    let mut _3014: uint32_t = 0;
    let mut _3015: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _3016: uint64_t = 0;
    let mut _3017: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _3018: uint64_t = 0;
    let mut _3019: uint32_t = 0;
    let mut _3020: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _3021: uint64_t = 0;
    let mut _3022: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _3023: uint64_t = 0;
    let mut _3024: uint32_t = 0;
    let mut _3025: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _3026: uint64_t = 0;
    let mut _3027: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _3028: uint64_t = 0;
    let mut _3029: uint32_t = 0;
    let mut _3030: uint32_t = 0;
    let mut _3031: uint32_t = 0;
    let mut _3032: uint32_t = 0;
    let mut _3033: uint32_t = 0;
    let mut _3034: uint32_t = 0;
    let mut _3035: uint32_t = 0;
    let mut _3036: uint32_t = 0;
    let mut _3037: uint32_t = 0;
    let mut _3038: uint32_t = 0;
    let mut _3039: uint32_t = 0;
    let mut _3040: uint32_t = 0;
    let mut _3041: uint32_t = 0;
    let mut _3042: uint32_t = 0;
    let mut _3043: uint32_t = 0;
    let mut _3044: uint32_t = 0;
    let mut _3045: uint32_t = 0;
    let mut _3046: uint32_t = 0;
    let mut _3047: uint32_t = 0;
    let mut _3048: uint32_t = 0;
    let mut _3049: uint32_t = 0;
    let mut _3050: uint32_t = 0;
    let mut _3051: uint32_t = 0;
    let mut _3052: uint32_t = 0;
    let mut _3053: uint32_t = 0;
    let mut _3054: uint32_t = 0;
    let mut _3055: uint32_t = 0;
    let mut _3056: uint32_t = 0;
    let mut _3057: uint32_t = 0;
    let mut _3058: uint32_t = 0;
    let mut _3059: uint32_t = 0;
    let mut _3060: uint32_t = 0;
    let mut _3061: uint32_t = 0;
    let mut _3062: uint32_t = 0;
    let mut _3063: uint32_t = 0;
    let mut _3064: uint32_t = 0;
    let mut _3065: uint32_t = 0;
    let mut _3066: uint32_t = 0;
    let mut _3067: uint32_t = 0;
    let mut _3068: uint32_t = 0;
    let mut _3069: uint32_t = 0;
    let mut _3070: uint32_t = 0;
    let mut _3071: uint32_t = 0;
    let mut _3072: uint32_t = 0;
    let mut _3073: uint32_t = 0;
    let mut _3074: uint32_t = 0;
    let mut _3075: uint32_t = 0;
    let mut _3076: uint32_t = 0;
    let mut _3077: uint32_t = 0;
    let mut _3078: uint32_t = 0;
    let mut _3079: uint32_t = 0;
    let mut _3080: uint32_t = 0;
    let mut _3081: uint32_t = 0;
    let mut _3082: uint32_t = 0;
    let mut _3083: uint32_t = 0;
    let mut _3084: uint32_t = 0;
    let mut _3085: uint32_t = 0;
    let mut _3086: uint32_t = 0;
    let mut _3087: uint32_t = 0;
    let mut _3088: uint32_t = 0;
    let mut _3089: uint32_t = 0;
    let mut _3090: uint32_t = 0;
    let mut _3091: uint32_t = 0;
    let mut _3092: uint32_t = 0;
    let mut _3093: uint32_t = 0;
    let mut _3094: uint32_t = 0;
    let mut _3095: uint32_t = 0;
    let mut _3096: uint32_t = 0;
    _2922 = _2920;
    _2923 = _2921;
    _2950 = _2922;
    _2951 = *(_2950 as *mut uint64_t);
    _2952 = _2923;
    _2953 = *(_2952 as *mut uint64_t);
    _2954 = _ZL8popcountm(_2951 ^ _2953);
    _2924 = _2954;
    _2955 = _2922;
    _2956 = *(_2955 as *mut uint64_t);
    _2957 = _2923;
    _2958 = *(&mut *(_2957 as *mut uint64_t).offset(1 as core::ffi::c_int as int64_t as isize)
        as *mut uint64_t);
    _2959 = _ZL8popcountm(_2956 ^ _2958);
    _2925 = _2959;
    _2960 = _2922;
    _2961 = *(_2960 as *mut uint64_t);
    _2962 = _2923;
    _2963 = *(&mut *(_2962 as *mut uint64_t).offset(2 as core::ffi::c_int as int64_t as isize)
        as *mut uint64_t);
    _2964 = _ZL8popcountm(_2961 ^ _2963);
    _2926 = _2964;
    _2965 = _2922;
    _2966 = *(_2965 as *mut uint64_t);
    _2967 = _2923;
    _2968 = *(&mut *(_2967 as *mut uint64_t).offset(3 as core::ffi::c_int as int64_t as isize)
        as *mut uint64_t);
    _2969 = _ZL8popcountm(_2966 ^ _2968);
    _2927 = _2969;
    _2970 = _2922;
    _2971 = *(&mut *(_2970 as *mut uint64_t).offset(1 as core::ffi::c_int as int64_t as isize)
        as *mut uint64_t);
    _2972 = _2923;
    _2973 = *(_2972 as *mut uint64_t);
    _2974 = _ZL8popcountm(_2971 ^ _2973);
    _2928 = _2974;
    _2975 = _2922;
    _2976 = *(&mut *(_2975 as *mut uint64_t).offset(1 as core::ffi::c_int as int64_t as isize)
        as *mut uint64_t);
    _2977 = _2923;
    _2978 = *(&mut *(_2977 as *mut uint64_t).offset(1 as core::ffi::c_int as int64_t as isize)
        as *mut uint64_t);
    _2979 = _ZL8popcountm(_2976 ^ _2978);
    _2929 = _2979;
    _2980 = _2922;
    _2981 = *(&mut *(_2980 as *mut uint64_t).offset(1 as core::ffi::c_int as int64_t as isize)
        as *mut uint64_t);
    _2982 = _2923;
    _2983 = *(&mut *(_2982 as *mut uint64_t).offset(2 as core::ffi::c_int as int64_t as isize)
        as *mut uint64_t);
    _2984 = _ZL8popcountm(_2981 ^ _2983);
    _2930 = _2984;
    _2985 = _2922;
    _2986 = *(&mut *(_2985 as *mut uint64_t).offset(1 as core::ffi::c_int as int64_t as isize)
        as *mut uint64_t);
    _2987 = _2923;
    _2988 = *(&mut *(_2987 as *mut uint64_t).offset(3 as core::ffi::c_int as int64_t as isize)
        as *mut uint64_t);
    _2989 = _ZL8popcountm(_2986 ^ _2988);
    _2931 = _2989;
    _2990 = _2922;
    _2991 = *(&mut *(_2990 as *mut uint64_t).offset(2 as core::ffi::c_int as int64_t as isize)
        as *mut uint64_t);
    _2992 = _2923;
    _2993 = *(_2992 as *mut uint64_t);
    _2994 = _ZL8popcountm(_2991 ^ _2993);
    _2932 = _2994;
    _2995 = _2922;
    _2996 = *(&mut *(_2995 as *mut uint64_t).offset(2 as core::ffi::c_int as int64_t as isize)
        as *mut uint64_t);
    _2997 = _2923;
    _2998 = *(&mut *(_2997 as *mut uint64_t).offset(1 as core::ffi::c_int as int64_t as isize)
        as *mut uint64_t);
    _2999 = _ZL8popcountm(_2996 ^ _2998);
    _2933 = _2999;
    _3000 = _2922;
    _3001 = *(&mut *(_3000 as *mut uint64_t).offset(2 as core::ffi::c_int as int64_t as isize)
        as *mut uint64_t);
    _3002 = _2923;
    _3003 = *(&mut *(_3002 as *mut uint64_t).offset(2 as core::ffi::c_int as int64_t as isize)
        as *mut uint64_t);
    _3004 = _ZL8popcountm(_3001 ^ _3003);
    _2934 = _3004;
    _3005 = _2922;
    _3006 = *(&mut *(_3005 as *mut uint64_t).offset(2 as core::ffi::c_int as int64_t as isize)
        as *mut uint64_t);
    _3007 = _2923;
    _3008 = *(&mut *(_3007 as *mut uint64_t).offset(3 as core::ffi::c_int as int64_t as isize)
        as *mut uint64_t);
    _3009 = _ZL8popcountm(_3006 ^ _3008);
    _2935 = _3009;
    _3010 = _2922;
    _3011 = *(&mut *(_3010 as *mut uint64_t).offset(3 as core::ffi::c_int as int64_t as isize)
        as *mut uint64_t);
    _3012 = _2923;
    _3013 = *(_3012 as *mut uint64_t);
    _3014 = _ZL8popcountm(_3011 ^ _3013);
    _2936 = _3014;
    _3015 = _2922;
    _3016 = *(&mut *(_3015 as *mut uint64_t).offset(3 as core::ffi::c_int as int64_t as isize)
        as *mut uint64_t);
    _3017 = _2923;
    _3018 = *(&mut *(_3017 as *mut uint64_t).offset(1 as core::ffi::c_int as int64_t as isize)
        as *mut uint64_t);
    _3019 = _ZL8popcountm(_3016 ^ _3018);
    _2937 = _3019;
    _3020 = _2922;
    _3021 = *(&mut *(_3020 as *mut uint64_t).offset(3 as core::ffi::c_int as int64_t as isize)
        as *mut uint64_t);
    _3022 = _2923;
    _3023 = *(&mut *(_3022 as *mut uint64_t).offset(2 as core::ffi::c_int as int64_t as isize)
        as *mut uint64_t);
    _3024 = _ZL8popcountm(_3021 ^ _3023);
    _2938 = _3024;
    _3025 = _2922;
    _3026 = *(&mut *(_3025 as *mut uint64_t).offset(3 as core::ffi::c_int as int64_t as isize)
        as *mut uint64_t);
    _3027 = _2923;
    _3028 = *(&mut *(_3027 as *mut uint64_t).offset(3 as core::ffi::c_int as int64_t as isize)
        as *mut uint64_t);
    _3029 = _ZL8popcountm(_3026 ^ _3028);
    _2939 = _3029;
    _3030 = _2934;
    _3031 = _2939;
    _3032 = _2935;
    _3033 = _2938;
    _3034 = _ZN4astcL3minIiEET_S1_S1_(llvm_add_u32(_3030, _3031), llvm_add_u32(_3032, _3033));
    _2940 = _3034;
    _3035 = _2933;
    _3036 = _2939;
    _3037 = _2935;
    _3038 = _2937;
    _3039 = _ZN4astcL3minIiEET_S1_S1_(llvm_add_u32(_3035, _3036), llvm_add_u32(_3037, _3038));
    _2941 = _3039;
    _3040 = _2933;
    _3041 = _2938;
    _3042 = _2934;
    _3043 = _2937;
    _3044 = _ZN4astcL3minIiEET_S1_S1_(llvm_add_u32(_3040, _3041), llvm_add_u32(_3042, _3043));
    _2942 = _3044;
    _3045 = _2932;
    _3046 = _2939;
    _3047 = _2935;
    _3048 = _2936;
    _3049 = _ZN4astcL3minIiEET_S1_S1_(llvm_add_u32(_3045, _3046), llvm_add_u32(_3047, _3048));
    _2943 = _3049;
    _3050 = _2932;
    _3051 = _2938;
    _3052 = _2934;
    _3053 = _2936;
    _3054 = _ZN4astcL3minIiEET_S1_S1_(llvm_add_u32(_3050, _3051), llvm_add_u32(_3052, _3053));
    _2944 = _3054;
    _3055 = _2933;
    _3056 = _2936;
    _3057 = _2932;
    _3058 = _2937;
    _3059 = _ZN4astcL3minIiEET_S1_S1_(llvm_add_u32(_3055, _3056), llvm_add_u32(_3057, _3058));
    _2945 = _3059;
    _3060 = _2924;
    _3061 = _2929;
    _3062 = _2940;
    _3063 = _2930;
    _3064 = _2941;
    _3065 = _2931;
    _3066 = _2942;
    _3067 = _ZN4astcL3minIiEET_S1_S1_S1_(
        llvm_add_u32(_3061, _3062),
        llvm_add_u32(_3063, _3064),
        llvm_add_u32(_3065, _3066),
    );
    _2946 = llvm_add_u32(_3060, _3067);
    _3068 = _2925;
    _3069 = _2928;
    _3070 = _2940;
    _3071 = _2930;
    _3072 = _2943;
    _3073 = _2931;
    _3074 = _2944;
    _3075 = _ZN4astcL3minIiEET_S1_S1_S1_(
        llvm_add_u32(_3069, _3070),
        llvm_add_u32(_3071, _3072),
        llvm_add_u32(_3073, _3074),
    );
    _2947 = llvm_add_u32(_3068, _3075);
    _3076 = _2926;
    _3077 = _2929;
    _3078 = _2943;
    _3079 = _2928;
    _3080 = _2941;
    _3081 = _2931;
    _3082 = _2945;
    _3083 = _ZN4astcL3minIiEET_S1_S1_S1_(
        llvm_add_u32(_3077, _3078),
        llvm_add_u32(_3079, _3080),
        llvm_add_u32(_3081, _3082),
    );
    _2948 = llvm_add_u32(_3076, _3083);
    _3084 = _2927;
    _3085 = _2929;
    _3086 = _2944;
    _3087 = _2930;
    _3088 = _2945;
    _3089 = _2928;
    _3090 = _2942;
    _3091 = _ZN4astcL3minIiEET_S1_S1_S1_(
        llvm_add_u32(_3085, _3086),
        llvm_add_u32(_3087, _3088),
        llvm_add_u32(_3089, _3090),
    );
    _2949 = llvm_add_u32(_3084, _3091);
    _3092 = _2946;
    _3093 = _2947;
    _3094 = _2948;
    _3095 = _2949;
    _3096 = _ZN4astcL3minIiEET_S1_S1_S1_S1_(_3092, _3093, _3094, _3095);
    return llvm_sdiv_u32(_3096 as int32_t, 2 as core::ffi::c_int) as uint8_t;
}
#[inline(never)]
unsafe extern "C" fn _ZL8popcountm(mut _3097: uint64_t) -> uint32_t {
    let mut _3098: uint64_t = 0;
    let mut _3099: uint64_t = 0;
    let mut _3100: uint64_t = 0;
    let mut _3101: uint64_t = 0;
    let mut _3102: uint64_t = 0;
    let mut _3103: uint64_t = 0;
    let mut _3104: uint64_t = 0;
    let mut _3105: uint64_t = 0;
    let mut _3106: uint64_t = 0;
    let mut _3107: uint64_t = 0;
    let mut _3108: uint64_t = 0;
    let mut _3109: uint64_t = 0;
    let mut _3110: uint64_t = 0;
    let mut _3111: uint64_t = 0;
    let mut _3112: uint64_t = 0;
    let mut _3113: uint64_t = 0;
    let mut _3114: uint64_t = 0;
    let mut _3115: uint64_t = 0;
    _3098 = _3097;
    _3099 = 6148914691236517205 as core::ffi::c_ulong;
    _3100 = 3689348814741910323 as core::ffi::c_long as uint64_t;
    _3101 = 1085102592571150095 as core::ffi::c_long as uint64_t;
    _3102 = _3098;
    _3103 = _3099;
    _3104 = _3098;
    _3098 = llvm_sub_u64(
        _3104,
        llvm_lshr_u64(_3102, 1 as core::ffi::c_int as uint64_t) & _3103,
    );
    _3105 = _3098;
    _3106 = _3100;
    _3107 = _3098;
    _3108 = _3100;
    _3098 = llvm_add_u64(
        _3105 & _3106,
        llvm_lshr_u64(_3107, 2 as core::ffi::c_int as uint64_t) & _3108,
    );
    _3109 = _3098;
    _3110 = _3098;
    _3098 = llvm_add_u64(
        _3110,
        llvm_lshr_u64(_3109, 4 as core::ffi::c_int as uint64_t),
    );
    _3111 = _3101;
    _3112 = _3098;
    _3098 = _3112 & _3111;
    _3113 = _3098;
    _3098 = llvm_mul_u64(_3113, 72340172838076673 as core::ffi::c_long as uint64_t);
    _3114 = _3098;
    _3098 = llvm_lshr_u64(_3114, 56 as core::ffi::c_int as uint64_t);
    _3115 = _3098;
    return _3115 as uint32_t;
}
#[inline(never)]
unsafe extern "C" fn _ZN4astcL3minIiEET_S1_S1_(
    mut _3116: uint32_t,
    mut _3117: uint32_t,
) -> uint32_t {
    let mut _3118: uint32_t = 0;
    let mut _3119: uint32_t = 0;
    let mut _3120: uint32_t = 0;
    let mut _3121: uint32_t = 0;
    let mut _3122: uint32_t = 0;
    let mut _3123: uint32_t = 0;
    let mut _3124: uint32_t = 0;
    let mut _3124__PHI_TEMPORARY: uint32_t = 0;
    _3118 = _3116;
    _3119 = _3117;
    _3120 = _3118;
    _3121 = _3119;
    if (_3120 as int32_t) < _3121 as int32_t {
        _3122 = _3118;
        _3124__PHI_TEMPORARY = _3122;
    } else {
        _3123 = _3119;
        _3124__PHI_TEMPORARY = _3123;
    }
    _3124 = _3124__PHI_TEMPORARY;
    return _3124;
}
#[inline(never)]
unsafe extern "C" fn _ZN4astcL3minIiEET_S1_S1_S1_(
    mut _3128: uint32_t,
    mut _3129: uint32_t,
    mut _3130: uint32_t,
) -> uint32_t {
    let mut _3131: uint32_t = 0;
    let mut _3132: uint32_t = 0;
    let mut _3133: uint32_t = 0;
    let mut _3134: uint32_t = 0;
    let mut _3135: uint32_t = 0;
    let mut _3136: uint32_t = 0;
    let mut _3137: uint32_t = 0;
    let mut _3138: uint32_t = 0;
    _3131 = _3128;
    _3132 = _3129;
    _3133 = _3130;
    _3134 = _3131;
    _3135 = _3132;
    _3136 = _ZN4astcL3minIiEET_S1_S1_(_3134, _3135);
    _3137 = _3133;
    _3138 = _ZN4astcL3minIiEET_S1_S1_(_3136, _3137);
    return _3138;
}
#[inline(never)]
unsafe extern "C" fn _ZN4astcL3minIiEET_S1_S1_S1_S1_(
    mut _3139: uint32_t,
    mut _3140: uint32_t,
    mut _3141: uint32_t,
    mut _3142: uint32_t,
) -> uint32_t {
    let mut _3143: uint32_t = 0;
    let mut _3144: uint32_t = 0;
    let mut _3145: uint32_t = 0;
    let mut _3146: uint32_t = 0;
    let mut _3147: uint32_t = 0;
    let mut _3148: uint32_t = 0;
    let mut _3149: uint32_t = 0;
    let mut _3150: uint32_t = 0;
    let mut _3151: uint32_t = 0;
    let mut _3152: uint32_t = 0;
    let mut _3153: uint32_t = 0;
    _3143 = _3139;
    _3144 = _3140;
    _3145 = _3141;
    _3146 = _3142;
    _3147 = _3143;
    _3148 = _3144;
    _3149 = _ZN4astcL3minIiEET_S1_S1_(_3147, _3148);
    _3150 = _3145;
    _3151 = _3146;
    _3152 = _ZN4astcL3minIiEET_S1_S1_(_3150, _3151);
    _3153 = _ZN4astcL3minIiEET_S1_S1_(_3149, _3152);
    return _3153;
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn _ZNK21block_size_descriptor19get_partition_tableEj(
    mut _3154: *mut core::ffi::c_void,
    mut _3155: uint32_t,
) -> *mut core::ffi::c_void {
    let mut _3156: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _3157: uint32_t = 0;
    let mut _3158: uint32_t = 0;
    let mut _3159: *mut core::ffi::c_void = 0 as *mut libc::c_void;
    let mut _3160: uint32_t = 0;
    let mut _3161: uint32_t = 0;
    let mut _3162: uint32_t = 0;
    _3156 = _3154;
    _3157 = _3155;
    _3159 = _3156;
    _3160 = _3157;
    if _3160 == 1 as core::ffi::c_uint {
        _3157 = 5 as core::ffi::c_int as uint32_t;
    }
    _3161 = _3157;
    _3158 = llvm_mul_u32(
        llvm_sub_u32(_3161, 2 as core::ffi::c_int as uint32_t),
        1024 as core::ffi::c_int as uint32_t,
    );
    _3162 = _3158;
    return &mut *(&mut *((*(&mut (*(_3159
        as *mut l_struct_struct_OC_block_size_descriptor))
        .field17 as *mut l_array_3073_struct_AC_l_struct_struct_OC_partition_info))
        .array)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as int64_t as isize)
        as *mut l_struct_struct_OC_partition_info)
        .offset(_3162 as uint64_t as int64_t as isize)
        as *mut l_struct_struct_OC_partition_info as *mut core::ffi::c_void;
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn _ZSt4sqrtf(mut _3165: core::ffi::c_float) -> libc::c_float {
    let mut _3166: core::ffi::c_float = 0.;
    let mut _3167: core::ffi::c_float = 0.;
    let mut _3168: core::ffi::c_float = 0.;
    _3166 = _3165;
    _3167 = _3166;
    _3168 = sqrtf(_3167);
    return _3168;
}
