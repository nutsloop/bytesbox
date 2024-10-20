use bytesbox::ByteBox;
#[test]
fn auto_resize() {
    let mut byte_box = ByteBox::prealloc(1);
    for i in 0..20 {
        byte_box.insert(format!("key{}", i).as_bytes(), b"value");
    }
    assert!(byte_box.allocation() > 16);
}
