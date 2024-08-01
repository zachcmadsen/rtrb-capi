#include <array>
#include <cstdint>

#include <catch2/catch_test_macros.hpp>
#include <rtrb.h>

TEST_CASE("write_and_read")
{
    auto *rb = rtrb_new(6);

    REQUIRE(rtrb_write_available(rb) == 6);
    REQUIRE(rtrb_read_available(rb) == 0);

    // Write and read twice to cover the case where the first and second slices
    // are non-empty.
    for (int i = 0; i < 2; ++i)
    {
        std::array<std::uint8_t, 4> src{1, 2, 3, 4};
        const auto written = rtrb_write(rb, src.data(), src.size());
        REQUIRE(written == src.size());

        std::array<std::uint8_t, 4> dst{0, 0, 0, 0};
        const auto read = rtrb_read(rb, dst.data(), dst.size());
        REQUIRE(read == dst.size());

        REQUIRE(src == dst);
    }

    REQUIRE(rtrb_write_available(rb) == 6);
    REQUIRE(rtrb_read_available(rb) == 0);

    rtrb_free(rb);
}

TEST_CASE("write_none_available")
{
    auto *rb = rtrb_new(0);

    std::array<std::uint8_t, 4> buf{1, 2, 3, 4};
    const auto written = rtrb_write(rb, buf.data(), buf.size());
    REQUIRE(written == 0);

    rtrb_free(rb);
}

TEST_CASE("read_none_available")
{
    auto *rb = rtrb_new(0);

    std::array<std::uint8_t, 4> buf{1, 2, 3, 4};
    const auto read = rtrb_read(rb, buf.data(), buf.size());
    REQUIRE(read == 0);

    rtrb_free(rb);
}
