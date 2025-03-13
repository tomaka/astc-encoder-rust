extern "C" {
    fn __assert_fail(
        _649: *mut core::ffi::c_void,
        _650: *mut core::ffi::c_void,
        _651: uint32_t,
        _652: *mut core::ffi::c_void,
    ) -> !;
    fn memset(
        _: *mut core::ffi::c_void,
        _: core::ffi::c_int,
        _: core::ffi::c_ulong,
    ) -> *mut core::ffi::c_void;
}
pub type __uint8_t = core::ffi::c_uchar;
pub type __int32_t = core::ffi::c_int;
pub type __uint32_t = core::ffi::c_uint;
pub type __int64_t = core::ffi::c_long;
pub type __uint64_t = core::ffi::c_ulong;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_struct_struct_OC_ise_size {
    pub field0: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_21_struct_AC_l_struct_struct_OC_ise_size {
    pub array: [l_struct_struct_OC_ise_size; 21],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_struct_struct_OC_std_KD__KD_array {
    pub field0: l_array_21_struct_AC_l_struct_struct_OC_ise_size,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_struct_struct_OC_btq_count {
    pub field0: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_21_struct_AC_l_struct_struct_OC_btq_count {
    pub array: [l_struct_struct_OC_btq_count; 21],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_struct_struct_OC_std_KD__KD_array_OC_0 {
    pub field0: l_array_21_struct_AC_l_struct_struct_OC_btq_count,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_20_uint8_t {
    pub array: [uint8_t; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_55_uint8_t {
    pub array: [uint8_t; 55],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_86_uint8_t {
    pub array: [uint8_t; 86],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_3_uint8_t {
    pub array: [uint8_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_3_struct_AC_l_array_3_uint8_t {
    pub array: [l_array_3_uint8_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_3_struct_AC_l_array_3_struct_AC_l_array_3_uint8_t {
    pub array: [l_array_3_struct_AC_l_array_3_uint8_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_3_struct_AC_l_array_3_struct_AC_l_array_3_struct_AC_l_array_3_uint8_t {
    pub array: [l_array_3_struct_AC_l_array_3_struct_AC_l_array_3_uint8_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_3_struct_AC_l_array_3_struct_AC_l_array_3_struct_AC_l_array_3_struct_AC_l_array_3_uint8_t
{
    pub array: [l_array_3_struct_AC_l_array_3_struct_AC_l_array_3_struct_AC_l_array_3_uint8_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_4_uint8_t {
    pub array: [uint8_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_5_uint8_t {
    pub array: [uint8_t; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_5_struct_AC_l_array_5_uint8_t {
    pub array: [l_array_5_uint8_t; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_5_struct_AC_l_array_5_struct_AC_l_array_5_uint8_t {
    pub array: [l_array_5_struct_AC_l_array_5_uint8_t; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_2_uint8_t {
    pub array: [uint8_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_16_uint8_t {
    pub array: [uint8_t; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_256_struct_AC_l_array_5_uint8_t {
    pub array: [l_array_5_uint8_t; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_17_uint8_t {
    pub array: [uint8_t; 17],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_128_struct_AC_l_array_3_uint8_t {
    pub array: [l_array_3_uint8_t; 128],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_68_uint8_t {
    pub array: [uint8_t; 68],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_array_22_uint8_t {
    pub array: [uint8_t; 22],
}
static mut _ZL9ise_sizes: l_struct_struct_OC_std_KD__KD_array = {
    let mut init = l_struct_struct_OC_std_KD__KD_array {
        field0: {
            let mut init = l_array_21_struct_AC_l_struct_struct_OC_ise_size {
                array: [
                    {
                        let mut init = l_struct_struct_OC_ise_size {
                            field0: 1 as core::ffi::c_int as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = l_struct_struct_OC_ise_size {
                            field0: 136 as core::ffi::c_uint as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = l_struct_struct_OC_ise_size {
                            field0: 2 as core::ffi::c_int as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = l_struct_struct_OC_ise_size {
                            field0: 71 as core::ffi::c_uint as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = l_struct_struct_OC_ise_size {
                            field0: 141 as core::ffi::c_uint as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = l_struct_struct_OC_ise_size {
                            field0: 3 as core::ffi::c_int as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = l_struct_struct_OC_ise_size {
                            field0: 74 as core::ffi::c_uint as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = l_struct_struct_OC_ise_size {
                            field0: 146 as core::ffi::c_uint as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = l_struct_struct_OC_ise_size {
                            field0: 4 as core::ffi::c_int as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = l_struct_struct_OC_ise_size {
                            field0: 77 as core::ffi::c_uint as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = l_struct_struct_OC_ise_size {
                            field0: 151 as core::ffi::c_uint as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = l_struct_struct_OC_ise_size {
                            field0: 5 as core::ffi::c_int as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = l_struct_struct_OC_ise_size {
                            field0: 80 as core::ffi::c_uint as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = l_struct_struct_OC_ise_size {
                            field0: 156 as core::ffi::c_uint as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = l_struct_struct_OC_ise_size {
                            field0: 6 as core::ffi::c_int as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = l_struct_struct_OC_ise_size {
                            field0: 83 as core::ffi::c_uint as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = l_struct_struct_OC_ise_size {
                            field0: 161 as core::ffi::c_uint as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = l_struct_struct_OC_ise_size {
                            field0: 7 as core::ffi::c_int as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = l_struct_struct_OC_ise_size {
                            field0: 86 as core::ffi::c_uint as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = l_struct_struct_OC_ise_size {
                            field0: 166 as core::ffi::c_uint as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = l_struct_struct_OC_ise_size {
                            field0: 8 as core::ffi::c_int as uint8_t,
                        };
                        init
                    },
                ],
            };
            init
        },
    };
    init
};
static mut _OC_str: l_array_20_uint8_t = unsafe {
    {
        let mut init = l_array_20_uint8_t {
            array: *::core::mem::transmute::<&[u8; 20], &mut [uint8_t; 20]>(
                b"character_count > 0\0",
            ),
        };
        init
    }
};
static mut _OC_str_OC_1: l_array_55_uint8_t = unsafe {
    {
        let mut init = l_array_55_uint8_t {
            array: *::core::mem::transmute::<&[u8; 55], &mut [uint8_t; 55]>(
                b"/root/astc-encoder/Source/astcenc_integer_sequence.cpp\0",
            ),
        };
        init
    }
};
static mut __PRETTY_FUNCTION___OC__Z10encode_ise12quant_methodjPKhPhj: l_array_86_uint8_t = unsafe {
    {
        let mut init = l_array_86_uint8_t {
            array: *::core::mem::transmute::<
                &[u8; 86],
                &mut [uint8_t; 86],
            >(
                b"void encode_ise(quant_method, unsigned int, const uint8_t *, uint8_t *, unsigned int)\0",
            ),
        };
        init
    }
};
static mut _ZL10btq_counts: l_struct_struct_OC_std_KD__KD_array_OC_0 = {
    let mut init = l_struct_struct_OC_std_KD__KD_array_OC_0 {
        field0: {
            let mut init = l_array_21_struct_AC_l_struct_struct_OC_btq_count {
                array: [
                    {
                        let mut init = l_struct_struct_OC_btq_count {
                            field0: 1 as core::ffi::c_int as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = l_struct_struct_OC_btq_count {
                            field0: 64 as core::ffi::c_uint as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = l_struct_struct_OC_btq_count {
                            field0: 2 as core::ffi::c_int as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = l_struct_struct_OC_btq_count {
                            field0: 128 as core::ffi::c_uint as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = l_struct_struct_OC_btq_count {
                            field0: 65 as core::ffi::c_uint as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = l_struct_struct_OC_btq_count {
                            field0: 3 as core::ffi::c_int as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = l_struct_struct_OC_btq_count {
                            field0: 129 as core::ffi::c_uint as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = l_struct_struct_OC_btq_count {
                            field0: 66 as core::ffi::c_uint as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = l_struct_struct_OC_btq_count {
                            field0: 4 as core::ffi::c_int as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = l_struct_struct_OC_btq_count {
                            field0: 130 as core::ffi::c_uint as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = l_struct_struct_OC_btq_count {
                            field0: 67 as core::ffi::c_uint as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = l_struct_struct_OC_btq_count {
                            field0: 5 as core::ffi::c_int as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = l_struct_struct_OC_btq_count {
                            field0: 131 as core::ffi::c_uint as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = l_struct_struct_OC_btq_count {
                            field0: 68 as core::ffi::c_uint as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = l_struct_struct_OC_btq_count {
                            field0: 6 as core::ffi::c_int as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = l_struct_struct_OC_btq_count {
                            field0: 132 as core::ffi::c_uint as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = l_struct_struct_OC_btq_count {
                            field0: 69 as core::ffi::c_uint as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = l_struct_struct_OC_btq_count {
                            field0: 7 as core::ffi::c_int as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = l_struct_struct_OC_btq_count {
                            field0: 133 as core::ffi::c_uint as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = l_struct_struct_OC_btq_count {
                            field0: 70 as core::ffi::c_uint as uint8_t,
                        };
                        init
                    },
                    {
                        let mut init = l_struct_struct_OC_btq_count {
                            field0: 8 as core::ffi::c_int as uint8_t,
                        };
                        init
                    },
                ],
            };
            init
        },
    };
    init
};
static mut _ZL16integer_of_trits: l_array_3_struct_AC_l_array_3_struct_AC_l_array_3_struct_AC_l_array_3_struct_AC_l_array_3_uint8_t = unsafe {
    {
        let mut init = l_array_3_struct_AC_l_array_3_struct_AC_l_array_3_struct_AC_l_array_3_struct_AC_l_array_3_uint8_t {
            array: [
                {
                    let mut init = l_array_3_struct_AC_l_array_3_struct_AC_l_array_3_struct_AC_l_array_3_uint8_t {
                        array: [
                            {
                                let mut init = l_array_3_struct_AC_l_array_3_struct_AC_l_array_3_uint8_t {
                                    array: [
                                        {
                                            let mut init = l_array_3_struct_AC_l_array_3_uint8_t {
                                                array: [
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\0\x01\x02"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\x04\x05\x06"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\x08\t\n"),
                                                        };
                                                        init
                                                    },
                                                ],
                                            };
                                            init
                                        },
                                        {
                                            let mut init = l_array_3_struct_AC_l_array_3_uint8_t {
                                                array: [
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\x10\x11\x12"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\x14\x15\x16"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\x18\x19\x1A"),
                                                        };
                                                        init
                                                    },
                                                ],
                                            };
                                            init
                                        },
                                        {
                                            let mut init = l_array_3_struct_AC_l_array_3_uint8_t {
                                                array: [
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\x03\x07\x0F"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\x13\x17\x1B"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\x0C\r\x0E"),
                                                        };
                                                        init
                                                    },
                                                ],
                                            };
                                            init
                                        },
                                    ],
                                };
                                init
                            },
                            {
                                let mut init = l_array_3_struct_AC_l_array_3_struct_AC_l_array_3_uint8_t {
                                    array: [
                                        {
                                            let mut init = l_array_3_struct_AC_l_array_3_uint8_t {
                                                array: [
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b" !\""),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"$%&"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"()*"),
                                                        };
                                                        init
                                                    },
                                                ],
                                            };
                                            init
                                        },
                                        {
                                            let mut init = l_array_3_struct_AC_l_array_3_uint8_t {
                                                array: [
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"012"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"456"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"89:"),
                                                        };
                                                        init
                                                    },
                                                ],
                                            };
                                            init
                                        },
                                        {
                                            let mut init = l_array_3_struct_AC_l_array_3_uint8_t {
                                                array: [
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"#'/"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"37;"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b",-."),
                                                        };
                                                        init
                                                    },
                                                ],
                                            };
                                            init
                                        },
                                    ],
                                };
                                init
                            },
                            {
                                let mut init = l_array_3_struct_AC_l_array_3_struct_AC_l_array_3_uint8_t {
                                    array: [
                                        {
                                            let mut init = l_array_3_struct_AC_l_array_3_uint8_t {
                                                array: [
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"@AB"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"DEF"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"HIJ"),
                                                        };
                                                        init
                                                    },
                                                ],
                                            };
                                            init
                                        },
                                        {
                                            let mut init = l_array_3_struct_AC_l_array_3_uint8_t {
                                                array: [
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"PQR"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"TUV"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"XYZ"),
                                                        };
                                                        init
                                                    },
                                                ],
                                            };
                                            init
                                        },
                                        {
                                            let mut init = l_array_3_struct_AC_l_array_3_uint8_t {
                                                array: [
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"CGO"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"SW["),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"LMN"),
                                                        };
                                                        init
                                                    },
                                                ],
                                            };
                                            init
                                        },
                                    ],
                                };
                                init
                            },
                        ],
                    };
                    init
                },
                {
                    let mut init = l_array_3_struct_AC_l_array_3_struct_AC_l_array_3_struct_AC_l_array_3_uint8_t {
                        array: [
                            {
                                let mut init = l_array_3_struct_AC_l_array_3_struct_AC_l_array_3_uint8_t {
                                    array: [
                                        {
                                            let mut init = l_array_3_struct_AC_l_array_3_uint8_t {
                                                array: [
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\x80\x81\x82"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\x84\x85\x86"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\x88\x89\x8A"),
                                                        };
                                                        init
                                                    },
                                                ],
                                            };
                                            init
                                        },
                                        {
                                            let mut init = l_array_3_struct_AC_l_array_3_uint8_t {
                                                array: [
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\x90\x91\x92"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\x94\x95\x96"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\x98\x99\x9A"),
                                                        };
                                                        init
                                                    },
                                                ],
                                            };
                                            init
                                        },
                                        {
                                            let mut init = l_array_3_struct_AC_l_array_3_uint8_t {
                                                array: [
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\x83\x87\x8F"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\x93\x97\x9B"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\x8C\x8D\x8E"),
                                                        };
                                                        init
                                                    },
                                                ],
                                            };
                                            init
                                        },
                                    ],
                                };
                                init
                            },
                            {
                                let mut init = l_array_3_struct_AC_l_array_3_struct_AC_l_array_3_uint8_t {
                                    array: [
                                        {
                                            let mut init = l_array_3_struct_AC_l_array_3_uint8_t {
                                                array: [
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\xA0\xA1\xA2"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\xA4\xA5\xA6"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\xA8\xA9\xAA"),
                                                        };
                                                        init
                                                    },
                                                ],
                                            };
                                            init
                                        },
                                        {
                                            let mut init = l_array_3_struct_AC_l_array_3_uint8_t {
                                                array: [
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\xB0\xB1\xB2"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\xB4\xB5\xB6"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\xB8\xB9\xBA"),
                                                        };
                                                        init
                                                    },
                                                ],
                                            };
                                            init
                                        },
                                        {
                                            let mut init = l_array_3_struct_AC_l_array_3_uint8_t {
                                                array: [
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\xA3\xA7\xAF"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\xB3\xB7\xBB"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\xAC\xAD\xAE"),
                                                        };
                                                        init
                                                    },
                                                ],
                                            };
                                            init
                                        },
                                    ],
                                };
                                init
                            },
                            {
                                let mut init = l_array_3_struct_AC_l_array_3_struct_AC_l_array_3_uint8_t {
                                    array: [
                                        {
                                            let mut init = l_array_3_struct_AC_l_array_3_uint8_t {
                                                array: [
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\xC0\xC1\xC2"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\xC4\xC5\xC6"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\xC8\xC9\xCA"),
                                                        };
                                                        init
                                                    },
                                                ],
                                            };
                                            init
                                        },
                                        {
                                            let mut init = l_array_3_struct_AC_l_array_3_uint8_t {
                                                array: [
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\xD0\xD1\xD2"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\xD4\xD5\xD6"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\xD8\xD9\xDA"),
                                                        };
                                                        init
                                                    },
                                                ],
                                            };
                                            init
                                        },
                                        {
                                            let mut init = l_array_3_struct_AC_l_array_3_uint8_t {
                                                array: [
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\xC3\xC7\xCF"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\xD3\xD7\xDB"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\xCC\xCD\xCE"),
                                                        };
                                                        init
                                                    },
                                                ],
                                            };
                                            init
                                        },
                                    ],
                                };
                                init
                            },
                        ],
                    };
                    init
                },
                {
                    let mut init = l_array_3_struct_AC_l_array_3_struct_AC_l_array_3_struct_AC_l_array_3_uint8_t {
                        array: [
                            {
                                let mut init = l_array_3_struct_AC_l_array_3_struct_AC_l_array_3_uint8_t {
                                    array: [
                                        {
                                            let mut init = l_array_3_struct_AC_l_array_3_uint8_t {
                                                array: [
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"`ab"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"def"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"hij"),
                                                        };
                                                        init
                                                    },
                                                ],
                                            };
                                            init
                                        },
                                        {
                                            let mut init = l_array_3_struct_AC_l_array_3_uint8_t {
                                                array: [
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"pqr"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"tuv"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"xyz"),
                                                        };
                                                        init
                                                    },
                                                ],
                                            };
                                            init
                                        },
                                        {
                                            let mut init = l_array_3_struct_AC_l_array_3_uint8_t {
                                                array: [
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"cgo"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"sw{"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"lmn"),
                                                        };
                                                        init
                                                    },
                                                ],
                                            };
                                            init
                                        },
                                    ],
                                };
                                init
                            },
                            {
                                let mut init = l_array_3_struct_AC_l_array_3_struct_AC_l_array_3_uint8_t {
                                    array: [
                                        {
                                            let mut init = l_array_3_struct_AC_l_array_3_uint8_t {
                                                array: [
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\xE0\xE1\xE2"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\xE4\xE5\xE6"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\xE8\xE9\xEA"),
                                                        };
                                                        init
                                                    },
                                                ],
                                            };
                                            init
                                        },
                                        {
                                            let mut init = l_array_3_struct_AC_l_array_3_uint8_t {
                                                array: [
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\xF0\xF1\xF2"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\xF4\xF5\xF6"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\xF8\xF9\xFA"),
                                                        };
                                                        init
                                                    },
                                                ],
                                            };
                                            init
                                        },
                                        {
                                            let mut init = l_array_3_struct_AC_l_array_3_uint8_t {
                                                array: [
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\xE3\xE7\xEF"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\xF3\xF7\xFB"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\xEC\xED\xEE"),
                                                        };
                                                        init
                                                    },
                                                ],
                                            };
                                            init
                                        },
                                    ],
                                };
                                init
                            },
                            {
                                let mut init = l_array_3_struct_AC_l_array_3_struct_AC_l_array_3_uint8_t {
                                    array: [
                                        {
                                            let mut init = l_array_3_struct_AC_l_array_3_uint8_t {
                                                array: [
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\x1C\x1D\x1E"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"<=>"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\\]^"),
                                                        };
                                                        init
                                                    },
                                                ],
                                            };
                                            init
                                        },
                                        {
                                            let mut init = l_array_3_struct_AC_l_array_3_uint8_t {
                                                array: [
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\x9C\x9D\x9E"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\xBC\xBD\xBE"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\xDC\xDD\xDE"),
                                                        };
                                                        init
                                                    },
                                                ],
                                            };
                                            init
                                        },
                                        {
                                            let mut init = l_array_3_struct_AC_l_array_3_uint8_t {
                                                array: [
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\x1F?\x7F"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\x9F\xBF\xFF"),
                                                        };
                                                        init
                                                    },
                                                    {
                                                        let mut init = l_array_3_uint8_t {
                                                            array: *::core::mem::transmute::<
                                                                &[u8; 3],
                                                                &mut [uint8_t; 3],
                                                            >(b"\xFC\xFD\xFE"),
                                                        };
                                                        init
                                                    },
                                                ],
                                            };
                                            init
                                        },
                                    ],
                                };
                                init
                            },
                        ],
                    };
                    init
                },
            ],
        };
        init
    }
};
static mut _ZZ10encode_ise12quant_methodjPKhPhjE5tbits: l_array_4_uint8_t = unsafe {
    {
        let mut init = l_array_4_uint8_t {
            array: *::core::mem::transmute::<&[u8; 4], &mut [uint8_t; 4]>(b"\x02\x02\x01\x02"),
        };
        init
    }
};
static mut _ZZ10encode_ise12quant_methodjPKhPhjE6tshift: l_array_4_uint8_t = unsafe {
    {
        let mut init = l_array_4_uint8_t {
            array: *::core::mem::transmute::<&[u8; 4], &mut [uint8_t; 4]>(b"\0\x02\x04\x05"),
        };
        init
    }
};
static mut _ZL17integer_of_quints: l_array_5_struct_AC_l_array_5_struct_AC_l_array_5_uint8_t = unsafe {
    {
        let mut init = l_array_5_struct_AC_l_array_5_struct_AC_l_array_5_uint8_t {
            array: [
                {
                    let mut init = l_array_5_struct_AC_l_array_5_uint8_t {
                        array: [
                            {
                                let mut init = l_array_5_uint8_t {
                                    array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                                        b"\0\x01\x02\x03\x04",
                                    ),
                                };
                                init
                            },
                            {
                                let mut init = l_array_5_uint8_t {
                                    array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                                        b"\x08\t\n\x0B\x0C",
                                    ),
                                };
                                init
                            },
                            {
                                let mut init = l_array_5_uint8_t {
                                    array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                                        b"\x10\x11\x12\x13\x14",
                                    ),
                                };
                                init
                            },
                            {
                                let mut init = l_array_5_uint8_t {
                                    array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                                        b"\x18\x19\x1A\x1B\x1C",
                                    ),
                                };
                                init
                            },
                            {
                                let mut init = l_array_5_uint8_t {
                                    array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                                        b"\x05\r\x15\x1D\x06",
                                    ),
                                };
                                init
                            },
                        ],
                    };
                    init
                },
                {
                    let mut init = l_array_5_struct_AC_l_array_5_uint8_t {
                        array: [
                            {
                                let mut init = l_array_5_uint8_t {
                                    array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                                        b" !\"#$",
                                    ),
                                };
                                init
                            },
                            {
                                let mut init = l_array_5_uint8_t {
                                    array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                                        b"()*+,",
                                    ),
                                };
                                init
                            },
                            {
                                let mut init = l_array_5_uint8_t {
                                    array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                                        b"01234",
                                    ),
                                };
                                init
                            },
                            {
                                let mut init = l_array_5_uint8_t {
                                    array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                                        b"89:;<",
                                    ),
                                };
                                init
                            },
                            {
                                let mut init = l_array_5_uint8_t {
                                    array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                                        b"%-5=\x0E",
                                    ),
                                };
                                init
                            },
                        ],
                    };
                    init
                },
                {
                    let mut init = l_array_5_struct_AC_l_array_5_uint8_t {
                        array: [
                            {
                                let mut init = l_array_5_uint8_t {
                                    array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                                        b"@ABCD",
                                    ),
                                };
                                init
                            },
                            {
                                let mut init = l_array_5_uint8_t {
                                    array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                                        b"HIJKL",
                                    ),
                                };
                                init
                            },
                            {
                                let mut init = l_array_5_uint8_t {
                                    array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                                        b"PQRST",
                                    ),
                                };
                                init
                            },
                            {
                                let mut init = l_array_5_uint8_t {
                                    array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                                        b"XYZ[\\",
                                    ),
                                };
                                init
                            },
                            {
                                let mut init = l_array_5_uint8_t {
                                    array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                                        b"EMU]\x16",
                                    ),
                                };
                                init
                            },
                        ],
                    };
                    init
                },
                {
                    let mut init = l_array_5_struct_AC_l_array_5_uint8_t {
                        array: [
                            {
                                let mut init = l_array_5_uint8_t {
                                    array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                                        b"`abcd",
                                    ),
                                };
                                init
                            },
                            {
                                let mut init = l_array_5_uint8_t {
                                    array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                                        b"hijkl",
                                    ),
                                };
                                init
                            },
                            {
                                let mut init = l_array_5_uint8_t {
                                    array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                                        b"pqrst",
                                    ),
                                };
                                init
                            },
                            {
                                let mut init = l_array_5_uint8_t {
                                    array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                                        b"xyz{|",
                                    ),
                                };
                                init
                            },
                            {
                                let mut init = l_array_5_uint8_t {
                                    array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                                        b"emu}\x1E",
                                    ),
                                };
                                init
                            },
                        ],
                    };
                    init
                },
                {
                    let mut init = l_array_5_struct_AC_l_array_5_uint8_t {
                        array: [
                            {
                                let mut init = l_array_5_uint8_t {
                                    array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                                        b"fgFG&",
                                    ),
                                };
                                init
                            },
                            {
                                let mut init = l_array_5_uint8_t {
                                    array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                                        b"noNO.",
                                    ),
                                };
                                init
                            },
                            {
                                let mut init = l_array_5_uint8_t {
                                    array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                                        b"vwVW6",
                                    ),
                                };
                                init
                            },
                            {
                                let mut init = l_array_5_uint8_t {
                                    array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                                        b"~\x7F^_>",
                                    ),
                                };
                                init
                            },
                            {
                                let mut init = l_array_5_uint8_t {
                                    array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                                        b"'/7?\x1F",
                                    ),
                                };
                                init
                            },
                        ],
                    };
                    init
                },
            ],
        };
        init
    }
};
static mut _ZZ10encode_ise12quant_methodjPKhPhjE5tbits_0: l_array_2_uint8_t = unsafe {
    {
        let mut init = l_array_2_uint8_t {
            array: *::core::mem::transmute::<&[u8; 2], &mut [uint8_t; 2]>(b"\x03\x02"),
        };
        init
    }
};
static mut _ZZ10encode_ise12quant_methodjPKhPhjE6tshift_0: l_array_2_uint8_t = unsafe {
    {
        let mut init = l_array_2_uint8_t {
            array: *::core::mem::transmute::<&[u8; 2], &mut [uint8_t; 2]>(b"\0\x03"),
        };
        init
    }
};
static mut __PRETTY_FUNCTION___OC__Z10decode_ise12quant_methodjPKhPhj: l_array_86_uint8_t = unsafe {
    {
        let mut init = l_array_86_uint8_t {
            array: *::core::mem::transmute::<
                &[u8; 86],
                &mut [uint8_t; 86],
            >(
                b"void decode_ise(quant_method, unsigned int, const uint8_t *, uint8_t *, unsigned int)\0",
            ),
        };
        init
    }
};
static mut _ZZ10decode_ise12quant_methodjPKhPhjE12bits_to_read: l_array_5_uint8_t = unsafe {
    {
        let mut init = l_array_5_uint8_t {
            array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(b"\x02\x02\x01\x02\x01"),
        };
        init
    }
};
static mut _ZZ10decode_ise12quant_methodjPKhPhjE11block_shift: l_array_5_uint8_t = unsafe {
    {
        let mut init = l_array_5_uint8_t {
            array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(b"\0\x02\x04\x05\x07"),
        };
        init
    }
};
static mut _ZZ10decode_ise12quant_methodjPKhPhjE13next_lcounter: l_array_5_uint8_t = unsafe {
    {
        let mut init = l_array_5_uint8_t {
            array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(b"\x01\x02\x03\x04\0"),
        };
        init
    }
};
static mut _ZZ10decode_ise12quant_methodjPKhPhjE13hcounter_incr: l_array_5_uint8_t = unsafe {
    {
        let mut init = l_array_5_uint8_t {
            array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(b"\0\0\0\0\x01"),
        };
        init
    }
};
static mut _ZZ10decode_ise12quant_methodjPKhPhjE12bits_to_read_0: l_array_3_uint8_t = unsafe {
    {
        let mut init = l_array_3_uint8_t {
            array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(b"\x03\x02\x02"),
        };
        init
    }
};
static mut _ZZ10decode_ise12quant_methodjPKhPhjE11block_shift_0: l_array_3_uint8_t = unsafe {
    {
        let mut init = l_array_3_uint8_t {
            array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(b"\0\x03\x05"),
        };
        init
    }
};
static mut _ZZ10decode_ise12quant_methodjPKhPhjE13next_lcounter_0: l_array_3_uint8_t = unsafe {
    {
        let mut init = l_array_3_uint8_t {
            array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(b"\x01\x02\0"),
        };
        init
    }
};
static mut _ZZ10decode_ise12quant_methodjPKhPhjE13hcounter_incr_0: l_array_3_uint8_t = unsafe {
    {
        let mut init = l_array_3_uint8_t {
            array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(b"\0\0\x01"),
        };
        init
    }
};
static mut _OC_str_OC_2: l_array_16_uint8_t = unsafe {
    {
        let mut init = l_array_16_uint8_t {
            array: *::core::mem::transmute::<&[u8; 16], &mut [uint8_t; 16]>(b"trit_blocks > 0\0"),
        };
        init
    }
};
static mut _ZL16trits_of_integer: l_array_256_struct_AC_l_array_5_uint8_t = unsafe {
    {
        let mut init = l_array_256_struct_AC_l_array_5_uint8_t {
            array: [
                {
                    let mut init = l_array_5_uint8_t {
                        array: [
                            0 as core::ffi::c_int as uint8_t,
                            0 as core::ffi::c_int as uint8_t,
                            0 as core::ffi::c_int as uint8_t,
                            0 as core::ffi::c_int as uint8_t,
                            0 as core::ffi::c_int as uint8_t,
                        ],
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\0\0\0\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\0\0\0\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\0\x02\0\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x01\0\0\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x01\0\0\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x01\0\0\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\0\x02\0\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x02\0\0\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x02\0\0\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x02\0\0\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\0\x02\0\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x02\x02\0\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x02\x02\0\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x02\x02\0\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\0\x02\0\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\0\x01\0\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\0\x01\0\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\0\x01\0\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x01\x02\0\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x01\x01\0\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x01\x01\0\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x01\x01\0\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x01\x02\0\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x02\x01\0\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x02\x01\0\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x02\x01\0\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x01\x02\0\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\0\0\x02\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\0\0\x02\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\0\0\x02\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\0\x02\x02\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\0\0\x01\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\0\0\x01\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\0\0\x01\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\0\x02\x01\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x01\0\x01\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x01\0\x01\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x01\0\x01\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\0\x02\x01\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x02\0\x01\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x02\0\x01\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x02\0\x01\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\0\x02\x01\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x02\x02\x01\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x02\x02\x01\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x02\x02\x01\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\0\x02\x01\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\0\x01\x01\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\0\x01\x01\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\0\x01\x01\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x01\x02\x01\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x01\x01\x01\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x01\x01\x01\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x01\x01\x01\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x01\x02\x01\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x02\x01\x01\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x02\x01\x01\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x02\x01\x01\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x01\x02\x01\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x01\0\x02\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x01\0\x02\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x01\0\x02\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\0\x02\x02\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\0\0\x02\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\0\0\x02\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\0\0\x02\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\0\x02\x02\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x01\0\x02\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x01\0\x02\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x01\0\x02\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\0\x02\x02\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x02\0\x02\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x02\0\x02\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x02\0\x02\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\0\x02\x02\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x02\x02\x02\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x02\x02\x02\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x02\x02\x02\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\0\x02\x02\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\0\x01\x02\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\0\x01\x02\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\0\x01\x02\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x01\x02\x02\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x01\x01\x02\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x01\x01\x02\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x01\x01\x02\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x01\x02\x02\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x02\x01\x02\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x02\x01\x02\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x02\x01\x02\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x01\x02\x02\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x02\0\x02\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x02\0\x02\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x02\0\x02\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\0\x02\x02\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\0\0\0\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\0\0\0\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\0\0\0\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\0\x02\0\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x01\0\0\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x01\0\0\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x01\0\0\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\0\x02\0\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x02\0\0\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x02\0\0\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x02\0\0\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\0\x02\0\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x02\x02\0\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x02\x02\0\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x02\x02\0\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\0\x02\0\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\0\x01\0\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\0\x01\0\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\0\x01\0\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x01\x02\0\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x01\x01\0\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x01\x01\0\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x01\x01\0\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x01\x02\0\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x02\x01\0\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x02\x01\0\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x02\x01\0\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x01\x02\0\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x02\x02\x02\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x02\x02\x02\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x02\x02\x02\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\0\x02\x02\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\0\0\0\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\0\0\0\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\0\0\0\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\0\x02\0\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x01\0\0\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x01\0\0\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x01\0\0\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\0\x02\0\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x02\0\0\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x02\0\0\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x02\0\0\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\0\x02\0\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x02\x02\0\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x02\x02\0\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x02\x02\0\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\0\x02\0\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\0\x01\0\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\0\x01\0\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\0\x01\0\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x01\x02\0\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x01\x01\0\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x01\x01\0\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x01\x01\0\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x01\x02\0\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x02\x01\0\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x02\x01\0\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x02\x01\0\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x01\x02\0\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\0\x01\x02\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\0\x01\x02\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\0\x01\x02\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x01\x02\x02\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\0\0\x01\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\0\0\x01\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\0\0\x01\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\0\x02\x01\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x01\0\x01\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x01\0\x01\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x01\0\x01\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\0\x02\x01\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x02\0\x01\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x02\0\x01\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x02\0\x01\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\0\x02\x01\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x02\x02\x01\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x02\x02\x01\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x02\x02\x01\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\0\x02\x01\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\0\x01\x01\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\0\x01\x01\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\0\x01\x01\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x01\x02\x01\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x01\x01\x01\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x01\x01\x01\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x01\x01\x01\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x01\x02\x01\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x02\x01\x01\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x02\x01\x01\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x02\x01\x01\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x01\x02\x01\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x01\x01\x02\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x01\x01\x02\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x01\x01\x02\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x01\x02\x02\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\0\0\x02\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\0\0\x02\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\0\0\x02\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\0\x02\x02\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x01\0\x02\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x01\0\x02\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x01\0\x02\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\0\x02\x02\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x02\0\x02\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x02\0\x02\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x02\0\x02\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\0\x02\x02\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x02\x02\x02\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x02\x02\x02\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x02\x02\x02\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\0\x02\x02\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\0\x01\x02\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\0\x01\x02\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\0\x01\x02\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x01\x02\x02\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x01\x01\x02\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x01\x01\x02\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x01\x01\x02\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x01\x02\x02\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x02\x01\x02\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x02\x01\x02\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x02\x01\x02\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x01\x02\x02\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x02\x01\x02\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x02\x01\x02\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x02\x01\x02\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x01\x02\x02\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\0\0\x01\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\0\0\x01\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\0\0\x01\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\0\x02\x01\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x01\0\x01\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x01\0\x01\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x01\0\x01\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\0\x02\x01\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x02\0\x01\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x02\0\x01\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x02\0\x01\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\0\x02\x01\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x02\x02\x01\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x02\x02\x01\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x02\x02\x01\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\0\x02\x01\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\0\x01\x01\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\0\x01\x01\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\0\x01\x01\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x01\x02\x01\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x01\x01\x01\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x01\x01\x01\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x01\x01\x01\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x01\x02\x01\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x02\x01\x01\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x02\x01\x01\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x02\x01\x01\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x01\x02\x01\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\0\x02\x02\x02\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x01\x02\x02\x02\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x02\x02\x02\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_5_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(
                            b"\x02\x01\x02\x02\x02",
                        ),
                    };
                    init
                },
            ],
        };
        init
    }
};
static mut _OC_str_OC_3: l_array_17_uint8_t = unsafe {
    {
        let mut init = l_array_17_uint8_t {
            array: *::core::mem::transmute::<&[u8; 17], &mut [uint8_t; 17]>(b"quint_blocks > 0\0"),
        };
        init
    }
};
static mut _ZL17quints_of_integer: l_array_128_struct_AC_l_array_3_uint8_t = unsafe {
    {
        let mut init = l_array_128_struct_AC_l_array_3_uint8_t {
            array: [
                {
                    let mut init = l_array_3_uint8_t {
                        array: [
                            0 as core::ffi::c_int as uint8_t,
                            0 as core::ffi::c_int as uint8_t,
                            0 as core::ffi::c_int as uint8_t,
                        ],
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(b"\x01\0\0"),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(b"\x02\0\0"),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(b"\x03\0\0"),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(b"\x04\0\0"),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(b"\0\x04\0"),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x04\x04\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x04\x04\x04",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(b"\0\x01\0"),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x01\x01\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x02\x01\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x03\x01\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x04\x01\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x01\x04\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x04\x04\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x04\x04\x04",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(b"\0\x02\0"),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x01\x02\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x02\x02\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x03\x02\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x04\x02\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x02\x04\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x04\x04\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x04\x04\x04",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(b"\0\x03\0"),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x01\x03\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x02\x03\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x03\x03\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x04\x03\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x03\x04\0",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x04\x04\x03",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x04\x04\x04",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(b"\0\0\x01"),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x01\0\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x02\0\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x03\0\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x04\0\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\0\x04\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x04\0\x04",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\0\x04\x04",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\0\x01\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x01\x01\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x02\x01\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x03\x01\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x04\x01\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x01\x04\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x04\x01\x04",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x01\x04\x04",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\0\x02\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x01\x02\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x02\x02\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x03\x02\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x04\x02\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x02\x04\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x04\x02\x04",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x02\x04\x04",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\0\x03\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x01\x03\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x02\x03\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x03\x03\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x04\x03\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x03\x04\x01",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x04\x03\x04",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x03\x04\x04",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(b"\0\0\x02"),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x01\0\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x02\0\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x03\0\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x04\0\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\0\x04\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x02\0\x04",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x03\0\x04",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\0\x01\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x01\x01\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x02\x01\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x03\x01\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x04\x01\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x01\x04\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x02\x01\x04",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x03\x01\x04",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\0\x02\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x01\x02\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x02\x02\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x03\x02\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x04\x02\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x02\x04\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x02\x02\x04",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x03\x02\x04",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\0\x03\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x01\x03\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x02\x03\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x03\x03\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x04\x03\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x03\x04\x02",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x02\x03\x04",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x03\x03\x04",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(b"\0\0\x03"),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x01\0\x03",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x02\0\x03",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x03\0\x03",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x04\0\x03",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\0\x04\x03",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(b"\0\0\x04"),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x01\0\x04",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\0\x01\x03",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x01\x01\x03",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x02\x01\x03",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x03\x01\x03",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x04\x01\x03",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x01\x04\x03",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\0\x01\x04",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x01\x01\x04",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\0\x02\x03",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x01\x02\x03",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x02\x02\x03",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x03\x02\x03",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x04\x02\x03",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x02\x04\x03",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\0\x02\x04",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x01\x02\x04",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\0\x03\x03",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x01\x03\x03",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x02\x03\x03",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x03\x03\x03",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x04\x03\x03",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x03\x04\x03",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\0\x03\x04",
                        ),
                    };
                    init
                },
                {
                    let mut init = l_array_3_uint8_t {
                        array: *::core::mem::transmute::<&[u8; 3], &mut [uint8_t; 3]>(
                            b"\x01\x03\x04",
                        ),
                    };
                    init
                },
            ],
        };
        init
    }
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
unsafe extern "C" fn llvm_lshr_u8(mut a: uint8_t, mut b: uint8_t) -> uint8_t {
    let mut r: uint8_t = (a as core::ffi::c_int >> b as core::ffi::c_int) as uint8_t;
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

#[inline(never)]
pub unsafe extern "C" fn _Z25get_ise_sequence_bitcountj12quant_method(
    mut _1: uint32_t,
    mut _2: uint32_t,
) -> uint32_t {
    let mut _3: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _4: uint32_t = 0;
    let mut _5: uint32_t = 0;
    let mut _6: uint32_t = 0;
    let mut _7: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _8: uint32_t = 0;
    let mut _9: uint32_t = 0;
    let mut _10: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _11: uint32_t = 0;
    let mut _12: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _13: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _14: uint8_t = 0;
    let mut _15: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _16: uint8_t = 0;
    let mut _17: uint32_t = 0;
    let mut _18: uint32_t = 0;
    let mut _19: uint32_t = 0;
    let mut _20: uint32_t = 0;
    _5 = _1;
    _6 = _2;
    _9 = _6;
    _3 = &_ZL9ise_sizes as *const l_struct_struct_OC_std_KD__KD_array as *mut core::ffi::c_void;
    _10 = _3;
    if _9 as uint64_t >= 21 as core::ffi::c_ulong {
        _4 = 1024 as core::ffi::c_int as uint32_t;
    } else {
        _11 = _6;
        _12 = _ZNKSt5arrayI8ise_sizeLm21EEixEm(
            &_ZL9ise_sizes as *const l_struct_struct_OC_std_KD__KD_array as *mut core::ffi::c_void,
            _11 as uint64_t,
        );
        _7 = _12;
        _13 = _7;
        _14 = *(_13 as *mut uint8_t);
        _8 = llvm_add_u32(
            (llvm_lshr_u8(_14, 6 as core::ffi::c_int as uint8_t) as uint32_t)
                << 1 as core::ffi::c_int,
            1 as core::ffi::c_int as uint32_t,
        );
        _15 = _7;
        _16 = *(_15 as *mut uint8_t);
        _17 = _5;
        _18 = _8;
        _19 = _8;
        _4 = llvm_udiv_u32(
            llvm_sub_u32(
                llvm_add_u32(
                    llvm_mul_u32(
                        llvm_and_u8(_16, 63 as core::ffi::c_int as uint8_t) as uint32_t,
                        _17,
                    ),
                    _18,
                ),
                1 as core::ffi::c_int as uint32_t,
            ),
            _19,
        );
    }
    _20 = _4;
    return _20;
}

#[inline(never)]
pub unsafe extern "C" fn _ZNKSt5arrayI8ise_sizeLm21EEixEm(
    mut _24: *mut core::ffi::c_void,
    mut _25: uint64_t,
) -> *mut core::ffi::c_void {
    let mut _26: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _27: uint64_t = 0;
    let mut _28: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _29: uint64_t = 0;
    _26 = _24;
    _27 = _25;
    _28 = _26;
    _29 = _27;
    return &mut *((*(&mut (*(_28 as *mut l_struct_struct_OC_std_KD__KD_array)).field0
        as *mut l_array_21_struct_AC_l_struct_struct_OC_ise_size))
        .array)
        .as_mut_ptr()
        .offset(_29 as int64_t as isize) as *mut l_struct_struct_OC_ise_size
        as *mut core::ffi::c_void;
}

#[inline(never)]
pub unsafe extern "C" fn _Z10encode_ise12quant_methodjPKhPhj(
    mut _30: uint32_t,
    mut _31: uint32_t,
    mut _32: *mut core::ffi::c_void,
    mut _33: *mut core::ffi::c_void,
    mut _34: uint32_t,
) {
    let mut _35: uint32_t = 0;
    let mut _36: uint32_t = 0;
    let mut _37: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _38: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _39: uint32_t = 0;
    let mut _40: uint32_t = 0;
    let mut _41: uint32_t = 0;
    let mut _42: uint32_t = 0;
    let mut _43: uint32_t = 0;
    let mut _44: uint32_t = 0;
    let mut _45: uint32_t = 0;
    let mut _46: uint32_t = 0;
    let mut _47: uint32_t = 0;
    let mut _48: uint32_t = 0;
    let mut _49: uint32_t = 0;
    let mut _50: uint32_t = 0;
    let mut _51: uint32_t = 0;
    let mut _52: uint8_t = 0;
    let mut _53: uint8_t = 0;
    let mut _54: uint32_t = 0;
    let mut _55: uint32_t = 0;
    let mut _56: uint32_t = 0;
    let mut _57: uint32_t = 0;
    let mut _58: uint32_t = 0;
    let mut _59: uint8_t = 0;
    let mut _60: uint32_t = 0;
    let mut _61: uint8_t = 0;
    let mut _62: uint32_t = 0;
    let mut _63: uint32_t = 0;
    let mut _64: uint32_t = 0;
    let mut _65: uint32_t = 0;
    let mut _66: uint32_t = 0;
    let mut _67: uint32_t = 0;
    let mut _68: uint8_t = 0;
    let mut _69: uint8_t = 0;
    let mut _70: uint32_t = 0;
    let mut _71: uint32_t = 0;
    let mut _72: uint32_t = 0;
    let mut _73: uint8_t = 0;
    let mut _74: uint32_t = 0;
    let mut _75: uint8_t = 0;
    let mut _76: uint32_t = 0;
    let mut _77: uint32_t = 0;
    let mut _78: uint32_t = 0;
    let mut _79: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _80: uint8_t = 0;
    let mut _81: uint32_t = 0;
    let mut _82: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _83: uint8_t = 0;
    let mut _84: uint32_t = 0;
    let mut _85: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _86: uint8_t = 0;
    let mut _87: uint32_t = 0;
    let mut _88: uint32_t = 0;
    let mut _89: uint32_t = 0;
    let mut _90: uint32_t = 0;
    let mut _91: uint32_t = 0;
    let mut _92: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _93: uint32_t = 0;
    let mut _94: uint8_t = 0;
    let mut _95: uint32_t = 0;
    let mut _96: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _97: uint32_t = 0;
    let mut _98: uint8_t = 0;
    let mut _99: uint32_t = 0;
    let mut _100: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _101: uint32_t = 0;
    let mut _102: uint8_t = 0;
    let mut _103: uint32_t = 0;
    let mut _104: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _105: uint32_t = 0;
    let mut _106: uint8_t = 0;
    let mut _107: uint32_t = 0;
    let mut _108: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _109: uint32_t = 0;
    let mut _110: uint8_t = 0;
    let mut _111: uint32_t = 0;
    let mut _112: uint32_t = 0;
    let mut _113: uint32_t = 0;
    let mut _114: uint32_t = 0;
    let mut _115: uint32_t = 0;
    let mut _116: uint32_t = 0;
    let mut _117: uint8_t = 0;
    let mut _118: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _119: uint32_t = 0;
    let mut _120: uint8_t = 0;
    let mut _121: uint32_t = 0;
    let mut _122: uint8_t = 0;
    let mut _123: uint32_t = 0;
    let mut _124: uint8_t = 0;
    let mut _125: uint32_t = 0;
    let mut _126: uint32_t = 0;
    let mut _127: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _128: uint32_t = 0;
    let mut _129: uint32_t = 0;
    let mut _130: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _131: uint32_t = 0;
    let mut _132: uint8_t = 0;
    let mut _133: uint32_t = 0;
    let mut _134: uint8_t = 0;
    let mut _135: uint32_t = 0;
    let mut _136: uint8_t = 0;
    let mut _137: uint32_t = 0;
    let mut _138: uint32_t = 0;
    let mut _139: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _140: uint32_t = 0;
    let mut _141: uint32_t = 0;
    let mut _142: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _143: uint32_t = 0;
    let mut _144: uint8_t = 0;
    let mut _145: uint32_t = 0;
    let mut _146: uint8_t = 0;
    let mut _147: uint32_t = 0;
    let mut _148: uint8_t = 0;
    let mut _149: uint32_t = 0;
    let mut _150: uint32_t = 0;
    let mut _151: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _152: uint32_t = 0;
    let mut _153: uint32_t = 0;
    let mut _154: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _155: uint32_t = 0;
    let mut _156: uint8_t = 0;
    let mut _157: uint32_t = 0;
    let mut _158: uint8_t = 0;
    let mut _159: uint32_t = 0;
    let mut _160: uint8_t = 0;
    let mut _161: uint32_t = 0;
    let mut _162: uint32_t = 0;
    let mut _163: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _164: uint32_t = 0;
    let mut _165: uint32_t = 0;
    let mut _166: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _167: uint32_t = 0;
    let mut _168: uint8_t = 0;
    let mut _169: uint32_t = 0;
    let mut _170: uint8_t = 0;
    let mut _171: uint32_t = 0;
    let mut _172: uint8_t = 0;
    let mut _173: uint32_t = 0;
    let mut _174: uint32_t = 0;
    let mut _175: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _176: uint32_t = 0;
    let mut _177: uint32_t = 0;
    let mut _178: uint32_t = 0;
    let mut _179: uint32_t = 0;
    let mut _180: uint32_t = 0;
    let mut _181: uint32_t = 0;
    let mut _182: uint32_t = 0;
    let mut _183: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _184: uint32_t = 0;
    let mut _185: uint8_t = 0;
    let mut _186: uint32_t = 0;
    let mut _187: uint32_t = 0;
    let mut _188: uint32_t = 0;
    let mut _188__PHI_TEMPORARY: uint32_t = 0;
    let mut _189: uint32_t = 0;
    let mut _190: uint32_t = 0;
    let mut _191: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _192: uint32_t = 0;
    let mut _193: uint8_t = 0;
    let mut _194: uint32_t = 0;
    let mut _195: uint32_t = 0;
    let mut _196: uint32_t = 0;
    let mut _196__PHI_TEMPORARY: uint32_t = 0;
    let mut _197: uint32_t = 0;
    let mut _198: uint32_t = 0;
    let mut _199: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _200: uint32_t = 0;
    let mut _201: uint8_t = 0;
    let mut _202: uint32_t = 0;
    let mut _203: uint32_t = 0;
    let mut _204: uint32_t = 0;
    let mut _204__PHI_TEMPORARY: uint32_t = 0;
    let mut _205: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _206: uint32_t = 0;
    let mut _207: uint8_t = 0;
    let mut _208: uint32_t = 0;
    let mut _209: uint32_t = 0;
    let mut _210: uint32_t = 0;
    let mut _211: uint32_t = 0;
    let mut _212: uint32_t = 0;
    let mut _213: uint32_t = 0;
    let mut _214: uint8_t = 0;
    let mut _215: uint32_t = 0;
    let mut _216: uint32_t = 0;
    let mut _217: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _218: uint32_t = 0;
    let mut _219: uint8_t = 0;
    let mut _220: uint32_t = 0;
    let mut _221: uint8_t = 0;
    let mut _222: uint32_t = 0;
    let mut _223: uint8_t = 0;
    let mut _224: uint32_t = 0;
    let mut _225: uint8_t = 0;
    let mut _226: uint32_t = 0;
    let mut _227: uint8_t = 0;
    let mut _228: uint32_t = 0;
    let mut _229: uint32_t = 0;
    let mut _230: uint8_t = 0;
    let mut _231: uint32_t = 0;
    let mut _232: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _233: uint32_t = 0;
    let mut _234: uint32_t = 0;
    let mut _235: uint8_t = 0;
    let mut _236: uint32_t = 0;
    let mut _237: uint32_t = 0;
    let mut _238: uint32_t = 0;
    let mut _239: uint32_t = 0;
    let mut _240: uint32_t = 0;
    let mut _241: uint32_t = 0;
    let mut _242: uint32_t = 0;
    let mut _243: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _244: uint32_t = 0;
    let mut _245: uint8_t = 0;
    let mut _246: uint32_t = 0;
    let mut _247: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _248: uint32_t = 0;
    let mut _249: uint8_t = 0;
    let mut _250: uint32_t = 0;
    let mut _251: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _252: uint32_t = 0;
    let mut _253: uint8_t = 0;
    let mut _254: uint32_t = 0;
    let mut _255: uint32_t = 0;
    let mut _256: uint32_t = 0;
    let mut _257: uint32_t = 0;
    let mut _258: uint8_t = 0;
    let mut _259: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _260: uint32_t = 0;
    let mut _261: uint8_t = 0;
    let mut _262: uint32_t = 0;
    let mut _263: uint8_t = 0;
    let mut _264: uint32_t = 0;
    let mut _265: uint8_t = 0;
    let mut _266: uint32_t = 0;
    let mut _267: uint32_t = 0;
    let mut _268: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _269: uint32_t = 0;
    let mut _270: uint32_t = 0;
    let mut _271: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _272: uint32_t = 0;
    let mut _273: uint8_t = 0;
    let mut _274: uint32_t = 0;
    let mut _275: uint8_t = 0;
    let mut _276: uint32_t = 0;
    let mut _277: uint8_t = 0;
    let mut _278: uint32_t = 0;
    let mut _279: uint32_t = 0;
    let mut _280: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _281: uint32_t = 0;
    let mut _282: uint32_t = 0;
    let mut _283: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _284: uint32_t = 0;
    let mut _285: uint8_t = 0;
    let mut _286: uint32_t = 0;
    let mut _287: uint8_t = 0;
    let mut _288: uint32_t = 0;
    let mut _289: uint8_t = 0;
    let mut _290: uint32_t = 0;
    let mut _291: uint32_t = 0;
    let mut _292: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _293: uint32_t = 0;
    let mut _294: uint32_t = 0;
    let mut _295: uint32_t = 0;
    let mut _296: uint32_t = 0;
    let mut _297: uint32_t = 0;
    let mut _298: uint32_t = 0;
    let mut _299: uint32_t = 0;
    let mut _300: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _301: uint32_t = 0;
    let mut _302: uint8_t = 0;
    let mut _303: uint32_t = 0;
    let mut _304: uint32_t = 0;
    let mut _305: uint32_t = 0;
    let mut _305__PHI_TEMPORARY: uint32_t = 0;
    let mut _306: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _307: uint32_t = 0;
    let mut _308: uint8_t = 0;
    let mut _309: uint32_t = 0;
    let mut _310: uint32_t = 0;
    let mut _311: uint32_t = 0;
    let mut _312: uint32_t = 0;
    let mut _313: uint8_t = 0;
    let mut _314: uint32_t = 0;
    let mut _315: uint32_t = 0;
    let mut _316: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _317: uint32_t = 0;
    let mut _318: uint8_t = 0;
    let mut _319: uint32_t = 0;
    let mut _320: uint8_t = 0;
    let mut _321: uint32_t = 0;
    let mut _322: uint8_t = 0;
    let mut _323: uint32_t = 0;
    let mut _324: uint8_t = 0;
    let mut _325: uint32_t = 0;
    let mut _326: uint8_t = 0;
    let mut _327: uint32_t = 0;
    let mut _328: uint32_t = 0;
    let mut _329: uint8_t = 0;
    let mut _330: uint32_t = 0;
    let mut _331: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _332: uint32_t = 0;
    let mut _333: uint32_t = 0;
    let mut _334: uint8_t = 0;
    let mut _335: uint32_t = 0;
    let mut _336: uint32_t = 0;
    let mut _337: uint32_t = 0;
    let mut _338: uint32_t = 0;
    let mut _339: uint32_t = 0;
    let mut _340: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _341: uint32_t = 0;
    let mut _342: uint8_t = 0;
    let mut _343: uint32_t = 0;
    let mut _344: uint32_t = 0;
    let mut _345: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _346: uint32_t = 0;
    let mut _347: uint32_t = 0;
    let mut _348: uint32_t = 0;
    _35 = _30;
    _36 = _31;
    _37 = _32;
    _38 = _33;
    _39 = _34;
    _77 = _36;
    if _77 > 0 as core::ffi::c_uint {
        _78 = _35;
        _79 = _ZNKSt5arrayI9btq_countLm21EEixEm(
            &_ZL10btq_counts as *const l_struct_struct_OC_std_KD__KD_array_OC_0
                as *mut core::ffi::c_void,
            _78 as uint64_t,
        );
        _80 = *(_79 as *mut uint8_t);
        _40 = llvm_and_u8(_80, 63 as core::ffi::c_int as uint8_t) as uint32_t;
        _81 = _35;
        _82 = _ZNKSt5arrayI9btq_countLm21EEixEm(
            &_ZL10btq_counts as *const l_struct_struct_OC_std_KD__KD_array_OC_0
                as *mut core::ffi::c_void,
            _81 as uint64_t,
        );
        _83 = *(_82 as *mut uint8_t);
        _41 = llvm_and_u8(
            llvm_lshr_u8(_83, 6 as core::ffi::c_int as uint8_t),
            1 as core::ffi::c_int as uint8_t,
        ) as uint32_t;
        _84 = _35;
        _85 = _ZNKSt5arrayI9btq_countLm21EEixEm(
            &_ZL10btq_counts as *const l_struct_struct_OC_std_KD__KD_array_OC_0
                as *mut core::ffi::c_void,
            _84 as uint64_t,
        );
        _86 = *(_85 as *mut uint8_t);
        _42 = llvm_lshr_u8(_86, 7 as core::ffi::c_int as uint8_t) as uint32_t;
        _87 = _40;
        _43 = llvm_sub_u32(
            ((1 as core::ffi::c_int) << _87) as uint32_t,
            1 as core::ffi::c_int as uint32_t,
        );
        _88 = _41;
        if _88 != 0 as core::ffi::c_uint {
            _44 = 0;
            _89 = _36;
            _45 = llvm_udiv_u32(_89, 5 as core::ffi::c_int as uint32_t);
            _46 = 0;
            loop {
                _90 = _46;
                _91 = _45;
                if !(_90 < _91) {
                    break;
                }
                _92 = _37;
                _93 = _44;
                _94 = *(&mut *(_92 as *mut uint8_t).offset((llvm_add_u32
                    as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                    _93,
                    4 as core::ffi::c_int as uint32_t,
                ) as uint64_t as int64_t
                    as isize) as *mut uint8_t);
                _95 = _40;
                _47 = llvm_ashr_u32(_94 as uint32_t as int32_t, _95 as int32_t);
                _96 = _37;
                _97 = _44;
                _98 = *(&mut *(_96 as *mut uint8_t).offset((llvm_add_u32
                    as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                    _97,
                    3 as core::ffi::c_int as uint32_t,
                ) as uint64_t as int64_t
                    as isize) as *mut uint8_t);
                _99 = _40;
                _48 = llvm_ashr_u32(_98 as uint32_t as int32_t, _99 as int32_t);
                _100 = _37;
                _101 = _44;
                _102 = *(&mut *(_100 as *mut uint8_t).offset((llvm_add_u32
                    as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                    _101,
                    2 as core::ffi::c_int as uint32_t,
                ) as uint64_t
                    as int64_t
                    as isize) as *mut uint8_t);
                _103 = _40;
                _49 = llvm_ashr_u32(_102 as uint32_t as int32_t, _103 as int32_t);
                _104 = _37;
                _105 = _44;
                _106 = *(&mut *(_104 as *mut uint8_t).offset((llvm_add_u32
                    as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                    _105,
                    1 as core::ffi::c_int as uint32_t,
                ) as uint64_t
                    as int64_t
                    as isize) as *mut uint8_t);
                _107 = _40;
                _50 = llvm_ashr_u32(_106 as uint32_t as int32_t, _107 as int32_t);
                _108 = _37;
                _109 = _44;
                _110 = *(&mut *(_108 as *mut uint8_t).offset((llvm_add_u32
                    as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                    _109, 0
                ) as uint64_t
                    as int64_t
                    as isize) as *mut uint8_t);
                _111 = _40;
                _51 = llvm_ashr_u32(_110 as uint32_t as int32_t, _111 as int32_t);
                _112 = _47;
                _113 = _48;
                _114 = _49;
                _115 = _50;
                _116 = _51;
                _117 = *(&mut *((*(&mut *((*(&mut *((*(&mut *((*(&*(_ZL16integer_of_trits
                    .array)
                    .as_ptr()
                    .offset(_112 as uint64_t as int64_t as isize)
                    as *const l_array_3_struct_AC_l_array_3_struct_AC_l_array_3_struct_AC_l_array_3_uint8_t
                    as *mut l_array_3_struct_AC_l_array_3_struct_AC_l_array_3_struct_AC_l_array_3_uint8_t))
                    .array)
                    .as_mut_ptr()
                    .offset(_113 as uint64_t as int64_t as isize)
                    as *mut l_array_3_struct_AC_l_array_3_struct_AC_l_array_3_uint8_t))
                    .array)
                    .as_mut_ptr()
                    .offset(_114 as uint64_t as int64_t as isize)
                    as *mut l_array_3_struct_AC_l_array_3_uint8_t))
                    .array)
                    .as_mut_ptr()
                    .offset(_115 as uint64_t as int64_t as isize)
                    as *mut l_array_3_uint8_t))
                    .array)
                    .as_mut_ptr()
                    .offset(_116 as uint64_t as int64_t as isize) as *mut uint8_t);
                _52 = _117;
                _118 = _37;
                _119 = _44;
                _44 = llvm_add_u32(_119, 1 as core::ffi::c_int as uint32_t);
                _120 = *(&mut *(_118 as *mut uint8_t).offset(_119 as uint64_t as int64_t as isize)
                    as *mut uint8_t);
                _121 = _43;
                _122 = _52;
                _123 = _40;
                _53 = (_120 as uint32_t & _121
                    | (llvm_ashr_u32(_122 as uint32_t as int32_t, 0 as core::ffi::c_int)
                        & 3 as core::ffi::c_int as uint32_t)
                        << _123) as uint8_t;
                _124 = _53;
                _125 = _40;
                _126 = _39;
                _127 = _38;
                _ZL10write_bitsjjjPh(
                    _124 as uint32_t,
                    llvm_add_u32(_125, 2 as core::ffi::c_int as uint32_t),
                    _126,
                    _127,
                );
                _128 = _40;
                _129 = _39;
                _39 = llvm_add_u32(_129, llvm_add_u32(_128, 2 as core::ffi::c_int as uint32_t));
                _130 = _37;
                _131 = _44;
                _44 = llvm_add_u32(_131, 1 as core::ffi::c_int as uint32_t);
                _132 = *(&mut *(_130 as *mut uint8_t).offset(_131 as uint64_t as int64_t as isize)
                    as *mut uint8_t);
                _133 = _43;
                _134 = _52;
                _135 = _40;
                _53 = (_132 as uint32_t & _133
                    | (llvm_ashr_u32(_134 as uint32_t as int32_t, 2 as core::ffi::c_int)
                        & 3 as core::ffi::c_int as uint32_t)
                        << _135) as uint8_t;
                _136 = _53;
                _137 = _40;
                _138 = _39;
                _139 = _38;
                _ZL10write_bitsjjjPh(
                    _136 as uint32_t,
                    llvm_add_u32(_137, 2 as core::ffi::c_int as uint32_t),
                    _138,
                    _139,
                );
                _140 = _40;
                _141 = _39;
                _39 = llvm_add_u32(_141, llvm_add_u32(_140, 2 as core::ffi::c_int as uint32_t));
                _142 = _37;
                _143 = _44;
                _44 = llvm_add_u32(_143, 1 as core::ffi::c_int as uint32_t);
                _144 = *(&mut *(_142 as *mut uint8_t).offset(_143 as uint64_t as int64_t as isize)
                    as *mut uint8_t);
                _145 = _43;
                _146 = _52;
                _147 = _40;
                _53 = (_144 as uint32_t & _145
                    | (llvm_ashr_u32(_146 as uint32_t as int32_t, 4 as core::ffi::c_int)
                        & 1 as core::ffi::c_int as uint32_t)
                        << _147) as uint8_t;
                _148 = _53;
                _149 = _40;
                _150 = _39;
                _151 = _38;
                _ZL10write_bitsjjjPh(
                    _148 as uint32_t,
                    llvm_add_u32(_149, 1 as core::ffi::c_int as uint32_t),
                    _150,
                    _151,
                );
                _152 = _40;
                _153 = _39;
                _39 = llvm_add_u32(_153, llvm_add_u32(_152, 1 as core::ffi::c_int as uint32_t));
                _154 = _37;
                _155 = _44;
                _44 = llvm_add_u32(_155, 1 as core::ffi::c_int as uint32_t);
                _156 = *(&mut *(_154 as *mut uint8_t).offset(_155 as uint64_t as int64_t as isize)
                    as *mut uint8_t);
                _157 = _43;
                _158 = _52;
                _159 = _40;
                _53 = (_156 as uint32_t & _157
                    | (llvm_ashr_u32(_158 as uint32_t as int32_t, 5 as core::ffi::c_int)
                        & 3 as core::ffi::c_int as uint32_t)
                        << _159) as uint8_t;
                _160 = _53;
                _161 = _40;
                _162 = _39;
                _163 = _38;
                _ZL10write_bitsjjjPh(
                    _160 as uint32_t,
                    llvm_add_u32(_161, 2 as core::ffi::c_int as uint32_t),
                    _162,
                    _163,
                );
                _164 = _40;
                _165 = _39;
                _39 = llvm_add_u32(_165, llvm_add_u32(_164, 2 as core::ffi::c_int as uint32_t));
                _166 = _37;
                _167 = _44;
                _44 = llvm_add_u32(_167, 1 as core::ffi::c_int as uint32_t);
                _168 = *(&mut *(_166 as *mut uint8_t).offset(_167 as uint64_t as int64_t as isize)
                    as *mut uint8_t);
                _169 = _43;
                _170 = _52;
                _171 = _40;
                _53 = (_168 as uint32_t & _169
                    | (llvm_ashr_u32(_170 as uint32_t as int32_t, 7 as core::ffi::c_int)
                        & 1 as core::ffi::c_int as uint32_t)
                        << _171) as uint8_t;
                _172 = _53;
                _173 = _40;
                _174 = _39;
                _175 = _38;
                _ZL10write_bitsjjjPh(
                    _172 as uint32_t,
                    llvm_add_u32(_173, 1 as core::ffi::c_int as uint32_t),
                    _174,
                    _175,
                );
                _176 = _40;
                _177 = _39;
                _39 = llvm_add_u32(_177, llvm_add_u32(_176, 1 as core::ffi::c_int as uint32_t));
                _178 = _46;
                _46 = llvm_add_u32(_178, 1 as core::ffi::c_int as uint32_t);
            }
            _179 = _44;
            _180 = _36;
            if _179 != _180 {
                _54 = 0;
                _181 = _44;
                _182 = _36;
                if llvm_add_u32(_181, 3 as core::ffi::c_int as uint32_t) >= _182 {
                    _188__PHI_TEMPORARY = 0;
                } else {
                    _183 = _37;
                    _184 = _44;
                    _185 = *(&mut *(_183 as *mut uint8_t).offset((llvm_add_u32
                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        _184,
                        3 as core::ffi::c_int as uint32_t,
                    ) as uint64_t
                        as int64_t
                        as isize) as *mut uint8_t);
                    _186 = _40;
                    _187 = llvm_ashr_u32(_185 as uint32_t as int32_t, _186 as int32_t);
                    _188__PHI_TEMPORARY = _187;
                }
                _188 = _188__PHI_TEMPORARY;
                _55 = _188;
                _189 = _44;
                _190 = _36;
                if llvm_add_u32(_189, 2 as core::ffi::c_int as uint32_t) >= _190 {
                    _196__PHI_TEMPORARY = 0;
                } else {
                    _191 = _37;
                    _192 = _44;
                    _193 = *(&mut *(_191 as *mut uint8_t).offset((llvm_add_u32
                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        _192,
                        2 as core::ffi::c_int as uint32_t,
                    ) as uint64_t
                        as int64_t
                        as isize) as *mut uint8_t);
                    _194 = _40;
                    _195 = llvm_ashr_u32(_193 as uint32_t as int32_t, _194 as int32_t);
                    _196__PHI_TEMPORARY = _195;
                }
                _196 = _196__PHI_TEMPORARY;
                _56 = _196;
                _197 = _44;
                _198 = _36;
                if llvm_add_u32(_197, 1 as core::ffi::c_int as uint32_t) >= _198 {
                    _204__PHI_TEMPORARY = 0;
                } else {
                    _199 = _37;
                    _200 = _44;
                    _201 = *(&mut *(_199 as *mut uint8_t).offset((llvm_add_u32
                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        _200,
                        1 as core::ffi::c_int as uint32_t,
                    ) as uint64_t
                        as int64_t
                        as isize) as *mut uint8_t);
                    _202 = _40;
                    _203 = llvm_ashr_u32(_201 as uint32_t as int32_t, _202 as int32_t);
                    _204__PHI_TEMPORARY = _203;
                }
                _204 = _204__PHI_TEMPORARY;
                _57 = _204;
                _205 = _37;
                _206 = _44;
                _207 = *(&mut *(_205 as *mut uint8_t).offset((llvm_add_u32
                    as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                    _206, 0
                ) as uint64_t
                    as int64_t
                    as isize) as *mut uint8_t);
                _208 = _40;
                _58 = llvm_ashr_u32(_207 as uint32_t as int32_t, _208 as int32_t);
                _209 = _54;
                _210 = _55;
                _211 = _56;
                _212 = _57;
                _213 = _58;
                _214 = *(&mut *((*(&mut *((*(&mut *((*(&mut *((*(&*(_ZL16integer_of_trits
                    .array)
                    .as_ptr()
                    .offset(_209 as uint64_t as int64_t as isize)
                    as *const l_array_3_struct_AC_l_array_3_struct_AC_l_array_3_struct_AC_l_array_3_uint8_t
                    as *mut l_array_3_struct_AC_l_array_3_struct_AC_l_array_3_struct_AC_l_array_3_uint8_t))
                    .array)
                    .as_mut_ptr()
                    .offset(_210 as uint64_t as int64_t as isize)
                    as *mut l_array_3_struct_AC_l_array_3_struct_AC_l_array_3_uint8_t))
                    .array)
                    .as_mut_ptr()
                    .offset(_211 as uint64_t as int64_t as isize)
                    as *mut l_array_3_struct_AC_l_array_3_uint8_t))
                    .array)
                    .as_mut_ptr()
                    .offset(_212 as uint64_t as int64_t as isize)
                    as *mut l_array_3_uint8_t))
                    .array)
                    .as_mut_ptr()
                    .offset(_213 as uint64_t as int64_t as isize) as *mut uint8_t);
                _59 = _214;
                _60 = 0;
                loop {
                    _215 = _44;
                    _216 = _36;
                    if !(_215 < _216) {
                        break;
                    }
                    _217 = _37;
                    _218 = _44;
                    _219 = *(&mut *(_217 as *mut uint8_t)
                        .offset(_218 as uint64_t as int64_t as isize)
                        as *mut uint8_t);
                    _220 = _43;
                    _221 = _59;
                    _222 = _60;
                    _223 = *(&*(_ZZ10encode_ise12quant_methodjPKhPhjE6tshift.array)
                        .as_ptr()
                        .offset(_222 as uint64_t as int64_t as isize)
                        as *const uint8_t as *mut uint8_t);
                    _224 = _60;
                    _225 = *(&*(_ZZ10encode_ise12quant_methodjPKhPhjE5tbits.array)
                        .as_ptr()
                        .offset(_224 as uint64_t as int64_t as isize)
                        as *const uint8_t as *mut uint8_t);
                    _226 = _40;
                    _61 = (_219 as uint32_t & _220
                        | (llvm_ashr_u32(_221 as uint32_t as int32_t, _223 as uint32_t as int32_t)
                            & llvm_sub_u32(
                                ((1 as core::ffi::c_int) << _225 as uint32_t) as uint32_t,
                                1 as core::ffi::c_int as uint32_t,
                            ))
                            << _226) as uint8_t;
                    _227 = _61;
                    _228 = _40;
                    _229 = _60;
                    _230 = *(&*(_ZZ10encode_ise12quant_methodjPKhPhjE5tbits.array)
                        .as_ptr()
                        .offset(_229 as uint64_t as int64_t as isize)
                        as *const uint8_t as *mut uint8_t);
                    _231 = _39;
                    _232 = _38;
                    _ZL10write_bitsjjjPh(
                        _227 as uint32_t,
                        llvm_add_u32(_228, _230 as uint32_t),
                        _231,
                        _232,
                    );
                    _233 = _40;
                    _234 = _60;
                    _235 = *(&*(_ZZ10encode_ise12quant_methodjPKhPhjE5tbits.array)
                        .as_ptr()
                        .offset(_234 as uint64_t as int64_t as isize)
                        as *const uint8_t as *mut uint8_t);
                    _236 = _39;
                    _39 = llvm_add_u32(_236, llvm_add_u32(_233, _235 as uint32_t));
                    _237 = _44;
                    _44 = llvm_add_u32(_237, 1 as core::ffi::c_int as uint32_t);
                    _238 = _60;
                    _60 = llvm_add_u32(_238, 1 as core::ffi::c_int as uint32_t);
                }
            }
        } else {
            _239 = _42;
            if _239 != 0 as core::ffi::c_uint {
                _62 = 0;
                _240 = _36;
                _63 = llvm_udiv_u32(_240, 3 as core::ffi::c_int as uint32_t);
                _64 = 0;
                loop {
                    _241 = _64;
                    _242 = _63;
                    if !(_241 < _242) {
                        break;
                    }
                    _243 = _37;
                    _244 = _62;
                    _245 = *(&mut *(_243 as *mut uint8_t).offset((llvm_add_u32
                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        _244,
                        2 as core::ffi::c_int as uint32_t,
                    ) as uint64_t
                        as int64_t
                        as isize) as *mut uint8_t);
                    _246 = _40;
                    _65 = llvm_ashr_u32(_245 as uint32_t as int32_t, _246 as int32_t);
                    _247 = _37;
                    _248 = _62;
                    _249 = *(&mut *(_247 as *mut uint8_t).offset((llvm_add_u32
                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        _248,
                        1 as core::ffi::c_int as uint32_t,
                    ) as uint64_t
                        as int64_t
                        as isize) as *mut uint8_t);
                    _250 = _40;
                    _66 = llvm_ashr_u32(_249 as uint32_t as int32_t, _250 as int32_t);
                    _251 = _37;
                    _252 = _62;
                    _253 = *(&mut *(_251 as *mut uint8_t).offset((llvm_add_u32
                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        _252, 0
                    ) as uint64_t
                        as int64_t
                        as isize) as *mut uint8_t);
                    _254 = _40;
                    _67 = llvm_ashr_u32(_253 as uint32_t as int32_t, _254 as int32_t);
                    _255 = _65;
                    _256 = _66;
                    _257 = _67;
                    _258 = *(&mut *((*(&mut *((*(&*(_ZL17integer_of_quints.array)
                        .as_ptr()
                        .offset(_255 as uint64_t as int64_t as isize)
                        as *const l_array_5_struct_AC_l_array_5_uint8_t
                        as *mut l_array_5_struct_AC_l_array_5_uint8_t))
                        .array)
                        .as_mut_ptr()
                        .offset(_256 as uint64_t as int64_t as isize)
                        as *mut l_array_5_uint8_t))
                        .array)
                        .as_mut_ptr()
                        .offset(_257 as uint64_t as int64_t as isize)
                        as *mut uint8_t);
                    _68 = _258;
                    _259 = _37;
                    _260 = _62;
                    _62 = llvm_add_u32(_260, 1 as core::ffi::c_int as uint32_t);
                    _261 = *(&mut *(_259 as *mut uint8_t)
                        .offset(_260 as uint64_t as int64_t as isize)
                        as *mut uint8_t);
                    _262 = _43;
                    _263 = _68;
                    _264 = _40;
                    _69 = (_261 as uint32_t & _262
                        | (llvm_ashr_u32(_263 as uint32_t as int32_t, 0 as core::ffi::c_int)
                            & 7 as core::ffi::c_int as uint32_t)
                            << _264) as uint8_t;
                    _265 = _69;
                    _266 = _40;
                    _267 = _39;
                    _268 = _38;
                    _ZL10write_bitsjjjPh(
                        _265 as uint32_t,
                        llvm_add_u32(_266, 3 as core::ffi::c_int as uint32_t),
                        _267,
                        _268,
                    );
                    _269 = _40;
                    _270 = _39;
                    _39 = llvm_add_u32(_270, llvm_add_u32(_269, 3 as core::ffi::c_int as uint32_t));
                    _271 = _37;
                    _272 = _62;
                    _62 = llvm_add_u32(_272, 1 as core::ffi::c_int as uint32_t);
                    _273 = *(&mut *(_271 as *mut uint8_t)
                        .offset(_272 as uint64_t as int64_t as isize)
                        as *mut uint8_t);
                    _274 = _43;
                    _275 = _68;
                    _276 = _40;
                    _69 = (_273 as uint32_t & _274
                        | (llvm_ashr_u32(_275 as uint32_t as int32_t, 3 as core::ffi::c_int)
                            & 3 as core::ffi::c_int as uint32_t)
                            << _276) as uint8_t;
                    _277 = _69;
                    _278 = _40;
                    _279 = _39;
                    _280 = _38;
                    _ZL10write_bitsjjjPh(
                        _277 as uint32_t,
                        llvm_add_u32(_278, 2 as core::ffi::c_int as uint32_t),
                        _279,
                        _280,
                    );
                    _281 = _40;
                    _282 = _39;
                    _39 = llvm_add_u32(_282, llvm_add_u32(_281, 2 as core::ffi::c_int as uint32_t));
                    _283 = _37;
                    _284 = _62;
                    _62 = llvm_add_u32(_284, 1 as core::ffi::c_int as uint32_t);
                    _285 = *(&mut *(_283 as *mut uint8_t)
                        .offset(_284 as uint64_t as int64_t as isize)
                        as *mut uint8_t);
                    _286 = _43;
                    _287 = _68;
                    _288 = _40;
                    _69 = (_285 as uint32_t & _286
                        | (llvm_ashr_u32(_287 as uint32_t as int32_t, 5 as core::ffi::c_int)
                            & 3 as core::ffi::c_int as uint32_t)
                            << _288) as uint8_t;
                    _289 = _69;
                    _290 = _40;
                    _291 = _39;
                    _292 = _38;
                    _ZL10write_bitsjjjPh(
                        _289 as uint32_t,
                        llvm_add_u32(_290, 2 as core::ffi::c_int as uint32_t),
                        _291,
                        _292,
                    );
                    _293 = _40;
                    _294 = _39;
                    _39 = llvm_add_u32(_294, llvm_add_u32(_293, 2 as core::ffi::c_int as uint32_t));
                    _295 = _64;
                    _64 = llvm_add_u32(_295, 1 as core::ffi::c_int as uint32_t);
                }
                _296 = _62;
                _297 = _36;
                if _296 != _297 {
                    _70 = 0;
                    _298 = _62;
                    _299 = _36;
                    if llvm_add_u32(_298, 1 as core::ffi::c_int as uint32_t) >= _299 {
                        _305__PHI_TEMPORARY = 0;
                    } else {
                        _300 = _37;
                        _301 = _62;
                        _302 = *(&mut *(_300 as *mut uint8_t).offset((llvm_add_u32
                            as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            _301,
                            1 as core::ffi::c_int as uint32_t,
                        )
                            as uint64_t
                            as int64_t
                            as isize) as *mut uint8_t);
                        _303 = _40;
                        _304 = llvm_ashr_u32(_302 as uint32_t as int32_t, _303 as int32_t);
                        _305__PHI_TEMPORARY = _304;
                    }
                    _305 = _305__PHI_TEMPORARY;
                    _71 = _305;
                    _306 = _37;
                    _307 = _62;
                    _308 = *(&mut *(_306 as *mut uint8_t).offset((llvm_add_u32
                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        _307, 0
                    ) as uint64_t
                        as int64_t
                        as isize) as *mut uint8_t);
                    _309 = _40;
                    _72 = llvm_ashr_u32(_308 as uint32_t as int32_t, _309 as int32_t);
                    _310 = _70;
                    _311 = _71;
                    _312 = _72;
                    _313 = *(&mut *((*(&mut *((*(&*(_ZL17integer_of_quints.array)
                        .as_ptr()
                        .offset(_310 as uint64_t as int64_t as isize)
                        as *const l_array_5_struct_AC_l_array_5_uint8_t
                        as *mut l_array_5_struct_AC_l_array_5_uint8_t))
                        .array)
                        .as_mut_ptr()
                        .offset(_311 as uint64_t as int64_t as isize)
                        as *mut l_array_5_uint8_t))
                        .array)
                        .as_mut_ptr()
                        .offset(_312 as uint64_t as int64_t as isize)
                        as *mut uint8_t);
                    _73 = _313;
                    _74 = 0;
                    loop {
                        _314 = _62;
                        _315 = _36;
                        if !(_314 < _315) {
                            break;
                        }
                        _316 = _37;
                        _317 = _62;
                        _318 = *(&mut *(_316 as *mut uint8_t)
                            .offset(_317 as uint64_t as int64_t as isize)
                            as *mut uint8_t);
                        _319 = _43;
                        _320 = _73;
                        _321 = _74;
                        _322 = *(&*(_ZZ10encode_ise12quant_methodjPKhPhjE6tshift_0.array)
                            .as_ptr()
                            .offset(_321 as uint64_t as int64_t as isize)
                            as *const uint8_t as *mut uint8_t);
                        _323 = _74;
                        _324 = *(&*(_ZZ10encode_ise12quant_methodjPKhPhjE5tbits_0.array)
                            .as_ptr()
                            .offset(_323 as uint64_t as int64_t as isize)
                            as *const uint8_t as *mut uint8_t);
                        _325 = _40;
                        _75 = (_318 as uint32_t & _319
                            | (llvm_ashr_u32(
                                _320 as uint32_t as int32_t,
                                _322 as uint32_t as int32_t,
                            ) & llvm_sub_u32(
                                ((1 as core::ffi::c_int) << _324 as uint32_t) as uint32_t,
                                1 as core::ffi::c_int as uint32_t,
                            )) << _325) as uint8_t;
                        _326 = _75;
                        _327 = _40;
                        _328 = _74;
                        _329 = *(&*(_ZZ10encode_ise12quant_methodjPKhPhjE5tbits_0.array)
                            .as_ptr()
                            .offset(_328 as uint64_t as int64_t as isize)
                            as *const uint8_t as *mut uint8_t);
                        _330 = _39;
                        _331 = _38;
                        _ZL10write_bitsjjjPh(
                            _326 as uint32_t,
                            llvm_add_u32(_327, _329 as uint32_t),
                            _330,
                            _331,
                        );
                        _332 = _40;
                        _333 = _74;
                        _334 = *(&*(_ZZ10encode_ise12quant_methodjPKhPhjE5tbits_0.array)
                            .as_ptr()
                            .offset(_333 as uint64_t as int64_t as isize)
                            as *const uint8_t as *mut uint8_t);
                        _335 = _39;
                        _39 = llvm_add_u32(_335, llvm_add_u32(_332, _334 as uint32_t));
                        _336 = _62;
                        _62 = llvm_add_u32(_336, 1 as core::ffi::c_int as uint32_t);
                        _337 = _74;
                        _74 = llvm_add_u32(_337, 1 as core::ffi::c_int as uint32_t);
                    }
                }
            } else {
                _76 = 0;
                loop {
                    _338 = _76;
                    _339 = _36;
                    if !(_338 < _339) {
                        break;
                    }
                    _340 = _37;
                    _341 = _76;
                    _342 = *(&mut *(_340 as *mut uint8_t)
                        .offset(_341 as uint64_t as int64_t as isize)
                        as *mut uint8_t);
                    _343 = _40;
                    _344 = _39;
                    _345 = _38;
                    _ZL10write_bitsjjjPh(_342 as uint32_t, _343, _344, _345);
                    _346 = _40;
                    _347 = _39;
                    _39 = llvm_add_u32(_347, _346);
                    _348 = _76;
                    _76 = llvm_add_u32(_348, 1 as core::ffi::c_int as uint32_t);
                }
            }
        }
        return;
    } else {
        __assert_fail(
            &_OC_str as *const l_array_20_uint8_t as *mut core::ffi::c_void,
            &_OC_str_OC_1 as *const l_array_55_uint8_t as *mut core::ffi::c_void,
            500,
            &__PRETTY_FUNCTION___OC__Z10encode_ise12quant_methodjPKhPhj as *const l_array_86_uint8_t
                as *mut core::ffi::c_void,
        );
    };
}

#[inline(never)]
pub unsafe extern "C" fn _ZNKSt5arrayI9btq_countLm21EEixEm(
    mut _394: *mut core::ffi::c_void,
    mut _395: uint64_t,
) -> *mut core::ffi::c_void {
    let mut _396: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _397: uint64_t = 0;
    let mut _398: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _399: uint64_t = 0;
    _396 = _394;
    _397 = _395;
    _398 = _396;
    _399 = _397;
    return &mut *((*(&mut (*(_398 as *mut l_struct_struct_OC_std_KD__KD_array_OC_0)).field0
        as *mut l_array_21_struct_AC_l_struct_struct_OC_btq_count))
        .array)
        .as_mut_ptr()
        .offset(_399 as int64_t as isize) as *mut l_struct_struct_OC_btq_count
        as *mut core::ffi::c_void;
}
#[inline(never)]
unsafe extern "C" fn _ZL10write_bitsjjjPh(
    mut _400: uint32_t,
    mut _401: uint32_t,
    mut _402: uint32_t,
    mut _403: *mut core::ffi::c_void,
) {
    let mut _404: uint32_t = 0;
    let mut _405: uint32_t = 0;
    let mut _406: uint32_t = 0;
    let mut _407: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _408: uint32_t = 0;
    let mut _409: uint32_t = 0;
    let mut _410: uint32_t = 0;
    let mut _411: uint32_t = 0;
    let mut _412: uint32_t = 0;
    let mut _413: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _414: uint32_t = 0;
    let mut _415: uint32_t = 0;
    let mut _416: uint32_t = 0;
    let mut _417: uint32_t = 0;
    let mut _418: uint32_t = 0;
    let mut _419: uint32_t = 0;
    let mut _420: uint32_t = 0;
    let mut _421: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _422: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _423: uint8_t = 0;
    let mut _424: uint32_t = 0;
    let mut _425: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _426: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _427: uint8_t = 0;
    let mut _428: uint32_t = 0;
    let mut _429: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _430: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _431: uint8_t = 0;
    let mut _432: uint32_t = 0;
    let mut _433: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _434: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _435: uint8_t = 0;
    _404 = _400;
    _405 = _401;
    _406 = _402;
    _407 = _403;
    _409 = _405;
    _408 = llvm_sub_u32(
        ((1 as core::ffi::c_int) << _409) as uint32_t,
        1 as core::ffi::c_int as uint32_t,
    );
    _410 = _408;
    _411 = _404;
    _404 = _411 & _410;
    _412 = _406;
    _413 = _407;
    _407 = &mut *(_413 as *mut uint8_t).offset((llvm_lshr_u32
        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
        _412,
        3 as core::ffi::c_int as uint32_t,
    ) as uint64_t as int64_t as isize) as *mut uint8_t as *mut core::ffi::c_void;
    _414 = _406;
    _406 = _414 & 7 as core::ffi::c_int as uint32_t;
    _415 = _406;
    _416 = _404;
    _404 = _416 << _415;
    _417 = _406;
    _418 = _408;
    _408 = _418 << _417;
    _419 = _408;
    _408 = _419 ^ -(1 as core::ffi::c_int) as uint32_t;
    _420 = _408;
    _421 = _407;
    _422 = _421 as *mut uint8_t as *mut core::ffi::c_void;
    _423 = *(_422 as *mut uint8_t);
    *(_422 as *mut uint8_t) = (_423 as uint32_t & _420) as uint8_t;
    _424 = _404;
    _425 = _407;
    _426 = _425 as *mut uint8_t as *mut core::ffi::c_void;
    _427 = *(_426 as *mut uint8_t);
    *(_426 as *mut uint8_t) = (_427 as uint32_t | _424) as uint8_t;
    _428 = _408;
    _429 = _407;
    _430 = &mut *(_429 as *mut uint8_t).offset(1 as core::ffi::c_int as int64_t as isize)
        as *mut uint8_t as *mut core::ffi::c_void;
    _431 = *(_430 as *mut uint8_t);
    *(_430 as *mut uint8_t) =
        (_431 as uint32_t & llvm_lshr_u32(_428, 8 as core::ffi::c_int as uint32_t)) as uint8_t;
    _432 = _404;
    _433 = _407;
    _434 = &mut *(_433 as *mut uint8_t).offset(1 as core::ffi::c_int as int64_t as isize)
        as *mut uint8_t as *mut core::ffi::c_void;
    _435 = *(_434 as *mut uint8_t);
    *(_434 as *mut uint8_t) =
        (_435 as uint32_t | llvm_lshr_u32(_432, 8 as core::ffi::c_int as uint32_t)) as uint8_t;
}

#[inline(never)]
pub unsafe extern "C" fn _Z10decode_ise12quant_methodjPKhPhj(
    mut _436: uint32_t,
    mut _437: uint32_t,
    mut _438: *mut core::ffi::c_void,
    mut _439: *mut core::ffi::c_void,
    mut _440: uint32_t,
) {
    let mut _441: uint32_t = 0;
    let mut _442: uint32_t = 0;
    let mut _443: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _444: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _445: uint32_t = 0;
    let mut _446: l_array_68_uint8_t = l_array_68_uint8_t { array: [0; 68] };
    let mut _447: l_array_22_uint8_t = l_array_22_uint8_t { array: [0; 22] };
    let mut _448: uint32_t = 0;
    let mut _449: uint32_t = 0;
    let mut _450: uint32_t = 0;
    let mut _451: uint32_t = 0;
    let mut _452: uint32_t = 0;
    let mut _453: uint32_t = 0;
    let mut _454: uint32_t = 0;
    let mut _455: uint32_t = 0;
    let mut _456: uint32_t = 0;
    let mut _457: uint32_t = 0;
    let mut _458: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _459: uint32_t = 0;
    let mut _460: uint32_t = 0;
    let mut _461: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _462: uint32_t = 0;
    let mut _463: uint32_t = 0;
    let mut _464: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _465: uint32_t = 0;
    let mut _466: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _467: uint8_t = 0;
    let mut _468: uint32_t = 0;
    let mut _469: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _470: uint8_t = 0;
    let mut _471: uint32_t = 0;
    let mut _472: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _473: uint8_t = 0;
    let mut _474: uint32_t = 0;
    let mut _475: uint32_t = 0;
    let mut _476: uint32_t = 0;
    let mut _477: uint32_t = 0;
    let mut _478: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _479: uint32_t = 0;
    let mut _480: uint32_t = 0;
    let mut _481: uint32_t = 0;
    let mut _482: uint32_t = 0;
    let mut _483: uint32_t = 0;
    let mut _484: uint32_t = 0;
    let mut _485: uint8_t = 0;
    let mut _486: uint32_t = 0;
    let mut _487: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _488: uint32_t = 0;
    let mut _489: uint32_t = 0;
    let mut _490: uint8_t = 0;
    let mut _491: uint32_t = 0;
    let mut _492: uint32_t = 0;
    let mut _493: uint32_t = 0;
    let mut _494: uint8_t = 0;
    let mut _495: uint32_t = 0;
    let mut _496: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _497: uint8_t = 0;
    let mut _498: uint32_t = 0;
    let mut _499: uint8_t = 0;
    let mut _500: uint32_t = 0;
    let mut _501: uint32_t = 0;
    let mut _502: uint8_t = 0;
    let mut _503: uint32_t = 0;
    let mut _504: uint32_t = 0;
    let mut _505: uint8_t = 0;
    let mut _506: uint32_t = 0;
    let mut _507: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _508: uint32_t = 0;
    let mut _509: uint32_t = 0;
    let mut _510: uint8_t = 0;
    let mut _511: uint32_t = 0;
    let mut _512: uint32_t = 0;
    let mut _513: uint32_t = 0;
    let mut _514: uint8_t = 0;
    let mut _515: uint32_t = 0;
    let mut _516: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _517: uint8_t = 0;
    let mut _518: uint32_t = 0;
    let mut _519: uint8_t = 0;
    let mut _520: uint32_t = 0;
    let mut _521: uint32_t = 0;
    let mut _522: uint8_t = 0;
    let mut _523: uint32_t = 0;
    let mut _524: uint32_t = 0;
    let mut _525: uint32_t = 0;
    let mut _526: uint32_t = 0;
    let mut _527: uint32_t = 0;
    let mut _528: uint32_t = 0;
    let mut _529: uint32_t = 0;
    let mut _530: uint8_t = 0;
    let mut _531: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _532: uint8_t = 0;
    let mut _533: uint32_t = 0;
    let mut _534: uint32_t = 0;
    let mut _535: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _536: uint8_t = 0;
    let mut _537: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _538: uint8_t = 0;
    let mut _539: uint32_t = 0;
    let mut _540: uint32_t = 0;
    let mut _541: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _542: uint8_t = 0;
    let mut _543: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _544: uint8_t = 0;
    let mut _545: uint32_t = 0;
    let mut _546: uint32_t = 0;
    let mut _547: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _548: uint8_t = 0;
    let mut _549: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _550: uint8_t = 0;
    let mut _551: uint32_t = 0;
    let mut _552: uint32_t = 0;
    let mut _553: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _554: uint8_t = 0;
    let mut _555: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _556: uint8_t = 0;
    let mut _557: uint32_t = 0;
    let mut _558: uint32_t = 0;
    let mut _559: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _560: uint8_t = 0;
    let mut _561: uint32_t = 0;
    let mut _562: uint32_t = 0;
    let mut _563: uint32_t = 0;
    let mut _564: uint32_t = 0;
    let mut _565: uint32_t = 0;
    let mut _566: uint32_t = 0;
    let mut _567: uint32_t = 0;
    let mut _568: uint8_t = 0;
    let mut _569: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _570: uint8_t = 0;
    let mut _571: uint32_t = 0;
    let mut _572: uint32_t = 0;
    let mut _573: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _574: uint8_t = 0;
    let mut _575: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _576: uint8_t = 0;
    let mut _577: uint32_t = 0;
    let mut _578: uint32_t = 0;
    let mut _579: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _580: uint8_t = 0;
    let mut _581: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _582: uint8_t = 0;
    let mut _583: uint32_t = 0;
    let mut _584: uint32_t = 0;
    let mut _585: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _586: uint8_t = 0;
    let mut _587: uint32_t = 0;
    let mut _588: uint32_t = 0;
    let mut _589: uint32_t = 0;
    let mut _590: uint32_t = 0;
    let mut _591: uint8_t = 0;
    let mut _592: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _593: uint32_t = 0;
    let mut _594: uint32_t = 0;
    _441 = _436;
    _442 = _437;
    _443 = _438;
    _444 = _439;
    _445 = _440;
    _463 = _442;
    if _463 > 0 as core::ffi::c_uint {
        _464 = memset(
            &mut _447 as *mut l_array_22_uint8_t as *mut core::ffi::c_void,
            0,
            22 as core::ffi::c_int as uint64_t,
        );
        _465 = _441;
        _466 = _ZNKSt5arrayI9btq_countLm21EEixEm(
            &_ZL10btq_counts as *const l_struct_struct_OC_std_KD__KD_array_OC_0
                as *mut core::ffi::c_void,
            _465 as uint64_t,
        );
        _467 = *(_466 as *mut uint8_t);
        _448 = llvm_and_u8(_467, 63 as core::ffi::c_int as uint8_t) as uint32_t;
        _468 = _441;
        _469 = _ZNKSt5arrayI9btq_countLm21EEixEm(
            &_ZL10btq_counts as *const l_struct_struct_OC_std_KD__KD_array_OC_0
                as *mut core::ffi::c_void,
            _468 as uint64_t,
        );
        _470 = *(_469 as *mut uint8_t);
        _449 = llvm_and_u8(
            llvm_lshr_u8(_470, 6 as core::ffi::c_int as uint8_t),
            1 as core::ffi::c_int as uint8_t,
        ) as uint32_t;
        _471 = _441;
        _472 = _ZNKSt5arrayI9btq_countLm21EEixEm(
            &_ZL10btq_counts as *const l_struct_struct_OC_std_KD__KD_array_OC_0
                as *mut core::ffi::c_void,
            _471 as uint64_t,
        );
        _473 = *(_472 as *mut uint8_t);
        _450 = llvm_lshr_u8(_473, 7 as core::ffi::c_int as uint8_t) as uint32_t;
        _451 = 0;
        _452 = 0;
        _453 = 0;
        loop {
            _474 = _453;
            _475 = _442;
            if !(_474 < _475) {
                break;
            }
            _476 = _448;
            _477 = _445;
            _478 = _443;
            _479 = _ZL9read_bitsjjPKh(_476, _477, _478);
            _480 = _453;
            *(&mut *(_446.array)
                .as_mut_ptr()
                .offset(_480 as uint64_t as int64_t as isize) as *mut uint8_t) = _479 as uint8_t;
            _481 = _448;
            _482 = _445;
            _445 = llvm_add_u32(_482, _481);
            _483 = _449;
            if _483 != 0 as core::ffi::c_uint {
                _484 = _451;
                _485 = *(&*(_ZZ10decode_ise12quant_methodjPKhPhjE12bits_to_read.array)
                    .as_ptr()
                    .offset(_484 as uint64_t as int64_t as isize)
                    as *const uint8_t as *mut uint8_t);
                _486 = _445;
                _487 = _443;
                _488 = _ZL9read_bitsjjPKh(_485 as uint32_t, _486, _487);
                _454 = _488;
                _489 = _451;
                _490 = *(&*(_ZZ10decode_ise12quant_methodjPKhPhjE12bits_to_read.array)
                    .as_ptr()
                    .offset(_489 as uint64_t as int64_t as isize)
                    as *const uint8_t as *mut uint8_t);
                _491 = _445;
                _445 = llvm_add_u32(_491, _490 as uint32_t);
                _492 = _454;
                _493 = _451;
                _494 = *(&*(_ZZ10decode_ise12quant_methodjPKhPhjE11block_shift.array)
                    .as_ptr()
                    .offset(_493 as uint64_t as int64_t as isize)
                    as *const uint8_t as *mut uint8_t);
                _495 = _452;
                _496 = &mut *(_447.array)
                    .as_mut_ptr()
                    .offset(_495 as uint64_t as int64_t as isize)
                    as *mut uint8_t as *mut core::ffi::c_void;
                _497 = *(_496 as *mut uint8_t);
                *(_496 as *mut uint8_t) = (_497 as uint32_t | _492 << _494 as uint32_t) as uint8_t;
                _498 = _451;
                _499 = *(&*(_ZZ10decode_ise12quant_methodjPKhPhjE13hcounter_incr.array)
                    .as_ptr()
                    .offset(_498 as uint64_t as int64_t as isize)
                    as *const uint8_t as *mut uint8_t);
                _500 = _452;
                _452 = llvm_add_u32(_500, _499 as uint32_t);
                _501 = _451;
                _502 = *(&*(_ZZ10decode_ise12quant_methodjPKhPhjE13next_lcounter.array)
                    .as_ptr()
                    .offset(_501 as uint64_t as int64_t as isize)
                    as *const uint8_t as *mut uint8_t);
                _451 = _502 as uint32_t;
            }
            _503 = _450;
            if _503 != 0 as core::ffi::c_uint {
                _504 = _451;
                _505 = *(&*(_ZZ10decode_ise12quant_methodjPKhPhjE12bits_to_read_0.array)
                    .as_ptr()
                    .offset(_504 as uint64_t as int64_t as isize)
                    as *const uint8_t as *mut uint8_t);
                _506 = _445;
                _507 = _443;
                _508 = _ZL9read_bitsjjPKh(_505 as uint32_t, _506, _507);
                _455 = _508;
                _509 = _451;
                _510 = *(&*(_ZZ10decode_ise12quant_methodjPKhPhjE12bits_to_read_0.array)
                    .as_ptr()
                    .offset(_509 as uint64_t as int64_t as isize)
                    as *const uint8_t as *mut uint8_t);
                _511 = _445;
                _445 = llvm_add_u32(_511, _510 as uint32_t);
                _512 = _455;
                _513 = _451;
                _514 = *(&*(_ZZ10decode_ise12quant_methodjPKhPhjE11block_shift_0.array)
                    .as_ptr()
                    .offset(_513 as uint64_t as int64_t as isize)
                    as *const uint8_t as *mut uint8_t);
                _515 = _452;
                _516 = &mut *(_447.array)
                    .as_mut_ptr()
                    .offset(_515 as uint64_t as int64_t as isize)
                    as *mut uint8_t as *mut core::ffi::c_void;
                _517 = *(_516 as *mut uint8_t);
                *(_516 as *mut uint8_t) = (_517 as uint32_t | _512 << _514 as uint32_t) as uint8_t;
                _518 = _451;
                _519 = *(&*(_ZZ10decode_ise12quant_methodjPKhPhjE13hcounter_incr_0.array)
                    .as_ptr()
                    .offset(_518 as uint64_t as int64_t as isize)
                    as *const uint8_t as *mut uint8_t);
                _520 = _452;
                _452 = llvm_add_u32(_520, _519 as uint32_t);
                _521 = _451;
                _522 = *(&*(_ZZ10decode_ise12quant_methodjPKhPhjE13next_lcounter_0.array)
                    .as_ptr()
                    .offset(_521 as uint64_t as int64_t as isize)
                    as *const uint8_t as *mut uint8_t);
                _451 = _522 as uint32_t;
            }
            _523 = _453;
            _453 = llvm_add_u32(_523, 1 as core::ffi::c_int as uint32_t);
        }
        _524 = _449;
        if _524 != 0 as core::ffi::c_uint {
            _525 = _442;
            _456 = llvm_udiv_u32(
                llvm_add_u32(_525, 4 as core::ffi::c_int as uint32_t),
                5 as core::ffi::c_int as uint32_t,
            );
            _526 = _456;
            if _526 > 0 as core::ffi::c_uint {
                _457 = 0;
                loop {
                    _527 = _457;
                    _528 = _456;
                    if !(_527 < _528) {
                        break;
                    }
                    _529 = _457;
                    _530 = *(&mut *(_447.array)
                        .as_mut_ptr()
                        .offset(_529 as uint64_t as int64_t as isize)
                        as *mut uint8_t);
                    _458 = &mut *((*(&*(_ZL16trits_of_integer.array)
                        .as_ptr()
                        .offset(_530 as uint64_t as int64_t as isize)
                        as *const l_array_5_uint8_t
                        as *mut l_array_5_uint8_t))
                        .array)
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as int64_t as isize)
                        as *mut uint8_t as *mut core::ffi::c_void;
                    _531 = _458;
                    _532 = *(_531 as *mut uint8_t);
                    _533 = _448;
                    _534 = _457;
                    _535 = &mut *(_446.array).as_mut_ptr().offset((llvm_mul_u32
                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        5 as core::ffi::c_int as uint32_t,
                        _534,
                    ) as uint64_t
                        as int64_t
                        as isize) as *mut uint8_t
                        as *mut core::ffi::c_void;
                    _536 = *(_535 as *mut uint8_t);
                    *(_535 as *mut uint8_t) =
                        (_536 as uint32_t | (_532 as uint32_t) << _533) as uint8_t;
                    _537 = _458;
                    _538 = *(&mut *(_537 as *mut uint8_t)
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut uint8_t);
                    _539 = _448;
                    _540 = _457;
                    _541 = &mut *(_446.array).as_mut_ptr().offset((llvm_add_u32
                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            5 as core::ffi::c_int as uint32_t,
                            _540,
                        ),
                        1 as core::ffi::c_int as uint32_t,
                    ) as uint64_t
                        as int64_t
                        as isize) as *mut uint8_t
                        as *mut core::ffi::c_void;
                    _542 = *(_541 as *mut uint8_t);
                    *(_541 as *mut uint8_t) =
                        (_542 as uint32_t | (_538 as uint32_t) << _539) as uint8_t;
                    _543 = _458;
                    _544 = *(&mut *(_543 as *mut uint8_t)
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut uint8_t);
                    _545 = _448;
                    _546 = _457;
                    _547 = &mut *(_446.array).as_mut_ptr().offset((llvm_add_u32
                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            5 as core::ffi::c_int as uint32_t,
                            _546,
                        ),
                        2 as core::ffi::c_int as uint32_t,
                    ) as uint64_t
                        as int64_t
                        as isize) as *mut uint8_t
                        as *mut core::ffi::c_void;
                    _548 = *(_547 as *mut uint8_t);
                    *(_547 as *mut uint8_t) =
                        (_548 as uint32_t | (_544 as uint32_t) << _545) as uint8_t;
                    _549 = _458;
                    _550 = *(&mut *(_549 as *mut uint8_t)
                        .offset(3 as core::ffi::c_int as int64_t as isize)
                        as *mut uint8_t);
                    _551 = _448;
                    _552 = _457;
                    _553 = &mut *(_446.array).as_mut_ptr().offset((llvm_add_u32
                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            5 as core::ffi::c_int as uint32_t,
                            _552,
                        ),
                        3 as core::ffi::c_int as uint32_t,
                    ) as uint64_t
                        as int64_t
                        as isize) as *mut uint8_t
                        as *mut core::ffi::c_void;
                    _554 = *(_553 as *mut uint8_t);
                    *(_553 as *mut uint8_t) =
                        (_554 as uint32_t | (_550 as uint32_t) << _551) as uint8_t;
                    _555 = _458;
                    _556 = *(&mut *(_555 as *mut uint8_t)
                        .offset(4 as core::ffi::c_int as int64_t as isize)
                        as *mut uint8_t);
                    _557 = _448;
                    _558 = _457;
                    _559 = &mut *(_446.array).as_mut_ptr().offset((llvm_add_u32
                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            5 as core::ffi::c_int as uint32_t,
                            _558,
                        ),
                        4 as core::ffi::c_int as uint32_t,
                    ) as uint64_t
                        as int64_t
                        as isize) as *mut uint8_t
                        as *mut core::ffi::c_void;
                    _560 = *(_559 as *mut uint8_t);
                    *(_559 as *mut uint8_t) =
                        (_560 as uint32_t | (_556 as uint32_t) << _557) as uint8_t;
                    _561 = _457;
                    _457 = llvm_add_u32(_561, 1 as core::ffi::c_int as uint32_t);
                }
            } else {
                __assert_fail(
                    &_OC_str_OC_2 as *const l_array_16_uint8_t as *mut core::ffi::c_void,
                    &_OC_str_OC_1 as *const l_array_55_uint8_t as *mut core::ffi::c_void,
                    710,
                    &__PRETTY_FUNCTION___OC__Z10decode_ise12quant_methodjPKhPhj
                        as *const l_array_86_uint8_t as *mut core::ffi::c_void,
                );
            }
        }
        _562 = _450;
        if _562 != 0 as core::ffi::c_uint {
            _563 = _442;
            _459 = llvm_udiv_u32(
                llvm_add_u32(_563, 2 as core::ffi::c_int as uint32_t),
                3 as core::ffi::c_int as uint32_t,
            );
            _564 = _459;
            if _564 > 0 as core::ffi::c_uint {
                _460 = 0;
                loop {
                    _565 = _460;
                    _566 = _459;
                    if !(_565 < _566) {
                        break;
                    }
                    _567 = _460;
                    _568 = *(&mut *(_447.array)
                        .as_mut_ptr()
                        .offset(_567 as uint64_t as int64_t as isize)
                        as *mut uint8_t);
                    _461 = &mut *((*(&*(_ZL17quints_of_integer.array)
                        .as_ptr()
                        .offset(_568 as uint64_t as int64_t as isize)
                        as *const l_array_3_uint8_t
                        as *mut l_array_3_uint8_t))
                        .array)
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as int64_t as isize)
                        as *mut uint8_t as *mut core::ffi::c_void;
                    _569 = _461;
                    _570 = *(_569 as *mut uint8_t);
                    _571 = _448;
                    _572 = _460;
                    _573 = &mut *(_446.array).as_mut_ptr().offset((llvm_mul_u32
                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        3 as core::ffi::c_int as uint32_t,
                        _572,
                    ) as uint64_t
                        as int64_t
                        as isize) as *mut uint8_t
                        as *mut core::ffi::c_void;
                    _574 = *(_573 as *mut uint8_t);
                    *(_573 as *mut uint8_t) =
                        (_574 as uint32_t | (_570 as uint32_t) << _571) as uint8_t;
                    _575 = _461;
                    _576 = *(&mut *(_575 as *mut uint8_t)
                        .offset(1 as core::ffi::c_int as int64_t as isize)
                        as *mut uint8_t);
                    _577 = _448;
                    _578 = _460;
                    _579 = &mut *(_446.array).as_mut_ptr().offset((llvm_add_u32
                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            3 as core::ffi::c_int as uint32_t,
                            _578,
                        ),
                        1 as core::ffi::c_int as uint32_t,
                    ) as uint64_t
                        as int64_t
                        as isize) as *mut uint8_t
                        as *mut core::ffi::c_void;
                    _580 = *(_579 as *mut uint8_t);
                    *(_579 as *mut uint8_t) =
                        (_580 as uint32_t | (_576 as uint32_t) << _577) as uint8_t;
                    _581 = _461;
                    _582 = *(&mut *(_581 as *mut uint8_t)
                        .offset(2 as core::ffi::c_int as int64_t as isize)
                        as *mut uint8_t);
                    _583 = _448;
                    _584 = _460;
                    _585 = &mut *(_446.array).as_mut_ptr().offset((llvm_add_u32
                        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                        (llvm_mul_u32 as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
                            3 as core::ffi::c_int as uint32_t,
                            _584,
                        ),
                        2 as core::ffi::c_int as uint32_t,
                    ) as uint64_t
                        as int64_t
                        as isize) as *mut uint8_t
                        as *mut core::ffi::c_void;
                    _586 = *(_585 as *mut uint8_t);
                    *(_585 as *mut uint8_t) =
                        (_586 as uint32_t | (_582 as uint32_t) << _583) as uint8_t;
                    _587 = _460;
                    _460 = llvm_add_u32(_587, 1 as core::ffi::c_int as uint32_t);
                }
            } else {
                __assert_fail(
                    &_OC_str_OC_3 as *const l_array_17_uint8_t as *mut core::ffi::c_void,
                    &_OC_str_OC_1 as *const l_array_55_uint8_t as *mut core::ffi::c_void,
                    725 as core::ffi::c_int as uint32_t,
                    &__PRETTY_FUNCTION___OC__Z10decode_ise12quant_methodjPKhPhj
                        as *const l_array_86_uint8_t as *mut core::ffi::c_void,
                );
            }
        }
        _462 = 0;
        loop {
            _588 = _462;
            _589 = _442;
            if !(_588 < _589) {
                break;
            }
            _590 = _462;
            _591 = *(&mut *(_446.array)
                .as_mut_ptr()
                .offset(_590 as uint64_t as int64_t as isize) as *mut uint8_t);
            _592 = _444;
            _593 = _462;
            *(&mut *(_592 as *mut uint8_t).offset(_593 as uint64_t as int64_t as isize)
                as *mut uint8_t) = _591;
            _594 = _462;
            _462 = llvm_add_u32(_594, 1 as core::ffi::c_int as uint32_t);
        }
        return;
    } else {
        __assert_fail(
            &_OC_str as *const l_array_20_uint8_t as *mut core::ffi::c_void,
            &_OC_str_OC_1 as *const l_array_55_uint8_t as *mut core::ffi::c_void,
            658 as core::ffi::c_int as uint32_t,
            &__PRETTY_FUNCTION___OC__Z10decode_ise12quant_methodjPKhPhj as *const l_array_86_uint8_t
                as *mut core::ffi::c_void,
        );
    };
}
#[inline(never)]
unsafe extern "C" fn _ZL9read_bitsjjPKh(
    mut _628: uint32_t,
    mut _629: uint32_t,
    mut _630: *mut core::ffi::c_void,
) -> uint32_t {
    let mut _631: uint32_t = 0;
    let mut _632: uint32_t = 0;
    let mut _633: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _634: uint32_t = 0;
    let mut _635: uint32_t = 0;
    let mut _636: uint32_t = 0;
    let mut _637: uint32_t = 0;
    let mut _638: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _639: uint32_t = 0;
    let mut _640: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _641: uint8_t = 0;
    let mut _642: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
    let mut _643: uint8_t = 0;
    let mut _644: uint32_t = 0;
    let mut _645: uint32_t = 0;
    let mut _646: uint32_t = 0;
    let mut _647: uint32_t = 0;
    let mut _648: uint32_t = 0;
    _631 = _628;
    _632 = _629;
    _633 = _630;
    _636 = _631;
    _634 = llvm_sub_u32(
        ((1 as core::ffi::c_int) << _636) as uint32_t,
        1 as core::ffi::c_int as uint32_t,
    );
    _637 = _632;
    _638 = _633;
    _633 = &mut *(_638 as *mut uint8_t).offset((llvm_lshr_u32
        as unsafe extern "C" fn(uint32_t, uint32_t) -> uint32_t)(
        _637,
        3 as core::ffi::c_int as uint32_t,
    ) as uint64_t as int64_t as isize) as *mut uint8_t as *mut core::ffi::c_void;
    _639 = _632;
    _632 = _639 & 7 as core::ffi::c_int as uint32_t;
    _640 = _633;
    _641 = *(_640 as *mut uint8_t);
    _642 = _633;
    _643 = *(&mut *(_642 as *mut uint8_t).offset(1 as core::ffi::c_int as int64_t as isize)
        as *mut uint8_t);
    _635 = _641 as uint32_t | (_643 as uint32_t) << 8 as core::ffi::c_int;
    _644 = _632;
    _645 = _635;
    _635 = llvm_lshr_u32(_645, _644);
    _646 = _634;
    _647 = _635;
    _635 = _647 & _646;
    _648 = _635;
    return _648;
}
