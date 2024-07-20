// Note: We assume that usize is compatible with size_t. See
// https://github.com/dtolnay/cxx/pull/576.

use rtrb::{chunks::ChunkError, CopyToUninit};

type Producer = rtrb::Producer<u8>;
type Consumer = rtrb::Consumer<u8>;

pub struct RingBuffer {
    producer: Producer,
    consumer: Consumer,
}

#[no_mangle]
pub extern "C" fn rtrb_new(capacity: usize) -> *mut RingBuffer {
    let (producer, consumer) = rtrb::RingBuffer::new(capacity);
    let rb = RingBuffer { producer, consumer };
    Box::into_raw(Box::new(rb))
}

#[no_mangle]
pub extern "C" fn rtrb_free(rb: *mut RingBuffer) {
    if rb.is_null() {
        return;
    }

    unsafe { drop(Box::from_raw(rb)) }
}

#[no_mangle]
pub extern "C" fn rtrb_write(
    rb: *mut RingBuffer,
    data: *const u8,
    len: usize,
) -> usize {
    if rb.is_null() || data.is_null() {
        return 0;
    }

    let rb = unsafe { &mut *rb };
    let mut chunk = match rb.producer.write_chunk_uninit(len) {
        Ok(chunk) => chunk,
        Err(ChunkError::TooFewSlots(available_slots)) => {
            rb.producer.write_chunk_uninit(available_slots).unwrap()
        }
    };

    let slice = unsafe { std::slice::from_raw_parts(data, len) };

    let (first, second) = chunk.as_mut_slices();
    let mid = first.len();
    let end = mid + second.len();

    slice[..mid].copy_to_uninit(first);
    slice[mid..end].copy_to_uninit(second);

    unsafe { chunk.commit_all() };

    end
}

#[no_mangle]
pub extern "C" fn rtrb_write_available(rb: *mut RingBuffer) -> usize {
    if rb.is_null() {
        return 0;
    }

    let rb = unsafe { &mut *rb };
    rb.producer.slots()
}

#[no_mangle]
pub extern "C" fn rtrb_read(
    rb: *mut RingBuffer,
    data: *mut u8,
    len: usize,
) -> usize {
    if rb.is_null() || data.is_null() {
        return 0;
    }

    let rb = unsafe { &mut *rb };
    let chunk = match rb.consumer.read_chunk(len) {
        Ok(chunk) => chunk,
        Err(ChunkError::TooFewSlots(available_slots)) => {
            rb.consumer.read_chunk(available_slots).unwrap()
        }
    };

    let slice = unsafe { std::slice::from_raw_parts_mut(data, len) };

    let (first, second) = chunk.as_slices();
    let mid = first.len();
    let end = mid + second.len();

    slice[..mid].copy_from_slice(first);
    slice[mid..end].copy_from_slice(second);

    chunk.commit_all();

    end
}

#[no_mangle]
pub extern "C" fn rtrb_read_available(rb: *mut RingBuffer) -> usize {
    if rb.is_null() {
        return 0;
    }

    let rb = unsafe { &mut *rb };
    rb.consumer.slots()
}
