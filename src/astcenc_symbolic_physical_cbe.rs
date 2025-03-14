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
pub struct l_struct_struct_OC_quant_and_transfer_table {
    pub field0: [uint8_t; 32],
    pub field1: [uint8_t; 32],
    pub field2: [uint8_t; 32],
    pub field3: [uint16_t; 65],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_struct_union_OC_anon {
    pub field0: [uint32_t; 4],
    pub field1: [uint8_t; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_struct_struct_OC_symbolic_compressed_block {
    pub field0: uint8_t,
    pub field1: uint8_t,
    pub field2: uint8_t,
    pub field3: uint8_t,
    pub field4: uint16_t,
    pub field5: uint16_t,
    pub field6: [uint8_t; 4],
    pub field7: uint32_t,
    pub field8: core::ffi::c_float,
    pub field9: l_struct_union_OC_anon,
    pub field10: [uint8_t; 64],
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
pub struct l_struct_struct_OC_decimation_info {
    pub field0: uint8_t,
    pub field1: uint8_t,
    pub field2: uint8_t,
    pub field3: uint8_t,
    pub field4: uint8_t,
    pub field5: uint8_t,
    pub field6: [uint8_t; 216],
    pub field7: [struct_AC_l_array_216_uint8_t; 4],
    pub field8: [struct_AC_l_array_216_uint8_t; 4],
    pub field9: [struct_AC_l_array_216_float; 4],
    pub field10: [uint8_t; 64],
    pub field11: [struct_AC_l_array_64_uint8_t; 216],
    pub field12: [struct_AC_l_array_64_float; 216],
    pub field13: [struct_AC_l_array_64_float; 216],
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
pub struct l_struct_struct_OC_partition_info {
    pub field0: uint16_t,
    pub field1: uint16_t,
    pub field2: [uint8_t; 4],
    pub field3: [uint8_t; 216],
    pub field4: [struct_AC_l_array_216_uint8_t; 4],
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
    pub field11: [uint32_t; 4],
    pub field12: [uint32_t; 4],
    pub field13: [struct_AC_l_struct_struct_OC_decimation_mode; 87],
    pub field14: [struct_AC_l_struct_struct_OC_decimation_info; 87],
    pub field15: [uint16_t; 2048],
    pub field16: [struct_AC_l_struct_struct_OC_block_mode; 2048],
    pub field17: [struct_AC_l_struct_struct_OC_partition_info; 3073],
    pub field18: [struct_AC_l_array_1024_uint16_t; 3],
    pub field19: [uint8_t; 64],
    pub field20: [struct_AC_l_array_2_uint64_t; 1024],
    pub field21: [struct_AC_l_array_3_uint64_t; 1024],
    pub field22: [struct_AC_l_array_4_uint64_t; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct [void_KC_; 17] {
    pub array: [*mut core::ffi::c_void; 17],
}
static mut _OC_str: [uint8_t; 34] = unsafe {
    {
        let mut init = {*::core::mem::transmute::<
                &[u8; 34],
                &mut [uint8_t; 34],
            >(b"scb.block_type != SYM_BTYPE_ERROR\0"),
        };
        init
    }
};
static mut _OC_str_OC_1: [uint8_t; 56] = unsafe {
    {
        let mut init = {*::core::mem::transmute::<
                &[u8; 56],
                &mut [uint8_t; 56],
            >(b"/root/astc-encoder/Source/astcenc_symbolic_physical.cpp\0"),
        };
        init
    }
};
static mut __PRETTY_FUNCTION___OC__Z20symbolic_to_physicalRK21block_size_descriptorRK25symbolic_compressed_blockPh: [uint8_t; 103] = unsafe {
    {
        let mut init = {*::core::mem::transmute::<
                &[u8; 103],
                &mut [uint8_t; 103],
            >(
                b"void symbolic_to_physical(const block_size_descriptor &, const symbolic_compressed_block &, uint8_t *)\0",
            ),
        };
        init
    }
};
static mut _ZZ20symbolic_to_physicalRK21block_size_descriptorRK25symbolic_compressed_blockPhE6cbytes: [uint8_t; 8] = unsafe {
    {
        let mut init = {*::core::mem::transmute::<
                &[u8; 8],
                &mut [uint8_t; 8],
            >(b"\xFC\xFD\xFF\xFF\xFF\xFF\xFF\xFF"),
        };
        init
    }
};
static mut _ZZ20symbolic_to_physicalRK21block_size_descriptorRK25symbolic_compressed_blockPhE6cbytes_0: [uint8_t; 8] = unsafe {
    {
        let mut init = {*::core::mem::transmute::<
                &[u8; 8],
                &mut [uint8_t; 8],
            >(b"\xFC\xFF\xFF\xFF\xFF\xFF\xFF\xFF"),
        };
        init
    }
};
static mut _OC_str_OC_2: [uint8_t; 10] = unsafe {
    {
        let mut init = {*::core::mem::transmute::<
                &[u8; 10],
                &mut [uint8_t; 10],
            >(b"vals <= 8\0"),
        };
        init
    }
};
static mut _OC_str_OC_3: [uint8_t; 17] = unsafe {
    {
        let mut init = {*::core::mem::transmute::<
                &[u8; 17],
                &mut [uint8_t; 17],
            >(b"weight_count > 0\0"),
        };
        init
    }
};
static mut __PRETTY_FUNCTION___OC__Z20physical_to_symbolicRK21block_size_descriptorPKhR25symbolic_compressed_block: [uint8_t; 103] = unsafe {
    {
        let mut init = {*::core::mem::transmute::<
                &[u8; 103],
                &mut [uint8_t; 103],
            >(
                b"void physical_to_symbolic(const block_size_descriptor &, const uint8_t *, symbolic_compressed_block &)\0",
            ),
        };
        init
    }
};
static mut _OC_str_OC_4: [uint8_t; 20] = unsafe {
    {
        let mut init = {*::core::mem::transmute::<
                &[u8; 20],
                &mut [uint8_t; 20],
            >(b"partition_count > 0\0"),
        };
        init
    }
};
static mut _ZZ20physical_to_symbolicRK21block_size_descriptorPKhR25symbolic_compressed_blockE14color_bits_arr: [uint32_t; 5] = {
    let mut init = {[
            -(1 as core::ffi::c_int) as uint32_t,
            111,
            99,
            99,
            99,
        ],
    };
    init
};
static mut _OC_str_OC_5: [uint8_t; 82] = unsafe {
    {
        let mut init = {*::core::mem::transmute::<
                &[u8; 82],
                &mut [uint8_t; 82],
            >(
                b"packed_index != BLOCK_BAD_BLOCK_MODE && packed_index < this->block_mode_count_all\0",
            ),
        };
        init
    }
};
static mut _OC_str_OC_6: [uint8_t; 45] = unsafe {
    {
        let mut init = {*::core::mem::transmute::<
                &[u8; 45],
                &mut [uint8_t; 45],
            >(b"/root/astc-encoder/Source/astcenc_internal.h\0"),
        };
        init
    }
};
static mut __PRETTY_FUNCTION___OC__ZNK21block_size_descriptor14get_block_modeEj: [uint8_t; 76] = unsafe {
    {
        let mut init = {*::core::mem::transmute::<
                &[u8; 76],
                &mut [uint8_t; 76],
            >(
                b"const block_mode &block_size_descriptor::get_block_mode(unsigned int) const\0",
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
    r = if condition as core::ffi::c_int != 0 { iftrue } else { ifnot };
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
unsafe extern "C" fn llvm_ashr_u32(mut a: int32_t, mut b: int32_t) -> uint32_t {
    let mut r: uint32_t = (a >> b) as uint32_t;
    return r;
}
#[inline(always)]
unsafe extern "C" fn llvm_and_u8(mut a: uint8_t, mut b: uint8_t) -> uint8_t {
    let mut r: uint8_t = (a as core::ffi::c_int & b as core::ffi::c_int) as uint8_t;
    return r;
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn _Z20symbolic_to_physicalRK21block_size_descriptorRK25symbolic_compressed_blockPh(
    mut _1: *mut core::ffi::c_void,
    mut _2: *mut core::ffi::c_void,
    mut _3: *mut core::ffi::c_void,
) {
    let mut current_block: u64;
    let mut _4: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _5: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _6: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _7: uint32_t = 0;
    let mut _8: uint32_t = 0;
    let mut _9: uint32_t = 0;
    let mut _10: uint32_t = 0;
    let mut _11: uint32_t = 0;
    let mut _12: [uint8_t; 16] = {[0; 16],
    };
    let mut _13: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _14: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _15: uint32_t = 0;
    let mut _16: uint32_t = 0;
    let mut _17: core::ffi::c_float = 0.;
    let mut _18: uint32_t = 0;
    let mut _19: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _20: uint32_t = 0;
    let mut _21: uint32_t = 0;
    let mut _22: [uint8_t; 64] = {[0; 64],
    };
    let mut _23: uint32_t = 0;
    let mut _24: core::ffi::c_float = 0.;
    let mut _25: core::ffi::c_float = 0.;
    let mut _26: uint32_t = 0;
    let mut _27: uint32_t = 0;
    let mut _28: core::ffi::c_float = 0.;
    let mut _29: core::ffi::c_float = 0.;
    let mut _30: uint32_t = 0;
    let mut _31: uint32_t = 0;
    let mut _32: uint32_t = 0;
    let mut _33: uint32_t = 0;
    let mut _34: uint32_t = 0;
    let mut _35: uint32_t = 0;
    let mut _36: uint32_t = 0;
    let mut _37: uint32_t = 0;
    let mut _38: uint32_t = 0;
    let mut _39: uint32_t = 0;
    let mut _40: uint32_t = 0;
    let mut _41: uint32_t = 0;
    let mut _42: uint32_t = 0;
    let mut _43: uint32_t = 0;
    let mut _44: uint32_t = 0;
    let mut _45: uint32_t = 0;
    let mut _46: [uint8_t; 32] = {[0; 32],
    };
    let mut _47: uint32_t = 0;
    let mut _48: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _49: uint32_t = 0;
    let mut _50: uint32_t = 0;
    let mut _51: uint32_t = 0;
    let mut _52: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _53: uint8_t = 0;
    let mut _54: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _55: uint8_t = 0;
    let mut _56: uint32_t = 0;
    let mut _57: uint32_t = 0;
    let mut _58: uint8_t = 0;
    let mut _59: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _60: uint32_t = 0;
    let mut _61: uint32_t = 0;
    let mut _62: uint32_t = 0;
    let mut _63: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _64: uint32_t = 0;
    let mut _65: uint32_t = 0;
    let mut _66: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _67: uint32_t = 0;
    let mut _68: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _69: uint32_t = 0;
    let mut _70: uint32_t = 0;
    let mut _71: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _72: uint32_t = 0;
    let mut _73: uint32_t = 0;
    let mut _74: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _75: uint8_t = 0;
    let mut _76: uint32_t = 0;
    let mut _77: uint32_t = 0;
    let mut _78: uint8_t = 0;
    let mut _79: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _80: uint32_t = 0;
    let mut _81: uint32_t = 0;
    let mut _82: uint32_t = 0;
    let mut _83: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _84: uint32_t = 0;
    let mut _85: uint32_t = 0;
    let mut _86: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _87: uint32_t = 0;
    let mut _88: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _89: uint32_t = 0;
    let mut _90: uint32_t = 0;
    let mut _91: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _92: uint32_t = 0;
    let mut _93: uint32_t = 0;
    let mut _94: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _95: uint8_t = 0;
    let mut _96: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _97: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _98: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _99: uint16_t = 0;
    let mut _100: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _101: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _102: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _103: uint8_t = 0;
    let mut _104: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _105: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _106: uint8_t = 0;
    let mut _107: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _108: uint32_t = 0;
    let mut _109: uint32_t = 0;
    let mut _110: uint32_t = 0;
    let mut _111: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _112: uint8_t = 0;
    let mut _113: uint32_t = 0;
    let mut _114: uint32_t = 0;
    let mut _115: uint32_t = 0;
    let mut _116: uint32_t = 0;
    let mut _117: uint32_t = 0;
    let mut _118: uint32_t = 0;
    let mut _118__PHI_TEMPORARY: uint32_t = 0;
    let mut _119: uint32_t = 0;
    let mut _120: uint32_t = 0;
    let mut _121: uint32_t = 0;
    let mut _122: uint32_t = 0;
    let mut _123: uint32_t = 0;
    let mut _124: uint32_t = 0;
    let mut _125: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _126: uint32_t = 0;
    let mut _127: uint8_t = 0;
    let mut _128: core::ffi::c_float = 0.;
    let mut _129: core::ffi::c_float = 0.;
    let mut _130: core::ffi::c_float = 0.;
    let mut _131: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _132: uint32_t = 0;
    let mut _133: uint8_t = 0;
    let mut _134: uint32_t = 0;
    let mut _135: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _136: uint32_t = 0;
    let mut _137: uint8_t = 0;
    let mut _138: core::ffi::c_float = 0.;
    let mut _139: core::ffi::c_float = 0.;
    let mut _140: core::ffi::c_float = 0.;
    let mut _141: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _142: uint32_t = 0;
    let mut _143: uint8_t = 0;
    let mut _144: uint32_t = 0;
    let mut _145: uint32_t = 0;
    let mut _146: uint32_t = 0;
    let mut _147: uint32_t = 0;
    let mut _148: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _149: uint32_t = 0;
    let mut _150: uint8_t = 0;
    let mut _151: core::ffi::c_float = 0.;
    let mut _152: core::ffi::c_float = 0.;
    let mut _153: core::ffi::c_float = 0.;
    let mut _154: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _155: uint32_t = 0;
    let mut _156: uint8_t = 0;
    let mut _157: uint32_t = 0;
    let mut _158: uint32_t = 0;
    let mut _159: uint32_t = 0;
    let mut _160: uint32_t = 0;
    let mut _161: uint32_t = 0;
    let mut _162: uint32_t = 0;
    let mut _163: uint8_t = 0;
    let mut _164: uint32_t = 0;
    let mut _165: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _166: uint32_t = 0;
    let mut _167: uint32_t = 0;
    let mut _168: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _169: uint16_t = 0;
    let mut _170: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _171: uint32_t = 0;
    let mut _172: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _173: uint32_t = 0;
    let mut _174: uint32_t = 0;
    let mut _175: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _176: uint16_t = 0;
    let mut _177: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _178: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _179: uint16_t = 0;
    let mut _180: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _181: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _182: uint8_t = 0;
    let mut _183: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _184: uint8_t = 0;
    let mut _185: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _186: uint32_t = 0;
    let mut _187: uint32_t = 0;
    let mut _188: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _189: uint32_t = 0;
    let mut _190: uint8_t = 0;
    let mut _191: uint32_t = 0;
    let mut _192: uint32_t = 0;
    let mut _193: uint32_t = 0;
    let mut _194: uint32_t = 0;
    let mut _195: uint32_t = 0;
    let mut _196: uint32_t = 0;
    let mut _197: uint32_t = 0;
    let mut _198: uint32_t = 0;
    let mut _199: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _200: uint32_t = 0;
    let mut _201: uint8_t = 0;
    let mut _202: uint32_t = 0;
    let mut _203: uint32_t = 0;
    let mut _204: uint32_t = 0;
    let mut _205: uint32_t = 0;
    let mut _206: uint32_t = 0;
    let mut _207: uint32_t = 0;
    let mut _208: uint32_t = 0;
    let mut _209: uint32_t = 0;
    let mut _210: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _211: uint32_t = 0;
    let mut _212: uint8_t = 0;
    let mut _213: uint32_t = 0;
    let mut _214: uint32_t = 0;
    let mut _215: uint32_t = 0;
    let mut _216: uint32_t = 0;
    let mut _217: uint32_t = 0;
    let mut _218: uint32_t = 0;
    let mut _219: uint32_t = 0;
    let mut _220: uint32_t = 0;
    let mut _221: uint32_t = 0;
    let mut _222: uint32_t = 0;
    let mut _223: uint32_t = 0;
    let mut _224: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _225: uint32_t = 0;
    let mut _226: uint32_t = 0;
    let mut _227: uint32_t = 0;
    let mut _228: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _229: uint32_t = 0;
    let mut _230: uint32_t = 0;
    let mut _231: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _232: uint8_t = 0;
    let mut _233: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _234: uint32_t = 0;
    let mut _235: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _236: uint8_t = 0;
    let mut _237: uint32_t = 0;
    let mut _238: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _239: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _240: uint32_t = 0;
    let mut _241: uint32_t = 0;
    let mut _242: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _243: uint8_t = 0;
    let mut _244: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _245: uint32_t = 0;
    let mut _246: uint8_t = 0;
    let mut _247: uint32_t = 0;
    let mut _248: uint32_t = 0;
    let mut _249: uint32_t = 0;
    let mut _250: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _251: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _252: uint32_t = 0;
    let mut _253: uint32_t = 0;
    let mut _254: uint8_t = 0;
    let mut _255: uint8_t = 0;
    let mut _256: uint32_t = 0;
    let mut _257: uint32_t = 0;
    let mut _258: uint32_t = 0;
    let mut _259: uint32_t = 0;
    let mut _260: uint32_t = 0;
    let mut _261: uint32_t = 0;
    let mut _262: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _263: uint32_t = 0;
    let mut _264: uint32_t = 0;
    let mut _265: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _266: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _267: uint8_t = 0;
    let mut _268: uint64_t = 0;
    _4 = _1;
    _5 = _2;
    _6 = _3;
    _52 = _5;
    _53 = *(&mut (*(_52 as *mut l_struct_struct_OC_symbolic_compressed_block)).field0
        as *mut uint8_t);
    if _53 as uint32_t != 0 as core::ffi::c_uint {
        _54 = _5;
        _55 = *(&mut (*(_54 as *mut l_struct_struct_OC_symbolic_compressed_block)).field0
            as *mut uint8_t);
        if _55 as uint32_t == 2 as core::ffi::c_uint {
            _7 = 0;
            loop {
                _56 = _7;
                if !(_56 < 8 as core::ffi::c_uint) {
                    break;
                }
                _57 = _7;
                _58 = *(&*(_ZZ20symbolic_to_physicalRK21block_size_descriptorRK25symbolic_compressed_blockPhE6cbytes
                    .array)
                    .as_ptr()
                    .offset(_57 as uint64_t as int64_t as isize) as *const uint8_t
                    as *mut uint8_t);
                _59 = _6;
                _60 = _7;
                *(&mut *(_59 as *mut uint8_t).offset(_60 as uint64_t as int64_t as isize)
                    as *mut uint8_t) = _58;
                _61 = _7;
                _7 = llvm_add_u32(_61, 1);
            }
            _8 = 0;
            loop {
                _62 = _8;
                if !(_62 < 4 as core::ffi::c_uint) {
                    break;
                }
                _63 = _5;
                _64 = _8;
                _65 = *(&mut *((*(&mut (*(_63
                    as *mut l_struct_struct_OC_symbolic_compressed_block))
                    .field9 as *mut l_struct_union_OC_anon as *mut [uint32_t; 4]))
                    .array)
                    .as_mut_ptr()
                    .offset(_64 as uint64_t as int64_t as isize) as *mut uint32_t);
                _66 = _6;
                _67 = _8;
                *(&mut *(_66 as *mut uint8_t)
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
                                ) -> uint32_t)(2, _67),
                            8,
                        ) as uint64_t as int64_t as isize,
                    )
                    as *mut uint8_t) = (_65 & 255) as uint8_t;
                _68 = _5;
                _69 = _8;
                _70 = *(&mut *((*(&mut (*(_68
                    as *mut l_struct_struct_OC_symbolic_compressed_block))
                    .field9 as *mut l_struct_union_OC_anon as *mut [uint32_t; 4]))
                    .array)
                    .as_mut_ptr()
                    .offset(_69 as uint64_t as int64_t as isize) as *mut uint32_t);
                _71 = _6;
                _72 = _8;
                *(&mut *(_71 as *mut uint8_t)
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
                                ) -> uint32_t)(2, _72),
                            9,
                        ) as uint64_t as int64_t as isize,
                    )
                    as *mut uint8_t) = (llvm_ashr_u32(_70 as int32_t, 8 as core::ffi::c_int)
                    & 255) as uint8_t;
                _73 = _8;
                _8 = llvm_add_u32(_73, 1);
            }
        } else {
            _74 = _5;
            _75 = *(&mut (*(_74 as *mut l_struct_struct_OC_symbolic_compressed_block))
                .field0 as *mut uint8_t);
            if _75 as uint32_t == 1 as core::ffi::c_uint {
                _9 = 0;
                loop {
                    _76 = _9;
                    if !(_76 < 8 as core::ffi::c_uint) {
                        break;
                    }
                    _77 = _9;
                    _78 = *(&*(_ZZ20symbolic_to_physicalRK21block_size_descriptorRK25symbolic_compressed_blockPhE6cbytes_0
                        .array)
                        .as_ptr()
                        .offset(_77 as uint64_t as int64_t as isize) as *const uint8_t
                        as *mut uint8_t);
                    _79 = _6;
                    _80 = _9;
                    *(&mut *(_79 as *mut uint8_t)
                        .offset(_80 as uint64_t as int64_t as isize)
                        as *mut uint8_t) = _78;
                    _81 = _9;
                    _9 = llvm_add_u32(_81, 1);
                }
                _10 = 0;
                loop {
                    _82 = _10;
                    if !(_82 < 4 as core::ffi::c_uint) {
                        break;
                    }
                    _83 = _5;
                    _84 = _10;
                    _85 = *(&mut *((*(&mut (*(_83
                        as *mut l_struct_struct_OC_symbolic_compressed_block))
                        .field9 as *mut l_struct_union_OC_anon
                        as *mut [uint32_t; 4]))
                        .array)
                        .as_mut_ptr()
                        .offset(_84 as uint64_t as int64_t as isize) as *mut uint32_t);
                    _86 = _6;
                    _87 = _10;
                    *(&mut *(_86 as *mut uint8_t)
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
                                    ) -> uint32_t)(2, _87),
                                8,
                            ) as uint64_t as int64_t as isize,
                        )
                        as *mut uint8_t) = (_85 & 255)
                        as uint8_t;
                    _88 = _5;
                    _89 = _10;
                    _90 = *(&mut *((*(&mut (*(_88
                        as *mut l_struct_struct_OC_symbolic_compressed_block))
                        .field9 as *mut l_struct_union_OC_anon
                        as *mut [uint32_t; 4]))
                        .array)
                        .as_mut_ptr()
                        .offset(_89 as uint64_t as int64_t as isize) as *mut uint32_t);
                    _91 = _6;
                    _92 = _10;
                    *(&mut *(_91 as *mut uint8_t)
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
                                    ) -> uint32_t)(2, _92),
                                9,
                            ) as uint64_t as int64_t as isize,
                        )
                        as *mut uint8_t) = (llvm_ashr_u32(
                        _90 as int32_t,
                        8 as core::ffi::c_int,
                    ) & 255) as uint8_t;
                    _93 = _10;
                    _10 = llvm_add_u32(_93, 1);
                }
            } else {
                _94 = _5;
                _95 = *(&mut (*(_94
                    as *mut l_struct_struct_OC_symbolic_compressed_block))
                    .field1 as *mut uint8_t);
                _11 = _95 as uint32_t;
                _96 = memset(
                    &mut _12 as *mut [uint8_t; 16] as *mut core::ffi::c_void,
                    0,
                    16,
                );
                _97 = _4;
                _98 = _5;
                _99 = *(&mut (*(_98
                    as *mut l_struct_struct_OC_symbolic_compressed_block))
                    .field4 as *mut uint16_t);
                _100 = _ZNK21block_size_descriptor14get_block_modeEj(
                    _97,
                    _99 as uint32_t,
                );
                _13 = _100;
                _101 = _4;
                _102 = _13;
                _103 = *(&mut (*(_102 as *mut l_struct_struct_OC_block_mode)).field1
                    as *mut uint8_t);
                _104 = _ZNK21block_size_descriptor19get_decimation_infoEj(
                    _101,
                    _103 as uint32_t,
                );
                _14 = _104;
                _105 = _14;
                _106 = *(&mut (*(_105 as *mut l_struct_struct_OC_decimation_info)).field2
                    as *mut uint8_t);
                _15 = _106 as uint32_t;
                _107 = _13;
                _108 = _ZNK10block_mode21get_weight_quant_modeEv(_107);
                _16 = _108;
                _109 = _16;
                _110 = _ZL15get_quant_level12quant_method(_109);
                _17 = _110 as core::ffi::c_float;
                _111 = _13;
                _112 = *(&mut (*(_111 as *mut l_struct_struct_OC_block_mode)).field4
                    as *mut uint8_t);
                _18 = llvm_and_u8(_112, 1 as core::ffi::c_int as uint8_t) as uint32_t;
                _113 = _16;
                _19 = &mut *(quant_and_xfer_tables.array)
                    .as_mut_ptr()
                    .offset(_113 as uint64_t as int64_t as isize)
                    as *mut l_struct_struct_OC_quant_and_transfer_table
                    as *mut core::ffi::c_void;
                _114 = _18;
                if _114 != 0 as core::ffi::c_uint {
                    _115 = _15;
                    _116 = llvm_mul_u32(2, _115);
                    _118__PHI_TEMPORARY = _116;
                } else {
                    _117 = _15;
                    _118__PHI_TEMPORARY = _117;
                }
                _118 = _118__PHI_TEMPORARY;
                _20 = _118;
                _119 = _20;
                _120 = _16;
                _121 = _Z25get_ise_sequence_bitcountj12quant_method(_119, _120);
                _21 = _121;
                _122 = _18;
                if _122 != 0 as core::ffi::c_uint {
                    _23 = 0;
                    loop {
                        _123 = _23;
                        _124 = _15;
                        if !((_123 as int32_t) < _124 as int32_t) {
                            break;
                        }
                        _125 = _5;
                        _126 = _23;
                        _127 = *(&mut *((*(&mut (*(_125
                            as *mut l_struct_struct_OC_symbolic_compressed_block))
                            .field10 as *mut [uint8_t; 64]))
                            .array)
                            .as_mut_ptr()
                            .offset(_126 as int32_t as int64_t as isize)
                            as *mut uint8_t);
                        _24 = _127 as core::ffi::c_float;
                        _128 = _24;
                        _129 = _17;
                        _25 = llvm_fmul_f32(
                            llvm_fdiv_f32(_128, 64 as core::ffi::c_int as core::ffi::c_float),
                            llvm_fsub_f32(_129, 1 as core::ffi::c_int as core::ffi::c_float),
                        );
                        _130 = _25;
                        _26 = llvm_fadd_f32(_130, 0.5f64 as core::ffi::c_float) as int32_t
                            as uint32_t;
                        _131 = _19;
                        _132 = _26;
                        _133 = *(&mut *((*(&mut (*(_131
                            as *mut l_struct_struct_OC_quant_and_transfer_table))
                            .field1 as *mut [uint8_t; 32]))
                            .array)
                            .as_mut_ptr()
                            .offset(_132 as int32_t as int64_t as isize)
                            as *mut uint8_t);
                        _134 = _23;
                        *(&mut *(_22.array)
                            .as_mut_ptr()
                            .offset(
                                (llvm_mul_u32
                                    as unsafe extern "C" fn(
                                        uint32_t,
                                        uint32_t,
                                    ) -> uint32_t)(2, _134)
                                    as int32_t as int64_t as isize,
                            ) as *mut uint8_t) = _133;
                        _135 = _5;
                        _136 = _23;
                        _137 = *(&mut *((*(&mut (*(_135
                            as *mut l_struct_struct_OC_symbolic_compressed_block))
                            .field10 as *mut [uint8_t; 64]))
                            .array)
                            .as_mut_ptr()
                            .offset(
                                (llvm_add_u32
                                    as unsafe extern "C" fn(
                                        uint32_t,
                                        uint32_t,
                                    ) -> uint32_t)(_136, 32)
                                    as uint64_t as int64_t as isize,
                            ) as *mut uint8_t);
                        _24 = _137 as core::ffi::c_float;
                        _138 = _24;
                        _139 = _17;
                        _25 = llvm_fmul_f32(
                            llvm_fdiv_f32(_138, 64 as core::ffi::c_int as core::ffi::c_float),
                            llvm_fsub_f32(_139, 1 as core::ffi::c_int as core::ffi::c_float),
                        );
                        _140 = _25;
                        _26 = llvm_fadd_f32(_140, 0.5f64 as core::ffi::c_float) as int32_t
                            as uint32_t;
                        _141 = _19;
                        _142 = _26;
                        _143 = *(&mut *((*(&mut (*(_141
                            as *mut l_struct_struct_OC_quant_and_transfer_table))
                            .field1 as *mut [uint8_t; 32]))
                            .array)
                            .as_mut_ptr()
                            .offset(_142 as int32_t as int64_t as isize)
                            as *mut uint8_t);
                        _144 = _23;
                        *(&mut *(_22.array)
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
                                        ) -> uint32_t)(2, _144),
                                    1,
                                ) as int32_t as int64_t as isize,
                            ) as *mut uint8_t) = _143;
                        _145 = _23;
                        _23 = llvm_add_u32(_145, 1);
                    }
                } else {
                    _27 = 0;
                    loop {
                        _146 = _27;
                        _147 = _15;
                        if !((_146 as int32_t) < _147 as int32_t) {
                            break;
                        }
                        _148 = _5;
                        _149 = _27;
                        _150 = *(&mut *((*(&mut (*(_148
                            as *mut l_struct_struct_OC_symbolic_compressed_block))
                            .field10 as *mut [uint8_t; 64]))
                            .array)
                            .as_mut_ptr()
                            .offset(_149 as int32_t as int64_t as isize)
                            as *mut uint8_t);
                        _28 = _150 as core::ffi::c_float;
                        _151 = _28;
                        _152 = _17;
                        _29 = llvm_fmul_f32(
                            llvm_fdiv_f32(_151, 64 as core::ffi::c_int as core::ffi::c_float),
                            llvm_fsub_f32(_152, 1 as core::ffi::c_int as core::ffi::c_float),
                        );
                        _153 = _29;
                        _30 = llvm_fadd_f32(_153, 0.5f64 as core::ffi::c_float) as int32_t
                            as uint32_t;
                        _154 = _19;
                        _155 = _30;
                        _156 = *(&mut *((*(&mut (*(_154
                            as *mut l_struct_struct_OC_quant_and_transfer_table))
                            .field1 as *mut [uint8_t; 32]))
                            .array)
                            .as_mut_ptr()
                            .offset(_155 as int32_t as int64_t as isize)
                            as *mut uint8_t);
                        _157 = _27;
                        *(&mut *(_22.array)
                            .as_mut_ptr()
                            .offset(_157 as int32_t as int64_t as isize)
                            as *mut uint8_t) = _156;
                        _158 = _27;
                        _27 = llvm_add_u32(_158, 1);
                    }
                }
                _159 = _16;
                _160 = _20;
                _Z10encode_ise12quant_methodjPKhPhj(
                    _159,
                    _160,
                    &mut *(_22.array)
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as int64_t as isize) as *mut uint8_t
                        as *mut core::ffi::c_void,
                    &mut *(_12.array)
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as int64_t as isize) as *mut uint8_t
                        as *mut core::ffi::c_void,
                    0,
                );
                _31 = 0;
                loop {
                    _161 = _31;
                    if !((_161 as int32_t) < 16 as core::ffi::c_uint as int32_t) {
                        break;
                    }
                    _162 = _31;
                    _163 = *(&mut *(_12.array)
                        .as_mut_ptr()
                        .offset(
                            (llvm_sub_u32
                                as unsafe extern "C" fn(
                                    uint32_t,
                                    uint32_t,
                                ) -> uint32_t)(15, _162)
                                as int32_t as int64_t as isize,
                        ) as *mut uint8_t);
                    _164 = _ZL7bitrev8i(_163 as uint32_t);
                    _165 = _6;
                    _166 = _31;
                    *(&mut *(_165 as *mut uint8_t)
                        .offset(_166 as int32_t as int64_t as isize)
                        as *mut uint8_t) = _164 as uint8_t;
                    _167 = _31;
                    _31 = llvm_add_u32(_167, 1);
                }
                _168 = _5;
                _169 = *(&mut (*(_168
                    as *mut l_struct_struct_OC_symbolic_compressed_block))
                    .field4 as *mut uint16_t);
                _170 = _6;
                _ZL10write_bitsiiiPh(
                    _169 as uint32_t,
                    11,
                    0,
                    _170,
                );
                _171 = _11;
                _172 = _6;
                _ZL10write_bitsiiiPh(
                    llvm_sub_u32(_171, 1),
                    2,
                    11,
                    _172,
                );
                _173 = _21;
                _32 = llvm_sub_u32(128, _173);
                _174 = _11;
                if _174 > 1 as core::ffi::c_uint {
                    _175 = _5;
                    _176 = *(&mut (*(_175
                        as *mut l_struct_struct_OC_symbolic_compressed_block))
                        .field5 as *mut uint16_t);
                    _177 = _6;
                    _ZL10write_bitsiiiPh(
                        _176 as uint32_t,
                        6,
                        13,
                        _177,
                    );
                    _178 = _5;
                    _179 = *(&mut (*(_178
                        as *mut l_struct_struct_OC_symbolic_compressed_block))
                        .field5 as *mut uint16_t);
                    _180 = _6;
                    _ZL10write_bitsiiiPh(
                        llvm_ashr_u32(_179 as uint32_t as int32_t, 6 as core::ffi::c_int),
                        4,
                        19,
                        _180,
                    );
                    _181 = _5;
                    _182 = *(&mut (*(_181
                        as *mut l_struct_struct_OC_symbolic_compressed_block))
                        .field2 as *mut uint8_t);
                    if _182 as core::ffi::c_int != 0 as core::ffi::c_int as uint8_t as core::ffi::c_int
                    {
                        _183 = _5;
                        _184 = *(&mut *((*(&mut (*(_183
                            as *mut l_struct_struct_OC_symbolic_compressed_block))
                            .field6 as *mut [uint8_t; 4]))
                            .array)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as int64_t as isize)
                            as *mut uint8_t);
                        _185 = _6;
                        _ZL10write_bitsiiiPh(
                            (_184 as uint32_t) << 2 as core::ffi::c_int,
                            6,
                            23,
                            _185,
                        );
                    } else {
                        _33 = 4;
                        _34 = 0;
                        loop {
                            _186 = _34;
                            _187 = _11;
                            if !(_186 < _187) {
                                break;
                            }
                            _188 = _5;
                            _189 = _34;
                            _190 = *(&mut *((*(&mut (*(_188
                                as *mut l_struct_struct_OC_symbolic_compressed_block))
                                .field6 as *mut [uint8_t; 4]))
                                .array)
                                .as_mut_ptr()
                                .offset(_189 as uint64_t as int64_t as isize)
                                as *mut uint8_t);
                            _35 = llvm_ashr_u32(
                                _190 as uint32_t as int32_t,
                                2 as core::ffi::c_int,
                            );
                            _191 = _35;
                            _192 = _33;
                            _193 = _ZN4astcL3minIiEET_S1_S1_(_191, _192);
                            _33 = _193;
                            _194 = _34;
                            _34 = llvm_add_u32(_194, 1);
                        }
                        _195 = _33;
                        if _195 == 3 as core::ffi::c_uint {
                            _33 = 2;
                        }
                        _196 = _33;
                        _36 = llvm_add_u32(_196, 1);
                        _37 = 2;
                        _38 = 0;
                        loop {
                            _197 = _38;
                            _198 = _11;
                            if !(_197 < _198) {
                                break;
                            }
                            _199 = _5;
                            _200 = _38;
                            _201 = *(&mut *((*(&mut (*(_199
                                as *mut l_struct_struct_OC_symbolic_compressed_block))
                                .field6 as *mut [uint8_t; 4]))
                                .array)
                                .as_mut_ptr()
                                .offset(_200 as uint64_t as int64_t as isize)
                                as *mut uint8_t);
                            _202 = _33;
                            _39 = llvm_sub_u32(
                                llvm_ashr_u32(
                                    _201 as uint32_t as int32_t,
                                    2 as core::ffi::c_int,
                                ),
                                _202,
                            );
                            _203 = _39;
                            _204 = _37;
                            _205 = _36;
                            _36 = _205 | _203 << _204;
                            _206 = _37;
                            _37 = llvm_add_u32(_206, 1);
                            _207 = _38;
                            _38 = llvm_add_u32(_207, 1);
                        }
                        _40 = 0;
                        loop {
                            _208 = _40;
                            _209 = _11;
                            if !(_208 < _209) {
                                break;
                            }
                            _210 = _5;
                            _211 = _40;
                            _212 = *(&mut *((*(&mut (*(_210
                                as *mut l_struct_struct_OC_symbolic_compressed_block))
                                .field6 as *mut [uint8_t; 4]))
                                .array)
                                .as_mut_ptr()
                                .offset(_211 as uint64_t as int64_t as isize)
                                as *mut uint8_t);
                            _41 = _212 as uint32_t & 3;
                            _213 = _41;
                            _214 = _37;
                            _215 = _36;
                            _36 = _215 | _213 << _214;
                            _216 = _37;
                            _37 = llvm_add_u32(_216, 2);
                            _217 = _40;
                            _40 = llvm_add_u32(_217, 1);
                        }
                        _218 = _36;
                        _42 = _218 & 63;
                        _219 = _36;
                        _43 = llvm_ashr_u32(_219 as int32_t, 6 as core::ffi::c_int);
                        _220 = _11;
                        _44 = llvm_sub_u32(
                            llvm_mul_u32(3, _220),
                            4,
                        );
                        _221 = _21;
                        _222 = _44;
                        _45 = llvm_sub_u32(
                            llvm_sub_u32(128, _221),
                            _222,
                        );
                        _223 = _42;
                        _224 = _6;
                        _ZL10write_bitsiiiPh(
                            _223,
                            6,
                            23,
                            _224,
                        );
                        _225 = _43;
                        _226 = _44;
                        _227 = _45;
                        _228 = _6;
                        _ZL10write_bitsiiiPh(_225, _226, _227, _228);
                        _229 = _44;
                        _230 = _32;
                        _32 = llvm_sub_u32(_230, _229);
                    }
                } else {
                    _231 = _5;
                    _232 = *(&mut *((*(&mut (*(_231
                        as *mut l_struct_struct_OC_symbolic_compressed_block))
                        .field6 as *mut [uint8_t; 4]))
                        .array)
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as int64_t as isize) as *mut uint8_t);
                    _233 = _6;
                    _ZL10write_bitsiiiPh(
                        _232 as uint32_t,
                        4,
                        13,
                        _233,
                    );
                }
                _234 = _18;
                if _234 != 0 as core::ffi::c_uint {
                    _235 = _5;
                    _236 = *(&mut (*(_235
                        as *mut l_struct_struct_OC_symbolic_compressed_block))
                        .field3 as *mut uint8_t);
                    _237 = _32;
                    _238 = _6;
                    _ZL10write_bitsiiiPh(
                        _236 as int8_t as int32_t as uint32_t,
                        2,
                        llvm_sub_u32(_237, 2),
                        _238,
                    );
                }
                _47 = 0;
                _239 = _5;
                _240 = *(&mut (*(_239
                    as *mut l_struct_struct_OC_symbolic_compressed_block))
                    .field7 as *mut uint32_t);
                _48 = &mut *((*(&*(color_uquant_to_scrambled_pquant_tables.array)
                    .as_ptr()
                    .offset(
                        (llvm_sub_u32
                            as unsafe extern "C" fn(
                                uint32_t,
                                uint32_t,
                            ) -> uint32_t)(_240, 4) as int32_t
                            as int64_t as isize,
                    ) as *const [uint8_t; 256] as *mut [uint8_t; 256]))
                    .array)
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as int64_t as isize) as *mut uint8_t
                    as *mut core::ffi::c_void;
                _49 = 0;
                loop {
                    _241 = _49;
                    _242 = _5;
                    _243 = *(&mut (*(_242
                        as *mut l_struct_struct_OC_symbolic_compressed_block))
                        .field1 as *mut uint8_t);
                    if !(_241 < _243 as uint32_t) {
                        current_block = 7649294695629136044;
                        break;
                    }
                    _244 = _5;
                    _245 = _49;
                    _246 = *(&mut *((*(&mut (*(_244
                        as *mut l_struct_struct_OC_symbolic_compressed_block))
                        .field6 as *mut [uint8_t; 4]))
                        .array)
                        .as_mut_ptr()
                        .offset(_245 as uint64_t as int64_t as isize) as *mut uint8_t);
                    _50 = llvm_add_u32(
                        llvm_mul_u32(
                            2,
                            llvm_ashr_u32(_246 as uint32_t as int32_t, 2 as core::ffi::c_int),
                        ),
                        2,
                    );
                    _247 = _50;
                    if !(_247 as int32_t <= 8 as core::ffi::c_uint as int32_t) {
                        current_block = 11333923632367046402;
                        break;
                    }
                    _51 = 0;
                    loop {
                        _248 = _51;
                        _249 = _50;
                        if !((_248 as int32_t) < _249 as int32_t) {
                            break;
                        }
                        _250 = _48;
                        _251 = _5;
                        _252 = _49;
                        _253 = _51;
                        _254 = *(&mut *((*(&mut *((*(&mut (*(_251
                            as *mut l_struct_struct_OC_symbolic_compressed_block))
                            .field9 as *mut l_struct_union_OC_anon
                            as *mut [struct_AC_l_array_8_uint8_t; 4]))
                            .array)
                            .as_mut_ptr()
                            .offset(_252 as uint64_t as int64_t as isize)
                            as *mut [uint8_t; 8]))
                            .array)
                            .as_mut_ptr()
                            .offset(_253 as int32_t as int64_t as isize)
                            as *mut uint8_t);
                        _255 = *(&mut *(_250 as *mut uint8_t)
                            .offset(_254 as uint64_t as int64_t as isize)
                            as *mut uint8_t);
                        _256 = _51;
                        _257 = _47;
                        *(&mut *(_46.array)
                            .as_mut_ptr()
                            .offset(
                                (llvm_add_u32
                                    as unsafe extern "C" fn(
                                        uint32_t,
                                        uint32_t,
                                    ) -> uint32_t)(_256, _257) as int32_t as int64_t as isize,
                            ) as *mut uint8_t) = _255;
                        _258 = _51;
                        _51 = llvm_add_u32(_258, 1);
                    }
                    _259 = _50;
                    _260 = _47;
                    _47 = llvm_add_u32(_260, _259);
                    _261 = _49;
                    _49 = llvm_add_u32(_261, 1);
                }
                match current_block {
                    11333923632367046402 => {
                        __assert_fail(
                            &_OC_str_OC_2 as *const [uint8_t; 10]
                                as *mut core::ffi::c_void,
                            &_OC_str_OC_1 as *const [uint8_t; 56]
                                as *mut core::ffi::c_void,
                            276,
                            &__PRETTY_FUNCTION___OC__Z20symbolic_to_physicalRK21block_size_descriptorRK25symbolic_compressed_blockPh
                                as *const [uint8_t; 103] as *mut core::ffi::c_void,
                        );
                    }
                    _ => {
                        _262 = _5;
                        _263 = _ZNK25symbolic_compressed_block20get_color_quant_modeEv(
                            _262,
                        );
                        _264 = _47;
                        _265 = _6;
                        _266 = _5;
                        _267 = *(&mut (*(_266
                            as *mut l_struct_struct_OC_symbolic_compressed_block))
                            .field1 as *mut uint8_t);
                        _268 = (_267 as uint32_t == 1 as core::ffi::c_uint) as core::ffi::c_int
                            as bool_0 as uint64_t;
                        _Z10encode_ise12quant_methodjPKhPhj(
                            _263,
                            _264,
                            &mut *(_46.array)
                                .as_mut_ptr()
                                .offset(0 as core::ffi::c_int as int64_t as isize)
                                as *mut uint8_t as *mut core::ffi::c_void,
                            _265,
                            llvm_select_u32(
                                (_267 as uint32_t == 1 as core::ffi::c_uint) as core::ffi::c_int
                                    as bool_0,
                                17,
                                29,
                            ),
                        );
                    }
                }
            }
        }
        return;
    } else {
        __assert_fail(
            &_OC_str as *const [uint8_t; 34] as *mut core::ffi::c_void,
            &_OC_str_OC_1 as *const [uint8_t; 56] as *mut core::ffi::c_void,
            107,
            &__PRETTY_FUNCTION___OC__Z20symbolic_to_physicalRK21block_size_descriptorRK25symbolic_compressed_blockPh
                as *const [uint8_t; 103] as *mut core::ffi::c_void,
        );
    };
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn _ZNK21block_size_descriptor14get_block_modeEj(
    mut _344: *mut core::ffi::c_void,
    mut _345: uint32_t,
) -> *mut core::ffi::c_void {
    let mut _346: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _347: uint32_t = 0;
    let mut _348: uint32_t = 0;
    let mut _349: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _350: uint32_t = 0;
    let mut _351: uint16_t = 0;
    let mut _352: uint32_t = 0;
    let mut _353: uint32_t = 0;
    let mut _354: uint32_t = 0;
    let mut _355: bool_0 = 0;
    let mut _355__PHI_TEMPORARY: bool_0 = 0;
    let mut _356: uint32_t = 0;
    _346 = _344;
    _347 = _345;
    _349 = _346;
    _350 = _347;
    _351 = *(&mut *((*(&mut (*(_349 as *mut l_struct_struct_OC_block_size_descriptor))
        .field15 as *mut [uint16_t; 2048]))
        .array)
        .as_mut_ptr()
        .offset(_350 as uint64_t as int64_t as isize) as *mut uint16_t);
    _348 = _351 as uint32_t;
    _352 = _348;
    if _352 != 65535 as core::ffi::c_uint {
        _353 = _348;
        _354 = *(&mut (*(_349 as *mut l_struct_struct_OC_block_size_descriptor)).field10
            as *mut uint32_t);
        _355__PHI_TEMPORARY = (_353 < _354) as core::ffi::c_int as bool_0;
    } else {
        _355__PHI_TEMPORARY = 0 as core::ffi::c_int as bool_0;
    }
    _355 = _355__PHI_TEMPORARY;
    if _355 != 0 {
        _356 = _348;
        return &mut *((*(&mut (*(_349 as *mut l_struct_struct_OC_block_size_descriptor))
            .field16 as *mut [struct_AC_l_struct_struct_OC_block_mode; 2048]))
            .array)
            .as_mut_ptr()
            .offset(_356 as uint64_t as int64_t as isize)
            as *mut l_struct_struct_OC_block_mode as *mut core::ffi::c_void;
    } else {
        __assert_fail(
            &_OC_str_OC_5 as *const [uint8_t; 82] as *mut core::ffi::c_void,
            &_OC_str_OC_6 as *const [uint8_t; 45] as *mut core::ffi::c_void,
            639,
            &__PRETTY_FUNCTION___OC__ZNK21block_size_descriptor14get_block_modeEj
                as *const [uint8_t; 76] as *mut core::ffi::c_void,
        );
    };
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn _ZNK21block_size_descriptor19get_decimation_infoEj(
    mut _362: *mut core::ffi::c_void,
    mut _363: uint32_t,
) -> *mut core::ffi::c_void {
    let mut _364: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _365: uint32_t = 0;
    let mut _366: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _367: uint32_t = 0;
    _364 = _362;
    _365 = _363;
    _366 = _364;
    _367 = _365;
    return &mut *((*(&mut (*(_366 as *mut l_struct_struct_OC_block_size_descriptor))
        .field14 as *mut [struct_AC_l_struct_struct_OC_decimation_info; 87]))
        .array)
        .as_mut_ptr()
        .offset(_367 as uint64_t as int64_t as isize)
        as *mut l_struct_struct_OC_decimation_info as *mut core::ffi::c_void;
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn _ZNK10block_mode21get_weight_quant_modeEv(
    mut _368: *mut core::ffi::c_void,
) -> uint32_t {
    let mut _369: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _370: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _371: uint8_t = 0;
    _369 = _368;
    _370 = _369;
    _371 = *(&mut (*(_370 as *mut l_struct_struct_OC_block_mode)).field2
        as *mut uint8_t);
    return _371 as uint32_t;
}
#[inline(never)]
unsafe extern "C" fn _ZL15get_quant_level12quant_method(mut _372: uint32_t) -> uint32_t {
    let mut _373: uint32_t = 0;
    let mut _374: uint32_t = 0;
    let mut _375: uint32_t = 0;
    let mut _376: uint32_t = 0;
    _374 = _372;
    _375 = _374;
    match _375 {
        0 => {
            _373 = 2;
        }
        1 => {
            _373 = 3;
        }
        2 => {
            _373 = 4;
        }
        3 => {
            _373 = 5;
        }
        4 => {
            _373 = 6;
        }
        5 => {
            _373 = 8;
        }
        6 => {
            _373 = 10;
        }
        7 => {
            _373 = 12;
        }
        8 => {
            _373 = 16;
        }
        9 => {
            _373 = 20;
        }
        10 => {
            _373 = 24;
        }
        11 => {
            _373 = 32;
        }
        12 => {
            _373 = 40;
        }
        13 => {
            _373 = 48;
        }
        14 => {
            _373 = 64;
        }
        15 => {
            _373 = 80;
        }
        16 => {
            _373 = 96;
        }
        17 => {
            _373 = 128;
        }
        18 => {
            _373 = 160;
        }
        19 => {
            _373 = 192;
        }
        20 => {
            _373 = 256;
        }
        _ => {
            _373 = 0;
        }
    }
    _376 = _373;
    return _376;
}
#[inline(never)]
unsafe extern "C" fn _ZL7bitrev8i(mut _400: uint32_t) -> uint32_t {
    let mut _401: uint32_t = 0;
    let mut _402: uint32_t = 0;
    let mut _403: uint32_t = 0;
    let mut _404: uint32_t = 0;
    let mut _405: uint32_t = 0;
    let mut _406: uint32_t = 0;
    let mut _407: uint32_t = 0;
    let mut _408: uint32_t = 0;
    _401 = _400;
    _402 = _401;
    _403 = _401;
    _401 = (_402 & 15) << 4 as core::ffi::c_int
        | llvm_ashr_u32(_403 as int32_t, 4 as core::ffi::c_int)
            & 15;
    _404 = _401;
    _405 = _401;
    _401 = (_404 & 51) << 2 as core::ffi::c_int
        | llvm_ashr_u32(_405 as int32_t, 2 as core::ffi::c_int)
            & 51;
    _406 = _401;
    _407 = _401;
    _401 = (_406 & 85) << 1 as core::ffi::c_int
        | llvm_ashr_u32(_407 as int32_t, 1 as core::ffi::c_int)
            & 85;
    _408 = _401;
    return _408;
}
#[inline(never)]
unsafe extern "C" fn _ZL10write_bitsiiiPh(
    mut _409: uint32_t,
    mut _410: uint32_t,
    mut _411: uint32_t,
    mut _412: *mut core::ffi::c_void,
) {
    let mut _413: uint32_t = 0;
    let mut _414: uint32_t = 0;
    let mut _415: uint32_t = 0;
    let mut _416: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _417: uint32_t = 0;
    let mut _418: uint32_t = 0;
    let mut _419: uint32_t = 0;
    let mut _420: uint32_t = 0;
    let mut _421: uint32_t = 0;
    let mut _422: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _423: uint32_t = 0;
    let mut _424: uint32_t = 0;
    let mut _425: uint32_t = 0;
    let mut _426: uint32_t = 0;
    let mut _427: uint32_t = 0;
    let mut _428: uint32_t = 0;
    let mut _429: uint32_t = 0;
    let mut _430: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _431: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _432: uint8_t = 0;
    let mut _433: uint32_t = 0;
    let mut _434: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _435: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _436: uint8_t = 0;
    let mut _437: uint32_t = 0;
    let mut _438: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _439: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _440: uint8_t = 0;
    let mut _441: uint32_t = 0;
    let mut _442: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _443: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _444: uint8_t = 0;
    _413 = _409;
    _414 = _410;
    _415 = _411;
    _416 = _412;
    _418 = _414;
    _417 = llvm_sub_u32(
        ((1 as core::ffi::c_int) << _418) as uint32_t,
        1,
    );
    _419 = _417;
    _420 = _413;
    _413 = _420 & _419;
    _421 = _415;
    _422 = _416;
    _416 = &mut *(_422 as *mut uint8_t)
        .offset(
            (llvm_ashr_u32
                as unsafe extern "C" fn(
                    int32_t,
                    int32_t,
                ) -> uint32_t)(_421 as int32_t, 3 as core::ffi::c_int) as int32_t as int64_t
                as isize,
        ) as *mut uint8_t as *mut core::ffi::c_void;
    _423 = _415;
    _415 = _423 & 7;
    _424 = _415;
    _425 = _413;
    _413 = _425 << _424;
    _426 = _415;
    _427 = _417;
    _417 = _427 << _426;
    _428 = _417;
    _417 = _428 ^ -(1 as core::ffi::c_int) as uint32_t;
    _429 = _417;
    _430 = _416;
    _431 = _430 as *mut uint8_t as *mut core::ffi::c_void;
    _432 = *(_431 as *mut uint8_t);
    *(_431 as *mut uint8_t) = (_432 as uint32_t & _429) as uint8_t;
    _433 = _413;
    _434 = _416;
    _435 = _434 as *mut uint8_t as *mut core::ffi::c_void;
    _436 = *(_435 as *mut uint8_t);
    *(_435 as *mut uint8_t) = (_436 as uint32_t | _433) as uint8_t;
    _437 = _417;
    _438 = _416;
    _439 = &mut *(_438 as *mut uint8_t).offset(1 as core::ffi::c_int as int64_t as isize)
        as *mut uint8_t as *mut core::ffi::c_void;
    _440 = *(_439 as *mut uint8_t);
    *(_439
        as *mut uint8_t) = (_440 as uint32_t
        & llvm_ashr_u32(_437 as int32_t, 8 as core::ffi::c_int)) as uint8_t;
    _441 = _413;
    _442 = _416;
    _443 = &mut *(_442 as *mut uint8_t).offset(1 as core::ffi::c_int as int64_t as isize)
        as *mut uint8_t as *mut core::ffi::c_void;
    _444 = *(_443 as *mut uint8_t);
    *(_443
        as *mut uint8_t) = (_444 as uint32_t
        | llvm_ashr_u32(_441 as int32_t, 8 as core::ffi::c_int)) as uint8_t;
}
#[inline(never)]
unsafe extern "C" fn _ZN4astcL3minIiEET_S1_S1_(
    mut _445: uint32_t,
    mut _446: uint32_t,
) -> uint32_t {
    let mut _447: uint32_t = 0;
    let mut _448: uint32_t = 0;
    let mut _449: uint32_t = 0;
    let mut _450: uint32_t = 0;
    let mut _451: uint32_t = 0;
    let mut _452: uint32_t = 0;
    let mut _453: uint32_t = 0;
    let mut _453__PHI_TEMPORARY: uint32_t = 0;
    _447 = _445;
    _448 = _446;
    _449 = _447;
    _450 = _448;
    if (_449 as int32_t) < _450 as int32_t {
        _451 = _447;
        _453__PHI_TEMPORARY = _451;
    } else {
        _452 = _448;
        _453__PHI_TEMPORARY = _452;
    }
    _453 = _453__PHI_TEMPORARY;
    return _453;
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn _ZNK25symbolic_compressed_block20get_color_quant_modeEv(
    mut _457: *mut core::ffi::c_void,
) -> uint32_t {
    let mut _458: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _459: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _460: uint32_t = 0;
    _458 = _457;
    _459 = _458;
    _460 = *(&mut (*(_459 as *mut l_struct_struct_OC_symbolic_compressed_block)).field7
        as *mut uint32_t);
    return _460;
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn _Z20physical_to_symbolicRK21block_size_descriptorPKhR25symbolic_compressed_block(
    mut _461: *mut core::ffi::c_void,
    mut _462: *mut core::ffi::c_void,
    mut _463: *mut core::ffi::c_void,
) {
    let mut current_block: u64;
    let mut _464: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _465: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _466: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _467: [uint8_t; 16] = {[0; 16],
    };
    let mut _468: uint32_t = 0;
    let mut _469: uint32_t = 0;
    let mut _470: uint32_t = 0;
    let mut _471: uint32_t = 0;
    let mut _472: uint32_t = 0;
    let mut _473: uint32_t = 0;
    let mut _474: uint32_t = 0;
    let mut _475: uint32_t = 0;
    let mut _476: uint32_t = 0;
    let mut _477: uint32_t = 0;
    let mut _478: uint32_t = 0;
    let mut _479: uint32_t = 0;
    let mut _480: uint32_t = 0;
    let mut _481: uint32_t = 0;
    let mut _482: uint32_t = 0;
    let mut _483: uint32_t = 0;
    let mut _484: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _485: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _486: uint32_t = 0;
    let mut _487: uint32_t = 0;
    let mut _488: uint32_t = 0;
    let mut _489: uint32_t = 0;
    let mut _490: uint32_t = 0;
    let mut _491: uint32_t = 0;
    let mut _492: uint32_t = 0;
    let mut _493: uint32_t = 0;
    let mut _494: [uint8_t; 64] = {[0; 64],
    };
    let mut _495: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _496: uint32_t = 0;
    let mut _497: uint32_t = 0;
    let mut _498: [uint32_t; 4] = {[0; 4],
    };
    let mut _499: uint32_t = 0;
    let mut _500: uint32_t = 0;
    let mut _501: uint32_t = 0;
    let mut _502: uint32_t = 0;
    let mut _503: uint32_t = 0;
    let mut _504: uint32_t = 0;
    let mut _505: uint32_t = 0;
    let mut _506: uint32_t = 0;
    let mut _507: uint32_t = 0;
    let mut _508: uint32_t = 0;
    let mut _509: uint32_t = 0;
    let mut _510: uint32_t = 0;
    let mut _511: uint32_t = 0;
    let mut _512: [uint8_t; 32] = {[0; 32],
    };
    let mut _513: uint32_t = 0;
    let mut _514: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _515: uint32_t = 0;
    let mut _516: uint32_t = 0;
    let mut _517: uint32_t = 0;
    let mut _518: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _519: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _520: uint32_t = 0;
    let mut _521: uint32_t = 0;
    let mut _522: uint32_t = 0;
    let mut _523: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _524: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _525: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _526: uint32_t = 0;
    let mut _527: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _528: uint32_t = 0;
    let mut _529: uint8_t = 0;
    let mut _530: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _531: uint32_t = 0;
    let mut _532: uint8_t = 0;
    let mut _533: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _534: uint32_t = 0;
    let mut _535: uint32_t = 0;
    let mut _536: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _537: uint8_t = 0;
    let mut _538: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _539: uint32_t = 0;
    let mut _540: uint32_t = 0;
    let mut _541: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _542: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _543: uint32_t = 0;
    let mut _544: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _545: uint32_t = 0;
    let mut _546: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _547: uint32_t = 0;
    let mut _548: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _549: uint32_t = 0;
    let mut _550: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _551: uint32_t = 0;
    let mut _552: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _553: uint32_t = 0;
    let mut _554: uint32_t = 0;
    let mut _555: uint32_t = 0;
    let mut _556: uint32_t = 0;
    let mut _557: uint32_t = 0;
    let mut _558: bool_0 = 0;
    let mut _558__PHI_TEMPORARY: bool_0 = 0;
    let mut _559: uint32_t = 0;
    let mut _560: uint32_t = 0;
    let mut _561: uint32_t = 0;
    let mut _562: uint32_t = 0;
    let mut _563: uint32_t = 0;
    let mut _564: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _565: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _566: uint32_t = 0;
    let mut _567: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _568: uint32_t = 0;
    let mut _569: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _570: uint32_t = 0;
    let mut _571: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _572: uint32_t = 0;
    let mut _573: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _574: uint32_t = 0;
    let mut _575: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _576: uint32_t = 0;
    let mut _577: uint32_t = 0;
    let mut _578: uint32_t = 0;
    let mut _579: uint32_t = 0;
    let mut _580: uint32_t = 0;
    let mut _581: uint32_t = 0;
    let mut _582: uint32_t = 0;
    let mut _583: bool_0 = 0;
    let mut _583__PHI_TEMPORARY: bool_0 = 0;
    let mut _584: uint32_t = 0;
    let mut _585: uint32_t = 0;
    let mut _586: uint32_t = 0;
    let mut _587: uint32_t = 0;
    let mut _588: uint32_t = 0;
    let mut _589: uint32_t = 0;
    let mut _590: uint32_t = 0;
    let mut _591: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _592: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _593: uint32_t = 0;
    let mut _594: uint16_t = 0;
    let mut _595: uint32_t = 0;
    let mut _596: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _597: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _598: uint32_t = 0;
    let mut _599: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _600: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _601: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _602: uint8_t = 0;
    let mut _603: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _604: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _605: uint8_t = 0;
    let mut _606: uint32_t = 0;
    let mut _607: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _608: uint8_t = 0;
    let mut _609: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _610: uint8_t = 0;
    let mut _611: uint32_t = 0;
    let mut _612: uint32_t = 0;
    let mut _613: uint32_t = 0;
    let mut _614: uint32_t = 0;
    let mut _615: uint32_t = 0;
    let mut _615__PHI_TEMPORARY: uint32_t = 0;
    let mut _616: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _617: uint32_t = 0;
    let mut _618: uint32_t = 0;
    let mut _619: uint32_t = 0;
    let mut _620: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _621: uint32_t = 0;
    let mut _622: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _623: uint32_t = 0;
    let mut _624: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _625: uint32_t = 0;
    let mut _626: uint8_t = 0;
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
    let mut _640: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _641: uint32_t = 0;
    let mut _642: uint8_t = 0;
    let mut _643: uint8_t = 0;
    let mut _644: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _645: uint32_t = 0;
    let mut _646: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _647: uint32_t = 0;
    let mut _648: uint8_t = 0;
    let mut _649: uint8_t = 0;
    let mut _650: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _651: uint32_t = 0;
    let mut _652: uint32_t = 0;
    let mut _653: uint32_t = 0;
    let mut _654: uint32_t = 0;
    let mut _655: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _656: uint32_t = 0;
    let mut _657: uint8_t = 0;
    let mut _658: uint8_t = 0;
    let mut _659: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _660: uint32_t = 0;
    let mut _661: uint32_t = 0;
    let mut _662: uint32_t = 0;
    let mut _663: uint32_t = 0;
    let mut _664: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _665: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _666: uint32_t = 0;
    let mut _667: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _668: uint32_t = 0;
    let mut _669: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _670: uint32_t = 0;
    let mut _671: uint32_t = 0;
    let mut _672: uint32_t = 0;
    let mut _673: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _674: uint32_t = 0;
    let mut _675: uint32_t = 0;
    let mut _676: uint32_t = 0;
    let mut _677: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
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
    let mut _688: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
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
    let mut _703: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _704: uint32_t = 0;
    let mut _705: uint32_t = 0;
    let mut _706: uint32_t = 0;
    let mut _707: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _708: uint32_t = 0;
    let mut _709: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _710: uint32_t = 0;
    let mut _711: uint32_t = 0;
    let mut _712: uint32_t = 0;
    let mut _713: uint32_t = 0;
    let mut _714: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
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
    let mut _725: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _726: uint32_t = 0;
    let mut _727: uint32_t = 0;
    let mut _728: uint32_t = 0;
    let mut _729: uint32_t = 0;
    let mut _730: uint32_t = 0;
    let mut _731: uint32_t = 0;
    let mut _732: uint32_t = 0;
    let mut _733: uint32_t = 0;
    let mut _734: uint32_t = 0;
    let mut _735: uint8_t = 0;
    let mut _736: uint32_t = 0;
    let mut _737: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _738: uint32_t = 0;
    let mut _739: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _740: uint32_t = 0;
    let mut _741: uint32_t = 0;
    let mut _742: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _743: uint32_t = 0;
    let mut _744: uint64_t = 0;
    let mut _745: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _746: uint32_t = 0;
    let mut _747: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _748: uint32_t = 0;
    let mut _749: uint32_t = 0;
    let mut _750: uint32_t = 0;
    let mut _751: uint32_t = 0;
    let mut _752: uint32_t = 0;
    let mut _753: uint32_t = 0;
    let mut _754: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _755: uint32_t = 0;
    let mut _756: uint32_t = 0;
    let mut _757: uint8_t = 0;
    let mut _758: uint8_t = 0;
    let mut _759: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _760: uint32_t = 0;
    let mut _761: uint32_t = 0;
    let mut _762: uint32_t = 0;
    let mut _763: uint32_t = 0;
    let mut _764: uint32_t = 0;
    let mut _765: uint32_t = 0;
    let mut _766: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _767: uint32_t = 0;
    let mut _768: uint32_t = 0;
    let mut _769: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _770: uint32_t = 0;
    let mut _771: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    _464 = _461;
    _465 = _462;
    _466 = _463;
    _518 = _466;
    *(&mut (*(_518 as *mut l_struct_struct_OC_symbolic_compressed_block)).field0
        as *mut uint8_t) = 3 as core::ffi::c_int as uint8_t;
    _519 = _465;
    _520 = _ZL9read_bitsiiPKh(
        11,
        0,
        _519,
    );
    _468 = _520;
    _521 = _468;
    if _521 & 511 == 508 as core::ffi::c_uint {
        _522 = _468;
        if _522 & 512 != 0 as core::ffi::c_uint {
            _523 = _466;
            *(&mut (*(_523 as *mut l_struct_struct_OC_symbolic_compressed_block)).field0
                as *mut uint8_t) = 1 as core::ffi::c_int as uint8_t;
        } else {
            _524 = _466;
            *(&mut (*(_524 as *mut l_struct_struct_OC_symbolic_compressed_block)).field0
                as *mut uint8_t) = 2 as core::ffi::c_int as uint8_t;
        }
        _525 = _466;
        *(&mut (*(_525 as *mut l_struct_struct_OC_symbolic_compressed_block)).field1
            as *mut uint8_t) = 0 as core::ffi::c_int as uint8_t;
        _469 = 0;
        loop {
            _526 = _469;
            if !((_526 as int32_t) < 4 as core::ffi::c_uint as int32_t) {
                break;
            }
            _527 = _465;
            _528 = _469;
            _529 = *(&mut *(_527 as *mut uint8_t)
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
                            ) -> uint32_t)(2, _528),
                        8,
                    ) as int32_t as int64_t as isize,
                ) as *mut uint8_t);
            _530 = _465;
            _531 = _469;
            _532 = *(&mut *(_530 as *mut uint8_t)
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
                            ) -> uint32_t)(2, _531),
                        9,
                    ) as int32_t as int64_t as isize,
                ) as *mut uint8_t);
            _533 = _466;
            _534 = _469;
            *(&mut *((*(&mut (*(_533
                as *mut l_struct_struct_OC_symbolic_compressed_block))
                .field9 as *mut l_struct_union_OC_anon as *mut [uint32_t; 4]))
                .array)
                .as_mut_ptr()
                .offset(_534 as int32_t as int64_t as isize)
                as *mut uint32_t) = _529 as uint32_t
                | (_532 as uint32_t) << 8 as core::ffi::c_int;
            _535 = _469;
            _469 = llvm_add_u32(_535, 1);
        }
        _536 = _464;
        _537 = *(&mut (*(_536 as *mut l_struct_struct_OC_block_size_descriptor)).field2
            as *mut uint8_t);
        if _537 as uint32_t == 1 as core::ffi::c_uint {
            _538 = _465;
            _539 = _ZL9read_bitsiiPKh(
                2,
                10,
                _538,
            );
            _470 = _539;
            _540 = _470;
            if _540 != 3 as core::ffi::c_uint {
                _541 = _466;
                *(&mut (*(_541 as *mut l_struct_struct_OC_symbolic_compressed_block))
                    .field0 as *mut uint8_t) = 0 as core::ffi::c_int as uint8_t;
            } else {
                _542 = _465;
                _543 = _ZL9read_bitsiiPKh(
                    8,
                    12,
                    _542,
                );
                _544 = _465;
                _545 = _ZL9read_bitsiiPKh(
                    5,
                    20,
                    _544,
                );
                _471 = _543 | _545 << 8 as core::ffi::c_int;
                _546 = _465;
                _547 = _ZL9read_bitsiiPKh(
                    13,
                    25,
                    _546,
                );
                _472 = _547;
                _548 = _465;
                _549 = _ZL9read_bitsiiPKh(
                    8,
                    38,
                    _548,
                );
                _550 = _465;
                _551 = _ZL9read_bitsiiPKh(
                    5,
                    46,
                    _550,
                );
                _473 = _549 | _551 << 8 as core::ffi::c_int;
                _552 = _465;
                _553 = _ZL9read_bitsiiPKh(
                    13,
                    51,
                    _552,
                );
                _474 = _553;
                _554 = _471;
                if _554 == 8191 as core::ffi::c_uint {
                    _555 = _472;
                    if _555 == 8191 as core::ffi::c_uint {
                        _556 = _473;
                        if _556 == 8191 as core::ffi::c_uint {
                            _557 = _474;
                            _558__PHI_TEMPORARY = (_557 == 8191 as core::ffi::c_uint)
                                as core::ffi::c_int as bool_0;
                        } else {
                            _558__PHI_TEMPORARY = 0 as core::ffi::c_int as bool_0;
                        }
                    } else {
                        _558__PHI_TEMPORARY = 0 as core::ffi::c_int as bool_0;
                    }
                } else {
                    _558__PHI_TEMPORARY = 0 as core::ffi::c_int as bool_0;
                }
                _558 = _558__PHI_TEMPORARY;
                _475 = _558 as uint32_t;
                _559 = _471;
                _560 = _472;
                if _559 as int32_t >= _560 as int32_t {
                    current_block = 15994008299637650043;
                } else {
                    _561 = _473;
                    _562 = _474;
                    if _561 as int32_t >= _562 as int32_t {
                        current_block = 15994008299637650043;
                    } else {
                        current_block = 4539013066515762475;
                    }
                }
                match current_block {
                    4539013066515762475 => {}
                    _ => {
                        _563 = _475;
                        if !(_563 != 0 as core::ffi::c_uint) {
                            _564 = _466;
                            *(&mut (*(_564
                                as *mut l_struct_struct_OC_symbolic_compressed_block))
                                .field0 as *mut uint8_t) = 0 as core::ffi::c_int as uint8_t;
                        }
                    }
                }
            }
        } else {
            _565 = _465;
            _566 = _ZL9read_bitsiiPKh(
                9,
                10,
                _565,
            );
            _476 = _566;
            _567 = _465;
            _568 = _ZL9read_bitsiiPKh(
                9,
                19,
                _567,
            );
            _477 = _568;
            _569 = _465;
            _570 = _ZL9read_bitsiiPKh(
                9,
                28,
                _569,
            );
            _478 = _570;
            _571 = _465;
            _572 = _ZL9read_bitsiiPKh(
                9,
                37,
                _571,
            );
            _479 = _572;
            _573 = _465;
            _574 = _ZL9read_bitsiiPKh(
                9,
                46,
                _573,
            );
            _480 = _574;
            _575 = _465;
            _576 = _ZL9read_bitsiiPKh(
                9,
                55,
                _575,
            );
            _481 = _576;
            _577 = _476;
            if _577 == 511 as core::ffi::c_uint {
                _578 = _477;
                if _578 == 511 as core::ffi::c_uint {
                    _579 = _478;
                    if _579 == 511 as core::ffi::c_uint {
                        _580 = _479;
                        if _580 == 511 as core::ffi::c_uint {
                            _581 = _480;
                            if _581 == 511 as core::ffi::c_uint {
                                _582 = _481;
                                _583__PHI_TEMPORARY = (_582 == 511 as core::ffi::c_uint)
                                    as core::ffi::c_int as bool_0;
                            } else {
                                _583__PHI_TEMPORARY = 0 as core::ffi::c_int as bool_0;
                            }
                        } else {
                            _583__PHI_TEMPORARY = 0 as core::ffi::c_int as bool_0;
                        }
                    } else {
                        _583__PHI_TEMPORARY = 0 as core::ffi::c_int as bool_0;
                    }
                } else {
                    _583__PHI_TEMPORARY = 0 as core::ffi::c_int as bool_0;
                }
            } else {
                _583__PHI_TEMPORARY = 0 as core::ffi::c_int as bool_0;
            }
            _583 = _583__PHI_TEMPORARY;
            _482 = _583 as uint32_t;
            _584 = _476;
            _585 = _477;
            if _584 as int32_t >= _585 as int32_t {
                current_block = 9753530599808234885;
            } else {
                _586 = _478;
                _587 = _479;
                if _586 as int32_t >= _587 as int32_t {
                    current_block = 9753530599808234885;
                } else {
                    _588 = _480;
                    _589 = _481;
                    if _588 as int32_t >= _589 as int32_t {
                        current_block = 9753530599808234885;
                    } else {
                        current_block = 4539013066515762475;
                    }
                }
            }
            match current_block {
                4539013066515762475 => {}
                _ => {
                    _590 = _482;
                    if !(_590 != 0 as core::ffi::c_uint) {
                        _591 = _466;
                        *(&mut (*(_591
                            as *mut l_struct_struct_OC_symbolic_compressed_block))
                            .field0 as *mut uint8_t) = 0 as core::ffi::c_int as uint8_t;
                    }
                }
            }
        }
    } else {
        _592 = _464;
        _593 = _468;
        _594 = *(&mut *((*(&mut (*(_592
            as *mut l_struct_struct_OC_block_size_descriptor))
            .field15 as *mut [uint16_t; 2048]))
            .array)
            .as_mut_ptr()
            .offset(_593 as int32_t as int64_t as isize) as *mut uint16_t);
        _483 = _594 as uint32_t;
        _595 = _483;
        if _595 == 65535 as core::ffi::c_uint {
            _596 = _466;
            *(&mut (*(_596 as *mut l_struct_struct_OC_symbolic_compressed_block)).field0
                as *mut uint8_t) = 0 as core::ffi::c_int as uint8_t;
        } else {
            _597 = _464;
            _598 = _468;
            _599 = _ZNK21block_size_descriptor14get_block_modeEj(_597, _598);
            _484 = _599;
            _600 = _464;
            _601 = _484;
            _602 = *(&mut (*(_601 as *mut l_struct_struct_OC_block_mode)).field1
                as *mut uint8_t);
            _603 = _ZNK21block_size_descriptor19get_decimation_infoEj(
                _600,
                _602 as uint32_t,
            );
            _485 = _603;
            _604 = _485;
            _605 = *(&mut (*(_604 as *mut l_struct_struct_OC_decimation_info)).field2
                as *mut uint8_t);
            _486 = _605 as uint32_t;
            _606 = _486;
            if _606 as int32_t > 0 as core::ffi::c_uint as int32_t {
                _607 = _484;
                _608 = *(&mut (*(_607 as *mut l_struct_struct_OC_block_mode)).field2
                    as *mut uint8_t);
                _487 = _608 as uint32_t;
                _609 = _484;
                _610 = *(&mut (*(_609 as *mut l_struct_struct_OC_block_mode)).field4
                    as *mut uint8_t);
                _488 = llvm_and_u8(_610, 1 as core::ffi::c_int as uint8_t) as uint32_t;
                _611 = _488;
                if _611 != 0 as core::ffi::c_uint {
                    _612 = _486;
                    _613 = llvm_mul_u32(2, _612);
                    _615__PHI_TEMPORARY = _613;
                } else {
                    _614 = _486;
                    _615__PHI_TEMPORARY = _614;
                }
                _615 = _615__PHI_TEMPORARY;
                _489 = _615;
                _616 = _465;
                _617 = _ZL9read_bitsiiPKh(
                    2,
                    11,
                    _616,
                );
                _490 = llvm_add_u32(_617, 1);
                _618 = _490;
                if _618 as int32_t > 0 as core::ffi::c_uint as int32_t {
                    _619 = _468;
                    _620 = _466;
                    *(&mut (*(_620 as *mut l_struct_struct_OC_symbolic_compressed_block))
                        .field4 as *mut uint16_t) = _619 as uint16_t;
                    _621 = _490;
                    _622 = _466;
                    *(&mut (*(_622 as *mut l_struct_struct_OC_symbolic_compressed_block))
                        .field1 as *mut uint8_t) = _621 as uint8_t;
                    _491 = 0;
                    loop {
                        _623 = _491;
                        if !((_623 as int32_t) < 16 as core::ffi::c_uint as int32_t) {
                            break;
                        }
                        _624 = _465;
                        _625 = _491;
                        _626 = *(&mut *(_624 as *mut uint8_t)
                            .offset(
                                (llvm_sub_u32
                                    as unsafe extern "C" fn(
                                        uint32_t,
                                        uint32_t,
                                    ) -> uint32_t)(15, _625)
                                    as int32_t as int64_t as isize,
                            ) as *mut uint8_t);
                        _627 = _ZL7bitrev8i(_626 as uint32_t);
                        _628 = _491;
                        *(&mut *(_467.array)
                            .as_mut_ptr()
                            .offset(_628 as int32_t as int64_t as isize)
                            as *mut uint8_t) = _627 as uint8_t;
                        _629 = _491;
                        _491 = llvm_add_u32(_629, 1);
                    }
                    _630 = _489;
                    _631 = _487;
                    _632 = _Z25get_ise_sequence_bitcountj12quant_method(_630, _631);
                    _492 = _632;
                    _633 = _492;
                    _493 = llvm_sub_u32(128, _633);
                    _634 = _487;
                    _495 = &mut *(quant_and_xfer_tables.array)
                        .as_mut_ptr()
                        .offset(_634 as uint64_t as int64_t as isize)
                        as *mut l_struct_struct_OC_quant_and_transfer_table
                        as *mut core::ffi::c_void;
                    _635 = _487;
                    _636 = _489;
                    _Z10decode_ise12quant_methodjPKhPhj(
                        _635,
                        _636,
                        &mut *(_467.array)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as int64_t as isize) as *mut uint8_t
                            as *mut core::ffi::c_void,
                        &mut *(_494.array)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as int64_t as isize) as *mut uint8_t
                            as *mut core::ffi::c_void,
                        0,
                    );
                    _637 = _488;
                    if _637 != 0 as core::ffi::c_uint {
                        _496 = 0;
                        loop {
                            _638 = _496;
                            _639 = _486;
                            if !((_638 as int32_t) < _639 as int32_t) {
                                break;
                            }
                            _640 = _495;
                            _641 = _496;
                            _642 = *(&mut *(_494.array)
                                .as_mut_ptr()
                                .offset(
                                    (llvm_mul_u32
                                        as unsafe extern "C" fn(
                                            uint32_t,
                                            uint32_t,
                                        ) -> uint32_t)(2, _641)
                                        as int32_t as int64_t as isize,
                                ) as *mut uint8_t);
                            _643 = *(&mut *((*(&mut (*(_640
                                as *mut l_struct_struct_OC_quant_and_transfer_table))
                                .field2 as *mut [uint8_t; 32]))
                                .array)
                                .as_mut_ptr()
                                .offset(_642 as uint64_t as int64_t as isize)
                                as *mut uint8_t);
                            _644 = _466;
                            _645 = _496;
                            *(&mut *((*(&mut (*(_644
                                as *mut l_struct_struct_OC_symbolic_compressed_block))
                                .field10 as *mut [uint8_t; 64]))
                                .array)
                                .as_mut_ptr()
                                .offset(_645 as int32_t as int64_t as isize)
                                as *mut uint8_t) = _643;
                            _646 = _495;
                            _647 = _496;
                            _648 = *(&mut *(_494.array)
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
                                            ) -> uint32_t)(2, _647),
                                        1,
                                    ) as int32_t as int64_t as isize,
                                ) as *mut uint8_t);
                            _649 = *(&mut *((*(&mut (*(_646
                                as *mut l_struct_struct_OC_quant_and_transfer_table))
                                .field2 as *mut [uint8_t; 32]))
                                .array)
                                .as_mut_ptr()
                                .offset(_648 as uint64_t as int64_t as isize)
                                as *mut uint8_t);
                            _650 = _466;
                            _651 = _496;
                            *(&mut *((*(&mut (*(_650
                                as *mut l_struct_struct_OC_symbolic_compressed_block))
                                .field10 as *mut [uint8_t; 64]))
                                .array)
                                .as_mut_ptr()
                                .offset(
                                    (llvm_add_u32
                                        as unsafe extern "C" fn(
                                            uint32_t,
                                            uint32_t,
                                        ) -> uint32_t)(_651, 32)
                                        as uint64_t as int64_t as isize,
                                ) as *mut uint8_t) = _649;
                            _652 = _496;
                            _496 = llvm_add_u32(_652, 1);
                        }
                    } else {
                        _497 = 0;
                        loop {
                            _653 = _497;
                            _654 = _486;
                            if !((_653 as int32_t) < _654 as int32_t) {
                                break;
                            }
                            _655 = _495;
                            _656 = _497;
                            _657 = *(&mut *(_494.array)
                                .as_mut_ptr()
                                .offset(_656 as int32_t as int64_t as isize)
                                as *mut uint8_t);
                            _658 = *(&mut *((*(&mut (*(_655
                                as *mut l_struct_struct_OC_quant_and_transfer_table))
                                .field2 as *mut [uint8_t; 32]))
                                .array)
                                .as_mut_ptr()
                                .offset(_657 as uint64_t as int64_t as isize)
                                as *mut uint8_t);
                            _659 = _466;
                            _660 = _497;
                            *(&mut *((*(&mut (*(_659
                                as *mut l_struct_struct_OC_symbolic_compressed_block))
                                .field10 as *mut [uint8_t; 64]))
                                .array)
                                .as_mut_ptr()
                                .offset(_660 as int32_t as int64_t as isize)
                                as *mut uint8_t) = _658;
                            _661 = _497;
                            _497 = llvm_add_u32(_661, 1);
                        }
                    }
                    _662 = _488;
                    if _662 != 0 as core::ffi::c_uint {
                        _663 = _490;
                        if _663 == 4 as core::ffi::c_uint {
                            _664 = _466;
                            *(&mut (*(_664
                                as *mut l_struct_struct_OC_symbolic_compressed_block))
                                .field0 as *mut uint8_t) = 0 as core::ffi::c_int as uint8_t;
                            current_block = 4539013066515762475;
                        } else {
                            current_block = 4314136905994515865;
                        }
                    } else {
                        current_block = 4314136905994515865;
                    }
                    match current_block {
                        4539013066515762475 => {}
                        _ => {
                            _665 = _466;
                            *(&mut (*(_665
                                as *mut l_struct_struct_OC_symbolic_compressed_block))
                                .field2 as *mut uint8_t) = 0 as core::ffi::c_int as uint8_t;
                            _499 = 0;
                            _666 = _490;
                            if _666 == 1 as core::ffi::c_uint {
                                _667 = _465;
                                _668 = _ZL9read_bitsiiPKh(
                                    4,
                                    13,
                                    _667,
                                );
                                *(&mut *(_498.array)
                                    .as_mut_ptr()
                                    .offset(0 as core::ffi::c_int as int64_t as isize)
                                    as *mut uint32_t) = _668;
                                _669 = _466;
                                *(&mut (*(_669
                                    as *mut l_struct_struct_OC_symbolic_compressed_block))
                                    .field5 as *mut uint16_t) = 0 as core::ffi::c_int as uint16_t;
                            } else {
                                _670 = _490;
                                _499 = llvm_sub_u32(
                                    llvm_mul_u32(3, _670),
                                    4,
                                );
                                _671 = _499;
                                _672 = _493;
                                _493 = llvm_sub_u32(_672, _671);
                                _673 = _465;
                                _674 = _ZL9read_bitsiiPKh(
                                    6,
                                    23,
                                    _673,
                                );
                                _675 = _499;
                                _676 = _493;
                                _677 = _465;
                                _678 = _ZL9read_bitsiiPKh(_675, _676, _677);
                                _500 = _674 | _678 << 6 as core::ffi::c_int;
                                _679 = _500;
                                _501 = _679 & 3;
                                _680 = _501;
                                if _680 == 0 as core::ffi::c_uint {
                                    _502 = 0;
                                    loop {
                                        _681 = _502;
                                        _682 = _490;
                                        if !((_681 as int32_t) < _682 as int32_t) {
                                            break;
                                        }
                                        _683 = _500;
                                        _684 = _502;
                                        *(&mut *(_498.array)
                                            .as_mut_ptr()
                                            .offset(_684 as int32_t as int64_t as isize)
                                            as *mut uint32_t) = llvm_ashr_u32(
                                            _683 as int32_t,
                                            2 as core::ffi::c_int,
                                        ) & 15;
                                        _685 = _502;
                                        _502 = llvm_add_u32(_685, 1);
                                    }
                                    _686 = _499;
                                    _687 = _493;
                                    _493 = llvm_add_u32(_687, _686);
                                    _688 = _466;
                                    *(&mut (*(_688
                                        as *mut l_struct_struct_OC_symbolic_compressed_block))
                                        .field2 as *mut uint8_t) = 1 as core::ffi::c_int as uint8_t;
                                    _499 = 0;
                                } else {
                                    _503 = 2;
                                    _689 = _501;
                                    _501 = llvm_add_u32(_689, -(1 as core::ffi::c_int) as uint32_t);
                                    _504 = 0;
                                    loop {
                                        _690 = _504;
                                        _691 = _490;
                                        if !((_690 as int32_t) < _691 as int32_t) {
                                            break;
                                        }
                                        _692 = _500;
                                        _693 = _503;
                                        _694 = _501;
                                        _695 = _504;
                                        *(&mut *(_498.array)
                                            .as_mut_ptr()
                                            .offset(_695 as int32_t as int64_t as isize)
                                            as *mut uint32_t) = llvm_add_u32(
                                            llvm_ashr_u32(_692 as int32_t, _693 as int32_t)
                                                & 1,
                                            _694,
                                        ) << 2 as core::ffi::c_int;
                                        _696 = _503;
                                        _503 = llvm_add_u32(_696, 1);
                                        _697 = _504;
                                        _504 = llvm_add_u32(_697, 1);
                                    }
                                    _505 = 0;
                                    loop {
                                        _698 = _505;
                                        _699 = _490;
                                        if !((_698 as int32_t) < _699 as int32_t) {
                                            break;
                                        }
                                        _700 = _500;
                                        _701 = _503;
                                        _702 = _505;
                                        _703 = &mut *(_498.array)
                                            .as_mut_ptr()
                                            .offset(_702 as int32_t as int64_t as isize)
                                            as *mut uint32_t as *mut core::ffi::c_void;
                                        _704 = *(_703 as *mut uint32_t);
                                        *(_703
                                            as *mut uint32_t) = _704
                                            | llvm_ashr_u32(_700 as int32_t, _701 as int32_t)
                                                & 3;
                                        _705 = _503;
                                        _503 = llvm_add_u32(_705, 2);
                                        _706 = _505;
                                        _505 = llvm_add_u32(_706, 1);
                                    }
                                }
                                _707 = _465;
                                _708 = _ZL9read_bitsiiPKh(
                                    10,
                                    13,
                                    _707,
                                );
                                _709 = _466;
                                *(&mut (*(_709
                                    as *mut l_struct_struct_OC_symbolic_compressed_block))
                                    .field5 as *mut uint16_t) = _708 as uint16_t;
                            }
                            _506 = 0;
                            loop {
                                _710 = _506;
                                _711 = _490;
                                if !((_710 as int32_t) < _711 as int32_t) {
                                    break;
                                }
                                _712 = _506;
                                _713 = *(&mut *(_498.array)
                                    .as_mut_ptr()
                                    .offset(_712 as int32_t as int64_t as isize)
                                    as *mut uint32_t);
                                _714 = _466;
                                _715 = _506;
                                *(&mut *((*(&mut (*(_714
                                    as *mut l_struct_struct_OC_symbolic_compressed_block))
                                    .field6 as *mut [uint8_t; 4]))
                                    .array)
                                    .as_mut_ptr()
                                    .offset(_715 as int32_t as int64_t as isize)
                                    as *mut uint8_t) = _713 as uint8_t;
                                _716 = _506;
                                _506 = llvm_add_u32(_716, 1);
                            }
                            _507 = 0;
                            _508 = 0;
                            loop {
                                _717 = _508;
                                _718 = _490;
                                if !((_717 as int32_t) < _718 as int32_t) {
                                    break;
                                }
                                _719 = _508;
                                _720 = *(&mut *(_498.array)
                                    .as_mut_ptr()
                                    .offset(_719 as int32_t as int64_t as isize)
                                    as *mut uint32_t);
                                _509 = llvm_ashr_u32(_720 as int32_t, 2 as core::ffi::c_int);
                                _721 = _509;
                                _722 = _507;
                                _507 = llvm_add_u32(
                                    _722,
                                    llvm_mul_u32(
                                        llvm_add_u32(_721, 1),
                                        2,
                                    ),
                                );
                                _723 = _508;
                                _508 = llvm_add_u32(_723, 1);
                            }
                            _724 = _507;
                            if _724 as int32_t > 18 as core::ffi::c_uint as int32_t {
                                _725 = _466;
                                *(&mut (*(_725
                                    as *mut l_struct_struct_OC_symbolic_compressed_block))
                                    .field0 as *mut uint8_t) = 0 as core::ffi::c_int as uint8_t;
                            } else {
                                _726 = _490;
                                _727 = *(&*(_ZZ20physical_to_symbolicRK21block_size_descriptorPKhR25symbolic_compressed_blockE14color_bits_arr
                                    .array)
                                    .as_ptr()
                                    .offset(_726 as int32_t as int64_t as isize)
                                    as *const uint32_t as *mut uint32_t);
                                _728 = _492;
                                _729 = _499;
                                _510 = llvm_sub_u32(llvm_sub_u32(_727, _728), _729);
                                _730 = _488;
                                if _730 != 0 as core::ffi::c_uint {
                                    _731 = _510;
                                    _510 = llvm_sub_u32(_731, 2);
                                }
                                _732 = _510;
                                if (_732 as int32_t) < 0 as core::ffi::c_uint as int32_t {
                                    _510 = 0;
                                }
                                _733 = _507;
                                _734 = _510;
                                _735 = *(&mut *((*(&*(quant_mode_table.array)
                                    .as_ptr()
                                    .offset(
                                        (llvm_ashr_u32
                                            as unsafe extern "C" fn(
                                                int32_t,
                                                int32_t,
                                            ) -> uint32_t)(_733 as int32_t, 1 as core::ffi::c_int) as int32_t
                                            as int64_t as isize,
                                    ) as *const [uint8_t; 128]
                                    as *mut [uint8_t; 128]))
                                    .array)
                                    .as_mut_ptr()
                                    .offset(_734 as int32_t as int64_t as isize)
                                    as *mut uint8_t);
                                _511 = _735 as int8_t as int32_t as uint32_t;
                                _736 = _511;
                                if (_736 as int32_t) < 4 as core::ffi::c_uint as int32_t {
                                    _737 = _466;
                                    *(&mut (*(_737
                                        as *mut l_struct_struct_OC_symbolic_compressed_block))
                                        .field0 as *mut uint8_t) = 0 as core::ffi::c_int as uint8_t;
                                } else {
                                    _738 = _511;
                                    _739 = _466;
                                    *(&mut (*(_739
                                        as *mut l_struct_struct_OC_symbolic_compressed_block))
                                        .field7 as *mut uint32_t) = _738;
                                    _740 = _511;
                                    _741 = _507;
                                    _742 = _465;
                                    _743 = _490;
                                    _744 = (_743 == 1 as core::ffi::c_uint) as core::ffi::c_int as bool_0
                                        as uint64_t;
                                    _Z10decode_ise12quant_methodjPKhPhj(
                                        _740,
                                        _741,
                                        _742,
                                        &mut *(_512.array)
                                            .as_mut_ptr()
                                            .offset(0 as core::ffi::c_int as int64_t as isize)
                                            as *mut uint8_t as *mut core::ffi::c_void,
                                        llvm_select_u32(
                                            (_743 == 1 as core::ffi::c_uint) as core::ffi::c_int as bool_0,
                                            17,
                                            29,
                                        ),
                                    );
                                    _513 = 0;
                                    _745 = _466;
                                    _746 = *(&mut (*(_745
                                        as *mut l_struct_struct_OC_symbolic_compressed_block))
                                        .field7 as *mut uint32_t);
                                    _747 = *(&mut *(color_scrambled_pquant_to_uquant_tables
                                        .array)
                                        .as_mut_ptr()
                                        .offset(
                                            (llvm_sub_u32
                                                as unsafe extern "C" fn(
                                                    uint32_t,
                                                    uint32_t,
                                                ) -> uint32_t)(_746, 4)
                                                as int32_t as int64_t as isize,
                                        ) as *mut *mut core::ffi::c_void);
                                    _514 = _747;
                                    _515 = 0;
                                    loop {
                                        _748 = _515;
                                        _749 = _490;
                                        if !((_748 as int32_t) < _749 as int32_t) {
                                            break;
                                        }
                                        _750 = _515;
                                        _751 = *(&mut *(_498.array)
                                            .as_mut_ptr()
                                            .offset(_750 as int32_t as int64_t as isize)
                                            as *mut uint32_t);
                                        _516 = llvm_add_u32(
                                            llvm_mul_u32(
                                                2,
                                                llvm_ashr_u32(_751 as int32_t, 2 as core::ffi::c_int),
                                            ),
                                            2,
                                        );
                                        _517 = 0;
                                        loop {
                                            _752 = _517;
                                            _753 = _516;
                                            if !((_752 as int32_t) < _753 as int32_t) {
                                                break;
                                            }
                                            _754 = _514;
                                            _755 = _517;
                                            _756 = _513;
                                            _757 = *(&mut *(_512.array)
                                                .as_mut_ptr()
                                                .offset(
                                                    (llvm_add_u32
                                                        as unsafe extern "C" fn(
                                                            uint32_t,
                                                            uint32_t,
                                                        ) -> uint32_t)(_755, _756) as int32_t as int64_t as isize,
                                                ) as *mut uint8_t);
                                            _758 = *(&mut *(_754 as *mut uint8_t)
                                                .offset(_757 as uint64_t as int64_t as isize)
                                                as *mut uint8_t);
                                            _759 = _466;
                                            _760 = _515;
                                            _761 = _517;
                                            *(&mut *((*(&mut *((*(&mut (*(_759
                                                as *mut l_struct_struct_OC_symbolic_compressed_block))
                                                .field9 as *mut l_struct_union_OC_anon
                                                as *mut [struct_AC_l_array_8_uint8_t; 4]))
                                                .array)
                                                .as_mut_ptr()
                                                .offset(_760 as int32_t as int64_t as isize)
                                                as *mut [uint8_t; 8]))
                                                .array)
                                                .as_mut_ptr()
                                                .offset(_761 as int32_t as int64_t as isize)
                                                as *mut uint8_t) = _758;
                                            _762 = _517;
                                            _517 = llvm_add_u32(_762, 1);
                                        }
                                        _763 = _516;
                                        _764 = _513;
                                        _513 = llvm_add_u32(_764, _763);
                                        _765 = _515;
                                        _515 = llvm_add_u32(_765, 1);
                                    }
                                    _766 = _466;
                                    *(&mut (*(_766
                                        as *mut l_struct_struct_OC_symbolic_compressed_block))
                                        .field3 as *mut uint8_t) = -(1 as core::ffi::c_int) as uint8_t;
                                    _767 = _488;
                                    if _767 != 0 as core::ffi::c_uint {
                                        _768 = _493;
                                        _769 = _465;
                                        _770 = _ZL9read_bitsiiPKh(
                                            2,
                                            llvm_sub_u32(_768, 2),
                                            _769,
                                        );
                                        _771 = _466;
                                        *(&mut (*(_771
                                            as *mut l_struct_struct_OC_symbolic_compressed_block))
                                            .field3 as *mut uint8_t) = _770 as uint8_t;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    __assert_fail(
                        &_OC_str_OC_4 as *const [uint8_t; 20] as *mut core::ffi::c_void,
                        &_OC_str_OC_1 as *const [uint8_t; 56] as *mut core::ffi::c_void,
                        391,
                        &__PRETTY_FUNCTION___OC__Z20physical_to_symbolicRK21block_size_descriptorPKhR25symbolic_compressed_block
                            as *const [uint8_t; 103] as *mut core::ffi::c_void,
                    );
                }
            } else {
                __assert_fail(
                    &_OC_str_OC_3 as *const [uint8_t; 17] as *mut core::ffi::c_void,
                    &_OC_str_OC_1 as *const [uint8_t; 56] as *mut core::ffi::c_void,
                    383,
                    &__PRETTY_FUNCTION___OC__Z20physical_to_symbolicRK21block_size_descriptorPKhR25symbolic_compressed_block
                        as *const [uint8_t; 103] as *mut core::ffi::c_void,
                );
            }
        }
    };
}
#[inline(never)]
unsafe extern "C" fn _ZL9read_bitsiiPKh(
    mut _878: uint32_t,
    mut _879: uint32_t,
    mut _880: *mut core::ffi::c_void,
) -> uint32_t {
    let mut _881: uint32_t = 0;
    let mut _882: uint32_t = 0;
    let mut _883: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _884: uint32_t = 0;
    let mut _885: uint32_t = 0;
    let mut _886: uint32_t = 0;
    let mut _887: uint32_t = 0;
    let mut _888: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _889: uint32_t = 0;
    let mut _890: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _891: uint8_t = 0;
    let mut _892: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _893: uint8_t = 0;
    let mut _894: uint32_t = 0;
    let mut _895: uint32_t = 0;
    let mut _896: uint32_t = 0;
    let mut _897: uint32_t = 0;
    let mut _898: uint32_t = 0;
    _881 = _878;
    _882 = _879;
    _883 = _880;
    _886 = _881;
    _884 = llvm_sub_u32(
        ((1 as core::ffi::c_int) << _886) as uint32_t,
        1,
    );
    _887 = _882;
    _888 = _883;
    _883 = &mut *(_888 as *mut uint8_t)
        .offset(
            (llvm_ashr_u32
                as unsafe extern "C" fn(
                    int32_t,
                    int32_t,
                ) -> uint32_t)(_887 as int32_t, 3 as core::ffi::c_int) as int32_t as int64_t
                as isize,
        ) as *mut uint8_t as *mut core::ffi::c_void;
    _889 = _882;
    _882 = _889 & 7;
    _890 = _883;
    _891 = *(_890 as *mut uint8_t);
    _892 = _883;
    _893 = *(&mut *(_892 as *mut uint8_t).offset(1 as core::ffi::c_int as int64_t as isize)
        as *mut uint8_t);
    _885 = _891 as uint32_t | (_893 as uint32_t) << 8 as core::ffi::c_int;
    _894 = _882;
    _895 = _885;
    _885 = llvm_ashr_u32(_895 as int32_t, _894 as int32_t);
    _896 = _884;
    _897 = _885;
    _885 = _897 & _896;
    _898 = _885;
    return _898;
}
