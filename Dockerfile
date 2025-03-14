FROM debian:trixie

RUN apt update -q
RUN apt install -yq ninja-build git cmake rustup build-essential llvm clang libclang-dev libssl-dev pkg-config python3

# Compile and install LLVM-CBE
WORKDIR /root
RUN apt install -yq llvm-19-dev clang-19
RUN git clone https://github.com/JuliaHubOSS/llvm-cbe
WORKDIR llvm-cbe
RUN git checkout 732f15aa9a7f7f63e2acdcb9b9836de70ee74135
RUN mkdir build
WORKDIR build
RUN cmake -S .. -G "Ninja"
RUN ninja llvm-cbe
ENV PATH="$PATH:/root/llvm-cbe/build/tools/llvm-cbe"

# Install Rust, bindgen, and c2rust
RUN rustup default stable
RUN cargo install c2rust@0.20.0
RUN cargo install bindgen-cli@0.71.0
ENV PATH="$PATH:/root/.cargo/bin"

# Clone astc-encoder
WORKDIR /root
RUN git clone https://github.com/ARM-software/astc-encoder
WORKDIR astc-encoder
RUN git checkout b5b87efbf679a36abbf03cbb1177982d60ac8d5b
# Generate a dummy version header (normally built by CMake variable expansion)
WORKDIR Source
RUN echo "#pragma once" > astcenccli_version.h
RUN echo "#define VERSION_STRING \"0.0.0\"" >> astcenccli_version.h
RUN echo "#define YEAR_STRING \"2021\"" >> astcenccli_version.h
# The `astcenc_vecmathlib_none_4.h` file contains calls to `nearbyint`, which is unfortunately unimplemented in `llvm-cbe`. Replacing it with `round` fixes the issue.
# See <https://github.com/ARM-software/astc-encoder/blob/b5b87efbf679a36abbf03cbb1177982d60ac8d5b/Source/astcenc_vecmathlib_none_4.h#L869-L873>.
# Because the code does `assert(std::fegetround() == FE_TONEAREST);` right before, replacing with `round` doesn't even change the behaviour.
RUN sed -i 's/std::nearbyint/std::round/' astcenc_vecmathlib_none_4.h

# Now do the transpiling
WORKDIR /root
RUN mkdir output
WORKDIR output
# Compile all the C++ files with `-emit-llvm`
# We don't compile the `astcenccli_` files, as we don't need the CLI
RUN for f in /root/astc-encoder/Source/astcenc_*.cpp; do clang -fno-builtin -D ASTCENC_NEON=0 -D ASTCENC_SVE=0 -D ASTCENC_SSE=0 -D ASTCENC_AVX=0 -D ASTCENC_POPCNT=0 -D ASTCENC_F16C=0 -mcpu=generic -fno-slp-vectorize -S -emit-llvm $f; done
# Call `llvm-cbe` to turn all the LLVM IR files into C
RUN for f in *.ll; do llvm-cbe $f; done
# Unfortunately, including `math.h` seems to break c2rust due to some weird stuff in `math-vector.h`. Thankfully, this is easily bypassed.
# See also <https://github.com/immunant/c2rust/issues/1204>.
RUN sed -i 's/define __ADVSIMD_VEC_MATH_SUPPORTED//' /usr/include/aarch64-linux-gnu/bits/math-vector.h
# Create a dummy CMake project for the C code just to be able to intercept the build configuration and later pass it to c2rust
RUN echo "cmake_minimum_required(VERSION 3.15)" >> CMakeLists.txt
RUN echo "project(astcencoder)" >> CMakeLists.txt
RUN echo "add_library(AstcEncoder `ls -1a *.cbe.c`)" >> CMakeLists.txt
RUN mkdir build && cd build && cmake -DCMAKE_C_COMPILER=clang -DCMAKE_EXPORT_COMPILE_COMMANDS=1 ..

# Create a directory for the final output
RUN mkdir final_output
# Turn the C code into Rust
# We need to increase the stack size to avoid stack overflow issues
RUN prlimit --stack=67108864 c2rust transpile --output-dir final_output --emit-build-files build/compile_commands.json
WORKDIR final_output
# For some reason, c2rust emits a rust-toolchain file to pin the Rust version to a specific nightly, but this seems completely unnecessary
RUN rm rust-toolchain.toml

# Run bindgen
RUN bindgen /root/astc-encoder/Source/astcenc.h -o bindings.rs --use-core --with-derive-hash --with-derive-partialeq --with-derive-eq -- -xc++
RUN echo "pub mod bindings;" >> lib.rs
RUN echo "pub use bindings::*;" >> lib.rs

