Note: this doesn't link yet, as many C/C++ symbols are undefined and need to be substituted manually.

# ASTC encoder transpiled to Rust

This repository contains a version of https://github.com/ARM-software/astc-encoder/ transpiled to Rust using [c2rust](https://github.com/immunant/c2rust).

Performances haven't been measured. The objective of this repository is to have something that can easily be compiled for any platform, which is something that Rust does well.

Caveat: building with debug symbols enabled makes Rust overflow its stack due to the large number of local variables being used. Building without debug symbols works fine.
You might want to add this to your Cargo.toml:

```
[profile.dev.package."astc-encoder-rust"]
debug = 0
[profile.test.package."astc-encoder-rust"]
debug = 0
```

## Build

In order to re-generate this code, do the following:

```
docker build -t test .
docker create --name dummy test
docker cp dummy:/root/output/final_output .
docker rm dummy
docker rmi test
```

All version numbers and commits are pinned in the Dockerfile. Updating any version
needs to be done manually by updating the Dockerfile.

Unfortunately, the generated Rust project doesn't compile immediately, and needs some tweaks:

- Two calls to `abs(a)` where `a` is a `u32` have been replaced with just `a`.
- Some unaligned fields pointer grabbing have been fixed using the `&raw mut` syntax.
- One [call to `LLVMMul_uov`](https://github.com/JuliaHubOSS/llvm-cbe/blob/732f15aa9a7f7f63e2acdcb9b9836de70ee74135/lib/Target/CBackend/CBackend.cpp#L4640-L4643), which is equivalent to [`std::instrinsics::mul_with_overflow`](https://doc.rust-lang.org/std/intrinsics/fn.mul_with_overflow.html), was replaced with [`u32::overflowing_mul`](https://doc.rust-lang.org/std/primitive.u32.html#method.overflowing_mul).
- Many functions were each defined multiple times. All `#[no_mangle]` were removed.
- Bindings were adjusted to directly call functions, rather than using `#[link_name]`.
- `libc::` can be replaced with `core::ffi::`.

# License

This code is licensed under the Apache 2.0 license, the same as https://github.com/ARM-software/astc-encoder/.
