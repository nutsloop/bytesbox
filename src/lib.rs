//! # ByteBox
//!
//! `ByteBox` is a high-performance, memory-efficient hash table implemented in Rust, designed specifically for scenarios where keys and values are naturally represented as byte arrays (`Vec<u8>`). This crate offers a robust and flexible solution for storing and managing byte-based key-value pairs, making it ideal for applications in networking, data serialization, caching, and more.
//!
//! ## Key Features
//!
//! - **Efficient Storage:** Utilizes separate chaining with linked lists to handle hash collisions, ensuring quick insertion and retrieval even under high load.
//! - **Dynamic Resizing:** Automatically resizes the underlying storage when the load factor exceeds a predefined threshold, maintaining optimal performance and preventing excessive collisions.
//! - **Primitive Type Support:** Provides convenient methods to insert primitive types by converting them into their byte representations, simplifying the process of storing numerical and other basic data types.
//! - **Iterative Access:** Implements iterator traits, allowing seamless traversal of all key-value pairs within the `ByteBox`, facilitating operations like searching, filtering, and bulk processing.
//! - **Customizable Hashing:** Leverages Rust’s `DefaultHasher` for hashing keys, ensuring a good distribution of entries across the hash table and minimizing collision rates.
//! - **User-Friendly Display:** Offers a formatted and colored visualization of the hash table’s structure, aiding in debugging and providing insights into the distribution of entries.
//! - **Comprehensive Documentation:** Comes with detailed documentation for all public interfaces, making it easy for developers to integrate and utilize `ByteBox` effectively in their projects.
//!
//! ## Design and Implementation
//!
//! `ByteBox` is built around the concept of storing keys and values as byte vectors, allowing for a wide range of applications where data is naturally in byte form or can be easily converted. The core structure consists of a vector of optional `Entry` boxes, each representing a key-value pair. By using separate chaining, `ByteBox` efficiently manages collisions, ensuring that even with a large number of entries, performance remains consistent.
//!
//! The crate emphasizes simplicity and efficiency, providing a straightforward API for common operations such as insertion, retrieval, and removal of entries. Additionally, the support for primitive types through the `BytesPrimitives` trait simplifies the process of working with numerical data, reducing the overhead of manual byte conversions.
//!
//! ## Use Cases
//!
//! - **Networking:** Ideal for managing protocol headers and payloads where data is inherently byte-oriented.
//! - **Data Serialization:** Facilitates the storage and retrieval of serialized data structures, enabling efficient caching and quick access.
//! - **Caching Systems:** Serves as an effective in-memory cache for applications requiring fast lookup and storage of byte-based data.
//! - **Configuration Management:** Allows for the storage of configuration settings and parameters in a compact byte format, enhancing performance and reducing memory footprint.
//!
//! ## Performance Considerations
//!
//! `ByteBox` is optimized for speed and memory usage, making it suitable for performance-critical applications. Its dynamic resizing mechanism ensures that the hash table maintains a low load factor, minimizing collisions and ensuring that operations remain efficient as the number of entries grows. By leveraging Rust’s ownership and borrowing principles, `ByteBox` ensures safe and concurrent access patterns without sacrificing performance.
//!
//! ## Getting Started
//!
//! Integrating `ByteBox` into your Rust project is straightforward. Simply add it as a dependency in your `Cargo.toml` and start utilizing its powerful API to manage your byte-based key-value pairs with ease and efficiency.
//!
//! ---
//!
//! ## Safety Considerations
//!
//!The `remove` method uses `unsafe` code to manipulate pointers for efficient removal of entries. Care has been taken to ensure this is safe, but users should be aware of the risks associated with `unsafe` blocks.
pub mod iterator;
pub mod primitives;

use iterator::*;
use primitives::*;

#[cfg(feature = "color")]
use bytescolor::ByteColor;

use std::collections::hash_map::DefaultHasher;
use std::fmt::{self, Display};
use std::hash::{Hash, Hasher};

/// Represents a key-value pair within the `ByteBox` hash table.
/// Each `Entry` may point to the next entry in case of hash collisions.
#[derive(Debug, Clone)]
struct Entry {
    key: Vec<u8>,
    value: Vec<u8>,
    next: Option<Box<Entry>>,
}

