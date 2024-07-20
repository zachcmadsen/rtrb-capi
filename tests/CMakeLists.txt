# add_executable(test-shared test.c)
# target_link_libraries(test-shared PRIVATE rtrb::rtrb-shared)

# if (WIN32)
#     add_custom_command(
#         TARGET test-shared
#         POST_BUILD
#         COMMAND "${CMAKE_COMMAND}" -E copy_if_different "$<TARGET_FILE:rtrb-shared>" "$<TARGET_FILE_DIR:test-shared>"
#         VERBATIM
#     )
# endif()

add_executable(rtrb-test test.c)
target_link_libraries(rtrb-test PRIVATE rtrb::rtrb-shared)

add_test("rtrb-test" rtrb-test)