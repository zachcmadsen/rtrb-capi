cmake_minimum_required(VERSION 3.27)

project(rtrb)

# option(RTRB_BUILD_TESTS "Build rtrb tests" OFF)

include(FetchContent)
FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.5
)
message(STATUS "Fetching Corrosion...")
FetchContent_MakeAvailable(Corrosion)

corrosion_import_crate(MANIFEST_PATH Cargo.toml)

add_library(rtrb::rtrb ALIAS rtrb)
target_include_directories(rtrb INTERFACE include)

# if(RTRB_BUILD_TESTS)
#     include(CTest)
#     enable_testing()
#     add_subdirectory(tests)
# endif()