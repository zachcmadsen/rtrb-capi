cmake_minimum_required(VERSION 3.27)

project(
    rtrb
    VERSION 0.1.1
)

include(FetchContent)
FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    # I got errors related to msvcrt linker options on v0.5.0, the latest
    # published release as of 2024-07-29. This commit removes msvcrt linker
    # options, leaving it up to CMake pick the right msvcrt.
    # 
    # Commit: https://github.com/corrosion-rs/corrosion/commit/b45275c8b5ee16118008a6a140ce842557b3e47a
    GIT_TAG b45275c8b5ee16118008a6a140ce842557b3e47a
)
message(STATUS "Fetching Corrosion...")
FetchContent_MakeAvailable(Corrosion)

corrosion_import_crate(MANIFEST_PATH Cargo.toml)

add_library(rtrb::rtrb ALIAS rtrb_capi)
target_include_directories(rtrb_capi INTERFACE include)
target_compile_features(rtrb_capi INTERFACE c_std_90)

# Define the test target when the project isn't used as a library, i.e., when it
# isn't a subdirectory.
if(CMAKE_CURRENT_SOURCE_DIR STREQUAL CMAKE_SOURCE_DIR)
    option(RTRB_BUILD_TESTS "Build rtrb tests" ON)

    if(RTRB_BUILD_TESTS)
        include(CTest)
        enable_testing()
        add_subdirectory(tests)
    endif()
endif()