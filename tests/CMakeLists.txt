Include(FetchContent)
FetchContent_Declare(
  Catch2
  GIT_REPOSITORY https://github.com/catchorg/Catch2.git
  GIT_TAG        v3.6.0
)
message(STATUS "Fetching Catch2...")
FetchContent_MakeAvailable(Catch2)

add_executable(tests)
target_sources(tests PRIVATE tests.cpp)
target_link_libraries(tests PRIVATE rtrb::rtrb Catch2::Catch2WithMain)
target_compile_features(tests PRIVATE cxx_std_17)

include(Catch)
catch_discover_tests(tests)