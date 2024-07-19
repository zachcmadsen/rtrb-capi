#pragma once

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

/*
 * A ring buffer.
 */
typedef struct rtrb rtrb;

/*
 * Creates a new ring buffer.
 */
rtrb *rtrb_new(size_t capacity);

/*
 * Frees the ring buffer `rb`.
 *
 * It's undefined behavior to call the function more than once for the same
 * pointer.
 */
void rtrb_free(rtrb *rb);

/*
 * Writes up to `len` bytes from `data` to `rb`, and returns the number of bytes
 * written.
 */
size_t rtrb_write(rtrb *rb, const uint8_t *data, size_t len);

/*
 * Reads up to `len` bytes from `rb` into `data`, and returns the number of
 * bytes read.
 */
size_t rtrb_read(rtrb *rb, uint8_t *data, size_t len);
