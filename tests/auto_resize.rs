use bytesbox::ByteBox;
#[test]
fn auto_resize() {
    let mut byte_box = ByteBox::new();
    for i in 0..20 {
        byte_box.insert(format!("key{}", i).as_bytes(), b"value");
    }
    assert!(byte_box.allocation() > 16); // The capacity increases as more elements are added
}