# Do some more tweaks to the Rust code
# Each Rust file starts with an `extern "C" {}` block containing some definitions. We remove all of them.
RUN cd src && for f in *.rs; do sed -i '/extern \"C\" {/,/}/d' $f; done
# Remove casting of numeric constants which causes signed/unsigned types issues
RUN cd src && for f in *.rs; do sed -i 's/\([0-9]*\) as libc::c_int as uint[0-9]*_t/\1/g' $f; done
RUN cd src && for f in *.rs; do sed -i 's/\([0-9]*\) as libc::c_int as libc::c_ulong/\1/g' $f; done
# Replace `libc::` with `core::ffi::` and remove `libc` altogether
RUN cd src && for f in *.rs; do sed -i 's/libc::/core::ffi::/g' $f; done
RUN cd src && for f in *.rs; do sed -i 's/use ::libc;//' $f; done
RUN sed -i 's/extern crate libc;//' lib.rs
# Remove all `#[no_mangle]` attributes
RUN cd src && for f in *.rs; do sed -i 's/#[no_mangle]//' $f; done
# Each Rust module imports every other public symbol of every other module
# This is done by adding `use crate::*;` at the head of each file, and `use src::foo::*;` in lib.rs for each module
RUN cd src && for f in *.rs; do sed -i '1 i\use crate::*;' $f; done
RUN cd src && for f in *.rs; do echo "use src::`echo $f | sed s/\.rs//`::*;" >> ../lib.rs; done

# There are many structs named `l_array_*_uint*_t` which have a single field with an array.
# These structs are redefined locally in each module. Unfortunately, when a function that returns
# a struct like this is called from a different module, the Rust compiler will yield a compilation
# error due to the returned struct being different from the locally-defined one, even though they
# are identical.
# To solve this, we replace all these structs with their array equivalent.
#RUN cd src && for f in *.rs; do sed -i -e ':a' -e 'N' -e '$!ba' -e 's/#\[derive(Copy, Clone)\]\n#\[repr(C)\]\npub struct l_array_[0-9]*_[_a-zA-Z0-9]* {\n[ ]*pub array: \[[:_a-zA-Z0-9]*; [0-9]*\],\n}\n//g' $f; done
#RUN cd src && for f in *.rs; do sed -i -e ':a' -e 'N' -e '$!ba' -e 's/l_array_[0-9]*_[_a-zA-Z0-9]*[[:space:]]*{[[:space:]]*array: /{/g' $f; done
#RUN cd src && for f in *.rs; do sed -i 's/l_array_\([0-9]*\)_\([_a-zA-Z0-9]*\)/[\2; \1]/g' $f; done

# Add some substitutes to C/C++ functions in `lib.rs`
RUN echo "unsafe fn memcpy(d: *mut core::ffi::c_void, s: *mut core::ffi::c_void, c: u64) -> *mut core::ffi::c_void { core::ptr::copy_nonoverlapping::<u8>(s.cast_const().cast(), d.cast(), c as usize); d }" >> lib.rs
RUN echo "unsafe fn memset(d: *mut core::ffi::c_void, ch: core::ffi::c_int, c: u64) -> *mut core::ffi::c_void { assert!(ch <= 255); core::ptr::write_bytes::<u8>(d.cast(), ch as u8, c as usize); d }" >> lib.rs
RUN echo "fn abs(v: u32) -> u32 { v }" >> lib.rs
RUN echo "fn cosf(v: f32) -> f32 { v.cos() }" >> lib.rs
RUN echo "fn sinf(v: f32) -> f32 { v.sin() }" >> lib.rs
RUN echo "fn fabsf(v: f32) -> f32 { v.abs() }" >> lib.rs
RUN echo "fn roundf(v: f32) -> f32 { v.round() }" >> lib.rs
RUN echo "fn sqrtf(v: f32) -> f32 { v.sqrt() }" >> lib.rs
RUN echo "fn logf(v: f32) -> f32 { v.ln() }" >> lib.rs
RUN echo "fn fegetround() -> core::ffi::c_uint { 0 }" >> lib.rs
RUN echo "fn __assert_fail(_assertion: *mut core::ffi::c_void, _file: *mut core::ffi::c_void, _line: core::ffi::c_uint, _function: *mut core::ffi::c_void) -> ! { panic!() }" >> lib.rs
RUN echo "fn LLVMMul_uov(_: core::ffi::c_ulong, a: &mut u64, b: &mut u64, out: &mut u64) -> u8 { let (res, carry) = (*a).overflowing_mul(*b); *out = res; carry as u8 }" >> lib.rs
RUN echo "use libc::posix_memalign;" >> lib.rs
RUN echo "use libc::free;" >> lib.rs
RUN echo "unsafe fn _Znwm(size: u64) -> *mut core::ffi::c_void { libc::malloc(size as libc::size_t) }" >> lib.rs
RUN echo "unsafe fn _Znam(size: u64) -> *mut core::ffi::c_void { libc::malloc(size as libc::size_t) }" >> lib.rs
RUN echo "unsafe fn _ZdaPv(ptr: *mut core::ffi::c_void) { libc::free(ptr) }" >> lib.rs
RUN echo "unsafe fn _ZdlPvm(ptr: *mut core::ffi::c_void, _: u64) { libc::free(ptr) }" >> lib.rs
RUN echo "unsafe fn _ZSt25__throw_bad_function_callv() -> ! { panic!() }" >> lib.rs
RUN echo "unsafe fn _ZSt20__throw_system_errori<T>(_: T) -> ! { panic!() }" >> lib.rs
RUN echo "use libc::pthread_mutex_lock;" >> lib.rs
RUN echo "use libc::pthread_mutex_unlock;" >> lib.rs

# Rustfmt
RUN cargo fmt
