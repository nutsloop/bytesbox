# ByteBox Crate

The `ByteBox` crate provides a custom hash map implementation optimized for byte slices (`Vec<u8>`). It allows you to map keys of type `Vec<u8>` to values of type `Vec<u8>`, offering an efficient way to work with raw byte data without unnecessary cloning or allocations.

## Features

- **Custom Hash Function (`hpulse`)**: Uses a bespoke hash function optimized for hashing byte slices.
- **Collision Resolution via Linked Lists**: Handles hash collisions using linked lists (chaining), ensuring access to all entries even when collisions occur.
- **Dynamic Resizing**: Automatically resizes the underlying storage when the load factor exceeds a predefined threshold, maintaining optimal performance.
- **Customizable Initial Capacity**: Provides constructors to create a `ByteBox` with a default capacity or a specified capacity.
- **Ownership Model**: Fully owns the keys and values (`Vec<u8>`), eliminating lifetime management issues.

## Example Usage

Below is a basic example of how to use the `ByteBox` crate in your Rust project.

```rust
let key = b"hello";
let value = b"world";

let mut byte_box = ByteBox::new();
byte_box.insert(key, value);

if let Some(val) = byte_box.get(key) {
    println!("Key: {:?}, Value: {:?}", key, val);
}
```

This will create a `ByteBox`, insert a key-value pair, and retrieve it.

## Custom Hashing

The `ByteBox` crate uses a custom hashing function, `hpulse`, based on the FNV-1a hashing algorithm. This hash function is optimized for hashing byte slices and provides good distribution across buckets.

### How the Hashing Works

The FNV-1a algorithm works by XORing each byte in the key with an offset value and then multiplying the result by a large prime number. This approach provides fast and simple hash computation while minimizing collisions.

## Handling Collisions

When two keys hash to the same index, `ByteBox` uses a linked list (chaining) to store the entries. This ensures that all key-value pairs are retrievable even when collisions occur.

### Example of Handling Collisions

Let's simulate a scenario where two different keys collide:

```rust
let mut byte_box = ByteBox::new();

byte_box.insert(b"key1", b"value1");
byte_box.insert(b"key2", b"value2"); // Assume key1 and key2 collide

byte_box.view_table(); // Display the hash table to see the collision
```

The `view_table` method provides a visual representation of the internal structure of the `ByteBox`, showing how collisions are handled.

## Dynamic Resizing

The `ByteBox` automatically resizes when the load factor exceeds a certain threshold (usually around 0.75). This ensures that the performance remains optimal even as more key-value pairs are inserted.

### Example of Resizing

```rust
let mut byte_box = ByteBox::new();

for i in 0..20 {
    byte_box.insert(format!("key{}", i).as_bytes(), b"value");
}

assert!(byte_box.capacity() > 16); // The capacity increases as more elements are added
```

## Displaying the Hash Table

The `view_table` method provides a way to display the internal structure of the `ByteBox` for debugging purposes.

### Example of Viewing the Hash Table

```rust
let mut byte_box = ByteBox::new();
byte_box.insert(b"key1", b"value1");
byte_box.insert(b"key2", b"value2");

byte_box.view_table();
```

This will print out the current state of the hash table, showing each cell and its associated entries.

## Safety Considerations

The `remove` method uses `unsafe` code to manipulate pointers for efficient removal of entries. Care has been taken to ensure this is safe, but users should be aware of the risks associated with `unsafe` blocks.

## License

This crate is provided under the Apache-2.0 License.