/// A hash table implementation that stores key-value pairs as byte vectors.
/// Uses separate chaining to handle hash collisions.
///
/// # Examples
///
/// ```rust
/// use bytesbox::ByteBox;
///
/// let mut bytebox = ByteBox::new();
/// bytebox.insert(b"key1", b"value1");
/// bytebox.insert(b"key2", b"value2");
///
/// assert_eq!(bytebox.get(b"key1"), Some(&b"value1"[..]));
/// assert_eq!(bytebox.len(), 2);
/// ```
#[derive(Clone, Debug)]
pub struct ByteBox {
    cells: Vec<Option<Box<Entry>>>,
    alloc: usize,
    len: usize,
    load_factor_threshold: f32,
}

impl Display for ByteBox {
    /// Formats the `ByteBox` for display purposes.
    ///
    /// This implementation displays the contents in a readable key-value format.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytesbox::ByteBox;
    ///
    /// let mut bytebox = ByteBox::new();
    /// bytebox.insert(b"key", b"value");
    /// println!("{}", bytebox);
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;

        let mut first = true;
        for (_, cell) in self.cells.iter().enumerate() {
            let mut current = cell.as_ref();
            while let Some(entry) = current {
                if !first {
                    write!(f, ", ")?;
                }
                write!(
                    f,
                    "{:?}: {:?}",
                    String::from_utf8_lossy(&entry.key),
                    String::from_utf8_lossy(&entry.value)
                )?;
                current = entry.next.as_ref();
            }
            first = false;
        }

        write!(f, "}}")
    }
}

impl ByteBox {
    /// Creates a new `ByteBox` with a default initial capacity of 16 cells.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytesbox::ByteBox;
    ///
    /// let bytebox = ByteBox::new();
    /// assert_eq!(bytebox.len(), 0);
    /// ```
    pub fn new() -> Self {
        Self::prealloc(16)
    }

    /// Creates a new `ByteBox` with a specified initial capacity.
    ///
    /// # Arguments
    ///
    /// * `size` - The initial number of cells to allocate.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytesbox::ByteBox;
    ///
    /// let bytebox = ByteBox::prealloc(32);
    /// assert_eq!(bytebox.allocation(), 32);
    /// ```
    pub fn prealloc(size: usize) -> Self {
        ByteBox {
            cells: vec![None; size],
            alloc: size,
            len: 0,
            load_factor_threshold: 0.75,
        }
    }

    /// Returns the number of key-value pairs stored in the `ByteBox`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytesbox::ByteBox;
    ///
    /// let mut bytebox = ByteBox::new();
    /// assert_eq!(bytebox.len(), 0);
    /// bytebox.insert(b"key", b"value");
    /// assert_eq!(bytebox.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns the current allocation size (number of cells) of the `ByteBox`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytesbox::ByteBox;
    ///
    /// let bytebox = ByteBox::new();
    /// assert_eq!(bytebox.allocation(), 16);
    /// ```
    pub fn allocation(&self) -> usize {
        self.alloc
    }

    /// Inserts a key-value pair into the `ByteBox`.
    ///
    /// If the key already exists, its value is updated.
    /// If the load factor exceeds the threshold after insertion, the table is resized.
    ///
    /// # Arguments
    ///
    /// * `key` - A byte slice representing the key.
    /// * `value` - A byte slice representing the value.
    ///
    /// # Returns
    ///
    /// * `true` if a new key-value pair was inserted.
    /// * `false` if an existing key was updated.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytesbox::ByteBox;
    ///
    /// let mut bytebox = ByteBox::new();
    /// assert!(bytebox.insert(b"key1", b"value1"));
    /// assert!(!bytebox.insert(b"key1", b"value2"));
    /// assert_eq!(bytebox.get(b"key1"), Some(&b"value2"[..]));
    /// ```
    pub fn insert(&mut self, key: &[u8], value: &[u8]) -> bool {
        if (self.len as f32) / (self.alloc as f32) >= self.load_factor_threshold {
            self.resize();
        }

        let idx = Self::hash(key, self.alloc);
        let mut current = &mut self.cells[idx];

        while let Some(entry) = current {
            if entry.key == key {
                entry.value = value.to_vec();
                return false;
            }
            current = &mut entry.next;
        }

        let new_entry = Box::new(Entry {
            key: key.to_vec(),
            value: value.to_vec(),
            next: self.cells[idx].take(),
        });
        self.cells[idx] = Some(new_entry);
        self.len += 1;

        true
    }

