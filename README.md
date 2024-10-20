# bytesbox Crate

The `bytesbox` crate provides a custom hash map implementation optimized for byte slices (`Vec<u8>`).
It allows you to map keys of type `Vec<u8>` to values of type `Vec<u8>`, offering an efficient way to work with raw byte data without unnecessary cloning or allocations.
Additionally, it includes methods for inserting primitive types such as integers and floating points.

## Features

- **Collision Resolution via Linked Lists**: Handles hash collisions using linked lists (chaining), ensuring access to all entries even when collisions occur.
- **Dynamic Resizing**: Automatically resizes the underlying storage when the load factor exceeds a predefined threshold, maintaining optimal performance.
- **Customizable Initial Capacity**: Provides constructors to create a `ByteBox` with a default capacity or a specified capacity.
- **Primitive Type Support**: Insert primitive types (e.g., `u8`, `i32`, `f64`) directly into the hash map.
- **Ownership Model**: Fully owns the keys and values (`Vec<u8>`), eliminating lifetime management issues.

## Installation

To use the `bytesbox` crate:

- add it as a dependency in your project's `Cargo.toml`:

  ```toml
  [dependencies]
  bytesbox = "0.2.0"
  ```

- use cargo add:

  ```bash
  cargo add bytesbox
  ```

Once added, you can import and use the crate in your Rust programs.

## Basic Example: Main Program

Here’s a simple example showing how to use the `ByteBox` in your `main.rs` file:

```rust
use bytesbox::ByteBox;

fn main() {
    let key = b"hello";
    let value = b"world";

    let mut byte_box = ByteBox::new();
    byte_box.insert(key, value);

    if let Some(val) = byte_box.get(key) {
        println!(
            "Key: {:?}, Value: {:?}",
            String::from_utf8_lossy(key),
            String::from_utf8_lossy(val)
        );
    }
}
```

and now run the program:

```bash
cargo run
```

it will print out:

```bash
Key: "hello", Value: "world"
```

## Handling Collisions

When two keys hash to the same index, `ByteBox` uses a linked list (chaining) to store the entries. This ensures that all key-value pairs are retrievable even when collisions occur.

### Example of Handling Collisions

Let's simulate a scenario where two different keys collide:

```rust
let mut byte_box = ByteBox::prealloc(2);

byte_box.insert(b"key1", b"value1");
byte_box.insert(b"key2", b"value2");

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

## Primitive Insertion with `insert_primitive`

You can insert primitive types into the `ByteBox` using the `insert_primitive` method. This automatically converts the primitive into a `Vec<u8>` for storage.

### Example of Inserting Primitives

```rust
let mut byte_box = ByteBox::new();
byte_box.insert_primitive(b"age", 30u8);
byte_box.insert_primitive(b"score", 99.5f64);
byte_box.insert_primitive(b"balance", -100i32);
```

In this example, you can see how to insert a `u8`, `f64`, and `i32` directly into the `ByteBox`.

## Iteration with `iter`

You can iterate over all key-value pairs in the ByteBox using the `iter` method. This allows you to traverse the entire collection, accessing each `key` and its corresponding `value` in a seamless and efficient manner.

### Example of Iterating

```rust
let mut byte_box = ByteBox::new();
byte_box.insert(b"key1", b"value1");
byte_box.insert(b"key2", b"value2");

for (key, value) in byte_box.iter() {
    println!("{:?}: {:?}", key, value);
}
```

## Safety Considerations

The `remove` method uses `unsafe` code to manipulate pointers for efficient removal of entries. Care has been taken to ensure this is safe, but users should be aware of the risks associated with `unsafe` blocks.

## License

This crate is provided under the Apache-2.0 License.
