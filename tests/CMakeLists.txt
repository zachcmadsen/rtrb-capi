add_executable(rtrb-test test.c)
target_link_libraries(rtrb-test PRIVATE rtrb::rtrb)

add_test("rtrb-test" rtrb-test)