# rtrb-capi

A C API for [`rtrb`][rtrb], a thread-safe SPSC ring buffer written in Rust.

The C API only covers a part of `rtrb`'s [chunk API][rtrb-chunks], and the
buffer only holds unsigned 8-bit integers.

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
    GIT_REPOSITORY  https://github.com/zachcmadsen/rtrb-capi
    GIT_TAG main
)
FetchContent_MakeAvailable(rtrb)

add_executable(rtrb-example main.cpp)
target_link_libraries(rtrb-example PRIVATE rtrb::rtrb-static) # or rtrb::rtrb-shared
```

[rtrb]: https://github.com/mgeier/rtrb
[rtrb-chunks]: https://docs.rs/rtrb/latest/rtrb/chunks/index.html
[include/rtrb.h]: https://github.com/zachcmadsen/rtrb-capi/blob/main/include/rtrb.h
[rtrb-capi-example]: https://github.com/zachcmadsen/rtrb-capi-example