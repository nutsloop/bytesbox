use bytesbox::*;
#[test]
fn insert_new_key_value() {
    let mut byte_box = ByteBox::prealloc(1);

    let key1 = b"key1";
    let val1 = b"value1";

    byte_box.insert(key1, val1);

    assert_eq!(byte_box.get(key1), Some(&val1[..]));
}

#[test]
fn insert_update_value() {
    let mut byte_box = ByteBox::prealloc(1);

    let key1 = b"key1";
    let val1 = b"value1";

    byte_box.insert(key1, val1);

    assert_eq!(byte_box.get(key1), Some(&val1[..]));

    let updated = b"value updated";
    assert!(!byte_box.insert(key1, b"value updated"));
    assert_eq!(byte_box.get(key1), Some(&updated[..]));
}
