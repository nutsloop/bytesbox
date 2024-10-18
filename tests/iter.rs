use bytesbox::ByteBox;

#[test]
fn iteration() {
    let mut byte_box = ByteBox::prealloc(4);

    byte_box.insert(b"key1", b"value1");
    byte_box.insert(b"key2", b"value2");
    byte_box.insert(b"key3", b"value3");

    let mut results = Vec::new();

    for (key, val) in byte_box.iter() {
        results.push((key.to_vec(), val.to_vec()));
    }

    assert!(results.contains(&(b"key1".to_vec(), b"value1".to_vec())));
    assert!(results.contains(&(b"key2".to_vec(), b"value2".to_vec())));
    assert!(results.contains(&(b"key3".to_vec(), b"value3".to_vec())));

    assert_eq!(results.len(), 3);
}
