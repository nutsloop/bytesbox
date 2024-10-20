use bytesbox::*;
#[test]
fn collision_handling_with_view_table() {
    let mut byte_box = ByteBox::prealloc(1); // Small capacity to force collisions

    let content_lenght = b"content-lenght";
    let content_lenght_value = b"74892034";
    let content_type = b"content-type";
    let content_type_value = b"text/html";
    let content_disposition = b"content-disposition";
    let content_disposition_value = b"form-data:image; type:file: filename:image.jpg";

    assert!(byte_box.insert(content_lenght, content_lenght_value));
    assert!(byte_box.insert(content_type, content_type_value));
    assert!(byte_box.insert(content_disposition, content_disposition_value));

    // Display the table to visualize the collision
    byte_box.view_table(); // This will show the internal state of the hash map
}
