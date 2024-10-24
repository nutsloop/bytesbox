use bytesbox::ByteBox;

#[test]
fn clear() {
    let mut byte_box = ByteBox::new();
    byte_box.insert(b"dynamic-resizing", b"true");
    byte_box.insert_primitive(b"font-size", 54);

    assert_eq!(byte_box.len(), 2usize);

    byte_box.clear();

    assert_eq!(byte_box.len(), 0usize);
}
