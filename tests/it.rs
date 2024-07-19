use rtrb_capi::*;

#[test]
fn write_and_read() {
    let rb = rtrb_new(6);

    // Write and read twice to cover the case where the first and second slices
    // are non-empty.
    for _ in 0..2 {
        let src = [1, 2, 3, 4];
        let written = rtrb_write(rb, src.as_ptr(), src.len());
        assert_eq!(written, 4);

        let mut dst = [0; 4];
        let read = rtrb_read(rb, dst.as_mut_ptr(), dst.len());
        assert_eq!(read, 4);

        assert_eq!(src, dst);
    }

    rtrb_free(rb);
}
