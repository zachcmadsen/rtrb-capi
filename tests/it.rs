use rtrb_capi::*;

#[test]
fn write_and_read() {
    let rb = rtrb_new(6);

    assert_eq!(rtrb_write_available(rb), 6);
    assert_eq!(rtrb_read_available(rb), 0);

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

    assert_eq!(rtrb_write_available(rb), 6);
    assert_eq!(rtrb_read_available(rb), 0);

    rtrb_free(rb);
}

#[test]
fn write_none_available() {
    let rb = rtrb_new(0);

    let buf = [1, 2, 3, 4];
    let written = rtrb_write(rb, buf.as_ptr(), buf.len());
    assert_eq!(written, 0);

    rtrb_free(rb);
}

#[test]
fn read_none_available() {
    let rb = rtrb_new(0);

    let mut buf = [1, 2, 3, 4];
    let read = rtrb_read(rb, buf.as_mut_ptr(), buf.len());
    assert_eq!(read, 0);

    rtrb_free(rb);
}
