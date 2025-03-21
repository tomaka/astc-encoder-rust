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

# Now add and compile a (pre-existing) Rust program that will do some twaeak to the generated Rust code
# Note that this should in theory be done earlier in the Dockerfile, but putting it later means
# that tweaks to this builder will not invalidate the Dockerfile layers cache
WORKDIR /root
RUN mkdir builder
WORKDIR builder
COPY . .
RUN cargo build --release

# Do the tweaks
RUN cargo run --release -- /root/output/final_output

# Rustfmt
WORKDIR /root/output/final_output
RUN cargo fmt