    /// Inserts a key and a primitive value into the `ByteBox`.
    ///
    /// The primitive value is converted to its byte representation using the `BytesPrimitives` trait.
    ///
    /// # Type Parameters
    ///
    /// * `T` - A type that implements the `BytesPrimitives` trait.
    ///
    /// # Arguments
    ///
    /// * `key` - A byte slice representing the key.
    /// * `value` - A primitive value that can be converted into bytes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytesbox::ByteBox;
    /// use bytesbox::primitives::BytesPrimitives;
    ///
    /// let mut bytebox = ByteBox::new();
    /// bytebox.insert_primitive(b"number", 42u32);
    /// assert_eq!(bytebox.get(b"number"), Some(&b"42"[..]));
    /// ```
    pub fn insert_primitive<T: BytesPrimitives>(&mut self, key: &[u8], value: T) {
        self.insert(key, &value.to_bytes());
    }

    /// Retrieves the value associated with the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - A byte slice representing the key to look up.
    ///
    /// # Returns
    ///
    /// * `Some(&[u8])` containing the value if the key exists.
    /// * `None` if the key does not exist in the `ByteBox`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytesbox::ByteBox;
    ///
    /// let mut bytebox = ByteBox::new();
    /// bytebox.insert(b"key", b"value");
    /// assert_eq!(bytebox.get(b"key"), Some(&b"value"[..]));
    /// assert_eq!(bytebox.get(b"nonexistent"), None);
    /// ```
    pub fn get(&self, key: &[u8]) -> Option<&[u8]> {
        let idx = Self::hash(key, self.alloc);
        let mut current = self.cells[idx].as_ref();

        while let Some(entry) = current {
            if entry.key == key {
                return Some(&entry.value.as_slice());
            }
            current = entry.next.as_ref();
        }

        None
    }

    /// Removes the key-value pair associated with the given key from the `ByteBox`.
    ///
    /// # Arguments
    ///
    /// * `key` - A byte slice representing the key to remove.
    ///
    /// # Returns
    ///
    /// * `Some(Vec<u8>)` containing the removed value if the key existed.
    /// * `None` if the key was not found.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytesbox::ByteBox;
    ///
    /// let mut bytebox = ByteBox::new();
    /// bytebox.insert(b"key", b"value");
    /// assert_eq!(bytebox.remove(b"key"), Some(b"value".to_vec()));
    /// assert_eq!(bytebox.remove(b"key"), None);
    /// ```
    pub fn remove(&mut self, key: &[u8]) -> Option<Vec<u8>> {
        let idx = Self::hash(key, self.alloc);
        let cell = &mut self.cells[idx];

        let mut prev = cell as *mut Option<Box<Entry>>;
        let mut curr = cell.as_mut();

        while let Some(entry) = curr {
            if entry.key == key {
                let removed_val = entry.value.clone();
                unsafe {
                    *prev = entry.next.take();
                }
                self.len -= 1;
                return Some(removed_val);
            }
            prev = &mut entry.next as *mut Option<Box<Entry>>;
            curr = entry.next.as_mut();
        }

        None
    }

