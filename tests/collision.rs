use bytesbox::*;
#[test]
fn collision_handling_with_view_table() {
    let mut byte_box = ByteBox::prealloc(3); // Small capacity to force collisions

    // Two different keys that we expect to collide
    let key1 = b"content-lenght"; // First key
    let val1 = b"74892034"; // Value associated with key1
    let key2 = b"content-type"; // Second key
    let val2 = b"text/html"; // Value associated with key2

    // Insert both keys into the ByteBox
    assert!(byte_box.insert(key1, val1)); // Insert key1
    assert!(byte_box.insert(key2, val2)); // Insert key2, expect collision with key1

    // Check that both keys exist in the hash map and can be retrieved correctly
    assert_eq!(byte_box.get(key1), Some(&val1[..])); // key1 should return value1
    assert_eq!(byte_box.get(key2), Some(&val2[..])); // key2 should return value2

    // Check that the length of the ByteBox is correct (i.e., 2 key-value pairs)
    assert_eq!(byte_box.len(), 2);

    // Display the table to visualize the collision
    byte_box.view_table(); // This will show the internal state of the hash map
}
