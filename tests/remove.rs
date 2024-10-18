use ::bytesbox::ByteBox;

#[test]
fn remove() {
    let mut byte_box = ByteBox::new();

    let key = b"key";
    let val = b"value";

    byte_box.insert(key, val);
    assert_eq!(byte_box.get(key), Some(&val[..]));

    let removed = byte_box.remove(key);
    assert_eq!(removed, Some(val.to_vec()));
    assert_eq!(byte_box.get(key), None);
}
