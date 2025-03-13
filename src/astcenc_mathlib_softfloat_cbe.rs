use ::libc;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_struct_union_OC_if32 {
    pub field0: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_64_uint32_t {
    pub array: [uint32_t; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_128_uint8_t {
    pub array: [uint8_t; 128],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct l_unnamed_1 {
    pub field0: l_array_128_uint8_t,
    pub field1: l_array_128_uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_512_uint8_t {
    pub array: [uint8_t; 512],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_60_uint32_t {
    pub array: [uint32_t; 60],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_256_uint8_t {
    pub array: [uint8_t; 256],
}
static mut _ZZL12sf16_to_sf32tE3tbl: l_array_64_uint32_t = {
    let mut init = l_array_64_uint32_t {
        array: [
            2147483648 as libc::c_uint,
            114688 as libc::c_int as uint32_t,
            114688 as libc::c_int as uint32_t,
            114688 as libc::c_int as uint32_t,
            114688 as libc::c_int as uint32_t,
            114688 as libc::c_int as uint32_t,
            114688 as libc::c_int as uint32_t,
            114688 as libc::c_int as uint32_t,
            114688 as libc::c_int as uint32_t,
            114688 as libc::c_int as uint32_t,
            114688 as libc::c_int as uint32_t,
            114688 as libc::c_int as uint32_t,
            114688 as libc::c_int as uint32_t,
            114688 as libc::c_int as uint32_t,
            114688 as libc::c_int as uint32_t,
            114688 as libc::c_int as uint32_t,
            114688 as libc::c_int as uint32_t,
            114688 as libc::c_int as uint32_t,
            114688 as libc::c_int as uint32_t,
            114688 as libc::c_int as uint32_t,
            114688 as libc::c_int as uint32_t,
            114688 as libc::c_int as uint32_t,
            114688 as libc::c_int as uint32_t,
            114688 as libc::c_int as uint32_t,
            114688 as libc::c_int as uint32_t,
            114688 as libc::c_int as uint32_t,
            114688 as libc::c_int as uint32_t,
            114688 as libc::c_int as uint32_t,
            114688 as libc::c_int as uint32_t,
            114688 as libc::c_int as uint32_t,
            114688 as libc::c_int as uint32_t,
            2147713024 as libc::c_uint,
            2147713024 as libc::c_uint,
            344064 as libc::c_int as uint32_t,
            344064 as libc::c_int as uint32_t,
            344064 as libc::c_int as uint32_t,
            344064 as libc::c_int as uint32_t,
            344064 as libc::c_int as uint32_t,
            344064 as libc::c_int as uint32_t,
            344064 as libc::c_int as uint32_t,
            344064 as libc::c_int as uint32_t,
            344064 as libc::c_int as uint32_t,
            344064 as libc::c_int as uint32_t,
            344064 as libc::c_int as uint32_t,
            344064 as libc::c_int as uint32_t,
            344064 as libc::c_int as uint32_t,
            344064 as libc::c_int as uint32_t,
            344064 as libc::c_int as uint32_t,
            344064 as libc::c_int as uint32_t,
            344064 as libc::c_int as uint32_t,
            344064 as libc::c_int as uint32_t,
            344064 as libc::c_int as uint32_t,
            344064 as libc::c_int as uint32_t,
            344064 as libc::c_int as uint32_t,
            344064 as libc::c_int as uint32_t,
            344064 as libc::c_int as uint32_t,
            344064 as libc::c_int as uint32_t,
            344064 as libc::c_int as uint32_t,
            344064 as libc::c_int as uint32_t,
            344064 as libc::c_int as uint32_t,
            344064 as libc::c_int as uint32_t,
            344064 as libc::c_int as uint32_t,
            344064 as libc::c_int as uint32_t,
            2147942400 as libc::c_uint,
        ],
    };
    init
};
static mut _ZL9clz_table: l_unnamed_1 = unsafe {
    {
        let mut init = l_unnamed_1 {
            field0: {
                let mut init = l_array_128_uint8_t {
                    array: *::core::mem::transmute::<
                        &[u8; 128],
                        &mut [uint8_t; 128],
                    >(
                        b"\x08\x07\x06\x06\x05\x05\x05\x05\x04\x04\x04\x04\x04\x04\x04\x04\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01",
                    ),
                };
                init
            },
            field1: {
                let mut init = l_array_128_uint8_t {
                    array: [
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as uint8_t,
                    ],
                };
                init
            },
        };
        init
    }
};
static mut _ZZL12sf32_to_sf16j9roundmodeE3tab: l_array_512_uint8_t = unsafe {
    {
        let mut init = l_array_512_uint8_t {
            array: *::core::mem::transmute::<
                &[u8; 512],
                &mut [uint8_t; 512],
            >(
                b"\0\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\x14\x14\x14\x14\x14\x14\x14\x14\x14\x14\x14\x14\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((2\x05\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x0F\x19\x19\x19\x19\x19\x19\x19\x19\x19\x19\x19\x19##############################----------------------------------------------------------------------------------------------------------------7",
            ),
        };
        init
    }
};
static mut _ZZL12sf32_to_sf16j9roundmodeE4tabx: l_array_60_uint32_t = {
    let mut init = l_array_60_uint32_t {
        array: [
            0,
            0,
            0,
            0,
            0,
            32768 as libc::c_int as uint32_t,
            2147483648 as libc::c_uint,
            32768 as libc::c_int as uint32_t,
            32768 as libc::c_int as uint32_t,
            32768 as libc::c_int as uint32_t,
            1 as libc::c_int as uint32_t,
            0,
            0,
            0,
            0,
            32768 as libc::c_int as uint32_t,
            32769 as libc::c_int as uint32_t,
            32768 as libc::c_int as uint32_t,
            32768 as libc::c_int as uint32_t,
            32768 as libc::c_int as uint32_t,
            0,
            0,
            0,
            0,
            0,
            32768 as libc::c_int as uint32_t,
            32768 as libc::c_int as uint32_t,
            32768 as libc::c_int as uint32_t,
            32768 as libc::c_int as uint32_t,
            32768 as libc::c_int as uint32_t,
            -(939515905 as libc::c_int) as uint32_t,
            -(939524096 as libc::c_int) as uint32_t,
            -(939524096 as libc::c_int) as uint32_t,
            -(939520001 as libc::c_int) as uint32_t,
            -(939520000 as libc::c_int) as uint32_t,
            1476395008 as libc::c_uint,
            939532287 as libc::c_int as uint32_t,
            1476395008 as libc::c_uint,
            1476399103 as libc::c_uint,
            1476399104 as libc::c_uint,
            31744 as libc::c_int as uint32_t,
            31743 as libc::c_int as uint32_t,
            31743 as libc::c_int as uint32_t,
            31744 as libc::c_int as uint32_t,
            31744 as libc::c_int as uint32_t,
            64511 as libc::c_int as uint32_t,
            64512 as libc::c_int as uint32_t,
            64511 as libc::c_int as uint32_t,
            64512 as libc::c_int as uint32_t,
            64512 as libc::c_int as uint32_t,
            2415919104 as libc::c_uint,
            2415919104 as libc::c_uint,
            2415919104 as libc::c_uint,
            2415919104 as libc::c_uint,
            2415919104 as libc::c_uint,
            536870912 as libc::c_int as uint32_t,
            536870912 as libc::c_int as uint32_t,
            536870912 as libc::c_int as uint32_t,
            536870912 as libc::c_int as uint32_t,
            536870912 as libc::c_int as uint32_t,
        ],
    };
    init
};
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
unsafe extern "C" fn llvm_lshr_u32(mut a: uint32_t, mut b: uint32_t) -> uint32_t {
    let mut r: uint32_t = a >> b;
    return r;
}
#[inline(always)]
unsafe extern "C" fn llvm_neg_u32(mut a: int32_t) -> uint32_t {
    let mut r: uint32_t = -a as uint32_t;
    return r;
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn _Z13sf16_to_floatt(mut _1: uint16_t) -> libc::c_float {
    let mut _2: uint16_t = 0;
    let mut _3: l_struct_union_OC_if32 = l_struct_union_OC_if32 { field0: 0 };
    let mut _4: uint16_t = 0;
    let mut _5: uint32_t = 0;
    let mut _6: libc::c_float = 0.;
    _2 = _1;
    _4 = _2;
    _5 = _ZL12sf16_to_sf32t(_4);
    *(&mut _3 as *mut l_struct_union_OC_if32 as *mut uint32_t) = _5;
    _6 = *(&mut _3 as *mut l_struct_union_OC_if32 as *mut libc::c_float);
    return _6;
}
#[inline(never)]
unsafe extern "C" fn _ZL12sf16_to_sf32t(mut _7: uint16_t) -> uint32_t {
    let mut _8: uint32_t = 0;
    let mut _9: uint16_t = 0;
    let mut _10: uint32_t = 0;
    let mut _11: uint32_t = 0;
    let mut _12: uint32_t = 0;
    let mut _13: uint32_t = 0;
    let mut _14: uint32_t = 0;
    let mut _15: uint16_t = 0;
    let mut _16: uint32_t = 0;
    let mut _17: uint32_t = 0;
    let mut _18: uint32_t = 0;
    let mut _19: uint32_t = 0;
    let mut _20: uint32_t = 0;
    let mut _21: uint32_t = 0;
    let mut _22: uint32_t = 0;
    let mut _23: uint32_t = 0;
    let mut _24: uint32_t = 0;
    let mut _25: uint32_t = 0;
    let mut _26: uint32_t = 0;
    let mut _27: uint32_t = 0;
    let mut _28: uint32_t = 0;
    let mut _29: uint32_t = 0;
    let mut _30: uint32_t = 0;
    let mut _31: uint32_t = 0;
    let mut _32: uint32_t = 0;
    let mut _33: uint32_t = 0;
    let mut _34: uint32_t = 0;
    let mut _35: uint32_t = 0;
    _9 = _7;
    _15 = _9;
    _10 = _15 as uint32_t;
    _16 = _10;
    _17 = *(&*(_ZZL12sf16_to_sf32tE3tbl.array)
        .as_ptr()
        .offset(
            (llvm_lshr_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(_16, 10)
                as uint64_t as int64_t as isize,
        ) as *const uint32_t as *mut uint32_t);
    _11 = _17;
    _18 = _10;
    _19 = _11;
    _11 = llvm_add_u32(_19, _18);
    _20 = _11;
    if _20 & 2147483648 as libc::c_uint == 0 as libc::c_uint {
        _21 = _11;
        _8 = _21 << 13 as libc::c_int;
    } else {
        _22 = _11;
        if _22 & 1023 as libc::c_int as uint32_t == 0 as libc::c_uint {
            _23 = _11;
            _8 = _23 << 13 as libc::c_int;
        } else {
            _24 = _10;
            if _24 & 31744 as libc::c_int as uint32_t != 0 as libc::c_uint {
                _25 = _11;
                _8 = _25 << 13 as libc::c_int | 4194304 as libc::c_int as uint32_t;
            } else {
                _26 = _10;
                _12 = (_26 & 32768 as libc::c_int as uint32_t) << 16 as libc::c_int;
                _27 = _10;
                _13 = _27 & 32767 as libc::c_int as uint32_t;
                _28 = _13;
                _29 = _ZL5clz32j(_28);
                _14 = _29;
                _30 = _14;
                _31 = _13;
                _13 = _31 << _30;
                _32 = _13;
                _33 = _14;
                _34 = _12;
                _8 = llvm_add_u32(
                    llvm_add_u32(
                        llvm_lshr_u32(_32, 8 as libc::c_int as uint32_t),
                        llvm_sub_u32(133 as libc::c_int as uint32_t, _33) << 23 as libc::c_int,
                    ),
                    _34,
                );
            }
        }
    }
    _35 = _8;
    return _35;
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn _Z13float_to_sf16f(mut _43: libc::c_float) -> uint16_t {
    let mut _44: libc::c_float = 0.;
    let mut _45: l_struct_union_OC_if32 = l_struct_union_OC_if32 { field0: 0 };
    let mut _46: libc::c_float = 0.;
    let mut _47: uint32_t = 0;
    let mut _48: uint16_t = 0;
    _44 = _43;
    _46 = _44;
    *(&mut _45 as *mut l_struct_union_OC_if32 as *mut libc::c_float) = _46;
    _47 = *(&mut _45 as *mut l_struct_union_OC_if32 as *mut uint32_t);
    _48 = _ZL12sf32_to_sf16j9roundmode(_47, 3 as libc::c_int as uint32_t);
    return _48;
}
#[inline(never)]
unsafe extern "C" fn _ZL12sf32_to_sf16j9roundmode(
    mut _49: uint32_t,
    mut _50: uint32_t,
) -> uint16_t {
    let mut _51: uint16_t = 0;
    let mut _52: uint32_t = 0;
    let mut _53: uint32_t = 0;
    let mut _54: uint32_t = 0;
    let mut _55: uint32_t = 0;
    let mut _56: uint32_t = 0;
    let mut _57: uint32_t = 0;
    let mut _58: uint32_t = 0;
    let mut _59: uint8_t = 0;
    let mut _60: uint32_t = 0;
    let mut _61: uint32_t = 0;
    let mut _62: uint32_t = 0;
    let mut _63: uint32_t = 0;
    let mut _64: uint32_t = 0;
    let mut _65: uint32_t = 0;
    let mut _66: uint32_t = 0;
    let mut _67: uint32_t = 0;
    let mut _68: uint32_t = 0;
    let mut _69: uint32_t = 0;
    let mut _70: uint32_t = 0;
    let mut _71: uint32_t = 0;
    let mut _72: uint32_t = 0;
    let mut _73: uint32_t = 0;
    let mut _74: uint32_t = 0;
    let mut _75: uint32_t = 0;
    let mut _76: uint32_t = 0;
    let mut _77: uint32_t = 0;
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
    let mut _91: uint32_t = 0;
    let mut _92: uint32_t = 0;
    let mut _93: uint32_t = 0;
    let mut _94: uint32_t = 0;
    let mut _95: uint32_t = 0;
    let mut _96: uint32_t = 0;
    let mut _97: uint16_t = 0;
    _52 = _49;
    _53 = _50;
    _57 = _53;
    _58 = _52;
    _59 = *(&*(_ZZL12sf32_to_sf16j9roundmodeE3tab.array)
        .as_ptr()
        .offset(
            (llvm_lshr_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                _58,
                23 as libc::c_int as uint32_t,
            ) as uint64_t as int64_t as isize,
        ) as *const uint8_t as *mut uint8_t);
    _55 = llvm_add_u32(_57, _59 as uint32_t);
    _60 = _55;
    _61 = *(&*(_ZZL12sf32_to_sf16j9roundmodeE4tabx.array)
        .as_ptr()
        .offset(_60 as uint64_t as int64_t as isize) as *const uint32_t
        as *mut uint32_t);
    _56 = _61;
    _62 = _55;
    match _62 {
        50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 58 | 59 => {
            _63 = _52;
            _54 = llvm_sub_u32(_63, 1 as libc::c_int as uint32_t)
                & 8388608 as libc::c_int as uint32_t;
            _64 = _52;
            _65 = _56;
            _66 = _54;
            _51 = (llvm_lshr_u32(llvm_add_u32(_64, _65), 13 as libc::c_int as uint32_t)
                | llvm_lshr_u32(_66, 14 as libc::c_int as uint32_t)) as uint16_t;
        }
        0 => {
            _67 = _52;
            _51 = llvm_lshr_u32(llvm_neg_u32(_67 as int32_t), 31 as libc::c_int as uint32_t)
                as uint16_t;
        }
        6 => {
            _68 = _56;
            _69 = _52;
            _51 = llvm_add_u32(
                llvm_lshr_u32(llvm_sub_u32(_68, _69), 31 as libc::c_int as uint32_t),
                32768 as libc::c_int as uint32_t,
            ) as uint16_t;
        }
        1 | 2 | 3 | 4 | 5 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 40
        | 41 | 42 | 43 | 44 | 45 | 46 | 47 | 48 | 49 => {
            _70 = _56;
            _51 = _70 as uint16_t;
        }
        30 | 31 | 32 | 34 | 35 | 36 | 37 | 39 => {
            _71 = _52;
            _72 = _56;
            _51 = llvm_lshr_u32(llvm_add_u32(_71, _72), 13 as libc::c_int as uint32_t) as uint16_t;
        }
        33 | 38 => {
            _73 = _52;
            _74 = _56;
            _54 = llvm_add_u32(_73, _74);
            _75 = _52;
            _76 = _54;
            _54 = llvm_add_u32(
                _76,
                llvm_lshr_u32(_75, 13 as libc::c_int as uint32_t) & 1 as libc::c_int as uint32_t,
            );
            _77 = _54;
            _51 = llvm_lshr_u32(_77, 13 as libc::c_int as uint32_t) as uint16_t;
        }
        21 | 22 | 25 | 27 => {
            _78 = _52;
            _54 = llvm_sub_u32(
                126 as libc::c_int as uint32_t,
                llvm_lshr_u32(_78, 23 as libc::c_int as uint32_t) & 255 as libc::c_int as uint32_t,
            );
            _79 = _52;
            _80 = _54;
            _81 = _56;
            _51 = (llvm_lshr_u32(
                llvm_add_u32(
                    _79 & 8388607 as libc::c_int as uint32_t,
                    8388608 as libc::c_int as uint32_t,
                ),
                _80,
            ) | _81) as uint16_t;
        }
        20 | 26 => {
            _82 = _52;
            _54 = llvm_sub_u32(
                126 as libc::c_int as uint32_t,
                llvm_lshr_u32(_82, 23 as libc::c_int as uint32_t) & 255 as libc::c_int as uint32_t,
            );
            _83 = _52;
            _84 = _54;
            _85 = _ZL12rtup_shift32jj(
                llvm_add_u32(
                    _83 & 8388607 as libc::c_int as uint32_t,
                    8388608 as libc::c_int as uint32_t,
                ),
                _84,
            );
            _86 = _56;
            _51 = (_85 | _86) as uint16_t;
        }
        24 | 29 => {
            _87 = _52;
            _54 = llvm_sub_u32(
                126 as libc::c_int as uint32_t,
                llvm_lshr_u32(_87, 23 as libc::c_int as uint32_t) & 255 as libc::c_int as uint32_t,
            );
            _88 = _52;
            _89 = _54;
            _90 = _ZL12rtna_shift32jj(
                llvm_add_u32(
                    _88 & 8388607 as libc::c_int as uint32_t,
                    8388608 as libc::c_int as uint32_t,
                ),
                _89,
            );
            _91 = _56;
            _51 = (_90 | _91) as uint16_t;
        }
        23 | 28 => {
            _92 = _52;
            _54 = llvm_sub_u32(
                126 as libc::c_int as uint32_t,
                llvm_lshr_u32(_92, 23 as libc::c_int as uint32_t) & 255 as libc::c_int as uint32_t,
            );
            _93 = _52;
            _94 = _54;
            _95 = _ZL12rtne_shift32jj(
                llvm_add_u32(
                    _93 & 8388607 as libc::c_int as uint32_t,
                    8388608 as libc::c_int as uint32_t,
                ),
                _94,
            );
            _96 = _56;
            _51 = (_95 | _96) as uint16_t;
        }
        _ => {
            _51 = 0 as libc::c_int as uint16_t;
        }
    }
    _97 = _51;
    return _97;
}
#[inline(never)]
unsafe extern "C" fn _ZL5clz32j(mut _110: uint32_t) -> uint32_t {
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
    let mut _121: uint8_t = 0;
    _111 = _110;
    _112 = 24 as libc::c_int as uint32_t;
    _113 = _111;
    if _113 >= 65536 as libc::c_uint {
        _114 = _111;
        _111 = llvm_lshr_u32(_114, 16 as libc::c_int as uint32_t);
        _115 = _112;
        _112 = llvm_sub_u32(_115, 16 as libc::c_int as uint32_t);
    }
    _116 = _111;
    if _116 >= 256 as libc::c_uint {
        _117 = _111;
        _111 = llvm_lshr_u32(_117, 8 as libc::c_int as uint32_t);
        _118 = _112;
        _112 = llvm_sub_u32(_118, 8 as libc::c_int as uint32_t);
    }
    _119 = _112;
    _120 = _111;
    _121 = *(&mut *((*(&_ZL9clz_table as *const l_unnamed_1 as *mut l_array_256_uint8_t)).array)
        .as_mut_ptr()
        .offset(_120 as uint64_t as int64_t as isize) as *mut uint8_t);
    return llvm_add_u32(_119, _121 as uint32_t);
}
#[inline(never)]
unsafe extern "C" fn _ZL12rtup_shift32jj(mut _126: uint32_t, mut _127: uint32_t) -> uint32_t {
    let mut _128: uint32_t = 0;
    let mut _129: uint32_t = 0;
    let mut _130: uint32_t = 0;
    let mut _131: uint32_t = 0;
    let mut _132: uint32_t = 0;
    let mut _133: uint32_t = 0;
    let mut _134: uint32_t = 0;
    let mut _135: uint32_t = 0;
    let mut _136: uint32_t = 0;
    let mut _137: uint32_t = 0;
    _128 = _126;
    _129 = _127;
    _131 = _129;
    _130 = ((1 as libc::c_int) << _131) as uint32_t;
    _132 = _130;
    _133 = _128;
    _128 = llvm_add_u32(_133, _132);
    _134 = _128;
    _128 = llvm_add_u32(_134, -(1 as libc::c_int) as uint32_t);
    _135 = _129;
    _136 = _128;
    _128 = llvm_lshr_u32(_136, _135);
    _137 = _128;
    return _137;
}
#[inline(never)]
unsafe extern "C" fn _ZL12rtna_shift32jj(mut _138: uint32_t, mut _139: uint32_t) -> uint32_t {
    let mut _140: uint32_t = 0;
    let mut _141: uint32_t = 0;
    let mut _142: uint32_t = 0;
    let mut _143: uint32_t = 0;
    let mut _144: uint32_t = 0;
    let mut _145: uint32_t = 0;
    let mut _146: uint32_t = 0;
    let mut _147: uint32_t = 0;
    let mut _148: uint32_t = 0;
    _140 = _138;
    _141 = _139;
    _143 = _141;
    _142 = llvm_lshr_u32(
        ((1 as libc::c_int) << _143) as uint32_t,
        1 as libc::c_int as uint32_t,
    );
    _144 = _142;
    _145 = _140;
    _140 = llvm_add_u32(_145, _144);
    _146 = _141;
    _147 = _140;
    _140 = llvm_lshr_u32(_147, _146);
    _148 = _140;
    return _148;
}
#[inline(never)]
unsafe extern "C" fn _ZL12rtne_shift32jj(mut _149: uint32_t, mut _150: uint32_t) -> uint32_t {
    let mut _151: uint32_t = 0;
    let mut _152: uint32_t = 0;
    let mut _153: uint32_t = 0;
    let mut _154: uint32_t = 0;
    let mut _155: uint32_t = 0;
    let mut _156: uint32_t = 0;
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
    _151 = _149;
    _152 = _150;
    _156 = _152;
    _153 = ((1 as libc::c_int) << _156) as uint32_t;
    _157 = _151;
    _158 = _153;
    _154 = llvm_add_u32(_157, llvm_lshr_u32(_158, 1 as libc::c_int as uint32_t));
    _159 = _151;
    _160 = _153;
    _155 = (_159 | 1 as libc::c_int as uint32_t) & _160;
    _161 = _155;
    _155 = llvm_add_u32(_161, -(1 as libc::c_int) as uint32_t);
    _162 = _155;
    _163 = _154;
    _154 = llvm_sub_u32(_163, llvm_lshr_u32(_162, 31 as libc::c_int as uint32_t));
    _164 = _152;
    _165 = _154;
    _154 = llvm_lshr_u32(_165, _164);
    _166 = _154;
    return _166;
}
