use ::libc;
extern "C" {
    fn _Znam(_662: uint64_t) -> *mut libc::c_void;
    fn _ZdaPv(_663: *mut libc::c_void);
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
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
    pub array: [libc::c_float; 216],
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
    pub array: [libc::c_float; 64],
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
pub struct l_array_1024_uint8_t {
    pub array: [uint8_t; 1024],
}
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
unsafe extern "C" fn llvm_udiv_u32(mut a: uint32_t, mut b: uint32_t) -> uint32_t {
    let mut r: uint32_t = a / b;
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

#[inline(never)]
pub unsafe extern "C" fn _Z21init_partition_tablesR21block_size_descriptorbj(
    mut _1: *mut libc::c_void,
    mut _2: bool_0,
    mut _3: uint32_t,
) {
    let mut _4: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _5: uint8_t = 0;
    let mut _6: uint32_t = 0;
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
    let mut _18: bool_0 = 0;
    let mut _19: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _20: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _21: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _22: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _23: uint8_t = 0;
    let mut _24: uint32_t = 0;
    let mut _25: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _26: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _27: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _28: uint8_t = 0;
    let mut _29: uint32_t = 0;
    let mut _30: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _31: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _32: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _33: uint8_t = 0;
    let mut _34: uint32_t = 0;
    let mut _35: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _36: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _37: *mut libc::c_void = 0 as *mut libc::c_void;
    _4 = _1;
    _5 = _2;
    _6 = _3;
    _12 = _4;
    _7 = &mut *((*(&mut (*(_12 as *mut l_struct_struct_OC_block_size_descriptor)).field17
        as *mut l_array_3073_struct_AC_l_struct_struct_OC_partition_info))
        .array)
        .as_mut_ptr()
        .offset(0 as libc::c_int as int64_t as isize)
        as *mut l_struct_struct_OC_partition_info as *mut libc::c_void;
    _13 = _7;
    _8 = &mut *(_13 as *mut l_struct_struct_OC_partition_info)
        .offset(1024 as libc::c_int as int64_t as isize)
        as *mut l_struct_struct_OC_partition_info as *mut libc::c_void;
    _14 = _8;
    _9 = &mut *(_14 as *mut l_struct_struct_OC_partition_info)
        .offset(1024 as libc::c_int as int64_t as isize)
        as *mut l_struct_struct_OC_partition_info as *mut libc::c_void;
    _15 = _9;
    _10 = &mut *(_15 as *mut l_struct_struct_OC_partition_info)
        .offset(1024 as libc::c_int as int64_t as isize)
        as *mut l_struct_struct_OC_partition_info as *mut libc::c_void;
    _16 = _4;
    _17 = _10;
    _18 = _ZL33generate_one_partition_info_entryR21block_size_descriptorjjjR14partition_info(
        _16,
        1 as libc::c_int as uint32_t,
        0,
        0,
        _17,
    );
    _19 = _4;
    *(&mut *((*(&mut (*(_19 as *mut l_struct_struct_OC_block_size_descriptor)).field11
        as *mut l_array_4_uint32_t))
        .array)
        .as_mut_ptr()
        .offset(0 as libc::c_int as int64_t as isize) as *mut uint32_t) =
        1 as libc::c_int as uint32_t;
    _20 = _4;
    *(&mut *((*(&mut (*(_20 as *mut l_struct_struct_OC_block_size_descriptor)).field12
        as *mut l_array_4_uint32_t))
        .array)
        .as_mut_ptr()
        .offset(0 as libc::c_int as int64_t as isize) as *mut uint32_t) =
        1 as libc::c_int as uint32_t;
    _21 = _Znam(57344 as libc::c_int as uint64_t);
    _11 = _21;
    _22 = _4;
    _23 = _5;
    _24 = _6;
    _25 = _7;
    _26 = _11;
    _ZL45build_partition_table_for_one_partition_countR21block_size_descriptorbjjP14partition_infoPm(
        _22,
        (_23 as libc::c_uint & 1 as libc::c_uint) as bool_0,
        _24,
        2 as libc::c_int as uint32_t,
        _25,
        _26,
    );
    _27 = _4;
    _28 = _5;
    _29 = _6;
    _30 = _8;
    _31 = _11;
    _ZL45build_partition_table_for_one_partition_countR21block_size_descriptorbjjP14partition_infoPm(
        _27,
        (_28 as libc::c_uint & 1 as libc::c_uint) as bool_0,
        _29,
        3 as libc::c_int as uint32_t,
        _30,
        _31,
    );
    _32 = _4;
    _33 = _5;
    _34 = _6;
    _35 = _9;
    _36 = _11;
    _ZL45build_partition_table_for_one_partition_countR21block_size_descriptorbjjP14partition_infoPm(
        _32,
        (_33 as libc::c_uint & 1 as libc::c_uint) as bool_0,
        _34,
        4 as libc::c_int as uint32_t,
        _35,
        _36,
    );
    _37 = _11;
    if !_37.is_null() {
        _ZdaPv(_37);
    }
}
#[inline(never)]
unsafe extern "C" fn _ZL33generate_one_partition_info_entryR21block_size_descriptorjjjR14partition_info(
    mut _40: *mut libc::c_void,
    mut _41: uint32_t,
    mut _42: uint32_t,
    mut _43: uint32_t,
    mut _44: *mut libc::c_void,
) -> bool_0 {
    let mut _45: uint32_t = 0;
    let mut _46: uint32_t = 0;
    let mut _47: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _48: uint32_t = 0;
    let mut _49: uint32_t = 0;
    let mut _50: uint32_t = 0;
    let mut _51: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _52: uint32_t = 0;
    let mut _53: uint8_t = 0;
    let mut _54: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _55: uint32_t = 0;
    let mut _56: l_array_4_uint32_t = l_array_4_uint32_t { array: [0; 4] };
    let mut _57: uint32_t = 0;
    let mut _58: uint32_t = 0;
    let mut _59: uint32_t = 0;
    let mut _60: uint8_t = 0;
    let mut _61: uint32_t = 0;
    let mut _62: uint32_t = 0;
    let mut _63: uint32_t = 0;
    let mut _64: uint32_t = 0;
    let mut _65: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _66: uint32_t = 0;
    let mut _67: uint8_t = 0;
    let mut _68: uint32_t = 0;
    let mut _69: uint32_t = 0;
    let mut _70: uint32_t = 0;
    let mut _71: uint32_t = 0;
    let mut _72: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _73: uint8_t = 0;
    let mut _74: uint32_t = 0;
    let mut _75: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _76: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _77: uint32_t = 0;
    let mut _78: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _79: uint8_t = 0;
    let mut _80: uint32_t = 0;
    let mut _81: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _82: uint8_t = 0;
    let mut _83: uint32_t = 0;
    let mut _84: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _85: uint8_t = 0;
    let mut _86: uint32_t = 0;
    let mut _87: uint32_t = 0;
    let mut _88: uint32_t = 0;
    let mut _89: uint32_t = 0;
    let mut _90: uint32_t = 0;
    let mut _91: uint8_t = 0;
    let mut _92: uint8_t = 0;
    let mut _93: uint32_t = 0;
    let mut _94: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _95: uint8_t = 0;
    let mut _96: uint8_t = 0;
    let mut _97: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _98: uint32_t = 0;
    let mut _99: uint8_t = 0;
    let mut _100: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _101: uint32_t = 0;
    let mut _102: uint32_t = 0;
    let mut _103: uint32_t = 0;
    let mut _104: uint32_t = 0;
    let mut _105: uint32_t = 0;
    let mut _106: uint32_t = 0;
    let mut _107: uint32_t = 0;
    let mut _108: uint32_t = 0;
    let mut _109: uint32_t = 0;
    let mut _110: uint32_t = 0;
    let mut _111: uint32_t = 0;
    let mut _112: uint32_t = 0;
    let mut _113: uint32_t = 0;
    let mut _114: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _115: uint32_t = 0;
    let mut _116: uint32_t = 0;
    let mut _117: uint8_t = 0;
    let mut _118: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _119: uint32_t = 0;
    let mut _120: uint32_t = 0;
    let mut _121: uint32_t = 0;
    let mut _122: uint32_t = 0;
    let mut _123: uint32_t = 0;
    let mut _124: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _125: uint32_t = 0;
    let mut _126: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _127: uint32_t = 0;
    let mut _128: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _129: uint32_t = 0;
    let mut _130: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _131: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _132: uint32_t = 0;
    let mut _133: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _134: uint32_t = 0;
    let mut _135: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _136: uint32_t = 0;
    let mut _137: uint32_t = 0;
    let mut _138: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _139: uint32_t = 0;
    let mut _140: uint32_t = 0;
    let mut _141: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _142: uint32_t = 0;
    let mut _143: uint32_t = 0;
    let mut _144: uint32_t = 0;
    let mut _145: uint32_t = 0;
    let mut _146: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _147: uint32_t = 0;
    let mut _148: uint32_t = 0;
    let mut _149: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _150: uint16_t = 0;
    let mut _151: uint32_t = 0;
    let mut _152: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _153: uint32_t = 0;
    let mut _154: uint32_t = 0;
    let mut _155: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _156: uint32_t = 0;
    let mut _157: uint32_t = 0;
    let mut _158: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _159: uint8_t = 0;
    let mut _160: uint8_t = 0;
    let mut _161: uint32_t = 0;
    let mut _162: uint32_t = 0;
    let mut _163: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _164: uint32_t = 0;
    let mut _165: uint8_t = 0;
    let mut _166: uint32_t = 0;
    let mut _167: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _168: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _169: uint32_t = 0;
    let mut _170: uint8_t = 0;
    let mut _171: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _172: uint64_t = 0;
    let mut _173: uint32_t = 0;
    let mut _174: uint8_t = 0;
    _47 = _40;
    _48 = _41;
    _49 = _42;
    _50 = _43;
    _51 = _44;
    _72 = _47;
    _73 = *(&mut (*(_72 as *mut l_struct_struct_OC_block_size_descriptor)).field3 as *mut uint8_t);
    _52 = _73 as uint32_t;
    _74 = _52;
    _53 = ((_74 as int32_t) < 32 as libc::c_uint as int32_t) as libc::c_int as bool_0;
    _75 = _51;
    _54 = &mut *((*(&mut (*(_75 as *mut l_struct_struct_OC_partition_info)).field3
        as *mut l_array_216_uint8_t))
        .array)
        .as_mut_ptr()
        .offset(0 as libc::c_int as int64_t as isize) as *mut uint8_t
        as *mut libc::c_void;
    _55 = 0;
    _76 = memset(
        &mut _56 as *mut l_array_4_uint32_t as *mut libc::c_void,
        0,
        16 as libc::c_int as uint64_t,
    );
    _57 = 0;
    loop {
        _77 = _57;
        _78 = _47;
        _79 = *(&mut (*(_78 as *mut l_struct_struct_OC_block_size_descriptor)).field2
            as *mut uint8_t);
        if !(_77 < _79 as uint32_t) {
            break;
        }
        _58 = 0;
        loop {
            _80 = _58;
            _81 = _47;
            _82 = *(&mut (*(_81 as *mut l_struct_struct_OC_block_size_descriptor)).field1
                as *mut uint8_t);
            if !(_80 < _82 as uint32_t) {
                break;
            }
            _59 = 0;
            loop {
                _83 = _59;
                _84 = _47;
                _85 = *(&mut (*(_84 as *mut l_struct_struct_OC_block_size_descriptor)).field0
                    as *mut uint8_t);
                if !(_83 < _85 as uint32_t) {
                    break;
                }
                _86 = _49;
                _87 = _59;
                _88 = _58;
                _89 = _57;
                _90 = _48;
                _91 = _53;
                _92 = _ZL16select_partitioniiiiib(
                    _86,
                    _87,
                    _88,
                    _89,
                    _90,
                    (_91 as libc::c_uint & 1 as libc::c_uint) as bool_0,
                );
                _60 = _92;
                _93 = _55;
                _55 = llvm_add_u32(_93, 1 as libc::c_int as uint32_t);
                _94 = _51;
                _95 = _60;
                _96 = _60;
                _97 = &mut *(_56.array)
                    .as_mut_ptr()
                    .offset(_96 as uint64_t as int64_t as isize)
                    as *mut uint32_t as *mut libc::c_void;
                _98 = *(_97 as *mut uint32_t);
                *(_97 as *mut uint32_t) = llvm_add_u32(_98, 1 as libc::c_int as uint32_t);
                *(&mut *((*(&mut *((*(&mut (*(_94
                    as *mut l_struct_struct_OC_partition_info))
                    .field4 as *mut l_array_4_struct_AC_l_array_216_uint8_t))
                    .array)
                    .as_mut_ptr()
                    .offset(_95 as uint64_t as int64_t as isize)
                    as *mut l_array_216_uint8_t))
                    .array)
                    .as_mut_ptr()
                    .offset(_98 as int32_t as int64_t as isize)
                    as *mut uint8_t) = _93 as uint8_t;
                _99 = _60;
                _100 = _54;
                _54 = &mut *(_100 as *mut uint8_t).offset(1 as libc::c_int as isize) as *mut uint8_t
                    as *mut libc::c_void;
                *(_100 as *mut uint8_t) = _99;
                _101 = _59;
                _59 = llvm_add_u32(_101, 1 as libc::c_int as uint32_t);
            }
            _102 = _58;
            _58 = llvm_add_u32(_102, 1 as libc::c_int as uint32_t);
        }
        _103 = _57;
        _57 = llvm_add_u32(_103, 1 as libc::c_int as uint32_t);
    }
    _61 = 0;
    loop {
        _104 = _61;
        _105 = _48;
        if !(_104 < _105) {
            break;
        }
        _106 = _61;
        _107 = *(&mut *(_56.array)
            .as_mut_ptr()
            .offset(_106 as uint64_t as int64_t as isize) as *mut uint32_t);
        _62 = _107;
        _108 = _62;
        _45 = _108;
        _109 = _45;
        _46 = llvm_udiv_u32(
            llvm_sub_u32(
                llvm_add_u32(_109, 4 as libc::c_int as uint32_t),
                1 as libc::c_int as uint32_t,
            ),
            4 as libc::c_int as uint32_t,
        );
        _110 = _46;
        _63 = llvm_mul_u32(_110, 4 as libc::c_int as uint32_t);
        _111 = _62;
        _64 = _111;
        loop {
            _112 = _64;
            _113 = _63;
            if !((_112 as int32_t) < _113 as int32_t) {
                break;
            }
            _114 = _51;
            _115 = _61;
            _116 = _62;
            _117 = *(&mut *((*(&mut *((*(&mut (*(_114 as *mut l_struct_struct_OC_partition_info))
                .field4
                as *mut l_array_4_struct_AC_l_array_216_uint8_t))
                .array)
                .as_mut_ptr()
                .offset(_115 as uint64_t as int64_t as isize)
                as *mut l_array_216_uint8_t))
                .array)
                .as_mut_ptr()
                .offset(
                    (llvm_sub_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        _116,
                        1 as libc::c_int as uint32_t,
                    ) as int32_t as int64_t as isize,
                ) as *mut uint8_t);
            _118 = _51;
            _119 = _61;
            _120 = _64;
            *(&mut *((*(&mut *((*(&mut (*(_118 as *mut l_struct_struct_OC_partition_info)).field4
                as *mut l_array_4_struct_AC_l_array_216_uint8_t))
                .array)
                .as_mut_ptr()
                .offset(_119 as uint64_t as int64_t as isize)
                as *mut l_array_216_uint8_t))
                .array)
                .as_mut_ptr()
                .offset(_120 as int32_t as int64_t as isize) as *mut uint8_t) = _117;
            _121 = _64;
            _64 = llvm_add_u32(_121, 1 as libc::c_int as uint32_t);
        }
        _122 = _61;
        _61 = llvm_add_u32(_122, 1 as libc::c_int as uint32_t);
    }
    _123 = *(&mut *(_56.array)
        .as_mut_ptr()
        .offset(0 as libc::c_int as int64_t as isize) as *mut uint32_t);
    if _123 == 0 as libc::c_uint {
        _124 = _51;
        *(&mut (*(_124 as *mut l_struct_struct_OC_partition_info)).field0 as *mut uint16_t) =
            0 as libc::c_int as uint16_t;
    } else {
        _125 = *(&mut *(_56.array)
            .as_mut_ptr()
            .offset(1 as libc::c_int as int64_t as isize) as *mut uint32_t);
        if _125 == 0 as libc::c_uint {
            _126 = _51;
            *(&mut (*(_126 as *mut l_struct_struct_OC_partition_info)).field0 as *mut uint16_t) =
                1 as libc::c_int as uint16_t;
        } else {
            _127 = *(&mut *(_56.array)
                .as_mut_ptr()
                .offset(2 as libc::c_int as int64_t as isize)
                as *mut uint32_t);
            if _127 == 0 as libc::c_uint {
                _128 = _51;
                *(&mut (*(_128 as *mut l_struct_struct_OC_partition_info)).field0
                    as *mut uint16_t) = 2 as libc::c_int as uint16_t;
            } else {
                _129 = *(&mut *(_56.array)
                    .as_mut_ptr()
                    .offset(3 as libc::c_int as int64_t as isize)
                    as *mut uint32_t);
                if _129 == 0 as libc::c_uint {
                    _130 = _51;
                    *(&mut (*(_130 as *mut l_struct_struct_OC_partition_info)).field0
                        as *mut uint16_t) = 3 as libc::c_int as uint16_t;
                } else {
                    _131 = _51;
                    *(&mut (*(_131 as *mut l_struct_struct_OC_partition_info)).field0
                        as *mut uint16_t) = 4 as libc::c_int as uint16_t;
                }
            }
        }
    }
    _132 = _49;
    _133 = _51;
    *(&mut (*(_133 as *mut l_struct_struct_OC_partition_info)).field1 as *mut uint16_t) =
        _132 as uint16_t;
    _65 = 0 as *mut libc::c_void;
    _134 = _48;
    if _134 == 2 as libc::c_uint {
        _135 = _47;
        _136 = _50;
        _65 = &mut *((*(&mut *((*(&mut (*(_135 as *mut l_struct_struct_OC_block_size_descriptor))
            .field20
            as *mut l_array_1024_struct_AC_l_array_2_uint64_t))
            .array)
            .as_mut_ptr()
            .offset(_136 as uint64_t as int64_t as isize)
            as *mut l_array_2_uint64_t))
            .array)
            .as_mut_ptr()
            .offset(0 as libc::c_int as int64_t as isize) as *mut uint64_t
            as *mut libc::c_void;
    } else {
        _137 = _48;
        if _137 == 3 as libc::c_uint {
            _138 = _47;
            _139 = _50;
            _65 = &mut *((*(&mut *((*(&mut (*(_138
                as *mut l_struct_struct_OC_block_size_descriptor))
                .field21
                as *mut l_array_1024_struct_AC_l_array_3_uint64_t))
                .array)
                .as_mut_ptr()
                .offset(_139 as uint64_t as int64_t as isize)
                as *mut l_array_3_uint64_t))
                .array)
                .as_mut_ptr()
                .offset(0 as libc::c_int as int64_t as isize) as *mut uint64_t
                as *mut libc::c_void;
        } else {
            _140 = _48;
            if _140 == 4 as libc::c_uint {
                _141 = _47;
                _142 = _50;
                _65 = &mut *((*(&mut *((*(&mut (*(_141
                    as *mut l_struct_struct_OC_block_size_descriptor))
                    .field22
                    as *mut l_array_1024_struct_AC_l_array_4_uint64_t))
                    .array)
                    .as_mut_ptr()
                    .offset(_142 as uint64_t as int64_t as isize)
                    as *mut l_array_4_uint64_t))
                    .array)
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as int64_t as isize)
                    as *mut uint64_t as *mut libc::c_void;
            }
        }
    }
    _66 = 0;
    loop {
        _143 = _66;
        if !(_143 < 4 as libc::c_uint) {
            break;
        }
        _144 = _66;
        _145 = *(&mut *(_56.array)
            .as_mut_ptr()
            .offset(_144 as uint64_t as int64_t as isize) as *mut uint32_t);
        _146 = _51;
        _147 = _66;
        *(&mut *((*(&mut (*(_146 as *mut l_struct_struct_OC_partition_info)).field2
            as *mut l_array_4_uint8_t))
            .array)
            .as_mut_ptr()
            .offset(_147 as uint64_t as int64_t as isize) as *mut uint8_t) = _145 as uint8_t;
        _148 = _66;
        _66 = llvm_add_u32(_148, 1 as libc::c_int as uint32_t);
    }
    _149 = _51;
    _150 = *(&mut (*(_149 as *mut l_struct_struct_OC_partition_info)).field0 as *mut uint16_t);
    _151 = _48;
    _67 = (_150 as uint32_t == _151) as libc::c_int as bool_0;
    _152 = _65;
    if !_152.is_null() {
        _68 = 0;
        loop {
            _153 = _68;
            _154 = _48;
            if !(_153 < _154) {
                break;
            }
            _155 = _65;
            _156 = _68;
            *(&mut *(_155 as *mut uint64_t).offset(_156 as uint64_t as int64_t as isize)
                as *mut uint64_t) = 0 as libc::c_int as uint64_t;
            _157 = _68;
            _68 = llvm_add_u32(_157, 1 as libc::c_int as uint32_t);
        }
        _158 = _47;
        _159 = *(&mut (*(_158 as *mut l_struct_struct_OC_block_size_descriptor)).field3
            as *mut uint8_t);
        _160 = _ZN4astcL3minIhEET_S1_S1_(_159, 64 as libc::c_uint as uint8_t);
        _69 = _160 as uint32_t;
        _70 = 0;
        loop {
            _161 = _70;
            _162 = _69;
            if !(_161 < _162) {
                break;
            }
            _163 = _47;
            _164 = _70;
            _165 = *(&mut *((*(&mut (*(_163 as *mut l_struct_struct_OC_block_size_descriptor))
                .field19 as *mut l_array_64_uint8_t))
                .array)
                .as_mut_ptr()
                .offset(_164 as uint64_t as int64_t as isize) as *mut uint8_t);
            _71 = _165 as uint32_t;
            _166 = _70;
            _167 = _65;
            _168 = _51;
            _169 = _71;
            _170 = *(&mut *((*(&mut (*(_168 as *mut l_struct_struct_OC_partition_info)).field3
                as *mut l_array_216_uint8_t))
                .array)
                .as_mut_ptr()
                .offset(_169 as uint64_t as int64_t as isize) as *mut uint8_t);
            _171 = &mut *(_167 as *mut uint64_t).offset(_170 as uint64_t as int64_t as isize)
                as *mut uint64_t as *mut libc::c_void;
            _172 = *(_171 as *mut uint64_t);
            *(_171 as *mut uint64_t) = _172 | ((1 as libc::c_int) << _166 as uint64_t) as uint64_t;
            _173 = _70;
            _70 = llvm_add_u32(_173, 1 as libc::c_int as uint32_t);
        }
    }
    _174 = _67;
    return (_174 as libc::c_uint & 1 as libc::c_uint) as bool_0;
}
#[inline(never)]
unsafe extern "C" fn _ZL45build_partition_table_for_one_partition_countR21block_size_descriptorbjjP14partition_infoPm(
    mut _229: *mut libc::c_void,
    mut _230: bool_0,
    mut _231: uint32_t,
    mut _232: uint32_t,
    mut _233: *mut libc::c_void,
    mut _234: *mut libc::c_void,
) {
    let mut current_block: u64;
    let mut _235: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _236: uint8_t = 0;
    let mut _237: uint32_t = 0;
    let mut _238: uint32_t = 0;
    let mut _239: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _240: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _241: uint32_t = 0;
    let mut _242: uint32_t = 0;
    let mut _243: l_array_1024_uint8_t = l_array_1024_uint8_t { array: [0; 1024] };
    let mut _244: uint32_t = 0;
    let mut _245: uint32_t = 0;
    let mut _246: uint8_t = 0;
    let mut _247: uint8_t = 0;
    let mut _248: uint32_t = 0;
    let mut _249: uint8_t = 0;
    let mut _250: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _251: uint32_t = 0;
    let mut _252: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _253: uint32_t = 0;
    let mut _254: uint8_t = 0;
    let mut _255: uint32_t = 0;
    let mut _256: uint32_t = 0;
    let mut _257: uint8_t = 0;
    let mut _258: bool_0 = 0;
    let mut _259: uint64_t = 0;
    let mut _260: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _261: uint32_t = 0;
    let mut _262: uint32_t = 0;
    let mut _263: uint32_t = 0;
    let mut _264: uint32_t = 0;
    let mut _265: uint32_t = 0;
    let mut _266: uint8_t = 0;
    let mut _267: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _268: uint32_t = 0;
    let mut _269: uint32_t = 0;
    let mut _270: uint32_t = 0;
    let mut _271: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _272: uint32_t = 0;
    let mut _273: bool_0 = 0;
    let mut _274: uint32_t = 0;
    let mut _275: uint8_t = 0;
    let mut _276: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _277: uint8_t = 0;
    let mut _278: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _279: uint32_t = 0;
    let mut _280: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _281: uint32_t = 0;
    let mut _282: uint32_t = 0;
    let mut _283: uint32_t = 0;
    let mut _284: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _285: uint32_t = 0;
    let mut _286: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _287: uint32_t = 0;
    let mut _288: bool_0 = 0;
    let mut _289: uint8_t = 0;
    let mut _290: uint32_t = 0;
    let mut _291: uint8_t = 0;
    let mut _292: uint8_t = 0;
    let mut _293: uint32_t = 0;
    let mut _294: uint32_t = 0;
    let mut _295: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _296: uint32_t = 0;
    let mut _297: uint32_t = 0;
    let mut _298: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _299: uint32_t = 0;
    let mut _300: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _301: uint32_t = 0;
    let mut _302: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _303: uint32_t = 0;
    let mut _304: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _305: uint32_t = 0;
    let mut _306: uint32_t = 0;
    let mut _307: uint32_t = 0;
    let mut _308: uint32_t = 0;
    let mut _309: uint32_t = 0;
    let mut _310: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _311: uint32_t = 0;
    let mut _312: uint32_t = 0;
    let mut _313: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _314: uint32_t = 0;
    let mut _315: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _316: uint32_t = 0;
    let mut _317: uint32_t = 0;
    let mut _318: uint32_t = 0;
    let mut _319: uint32_t = 0;
    _235 = _229;
    _236 = _230;
    _237 = _231;
    _238 = _232;
    _239 = _233;
    _240 = _234;
    _241 = 0;
    _250 = _235;
    _251 = _238;
    *(&mut *((*(&mut (*(_250 as *mut l_struct_struct_OC_block_size_descriptor)).field11
        as *mut l_array_4_uint32_t))
        .array)
        .as_mut_ptr()
        .offset(
            (llvm_sub_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                _251,
                1 as libc::c_int as uint32_t,
            ) as uint64_t as int64_t as isize,
        ) as *mut uint32_t) = 0;
    _252 = _235;
    _253 = _238;
    *(&mut *((*(&mut (*(_252 as *mut l_struct_struct_OC_block_size_descriptor)).field12
        as *mut l_array_4_uint32_t))
        .array)
        .as_mut_ptr()
        .offset(
            (llvm_sub_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                _253,
                1 as libc::c_int as uint32_t,
            ) as uint64_t as int64_t as isize,
        ) as *mut uint32_t) = 0;
    _254 = _236;
    if _254 as libc::c_uint & 1 as libc::c_uint != 0 {
        _255 = _238;
        _256 = _237;
        if _255 > _256 {
            current_block = 14172905532164118975;
        } else {
            current_block = 14398550588937055773;
        }
    } else {
        current_block = 14398550588937055773;
    }
    match current_block {
        14398550588937055773 => {
            _257 = _236;
            _258 = (_257 as libc::c_uint & 1 as libc::c_uint) as bool_0;
            _259 = _258 as uint64_t;
            _242 = llvm_select_u32(
                _258,
                1 as libc::c_int as uint32_t,
                2 as libc::c_int as uint32_t,
            );
            _260 = memset(
                &mut _243 as *mut l_array_1024_uint8_t as *mut libc::c_void,
                0,
                1024 as libc::c_int as uint64_t,
            );
            _244 = 0;
            loop {
                _261 = _244;
                _262 = _242;
                if !(_261 < _262) {
                    break;
                }
                _245 = 0;
                loop {
                    _263 = _245;
                    if !(_263 < 1024 as libc::c_uint) {
                        break;
                    }
                    _264 = _244;
                    if _264 == 1 as libc::c_uint {
                        _265 = _245;
                        _266 = *(&mut *(_243.array)
                            .as_mut_ptr()
                            .offset(_265 as uint64_t as int64_t as isize)
                            as *mut uint8_t);
                        if _266 as libc::c_int != 0 as libc::c_int as uint8_t as libc::c_int {
                            current_block = 16432741364899003899;
                        } else {
                            current_block = 16512802516614476837;
                        }
                    } else {
                        current_block = 16512802516614476837;
                    }
                    match current_block {
                        16512802516614476837 => {
                            _267 = _235;
                            _268 = _238;
                            _269 = _245;
                            _270 = _241;
                            _271 = _239;
                            _272 = _241;
                            _273 = _ZL33generate_one_partition_info_entryR21block_size_descriptorjjjR14partition_info(
                                _267,
                                _268,
                                _269,
                                _270,
                                &mut *(_271 as *mut l_struct_struct_OC_partition_info)
                                    .offset(_272 as uint64_t as int64_t as isize)
                                    as *mut l_struct_struct_OC_partition_info
                                    as *mut libc::c_void,
                            );
                            _246 = _273;
                            _274 = _244;
                            if _274 == 0 as libc::c_uint {
                                _275 = _246;
                                if _275 as libc::c_uint & 1 as libc::c_uint != 0 {
                                    current_block = 16562591362786563153;
                                } else {
                                    current_block = 16432741364899003899;
                                }
                            } else {
                                current_block = 16562591362786563153;
                            }
                            match current_block {
                                16432741364899003899 => {}
                                _ => {
                                    _276 = _235;
                                    _277 = *(&mut (*(_276
                                        as *mut l_struct_struct_OC_block_size_descriptor))
                                        .field3
                                        as *mut uint8_t);
                                    _278 = _239;
                                    _279 = _241;
                                    _280 = _240;
                                    _281 = _241;
                                    _ZL31generate_canonical_partitioningjPKhPm(
                                        _277 as uint32_t,
                                        &mut *((*(&mut (*(&mut *(_278
                                            as *mut l_struct_struct_OC_partition_info)
                                            .offset(_279 as uint64_t as int64_t as isize)
                                            as *mut l_struct_struct_OC_partition_info))
                                            .field3 as *mut l_array_216_uint8_t))
                                            .array)
                                            .as_mut_ptr()
                                            .offset(0 as libc::c_int as int64_t as isize)
                                            as *mut uint8_t as *mut libc::c_void,
                                        &mut *(_280 as *mut uint64_t)
                                            .offset(
                                                (llvm_mul_u32
                                                    as unsafe extern "C" fn(
                                                        uint32_t,
                                                        uint32_t,
                                                    ) -> uint32_t)(_281, 7 as libc::c_int as uint32_t)
                                                    as uint64_t as int64_t as isize,
                                            ) as *mut uint64_t as *mut libc::c_void,
                                    );
                                    _247 = 1 as libc::c_int as uint8_t;
                                    _248 = 0;
                                    loop {
                                        _282 = _248;
                                        _283 = _241;
                                        if !(_282 < _283) {
                                            current_block = 12935133098487021850;
                                            break;
                                        }
                                        _284 = _240;
                                        _285 = _241;
                                        _286 = _240;
                                        _287 = _248;
                                        _288 = _ZL31compare_canonical_partitioningsPKmS0_(
                                            &mut *(_284 as *mut uint64_t).offset((llvm_mul_u32
                                                as unsafe extern "C" fn(
                                                    uint32_t,
                                                    uint32_t,
                                                )
                                                    -> uint32_t)(
                                                _285,
                                                7 as libc::c_int as uint32_t,
                                            )
                                                as uint64_t
                                                as int64_t
                                                as isize)
                                                as *mut uint64_t
                                                as *mut libc::c_void,
                                            &mut *(_286 as *mut uint64_t).offset((llvm_mul_u32
                                                as unsafe extern "C" fn(
                                                    uint32_t,
                                                    uint32_t,
                                                )
                                                    -> uint32_t)(
                                                _287,
                                                7 as libc::c_int as uint32_t,
                                            )
                                                as uint64_t
                                                as int64_t
                                                as isize)
                                                as *mut uint64_t
                                                as *mut libc::c_void,
                                        );
                                        _249 = _288;
                                        _289 = _249;
                                        if _289 as libc::c_uint & 1 as libc::c_uint != 0 {
                                            current_block = 13851758249121041117;
                                            break;
                                        }
                                        _290 = _248;
                                        _248 = llvm_add_u32(_290, 1 as libc::c_int as uint32_t);
                                    }
                                    match current_block {
                                        13851758249121041117 => {
                                            _247 = 0 as libc::c_int as uint8_t;
                                        }
                                        _ => {}
                                    }
                                    _291 = _246;
                                    if _291 as libc::c_uint & 1 as libc::c_uint != 0 {
                                        _292 = _247;
                                        if _292 as libc::c_uint & 1 as libc::c_uint != 0 {
                                            _293 = _244;
                                            if _293 == 0 as libc::c_uint {
                                                _294 = _241;
                                                _295 = _235;
                                                _296 = _238;
                                                _297 = _245;
                                                *(&mut *((*(&mut *((*(&mut (*(_295
                                                    as *mut l_struct_struct_OC_block_size_descriptor))
                                                    .field18 as *mut l_array_3_struct_AC_l_array_1024_uint16_t))
                                                    .array)
                                                    .as_mut_ptr()
                                                    .offset(
                                                        (llvm_sub_u32
                                                            as unsafe extern "C" fn(
                                                                uint32_t,
                                                                uint32_t,
                                                            ) -> uint32_t)(_296, 2 as libc::c_int as uint32_t)
                                                            as uint64_t as int64_t as isize,
                                                    ) as *mut l_array_1024_uint16_t))
                                                    .array)
                                                    .as_mut_ptr()
                                                    .offset(_297 as uint64_t as int64_t as isize)
                                                    as *mut uint16_t) = _294 as uint16_t;
                                                _298 = _235;
                                                _299 = _238;
                                                _300 = &mut *((*(&mut (*(_298
                                                    as *mut l_struct_struct_OC_block_size_descriptor))
                                                    .field11 as *mut l_array_4_uint32_t))
                                                    .array)
                                                    .as_mut_ptr()
                                                    .offset(
                                                        (llvm_sub_u32
                                                            as unsafe extern "C" fn(
                                                                uint32_t,
                                                                uint32_t,
                                                            ) -> uint32_t)(_299, 1 as libc::c_int as uint32_t)
                                                            as uint64_t as int64_t as isize,
                                                    ) as *mut uint32_t as *mut libc::c_void;
                                                _301 = *(_300 as *mut uint32_t);
                                                *(_300 as *mut uint32_t) = llvm_add_u32(
                                                    _301,
                                                    1 as libc::c_int as uint32_t,
                                                );
                                                _302 = _235;
                                                _303 = _238;
                                                _304 = &mut *((*(&mut (*(_302
                                                    as *mut l_struct_struct_OC_block_size_descriptor))
                                                    .field12 as *mut l_array_4_uint32_t))
                                                    .array)
                                                    .as_mut_ptr()
                                                    .offset(
                                                        (llvm_sub_u32
                                                            as unsafe extern "C" fn(
                                                                uint32_t,
                                                                uint32_t,
                                                            ) -> uint32_t)(_303, 1 as libc::c_int as uint32_t)
                                                            as uint64_t as int64_t as isize,
                                                    ) as *mut uint32_t as *mut libc::c_void;
                                                _305 = *(_304 as *mut uint32_t);
                                                *(_304 as *mut uint32_t) = llvm_add_u32(
                                                    _305,
                                                    1 as libc::c_int as uint32_t,
                                                );
                                                _306 = _245;
                                                *(&mut *(_243.array)
                                                    .as_mut_ptr()
                                                    .offset(_306 as uint64_t as int64_t as isize)
                                                    as *mut uint8_t) = 1 as libc::c_int as uint8_t;
                                                _307 = _241;
                                                _241 = llvm_add_u32(
                                                    _307,
                                                    1 as libc::c_int as uint32_t,
                                                );
                                                current_block = 16432741364899003899;
                                            } else {
                                                current_block = 16432741364899003899;
                                            }
                                        } else {
                                            current_block = 17434452432861138663;
                                        }
                                    } else {
                                        current_block = 17434452432861138663;
                                    }
                                    match current_block {
                                        16432741364899003899 => {}
                                        _ => {
                                            _308 = _244;
                                            if _308 == 1 as libc::c_uint {
                                                _309 = _241;
                                                _310 = _235;
                                                _311 = _238;
                                                _312 = _245;
                                                *(&mut *((*(&mut *((*(&mut (*(_310
                                                    as *mut l_struct_struct_OC_block_size_descriptor))
                                                    .field18 as *mut l_array_3_struct_AC_l_array_1024_uint16_t))
                                                    .array)
                                                    .as_mut_ptr()
                                                    .offset(
                                                        (llvm_sub_u32
                                                            as unsafe extern "C" fn(
                                                                uint32_t,
                                                                uint32_t,
                                                            ) -> uint32_t)(_311, 2 as libc::c_int as uint32_t)
                                                            as uint64_t as int64_t as isize,
                                                    ) as *mut l_array_1024_uint16_t))
                                                    .array)
                                                    .as_mut_ptr()
                                                    .offset(_312 as uint64_t as int64_t as isize)
                                                    as *mut uint16_t) = _309 as uint16_t;
                                                _313 = _235;
                                                _314 = _238;
                                                _315 = &mut *((*(&mut (*(_313
                                                    as *mut l_struct_struct_OC_block_size_descriptor))
                                                    .field12 as *mut l_array_4_uint32_t))
                                                    .array)
                                                    .as_mut_ptr()
                                                    .offset(
                                                        (llvm_sub_u32
                                                            as unsafe extern "C" fn(
                                                                uint32_t,
                                                                uint32_t,
                                                            ) -> uint32_t)(_314, 1 as libc::c_int as uint32_t)
                                                            as uint64_t as int64_t as isize,
                                                    ) as *mut uint32_t as *mut libc::c_void;
                                                _316 = *(_315 as *mut uint32_t);
                                                *(_315 as *mut uint32_t) = llvm_add_u32(
                                                    _316,
                                                    1 as libc::c_int as uint32_t,
                                                );
                                                _317 = _241;
                                                _241 = llvm_add_u32(
                                                    _317,
                                                    1 as libc::c_int as uint32_t,
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                    _318 = _245;
                    _245 = llvm_add_u32(_318, 1 as libc::c_int as uint32_t);
                }
                _319 = _244;
                _244 = llvm_add_u32(_319, 1 as libc::c_int as uint32_t);
            }
        }
        _ => {}
    };
}
#[inline(never)]
unsafe extern "C" fn _ZL16select_partitioniiiiib(
    mut _351: uint32_t,
    mut _352: uint32_t,
    mut _353: uint32_t,
    mut _354: uint32_t,
    mut _355: uint32_t,
    mut _356: bool_0,
) -> uint8_t {
    let mut current_block: u64;
    let mut _357: uint32_t = 0;
    let mut _358: uint32_t = 0;
    let mut _359: uint32_t = 0;
    let mut _360: uint32_t = 0;
    let mut _361: uint32_t = 0;
    let mut _362: uint8_t = 0;
    let mut _363: uint32_t = 0;
    let mut _364: uint8_t = 0;
    let mut _365: uint8_t = 0;
    let mut _366: uint8_t = 0;
    let mut _367: uint8_t = 0;
    let mut _368: uint8_t = 0;
    let mut _369: uint8_t = 0;
    let mut _370: uint8_t = 0;
    let mut _371: uint8_t = 0;
    let mut _372: uint8_t = 0;
    let mut _373: uint8_t = 0;
    let mut _374: uint8_t = 0;
    let mut _375: uint8_t = 0;
    let mut _376: uint32_t = 0;
    let mut _377: uint32_t = 0;
    let mut _378: uint32_t = 0;
    let mut _379: uint32_t = 0;
    let mut _380: uint32_t = 0;
    let mut _381: uint32_t = 0;
    let mut _382: uint32_t = 0;
    let mut _383: uint8_t = 0;
    let mut _384: uint8_t = 0;
    let mut _385: uint32_t = 0;
    let mut _386: uint32_t = 0;
    let mut _387: uint32_t = 0;
    let mut _388: uint32_t = 0;
    let mut _389: uint32_t = 0;
    let mut _390: uint32_t = 0;
    let mut _391: uint32_t = 0;
    let mut _392: uint32_t = 0;
    let mut _393: uint32_t = 0;
    let mut _394: uint32_t = 0;
    let mut _395: uint32_t = 0;
    let mut _396: uint32_t = 0;
    let mut _397: uint32_t = 0;
    let mut _398: uint32_t = 0;
    let mut _399: uint32_t = 0;
    let mut _400: uint32_t = 0;
    let mut _401: uint32_t = 0;
    let mut _402: uint32_t = 0;
    let mut _403: uint32_t = 0;
    let mut _404: uint32_t = 0;
    let mut _405: uint8_t = 0;
    let mut _406: uint8_t = 0;
    let mut _407: uint8_t = 0;
    let mut _408: uint8_t = 0;
    let mut _409: uint8_t = 0;
    let mut _410: uint8_t = 0;
    let mut _411: uint8_t = 0;
    let mut _412: uint8_t = 0;
    let mut _413: uint8_t = 0;
    let mut _414: uint8_t = 0;
    let mut _415: uint8_t = 0;
    let mut _416: uint8_t = 0;
    let mut _417: uint8_t = 0;
    let mut _418: uint8_t = 0;
    let mut _419: uint8_t = 0;
    let mut _420: uint8_t = 0;
    let mut _421: uint8_t = 0;
    let mut _422: uint8_t = 0;
    let mut _423: uint8_t = 0;
    let mut _424: uint8_t = 0;
    let mut _425: uint8_t = 0;
    let mut _426: uint8_t = 0;
    let mut _427: uint8_t = 0;
    let mut _428: uint8_t = 0;
    let mut _429: uint32_t = 0;
    let mut _430: uint32_t = 0;
    let mut _431: uint64_t = 0;
    let mut _432: uint32_t = 0;
    let mut _433: uint64_t = 0;
    let mut _434: uint32_t = 0;
    let mut _435: uint64_t = 0;
    let mut _436: uint32_t = 0;
    let mut _437: uint64_t = 0;
    let mut _438: uint32_t = 0;
    let mut _439: uint32_t = 0;
    let mut _440: uint32_t = 0;
    let mut _441: uint32_t = 0;
    let mut _441__PHI_TEMPORARY: uint32_t = 0;
    let mut _442: uint32_t = 0;
    let mut _443: uint8_t = 0;
    let mut _444: uint32_t = 0;
    let mut _445: uint8_t = 0;
    let mut _446: uint32_t = 0;
    let mut _447: uint8_t = 0;
    let mut _448: uint32_t = 0;
    let mut _449: uint8_t = 0;
    let mut _450: uint32_t = 0;
    let mut _451: uint8_t = 0;
    let mut _452: uint32_t = 0;
    let mut _453: uint8_t = 0;
    let mut _454: uint32_t = 0;
    let mut _455: uint8_t = 0;
    let mut _456: uint32_t = 0;
    let mut _457: uint8_t = 0;
    let mut _458: uint32_t = 0;
    let mut _459: uint8_t = 0;
    let mut _460: uint32_t = 0;
    let mut _461: uint8_t = 0;
    let mut _462: uint32_t = 0;
    let mut _463: uint8_t = 0;
    let mut _464: uint32_t = 0;
    let mut _465: uint8_t = 0;
    let mut _466: uint8_t = 0;
    let mut _467: uint32_t = 0;
    let mut _468: uint8_t = 0;
    let mut _469: uint32_t = 0;
    let mut _470: uint8_t = 0;
    let mut _471: uint32_t = 0;
    let mut _472: uint32_t = 0;
    let mut _473: uint8_t = 0;
    let mut _474: uint32_t = 0;
    let mut _475: uint8_t = 0;
    let mut _476: uint32_t = 0;
    let mut _477: uint8_t = 0;
    let mut _478: uint32_t = 0;
    let mut _479: uint32_t = 0;
    let mut _480: uint8_t = 0;
    let mut _481: uint32_t = 0;
    let mut _482: uint8_t = 0;
    let mut _483: uint32_t = 0;
    let mut _484: uint8_t = 0;
    let mut _485: uint32_t = 0;
    let mut _486: uint32_t = 0;
    let mut _487: uint8_t = 0;
    let mut _488: uint32_t = 0;
    let mut _489: uint8_t = 0;
    let mut _490: uint32_t = 0;
    let mut _491: uint8_t = 0;
    let mut _492: uint32_t = 0;
    let mut _493: uint32_t = 0;
    let mut _494: uint32_t = 0;
    let mut _495: uint32_t = 0;
    let mut _496: uint32_t = 0;
    let mut _497: uint32_t = 0;
    let mut _498: uint32_t = 0;
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
    let mut _512: uint32_t = 0;
    let mut _513: uint8_t = 0;
    _357 = _351;
    _358 = _352;
    _359 = _353;
    _360 = _354;
    _361 = _355;
    _362 = _356;
    _384 = _362;
    if _384 as libc::c_uint & 1 as libc::c_uint != 0 {
        _385 = _358;
        _358 = _385 << 1 as libc::c_int;
        _386 = _359;
        _359 = _386 << 1 as libc::c_int;
        _387 = _360;
        _360 = _387 << 1 as libc::c_int;
    }
    _388 = _361;
    _389 = _357;
    _357 = llvm_add_u32(
        _389,
        llvm_mul_u32(
            llvm_sub_u32(_388, 1 as libc::c_int as uint32_t),
            1024 as libc::c_int as uint32_t,
        ),
    );
    _390 = _357;
    _391 = _ZL6hash52j(_390);
    _363 = _391;
    _392 = _363;
    _364 = (_392 & 15 as libc::c_int as uint32_t) as uint8_t;
    _393 = _363;
    _365 = (llvm_lshr_u32(_393, 4 as libc::c_int as uint32_t) & 15 as libc::c_int as uint32_t)
        as uint8_t;
    _394 = _363;
    _366 = (llvm_lshr_u32(_394, 8 as libc::c_int as uint32_t) & 15 as libc::c_int as uint32_t)
        as uint8_t;
    _395 = _363;
    _367 = (llvm_lshr_u32(_395, 12 as libc::c_int as uint32_t) & 15 as libc::c_int as uint32_t)
        as uint8_t;
    _396 = _363;
    _368 = (llvm_lshr_u32(_396, 16 as libc::c_int as uint32_t) & 15 as libc::c_int as uint32_t)
        as uint8_t;
    _397 = _363;
    _369 = (llvm_lshr_u32(_397, 20) & 15 as libc::c_int as uint32_t) as uint8_t;
    _398 = _363;
    _370 = (llvm_lshr_u32(_398, 24 as libc::c_int as uint32_t) & 15 as libc::c_int as uint32_t)
        as uint8_t;
    _399 = _363;
    _371 = (llvm_lshr_u32(_399, 28 as libc::c_int as uint32_t) & 15 as libc::c_int as uint32_t)
        as uint8_t;
    _400 = _363;
    _372 = (llvm_lshr_u32(_400, 18 as libc::c_int as uint32_t) & 15 as libc::c_int as uint32_t)
        as uint8_t;
    _401 = _363;
    _373 = (llvm_lshr_u32(_401, 22 as libc::c_int as uint32_t) & 15 as libc::c_int as uint32_t)
        as uint8_t;
    _402 = _363;
    _374 = (llvm_lshr_u32(_402, 26 as libc::c_int as uint32_t) & 15 as libc::c_int as uint32_t)
        as uint8_t;
    _403 = _363;
    _404 = _363;
    _375 = ((llvm_lshr_u32(_403, 30) | _404 << 2 as libc::c_int) & 15 as libc::c_int as uint32_t)
        as uint8_t;
    _405 = _364;
    _406 = _364;
    _364 = llvm_mul_u32(_406 as uint32_t, _405 as uint32_t) as uint8_t;
    _407 = _365;
    _408 = _365;
    _365 = llvm_mul_u32(_408 as uint32_t, _407 as uint32_t) as uint8_t;
    _409 = _366;
    _410 = _366;
    _366 = llvm_mul_u32(_410 as uint32_t, _409 as uint32_t) as uint8_t;
    _411 = _367;
    _412 = _367;
    _367 = llvm_mul_u32(_412 as uint32_t, _411 as uint32_t) as uint8_t;
    _413 = _368;
    _414 = _368;
    _368 = llvm_mul_u32(_414 as uint32_t, _413 as uint32_t) as uint8_t;
    _415 = _369;
    _416 = _369;
    _369 = llvm_mul_u32(_416 as uint32_t, _415 as uint32_t) as uint8_t;
    _417 = _370;
    _418 = _370;
    _370 = llvm_mul_u32(_418 as uint32_t, _417 as uint32_t) as uint8_t;
    _419 = _371;
    _420 = _371;
    _371 = llvm_mul_u32(_420 as uint32_t, _419 as uint32_t) as uint8_t;
    _421 = _372;
    _422 = _372;
    _372 = llvm_mul_u32(_422 as uint32_t, _421 as uint32_t) as uint8_t;
    _423 = _373;
    _424 = _373;
    _373 = llvm_mul_u32(_424 as uint32_t, _423 as uint32_t) as uint8_t;
    _425 = _374;
    _426 = _374;
    _374 = llvm_mul_u32(_426 as uint32_t, _425 as uint32_t) as uint8_t;
    _427 = _375;
    _428 = _375;
    _375 = llvm_mul_u32(_428 as uint32_t, _427 as uint32_t) as uint8_t;
    _429 = _357;
    if _429 & 1 as libc::c_int as uint32_t != 0 as libc::c_uint {
        _430 = _357;
        _431 = (_430 & 2 as libc::c_int as uint32_t != 0 as libc::c_uint) as libc::c_int as bool_0
            as uint64_t;
        _376 = llvm_select_u32(
            (_430 & 2 as libc::c_int as uint32_t != 0 as libc::c_uint) as libc::c_int as bool_0,
            4 as libc::c_int as uint32_t,
            5 as libc::c_int as uint32_t,
        );
        _432 = _361;
        _433 = (_432 == 3 as libc::c_uint) as libc::c_int as bool_0 as uint64_t;
        _377 = llvm_select_u32(
            (_432 == 3 as libc::c_uint) as libc::c_int as bool_0,
            6 as libc::c_int as uint32_t,
            5 as libc::c_int as uint32_t,
        );
    } else {
        _434 = _361;
        _435 = (_434 == 3 as libc::c_uint) as libc::c_int as bool_0 as uint64_t;
        _376 = llvm_select_u32(
            (_434 == 3 as libc::c_uint) as libc::c_int as bool_0,
            6 as libc::c_int as uint32_t,
            5 as libc::c_int as uint32_t,
        );
        _436 = _357;
        _437 = (_436 & 2 as libc::c_int as uint32_t != 0 as libc::c_uint) as libc::c_int as bool_0
            as uint64_t;
        _377 = llvm_select_u32(
            (_436 & 2 as libc::c_int as uint32_t != 0 as libc::c_uint) as libc::c_int as bool_0,
            4 as libc::c_int as uint32_t,
            5 as libc::c_int as uint32_t,
        );
    }
    _438 = _357;
    if _438 & 16 as libc::c_int as uint32_t != 0 as libc::c_uint {
        _439 = _376;
        _441__PHI_TEMPORARY = _439;
    } else {
        _440 = _377;
        _441__PHI_TEMPORARY = _440;
    }
    _441 = _441__PHI_TEMPORARY;
    _378 = _441;
    _442 = _376;
    _443 = _364;
    _364 = llvm_ashr_u32(_443 as uint32_t as int32_t, _442 as int32_t) as uint8_t;
    _444 = _377;
    _445 = _365;
    _365 = llvm_ashr_u32(_445 as uint32_t as int32_t, _444 as int32_t) as uint8_t;
    _446 = _376;
    _447 = _366;
    _366 = llvm_ashr_u32(_447 as uint32_t as int32_t, _446 as int32_t) as uint8_t;
    _448 = _377;
    _449 = _367;
    _367 = llvm_ashr_u32(_449 as uint32_t as int32_t, _448 as int32_t) as uint8_t;
    _450 = _376;
    _451 = _368;
    _368 = llvm_ashr_u32(_451 as uint32_t as int32_t, _450 as int32_t) as uint8_t;
    _452 = _377;
    _453 = _369;
    _369 = llvm_ashr_u32(_453 as uint32_t as int32_t, _452 as int32_t) as uint8_t;
    _454 = _376;
    _455 = _370;
    _370 = llvm_ashr_u32(_455 as uint32_t as int32_t, _454 as int32_t) as uint8_t;
    _456 = _377;
    _457 = _371;
    _371 = llvm_ashr_u32(_457 as uint32_t as int32_t, _456 as int32_t) as uint8_t;
    _458 = _378;
    _459 = _372;
    _372 = llvm_ashr_u32(_459 as uint32_t as int32_t, _458 as int32_t) as uint8_t;
    _460 = _378;
    _461 = _373;
    _373 = llvm_ashr_u32(_461 as uint32_t as int32_t, _460 as int32_t) as uint8_t;
    _462 = _378;
    _463 = _374;
    _374 = llvm_ashr_u32(_463 as uint32_t as int32_t, _462 as int32_t) as uint8_t;
    _464 = _378;
    _465 = _375;
    _375 = llvm_ashr_u32(_465 as uint32_t as int32_t, _464 as int32_t) as uint8_t;
    _466 = _364;
    _467 = _358;
    _468 = _365;
    _469 = _359;
    _470 = _374;
    _471 = _360;
    _472 = _363;
    _379 = llvm_add_u32(
        llvm_add_u32(
            llvm_add_u32(
                llvm_mul_u32(_466 as uint32_t, _467),
                llvm_mul_u32(_468 as uint32_t, _469),
            ),
            llvm_mul_u32(_470 as uint32_t, _471),
        ),
        llvm_lshr_u32(_472, 14 as libc::c_int as uint32_t),
    );
    _473 = _366;
    _474 = _358;
    _475 = _367;
    _476 = _359;
    _477 = _375;
    _478 = _360;
    _479 = _363;
    _380 = llvm_add_u32(
        llvm_add_u32(
            llvm_add_u32(
                llvm_mul_u32(_473 as uint32_t, _474),
                llvm_mul_u32(_475 as uint32_t, _476),
            ),
            llvm_mul_u32(_477 as uint32_t, _478),
        ),
        llvm_lshr_u32(_479, 10),
    );
    _480 = _368;
    _481 = _358;
    _482 = _369;
    _483 = _359;
    _484 = _372;
    _485 = _360;
    _486 = _363;
    _381 = llvm_add_u32(
        llvm_add_u32(
            llvm_add_u32(
                llvm_mul_u32(_480 as uint32_t, _481),
                llvm_mul_u32(_482 as uint32_t, _483),
            ),
            llvm_mul_u32(_484 as uint32_t, _485),
        ),
        llvm_lshr_u32(_486, 6 as libc::c_int as uint32_t),
    );
    _487 = _370;
    _488 = _358;
    _489 = _371;
    _490 = _359;
    _491 = _373;
    _492 = _360;
    _493 = _363;
    _382 = llvm_add_u32(
        llvm_add_u32(
            llvm_add_u32(
                llvm_mul_u32(_487 as uint32_t, _488),
                llvm_mul_u32(_489 as uint32_t, _490),
            ),
            llvm_mul_u32(_491 as uint32_t, _492),
        ),
        llvm_lshr_u32(_493, 2 as libc::c_int as uint32_t),
    );
    _494 = _379;
    _379 = _494 & 63 as libc::c_int as uint32_t;
    _495 = _380;
    _380 = _495 & 63 as libc::c_int as uint32_t;
    _496 = _381;
    _381 = _496 & 63 as libc::c_int as uint32_t;
    _497 = _382;
    _382 = _497 & 63 as libc::c_int as uint32_t;
    _498 = _361;
    if _498 as int32_t <= 3 as libc::c_uint as int32_t {
        _382 = 0;
    }
    _499 = _361;
    if _499 as int32_t <= 2 as libc::c_uint as int32_t {
        _381 = 0;
    }
    _500 = _361;
    if _500 as int32_t <= 1 as libc::c_uint as int32_t {
        _380 = 0;
    }
    _501 = _379;
    _502 = _380;
    if _501 as int32_t >= _502 as int32_t {
        _503 = _379;
        _504 = _381;
        if _503 as int32_t >= _504 as int32_t {
            _505 = _379;
            _506 = _382;
            if _505 as int32_t >= _506 as int32_t {
                _383 = 0 as libc::c_int as uint8_t;
                current_block = 17009346936204501982;
            } else {
                current_block = 5529794599794551815;
            }
        } else {
            current_block = 5529794599794551815;
        }
    } else {
        current_block = 5529794599794551815;
    }
    match current_block {
        5529794599794551815 => {
            _507 = _380;
            _508 = _381;
            if _507 as int32_t >= _508 as int32_t {
                _509 = _380;
                _510 = _382;
                if _509 as int32_t >= _510 as int32_t {
                    _383 = 1 as libc::c_int as uint8_t;
                    current_block = 17009346936204501982;
                } else {
                    current_block = 16697689281210591392;
                }
            } else {
                current_block = 16697689281210591392;
            }
            match current_block {
                17009346936204501982 => {}
                _ => {
                    _511 = _381;
                    _512 = _382;
                    if _511 as int32_t >= _512 as int32_t {
                        _383 = 2 as libc::c_int as uint8_t;
                    } else {
                        _383 = 3 as libc::c_int as uint8_t;
                    }
                }
            }
        }
        _ => {}
    }
    _513 = _383;
    return _513;
}
#[inline(never)]
unsafe extern "C" fn _ZN4astcL3minIhEET_S1_S1_(mut _540: uint8_t, mut _541: uint8_t) -> uint8_t {
    let mut _542: uint8_t = 0;
    let mut _543: uint8_t = 0;
    let mut _544: uint8_t = 0;
    let mut _545: uint8_t = 0;
    let mut _546: uint8_t = 0;
    let mut _547: uint8_t = 0;
    let mut _548: uint8_t = 0;
    let mut _548__PHI_TEMPORARY: uint8_t = 0;
    _542 = _540;
    _543 = _541;
    _544 = _542;
    _545 = _543;
    if (_544 as uint32_t as int32_t) < _545 as uint32_t as int32_t {
        _546 = _542;
        _548__PHI_TEMPORARY = _546;
    } else {
        _547 = _543;
        _548__PHI_TEMPORARY = _547;
    }
    _548 = _548__PHI_TEMPORARY;
    return _548;
}
#[inline(never)]
unsafe extern "C" fn _ZL6hash52j(mut _552: uint32_t) -> uint32_t {
    let mut _553: uint32_t = 0;
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
    let mut _569: uint32_t = 0;
    _553 = _552;
    _554 = _553;
    _555 = _553;
    _553 = _555 ^ llvm_lshr_u32(_554, 15 as libc::c_int as uint32_t);
    _556 = _553;
    _553 = llvm_mul_u32(_556, -(287438703 as libc::c_int) as uint32_t);
    _557 = _553;
    _558 = _553;
    _553 = _558 ^ llvm_lshr_u32(_557, 5 as libc::c_int as uint32_t);
    _559 = _553;
    _560 = _553;
    _553 = llvm_add_u32(_560, _559 << 16 as libc::c_int);
    _561 = _553;
    _562 = _553;
    _553 = _562 ^ llvm_lshr_u32(_561, 7 as libc::c_int as uint32_t);
    _563 = _553;
    _564 = _553;
    _553 = _564 ^ llvm_lshr_u32(_563, 3 as libc::c_int as uint32_t);
    _565 = _553;
    _566 = _553;
    _553 = _566 ^ _565 << 6 as libc::c_int;
    _567 = _553;
    _568 = _553;
    _553 = _568 ^ llvm_lshr_u32(_567, 17 as libc::c_int as uint32_t);
    _569 = _553;
    return _569;
}
#[inline(never)]
unsafe extern "C" fn _ZL31generate_canonical_partitioningjPKhPm(
    mut _570: uint32_t,
    mut _571: *mut libc::c_void,
    mut _572: *mut libc::c_void,
) {
    let mut _573: uint32_t = 0;
    let mut _574: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _575: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _576: uint32_t = 0;
    let mut _577: l_array_4_uint32_t = l_array_4_uint32_t { array: [0; 4] };
    let mut _578: uint32_t = 0;
    let mut _579: uint32_t = 0;
    let mut _580: uint32_t = 0;
    let mut _581: uint32_t = 0;
    let mut _582: uint64_t = 0;
    let mut _583: uint32_t = 0;
    let mut _584: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _585: uint32_t = 0;
    let mut _586: uint32_t = 0;
    let mut _587: uint32_t = 0;
    let mut _588: uint32_t = 0;
    let mut _589: uint32_t = 0;
    let mut _590: uint32_t = 0;
    let mut _591: uint32_t = 0;
    let mut _592: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _593: uint32_t = 0;
    let mut _594: uint8_t = 0;
    let mut _595: uint32_t = 0;
    let mut _596: uint32_t = 0;
    let mut _597: uint32_t = 0;
    let mut _598: uint32_t = 0;
    let mut _599: uint32_t = 0;
    let mut _600: uint32_t = 0;
    let mut _601: uint64_t = 0;
    let mut _602: uint32_t = 0;
    let mut _603: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _604: uint32_t = 0;
    let mut _605: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _606: uint64_t = 0;
    let mut _607: uint32_t = 0;
    _573 = _570;
    _574 = _571;
    _575 = _572;
    _576 = 0;
    loop {
        _583 = _576;
        if !(_583 < 7 as libc::c_uint) {
            break;
        }
        _584 = _575;
        _585 = _576;
        *(&mut *(_584 as *mut uint64_t).offset(_585 as uint64_t as int64_t as isize)
            as *mut uint64_t) = 0 as libc::c_int as uint64_t;
        _586 = _576;
        _576 = llvm_add_u32(_586, 1 as libc::c_int as uint32_t);
    }
    _578 = 0;
    _579 = 0;
    loop {
        _587 = _579;
        if !(_587 < 4 as libc::c_uint) {
            break;
        }
        _588 = _579;
        *(&mut *(_577.array)
            .as_mut_ptr()
            .offset(_588 as uint64_t as int64_t as isize) as *mut uint32_t) =
            -(1 as libc::c_int) as uint32_t;
        _589 = _579;
        _579 = llvm_add_u32(_589, 1 as libc::c_int as uint32_t);
    }
    _580 = 0;
    loop {
        _590 = _580;
        _591 = _573;
        if !(_590 < _591) {
            break;
        }
        _592 = _574;
        _593 = _580;
        _594 = *(&mut *(_592 as *mut uint8_t).offset(_593 as uint64_t as int64_t as isize)
            as *mut uint8_t);
        _581 = _594 as uint32_t;
        _595 = _581;
        _596 = *(&mut *(_577.array)
            .as_mut_ptr()
            .offset(_595 as int32_t as int64_t as isize) as *mut uint32_t);
        if (_596 as int32_t) < 0 as libc::c_uint as int32_t {
            _597 = _578;
            _578 = llvm_add_u32(_597, 1 as libc::c_int as uint32_t);
            _598 = _581;
            *(&mut *(_577.array)
                .as_mut_ptr()
                .offset(_598 as int32_t as int64_t as isize) as *mut uint32_t) = _597;
        }
        _599 = _581;
        _600 = *(&mut *(_577.array)
            .as_mut_ptr()
            .offset(_599 as int32_t as int64_t as isize) as *mut uint32_t);
        _582 = _600 as int32_t as int64_t as uint64_t;
        _601 = _582;
        _602 = _580;
        _603 = _575;
        _604 = _580;
        _605 = &mut *(_603 as *mut uint64_t).offset((llvm_lshr_u32
            as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
            _604,
            5 as libc::c_int as uint32_t,
        ) as uint64_t as int64_t as isize) as *mut uint64_t as *mut libc::c_void;
        _606 = *(_605 as *mut uint64_t);
        *(_605 as *mut uint64_t) = _606
            | _601
                << llvm_mul_u32(
                    2 as libc::c_int as uint32_t,
                    _602 & 31 as libc::c_int as uint32_t,
                ) as uint64_t;
        _607 = _580;
        _580 = llvm_add_u32(_607, 1 as libc::c_int as uint32_t);
    }
}
#[inline(never)]
unsafe extern "C" fn _ZL31compare_canonical_partitioningsPKmS0_(
    mut _622: *mut libc::c_void,
    mut _623: *mut libc::c_void,
) -> bool_0 {
    let mut _624: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _625: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _626: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _627: uint64_t = 0;
    let mut _628: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _629: uint64_t = 0;
    let mut _630: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _631: uint64_t = 0;
    let mut _632: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _633: uint64_t = 0;
    let mut _634: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _635: uint64_t = 0;
    let mut _636: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _637: uint64_t = 0;
    let mut _638: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _639: uint64_t = 0;
    let mut _640: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _641: uint64_t = 0;
    let mut _642: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _643: uint64_t = 0;
    let mut _644: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _645: uint64_t = 0;
    let mut _646: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _647: uint64_t = 0;
    let mut _648: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _649: uint64_t = 0;
    let mut _650: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _651: uint64_t = 0;
    let mut _652: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _653: uint64_t = 0;
    let mut _654: bool_0 = 0;
    let mut _654__PHI_TEMPORARY: bool_0 = 0;
    _624 = _622;
    _625 = _623;
    _626 = _624;
    _627 = *(_626 as *mut uint64_t);
    _628 = _625;
    _629 = *(_628 as *mut uint64_t);
    if _627 == _629 {
        _630 = _624;
        _631 = *(&mut *(_630 as *mut uint64_t).offset(1 as libc::c_int as int64_t as isize)
            as *mut uint64_t);
        _632 = _625;
        _633 = *(&mut *(_632 as *mut uint64_t).offset(1 as libc::c_int as int64_t as isize)
            as *mut uint64_t);
        if _631 == _633 {
            _634 = _624;
            _635 = *(&mut *(_634 as *mut uint64_t).offset(2 as libc::c_int as int64_t as isize)
                as *mut uint64_t);
            _636 = _625;
            _637 = *(&mut *(_636 as *mut uint64_t).offset(2 as libc::c_int as int64_t as isize)
                as *mut uint64_t);
            if _635 == _637 {
                _638 = _624;
                _639 = *(&mut *(_638 as *mut uint64_t).offset(3 as libc::c_int as int64_t as isize)
                    as *mut uint64_t);
                _640 = _625;
                _641 = *(&mut *(_640 as *mut uint64_t).offset(3 as libc::c_int as int64_t as isize)
                    as *mut uint64_t);
                if _639 == _641 {
                    _642 = _624;
                    _643 = *(&mut *(_642 as *mut uint64_t)
                        .offset(4 as libc::c_int as int64_t as isize)
                        as *mut uint64_t);
                    _644 = _625;
                    _645 = *(&mut *(_644 as *mut uint64_t)
                        .offset(4 as libc::c_int as int64_t as isize)
                        as *mut uint64_t);
                    if _643 == _645 {
                        _646 = _624;
                        _647 = *(&mut *(_646 as *mut uint64_t)
                            .offset(5 as libc::c_int as int64_t as isize)
                            as *mut uint64_t);
                        _648 = _625;
                        _649 = *(&mut *(_648 as *mut uint64_t)
                            .offset(5 as libc::c_int as int64_t as isize)
                            as *mut uint64_t);
                        if _647 == _649 {
                            _650 = _624;
                            _651 = *(&mut *(_650 as *mut uint64_t)
                                .offset(6 as libc::c_int as int64_t as isize)
                                as *mut uint64_t);
                            _652 = _625;
                            _653 = *(&mut *(_652 as *mut uint64_t)
                                .offset(6 as libc::c_int as int64_t as isize)
                                as *mut uint64_t);
                            _654__PHI_TEMPORARY = (_651 == _653) as libc::c_int as bool_0;
                        } else {
                            _654__PHI_TEMPORARY = 0 as libc::c_int as bool_0;
                        }
                    } else {
                        _654__PHI_TEMPORARY = 0 as libc::c_int as bool_0;
                    }
                } else {
                    _654__PHI_TEMPORARY = 0 as libc::c_int as bool_0;
                }
            } else {
                _654__PHI_TEMPORARY = 0 as libc::c_int as bool_0;
            }
        } else {
            _654__PHI_TEMPORARY = 0 as libc::c_int as bool_0;
        }
    } else {
        _654__PHI_TEMPORARY = 0 as libc::c_int as bool_0;
    }
    _654 = _654__PHI_TEMPORARY;
    return _654;
}
