#pragma once

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C"
{
#endif

/*
 * A lock-free ring buffer.
 * 
 * It's safe to use an rtrb from two threads simultaneously with one thread
 * writing to the buffer and one reading from it.
 */
typedef struct rtrb rtrb;

/*
 * Creates a new ring buffer with the given capacity.
 */
rtrb *rtrb_new(size_t capacity);

/*
 * Frees the given ring buffer.
 *
 * It's undefined behavior to free the same ring buffer more than once.
 */
void rtrb_free(rtrb *rb);

/*
 * Writes up to len bytes from data to the ring buffer and returns the number of
 * bytes written.
 */
size_t rtrb_write(rtrb *rb, const uint8_t *data, size_t len);

/*
 * Returns the number of bytes available for writing.
 */
size_t rtrb_write_available(rtrb *rb);

/*
 * Reads up to len bytes from the ring buffer into data and returns the number
 * of bytes read.
 */
size_t rtrb_read(rtrb *rb, uint8_t *data, size_t len);

/*
 * Returns the number of bytes available for reading.
 */
size_t rtrb_read_available(rtrb *rb);

#ifdef __cplusplus
}
#endif
