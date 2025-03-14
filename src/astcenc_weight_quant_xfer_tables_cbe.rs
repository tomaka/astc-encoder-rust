use crate::*;

pub type __uint8_t = core::ffi::c_uchar;
pub type __uint16_t = core::ffi::c_ushort;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct l_unnamed_2 {
    pub field0: uint8_t,
    pub field1: uint8_t,
    pub field2: [uint8_t; 30],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_unnamed_3 {
    pub field0: l_unnamed_2,
    pub field1: l_unnamed_2,
    pub field2: l_unnamed_2,
    pub field3: [uint16_t; 65],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct l_unnamed_4 {
    pub field0: uint8_t,
    pub field1: uint8_t,
    pub field2: uint8_t,
    pub field3: [uint8_t; 29],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_unnamed_5 {
    pub field0: l_unnamed_4,
    pub field1: l_unnamed_4,
    pub field2: l_unnamed_4,
    pub field3: [uint16_t; 65],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct l_unnamed_6 {
    pub field0: uint8_t,
    pub field1: uint8_t,
    pub field2: uint8_t,
    pub field3: uint8_t,
    pub field4: [uint8_t; 28],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_unnamed_7 {
    pub field0: l_unnamed_6,
    pub field1: l_unnamed_6,
    pub field2: l_unnamed_6,
    pub field3: [uint16_t; 65],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct l_unnamed_8 {
    pub field0: uint8_t,
    pub field1: uint8_t,
    pub field2: uint8_t,
    pub field3: uint8_t,
    pub field4: uint8_t,
    pub field5: [uint8_t; 27],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_unnamed_9 {
    pub field0: l_unnamed_8,
    pub field1: l_unnamed_8,
    pub field2: l_unnamed_8,
    pub field3: [uint16_t; 65],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct l_unnamed_10 {
    pub field0: uint8_t,
    pub field1: uint8_t,
    pub field2: uint8_t,
    pub field3: uint8_t,
    pub field4: uint8_t,
    pub field5: uint8_t,
    pub field6: [uint8_t; 26],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_unnamed_11 {
    pub field0: l_unnamed_10,
    pub field1: l_unnamed_10,
    pub field2: l_unnamed_10,
    pub field3: [uint16_t; 65],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct l_unnamed_12 {
    pub field0: [uint8_t; 8],
    pub field1: [uint8_t; 24],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_unnamed_13 {
    pub field0: l_unnamed_12,
    pub field1: l_unnamed_12,
    pub field2: l_unnamed_12,
    pub field3: [uint16_t; 65],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct l_unnamed_14 {
    pub field0: [uint8_t; 10],
    pub field1: [uint8_t; 22],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_unnamed_15 {
    pub field0: l_unnamed_14,
    pub field1: l_unnamed_14,
    pub field2: l_unnamed_14,
    pub field3: [uint16_t; 65],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct l_unnamed_16 {
    pub field0: [uint8_t; 12],
    pub field1: [uint8_t; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_unnamed_17 {
    pub field0: l_unnamed_16,
    pub field1: l_unnamed_16,
    pub field2: l_unnamed_16,
    pub field3: [uint16_t; 65],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct l_unnamed_18 {
    pub field0: [uint8_t; 16],
    pub field1: [uint8_t; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_unnamed_19 {
    pub field0: l_unnamed_18,
    pub field1: l_unnamed_18,
    pub field2: l_unnamed_18,
    pub field3: [uint16_t; 65],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct l_unnamed_20 {
    pub field0: [uint8_t; 20],
    pub field1: [uint8_t; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_unnamed_21 {
    pub field0: l_unnamed_20,
    pub field1: l_unnamed_20,
    pub field2: l_unnamed_20,
    pub field3: [uint16_t; 65],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct l_unnamed_22 {
    pub field0: [uint8_t; 24],
    pub field1: [uint8_t; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_unnamed_23 {
    pub field0: l_unnamed_22,
    pub field1: l_unnamed_22,
    pub field2: l_unnamed_22,
    pub field3: [uint16_t; 65],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct l_struct_struct_OC_quant_and_transfer_table {
    pub field0: [uint8_t; 32],
    pub field1: [uint8_t; 32],
    pub field2: [uint8_t; 32],
    pub field3: [uint16_t; 65],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct l_unnamed_1 {
    pub field0: l_unnamed_3,
    pub field1: l_unnamed_5,
    pub field2: l_unnamed_7,
    pub field3: l_unnamed_9,
    pub field4: l_unnamed_11,
    pub field5: l_unnamed_13,
    pub field6: l_unnamed_15,
    pub field7: l_unnamed_17,
    pub field8: l_unnamed_19,
    pub field9: l_unnamed_21,
    pub field10: l_unnamed_23,
    pub field11: l_struct_struct_OC_quant_and_transfer_table,
}
#[no_mangle]
pub static mut quant_and_xfer_tables: l_unnamed_1 = unsafe {
    {
        let mut init = l_unnamed_1 {
            field0: {
                let mut init = l_unnamed_3 {
                    field0: {
                        let mut init = l_unnamed_2 {
                            field0: 0 as core::ffi::c_int as uint8_t,
                            field1: 64 as core::ffi::c_uint as uint8_t,
                            field2: {
                                let mut init = {[
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                    ],
                                };
                                init
                            },
                        };
                        init
                    },
                    field1: {
                        let mut init = l_unnamed_2 {
                            field0: 0 as core::ffi::c_int as uint8_t,
                            field1: 1 as core::ffi::c_int as uint8_t,
                            field2: {
                                let mut init = {[
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                    ],
                                };
                                init
                            },
                        };
                        init
                    },
                    field2: {
                        let mut init = l_unnamed_2 {
                            field0: 0 as core::ffi::c_int as uint8_t,
                            field1: 64 as core::ffi::c_uint as uint8_t,
                            field2: {
                                let mut init = {[
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                    ],
                                };
                                init
                            },
                        };
                        init
                    },
                    field3: {
                        let mut init = {[
                                16384 as core::ffi::c_uint as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                16384 as core::ffi::c_uint as uint16_t,
                            ],
                        };
                        init
                    },
                };
                init
            },
            field1: {
                let mut init = l_unnamed_5 {
                    field0: {
                        let mut init = l_unnamed_4 {
                            field0: 0 as core::ffi::c_int as uint8_t,
                            field1: 32 as core::ffi::c_int as uint8_t,
                            field2: 64 as core::ffi::c_uint as uint8_t,
                            field3: {
                                let mut init = {[
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                    ],
                                };
                                init
                            },
                        };
                        init
                    },
                    field1: {
                        let mut init = l_unnamed_4 {
                            field0: 0 as core::ffi::c_int as uint8_t,
                            field1: 1 as core::ffi::c_int as uint8_t,
                            field2: 2 as core::ffi::c_int as uint8_t,
                            field3: {
                                let mut init = {[
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                    ],
                                };
                                init
                            },
                        };
                        init
                    },
                    field2: {
                        let mut init = l_unnamed_4 {
                            field0: 0 as core::ffi::c_int as uint8_t,
                            field1: 32 as core::ffi::c_int as uint8_t,
                            field2: 64 as core::ffi::c_uint as uint8_t,
                            field3: {
                                let mut init = {[
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                    ],
                                };
                                init
                            },
                        };
                        init
                    },
                    field3: {
                        let mut init = {[
                                8192 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                16384 as core::ffi::c_uint as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                16416 as core::ffi::c_uint as uint16_t,
                            ],
                        };
                        init
                    },
                };
                init
            },
            field2: {
                let mut init = l_unnamed_7 {
                    field0: {
                        let mut init = l_unnamed_6 {
                            field0: 0 as core::ffi::c_int as uint8_t,
                            field1: 21 as core::ffi::c_int as uint8_t,
                            field2: 43 as core::ffi::c_int as uint8_t,
                            field3: 64 as core::ffi::c_uint as uint8_t,
                            field4: {
                                let mut init = {[
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                    ],
                                };
                                init
                            },
                        };
                        init
                    },
                    field1: {
                        let mut init = l_unnamed_6 {
                            field0: 0 as core::ffi::c_int as uint8_t,
                            field1: 1 as core::ffi::c_int as uint8_t,
                            field2: 2 as core::ffi::c_int as uint8_t,
                            field3: 3 as core::ffi::c_int as uint8_t,
                            field4: {
                                let mut init = {[
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                    ],
                                };
                                init
                            },
                        };
                        init
                    },
                    field2: {
                        let mut init = l_unnamed_6 {
                            field0: 0 as core::ffi::c_int as uint8_t,
                            field1: 21 as core::ffi::c_int as uint8_t,
                            field2: 43 as core::ffi::c_int as uint8_t,
                            field3: 64 as core::ffi::c_uint as uint8_t,
                            field4: {
                                let mut init = {[
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                    ],
                                };
                                init
                            },
                        };
                        init
                    },
                    field3: {
                        let mut init = {[
                                5376 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                11008 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                16405 as core::ffi::c_uint as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                16427 as core::ffi::c_uint as uint16_t,
                            ],
                        };
                        init
                    },
                };
                init
            },
            field3: {
                let mut init = l_unnamed_9 {
                    field0: {
                        let mut init = l_unnamed_8 {
                            field0: 0 as core::ffi::c_int as uint8_t,
                            field1: 16 as core::ffi::c_int as uint8_t,
                            field2: 32 as core::ffi::c_int as uint8_t,
                            field3: 48 as core::ffi::c_int as uint8_t,
                            field4: 64 as core::ffi::c_uint as uint8_t,
                            field5: {
                                let mut init = {[
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                    ],
                                };
                                init
                            },
                        };
                        init
                    },
                    field1: {
                        let mut init = l_unnamed_8 {
                            field0: 0 as core::ffi::c_int as uint8_t,
                            field1: 1 as core::ffi::c_int as uint8_t,
                            field2: 2 as core::ffi::c_int as uint8_t,
                            field3: 3 as core::ffi::c_int as uint8_t,
                            field4: 4 as core::ffi::c_int as uint8_t,
                            field5: {
                                let mut init = {[
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                    ],
                                };
                                init
                            },
                        };
                        init
                    },
                    field2: {
                        let mut init = l_unnamed_8 {
                            field0: 0 as core::ffi::c_int as uint8_t,
                            field1: 16 as core::ffi::c_int as uint8_t,
                            field2: 32 as core::ffi::c_int as uint8_t,
                            field3: 48 as core::ffi::c_int as uint8_t,
                            field4: 64 as core::ffi::c_uint as uint8_t,
                            field5: {
                                let mut init = {[
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                    ],
                                };
                                init
                            },
                        };
                        init
                    },
                    field3: {
                        let mut init = {[
                                4096 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                8192 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                12304 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                16416 as core::ffi::c_uint as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                16432 as core::ffi::c_uint as uint16_t,
                            ],
                        };
                        init
                    },
                };
                init
            },
            field4: {
                let mut init = l_unnamed_11 {
                    field0: {
                        let mut init = l_unnamed_10 {
                            field0: 0 as core::ffi::c_int as uint8_t,
                            field1: 12 as core::ffi::c_int as uint8_t,
                            field2: 25 as core::ffi::c_int as uint8_t,
                            field3: 39 as core::ffi::c_int as uint8_t,
                            field4: 52 as core::ffi::c_int as uint8_t,
                            field5: 64 as core::ffi::c_uint as uint8_t,
                            field6: {
                                let mut init = {[
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                    ],
                                };
                                init
                            },
                        };
                        init
                    },
                    field1: {
                        let mut init = l_unnamed_10 {
                            field0: 0 as core::ffi::c_int as uint8_t,
                            field1: 2 as core::ffi::c_int as uint8_t,
                            field2: 4 as core::ffi::c_int as uint8_t,
                            field3: 5 as core::ffi::c_int as uint8_t,
                            field4: 3 as core::ffi::c_int as uint8_t,
                            field5: 1 as core::ffi::c_int as uint8_t,
                            field6: {
                                let mut init = {[
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                    ],
                                };
                                init
                            },
                        };
                        init
                    },
                    field2: {
                        let mut init = l_unnamed_10 {
                            field0: 0 as core::ffi::c_int as uint8_t,
                            field1: 64 as core::ffi::c_uint as uint8_t,
                            field2: 12 as core::ffi::c_int as uint8_t,
                            field3: 52 as core::ffi::c_int as uint8_t,
                            field4: 25 as core::ffi::c_int as uint8_t,
                            field5: 39 as core::ffi::c_int as uint8_t,
                            field6: {
                                let mut init = {[
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                    ],
                                };
                                init
                            },
                        };
                        init
                    },
                    field3: {
                        let mut init = {[
                                3072 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                6400 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                9996 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                13337 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                16423 as core::ffi::c_uint as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                16436 as core::ffi::c_uint as uint16_t,
                            ],
                        };
                        init
                    },
                };
                init
            },
            field5: {
                let mut init = l_unnamed_13 {
                    field0: {
                        let mut init = l_unnamed_12 {
                            field0: {
                                let mut init = {*::core::mem::transmute::<
                                        &[u8; 8],
                                        &mut [uint8_t; 8],
                                    >(b"\0\t\x12\x1B%.7@"),
                                };
                                init
                            },
                            field1: {
                                let mut init = {[
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                    ],
                                };
                                init
                            },
                        };
                        init
                    },
                    field1: {
                        let mut init = l_unnamed_12 {
                            field0: {
                                let mut init = {*::core::mem::transmute::<
                                        &[u8; 8],
                                        &mut [uint8_t; 8],
                                    >(b"\0\x01\x02\x03\x04\x05\x06\x07"),
                                };
                                init
                            },
                            field1: {
                                let mut init = {[
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                    ],
                                };
                                init
                            },
                        };
                        init
                    },
                    field2: {
                        let mut init = l_unnamed_12 {
                            field0: {
                                let mut init = {*::core::mem::transmute::<
                                        &[u8; 8],
                                        &mut [uint8_t; 8],
                                    >(b"\0\t\x12\x1B%.7@"),
                                };
                                init
                            },
                            field1: {
                                let mut init = {[
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                    ],
                                };
                                init
                            },
                        };
                        init
                    },
                    field3: {
                        let mut init = {[
                                2304 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                4608 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                6921 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                9490 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                11803 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                14117 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                16430 as core::ffi::c_uint as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                16439 as core::ffi::c_uint as uint16_t,
                            ],
                        };
                        init
                    },
                };
                init
            },
            field6: {
                let mut init = l_unnamed_15 {
                    field0: {
                        let mut init = l_unnamed_14 {
                            field0: {
                                let mut init = {*::core::mem::transmute::<
                                        &[u8; 10],
                                        &mut [uint8_t; 10],
                                    >(b"\0\x07\x0E\x15\x1C$+29@"),
                                };
                                init
                            },
                            field1: {
                                let mut init = {[
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                    ],
                                };
                                init
                            },
                        };
                        init
                    },
                    field1: {
                        let mut init = l_unnamed_14 {
                            field0: {
                                let mut init = {*::core::mem::transmute::<
                                        &[u8; 10],
                                        &mut [uint8_t; 10],
                                    >(b"\0\x02\x04\x06\x08\t\x07\x05\x03\x01"),
                                };
                                init
                            },
                            field1: {
                                let mut init = {[
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                    ],
                                };
                                init
                            },
                        };
                        init
                    },
                    field2: {
                        let mut init = l_unnamed_14 {
                            field0: {
                                let mut init = {*::core::mem::transmute::<
                                        &[u8; 10],
                                        &mut [uint8_t; 10],
                                    >(b"\0@\x079\x0E2\x15+\x1C$"),
                                };
                                init
                            },
                            field1: {
                                let mut init = {[
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                    ],
                                };
                                init
                            },
                        };
                        init
                    },
                    field3: {
                        let mut init = {[
                                1792 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                3584 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                5383 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                7182 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                9237 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                11036 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                12836 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                14635 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                16434 as core::ffi::c_uint as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                16441 as core::ffi::c_uint as uint16_t,
                            ],
                        };
                        init
                    },
                };
                init
            },
            field7: {
                let mut init = l_unnamed_17 {
                    field0: {
                        let mut init = l_unnamed_16 {
                            field0: {
                                let mut init = {*::core::mem::transmute::<
                                        &[u8; 12],
                                        &mut [uint8_t; 12],
                                    >(b"\0\x05\x0B\x11\x17\x1C$)/5;@"),
                                };
                                init
                            },
                            field1: {
                                let mut init = {[
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                    ],
                                };
                                init
                            },
                        };
                        init
                    },
                    field1: {
                        let mut init = l_unnamed_16 {
                            field0: {
                                let mut init = {*::core::mem::transmute::<
                                        &[u8; 12],
                                        &mut [uint8_t; 12],
                                    >(b"\0\x04\x08\x02\x06\n\x0B\x07\x03\t\x05\x01"),
                                };
                                init
                            },
                            field1: {
                                let mut init = {[
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                    ],
                                };
                                init
                            },
                        };
                        init
                    },
                    field2: {
                        let mut init = l_unnamed_16 {
                            field0: {
                                let mut init = {*::core::mem::transmute::<
                                        &[u8; 12],
                                        &mut [uint8_t; 12],
                                    >(b"\0@\x11/\x05;\x17)\x0B5\x1C$"),
                                };
                                init
                            },
                            field1: {
                                let mut init = {[
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                    ],
                                };
                                init
                            },
                        };
                        init
                    },
                    field3: {
                        let mut init = {[
                                1280 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                2816 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                4357 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                5899 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                7185 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                9239 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                10524 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                12068 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                13609 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                15151 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                16437 as core::ffi::c_uint as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                16443 as core::ffi::c_uint as uint16_t,
                            ],
                        };
                        init
                    },
                };
                init
            },
            field8: {
                let mut init = l_unnamed_19 {
                    field0: {
                        let mut init = l_unnamed_18 {
                            field0: {
                                let mut init = {*::core::mem::transmute::<
                                        &[u8; 16],
                                        &mut [uint8_t; 16],
                                    >(b"\0\x04\x08\x0C\x11\x15\x19\x1D#'+/48<@"),
                                };
                                init
                            },
                            field1: {
                                let mut init = {[
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                    ],
                                };
                                init
                            },
                        };
                        init
                    },
                    field1: {
                        let mut init = l_unnamed_18 {
                            field0: {
                                let mut init = {*::core::mem::transmute::<
                                        &[u8; 16],
                                        &mut [uint8_t; 16],
                                    >(
                                        b"\0\x01\x02\x03\x04\x05\x06\x07\x08\t\n\x0B\x0C\r\x0E\x0F",
                                    ),
                                };
                                init
                            },
                            field1: {
                                let mut init = {[
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                    ],
                                };
                                init
                            },
                        };
                        init
                    },
                    field2: {
                        let mut init = l_unnamed_18 {
                            field0: {
                                let mut init = {*::core::mem::transmute::<
                                        &[u8; 16],
                                        &mut [uint8_t; 16],
                                    >(b"\0\x04\x08\x0C\x11\x15\x19\x1D#'+/48<@"),
                                };
                                init
                            },
                            field1: {
                                let mut init = {[
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                    ],
                                };
                                init
                            },
                        };
                        init
                    },
                    field3: {
                        let mut init = {[
                                1024 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                2048 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                3076 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                4360 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                5388 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                6417 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                7445 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                8985 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                10013 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                11043 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                12071 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                13355 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                14383 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                15412 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                16440 as core::ffi::c_uint as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                16444 as core::ffi::c_uint as uint16_t,
                            ],
                        };
                        init
                    },
                };
                init
            },
            field9: {
                let mut init = l_unnamed_21 {
                    field0: {
                        let mut init = l_unnamed_20 {
                            field0: {
                                let mut init = {*::core::mem::transmute::<
                                        &[u8; 20],
                                        &mut [uint8_t; 20],
                                    >(b"\0\x03\x06\t\r\x10\x13\x17\x1A\x1D#&)-037:=@"),
                                };
                                init
                            },
                            field1: {
                                let mut init = {[
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                    ],
                                };
                                init
                            },
                        };
                        init
                    },
                    field1: {
                        let mut init = l_unnamed_20 {
                            field0: {
                                let mut init = {*::core::mem::transmute::<
                                        &[u8; 20],
                                        &mut [uint8_t; 20],
                                    >(
                                        b"\0\x04\x08\x0C\x10\x02\x06\n\x0E\x12\x13\x0F\x0B\x07\x03\x11\r\t\x05\x01",
                                    ),
                                };
                                init
                            },
                            field1: {
                                let mut init = {[
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                    ],
                                };
                                init
                            },
                        };
                        init
                    },
                    field2: {
                        let mut init = l_unnamed_20 {
                            field0: {
                                let mut init = {*::core::mem::transmute::<
                                        &[u8; 20],
                                        &mut [uint8_t; 20],
                                    >(b"\0@\x100\x03=\x13-\x06:\x17)\t7\x1A&\r3\x1D#"),
                                };
                                init
                            },
                            field1: {
                                let mut init = {[
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                    ],
                                };
                                init
                            },
                        };
                        init
                    },
                    field3: {
                        let mut init = {[
                                768 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                1536 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                2307 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                3334 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                4105 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                4877 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                5904 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                6675 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                7447 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                8986 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                9757 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                10531 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                11558 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                12329 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                13101 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                14128 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                14899 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                15671 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                16442 as core::ffi::c_uint as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                16445 as core::ffi::c_uint as uint16_t,
                            ],
                        };
                        init
                    },
                };
                init
            },
            field10: {
                let mut init = l_unnamed_23 {
                    field0: {
                        let mut init = l_unnamed_22 {
                            field0: {
                                let mut init = {*::core::mem::transmute::<
                                        &[u8; 24],
                                        &mut [uint8_t; 24],
                                    >(
                                        b"\0\x02\x05\x08\x0B\r\x10\x13\x16\x18\x1B\x1E\"%(*-0358;>@",
                                    ),
                                };
                                init
                            },
                            field1: {
                                let mut init = {[
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                    ],
                                };
                                init
                            },
                        };
                        init
                    },
                    field1: {
                        let mut init = l_unnamed_22 {
                            field0: {
                                let mut init = {*::core::mem::transmute::<
                                        &[u8; 24],
                                        &mut [uint8_t; 24],
                                    >(
                                        b"\0\x08\x10\x02\n\x12\x04\x0C\x14\x06\x0E\x16\x17\x0F\x07\x15\r\x05\x13\x0B\x03\x11\t\x01",
                                    ),
                                };
                                init
                            },
                            field1: {
                                let mut init = {[
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                    ],
                                };
                                init
                            },
                        };
                        init
                    },
                    field2: {
                        let mut init = l_unnamed_22 {
                            field0: {
                                let mut init = {*::core::mem::transmute::<
                                        &[u8; 24],
                                        &mut [uint8_t; 24],
                                    >(
                                        b"\0@\x088\x100\x18(\x02>\x0B5\x13-\x1B%\x05;\r3\x16*\x1E\"",
                                    ),
                                };
                                init
                            },
                            field1: {
                                let mut init = {[
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                        0 as core::ffi::c_int as uint8_t,
                                    ],
                                };
                                init
                            },
                        };
                        init
                    },
                    field3: {
                        let mut init = {[
                                512 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                1280 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                2050 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                2821 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                3336 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                4107 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                4877 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                5648 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                6163 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                6934 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                7704 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                8731 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                9502 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                10274 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                10789 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                11560 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                12330 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                13101 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                13616 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                14387 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                15157 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                15928 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                16443 as core::ffi::c_uint as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                16446 as core::ffi::c_uint as uint16_t,
                            ],
                        };
                        init
                    },
                };
                init
            },
            field11: {
                let mut init = l_struct_struct_OC_quant_and_transfer_table {
                    field0: {
                        let mut init = {*::core::mem::transmute::<
                                &[u8; 32],
                                &mut [uint8_t; 32],
                            >(
                                b"\0\x02\x04\x06\x08\n\x0C\x0E\x10\x12\x14\x16\x18\x1A\x1C\x1E\"$&(*,.02468:<>@",
                            ),
                        };
                        init
                    },
                    field1: {
                        let mut init = {*::core::mem::transmute::<
                                &[u8; 32],
                                &mut [uint8_t; 32],
                            >(
                                b"\0\x01\x02\x03\x04\x05\x06\x07\x08\t\n\x0B\x0C\r\x0E\x0F\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1A\x1B\x1C\x1D\x1E\x1F",
                            ),
                        };
                        init
                    },
                    field2: {
                        let mut init = {*::core::mem::transmute::<
                                &[u8; 32],
                                &mut [uint8_t; 32],
                            >(
                                b"\0\x02\x04\x06\x08\n\x0C\x0E\x10\x12\x14\x16\x18\x1A\x1C\x1E\"$&(*,.02468:<>@",
                            ),
                        };
                        init
                    },
                    field3: {
                        let mut init = {[
                                512 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                1024 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                1538 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                2052 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                2566 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                3080 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                3594 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                4108 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                4622 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                5136 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                5650 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                6164 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                6678 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                7192 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                7706 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                8732 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                9246 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                9762 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                10276 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                10790 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                11304 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                11818 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                12332 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                12846 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                13360 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                13874 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                14388 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                14902 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                15416 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                15930 as core::ffi::c_int as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                16444 as core::ffi::c_uint as uint16_t,
                                0 as core::ffi::c_int as uint16_t,
                                16446 as core::ffi::c_uint as uint16_t,
                            ],
                        };
                        init
                    },
                };
                init
            },
        };
        init
    }
};
