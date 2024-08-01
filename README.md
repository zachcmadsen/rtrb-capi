# rtrb-capi

[![CI][ci_badge]][ci]

A C API for [`rtrb`][rtrb], a thread-safe SPSC ring buffer written in Rust.

The C API only covers part of `rtrb`'s API.

## Documentation

The API is documented in [`include/rtrb.h`][include/rtrb.h].

See [`rtrb-capi-example`][rtrb-capi-example] for an example C++ project that
uses `rtrb-capi`.

## Usage

You need Rust and Cargo installed to build `rtrb-capi`.

If your project uses CMake, then you can include `rtrb-capi` with
`FetchContent`:

```cmake
include(FetchContent)
FetchContent_Declare(
    rtrb
    URL https://github.com/zachcmadsen/rtrb-capi/releases/download/v0.1.0/rtrb-src.tar.gz
)
FetchContent_MakeAvailable(rtrb)

add_executable(rtrb-example)
target_sources(rtrb-example PRIVATE main.cpp)
target_link_libraries(rtrb-example PRIVATE rtrb::rtrb)
```

<!-- TODO: Add instructions for using it without CMake. -->

[ci_badge]: https://github.com/zachcmadsen/rtrb-capi/workflows/CI/badge.svg
[ci]: https://github.com/zachcmadsen/rtrb-capi/actions?query=branch%3Amain

[rtrb]: https://github.com/mgeier/rtrb
[rtrb-chunks]: https://docs.rs/rtrb/latest/rtrb/chunks/index.html
[include/rtrb.h]: https://github.com/zachcmadsen/rtrb-capi/blob/main/include/rtrb.h
[rtrb-capi-example]: https://github.com/zachcmadsen/rtrb-capi-example