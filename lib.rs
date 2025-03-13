#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

pub mod src {
    pub mod astcenc_averages_and_directions_cbe;
    pub mod astcenc_block_sizes_cbe;
    pub mod astcenc_color_quantize_cbe;
    pub mod astcenc_color_unquantize_cbe;
    pub mod astcenc_compress_symbolic_cbe;
    pub mod astcenc_compute_variance_cbe;
    pub mod astcenc_decompress_symbolic_cbe;
    pub mod astcenc_diagnostic_trace_cbe;
    pub mod astcenc_entry_cbe;
    pub mod astcenc_find_best_partitioning_cbe;
    pub mod astcenc_ideal_endpoints_and_weights_cbe;
    pub mod astcenc_image_cbe;
    pub mod astcenc_integer_sequence_cbe;
    pub mod astcenc_mathlib_cbe;
    pub mod astcenc_mathlib_softfloat_cbe;
    pub mod astcenc_partition_tables_cbe;
    pub mod astcenc_percentile_tables_cbe;
    pub mod astcenc_pick_best_endpoint_format_cbe;
    pub mod astcenc_quantization_cbe;
    pub mod astcenc_symbolic_physical_cbe;
    pub mod astcenc_weight_align_cbe;
    pub mod astcenc_weight_quant_xfer_tables_cbe;
} // mod src
pub mod bindings;
pub use bindings::*;
