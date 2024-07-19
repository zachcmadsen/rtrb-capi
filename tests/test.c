#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#include <rtrb.h>

bool test_write_and_read()
{
    bool passed = true;

    rtrb *rb = rtrb_new(6);

    /*
     * Write and read twice to cover the case where the first and second slices
     * are non-empty.
     */
    int i;
    for (i = 0; i < 2; ++i)
    {
        uint8_t src[] = {1, 2, 3, 4};
        size_t written = rtrb_write(rb, src, sizeof(src));
        if (written != sizeof(src))
        {
            passed = false;
            goto cleanup;
        }

        uint8_t dst[] = {0, 0, 0, 0};
        size_t read = rtrb_read(rb, dst, sizeof(dst));
        if (read != sizeof(dst))
        {
            passed = false;
            goto cleanup;
        }

        size_t j;
        for (j = 0; j < sizeof(dst); ++j)
        {
            if (src[j] != dst[j])
            {
                passed = false;
                goto cleanup;
            }
        }
    }

cleanup:
    rtrb_free(rb);

    return passed;
}

int main(int argc, char *argv[])
{
    bool passed = true;

    passed &= test_write_and_read();

    return passed ? EXIT_SUCCESS : EXIT_FAILURE;
}
