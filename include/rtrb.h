#pragma once

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

typedef struct rtrb rtrb;

rtrb *rtrb_new(size_t capacity);

void rtrb_free(rtrb *rb);

size_t rtrb_write(rtrb *rb, const uint8_t *data, size_t len);

size_t rtrb_read(rtrb *rb, uint8_t *data, size_t len);
