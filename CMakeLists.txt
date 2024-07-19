cmake_minimum_required(VERSION 3.27)

project(rtrb)

set(CMAKE_C_STANDARD 90)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

option(RTRB_BUILD_TESTS "Build rtrb tests" OFF)

# The code to parse link libraries and options from rustc is mostly taken from
# https://github.com/corrosion-rs/corrosion/blob/master/cmake/FindRust.cmake.
execute_process(
	COMMAND cargo rustc --manifest-path ${PROJECT_SOURCE_DIR}/Cargo.toml -- --print=native-static-libs
	ERROR_VARIABLE CARGO_STDERR
)

set(RTRB_STATIC_LINK_LIBRARIES "")
set(RTRB_STATIC_LINK_OPTIONS "")

# Match everything after 'native-static-libs: ' except newline characters.
if(CARGO_STDERR MATCHES "native-static-libs: ([^\r\n]+)\r?\n")
	# The capture group is stored in CMAKE_MATCH_1 (CMAKE_MATCH_0 holds the
	# entire match).
	string(REPLACE " " ";" LIBS_LIST ${CMAKE_MATCH_1})
	
	foreach(LIB ${LIBS_LIST})
		# MSVC flags start with /.
		if(LIB MATCHES "^/")
            list(APPEND RTRB_STATIC_LINK_OPTIONS ${LIB})
        else()
            # Strip '-l' prefix and .lib suffix.
            string(REGEX REPLACE "^-l" "" STRIPPED_LIB ${LIB})
            string(REGEX REPLACE "\.lib$" "" STRIPPED_LIB ${LIB})
            list(APPEND RTRB_STATIC_LINK_LIBRARIES ${STRIPPED_LIB})
        endif()
	endforeach()

	# Corrosion lets CMake figure out which version of msvcrt to link against so
	# it's probably fine to omit them.
	#
	# https://github.com/corrosion-rs/corrosion/pull/537
	list(FILTER RTRB_STATIC_LINK_LIBRARIES EXCLUDE REGEX "^msvcrtd?")
    list(FILTER RTRB_STATIC_LINK_OPTIONS EXCLUDE REGEX "^/defaultlib:msvcrtd?")
else()
	message(FATAL_ERROR "failed to parse linker options from cargo output")
endif()

set(RTRB_SHARED_LIB ${PROJECT_SOURCE_DIR}/target/debug/${CMAKE_SHARED_LIBRARY_PREFIX}rtrb_capi${CMAKE_SHARED_LIBRARY_SUFFIX})
set(RTRB_STATIC_LIB ${PROJECT_SOURCE_DIR}/target/debug/${CMAKE_STATIC_LIBRARY_PREFIX}rtrb_capi${CMAKE_STATIC_LIBRARY_SUFFIX})

add_custom_command(
	OUTPUT ${RTRB_SHARED_LIB} ${RTRB_STATIC_LIB}
	COMMAND cargo build --manifest-path ${PROJECT_SOURCE_DIR}/Cargo.toml
	DEPENDS ${PROJECT_SOURCE_DIR}/src/lib.rs
)

# The shared and static libraries should probably have different custom targets.
# They share the same for now because they're both produced by the same cargo
# build invocation.
add_custom_target(rtrb-capi DEPENDS ${RTRB_SHARED_LIB} ${RTRB_STATIC_LIB})

add_library(rtrb-shared SHARED IMPORTED)
add_library(rtrb::rtrb-shared ALIAS rtrb-shared)
add_dependencies(rtrb-shared rtrb-capi)

set_target_properties(rtrb-shared PROPERTIES IMPORTED_LOCATION ${RTRB_SHARED_LIB})
if(WIN32)
	# Shared libraries on Windows need to specify the location of the DLL import
	# library file in IMPORTED_IMPLIB.
	set_target_properties(rtrb-shared PROPERTIES IMPORTED_IMPLIB ${RTRB_SHARED_LIB}.lib)
endif()

target_include_directories(rtrb-shared INTERFACE ${PROJECT_SOURCE_DIR}/include)

add_library(rtrb-static STATIC IMPORTED)
add_library(rtrb::rtrb-static ALIAS rtrb-static)
add_dependencies(rtrb-static rtrb-capi)

set_target_properties(rtrb-static PROPERTIES IMPORTED_LOCATION ${RTRB_STATIC_LIB})

target_include_directories(rtrb-static INTERFACE ${PROJECT_SOURCE_DIR}/include)

target_link_libraries(rtrb-static INTERFACE ${RTRB_STATIC_LINK_LIBRARIES})
target_link_options(rtrb-static INTERFACE ${RTRB_STATIC_LINK_OPTIONS})

if(RTRB_BUILD_TESTS)
    include(CTest)
    enable_testing()
    add_subdirectory(tests)
endif()