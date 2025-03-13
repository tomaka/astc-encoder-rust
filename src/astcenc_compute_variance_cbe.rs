use ::libc;
extern "C" {
    fn __assert_fail(
        _1264: *mut libc::c_void,
        _1265: *mut libc::c_void,
        _1266: uint32_t,
        _1267: *mut libc::c_void,
    ) -> !;
    fn _Z13sf16_to_floatt(_1268: uint16_t) -> libc::c_float;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type bool_0 = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_4_float {
    pub array: [libc::c_float; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_struct_struct_OC_vfloat4 {
    pub field0: l_array_4_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_4_uint32_t {
    pub array: [uint32_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_struct_struct_OC_vint4 {
    pub field0: l_array_4_uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_struct_struct_OC_astcenc_swizzle {
    pub field0: uint32_t,
    pub field1: uint32_t,
    pub field2: uint32_t,
    pub field3: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_struct_struct_OC_pixel_region_args {
    pub field0: *mut libc::c_void,
    pub field1: l_struct_struct_OC_astcenc_swizzle,
    pub field2: uint8_t,
    pub field3: uint32_t,
    pub field4: uint32_t,
    pub field5: uint32_t,
    pub field6: uint32_t,
    pub field7: uint32_t,
    pub field8: uint32_t,
    pub field9: uint32_t,
    pub field10: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_struct_struct_OC_astcenc_config {
    pub field0: uint32_t,
    pub field1: uint32_t,
    pub field2: uint32_t,
    pub field3: uint32_t,
    pub field4: uint32_t,
    pub field5: libc::c_float,
    pub field6: libc::c_float,
    pub field7: libc::c_float,
    pub field8: libc::c_float,
    pub field9: uint32_t,
    pub field10: libc::c_float,
    pub field11: uint32_t,
    pub field12: uint32_t,
    pub field13: uint32_t,
    pub field14: uint32_t,
    pub field15: uint32_t,
    pub field16: uint32_t,
    pub field17: uint32_t,
    pub field18: uint32_t,
    pub field19: uint32_t,
    pub field20: uint32_t,
    pub field21: libc::c_float,
    pub field22: libc::c_float,
    pub field23: libc::c_float,
    pub field24: libc::c_float,
    pub field25: libc::c_float,
    pub field26: libc::c_float,
    pub field27: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_struct_struct_OC_avg_args {
    pub field0: l_struct_struct_OC_pixel_region_args,
    pub field1: uint32_t,
    pub field2: uint32_t,
    pub field3: uint32_t,
    pub field4: uint32_t,
    pub field5: uint32_t,
    pub field6: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_struct_struct_OC_astcenc_contexti {
    pub field0: l_struct_struct_OC_astcenc_config,
    pub field1: uint32_t,
    pub field2: *mut libc::c_void,
    pub field3: *mut libc::c_void,
    pub field4: *mut libc::c_void,
    pub field5: l_struct_struct_OC_avg_args,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_struct_struct_OC_astcenc_image {
    pub field0: uint32_t,
    pub field1: uint32_t,
    pub field2: uint32_t,
    pub field3: uint32_t,
    pub field4: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_35_uint8_t {
    pub array: [uint8_t; 35],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_55_uint8_t {
    pub array: [uint8_t; 55],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_82_uint8_t {
    pub array: [uint8_t; 82],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_6_uint8_t {
    pub array: [uint8_t; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_6_uint16_t {
    pub array: [uint16_t; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_6_float {
    pub array: [libc::c_float; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_2_uint64_t {
    pub array: [uint64_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub data: l_array_2_uint64_t,
}
static mut _OC_str: l_array_35_uint8_t = unsafe {
    {
        let mut init = l_array_35_uint8_t {
            array: *::core::mem::transmute::<&[u8; 35], &mut [uint8_t; 35]>(
                b"img->data_type == ASTCENC_TYPE_F32\0",
            ),
        };
        init
    }
};
static mut _OC_str_OC_1: l_array_55_uint8_t = unsafe {
    {
        let mut init = l_array_55_uint8_t {
            array: *::core::mem::transmute::<&[u8; 55], &mut [uint8_t; 55]>(
                b"/root/astc-encoder/Source/astcenc_compute_variance.cpp\0",
            ),
        };
        init
    }
};
static mut __PRETTY_FUNCTION___OC__Z29compute_pixel_region_varianceR16astcenc_contextiRK17pixel_region_args: l_array_82_uint8_t = unsafe {
    {
        let mut init = l_array_82_uint8_t {
            array: *::core::mem::transmute::<
                &[u8; 82],
                &mut [uint8_t; 82],
            >(
                b"void compute_pixel_region_variance(astcenc_contexti &, const pixel_region_args &)\0",
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
    r = if condition as libc::c_int != 0 {
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
unsafe extern "C" fn llvm_add_u64(mut a: uint64_t, mut b: uint64_t) -> uint64_t {
    let mut r: uint64_t = a.wrapping_add(b);
    return r;
}
#[inline(always)]
unsafe extern "C" fn llvm_fadd_f32(mut a: libc::c_float, mut b: libc::c_float) -> libc::c_float {
    let mut r: libc::c_float = a + b;
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
unsafe extern "C" fn llvm_fsub_f32(mut a: libc::c_float, mut b: libc::c_float) -> libc::c_float {
    let mut r: libc::c_float = a - b;
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
unsafe extern "C" fn llvm_fmul_f32(mut a: libc::c_float, mut b: libc::c_float) -> libc::c_float {
    let mut r: libc::c_float = a * b;
    return r;
}
#[inline(always)]
unsafe extern "C" fn llvm_udiv_u32(mut a: uint32_t, mut b: uint32_t) -> uint32_t {
    let mut r: uint32_t = a / b;
    return r;
}
#[inline(always)]
unsafe extern "C" fn llvm_fdiv_f32(mut a: libc::c_float, mut b: libc::c_float) -> libc::c_float {
    let mut r: libc::c_float = a / b;
    return r;
}
#[inline(always)]
unsafe extern "C" fn llvm_lshr_u64(mut a: uint64_t, mut b: uint64_t) -> uint64_t {
    let mut r: uint64_t = a >> b;
    return r;
}
#[inline(always)]
unsafe extern "C" fn llvm_neg_u64(mut a: int64_t) -> uint64_t {
    let mut r: uint64_t = -a as uint64_t;
    return r;
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn _Z29compute_pixel_region_varianceR16astcenc_contextiRK17pixel_region_args(
    mut _1: *mut libc::c_void,
    mut _2: *mut libc::c_void,
) {
    let mut _3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _4: libc::c_float = 0.;
    let mut _5: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _6: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _7: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _8: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _9: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _10: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _11: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _12: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _13: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _14: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _15: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _16: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _17: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _18: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _19: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _20: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _21: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _22: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _23: libc::c_float = 0.;
    let mut _24: libc::c_float = 0.;
    let mut _25: libc::c_float = 0.;
    let mut _26: libc::c_float = 0.;
    let mut _27: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _28: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _29: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _30: uint32_t = 0;
    let mut _31: uint32_t = 0;
    let mut _32: uint32_t = 0;
    let mut _33: uint32_t = 0;
    let mut _34: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _35: libc::c_float = 0.;
    let mut _36: libc::c_float = 0.;
    let mut _37: libc::c_float = 0.;
    let mut _38: libc::c_float = 0.;
    let mut _39: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _40: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _41: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _42: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _43: libc::c_float = 0.;
    let mut _44: libc::c_float = 0.;
    let mut _45: libc::c_float = 0.;
    let mut _46: libc::c_float = 0.;
    let mut _47: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _48: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _49: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _50: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _51: libc::c_float = 0.;
    let mut _52: libc::c_float = 0.;
    let mut _53: libc::c_float = 0.;
    let mut _54: libc::c_float = 0.;
    let mut _55: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _56: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _57: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _58: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _59: libc::c_float = 0.;
    let mut _60: libc::c_float = 0.;
    let mut _61: libc::c_float = 0.;
    let mut _62: libc::c_float = 0.;
    let mut _63: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _64: libc::c_float = 0.;
    let mut _65: libc::c_float = 0.;
    let mut _66: libc::c_float = 0.;
    let mut _67: libc::c_float = 0.;
    let mut _68: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _69: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _70: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _71: l_struct_struct_OC_astcenc_swizzle = l_struct_struct_OC_astcenc_swizzle {
        field0: 0,
        field1: 0,
        field2: 0,
        field3: 0,
    };
    let mut _72: uint8_t = 0;
    let mut _73: uint32_t = 0;
    let mut _74: uint32_t = 0;
    let mut _75: uint32_t = 0;
    let mut _76: uint32_t = 0;
    let mut _77: uint32_t = 0;
    let mut _78: uint32_t = 0;
    let mut _79: uint32_t = 0;
    let mut _80: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _81: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _82: uint32_t = 0;
    let mut _83: uint32_t = 0;
    let mut _84: uint32_t = 0;
    let mut _85: uint32_t = 0;
    let mut _86: uint32_t = 0;
    let mut _87: uint32_t = 0;
    let mut _88: uint32_t = 0;
    let mut _89: uint32_t = 0;
    let mut _90: uint32_t = 0;
    let mut _91: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _92: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _93: uint32_t = 0;
    let mut _94: uint32_t = 0;
    let mut _95: uint32_t = 0;
    let mut _96: uint32_t = 0;
    let mut _97: l_array_6_uint8_t = l_array_6_uint8_t { array: [0; 6] };
    let mut _98: uint32_t = 0;
    let mut _99: uint32_t = 0;
    let mut _100: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _101: uint32_t = 0;
    let mut _102: uint32_t = 0;
    let mut _103: uint32_t = 0;
    let mut _104: uint32_t = 0;
    let mut _105: uint8_t = 0;
    let mut _106: uint8_t = 0;
    let mut _107: uint8_t = 0;
    let mut _108: uint8_t = 0;
    let mut _109: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _110: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _111: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _112: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _113: l_array_6_uint16_t = l_array_6_uint16_t { array: [0; 6] };
    let mut _114: uint32_t = 0;
    let mut _115: uint32_t = 0;
    let mut _116: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _117: uint32_t = 0;
    let mut _118: uint32_t = 0;
    let mut _119: uint32_t = 0;
    let mut _120: uint32_t = 0;
    let mut _121: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _122: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _123: l_struct_struct_OC_vint4 = l_struct_struct_OC_vint4 {
        field0: l_array_4_uint32_t { array: [0; 4] },
    };
    let mut _124: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _125: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _126: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _127: l_array_6_float = l_array_6_float { array: [0.; 6] };
    let mut _128: uint32_t = 0;
    let mut _129: uint32_t = 0;
    let mut _130: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _131: uint32_t = 0;
    let mut _132: uint32_t = 0;
    let mut _133: uint32_t = 0;
    let mut _134: uint32_t = 0;
    let mut _135: libc::c_float = 0.;
    let mut _136: libc::c_float = 0.;
    let mut _137: libc::c_float = 0.;
    let mut _138: libc::c_float = 0.;
    let mut _139: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _140: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _141: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _142: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _143: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _144: uint32_t = 0;
    let mut _145: uint32_t = 0;
    let mut _146: uint32_t = 0;
    let mut _147: uint32_t = 0;
    let mut _148: uint32_t = 0;
    let mut _149: uint32_t = 0;
    let mut _150: uint32_t = 0;
    let mut _151: uint32_t = 0;
    let mut _152: uint32_t = 0;
    let mut _153: uint32_t = 0;
    let mut _154: uint32_t = 0;
    let mut _155: libc::c_float = 0.;
    let mut _156: libc::c_float = 0.;
    let mut _157: uint32_t = 0;
    let mut _158: uint32_t = 0;
    let mut _159: uint32_t = 0;
    let mut _160: uint32_t = 0;
    let mut _161: uint32_t = 0;
    let mut _162: uint32_t = 0;
    let mut _163: uint32_t = 0;
    let mut _164: uint32_t = 0;
    let mut _165: uint32_t = 0;
    let mut _166: uint32_t = 0;
    let mut _167: uint32_t = 0;
    let mut _168: uint32_t = 0;
    let mut _169: uint32_t = 0;
    let mut _170: uint32_t = 0;
    let mut _171: uint32_t = 0;
    let mut _172: libc::c_float = 0.;
    let mut _173: uint32_t = 0;
    let mut _174: uint32_t = 0;
    let mut _175: uint32_t = 0;
    let mut _176: uint32_t = 0;
    let mut _177: uint32_t = 0;
    let mut _178: uint32_t = 0;
    let mut _179: uint32_t = 0;
    let mut _180: uint32_t = 0;
    let mut _181: uint32_t = 0;
    let mut _182: uint32_t = 0;
    let mut _183: uint32_t = 0;
    let mut _184: libc::c_float = 0.;
    let mut _185: uint32_t = 0;
    let mut _186: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _187: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _188: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _189: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _190: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _191: uint8_t = 0;
    let mut _192: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _193: uint32_t = 0;
    let mut _194: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _195: uint32_t = 0;
    let mut _196: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _197: uint32_t = 0;
    let mut _198: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _199: uint32_t = 0;
    let mut _200: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _201: uint32_t = 0;
    let mut _202: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _203: uint32_t = 0;
    let mut _204: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _205: uint32_t = 0;
    let mut _206: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _207: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _208: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _209: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _210: uint32_t = 0;
    let mut _211: uint32_t = 0;
    let mut _212: uint32_t = 0;
    let mut _213: uint8_t = 0;
    let mut _214: uint32_t = 0;
    let mut _215: uint32_t = 0;
    let mut _215__PHI_TEMPORARY: uint32_t = 0;
    let mut _216: uint32_t = 0;
    let mut _217: uint32_t = 0;
    let mut _218: uint32_t = 0;
    let mut _219: uint32_t = 0;
    let mut _220: uint32_t = 0;
    let mut _221: uint8_t = 0;
    let mut _222: uint32_t = 0;
    let mut _223: uint32_t = 0;
    let mut _223__PHI_TEMPORARY: uint32_t = 0;
    let mut _224: uint32_t = 0;
    let mut _225: uint32_t = 0;
    let mut _226: uint32_t = 0;
    let mut _227: uint8_t = 0;
    let mut _228: bool_0 = 0;
    let mut _229: uint64_t = 0;
    let mut _230: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _231: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _232: uint32_t = 0;
    let mut _233: uint32_t = 0;
    let mut _234: uint32_t = 0;
    let mut _235: uint32_t = 0;
    let mut _236: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _237: uint32_t = 0;
    let mut _238: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _239: uint32_t = 0;
    let mut _240: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _241: uint32_t = 0;
    let mut _242: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _243: uint32_t = 0;
    let mut _244: uint32_t = 0;
    let mut _245: uint32_t = 0;
    let mut _246: uint32_t = 0;
    let mut _247: uint32_t = 0;
    let mut _248: uint32_t = 0;
    let mut _249: uint32_t = 0;
    let mut _250: uint32_t = 0;
    let mut _251: uint32_t = 0;
    let mut _252: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _253: uint32_t = 0;
    let mut _254: uint32_t = 0;
    let mut _255: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _256: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _257: uint32_t = 0;
    let mut _258: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _259: uint32_t = 0;
    let mut _260: uint32_t = 0;
    let mut _261: uint32_t = 0;
    let mut _262: uint32_t = 0;
    let mut _263: uint32_t = 0;
    let mut _264: uint32_t = 0;
    let mut _265: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _266: uint32_t = 0;
    let mut _267: uint32_t = 0;
    let mut _268: uint32_t = 0;
    let mut _269: uint32_t = 0;
    let mut _270: uint32_t = 0;
    let mut _271: uint32_t = 0;
    let mut _272: uint32_t = 0;
    let mut _273: uint32_t = 0;
    let mut _274: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _275: uint32_t = 0;
    let mut _276: uint32_t = 0;
    let mut _277: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _278: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _279: uint32_t = 0;
    let mut _280: uint32_t = 0;
    let mut _281: uint32_t = 0;
    let mut _282: uint8_t = 0;
    let mut _283: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _284: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _285: uint32_t = 0;
    let mut _286: uint32_t = 0;
    let mut _287: uint32_t = 0;
    let mut _288: uint8_t = 0;
    let mut _289: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _290: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _291: uint32_t = 0;
    let mut _292: uint32_t = 0;
    let mut _293: uint32_t = 0;
    let mut _294: uint8_t = 0;
    let mut _295: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _296: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _297: uint32_t = 0;
    let mut _298: uint32_t = 0;
    let mut _299: uint32_t = 0;
    let mut _300: uint8_t = 0;
    let mut _301: uint32_t = 0;
    let mut _302: uint8_t = 0;
    let mut _303: uint32_t = 0;
    let mut _304: uint8_t = 0;
    let mut _305: uint32_t = 0;
    let mut _306: uint8_t = 0;
    let mut _307: uint32_t = 0;
    let mut _308: uint8_t = 0;
    let mut _309: uint8_t = 0;
    let mut _310: uint8_t = 0;
    let mut _311: uint8_t = 0;
    let mut _312: uint8_t = 0;
    let mut _313: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _314: libc::c_float = 0.;
    let mut _315: libc::c_float = 0.;
    let mut _316: libc::c_float = 0.;
    let mut _317: libc::c_float = 0.;
    let mut _318: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _319: uint32_t = 0;
    let mut _320: uint32_t = 0;
    let mut _321: uint32_t = 0;
    let mut _322: uint32_t = 0;
    let mut _323: uint32_t = 0;
    let mut _324: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _325: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _326: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _327: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _328: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _329: libc::c_float = 0.;
    let mut _330: libc::c_float = 0.;
    let mut _331: libc::c_float = 0.;
    let mut _332: libc::c_float = 0.;
    let mut _333: libc::c_float = 0.;
    let mut _334: libc::c_float = 0.;
    let mut _335: libc::c_float = 0.;
    let mut _336: libc::c_float = 0.;
    let mut _337: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _338: libc::c_float = 0.;
    let mut _339: libc::c_float = 0.;
    let mut _340: libc::c_float = 0.;
    let mut _341: libc::c_float = 0.;
    let mut _342: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _343: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _344: uint32_t = 0;
    let mut _345: uint32_t = 0;
    let mut _346: uint32_t = 0;
    let mut _347: uint32_t = 0;
    let mut _348: uint32_t = 0;
    let mut _349: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _350: uint32_t = 0;
    let mut _351: uint32_t = 0;
    let mut _352: uint32_t = 0;
    let mut _353: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _354: uint32_t = 0;
    let mut _355: uint32_t = 0;
    let mut _356: uint32_t = 0;
    let mut _357: uint32_t = 0;
    let mut _358: uint32_t = 0;
    let mut _359: uint32_t = 0;
    let mut _360: uint32_t = 0;
    let mut _361: uint32_t = 0;
    let mut _362: uint32_t = 0;
    let mut _363: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _364: uint32_t = 0;
    let mut _365: uint32_t = 0;
    let mut _366: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _367: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _368: uint32_t = 0;
    let mut _369: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _370: uint32_t = 0;
    let mut _371: uint32_t = 0;
    let mut _372: uint32_t = 0;
    let mut _373: uint32_t = 0;
    let mut _374: uint32_t = 0;
    let mut _375: uint32_t = 0;
    let mut _376: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _377: uint32_t = 0;
    let mut _378: uint32_t = 0;
    let mut _379: uint32_t = 0;
    let mut _380: uint32_t = 0;
    let mut _381: uint32_t = 0;
    let mut _382: uint32_t = 0;
    let mut _383: uint32_t = 0;
    let mut _384: uint32_t = 0;
    let mut _385: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _386: uint32_t = 0;
    let mut _387: uint32_t = 0;
    let mut _388: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _389: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _390: uint32_t = 0;
    let mut _391: uint32_t = 0;
    let mut _392: uint32_t = 0;
    let mut _393: uint16_t = 0;
    let mut _394: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _395: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _396: uint32_t = 0;
    let mut _397: uint32_t = 0;
    let mut _398: uint32_t = 0;
    let mut _399: uint16_t = 0;
    let mut _400: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _401: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _402: uint32_t = 0;
    let mut _403: uint32_t = 0;
    let mut _404: uint32_t = 0;
    let mut _405: uint16_t = 0;
    let mut _406: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _407: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _408: uint32_t = 0;
    let mut _409: uint32_t = 0;
    let mut _410: uint32_t = 0;
    let mut _411: uint16_t = 0;
    let mut _412: uint32_t = 0;
    let mut _413: uint16_t = 0;
    let mut _414: uint32_t = 0;
    let mut _415: uint16_t = 0;
    let mut _416: uint32_t = 0;
    let mut _417: uint16_t = 0;
    let mut _418: uint32_t = 0;
    let mut _419: uint16_t = 0;
    let mut _420: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _421: uint32_t = 0;
    let mut _422: uint32_t = 0;
    let mut _423: uint32_t = 0;
    let mut _424: uint32_t = 0;
    let mut _425: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _426: l_array_2_uint64_t = l_array_2_uint64_t { array: [0; 2] };
    let mut _427: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _428: uint32_t = 0;
    let mut _429: libc::c_float = 0.;
    let mut _430: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _431: uint32_t = 0;
    let mut _432: libc::c_float = 0.;
    let mut _433: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _434: uint32_t = 0;
    let mut _435: libc::c_float = 0.;
    let mut _436: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _437: uint32_t = 0;
    let mut _438: libc::c_float = 0.;
    let mut _439: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _440: libc::c_float = 0.;
    let mut _441: libc::c_float = 0.;
    let mut _442: libc::c_float = 0.;
    let mut _443: libc::c_float = 0.;
    let mut _444: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _445: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _446: uint32_t = 0;
    let mut _447: uint32_t = 0;
    let mut _448: uint32_t = 0;
    let mut _449: uint32_t = 0;
    let mut _450: uint32_t = 0;
    let mut _451: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _452: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _453: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _454: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _455: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _456: libc::c_float = 0.;
    let mut _457: libc::c_float = 0.;
    let mut _458: libc::c_float = 0.;
    let mut _459: libc::c_float = 0.;
    let mut _460: libc::c_float = 0.;
    let mut _461: libc::c_float = 0.;
    let mut _462: libc::c_float = 0.;
    let mut _463: libc::c_float = 0.;
    let mut _464: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _465: libc::c_float = 0.;
    let mut _466: libc::c_float = 0.;
    let mut _467: libc::c_float = 0.;
    let mut _468: libc::c_float = 0.;
    let mut _469: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _470: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _471: uint32_t = 0;
    let mut _472: uint32_t = 0;
    let mut _473: uint32_t = 0;
    let mut _474: uint32_t = 0;
    let mut _475: uint32_t = 0;
    let mut _476: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _477: uint32_t = 0;
    let mut _478: uint32_t = 0;
    let mut _479: uint32_t = 0;
    let mut _480: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _481: uint32_t = 0;
    let mut _482: uint32_t = 0;
    let mut _483: uint32_t = 0;
    let mut _484: uint32_t = 0;
    let mut _485: uint32_t = 0;
    let mut _486: uint32_t = 0;
    let mut _487: uint32_t = 0;
    let mut _488: uint32_t = 0;
    let mut _489: uint32_t = 0;
    let mut _490: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _491: uint32_t = 0;
    let mut _492: uint32_t = 0;
    let mut _493: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _494: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _495: uint32_t = 0;
    let mut _496: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _497: uint32_t = 0;
    let mut _498: uint32_t = 0;
    let mut _499: uint32_t = 0;
    let mut _500: uint32_t = 0;
    let mut _501: uint32_t = 0;
    let mut _502: uint32_t = 0;
    let mut _503: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _504: uint32_t = 0;
    let mut _505: uint32_t = 0;
    let mut _506: uint32_t = 0;
    let mut _507: uint32_t = 0;
    let mut _508: uint32_t = 0;
    let mut _509: uint32_t = 0;
    let mut _510: uint32_t = 0;
    let mut _511: uint32_t = 0;
    let mut _512: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _513: uint32_t = 0;
    let mut _514: uint32_t = 0;
    let mut _515: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _516: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _517: uint32_t = 0;
    let mut _518: uint32_t = 0;
    let mut _519: uint32_t = 0;
    let mut _520: libc::c_float = 0.;
    let mut _521: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _522: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _523: uint32_t = 0;
    let mut _524: uint32_t = 0;
    let mut _525: uint32_t = 0;
    let mut _526: libc::c_float = 0.;
    let mut _527: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _528: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _529: uint32_t = 0;
    let mut _530: uint32_t = 0;
    let mut _531: uint32_t = 0;
    let mut _532: libc::c_float = 0.;
    let mut _533: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _534: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _535: uint32_t = 0;
    let mut _536: uint32_t = 0;
    let mut _537: uint32_t = 0;
    let mut _538: libc::c_float = 0.;
    let mut _539: uint32_t = 0;
    let mut _540: libc::c_float = 0.;
    let mut _541: uint32_t = 0;
    let mut _542: libc::c_float = 0.;
    let mut _543: uint32_t = 0;
    let mut _544: libc::c_float = 0.;
    let mut _545: uint32_t = 0;
    let mut _546: libc::c_float = 0.;
    let mut _547: libc::c_float = 0.;
    let mut _548: libc::c_float = 0.;
    let mut _549: libc::c_float = 0.;
    let mut _550: libc::c_float = 0.;
    let mut _551: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _552: libc::c_float = 0.;
    let mut _553: libc::c_float = 0.;
    let mut _554: libc::c_float = 0.;
    let mut _555: libc::c_float = 0.;
    let mut _556: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _557: uint32_t = 0;
    let mut _558: uint32_t = 0;
    let mut _559: uint32_t = 0;
    let mut _560: uint32_t = 0;
    let mut _561: uint32_t = 0;
    let mut _562: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _563: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _564: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _565: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _566: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _567: libc::c_float = 0.;
    let mut _568: libc::c_float = 0.;
    let mut _569: libc::c_float = 0.;
    let mut _570: libc::c_float = 0.;
    let mut _571: libc::c_float = 0.;
    let mut _572: libc::c_float = 0.;
    let mut _573: libc::c_float = 0.;
    let mut _574: libc::c_float = 0.;
    let mut _575: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _576: libc::c_float = 0.;
    let mut _577: libc::c_float = 0.;
    let mut _578: libc::c_float = 0.;
    let mut _579: libc::c_float = 0.;
    let mut _580: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _581: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _582: uint32_t = 0;
    let mut _583: uint32_t = 0;
    let mut _584: uint32_t = 0;
    let mut _585: uint32_t = 0;
    let mut _586: uint32_t = 0;
    let mut _587: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _588: uint32_t = 0;
    let mut _589: uint32_t = 0;
    let mut _590: uint32_t = 0;
    let mut _591: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _592: libc::c_float = 0.;
    let mut _593: libc::c_float = 0.;
    let mut _594: libc::c_float = 0.;
    let mut _595: libc::c_float = 0.;
    let mut _596: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _597: uint32_t = 0;
    let mut _598: uint32_t = 0;
    let mut _599: uint32_t = 0;
    let mut _600: uint32_t = 0;
    let mut _601: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _602: uint32_t = 0;
    let mut _603: uint32_t = 0;
    let mut _604: uint32_t = 0;
    let mut _605: uint32_t = 0;
    let mut _606: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _607: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _608: uint32_t = 0;
    let mut _609: uint32_t = 0;
    let mut _610: uint32_t = 0;
    let mut _611: uint32_t = 0;
    let mut _612: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _613: uint32_t = 0;
    let mut _614: uint32_t = 0;
    let mut _615: uint32_t = 0;
    let mut _616: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _617: uint32_t = 0;
    let mut _618: uint32_t = 0;
    let mut _619: uint32_t = 0;
    let mut _620: uint32_t = 0;
    let mut _621: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _622: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _623: uint32_t = 0;
    let mut _624: uint32_t = 0;
    let mut _625: uint32_t = 0;
    let mut _626: uint32_t = 0;
    let mut _627: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _628: uint32_t = 0;
    let mut _629: uint32_t = 0;
    let mut _630: uint8_t = 0;
    let mut _631: uint32_t = 0;
    let mut _632: uint32_t = 0;
    let mut _633: uint32_t = 0;
    let mut _634: uint32_t = 0;
    let mut _635: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _636: uint32_t = 0;
    let mut _637: uint32_t = 0;
    let mut _638: uint32_t = 0;
    let mut _639: uint32_t = 0;
    let mut _640: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _641: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _642: uint32_t = 0;
    let mut _643: uint32_t = 0;
    let mut _644: uint32_t = 0;
    let mut _645: uint32_t = 0;
    let mut _646: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _647: uint32_t = 0;
    let mut _648: uint32_t = 0;
    let mut _649: uint32_t = 0;
    let mut _650: uint32_t = 0;
    let mut _651: uint32_t = 0;
    let mut _652: uint32_t = 0;
    let mut _653: uint32_t = 0;
    let mut _654: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _655: uint32_t = 0;
    let mut _656: uint32_t = 0;
    let mut _657: uint32_t = 0;
    let mut _658: uint32_t = 0;
    let mut _659: uint32_t = 0;
    let mut _660: *mut libc::c_void = 0 as *mut libc::c_void;
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
    let mut _673: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _674: uint32_t = 0;
    let mut _675: uint32_t = 0;
    let mut _676: uint32_t = 0;
    let mut _677: uint32_t = 0;
    let mut _678: uint32_t = 0;
    let mut _679: uint32_t = 0;
    let mut _680: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _681: uint32_t = 0;
    let mut _682: uint32_t = 0;
    let mut _683: uint32_t = 0;
    let mut _684: uint32_t = 0;
    let mut _685: uint32_t = 0;
    let mut _686: uint32_t = 0;
    let mut _687: uint32_t = 0;
    let mut _688: uint32_t = 0;
    let mut _689: uint8_t = 0;
    let mut _690: uint32_t = 0;
    let mut _691: uint32_t = 0;
    let mut _692: uint32_t = 0;
    let mut _693: uint32_t = 0;
    let mut _694: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _695: uint32_t = 0;
    let mut _696: uint32_t = 0;
    let mut _697: uint32_t = 0;
    let mut _698: uint32_t = 0;
    let mut _699: uint32_t = 0;
    let mut _700: uint32_t = 0;
    let mut _701: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _702: uint32_t = 0;
    let mut _703: uint32_t = 0;
    let mut _704: uint32_t = 0;
    let mut _705: uint32_t = 0;
    let mut _706: uint32_t = 0;
    let mut _707: uint32_t = 0;
    let mut _708: uint32_t = 0;
    let mut _709: uint32_t = 0;
    let mut _710: uint32_t = 0;
    let mut _711: uint8_t = 0;
    let mut _712: libc::c_float = 0.;
    let mut _713: libc::c_float = 0.;
    let mut _714: libc::c_float = 0.;
    let mut _715: libc::c_float = 0.;
    let mut _716: libc::c_float = 0.;
    let mut _717: uint8_t = 0;
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
    let mut _740: uint32_t = 0;
    let mut _741: uint32_t = 0;
    let mut _742: uint32_t = 0;
    let mut _743: uint32_t = 0;
    let mut _744: uint32_t = 0;
    let mut _745: uint32_t = 0;
    let mut _746: uint32_t = 0;
    let mut _747: uint32_t = 0;
    let mut _748: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _749: uint32_t = 0;
    let mut _750: uint32_t = 0;
    let mut _751: uint32_t = 0;
    let mut _752: uint32_t = 0;
    let mut _753: uint32_t = 0;
    let mut _754: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _755: libc::c_float = 0.;
    let mut _756: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _757: uint32_t = 0;
    let mut _758: uint32_t = 0;
    let mut _759: uint32_t = 0;
    let mut _760: uint32_t = 0;
    let mut _761: uint32_t = 0;
    let mut _762: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _763: libc::c_float = 0.;
    let mut _764: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _765: uint32_t = 0;
    let mut _766: uint32_t = 0;
    let mut _767: uint32_t = 0;
    let mut _768: uint32_t = 0;
    let mut _769: uint32_t = 0;
    let mut _770: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _771: libc::c_float = 0.;
    let mut _772: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _773: uint32_t = 0;
    let mut _774: uint32_t = 0;
    let mut _775: uint32_t = 0;
    let mut _776: uint32_t = 0;
    let mut _777: uint32_t = 0;
    let mut _778: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _779: libc::c_float = 0.;
    let mut _780: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _781: uint32_t = 0;
    let mut _782: uint32_t = 0;
    let mut _783: uint32_t = 0;
    let mut _784: uint32_t = 0;
    let mut _785: uint32_t = 0;
    let mut _786: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _787: libc::c_float = 0.;
    let mut _788: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _789: uint32_t = 0;
    let mut _790: uint32_t = 0;
    let mut _791: uint32_t = 0;
    let mut _792: uint32_t = 0;
    let mut _793: uint32_t = 0;
    let mut _794: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _795: libc::c_float = 0.;
    let mut _796: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _797: uint32_t = 0;
    let mut _798: uint32_t = 0;
    let mut _799: uint32_t = 0;
    let mut _800: uint32_t = 0;
    let mut _801: uint32_t = 0;
    let mut _802: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _803: libc::c_float = 0.;
    let mut _804: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _805: uint32_t = 0;
    let mut _806: uint32_t = 0;
    let mut _807: uint32_t = 0;
    let mut _808: uint32_t = 0;
    let mut _809: uint32_t = 0;
    let mut _810: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _811: libc::c_float = 0.;
    let mut _812: uint32_t = 0;
    let mut _813: uint32_t = 0;
    let mut _814: uint32_t = 0;
    let mut _815: uint32_t = 0;
    let mut _816: uint32_t = 0;
    let mut _817: libc::c_float = 0.;
    let mut _818: libc::c_float = 0.;
    let mut _819: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _820: uint32_t = 0;
    let mut _821: uint32_t = 0;
    let mut _822: uint32_t = 0;
    let mut _823: uint32_t = 0;
    let mut _824: uint32_t = 0;
    let mut _825: uint32_t = 0;
    let mut _826: uint32_t = 0;
    let mut _827: uint32_t = 0;
    let mut _828: uint32_t = 0;
    let mut _829: uint32_t = 0;
    let mut _830: uint32_t = 0;
    let mut _831: uint32_t = 0;
    let mut _832: uint32_t = 0;
    let mut _833: uint32_t = 0;
    let mut _834: uint32_t = 0;
    let mut _835: uint32_t = 0;
    let mut _836: uint32_t = 0;
    let mut _837: uint32_t = 0;
    let mut _838: uint32_t = 0;
    let mut _839: uint32_t = 0;
    let mut _840: uint32_t = 0;
    let mut _841: uint32_t = 0;
    let mut _842: uint32_t = 0;
    let mut _843: uint32_t = 0;
    let mut _844: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _845: uint32_t = 0;
    let mut _846: uint32_t = 0;
    let mut _847: uint32_t = 0;
    let mut _848: uint32_t = 0;
    let mut _849: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _850: libc::c_float = 0.;
    let mut _851: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _852: uint32_t = 0;
    let mut _853: uint32_t = 0;
    let mut _854: uint32_t = 0;
    let mut _855: uint32_t = 0;
    let mut _856: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _857: libc::c_float = 0.;
    let mut _858: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _859: uint32_t = 0;
    let mut _860: uint32_t = 0;
    let mut _861: uint32_t = 0;
    let mut _862: uint32_t = 0;
    let mut _863: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _864: libc::c_float = 0.;
    let mut _865: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _866: uint32_t = 0;
    let mut _867: uint32_t = 0;
    let mut _868: uint32_t = 0;
    let mut _869: uint32_t = 0;
    let mut _870: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _871: libc::c_float = 0.;
    let mut _872: uint32_t = 0;
    let mut _873: uint32_t = 0;
    let mut _874: uint32_t = 0;
    let mut _875: libc::c_float = 0.;
    let mut _876: libc::c_float = 0.;
    let mut _877: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _878: uint32_t = 0;
    let mut _879: uint32_t = 0;
    let mut _880: uint32_t = 0;
    _68 = _1;
    _69 = _2;
    _186 = _69;
    _187 = *(&mut (*(_186 as *mut l_struct_struct_OC_pixel_region_args)).field0
        as *mut *mut libc::c_void);
    _70 = _187;
    _188 = _69;
    _189 = memcpy(
        &mut _71 as *mut l_struct_struct_OC_astcenc_swizzle as *mut libc::c_void,
        &mut (*(_188 as *mut l_struct_struct_OC_pixel_region_args)).field1
            as *mut l_struct_struct_OC_astcenc_swizzle as *mut libc::c_void,
        16 as libc::c_int as uint64_t,
    );
    _190 = _69;
    _191 = *(&mut (*(_190 as *mut l_struct_struct_OC_pixel_region_args)).field2 as *mut uint8_t);
    _72 = (_191 as libc::c_uint & 1 as libc::c_uint) as bool_0;
    _192 = _69;
    _193 = *(&mut (*(_192 as *mut l_struct_struct_OC_pixel_region_args)).field4 as *mut uint32_t);
    _73 = _193;
    _194 = _69;
    _195 = *(&mut (*(_194 as *mut l_struct_struct_OC_pixel_region_args)).field5 as *mut uint32_t);
    _74 = _195;
    _196 = _69;
    _197 = *(&mut (*(_196 as *mut l_struct_struct_OC_pixel_region_args)).field6 as *mut uint32_t);
    _75 = _197;
    _198 = _69;
    _199 = *(&mut (*(_198 as *mut l_struct_struct_OC_pixel_region_args)).field7 as *mut uint32_t);
    _76 = _199;
    _200 = _69;
    _201 = *(&mut (*(_200 as *mut l_struct_struct_OC_pixel_region_args)).field8 as *mut uint32_t);
    _77 = _201;
    _202 = _69;
    _203 = *(&mut (*(_202 as *mut l_struct_struct_OC_pixel_region_args)).field9 as *mut uint32_t);
    _78 = _203;
    _204 = _69;
    _205 = *(&mut (*(_204 as *mut l_struct_struct_OC_pixel_region_args)).field3 as *mut uint32_t);
    _79 = _205;
    _206 = _68;
    _207 = *(&mut (*(_206 as *mut l_struct_struct_OC_astcenc_contexti)).field3
        as *mut *mut libc::c_void);
    _80 = _207;
    _208 = _69;
    _209 = *(&mut (*(_208 as *mut l_struct_struct_OC_pixel_region_args)).field10
        as *mut *mut libc::c_void);
    _81 = _209;
    _210 = _79;
    _82 = _210;
    _211 = _82;
    _83 = llvm_add_u32(
        llvm_mul_u32(2 as libc::c_int as uint32_t, _211),
        1 as libc::c_int as uint32_t,
    );
    _212 = _82;
    _84 = _212;
    _213 = _72;
    if _213 as libc::c_uint & 1 as libc::c_uint != 0 {
        _214 = _82;
        _215__PHI_TEMPORARY = _214;
    } else {
        _215__PHI_TEMPORARY = 0;
    }
    _215 = _215__PHI_TEMPORARY;
    _85 = _215;
    _216 = _73;
    _217 = _83;
    _86 = llvm_add_u32(_216, _217);
    _218 = _74;
    _219 = _83;
    _87 = llvm_add_u32(_218, _219);
    _220 = _75;
    _221 = _72;
    if _221 as libc::c_uint & 1 as libc::c_uint != 0 {
        _222 = _83;
        _223__PHI_TEMPORARY = _222;
    } else {
        _223__PHI_TEMPORARY = 0;
    }
    _223 = _223__PHI_TEMPORARY;
    _88 = llvm_add_u32(_220, _223);
    _224 = _86;
    _225 = _87;
    _226 = _88;
    _89 = llvm_mul_u32(llvm_mul_u32(_224, _225), _226);
    _227 = _72;
    _228 = (_227 as libc::c_uint & 1 as libc::c_uint) as bool_0;
    _229 = _228 as uint64_t;
    _90 = llvm_select_u32(_228, 1 as libc::c_int as uint32_t, 0);
    _230 = _81;
    _91 = _230;
    _231 = _81;
    _232 = _89;
    _92 = &mut *(_231 as *mut l_struct_struct_OC_vfloat4)
        .offset(_232 as int32_t as int64_t as isize) as *mut l_struct_struct_OC_vfloat4
        as *mut libc::c_void;
    _233 = _86;
    _93 = _233;
    _234 = _86;
    _235 = _87;
    _94 = llvm_mul_u32(_234, _235);
    _236 = _70;
    _237 = *(&mut (*(_236 as *mut l_struct_struct_OC_astcenc_image)).field0 as *mut uint32_t);
    _95 = _237;
    _238 = _70;
    _239 = *(&mut (*(_238 as *mut l_struct_struct_OC_astcenc_image)).field0 as *mut uint32_t);
    _240 = _70;
    _241 = *(&mut (*(_240 as *mut l_struct_struct_OC_astcenc_image)).field1 as *mut uint32_t);
    _96 = llvm_mul_u32(_239, _241);
    _242 = _70;
    _243 = *(&mut (*(_242 as *mut l_struct_struct_OC_astcenc_image)).field3 as *mut uint32_t);
    if _243 == 0 as libc::c_uint {
        *(&mut *(_97.array)
            .as_mut_ptr()
            .offset(4 as libc::c_int as int64_t as isize) as *mut uint8_t) =
            0 as libc::c_int as uint8_t;
        *(&mut *(_97.array)
            .as_mut_ptr()
            .offset(5 as libc::c_int as int64_t as isize) as *mut uint8_t) =
            -(1 as libc::c_int) as uint8_t;
        _244 = _90;
        _98 = _244;
        loop {
            _245 = _98;
            _246 = _88;
            if !((_245 as int32_t) < _246 as int32_t) {
                break;
            }
            _247 = _98;
            _248 = _90;
            _249 = _78;
            _250 = _85;
            _99 = llvm_sub_u32(llvm_add_u32(llvm_sub_u32(_247, _248), _249), _250);
            _251 = _99;
            _252 = _70;
            _253 =
                *(&mut (*(_252 as *mut l_struct_struct_OC_astcenc_image)).field2 as *mut uint32_t);
            _254 = _ZN4astc5clampIiEET_S1_S1_S1_(
                _251,
                0,
                llvm_sub_u32(_253, 1 as libc::c_int as uint32_t),
            );
            _99 = _254;
            _255 = _70;
            _256 = *(&mut (*(_255 as *mut l_struct_struct_OC_astcenc_image)).field4
                as *mut *mut libc::c_void);
            _257 = _99;
            _258 = *(&mut *(_256 as *mut *mut libc::c_void)
                .offset(_257 as int32_t as int64_t as isize)
                as *mut *mut libc::c_void);
            _100 = _258;
            _101 = 1 as libc::c_int as uint32_t;
            loop {
                _259 = _101;
                _260 = _87;
                if !((_259 as int32_t) < _260 as int32_t) {
                    break;
                }
                _261 = _101;
                _262 = _77;
                _263 = _84;
                _102 = llvm_sub_u32(
                    llvm_add_u32(llvm_sub_u32(_261, 1 as libc::c_int as uint32_t), _262),
                    _263,
                );
                _264 = _102;
                _265 = _70;
                _266 = *(&mut (*(_265 as *mut l_struct_struct_OC_astcenc_image)).field1
                    as *mut uint32_t);
                _267 = _ZN4astc5clampIiEET_S1_S1_S1_(
                    _264,
                    0,
                    llvm_sub_u32(_266, 1 as libc::c_int as uint32_t),
                );
                _102 = _267;
                _103 = 1 as libc::c_int as uint32_t;
                loop {
                    _268 = _103;
                    _269 = _86;
                    if !((_268 as int32_t) < _269 as int32_t) {
                        break;
                    }
                    _270 = _103;
                    _271 = _76;
                    _272 = _84;
                    _104 = llvm_sub_u32(
                        llvm_add_u32(llvm_sub_u32(_270, 1 as libc::c_int as uint32_t), _271),
                        _272,
                    );
                    _273 = _104;
                    _274 = _70;
                    _275 = *(&mut (*(_274 as *mut l_struct_struct_OC_astcenc_image)).field0
                        as *mut uint32_t);
                    _276 = _ZN4astc5clampIiEET_S1_S1_S1_(
                        _273,
                        0,
                        llvm_sub_u32(_275, 1 as libc::c_int as uint32_t),
                    );
                    _104 = _276;
                    _277 = _100;
                    _278 = _70;
                    _279 = *(&mut (*(_278 as *mut l_struct_struct_OC_astcenc_image)).field0
                        as *mut uint32_t);
                    _280 = _102;
                    _281 = _104;
                    _282 = *(&mut *(_277 as *mut uint8_t).offset((llvm_add_u32
                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                4 as libc::c_int as uint32_t,
                                _279,
                            ),
                            _280,
                        ),
                        (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            4 as libc::c_int as uint32_t,
                            _281,
                        ),
                    ) as uint64_t
                        as int64_t
                        as isize) as *mut uint8_t);
                    *(&mut *(_97.array)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as int64_t as isize)
                        as *mut uint8_t) = _282;
                    _283 = _100;
                    _284 = _70;
                    _285 = *(&mut (*(_284 as *mut l_struct_struct_OC_astcenc_image)).field0
                        as *mut uint32_t);
                    _286 = _102;
                    _287 = _104;
                    _288 = *(&mut *(_283 as *mut uint8_t).offset((llvm_add_u32
                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                4 as libc::c_int as uint32_t,
                                _285,
                            ),
                            _286,
                        ),
                        (llvm_add_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                4 as libc::c_int as uint32_t,
                                _287,
                            ),
                            1 as libc::c_int as uint32_t,
                        ),
                    ) as uint64_t
                        as int64_t
                        as isize) as *mut uint8_t);
                    *(&mut *(_97.array)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as int64_t as isize)
                        as *mut uint8_t) = _288;
                    _289 = _100;
                    _290 = _70;
                    _291 = *(&mut (*(_290 as *mut l_struct_struct_OC_astcenc_image)).field0
                        as *mut uint32_t);
                    _292 = _102;
                    _293 = _104;
                    _294 = *(&mut *(_289 as *mut uint8_t).offset((llvm_add_u32
                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                4 as libc::c_int as uint32_t,
                                _291,
                            ),
                            _292,
                        ),
                        (llvm_add_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                4 as libc::c_int as uint32_t,
                                _293,
                            ),
                            2 as libc::c_int as uint32_t,
                        ),
                    ) as uint64_t
                        as int64_t
                        as isize) as *mut uint8_t);
                    *(&mut *(_97.array)
                        .as_mut_ptr()
                        .offset(2 as libc::c_int as int64_t as isize)
                        as *mut uint8_t) = _294;
                    _295 = _100;
                    _296 = _70;
                    _297 = *(&mut (*(_296 as *mut l_struct_struct_OC_astcenc_image)).field0
                        as *mut uint32_t);
                    _298 = _102;
                    _299 = _104;
                    _300 = *(&mut *(_295 as *mut uint8_t).offset((llvm_add_u32
                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                4 as libc::c_int as uint32_t,
                                _297,
                            ),
                            _298,
                        ),
                        (llvm_add_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                4 as libc::c_int as uint32_t,
                                _299,
                            ),
                            3 as libc::c_int as uint32_t,
                        ),
                    ) as uint64_t
                        as int64_t
                        as isize) as *mut uint8_t);
                    *(&mut *(_97.array)
                        .as_mut_ptr()
                        .offset(3 as libc::c_int as int64_t as isize)
                        as *mut uint8_t) = _300;
                    _301 = *(&mut _71.field0 as *mut uint32_t);
                    _302 = *(&mut *(_97.array)
                        .as_mut_ptr()
                        .offset(_301 as uint64_t as int64_t as isize)
                        as *mut uint8_t);
                    _105 = _302;
                    _303 = *(&mut _71.field1 as *mut uint32_t);
                    _304 = *(&mut *(_97.array)
                        .as_mut_ptr()
                        .offset(_303 as uint64_t as int64_t as isize)
                        as *mut uint8_t);
                    _106 = _304;
                    _305 = *(&mut _71.field2 as *mut uint32_t);
                    _306 = *(&mut *(_97.array)
                        .as_mut_ptr()
                        .offset(_305 as uint64_t as int64_t as isize)
                        as *mut uint8_t);
                    _107 = _306;
                    _307 = *(&mut _71.field3 as *mut uint32_t);
                    _308 = *(&mut *(_97.array)
                        .as_mut_ptr()
                        .offset(_307 as uint64_t as int64_t as isize)
                        as *mut uint8_t);
                    _108 = _308;
                    _309 = _105;
                    _310 = _106;
                    _311 = _107;
                    _312 = _108;
                    _63 = &mut _109 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_void;
                    _64 = llvm_fmul_f32(
                        _309 as uint32_t as int32_t as libc::c_float,
                        0.00392156886f64 as libc::c_float,
                    );
                    _65 = llvm_fmul_f32(
                        _310 as uint32_t as int32_t as libc::c_float,
                        0.00392156886f64 as libc::c_float,
                    );
                    _66 = llvm_fmul_f32(
                        _311 as uint32_t as int32_t as libc::c_float,
                        0.00392156886f64 as libc::c_float,
                    );
                    _67 = llvm_fmul_f32(
                        _312 as uint32_t as int32_t as libc::c_float,
                        0.00392156886f64 as libc::c_float,
                    );
                    _313 = _63;
                    _314 = _64;
                    *(_313 as *mut libc::c_float) = _314;
                    _315 = _65;
                    *(&mut *((*(_313 as *mut l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as int64_t as isize)
                        as *mut libc::c_float) = _315;
                    _316 = _66;
                    *(&mut *((*(_313 as *mut l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(2 as libc::c_int as int64_t as isize)
                        as *mut libc::c_float) = _316;
                    _317 = _67;
                    *(&mut *((*(_313 as *mut l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(3 as libc::c_int as int64_t as isize)
                        as *mut libc::c_float) = _317;
                    _318 = _91;
                    _319 = _98;
                    _320 = _94;
                    _321 = _101;
                    _322 = _93;
                    _323 = _103;
                    _324 = memcpy(
                        &mut *(_318 as *mut l_struct_struct_OC_vfloat4).offset((llvm_add_u32
                            as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            (llvm_add_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                (llvm_mul_u32
                                    as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                    _319, _320,
                                ),
                                (llvm_mul_u32
                                    as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                    _321, _322,
                                ),
                            ),
                            _323,
                        )
                            as int32_t
                            as int64_t
                            as isize) as *mut l_struct_struct_OC_vfloat4
                            as *mut libc::c_void,
                        &mut _109 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_void,
                        16 as libc::c_int as uint64_t,
                    );
                    _325 = memcpy(
                        &mut _111 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_void,
                        &mut _109 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_void,
                        16 as libc::c_int as uint64_t,
                    );
                    _326 = memcpy(
                        &mut _112 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_void,
                        &mut _109 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_void,
                        16 as libc::c_int as uint64_t,
                    );
                    _327 = *(&mut _111.field0 as *mut l_array_4_float);
                    _328 = *(&mut _112.field0 as *mut l_array_4_float);
                    *(&mut _40 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) = _327;
                    *(&mut _41 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) = _328;
                    _329 = *(&mut _40 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_float);
                    _330 = *(&mut _41 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_float);
                    _331 = *(&mut *((*(&mut _40 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as int64_t as isize)
                        as *mut libc::c_float);
                    _332 = *(&mut *((*(&mut _41 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as int64_t as isize)
                        as *mut libc::c_float);
                    _333 = *(&mut *((*(&mut _40 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as libc::c_int as int64_t as isize)
                        as *mut libc::c_float);
                    _334 = *(&mut *((*(&mut _41 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(2 as libc::c_int as int64_t as isize)
                        as *mut libc::c_float);
                    _335 = *(&mut *((*(&mut _40 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as libc::c_int as int64_t as isize)
                        as *mut libc::c_float);
                    _336 = *(&mut *((*(&mut _41 as *mut l_struct_struct_OC_vfloat4
                        as *mut l_array_4_float))
                        .array)
                        .as_mut_ptr()
                        .offset(3 as libc::c_int as int64_t as isize)
                        as *mut libc::c_float);
                    _34 = &mut _39 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_void;
                    _35 = llvm_fmul_f32(_329, _330);
                    _36 = llvm_fmul_f32(_331, _332);
                    _37 = llvm_fmul_f32(_333, _334);
                    _38 = llvm_fmul_f32(_335, _336);
                    _337 = _34;
                    _338 = _35;
                    *(_337 as *mut libc::c_float) = _338;
                    _339 = _36;
                    *(&mut *((*(_337 as *mut l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as int64_t as isize)
                        as *mut libc::c_float) = _339;
                    _340 = _37;
                    *(&mut *((*(_337 as *mut l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(2 as libc::c_int as int64_t as isize)
                        as *mut libc::c_float) = _340;
                    _341 = _38;
                    *(&mut *((*(_337 as *mut l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(3 as libc::c_int as int64_t as isize)
                        as *mut libc::c_float) = _341;
                    _342 = _39;
                    *(&mut _110.field0 as *mut l_array_4_float) = _342.field0;
                    _343 = _92;
                    _344 = _98;
                    _345 = _94;
                    _346 = _101;
                    _347 = _93;
                    _348 = _103;
                    _349 = memcpy(
                        &mut *(_343 as *mut l_struct_struct_OC_vfloat4).offset((llvm_add_u32
                            as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            (llvm_add_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                (llvm_mul_u32
                                    as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                    _344, _345,
                                ),
                                (llvm_mul_u32
                                    as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                    _346, _347,
                                ),
                            ),
                            _348,
                        )
                            as int32_t
                            as int64_t
                            as isize) as *mut l_struct_struct_OC_vfloat4
                            as *mut libc::c_void,
                        &mut _110 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_void,
                        16 as libc::c_int as uint64_t,
                    );
                    _350 = _103;
                    _103 = llvm_add_u32(_350, 1 as libc::c_int as uint32_t);
                }
                _351 = _101;
                _101 = llvm_add_u32(_351, 1 as libc::c_int as uint32_t);
            }
            _352 = _98;
            _98 = llvm_add_u32(_352, 1 as libc::c_int as uint32_t);
        }
    } else {
        _353 = _70;
        _354 = *(&mut (*(_353 as *mut l_struct_struct_OC_astcenc_image)).field3 as *mut uint32_t);
        if _354 == 1 as libc::c_uint {
            *(&mut *(_113.array)
                .as_mut_ptr()
                .offset(4 as libc::c_int as int64_t as isize) as *mut uint16_t) =
                0 as libc::c_int as uint16_t;
            *(&mut *(_113.array)
                .as_mut_ptr()
                .offset(5 as libc::c_int as int64_t as isize) as *mut uint16_t) =
                15360 as libc::c_int as uint16_t;
            _355 = _90;
            _114 = _355;
            loop {
                _356 = _114;
                _357 = _88;
                if !((_356 as int32_t) < _357 as int32_t) {
                    break;
                }
                _358 = _114;
                _359 = _90;
                _360 = _78;
                _361 = _85;
                _115 = llvm_sub_u32(llvm_add_u32(llvm_sub_u32(_358, _359), _360), _361);
                _362 = _115;
                _363 = _70;
                _364 = *(&mut (*(_363 as *mut l_struct_struct_OC_astcenc_image)).field2
                    as *mut uint32_t);
                _365 = _ZN4astc5clampIiEET_S1_S1_S1_(
                    _362,
                    0,
                    llvm_sub_u32(_364, 1 as libc::c_int as uint32_t),
                );
                _115 = _365;
                _366 = _70;
                _367 = *(&mut (*(_366 as *mut l_struct_struct_OC_astcenc_image)).field4
                    as *mut *mut libc::c_void);
                _368 = _115;
                _369 = *(&mut *(_367 as *mut *mut libc::c_void)
                    .offset(_368 as int32_t as int64_t as isize)
                    as *mut *mut libc::c_void);
                _116 = _369;
                _117 = 1 as libc::c_int as uint32_t;
                loop {
                    _370 = _117;
                    _371 = _87;
                    if !((_370 as int32_t) < _371 as int32_t) {
                        break;
                    }
                    _372 = _117;
                    _373 = _77;
                    _374 = _84;
                    _118 = llvm_sub_u32(
                        llvm_add_u32(llvm_sub_u32(_372, 1 as libc::c_int as uint32_t), _373),
                        _374,
                    );
                    _375 = _118;
                    _376 = _70;
                    _377 = *(&mut (*(_376 as *mut l_struct_struct_OC_astcenc_image)).field1
                        as *mut uint32_t);
                    _378 = _ZN4astc5clampIiEET_S1_S1_S1_(
                        _375,
                        0,
                        llvm_sub_u32(_377, 1 as libc::c_int as uint32_t),
                    );
                    _118 = _378;
                    _119 = 1 as libc::c_int as uint32_t;
                    loop {
                        _379 = _119;
                        _380 = _86;
                        if !((_379 as int32_t) < _380 as int32_t) {
                            break;
                        }
                        _381 = _119;
                        _382 = _76;
                        _383 = _84;
                        _120 = llvm_sub_u32(
                            llvm_add_u32(llvm_sub_u32(_381, 1 as libc::c_int as uint32_t), _382),
                            _383,
                        );
                        _384 = _120;
                        _385 = _70;
                        _386 = *(&mut (*(_385 as *mut l_struct_struct_OC_astcenc_image)).field0
                            as *mut uint32_t);
                        _387 = _ZN4astc5clampIiEET_S1_S1_S1_(
                            _384,
                            0,
                            llvm_sub_u32(_386, 1 as libc::c_int as uint32_t),
                        );
                        _120 = _387;
                        _388 = _116;
                        _389 = _70;
                        _390 = *(&mut (*(_389 as *mut l_struct_struct_OC_astcenc_image)).field0
                            as *mut uint32_t);
                        _391 = _118;
                        _392 = _120;
                        _393 = *(&mut *(_388 as *mut uint16_t).offset((llvm_add_u32
                            as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                (llvm_mul_u32
                                    as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                    4 as libc::c_int as uint32_t,
                                    _390,
                                ),
                                _391,
                            ),
                            (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                4 as libc::c_int as uint32_t,
                                _392,
                            ),
                        )
                            as uint64_t
                            as int64_t
                            as isize) as *mut uint16_t);
                        *(&mut *(_113.array)
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as int64_t as isize)
                            as *mut uint16_t) = _393;
                        _394 = _116;
                        _395 = _70;
                        _396 = *(&mut (*(_395 as *mut l_struct_struct_OC_astcenc_image)).field0
                            as *mut uint32_t);
                        _397 = _118;
                        _398 = _120;
                        _399 = *(&mut *(_394 as *mut uint16_t).offset((llvm_add_u32
                            as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                (llvm_mul_u32
                                    as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                    4 as libc::c_int as uint32_t,
                                    _396,
                                ),
                                _397,
                            ),
                            (llvm_add_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                (llvm_mul_u32
                                    as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                    4 as libc::c_int as uint32_t,
                                    _398,
                                ),
                                1 as libc::c_int as uint32_t,
                            ),
                        )
                            as uint64_t
                            as int64_t
                            as isize) as *mut uint16_t);
                        *(&mut *(_113.array)
                            .as_mut_ptr()
                            .offset(1 as libc::c_int as int64_t as isize)
                            as *mut uint16_t) = _399;
                        _400 = _116;
                        _401 = _70;
                        _402 = *(&mut (*(_401 as *mut l_struct_struct_OC_astcenc_image)).field0
                            as *mut uint32_t);
                        _403 = _118;
                        _404 = _120;
                        _405 = *(&mut *(_400 as *mut uint16_t).offset((llvm_add_u32
                            as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                (llvm_mul_u32
                                    as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                    4 as libc::c_int as uint32_t,
                                    _402,
                                ),
                                _403,
                            ),
                            (llvm_add_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                (llvm_mul_u32
                                    as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                    4 as libc::c_int as uint32_t,
                                    _404,
                                ),
                                2 as libc::c_int as uint32_t,
                            ),
                        )
                            as uint64_t
                            as int64_t
                            as isize) as *mut uint16_t);
                        *(&mut *(_113.array)
                            .as_mut_ptr()
                            .offset(2 as libc::c_int as int64_t as isize)
                            as *mut uint16_t) = _405;
                        _406 = _116;
                        _407 = _70;
                        _408 = *(&mut (*(_407 as *mut l_struct_struct_OC_astcenc_image)).field0
                            as *mut uint32_t);
                        _409 = _118;
                        _410 = _120;
                        _411 = *(&mut *(_406 as *mut uint16_t).offset((llvm_add_u32
                            as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                (llvm_mul_u32
                                    as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                    4 as libc::c_int as uint32_t,
                                    _408,
                                ),
                                _409,
                            ),
                            (llvm_add_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                (llvm_mul_u32
                                    as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                    4 as libc::c_int as uint32_t,
                                    _410,
                                ),
                                3 as libc::c_int as uint32_t,
                            ),
                        )
                            as uint64_t
                            as int64_t
                            as isize) as *mut uint16_t);
                        *(&mut *(_113.array)
                            .as_mut_ptr()
                            .offset(3 as libc::c_int as int64_t as isize)
                            as *mut uint16_t) = _411;
                        _412 = *(&mut _71.field0 as *mut uint32_t);
                        _413 = *(&mut *(_113.array)
                            .as_mut_ptr()
                            .offset(_412 as uint64_t as int64_t as isize)
                            as *mut uint16_t);
                        _414 = *(&mut _71.field1 as *mut uint32_t);
                        _415 = *(&mut *(_113.array)
                            .as_mut_ptr()
                            .offset(_414 as uint64_t as int64_t as isize)
                            as *mut uint16_t);
                        _416 = *(&mut _71.field2 as *mut uint32_t);
                        _417 = *(&mut *(_113.array)
                            .as_mut_ptr()
                            .offset(_416 as uint64_t as int64_t as isize)
                            as *mut uint16_t);
                        _418 = *(&mut _71.field3 as *mut uint32_t);
                        _419 = *(&mut *(_113.array)
                            .as_mut_ptr()
                            .offset(_418 as uint64_t as int64_t as isize)
                            as *mut uint16_t);
                        _29 = &mut _121 as *mut l_struct_struct_OC_vint4 as *mut libc::c_void;
                        _30 = _413 as uint32_t;
                        _31 = _415 as uint32_t;
                        _32 = _417 as uint32_t;
                        _33 = _419 as uint32_t;
                        _420 = _29;
                        _421 = _30;
                        *(_420 as *mut uint32_t) = _421;
                        _422 = _31;
                        *(&mut *((*(_420 as *mut l_array_4_uint32_t)).array)
                            .as_mut_ptr()
                            .offset(1 as libc::c_int as int64_t as isize)
                            as *mut uint32_t) = _422;
                        _423 = _32;
                        *(&mut *((*(_420 as *mut l_array_4_uint32_t)).array)
                            .as_mut_ptr()
                            .offset(2 as libc::c_int as int64_t as isize)
                            as *mut uint32_t) = _423;
                        _424 = _33;
                        *(&mut *((*(_420 as *mut l_array_4_uint32_t)).array)
                            .as_mut_ptr()
                            .offset(3 as libc::c_int as int64_t as isize)
                            as *mut uint32_t) = _424;
                        _425 = memcpy(
                            &mut _123 as *mut l_struct_struct_OC_vint4 as *mut libc::c_void,
                            &mut _121 as *mut l_struct_struct_OC_vint4 as *mut libc::c_void,
                            16 as libc::c_int as uint64_t,
                        );
                        _426 = (*(&mut _123.field0 as *mut l_array_4_uint32_t
                            as *mut C2RustUnnamed))
                            .data;
                        *(&mut _28 as *mut l_struct_struct_OC_vint4 as *mut l_array_2_uint64_t) =
                            _426;
                        _8 = &mut _28 as *mut l_struct_struct_OC_vint4 as *mut libc::c_void;
                        _427 = _8;
                        _428 = *(_427 as *mut uint32_t);
                        _429 = _Z13sf16_to_floatt(_428 as uint16_t);
                        _7 = &mut _28 as *mut l_struct_struct_OC_vint4 as *mut libc::c_void;
                        _430 = _7;
                        _431 = *(&mut *((*(_430 as *mut l_array_4_uint32_t)).array)
                            .as_mut_ptr()
                            .offset(1 as libc::c_int as int64_t as isize)
                            as *mut uint32_t);
                        _432 = _Z13sf16_to_floatt(_431 as uint16_t);
                        _6 = &mut _28 as *mut l_struct_struct_OC_vint4 as *mut libc::c_void;
                        _433 = _6;
                        _434 = *(&mut *((*(_433 as *mut l_array_4_uint32_t)).array)
                            .as_mut_ptr()
                            .offset(2 as libc::c_int as int64_t as isize)
                            as *mut uint32_t);
                        _435 = _Z13sf16_to_floatt(_434 as uint16_t);
                        _5 = &mut _28 as *mut l_struct_struct_OC_vint4 as *mut libc::c_void;
                        _436 = _5;
                        _437 = *(&mut *((*(_436 as *mut l_array_4_uint32_t)).array)
                            .as_mut_ptr()
                            .offset(3 as libc::c_int as int64_t as isize)
                            as *mut uint32_t);
                        _438 = _Z13sf16_to_floatt(_437 as uint16_t);
                        _22 = &mut _27 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_void;
                        _23 = _429;
                        _24 = _432;
                        _25 = _435;
                        _26 = _438;
                        _439 = _22;
                        _440 = _23;
                        *(_439 as *mut libc::c_float) = _440;
                        _441 = _24;
                        *(&mut *((*(_439 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as libc::c_int as int64_t as isize)
                            as *mut libc::c_float) = _441;
                        _442 = _25;
                        *(&mut *((*(_439 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as libc::c_int as int64_t as isize)
                            as *mut libc::c_float) = _442;
                        _443 = _26;
                        *(&mut *((*(_439 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(3 as libc::c_int as int64_t as isize)
                            as *mut libc::c_float) = _443;
                        _444 = _27;
                        *(&mut _122.field0 as *mut l_array_4_float) = _444.field0;
                        _445 = _91;
                        _446 = _114;
                        _447 = _94;
                        _448 = _117;
                        _449 = _93;
                        _450 = _119;
                        _451 = memcpy(
                            &mut *(_445 as *mut l_struct_struct_OC_vfloat4).offset((llvm_add_u32
                                as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                (llvm_add_u32
                                    as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                    (llvm_mul_u32
                                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                        _446, _447,
                                    ),
                                    (llvm_mul_u32
                                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                        _448, _449,
                                    ),
                                ),
                                _450,
                            )
                                as int32_t
                                as int64_t
                                as isize)
                                as *mut l_struct_struct_OC_vfloat4
                                as *mut libc::c_void,
                            &mut _122 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_void,
                            16 as libc::c_int as uint64_t,
                        );
                        _452 = memcpy(
                            &mut _125 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_void,
                            &mut _122 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_void,
                            16 as libc::c_int as uint64_t,
                        );
                        _453 = memcpy(
                            &mut _126 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_void,
                            &mut _122 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_void,
                            16 as libc::c_int as uint64_t,
                        );
                        _454 = *(&mut _125.field0 as *mut l_array_4_float);
                        _455 = *(&mut _126.field0 as *mut l_array_4_float);
                        *(&mut _48 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _454;
                        *(&mut _49 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) =
                            _455;
                        _456 = *(&mut _48 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_float);
                        _457 = *(&mut _49 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_float);
                        _458 = *(&mut *((*(&mut _48 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as libc::c_int as int64_t as isize)
                            as *mut libc::c_float);
                        _459 = *(&mut *((*(&mut _49 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(1 as libc::c_int as int64_t as isize)
                            as *mut libc::c_float);
                        _460 = *(&mut *((*(&mut _48 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as libc::c_int as int64_t as isize)
                            as *mut libc::c_float);
                        _461 = *(&mut *((*(&mut _49 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(2 as libc::c_int as int64_t as isize)
                            as *mut libc::c_float);
                        _462 = *(&mut *((*(&mut _48 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as libc::c_int as int64_t as isize)
                            as *mut libc::c_float);
                        _463 = *(&mut *((*(&mut _49 as *mut l_struct_struct_OC_vfloat4
                            as *mut l_array_4_float))
                            .array)
                            .as_mut_ptr()
                            .offset(3 as libc::c_int as int64_t as isize)
                            as *mut libc::c_float);
                        _42 = &mut _47 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_void;
                        _43 = llvm_fmul_f32(_456, _457);
                        _44 = llvm_fmul_f32(_458, _459);
                        _45 = llvm_fmul_f32(_460, _461);
                        _46 = llvm_fmul_f32(_462, _463);
                        _464 = _42;
                        _465 = _43;
                        *(_464 as *mut libc::c_float) = _465;
                        _466 = _44;
                        *(&mut *((*(_464 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(1 as libc::c_int as int64_t as isize)
                            as *mut libc::c_float) = _466;
                        _467 = _45;
                        *(&mut *((*(_464 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(2 as libc::c_int as int64_t as isize)
                            as *mut libc::c_float) = _467;
                        _468 = _46;
                        *(&mut *((*(_464 as *mut l_array_4_float)).array)
                            .as_mut_ptr()
                            .offset(3 as libc::c_int as int64_t as isize)
                            as *mut libc::c_float) = _468;
                        _469 = _47;
                        *(&mut _124.field0 as *mut l_array_4_float) = _469.field0;
                        _470 = _92;
                        _471 = _114;
                        _472 = _94;
                        _473 = _117;
                        _474 = _93;
                        _475 = _119;
                        _476 = memcpy(
                            &mut *(_470 as *mut l_struct_struct_OC_vfloat4).offset((llvm_add_u32
                                as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                (llvm_add_u32
                                    as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                    (llvm_mul_u32
                                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                        _471, _472,
                                    ),
                                    (llvm_mul_u32
                                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                        _473, _474,
                                    ),
                                ),
                                _475,
                            )
                                as int32_t
                                as int64_t
                                as isize)
                                as *mut l_struct_struct_OC_vfloat4
                                as *mut libc::c_void,
                            &mut _124 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_void,
                            16 as libc::c_int as uint64_t,
                        );
                        _477 = _119;
                        _119 = llvm_add_u32(_477, 1 as libc::c_int as uint32_t);
                    }
                    _478 = _117;
                    _117 = llvm_add_u32(_478, 1 as libc::c_int as uint32_t);
                }
                _479 = _114;
                _114 = llvm_add_u32(_479, 1 as libc::c_int as uint32_t);
            }
        } else {
            _480 = _70;
            _481 =
                *(&mut (*(_480 as *mut l_struct_struct_OC_astcenc_image)).field3 as *mut uint32_t);
            if _481 == 2 as libc::c_uint {
                *(&mut *(_127.array)
                    .as_mut_ptr()
                    .offset(4 as libc::c_int as int64_t as isize)
                    as *mut libc::c_float) = 0 as libc::c_int as libc::c_float;
                *(&mut *(_127.array)
                    .as_mut_ptr()
                    .offset(5 as libc::c_int as int64_t as isize)
                    as *mut libc::c_float) = 1 as libc::c_int as libc::c_float;
                _482 = _90;
                _128 = _482;
                loop {
                    _483 = _128;
                    _484 = _88;
                    if !((_483 as int32_t) < _484 as int32_t) {
                        break;
                    }
                    _485 = _128;
                    _486 = _90;
                    _487 = _78;
                    _488 = _85;
                    _129 = llvm_sub_u32(llvm_add_u32(llvm_sub_u32(_485, _486), _487), _488);
                    _489 = _129;
                    _490 = _70;
                    _491 = *(&mut (*(_490 as *mut l_struct_struct_OC_astcenc_image)).field2
                        as *mut uint32_t);
                    _492 = _ZN4astc5clampIiEET_S1_S1_S1_(
                        _489,
                        0,
                        llvm_sub_u32(_491, 1 as libc::c_int as uint32_t),
                    );
                    _129 = _492;
                    _493 = _70;
                    _494 = *(&mut (*(_493 as *mut l_struct_struct_OC_astcenc_image)).field4
                        as *mut *mut libc::c_void);
                    _495 = _129;
                    _496 = *(&mut *(_494 as *mut *mut libc::c_void)
                        .offset(_495 as int32_t as int64_t as isize)
                        as *mut *mut libc::c_void);
                    _130 = _496;
                    _131 = 1 as libc::c_int as uint32_t;
                    loop {
                        _497 = _131;
                        _498 = _87;
                        if !((_497 as int32_t) < _498 as int32_t) {
                            break;
                        }
                        _499 = _131;
                        _500 = _77;
                        _501 = _84;
                        _132 = llvm_sub_u32(
                            llvm_add_u32(llvm_sub_u32(_499, 1 as libc::c_int as uint32_t), _500),
                            _501,
                        );
                        _502 = _132;
                        _503 = _70;
                        _504 = *(&mut (*(_503 as *mut l_struct_struct_OC_astcenc_image)).field1
                            as *mut uint32_t);
                        _505 = _ZN4astc5clampIiEET_S1_S1_S1_(
                            _502,
                            0,
                            llvm_sub_u32(_504, 1 as libc::c_int as uint32_t),
                        );
                        _132 = _505;
                        _133 = 1 as libc::c_int as uint32_t;
                        loop {
                            _506 = _133;
                            _507 = _86;
                            if !((_506 as int32_t) < _507 as int32_t) {
                                break;
                            }
                            _508 = _133;
                            _509 = _76;
                            _510 = _84;
                            _134 = llvm_sub_u32(
                                llvm_add_u32(
                                    llvm_sub_u32(_508, 1 as libc::c_int as uint32_t),
                                    _509,
                                ),
                                _510,
                            );
                            _511 = _134;
                            _512 = _70;
                            _513 = *(&mut (*(_512 as *mut l_struct_struct_OC_astcenc_image)).field0
                                as *mut uint32_t);
                            _514 = _ZN4astc5clampIiEET_S1_S1_S1_(
                                _511,
                                0,
                                llvm_sub_u32(_513, 1 as libc::c_int as uint32_t),
                            );
                            _134 = _514;
                            _515 = _130;
                            _516 = _70;
                            _517 = *(&mut (*(_516 as *mut l_struct_struct_OC_astcenc_image)).field0
                                as *mut uint32_t);
                            _518 = _132;
                            _519 = _134;
                            _520 = *(&mut *(_515 as *mut libc::c_float).offset((llvm_add_u32
                                as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                (llvm_mul_u32
                                    as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                    (llvm_mul_u32
                                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                        4 as libc::c_int as uint32_t,
                                        _517,
                                    ),
                                    _518,
                                ),
                                (llvm_mul_u32
                                    as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                    4 as libc::c_int as uint32_t,
                                    _519,
                                ),
                            )
                                as uint64_t
                                as int64_t
                                as isize)
                                as *mut libc::c_float);
                            *(&mut *(_127.array)
                                .as_mut_ptr()
                                .offset(0 as libc::c_int as int64_t as isize)
                                as *mut libc::c_float) = _520;
                            _521 = _130;
                            _522 = _70;
                            _523 = *(&mut (*(_522 as *mut l_struct_struct_OC_astcenc_image)).field0
                                as *mut uint32_t);
                            _524 = _132;
                            _525 = _134;
                            _526 = *(&mut *(_521 as *mut libc::c_float).offset((llvm_add_u32
                                as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                (llvm_mul_u32
                                    as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                    (llvm_mul_u32
                                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                        4 as libc::c_int as uint32_t,
                                        _523,
                                    ),
                                    _524,
                                ),
                                (llvm_add_u32
                                    as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                    (llvm_mul_u32
                                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                        4 as libc::c_int as uint32_t,
                                        _525,
                                    ),
                                    1 as libc::c_int as uint32_t,
                                ),
                            )
                                as uint64_t
                                as int64_t
                                as isize)
                                as *mut libc::c_float);
                            *(&mut *(_127.array)
                                .as_mut_ptr()
                                .offset(1 as libc::c_int as int64_t as isize)
                                as *mut libc::c_float) = _526;
                            _527 = _130;
                            _528 = _70;
                            _529 = *(&mut (*(_528 as *mut l_struct_struct_OC_astcenc_image)).field0
                                as *mut uint32_t);
                            _530 = _132;
                            _531 = _134;
                            _532 = *(&mut *(_527 as *mut libc::c_float).offset((llvm_add_u32
                                as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                (llvm_mul_u32
                                    as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                    (llvm_mul_u32
                                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                        4 as libc::c_int as uint32_t,
                                        _529,
                                    ),
                                    _530,
                                ),
                                (llvm_add_u32
                                    as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                    (llvm_mul_u32
                                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                        4 as libc::c_int as uint32_t,
                                        _531,
                                    ),
                                    2 as libc::c_int as uint32_t,
                                ),
                            )
                                as uint64_t
                                as int64_t
                                as isize)
                                as *mut libc::c_float);
                            *(&mut *(_127.array)
                                .as_mut_ptr()
                                .offset(2 as libc::c_int as int64_t as isize)
                                as *mut libc::c_float) = _532;
                            _533 = _130;
                            _534 = _70;
                            _535 = *(&mut (*(_534 as *mut l_struct_struct_OC_astcenc_image)).field0
                                as *mut uint32_t);
                            _536 = _132;
                            _537 = _134;
                            _538 = *(&mut *(_533 as *mut libc::c_float).offset((llvm_add_u32
                                as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                (llvm_mul_u32
                                    as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                    (llvm_mul_u32
                                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                        4 as libc::c_int as uint32_t,
                                        _535,
                                    ),
                                    _536,
                                ),
                                (llvm_add_u32
                                    as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                    (llvm_mul_u32
                                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                        4 as libc::c_int as uint32_t,
                                        _537,
                                    ),
                                    3 as libc::c_int as uint32_t,
                                ),
                            )
                                as uint64_t
                                as int64_t
                                as isize)
                                as *mut libc::c_float);
                            *(&mut *(_127.array)
                                .as_mut_ptr()
                                .offset(3 as libc::c_int as int64_t as isize)
                                as *mut libc::c_float) = _538;
                            _539 = *(&mut _71.field0 as *mut uint32_t);
                            _540 = *(&mut *(_127.array)
                                .as_mut_ptr()
                                .offset(_539 as uint64_t as int64_t as isize)
                                as *mut libc::c_float);
                            _135 = _540;
                            _541 = *(&mut _71.field1 as *mut uint32_t);
                            _542 = *(&mut *(_127.array)
                                .as_mut_ptr()
                                .offset(_541 as uint64_t as int64_t as isize)
                                as *mut libc::c_float);
                            _136 = _542;
                            _543 = *(&mut _71.field2 as *mut uint32_t);
                            _544 = *(&mut *(_127.array)
                                .as_mut_ptr()
                                .offset(_543 as uint64_t as int64_t as isize)
                                as *mut libc::c_float);
                            _137 = _544;
                            _545 = *(&mut _71.field3 as *mut uint32_t);
                            _546 = *(&mut *(_127.array)
                                .as_mut_ptr()
                                .offset(_545 as uint64_t as int64_t as isize)
                                as *mut libc::c_float);
                            _138 = _546;
                            _547 = _135;
                            _548 = _136;
                            _549 = _137;
                            _550 = _138;
                            _58 = &mut _139 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_void;
                            _59 = _547;
                            _60 = _548;
                            _61 = _549;
                            _62 = _550;
                            _551 = _58;
                            _552 = _59;
                            *(_551 as *mut libc::c_float) = _552;
                            _553 = _60;
                            *(&mut *((*(_551 as *mut l_array_4_float)).array)
                                .as_mut_ptr()
                                .offset(1 as libc::c_int as int64_t as isize)
                                as *mut libc::c_float) = _553;
                            _554 = _61;
                            *(&mut *((*(_551 as *mut l_array_4_float)).array)
                                .as_mut_ptr()
                                .offset(2 as libc::c_int as int64_t as isize)
                                as *mut libc::c_float) = _554;
                            _555 = _62;
                            *(&mut *((*(_551 as *mut l_array_4_float)).array)
                                .as_mut_ptr()
                                .offset(3 as libc::c_int as int64_t as isize)
                                as *mut libc::c_float) = _555;
                            _556 = _91;
                            _557 = _128;
                            _558 = _94;
                            _559 = _131;
                            _560 = _93;
                            _561 = _133;
                            _562 = memcpy(
                                &mut *(_556 as *mut l_struct_struct_OC_vfloat4).offset(
                                    (llvm_add_u32
                                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                        (llvm_add_u32
                                            as unsafe extern "C" fn(
                                                uint32_t,
                                                uint32_t,
                                            )
                                                -> uint32_t)(
                                            (llvm_mul_u32
                                                as unsafe extern "C" fn(
                                                    uint32_t,
                                                    uint32_t,
                                                )
                                                    -> uint32_t)(
                                                _557, _558
                                            ),
                                            (llvm_mul_u32
                                                as unsafe extern "C" fn(
                                                    uint32_t,
                                                    uint32_t,
                                                )
                                                    -> uint32_t)(
                                                _559, _560
                                            ),
                                        ),
                                        _561,
                                    ) as int32_t as int64_t
                                        as isize,
                                ) as *mut l_struct_struct_OC_vfloat4
                                    as *mut libc::c_void,
                                &mut _139 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_void,
                                16 as libc::c_int as uint64_t,
                            );
                            _563 = memcpy(
                                &mut _141 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_void,
                                &mut _139 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_void,
                                16 as libc::c_int as uint64_t,
                            );
                            _564 = memcpy(
                                &mut _142 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_void,
                                &mut _139 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_void,
                                16 as libc::c_int as uint64_t,
                            );
                            _565 = *(&mut _141.field0 as *mut l_array_4_float);
                            _566 = *(&mut _142.field0 as *mut l_array_4_float);
                            *(&mut _56 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float) = _565;
                            *(&mut _57 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float) = _566;
                            _567 = *(&mut _56 as *mut l_struct_struct_OC_vfloat4
                                as *mut libc::c_float);
                            _568 = *(&mut _57 as *mut l_struct_struct_OC_vfloat4
                                as *mut libc::c_float);
                            _569 = *(&mut *((*(&mut _56 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float))
                                .array)
                                .as_mut_ptr()
                                .offset(1 as libc::c_int as int64_t as isize)
                                as *mut libc::c_float);
                            _570 = *(&mut *((*(&mut _57 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float))
                                .array)
                                .as_mut_ptr()
                                .offset(1 as libc::c_int as int64_t as isize)
                                as *mut libc::c_float);
                            _571 = *(&mut *((*(&mut _56 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float))
                                .array)
                                .as_mut_ptr()
                                .offset(2 as libc::c_int as int64_t as isize)
                                as *mut libc::c_float);
                            _572 = *(&mut *((*(&mut _57 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float))
                                .array)
                                .as_mut_ptr()
                                .offset(2 as libc::c_int as int64_t as isize)
                                as *mut libc::c_float);
                            _573 = *(&mut *((*(&mut _56 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float))
                                .array)
                                .as_mut_ptr()
                                .offset(3 as libc::c_int as int64_t as isize)
                                as *mut libc::c_float);
                            _574 = *(&mut *((*(&mut _57 as *mut l_struct_struct_OC_vfloat4
                                as *mut l_array_4_float))
                                .array)
                                .as_mut_ptr()
                                .offset(3 as libc::c_int as int64_t as isize)
                                as *mut libc::c_float);
                            _50 = &mut _55 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_void;
                            _51 = llvm_fmul_f32(_567, _568);
                            _52 = llvm_fmul_f32(_569, _570);
                            _53 = llvm_fmul_f32(_571, _572);
                            _54 = llvm_fmul_f32(_573, _574);
                            _575 = _50;
                            _576 = _51;
                            *(_575 as *mut libc::c_float) = _576;
                            _577 = _52;
                            *(&mut *((*(_575 as *mut l_array_4_float)).array)
                                .as_mut_ptr()
                                .offset(1 as libc::c_int as int64_t as isize)
                                as *mut libc::c_float) = _577;
                            _578 = _53;
                            *(&mut *((*(_575 as *mut l_array_4_float)).array)
                                .as_mut_ptr()
                                .offset(2 as libc::c_int as int64_t as isize)
                                as *mut libc::c_float) = _578;
                            _579 = _54;
                            *(&mut *((*(_575 as *mut l_array_4_float)).array)
                                .as_mut_ptr()
                                .offset(3 as libc::c_int as int64_t as isize)
                                as *mut libc::c_float) = _579;
                            _580 = _55;
                            *(&mut _140.field0 as *mut l_array_4_float) = _580.field0;
                            _581 = _92;
                            _582 = _128;
                            _583 = _94;
                            _584 = _131;
                            _585 = _93;
                            _586 = _133;
                            _587 = memcpy(
                                &mut *(_581 as *mut l_struct_struct_OC_vfloat4).offset(
                                    (llvm_add_u32
                                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                        (llvm_add_u32
                                            as unsafe extern "C" fn(
                                                uint32_t,
                                                uint32_t,
                                            )
                                                -> uint32_t)(
                                            (llvm_mul_u32
                                                as unsafe extern "C" fn(
                                                    uint32_t,
                                                    uint32_t,
                                                )
                                                    -> uint32_t)(
                                                _582, _583
                                            ),
                                            (llvm_mul_u32
                                                as unsafe extern "C" fn(
                                                    uint32_t,
                                                    uint32_t,
                                                )
                                                    -> uint32_t)(
                                                _584, _585
                                            ),
                                        ),
                                        _586,
                                    ) as int32_t as int64_t
                                        as isize,
                                ) as *mut l_struct_struct_OC_vfloat4
                                    as *mut libc::c_void,
                                &mut _140 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_void,
                                16 as libc::c_int as uint64_t,
                            );
                            _588 = _133;
                            _133 = llvm_add_u32(_588, 1 as libc::c_int as uint32_t);
                        }
                        _589 = _131;
                        _131 = llvm_add_u32(_589, 1 as libc::c_int as uint32_t);
                    }
                    _590 = _128;
                    _128 = llvm_add_u32(_590, 1 as libc::c_int as uint32_t);
                }
            } else {
                __assert_fail(
                    &_OC_str as *const l_array_35_uint8_t as *mut libc::c_void,
                    &_OC_str_OC_1 as *const l_array_55_uint8_t as *mut libc::c_void,
                    237 as libc::c_int as uint32_t,
                    &__PRETTY_FUNCTION___OC__Z29compute_pixel_region_varianceR16astcenc_contextiRK17pixel_region_args
                        as *const l_array_82_uint8_t as *mut libc::c_void,
                );
            }
        }
    }
    _3 = &mut _21 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_void;
    _4 = 0 as libc::c_int as libc::c_float;
    _591 = _3;
    _592 = _4;
    *(_591 as *mut libc::c_float) = _592;
    _593 = _4;
    *(&mut *((*(_591 as *mut l_array_4_float)).array)
        .as_mut_ptr()
        .offset(1 as libc::c_int as int64_t as isize) as *mut libc::c_float) = _593;
    _594 = _4;
    *(&mut *((*(_591 as *mut l_array_4_float)).array)
        .as_mut_ptr()
        .offset(2 as libc::c_int as int64_t as isize) as *mut libc::c_float) = _594;
    _595 = _4;
    *(&mut *((*(_591 as *mut l_array_4_float)).array)
        .as_mut_ptr()
        .offset(3 as libc::c_int as int64_t as isize) as *mut libc::c_float) = _595;
    _596 = _21;
    *(&mut _143.field0 as *mut l_array_4_float) = _596.field0;
    _144 = 0;
    loop {
        _597 = _144;
        _598 = _88;
        if !((_597 as int32_t) < _598 as int32_t) {
            break;
        }
        _145 = 0;
        loop {
            _599 = _145;
            _600 = _87;
            if !((_599 as int32_t) < _600 as int32_t) {
                break;
            }
            _601 = _91;
            _602 = _144;
            _603 = _94;
            _604 = _145;
            _605 = _93;
            _606 = memcpy(
                &mut *(_601 as *mut l_struct_struct_OC_vfloat4).offset((llvm_add_u32
                    as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                    (llvm_add_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            _602, _603,
                        ),
                        (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            _604, _605,
                        ),
                    ),
                    0,
                ) as int32_t
                    as int64_t
                    as isize) as *mut l_struct_struct_OC_vfloat4
                    as *mut libc::c_void,
                &mut _143 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_void,
                16 as libc::c_int as uint64_t,
            );
            _607 = _92;
            _608 = _144;
            _609 = _94;
            _610 = _145;
            _611 = _93;
            _612 = memcpy(
                &mut *(_607 as *mut l_struct_struct_OC_vfloat4).offset((llvm_add_u32
                    as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                    (llvm_add_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            _608, _609,
                        ),
                        (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            _610, _611,
                        ),
                    ),
                    0,
                ) as int32_t
                    as int64_t
                    as isize) as *mut l_struct_struct_OC_vfloat4
                    as *mut libc::c_void,
                &mut _143 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_void,
                16 as libc::c_int as uint64_t,
            );
            _613 = _145;
            _145 = llvm_add_u32(_613, 1 as libc::c_int as uint32_t);
        }
        _146 = 0;
        loop {
            _614 = _146;
            _615 = _86;
            if !((_614 as int32_t) < _615 as int32_t) {
                break;
            }
            _616 = _91;
            _617 = _144;
            _618 = _94;
            _619 = _93;
            _620 = _146;
            _621 = memcpy(
                &mut *(_616 as *mut l_struct_struct_OC_vfloat4).offset((llvm_add_u32
                    as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                    (llvm_add_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            _617, _618,
                        ),
                        (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            0, _619,
                        ),
                    ),
                    _620,
                ) as int32_t
                    as int64_t
                    as isize) as *mut l_struct_struct_OC_vfloat4
                    as *mut libc::c_void,
                &mut _143 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_void,
                16 as libc::c_int as uint64_t,
            );
            _622 = _92;
            _623 = _144;
            _624 = _94;
            _625 = _93;
            _626 = _146;
            _627 = memcpy(
                &mut *(_622 as *mut l_struct_struct_OC_vfloat4).offset((llvm_add_u32
                    as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                    (llvm_add_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            _623, _624,
                        ),
                        (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            0, _625,
                        ),
                    ),
                    _626,
                ) as int32_t
                    as int64_t
                    as isize) as *mut l_struct_struct_OC_vfloat4
                    as *mut libc::c_void,
                &mut _143 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_void,
                16 as libc::c_int as uint64_t,
            );
            _628 = _146;
            _146 = llvm_add_u32(_628, 1 as libc::c_int as uint32_t);
        }
        _629 = _144;
        _144 = llvm_add_u32(_629, 1 as libc::c_int as uint32_t);
    }
    _630 = _72;
    if _630 as libc::c_uint & 1 as libc::c_uint != 0 {
        _147 = 0;
        loop {
            _631 = _147;
            _632 = _87;
            if !((_631 as int32_t) < _632 as int32_t) {
                break;
            }
            _148 = 0;
            loop {
                _633 = _148;
                _634 = _86;
                if !((_633 as int32_t) < _634 as int32_t) {
                    break;
                }
                _635 = _91;
                _636 = _94;
                _637 = _147;
                _638 = _93;
                _639 = _148;
                _640 = memcpy(
                    &mut *(_635 as *mut l_struct_struct_OC_vfloat4).offset((llvm_add_u32
                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        (llvm_add_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                0, _636,
                            ),
                            (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                _637, _638,
                            ),
                        ),
                        _639,
                    )
                        as int32_t
                        as int64_t
                        as isize) as *mut l_struct_struct_OC_vfloat4
                        as *mut libc::c_void,
                    &mut _143 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_void,
                    16 as libc::c_int as uint64_t,
                );
                _641 = _92;
                _642 = _94;
                _643 = _147;
                _644 = _93;
                _645 = _148;
                _646 = memcpy(
                    &mut *(_641 as *mut l_struct_struct_OC_vfloat4).offset((llvm_add_u32
                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        (llvm_add_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                0, _642,
                            ),
                            (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                _643, _644,
                            ),
                        ),
                        _645,
                    )
                        as int32_t
                        as int64_t
                        as isize) as *mut l_struct_struct_OC_vfloat4
                        as *mut libc::c_void,
                    &mut _143 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_void,
                    16 as libc::c_int as uint64_t,
                );
                _647 = _148;
                _148 = llvm_add_u32(_647, 1 as libc::c_int as uint32_t);
            }
            _648 = _147;
            _147 = llvm_add_u32(_648, 1 as libc::c_int as uint32_t);
        }
    }
    _649 = _90;
    _149 = _649;
    loop {
        _650 = _149;
        _651 = _88;
        if !((_650 as int32_t) < _651 as int32_t) {
            break;
        }
        _150 = 1 as libc::c_int as uint32_t;
        loop {
            _652 = _150;
            _653 = _87;
            if !((_652 as int32_t) < _653 as int32_t) {
                break;
            }
            _654 = _91;
            _655 = _149;
            _656 = _94;
            _657 = _150;
            _658 = _93;
            _659 = _86;
            _ZL21brent_kung_prefix_sumP7vfloat4mi(
                &mut *(_654 as *mut l_struct_struct_OC_vfloat4).offset((llvm_add_u32
                    as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                    (llvm_add_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            _655, _656,
                        ),
                        (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            _657, _658,
                        ),
                    ),
                    1 as libc::c_int as uint32_t,
                ) as int32_t
                    as int64_t
                    as isize) as *mut l_struct_struct_OC_vfloat4
                    as *mut libc::c_void,
                llvm_sub_u32(_659, 1 as libc::c_int as uint32_t) as int32_t as int64_t as uint64_t,
                1 as libc::c_int as uint32_t,
            );
            _660 = _92;
            _661 = _149;
            _662 = _94;
            _663 = _150;
            _664 = _93;
            _665 = _86;
            _ZL21brent_kung_prefix_sumP7vfloat4mi(
                &mut *(_660 as *mut l_struct_struct_OC_vfloat4).offset((llvm_add_u32
                    as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                    (llvm_add_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            _661, _662,
                        ),
                        (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            _663, _664,
                        ),
                    ),
                    1 as libc::c_int as uint32_t,
                ) as int32_t
                    as int64_t
                    as isize) as *mut l_struct_struct_OC_vfloat4
                    as *mut libc::c_void,
                llvm_sub_u32(_665, 1 as libc::c_int as uint32_t) as int32_t as int64_t as uint64_t,
                1 as libc::c_int as uint32_t,
            );
            _666 = _150;
            _150 = llvm_add_u32(_666, 1 as libc::c_int as uint32_t);
        }
        _667 = _149;
        _149 = llvm_add_u32(_667, 1 as libc::c_int as uint32_t);
    }
    _668 = _90;
    _151 = _668;
    loop {
        _669 = _151;
        _670 = _88;
        if !((_669 as int32_t) < _670 as int32_t) {
            break;
        }
        _152 = 1 as libc::c_int as uint32_t;
        loop {
            _671 = _152;
            _672 = _86;
            if !((_671 as int32_t) < _672 as int32_t) {
                break;
            }
            _673 = _91;
            _674 = _151;
            _675 = _94;
            _676 = _93;
            _677 = _152;
            _678 = _87;
            _679 = _93;
            _ZL21brent_kung_prefix_sumP7vfloat4mi(
                &mut *(_673 as *mut l_struct_struct_OC_vfloat4).offset((llvm_add_u32
                    as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                    (llvm_add_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            _674, _675,
                        ),
                        (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            1 as libc::c_int as uint32_t,
                            _676,
                        ),
                    ),
                    _677,
                ) as int32_t
                    as int64_t
                    as isize) as *mut l_struct_struct_OC_vfloat4
                    as *mut libc::c_void,
                llvm_sub_u32(_678, 1 as libc::c_int as uint32_t) as int32_t as int64_t as uint64_t,
                _679,
            );
            _680 = _92;
            _681 = _151;
            _682 = _94;
            _683 = _93;
            _684 = _152;
            _685 = _87;
            _686 = _93;
            _ZL21brent_kung_prefix_sumP7vfloat4mi(
                &mut *(_680 as *mut l_struct_struct_OC_vfloat4).offset((llvm_add_u32
                    as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                    (llvm_add_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            _681, _682,
                        ),
                        (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            1 as libc::c_int as uint32_t,
                            _683,
                        ),
                    ),
                    _684,
                ) as int32_t
                    as int64_t
                    as isize) as *mut l_struct_struct_OC_vfloat4
                    as *mut libc::c_void,
                llvm_sub_u32(_685, 1 as libc::c_int as uint32_t) as int32_t as int64_t as uint64_t,
                _686,
            );
            _687 = _152;
            _152 = llvm_add_u32(_687, 1 as libc::c_int as uint32_t);
        }
        _688 = _151;
        _151 = llvm_add_u32(_688, 1 as libc::c_int as uint32_t);
    }
    _689 = _72;
    if _689 as libc::c_uint & 1 as libc::c_uint != 0 {
        _153 = 1 as libc::c_int as uint32_t;
        loop {
            _690 = _153;
            _691 = _87;
            if !((_690 as int32_t) < _691 as int32_t) {
                break;
            }
            _154 = 1 as libc::c_int as uint32_t;
            loop {
                _692 = _154;
                _693 = _86;
                if !((_692 as int32_t) < _693 as int32_t) {
                    break;
                }
                _694 = _91;
                _695 = _94;
                _696 = _153;
                _697 = _93;
                _698 = _154;
                _699 = _88;
                _700 = _94;
                _ZL21brent_kung_prefix_sumP7vfloat4mi(
                    &mut *(_694 as *mut l_struct_struct_OC_vfloat4).offset((llvm_add_u32
                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        (llvm_add_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                1 as libc::c_int as uint32_t,
                                _695,
                            ),
                            (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                _696, _697,
                            ),
                        ),
                        _698,
                    )
                        as int32_t
                        as int64_t
                        as isize) as *mut l_struct_struct_OC_vfloat4
                        as *mut libc::c_void,
                    llvm_sub_u32(_699, 1 as libc::c_int as uint32_t) as int32_t as int64_t
                        as uint64_t,
                    _700,
                );
                _701 = _92;
                _702 = _94;
                _703 = _153;
                _704 = _93;
                _705 = _154;
                _706 = _88;
                _707 = _94;
                _ZL21brent_kung_prefix_sumP7vfloat4mi(
                    &mut *(_701 as *mut l_struct_struct_OC_vfloat4).offset((llvm_add_u32
                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        (llvm_add_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                1 as libc::c_int as uint32_t,
                                _702,
                            ),
                            (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                _703, _704,
                            ),
                        ),
                        _705,
                    )
                        as int32_t
                        as int64_t
                        as isize) as *mut l_struct_struct_OC_vfloat4
                        as *mut libc::c_void,
                    llvm_sub_u32(_706, 1 as libc::c_int as uint32_t) as int32_t as int64_t
                        as uint64_t,
                    _707,
                );
                _708 = _154;
                _154 = llvm_add_u32(_708, 1 as libc::c_int as uint32_t);
            }
            _709 = _153;
            _153 = llvm_add_u32(_709, 1 as libc::c_int as uint32_t);
        }
    }
    _710 = _79;
    _155 = llvm_add_u32(
        llvm_mul_u32(2 as libc::c_int as uint32_t, _710),
        1 as libc::c_int as uint32_t,
    ) as int32_t as libc::c_float;
    _711 = _72;
    if _711 as libc::c_uint & 1 as libc::c_uint != 0 {
        _712 = _155;
        _713 = _155;
        _714 = _155;
        _156 = llvm_fdiv_f32(
            1 as libc::c_int as libc::c_float,
            llvm_fmul_f32(llvm_fmul_f32(_712, _713), _714),
        );
    } else {
        _715 = _155;
        _716 = _155;
        _156 = llvm_fdiv_f32(1 as libc::c_int as libc::c_float, llvm_fmul_f32(_715, _716));
    }
    _717 = _72;
    if _717 as libc::c_uint & 1 as libc::c_uint != 0 {
        _157 = 0;
        loop {
            _718 = _157;
            _719 = _75;
            if !((_718 as int32_t) < _719 as int32_t) {
                break;
            }
            _720 = _157;
            _721 = _85;
            _158 = llvm_add_u32(_720, _721);
            _722 = _157;
            _723 = _78;
            _159 = llvm_add_u32(_722, _723);
            _724 = _158;
            _725 = _79;
            _160 = llvm_sub_u32(_724, _725);
            _726 = _158;
            _727 = _79;
            _161 = llvm_add_u32(llvm_add_u32(_726, _727), 1 as libc::c_int as uint32_t);
            _162 = 0;
            loop {
                _728 = _162;
                _729 = _74;
                if !((_728 as int32_t) < _729 as int32_t) {
                    break;
                }
                _730 = _162;
                _731 = _84;
                _163 = llvm_add_u32(_730, _731);
                _732 = _162;
                _733 = _77;
                _164 = llvm_add_u32(_732, _733);
                _734 = _163;
                _735 = _79;
                _165 = llvm_sub_u32(_734, _735);
                _736 = _163;
                _737 = _79;
                _166 = llvm_add_u32(llvm_add_u32(_736, _737), 1 as libc::c_int as uint32_t);
                _167 = 0;
                loop {
                    _738 = _167;
                    _739 = _73;
                    if !((_738 as int32_t) < _739 as int32_t) {
                        break;
                    }
                    _740 = _167;
                    _741 = _84;
                    _168 = llvm_add_u32(_740, _741);
                    _742 = _167;
                    _743 = _76;
                    _169 = llvm_add_u32(_742, _743);
                    _744 = _168;
                    _745 = _79;
                    _170 = llvm_sub_u32(_744, _745);
                    _746 = _168;
                    _747 = _79;
                    _171 = llvm_add_u32(llvm_add_u32(_746, _747), 1 as libc::c_int as uint32_t);
                    _748 = _91;
                    _749 = _161;
                    _750 = _94;
                    _751 = _165;
                    _752 = _93;
                    _753 = _170;
                    _9 = &mut *(_748 as *mut l_struct_struct_OC_vfloat4).offset((llvm_add_u32
                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        (llvm_add_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                _749, _750,
                            ),
                            (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                _751, _752,
                            ),
                        ),
                        _753,
                    )
                        as int32_t
                        as int64_t
                        as isize) as *mut l_struct_struct_OC_vfloat4
                        as *mut libc::c_void;
                    _754 = _9;
                    _755 = *(&mut *((*(_754 as *mut l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(3 as libc::c_int as int64_t as isize)
                        as *mut libc::c_float);
                    _756 = _91;
                    _757 = _161;
                    _758 = _94;
                    _759 = _165;
                    _760 = _93;
                    _761 = _171;
                    _10 = &mut *(_756 as *mut l_struct_struct_OC_vfloat4).offset((llvm_add_u32
                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        (llvm_add_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                _757, _758,
                            ),
                            (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                _759, _760,
                            ),
                        ),
                        _761,
                    )
                        as int32_t
                        as int64_t
                        as isize) as *mut l_struct_struct_OC_vfloat4
                        as *mut libc::c_void;
                    _762 = _10;
                    _763 = *(&mut *((*(_762 as *mut l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(3 as libc::c_int as int64_t as isize)
                        as *mut libc::c_float);
                    _764 = _91;
                    _765 = _161;
                    _766 = _94;
                    _767 = _166;
                    _768 = _93;
                    _769 = _170;
                    _11 = &mut *(_764 as *mut l_struct_struct_OC_vfloat4).offset((llvm_add_u32
                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        (llvm_add_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                _765, _766,
                            ),
                            (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                _767, _768,
                            ),
                        ),
                        _769,
                    )
                        as int32_t
                        as int64_t
                        as isize) as *mut l_struct_struct_OC_vfloat4
                        as *mut libc::c_void;
                    _770 = _11;
                    _771 = *(&mut *((*(_770 as *mut l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(3 as libc::c_int as int64_t as isize)
                        as *mut libc::c_float);
                    _772 = _91;
                    _773 = _161;
                    _774 = _94;
                    _775 = _166;
                    _776 = _93;
                    _777 = _171;
                    _12 = &mut *(_772 as *mut l_struct_struct_OC_vfloat4).offset((llvm_add_u32
                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        (llvm_add_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                _773, _774,
                            ),
                            (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                _775, _776,
                            ),
                        ),
                        _777,
                    )
                        as int32_t
                        as int64_t
                        as isize) as *mut l_struct_struct_OC_vfloat4
                        as *mut libc::c_void;
                    _778 = _12;
                    _779 = *(&mut *((*(_778 as *mut l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(3 as libc::c_int as int64_t as isize)
                        as *mut libc::c_float);
                    _780 = _91;
                    _781 = _160;
                    _782 = _94;
                    _783 = _165;
                    _784 = _93;
                    _785 = _170;
                    _13 = &mut *(_780 as *mut l_struct_struct_OC_vfloat4).offset((llvm_add_u32
                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        (llvm_add_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                _781, _782,
                            ),
                            (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                _783, _784,
                            ),
                        ),
                        _785,
                    )
                        as int32_t
                        as int64_t
                        as isize) as *mut l_struct_struct_OC_vfloat4
                        as *mut libc::c_void;
                    _786 = _13;
                    _787 = *(&mut *((*(_786 as *mut l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(3 as libc::c_int as int64_t as isize)
                        as *mut libc::c_float);
                    _788 = _91;
                    _789 = _160;
                    _790 = _94;
                    _791 = _165;
                    _792 = _93;
                    _793 = _171;
                    _14 = &mut *(_788 as *mut l_struct_struct_OC_vfloat4).offset((llvm_add_u32
                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        (llvm_add_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                _789, _790,
                            ),
                            (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                _791, _792,
                            ),
                        ),
                        _793,
                    )
                        as int32_t
                        as int64_t
                        as isize) as *mut l_struct_struct_OC_vfloat4
                        as *mut libc::c_void;
                    _794 = _14;
                    _795 = *(&mut *((*(_794 as *mut l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(3 as libc::c_int as int64_t as isize)
                        as *mut libc::c_float);
                    _796 = _91;
                    _797 = _160;
                    _798 = _94;
                    _799 = _166;
                    _800 = _93;
                    _801 = _170;
                    _15 = &mut *(_796 as *mut l_struct_struct_OC_vfloat4).offset((llvm_add_u32
                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        (llvm_add_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                _797, _798,
                            ),
                            (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                _799, _800,
                            ),
                        ),
                        _801,
                    )
                        as int32_t
                        as int64_t
                        as isize) as *mut l_struct_struct_OC_vfloat4
                        as *mut libc::c_void;
                    _802 = _15;
                    _803 = *(&mut *((*(_802 as *mut l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(3 as libc::c_int as int64_t as isize)
                        as *mut libc::c_float);
                    _804 = _91;
                    _805 = _160;
                    _806 = _94;
                    _807 = _166;
                    _808 = _93;
                    _809 = _171;
                    _16 = &mut *(_804 as *mut l_struct_struct_OC_vfloat4).offset((llvm_add_u32
                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        (llvm_add_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                _805, _806,
                            ),
                            (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                                _807, _808,
                            ),
                        ),
                        _809,
                    )
                        as int32_t
                        as int64_t
                        as isize) as *mut l_struct_struct_OC_vfloat4
                        as *mut libc::c_void;
                    _810 = _16;
                    _811 = *(&mut *((*(_810 as *mut l_array_4_float)).array)
                        .as_mut_ptr()
                        .offset(3 as libc::c_int as int64_t as isize)
                        as *mut libc::c_float);
                    _172 = llvm_fsub_f32(
                        llvm_fadd_f32(llvm_fsub_f32(llvm_fsub_f32(_755, _763), _771), _779),
                        llvm_fadd_f32(llvm_fsub_f32(llvm_fsub_f32(_787, _795), _803), _811),
                    );
                    _812 = _159;
                    _813 = _96;
                    _814 = _164;
                    _815 = _95;
                    _816 = _169;
                    _173 = llvm_add_u32(
                        llvm_add_u32(llvm_mul_u32(_812, _813), llvm_mul_u32(_814, _815)),
                        _816,
                    );
                    _817 = _172;
                    _818 = _156;
                    _819 = _80;
                    _820 = _173;
                    *(&mut *(_819 as *mut libc::c_float).offset(_820 as int32_t as int64_t as isize)
                        as *mut libc::c_float) = llvm_fmul_f32(_817, _818);
                    _821 = _167;
                    _167 = llvm_add_u32(_821, 1 as libc::c_int as uint32_t);
                }
                _822 = _162;
                _162 = llvm_add_u32(_822, 1 as libc::c_int as uint32_t);
            }
            _823 = _157;
            _157 = llvm_add_u32(_823, 1 as libc::c_int as uint32_t);
        }
    } else {
        _174 = 0;
        loop {
            _824 = _174;
            _825 = _74;
            if !((_824 as int32_t) < _825 as int32_t) {
                break;
            }
            _826 = _174;
            _827 = _84;
            _175 = llvm_add_u32(_826, _827);
            _828 = _174;
            _829 = _77;
            _176 = llvm_add_u32(_828, _829);
            _830 = _175;
            _831 = _79;
            _177 = llvm_sub_u32(_830, _831);
            _832 = _175;
            _833 = _79;
            _178 = llvm_add_u32(llvm_add_u32(_832, _833), 1 as libc::c_int as uint32_t);
            _179 = 0;
            loop {
                _834 = _179;
                _835 = _73;
                if !((_834 as int32_t) < _835 as int32_t) {
                    break;
                }
                _836 = _179;
                _837 = _84;
                _180 = llvm_add_u32(_836, _837);
                _838 = _179;
                _839 = _76;
                _181 = llvm_add_u32(_838, _839);
                _840 = _180;
                _841 = _79;
                _182 = llvm_sub_u32(_840, _841);
                _842 = _180;
                _843 = _79;
                _183 = llvm_add_u32(llvm_add_u32(_842, _843), 1 as libc::c_int as uint32_t);
                _844 = _91;
                _845 = _94;
                _846 = _177;
                _847 = _93;
                _848 = _182;
                _17 = &mut *(_844 as *mut l_struct_struct_OC_vfloat4).offset((llvm_add_u32
                    as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                    (llvm_add_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            0, _845,
                        ),
                        (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            _846, _847,
                        ),
                    ),
                    _848,
                )
                    as int32_t
                    as int64_t
                    as isize) as *mut l_struct_struct_OC_vfloat4
                    as *mut libc::c_void;
                _849 = _17;
                _850 = *(&mut *((*(_849 as *mut l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as libc::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _851 = _91;
                _852 = _94;
                _853 = _177;
                _854 = _93;
                _855 = _183;
                _18 = &mut *(_851 as *mut l_struct_struct_OC_vfloat4).offset((llvm_add_u32
                    as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                    (llvm_add_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            0, _852,
                        ),
                        (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            _853, _854,
                        ),
                    ),
                    _855,
                )
                    as int32_t
                    as int64_t
                    as isize) as *mut l_struct_struct_OC_vfloat4
                    as *mut libc::c_void;
                _856 = _18;
                _857 = *(&mut *((*(_856 as *mut l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as libc::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _858 = _91;
                _859 = _94;
                _860 = _178;
                _861 = _93;
                _862 = _182;
                _19 = &mut *(_858 as *mut l_struct_struct_OC_vfloat4).offset((llvm_add_u32
                    as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                    (llvm_add_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            0, _859,
                        ),
                        (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            _860, _861,
                        ),
                    ),
                    _862,
                )
                    as int32_t
                    as int64_t
                    as isize) as *mut l_struct_struct_OC_vfloat4
                    as *mut libc::c_void;
                _863 = _19;
                _864 = *(&mut *((*(_863 as *mut l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as libc::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _865 = _91;
                _866 = _94;
                _867 = _178;
                _868 = _93;
                _869 = _183;
                _20 = &mut *(_865 as *mut l_struct_struct_OC_vfloat4).offset((llvm_add_u32
                    as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                    (llvm_add_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            0, _866,
                        ),
                        (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            _867, _868,
                        ),
                    ),
                    _869,
                )
                    as int32_t
                    as int64_t
                    as isize) as *mut l_struct_struct_OC_vfloat4
                    as *mut libc::c_void;
                _870 = _20;
                _871 = *(&mut *((*(_870 as *mut l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as libc::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _184 = llvm_fadd_f32(llvm_fsub_f32(llvm_fsub_f32(_850, _857), _864), _871);
                _872 = _176;
                _873 = _95;
                _874 = _181;
                _185 = llvm_add_u32(llvm_mul_u32(_872, _873), _874);
                _875 = _184;
                _876 = _156;
                _877 = _80;
                _878 = _185;
                *(&mut *(_877 as *mut libc::c_float).offset(_878 as int32_t as int64_t as isize)
                    as *mut libc::c_float) = llvm_fmul_f32(_875, _876);
                _879 = _179;
                _179 = llvm_add_u32(_879, 1 as libc::c_int as uint32_t);
            }
            _880 = _174;
            _174 = llvm_add_u32(_880, 1 as libc::c_int as uint32_t);
        }
    };
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn _ZN4astc5clampIiEET_S1_S1_S1_(
    mut _1006: uint32_t,
    mut _1007: uint32_t,
    mut _1008: uint32_t,
) -> uint32_t {
    let mut _1009: uint32_t = 0;
    let mut _1010: uint32_t = 0;
    let mut _1011: uint32_t = 0;
    let mut _1012: uint32_t = 0;
    let mut _1013: uint32_t = 0;
    let mut _1014: uint32_t = 0;
    let mut _1015: uint32_t = 0;
    let mut _1016: uint32_t = 0;
    let mut _1017: uint32_t = 0;
    let mut _1018: uint32_t = 0;
    let mut _1019: uint32_t = 0;
    let mut _1020: uint32_t = 0;
    _1010 = _1006;
    _1011 = _1007;
    _1012 = _1008;
    _1013 = _1010;
    _1014 = _1012;
    if _1013 as int32_t > _1014 as int32_t {
        _1015 = _1012;
        _1009 = _1015;
    } else {
        _1016 = _1010;
        _1017 = _1011;
        if _1016 as int32_t > _1017 as int32_t {
            _1018 = _1010;
            _1009 = _1018;
        } else {
            _1019 = _1011;
            _1009 = _1019;
        }
    }
    _1020 = _1009;
    return _1020;
}
#[inline(never)]
unsafe extern "C" fn _ZL21brent_kung_prefix_sumP7vfloat4mi(
    mut _1026: *mut libc::c_void,
    mut _1027: uint64_t,
    mut _1028: uint32_t,
) {
    let mut _1029: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1030: libc::c_float = 0.;
    let mut _1031: libc::c_float = 0.;
    let mut _1032: libc::c_float = 0.;
    let mut _1033: libc::c_float = 0.;
    let mut _1034: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1035: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1036: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1037: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1038: libc::c_float = 0.;
    let mut _1039: libc::c_float = 0.;
    let mut _1040: libc::c_float = 0.;
    let mut _1041: libc::c_float = 0.;
    let mut _1042: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1043: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1044: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1045: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1046: uint64_t = 0;
    let mut _1047: uint32_t = 0;
    let mut _1048: uint64_t = 0;
    let mut _1049: uint64_t = 0;
    let mut _1050: uint64_t = 0;
    let mut _1051: uint64_t = 0;
    let mut _1052: uint64_t = 0;
    let mut _1053: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1054: uint64_t = 0;
    let mut _1055: uint64_t = 0;
    let mut _1056: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1057: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1058: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1059: uint64_t = 0;
    let mut _1060: uint64_t = 0;
    let mut _1061: uint64_t = 0;
    let mut _1062: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1063: uint64_t = 0;
    let mut _1064: uint64_t = 0;
    let mut _1065: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1066: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1067: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1068: uint64_t = 0;
    let mut _1069: uint64_t = 0;
    let mut _1070: uint64_t = 0;
    let mut _1071: uint64_t = 0;
    let mut _1072: uint64_t = 0;
    let mut _1073: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1074: uint64_t = 0;
    let mut _1075: uint32_t = 0;
    let mut _1076: uint64_t = 0;
    let mut _1077: uint32_t = 0;
    let mut _1078: uint32_t = 0;
    let mut _1079: uint64_t = 0;
    let mut _1080: uint64_t = 0;
    let mut _1081: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1082: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1083: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1084: uint64_t = 0;
    let mut _1085: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1086: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1087: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1088: libc::c_float = 0.;
    let mut _1089: libc::c_float = 0.;
    let mut _1090: libc::c_float = 0.;
    let mut _1091: libc::c_float = 0.;
    let mut _1092: libc::c_float = 0.;
    let mut _1093: libc::c_float = 0.;
    let mut _1094: libc::c_float = 0.;
    let mut _1095: libc::c_float = 0.;
    let mut _1096: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1097: libc::c_float = 0.;
    let mut _1098: libc::c_float = 0.;
    let mut _1099: libc::c_float = 0.;
    let mut _1100: libc::c_float = 0.;
    let mut _1101: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1102: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1103: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1104: uint64_t = 0;
    let mut _1105: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1106: uint64_t = 0;
    let mut _1107: uint64_t = 0;
    let mut _1108: uint64_t = 0;
    let mut _1109: uint64_t = 0;
    let mut _1110: uint64_t = 0;
    let mut _1111: uint64_t = 0;
    let mut _1112: uint64_t = 0;
    let mut _1113: uint64_t = 0;
    let mut _1114: uint64_t = 0;
    let mut _1115: uint64_t = 0;
    let mut _1116: uint64_t = 0;
    let mut _1117: uint64_t = 0;
    let mut _1118: uint64_t = 0;
    let mut _1119: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1120: uint64_t = 0;
    let mut _1121: uint32_t = 0;
    let mut _1122: uint64_t = 0;
    let mut _1123: uint32_t = 0;
    let mut _1124: uint32_t = 0;
    let mut _1125: uint64_t = 0;
    let mut _1126: uint64_t = 0;
    let mut _1127: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1128: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1129: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1130: uint64_t = 0;
    let mut _1131: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1132: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1133: l_array_4_float = l_array_4_float { array: [0.; 4] };
    let mut _1134: libc::c_float = 0.;
    let mut _1135: libc::c_float = 0.;
    let mut _1136: libc::c_float = 0.;
    let mut _1137: libc::c_float = 0.;
    let mut _1138: libc::c_float = 0.;
    let mut _1139: libc::c_float = 0.;
    let mut _1140: libc::c_float = 0.;
    let mut _1141: libc::c_float = 0.;
    let mut _1142: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1143: libc::c_float = 0.;
    let mut _1144: libc::c_float = 0.;
    let mut _1145: libc::c_float = 0.;
    let mut _1146: libc::c_float = 0.;
    let mut _1147: l_struct_struct_OC_vfloat4 = l_struct_struct_OC_vfloat4 {
        field0: l_array_4_float { array: [0.; 4] },
    };
    let mut _1148: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1149: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1150: uint64_t = 0;
    let mut _1151: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1152: uint64_t = 0;
    let mut _1153: uint64_t = 0;
    _1045 = _1026;
    _1046 = _1027;
    _1047 = _1028;
    _1068 = _1046;
    if !(_1068 < 2 as libc::c_ulong) {
        _1048 = 2 as libc::c_int as uint64_t;
        _1049 = 1 as libc::c_int as uint64_t;
        loop {
            _1069 = _1048;
            _1050 = llvm_lshr_u64(_1069, 1 as libc::c_int as uint64_t);
            _1070 = _1048;
            _1051 = llvm_sub_u64(_1070, 1 as libc::c_int as uint64_t);
            _1071 = _1046;
            _1072 = _1049;
            _1052 = llvm_lshr_u64(_1071, _1072);
            _1073 = _1045;
            _1074 = _1051;
            _1075 = _1047;
            _1053 = &mut *(_1073 as *mut l_struct_struct_OC_vfloat4).offset((llvm_mul_u64
                as unsafe extern "C" fn(uint64_t, uint64_t) -> uint64_t)(
                _1074,
                _1075 as int32_t as int64_t as uint64_t,
            ) as int64_t
                as isize) as *mut l_struct_struct_OC_vfloat4
                as *mut libc::c_void;
            _1076 = _1050;
            _1077 = _1047;
            _1054 = llvm_neg_u64(
                llvm_mul_u64(_1076, _1077 as int32_t as int64_t as uint64_t) as int64_t
            );
            _1078 = _1047;
            _1079 = _1049;
            _1055 = (_1078 << _1079 as uint32_t) as int32_t as int64_t as uint64_t;
            loop {
                _1080 = _1052;
                if !(_1080 != 0 as libc::c_ulong) {
                    break;
                }
                _1081 = _1053;
                _1082 = memcpy(
                    &mut _1057 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_void,
                    _1081,
                    16 as libc::c_int as uint64_t,
                );
                _1083 = _1053;
                _1084 = _1054;
                _1085 = memcpy(
                    &mut _1058 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_void,
                    &mut *(_1083 as *mut l_struct_struct_OC_vfloat4)
                        .offset(_1084 as int64_t as isize)
                        as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_void,
                    16 as libc::c_int as uint64_t,
                );
                _1086 = *(&mut _1057.field0 as *mut l_array_4_float);
                _1087 = *(&mut _1058.field0 as *mut l_array_4_float);
                *(&mut _1035 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) = _1086;
                *(&mut _1036 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) = _1087;
                _1088 = *(&mut _1035 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_float);
                _1089 = *(&mut _1036 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_float);
                _1090 = *(&mut *((*(&mut _1035 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _1091 = *(&mut *((*(&mut _1036 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _1092 = *(&mut *((*(&mut _1035 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _1093 = *(&mut *((*(&mut _1036 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _1094 = *(&mut *((*(&mut _1035 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as libc::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _1095 = *(&mut *((*(&mut _1036 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as libc::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _1029 = &mut _1034 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_void;
                _1030 = llvm_fadd_f32(_1088, _1089);
                _1031 = llvm_fadd_f32(_1090, _1091);
                _1032 = llvm_fadd_f32(_1092, _1093);
                _1033 = llvm_fadd_f32(_1094, _1095);
                _1096 = _1029;
                _1097 = _1030;
                *(_1096 as *mut libc::c_float) = _1097;
                _1098 = _1031;
                *(&mut *((*(_1096 as *mut l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as int64_t as isize)
                    as *mut libc::c_float) = _1098;
                _1099 = _1032;
                *(&mut *((*(_1096 as *mut l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as int64_t as isize)
                    as *mut libc::c_float) = _1099;
                _1100 = _1033;
                *(&mut *((*(_1096 as *mut l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as libc::c_int as int64_t as isize)
                    as *mut libc::c_float) = _1100;
                _1101 = _1034;
                *(&mut _1056.field0 as *mut l_array_4_float) = _1101.field0;
                _1102 = _1053;
                _1103 = memcpy(
                    _1102,
                    &mut _1056 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_void,
                    16 as libc::c_int as uint64_t,
                );
                _1104 = _1055;
                _1105 = _1053;
                _1053 = &mut *(_1105 as *mut l_struct_struct_OC_vfloat4)
                    .offset(_1104 as int64_t as isize)
                    as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_void;
                _1106 = _1052;
                _1052 = llvm_add_u64(_1106, -(1 as libc::c_int) as uint64_t);
            }
            _1107 = _1049;
            _1049 = llvm_add_u64(_1107, 1 as libc::c_int as uint64_t);
            _1108 = _1048;
            _1048 = _1108 << 1 as libc::c_int;
            _1109 = _1048;
            _1110 = _1046;
            if !(_1109 <= _1110) {
                break;
            }
        }
        loop {
            _1111 = _1049;
            _1049 = llvm_sub_u64(_1111, 1 as libc::c_int as uint64_t);
            _1112 = _1048;
            _1048 = llvm_lshr_u64(_1112, 1 as libc::c_int as uint64_t);
            _1113 = _1048;
            _1059 = llvm_lshr_u64(_1113, 1 as libc::c_int as uint64_t);
            _1114 = _1059;
            _1115 = _1048;
            _1060 = llvm_sub_u64(llvm_add_u64(_1114, _1115), 1 as libc::c_int as uint64_t);
            _1116 = _1046;
            _1117 = _1059;
            _1118 = _1049;
            _1061 = llvm_lshr_u64(llvm_sub_u64(_1116, _1117), _1118);
            _1119 = _1045;
            _1120 = _1060;
            _1121 = _1047;
            _1062 = &mut *(_1119 as *mut l_struct_struct_OC_vfloat4).offset((llvm_mul_u64
                as unsafe extern "C" fn(uint64_t, uint64_t) -> uint64_t)(
                _1120,
                _1121 as int32_t as int64_t as uint64_t,
            ) as int64_t
                as isize) as *mut l_struct_struct_OC_vfloat4
                as *mut libc::c_void;
            _1122 = _1059;
            _1123 = _1047;
            _1063 = llvm_neg_u64(
                llvm_mul_u64(_1122, _1123 as int32_t as int64_t as uint64_t) as int64_t
            );
            _1124 = _1047;
            _1125 = _1049;
            _1064 = (_1124 << _1125 as uint32_t) as int32_t as int64_t as uint64_t;
            loop {
                _1126 = _1061;
                if !(_1126 != 0 as libc::c_ulong) {
                    break;
                }
                _1127 = _1062;
                _1128 = memcpy(
                    &mut _1066 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_void,
                    _1127,
                    16 as libc::c_int as uint64_t,
                );
                _1129 = _1062;
                _1130 = _1063;
                _1131 = memcpy(
                    &mut _1067 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_void,
                    &mut *(_1129 as *mut l_struct_struct_OC_vfloat4)
                        .offset(_1130 as int64_t as isize)
                        as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_void,
                    16 as libc::c_int as uint64_t,
                );
                _1132 = *(&mut _1066.field0 as *mut l_array_4_float);
                _1133 = *(&mut _1067.field0 as *mut l_array_4_float);
                *(&mut _1043 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) = _1132;
                *(&mut _1044 as *mut l_struct_struct_OC_vfloat4 as *mut l_array_4_float) = _1133;
                _1134 = *(&mut _1043 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_float);
                _1135 = *(&mut _1044 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_float);
                _1136 = *(&mut *((*(&mut _1043 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _1137 = *(&mut *((*(&mut _1044 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _1138 = *(&mut *((*(&mut _1043 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _1139 = *(&mut *((*(&mut _1044 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _1140 = *(&mut *((*(&mut _1043 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as libc::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _1141 = *(&mut *((*(&mut _1044 as *mut l_struct_struct_OC_vfloat4
                    as *mut l_array_4_float))
                    .array)
                    .as_mut_ptr()
                    .offset(3 as libc::c_int as int64_t as isize)
                    as *mut libc::c_float);
                _1037 = &mut _1042 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_void;
                _1038 = llvm_fadd_f32(_1134, _1135);
                _1039 = llvm_fadd_f32(_1136, _1137);
                _1040 = llvm_fadd_f32(_1138, _1139);
                _1041 = llvm_fadd_f32(_1140, _1141);
                _1142 = _1037;
                _1143 = _1038;
                *(_1142 as *mut libc::c_float) = _1143;
                _1144 = _1039;
                *(&mut *((*(_1142 as *mut l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as int64_t as isize)
                    as *mut libc::c_float) = _1144;
                _1145 = _1040;
                *(&mut *((*(_1142 as *mut l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as int64_t as isize)
                    as *mut libc::c_float) = _1145;
                _1146 = _1041;
                *(&mut *((*(_1142 as *mut l_array_4_float)).array)
                    .as_mut_ptr()
                    .offset(3 as libc::c_int as int64_t as isize)
                    as *mut libc::c_float) = _1146;
                _1147 = _1042;
                *(&mut _1065.field0 as *mut l_array_4_float) = _1147.field0;
                _1148 = _1062;
                _1149 = memcpy(
                    _1148,
                    &mut _1065 as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_void,
                    16 as libc::c_int as uint64_t,
                );
                _1150 = _1064;
                _1151 = _1062;
                _1062 = &mut *(_1151 as *mut l_struct_struct_OC_vfloat4)
                    .offset(_1150 as int64_t as isize)
                    as *mut l_struct_struct_OC_vfloat4 as *mut libc::c_void;
                _1152 = _1061;
                _1061 = llvm_add_u64(_1152, -(1 as libc::c_int) as uint64_t);
            }
            _1153 = _1048;
            if !(_1153 > 2 as libc::c_ulong) {
                break;
            }
        }
    }
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn _Z21init_compute_averagesRK13astcenc_imagejRK15astcenc_swizzleR8avg_args(
    mut _1168: *mut libc::c_void,
    mut _1169: uint32_t,
    mut _1170: *mut libc::c_void,
    mut _1171: *mut libc::c_void,
) -> uint32_t {
    let mut _1172: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1173: uint32_t = 0;
    let mut _1174: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1175: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1176: uint32_t = 0;
    let mut _1177: uint32_t = 0;
    let mut _1178: uint32_t = 0;
    let mut _1179: uint32_t = 0;
    let mut _1180: uint32_t = 0;
    let mut _1181: uint8_t = 0;
    let mut _1182: uint32_t = 0;
    let mut _1183: uint32_t = 0;
    let mut _1184: uint32_t = 0;
    let mut _1185: uint32_t = 0;
    let mut _1186: uint32_t = 0;
    let mut _1187: uint32_t = 0;
    let mut _1188: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1189: uint32_t = 0;
    let mut _1190: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1191: uint32_t = 0;
    let mut _1192: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1193: uint32_t = 0;
    let mut _1194: uint32_t = 0;
    let mut _1195: uint32_t = 0;
    let mut _1196: uint32_t = 0;
    let mut _1197: uint8_t = 0;
    let mut _1198: bool_0 = 0;
    let mut _1199: uint64_t = 0;
    let mut _1200: uint32_t = 0;
    let mut _1201: uint8_t = 0;
    let mut _1202: bool_0 = 0;
    let mut _1203: uint64_t = 0;
    let mut _1204: uint32_t = 0;
    let mut _1205: uint32_t = 0;
    let mut _1206: uint32_t = 0;
    let mut _1207: uint32_t = 0;
    let mut _1208: uint8_t = 0;
    let mut _1209: uint32_t = 0;
    let mut _1210: uint32_t = 0;
    let mut _1210__PHI_TEMPORARY: uint32_t = 0;
    let mut _1211: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1212: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1213: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1214: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1215: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1216: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1217: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1218: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1219: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1220: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1221: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1222: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1223: uint8_t = 0;
    let mut _1224: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1225: uint32_t = 0;
    let mut _1226: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1227: uint32_t = 0;
    let mut _1228: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1229: uint32_t = 0;
    let mut _1230: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1231: uint32_t = 0;
    let mut _1232: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1233: uint32_t = 0;
    let mut _1234: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1235: uint32_t = 0;
    let mut _1236: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1237: uint32_t = 0;
    let mut _1238: uint32_t = 0;
    let mut _1239: uint32_t = 0;
    let mut _1240: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _1241: uint32_t = 0;
    let mut _1242: uint32_t = 0;
    let mut _1243: uint32_t = 0;
    let mut _1244: uint32_t = 0;
    let mut _1245: uint32_t = 0;
    let mut _1246: uint32_t = 0;
    let mut _1247: uint32_t = 0;
    let mut _1248: uint32_t = 0;
    _1172 = _1168;
    _1173 = _1169;
    _1174 = _1170;
    _1175 = _1171;
    _1188 = _1172;
    _1189 = *(&mut (*(_1188 as *mut l_struct_struct_OC_astcenc_image)).field0 as *mut uint32_t);
    _1176 = _1189;
    _1190 = _1172;
    _1191 = *(&mut (*(_1190 as *mut l_struct_struct_OC_astcenc_image)).field1 as *mut uint32_t);
    _1177 = _1191;
    _1192 = _1172;
    _1193 = *(&mut (*(_1192 as *mut l_struct_struct_OC_astcenc_image)).field2 as *mut uint32_t);
    _1178 = _1193;
    _1194 = _1173;
    _1179 = _1194;
    _1195 = _1179;
    _1180 = llvm_add_u32(
        llvm_mul_u32(2 as libc::c_int as uint32_t, _1195),
        1 as libc::c_int as uint32_t,
    );
    _1196 = _1178;
    _1181 = (_1196 > 1 as libc::c_uint) as libc::c_int as bool_0;
    _1197 = _1181;
    _1198 = (_1197 as libc::c_uint & 1 as libc::c_uint) as bool_0;
    _1199 = _1198 as uint64_t;
    _1182 = llvm_select_u32(
        _1198,
        16 as libc::c_int as uint32_t,
        32 as libc::c_int as uint32_t,
    );
    _1200 = _1178;
    _1201 = _1181;
    _1202 = (_1201 as libc::c_uint & 1 as libc::c_uint) as bool_0;
    _1203 = _1202 as uint64_t;
    _1204 = _ZN4astcL3minIjEET_S1_S1_(
        _1200,
        llvm_select_u32(
            _1202,
            16 as libc::c_int as uint32_t,
            1 as libc::c_int as uint32_t,
        ),
    );
    _1183 = _1204;
    _1205 = _1182;
    _1206 = _1180;
    _1184 = llvm_add_u32(_1205, _1206);
    _1207 = _1183;
    _1208 = _1181;
    if _1208 as libc::c_uint & 1 as libc::c_uint != 0 {
        _1209 = _1180;
        _1210__PHI_TEMPORARY = _1209;
    } else {
        _1210__PHI_TEMPORARY = 0;
    }
    _1210 = _1210__PHI_TEMPORARY;
    _1185 = llvm_add_u32(_1207, _1210);
    _1211 = _1175;
    *(&mut (*(&mut (*(_1211 as *mut l_struct_struct_OC_avg_args)).field0
        as *mut l_struct_struct_OC_pixel_region_args))
        .field4 as *mut uint32_t) = 0;
    _1212 = _1175;
    *(&mut (*(&mut (*(_1212 as *mut l_struct_struct_OC_avg_args)).field0
        as *mut l_struct_struct_OC_pixel_region_args))
        .field5 as *mut uint32_t) = 0;
    _1213 = _1175;
    *(&mut (*(&mut (*(_1213 as *mut l_struct_struct_OC_avg_args)).field0
        as *mut l_struct_struct_OC_pixel_region_args))
        .field6 as *mut uint32_t) = 0;
    _1214 = _1175;
    *(&mut (*(&mut (*(_1214 as *mut l_struct_struct_OC_avg_args)).field0
        as *mut l_struct_struct_OC_pixel_region_args))
        .field7 as *mut uint32_t) = 0;
    _1215 = _1175;
    *(&mut (*(&mut (*(_1215 as *mut l_struct_struct_OC_avg_args)).field0
        as *mut l_struct_struct_OC_pixel_region_args))
        .field8 as *mut uint32_t) = 0;
    _1216 = _1175;
    *(&mut (*(&mut (*(_1216 as *mut l_struct_struct_OC_avg_args)).field0
        as *mut l_struct_struct_OC_pixel_region_args))
        .field9 as *mut uint32_t) = 0;
    _1217 = _1175;
    let ref mut fresh0 = *(&mut (*(&mut (*(_1217 as *mut l_struct_struct_OC_avg_args)).field0
        as *mut l_struct_struct_OC_pixel_region_args))
        .field10 as *mut *mut libc::c_void);
    *fresh0 = 0 as *mut libc::c_void;
    _1218 = _1172;
    _1219 = _1175;
    let ref mut fresh1 = *(&mut (*(&mut (*(_1219 as *mut l_struct_struct_OC_avg_args)).field0
        as *mut l_struct_struct_OC_pixel_region_args))
        .field0 as *mut *mut libc::c_void);
    *fresh1 = _1218;
    _1220 = _1174;
    _1221 = _1175;
    _1222 = memcpy(
        &mut (*(&mut (*(_1221 as *mut l_struct_struct_OC_avg_args)).field0
            as *mut l_struct_struct_OC_pixel_region_args))
            .field1 as *mut l_struct_struct_OC_astcenc_swizzle as *mut libc::c_void,
        _1220,
        16 as libc::c_int as uint64_t,
    );
    _1223 = _1181;
    _1224 = _1175;
    *(&mut (*(&mut (*(_1224 as *mut l_struct_struct_OC_avg_args)).field0
        as *mut l_struct_struct_OC_pixel_region_args))
        .field2 as *mut uint8_t) = (_1223 as libc::c_uint & 1 as libc::c_uint) as bool_0;
    _1225 = _1173;
    _1226 = _1175;
    *(&mut (*(&mut (*(_1226 as *mut l_struct_struct_OC_avg_args)).field0
        as *mut l_struct_struct_OC_pixel_region_args))
        .field3 as *mut uint32_t) = _1225;
    _1227 = _1176;
    _1228 = _1175;
    *(&mut (*(_1228 as *mut l_struct_struct_OC_avg_args)).field1 as *mut uint32_t) = _1227;
    _1229 = _1177;
    _1230 = _1175;
    *(&mut (*(_1230 as *mut l_struct_struct_OC_avg_args)).field2 as *mut uint32_t) = _1229;
    _1231 = _1178;
    _1232 = _1175;
    *(&mut (*(_1232 as *mut l_struct_struct_OC_avg_args)).field3 as *mut uint32_t) = _1231;
    _1233 = _1182;
    _1234 = _1175;
    *(&mut (*(_1234 as *mut l_struct_struct_OC_avg_args)).field4 as *mut uint32_t) = _1233;
    _1235 = _1183;
    _1236 = _1175;
    *(&mut (*(_1236 as *mut l_struct_struct_OC_avg_args)).field5 as *mut uint32_t) = _1235;
    _1237 = _1184;
    _1238 = _1184;
    _1239 = _1185;
    _1240 = _1175;
    *(&mut (*(_1240 as *mut l_struct_struct_OC_avg_args)).field6 as *mut uint32_t) = llvm_mul_u32(
        llvm_mul_u32(llvm_mul_u32(2 as libc::c_int as uint32_t, _1237), _1238),
        _1239,
    );
    _1241 = _1178;
    _1242 = _1183;
    _1243 = _1183;
    _1186 = llvm_udiv_u32(
        llvm_sub_u32(llvm_add_u32(_1241, _1242), 1 as libc::c_int as uint32_t),
        _1243,
    );
    _1244 = _1177;
    _1245 = _1182;
    _1246 = _1182;
    _1187 = llvm_udiv_u32(
        llvm_sub_u32(llvm_add_u32(_1244, _1245), 1 as libc::c_int as uint32_t),
        _1246,
    );
    _1247 = _1186;
    _1248 = _1187;
    return llvm_mul_u32(_1247, _1248);
}
#[inline(never)]
unsafe extern "C" fn _ZN4astcL3minIjEET_S1_S1_(
    mut _1252: uint32_t,
    mut _1253: uint32_t,
) -> uint32_t {
    let mut _1254: uint32_t = 0;
    let mut _1255: uint32_t = 0;
    let mut _1256: uint32_t = 0;
    let mut _1257: uint32_t = 0;
    let mut _1258: uint32_t = 0;
    let mut _1259: uint32_t = 0;
    let mut _1260: uint32_t = 0;
    let mut _1260__PHI_TEMPORARY: uint32_t = 0;
    _1254 = _1252;
    _1255 = _1253;
    _1256 = _1254;
    _1257 = _1255;
    if _1256 < _1257 {
        _1258 = _1254;
        _1260__PHI_TEMPORARY = _1258;
    } else {
        _1259 = _1255;
        _1260__PHI_TEMPORARY = _1259;
    }
    _1260 = _1260__PHI_TEMPORARY;
    return _1260;
}
