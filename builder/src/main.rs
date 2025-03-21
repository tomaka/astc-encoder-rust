use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

fn main() {
    let target_rust_dir = Path::new(&std::env::args().nth(1).unwrap()).to_owned();

    // Content of all the source files.
    let mut source_files = fs::read_dir(target_rust_dir.join("src"))
        .unwrap()
        .map(|r| r.unwrap())
        .map(|source_file| {
            let content = String::from_utf8(fs::read(source_file.path()).unwrap()).unwrap();
            (source_file.path().to_owned(), content)
        })
        .collect::<HashMap<PathBuf, String>>();

    // Replace `libc::` with `core::ffi::` and remove `libc` altogether.
    for (_, source_text) in &mut source_files {
        *source_text = source_text.replace("libc::", "core::ffi::");
        *source_text = source_text.replace("use ::libc", "");
    }

    // Now parse all the source files.
    let mut source_files = source_files
        .into_iter()
        .map(|(path, content)| (path, syn::parse_file(&content).unwrap()))
        .collect::<HashMap<PathBuf, syn::File>>();

    // Each Rust file starts with an `extern "C" {}` block containing some definitions.
    // We remove all of them.
    for src in source_files.values_mut() {
        src.items
            .retain(|item| !matches!(item, syn::Item::ForeignMod(_)));
    }

    // Write the modifications.
    for (target_path, target_content) in source_files {
        let content = quote::ToTokens::into_token_stream(target_content).to_string();
        fs::write(target_path, &content).unwrap();
    }

    std::io::Write::write_all(
        &mut fs::File::options().append(true).open(target_rust_dir.join("lib.rs")).unwrap(),
        quote::quote! {
            unsafe fn memcpy(d: *mut core::ffi::c_void, s: *mut core::ffi::c_void, c: u64) -> *mut core::ffi::c_void { core::ptr::copy_nonoverlapping::<u8>(s.cast_const().cast(), d.cast(), c as usize); d }
            unsafe fn memset(d: *mut core::ffi::c_void, ch: core::ffi::c_int, c: u64) -> *mut core::ffi::c_void { assert!(ch <= 255); core::ptr::write_bytes::<u8>(d.cast(), ch as u8, c as usize); d }
            fn abs(v: u32) -> u32 { v }
            fn cosf(v: f32) -> f32 { v.cos() }
            fn sinf(v: f32) -> f32 { v.sin() }
            fn fabsf(v: f32) -> f32 { v.abs() }
            fn roundf(v: f32) -> f32 { v.round() }
            fn sqrtf(v: f32) -> f32 { v.sqrt() }
            fn logf(v: f32) -> f32 { v.ln() }
            fn fegetround() -> core::ffi::c_uint { 0 }
            fn __assert_fail(_assertion: *mut core::ffi::c_void, _file: *mut core::ffi::c_void, _line: core::ffi::c_uint, _function: *mut core::ffi::c_void) -> ! { panic!() }
            fn LLVMMul_uov(_: core::ffi::c_ulong, a: &mut u64, b: &mut u64, out: &mut u64) -> u8 { let (res, carry) = (*a).overflowing_mul(*b); *out = res; carry as u8 }
            unsafe fn posix_memalign(memptr: *mut core::ffi::c_void, align: u64, size: u64) -> core::ffi::c_int { libc::posix_memalign(memptr as *mut _, align as usize, size as usize) }
            use libc::free;
            unsafe fn _Znwm(size: u64) -> *mut core::ffi::c_void { libc::malloc(size as libc::size_t) }
            unsafe fn _Znam(size: u64) -> *mut core::ffi::c_void { libc::malloc(size as libc::size_t) }
            unsafe fn _ZdaPv(ptr: *mut core::ffi::c_void) { libc::free(ptr) }
            unsafe fn _ZdlPvm(ptr: *mut core::ffi::c_void, _: u64) { libc::free(ptr) }
            unsafe fn _ZSt25__throw_bad_function_callv() -> ! { panic!() }
            unsafe fn _ZSt20__throw_system_errori<T>(_: T) -> ! { panic!() }
            // __cxa_begin_catch is only ever used right before a call to terminate()
            unsafe fn __cxa_begin_catch(_: *mut core::ffi::c_void) -> *mut core::ffi::c_void { core::ptr::null_mut() }
            unsafe fn _ZSt9terminatev() -> ! { panic!() }
            unsafe fn pthread_mutex_lock(mutex: *mut core::ffi::c_void) -> core::ffi::c_uint { libc::pthread_mutex_lock(mutex as *mut _) as core::ffi::c_uint }
            unsafe fn pthread_mutex_unlock(mutex: *mut core::ffi::c_void) -> core::ffi::c_uint { libc::pthread_mutex_unlock(mutex as *mut _) as core::ffi::c_uint }
        }.to_string().as_bytes()
    ).unwrap();
}