    /// Removes all key-value pairs from the `ByteBox`, resetting it to an empty state.
    ///
    /// This method retains the current capacity of the hash table, allowing it to be reused
    /// without the overhead of reallocating memory. All existing entries are removed, and
    /// the length (`len`) is set to zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytesbox::ByteBox;
    ///
    /// let mut bytebox = ByteBox::new();
    /// bytebox.insert(b"key1", b"value1");
    /// bytebox.insert(b"key2", b"value2");
    /// assert_eq!(bytebox.len(), 2);
    ///
    /// bytebox.clear();
    /// assert_eq!(bytebox.len(), 0);
    /// assert_eq!(bytebox.get(b"key1"), None);
    /// assert_eq!(bytebox.get(b"key2"), None);
    /// ```
    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            *cell = None;
        }
        self.len = 0;
    }

    /// Doubles the current capacity of the `ByteBox` and rehashes all existing entries.
    ///
    /// This method is called internally when the load factor exceeds the threshold.
    fn resize(&mut self) {
        let new_cap = self.alloc * 2;
        let mut new_cells: Vec<Option<Box<Entry>>> = vec![None; new_cap];

        for (_, cell) in self.cells.iter_mut().enumerate() {
            let mut current = cell.take();
            while let Some(mut entry) = current {
                let idx = Self::hash(&entry.key, new_cap);
                current = entry.next.take();
                entry.next = new_cells[idx].take();
                new_cells[idx] = Some(entry);
            }
        }

        self.cells = new_cells;
        self.alloc = new_cap;
    }

    /// Computes the hash index for a given key based on the current capacity.
    ///
    /// # Arguments
    ///
    /// * `key` - A byte slice representing the key to hash.
    /// * `capacity` - The current or new capacity of the hash table.
    ///
    /// # Returns
    ///
    /// * `usize` representing the index in the cells vector.
    fn hash(key: &[u8], capacity: usize) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish();
        let index = (hash as usize) % capacity;
        index
    }

    /// Provides an iterator over the `ByteBox` that allows for iteration using `for` loops.
    ///
    /// This enables the use of `ByteBox` in contexts where an iterator is expected.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytesbox::ByteBox;
    ///
    /// let mut bytebox = ByteBox::new();
    /// bytebox.insert(b"key1", b"value1");
    /// bytebox.insert(b"key2", b"value2");
    ///
    /// for (key, value) in bytebox.iter() {
    ///     println!("{:?}: {:?}", key, value);
    /// }
    /// ```
    pub fn iter(&self) -> ByteBoxIterator {
        ByteBoxIterator {
            byte_box: &self,
            entry: None,
            index: 0,
        }
    }
    /// Provides a detailed, colored visualization of the hash table.
    ///
    /// This function prints the structure of the `ByteBox`, including each cell and its entries.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytesbox::ByteBox;
    ///
    /// let mut bytebox = ByteBox::new();
    /// bytebox.insert(b"key", b"value");
    /// bytebox.view_table();
    /// ```
    #[cfg(feature = "color")]
    pub fn view_table(&self) {
        // Cell Header
        let bytebox_header = format!(
            "{}, number of cell ({}), allocation ({})",
            b"ByteBox".blue().bold().underline(),
            self.len().red(),
            self.allocation().red()
        );
        // Print separator before each cell
        println!(
            "{}",
            "────────────────────────────────────────────────".blue()
        );
        println!("{}", bytebox_header);
        for (index, cell) in self.cells.iter().enumerate() {
            let mut current = cell.as_ref();
            // Cell Header
            let cell_header = format!("  Cell {}:", index).magenta();
            // Print separator before each cell
            println!(
                "{}",
                "────────────────────────────────────────────────".red()
            );
            println!("{}", cell_header);

            while let Some(entry) = current {
                let mut max_key_len = 0;
                let mut max_val_len = 0;

                let k_len = entry.key.len();
                let v_len = entry.value.len();

                if k_len > max_key_len {
                    max_key_len = k_len;
                }
                if v_len > max_val_len {
                    max_val_len = v_len;
                }

                // Determine the longest length
                let get_longest_len = std::cmp::max(max_key_len, max_val_len);
                let k_closing_pipe = get_longest_len - k_len;
                let v_closing_pipe = get_longest_len - v_len;
                // Start of the cell box
                // key val display Start
                println!(
                    "    {}",
                    format!("+---+  +-{}-+", "-".repeat(get_longest_len))
                );
                // Key and value with arrows
                println!(
                    "    {}",
                    format!(
                        "| {} |->| {}{} |",
                        "k".red(),
                        format!("{}", String::from_utf8_lossy(&entry.key)).green(),
                        " ".repeat(k_closing_pipe)
                    )
                );
                println!(
                    "    {}",
                    format!("+---+  +-{}-+", "-".repeat(get_longest_len))
                );
                println!(
                    "    {}",
                    format!(
                        "| {} |->| {}{} |",
                        "v".red(),
                        format!("{}", String::from_utf8_lossy(&entry.value)).yellow(),
                        " ".repeat(v_closing_pipe)
                    )
                );
                println!(
                    "    {}",
                    format!("+---+  +-{}-+", "-".repeat(get_longest_len))
                );
                // key val display END

                // represantation on the Entry START
                println!("    | byte_box | contains:");
                let box_container = format!(
                    "    {}{}+",
                    "|           +-------------------------------",
                    "-".repeat(get_longest_len)
                );
                println!("{}", box_container);
                let box_container_len = box_container.len() - 36;
                println!(
                    "    {}{}|",
                    "|           | Entry:                        ",
                    " ".repeat(get_longest_len)
                );
                println!(
                    "    {}{}|",
                    format!(
                        "|           | - key: Vec<u8> ({})",
                        format!("{}", String::from_utf8_lossy(&entry.key)).green()
                    ),
                    " ".repeat(box_container_len - k_len)
                );
                println!(
                    "    {}{}|",
                    format!(
                        "|           | - val: Vec<u8> ({})",
                        format!("{}", String::from_utf8_lossy(&entry.value)).yellow()
                    ),
                    " ".repeat(box_container_len - v_len)
                );
                println!(
                    "    |           | - next: None                  {}|",
                    " ".repeat(get_longest_len)
                );
                println!(
                    "    {}{}+",
                    "|           +-------------------------------",
                    "-".repeat(get_longest_len)
                );
                println!("    {}{}+", "+-------", "-".repeat(box_container_len + 24));
                current = entry.next.as_ref();
            }
            // Indicate that the cell is empty in red
            println!("    {}", b"Empty".red());

            // representation of the Entry END
        }

        // Separator line
        println!(
            "{}",
            "────────────────────────────────────────────────".red()
        );
        println!(
            "{}",
            "────────────────────────────────────────────────".blue()
        );
    }
    #[cfg(not(feature = "color"))]
    pub fn view_table(&self) {
        // Cell Header
        let bytebox_header = format!(
            "{}, number of cell ({}), allocation ({})",
            "ByteBox",
            self.len(),
            self.allocation()
        );
        // Print separator before each cell
        println!("{}", "────────────────────────────────────────────────");
        println!("{}", bytebox_header);
        for (index, cell) in self.cells.iter().enumerate() {
            let mut current = cell.as_ref();
            // Cell Header
            let cell_header = format!("  Cell {}:", index);
            // Print separator before each cell
            println!("{}", "────────────────────────────────────────────────");
            println!("{}", cell_header);

            while let Some(entry) = current {
                let mut max_key_len = 0;
                let mut max_val_len = 0;

                let k_len = entry.key.len();
                let v_len = entry.value.len();

                if k_len > max_key_len {
                    max_key_len = k_len;
                }
                if v_len > max_val_len {
                    max_val_len = v_len;
                }

                // Determine the longest length
                let get_longest_len = std::cmp::max(max_key_len, max_val_len);
                let k_closing_pipe = get_longest_len - k_len;
                let v_closing_pipe = get_longest_len - v_len;
                // Start of the cell box
                // key val display Start
                println!(
                    "    {}",
                    format!("+---+  +-{}-+", "-".repeat(get_longest_len))
                );
                // Key and value with arrows
                println!(
                    "    {}",
                    format!(
                        "| {} |->| {}{} |",
                        "k",
                        format!("{}", String::from_utf8_lossy(&entry.key)),
                        " ".repeat(k_closing_pipe)
                    )
                );
                println!(
                    "    {}",
                    format!("+---+  +-{}-+", "-".repeat(get_longest_len))
                );
                println!(
                    "    {}",
                    format!(
                        "| {} |->| {}{} |",
                        "v",
                        format!("{}", String::from_utf8_lossy(&entry.value)),
                        " ".repeat(v_closing_pipe)
                    )
                );
                println!(
                    "    {}",
                    format!("+---+  +-{}-+", "-".repeat(get_longest_len))
                );
                // key val display END

                // represantation on the Entry START
                println!("    | byte_box | contains:");
                let box_container = format!(
                    "    {}{}+",
                    "|           +-------------------------------",
                    "-".repeat(get_longest_len)
                );
                println!("{}", box_container);
                let box_container_len = box_container.len() - 36;
                println!(
                    "    {}{}|",
                    "|           | Entry:                        ",
                    " ".repeat(get_longest_len)
                );
                println!(
                    "    {}{}|",
                    format!(
                        "|           | - key: Vec<u8> ({})",
                        format!("{}", String::from_utf8_lossy(&entry.key))
                    ),
                    " ".repeat(box_container_len - k_len)
                );
                println!(
                    "    {}{}|",
                    format!(
                        "|           | - val: Vec<u8> ({})",
                        format!("{}", String::from_utf8_lossy(&entry.value))
                    ),
                    " ".repeat(box_container_len - v_len)
                );
                println!(
                    "    |           | - next: None                  {}|",
                    " ".repeat(get_longest_len)
                );
                println!(
                    "    {}{}+",
                    "|           +-------------------------------",
                    "-".repeat(get_longest_len)
                );
                println!("    {}{}+", "+-------", "-".repeat(box_container_len + 24));
                current = entry.next.as_ref();
            }
            // Indicate that the cell is empty in red
            println!("    {}", "Empty");

            // representation of the Entry END
        }

        // Separator line
        println!("{}", "────────────────────────────────────────────────");
        println!("{}", "────────────────────────────────────────────────");
    }
}
