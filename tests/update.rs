use bytesbox::ByteBox;

#[test]
fn update() {
    let mut byte_box = ByteBox::new();

    let key = b"key";
    let val1 = b"value1";
    let val2 = b"value2";

    byte_box.insert(key, val1);
    assert_eq!(byte_box.get(key), Some(&val1[..]));

    byte_box.insert(key, val2);
    assert_eq!(byte_box.get(key), Some(&val2[..]));
}
