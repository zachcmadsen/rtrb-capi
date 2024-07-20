add_executable(rtrb-test test.c)
target_link_libraries(rtrb-test PRIVATE rtrb::rtrb-shared)

if (WIN32)
    add_custom_command(
        TARGET rtrb-test
        POST_BUILD
        COMMAND "${CMAKE_COMMAND}" -E copy_if_different "$<TARGET_FILE:rtrb::rtrb-shared>" "$<TARGET_FILE_DIR:rtrb-test>"
        VERBATIM
    )
endif()

add_test("rtrb-test" rtrb-test)