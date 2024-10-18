//! # ByteBox Crate
//!
//! The `ByteBox` crate provides a custom hash map implementation optimized for byte slices.
//! It allows you to map keys of type `Vec<u8>` to values of type `Vec<u8>`, offering an efficient
//! way to work with raw byte data without unnecessary cloning or allocations.
//!
//! ## Features
//!
//! - **Custom Hash Function (`hpulse`)**: Utilizes a bespoke hash function designed for efficient
//!   hashing of byte slices.
//! - **Collision Resolution via Linked Lists**: Handles hash collisions using linked lists
//!   (chaining), ensuring that all entries are accessible even when collisions occur.
//! - **Dynamic Resizing**: Automatically resizes the underlying storage when the load factor
//!   exceeds a predefined threshold, maintaining optimal performance.
//! - **Customizable Initial Allocation**: Provides constructors to create a `ByteBox` with a default
//!   allocation or a specified allocation.
//!
//! ## Important Considerations
//!
//! - **Ownership**: The `ByteBox` now owns the keys and values (`Vec<u8>`), eliminating
//!   lifetime management issues. This means that the data inserted into the `ByteBox` is fully
//!   owned by the structure and will be cleaned up when it is dropped.
//!
//! ## Example
//!
//! ```rust
//! use bytesbox::ByteBox;
//!
//! fn main() {
//!     let key = b"hello";
//!     let value = b"world";
//!
//!     let mut byte_box = ByteBox::new();
//!     byte_box.insert(key, value);
//!
//!     if let Some(val) = byte_box.get(key) {
//!         println!("Key: {:?}, Value: {:?}", key, val);
//!     }
//! }
//! ```
//!
//! ## Safety Notes
//!
//! - The `remove` method uses `unsafe` code to manipulate pointers for efficient removal of entries.
//!   Care has been taken to ensure this is safe, but users should be aware of the risks associated
//!   with `unsafe` blocks.
//!
//! ## License
//!
//! This crate is provided under the Apache-2.0 License.

use bytescolor::*;
use std::fmt::{self, Display};
/// An internal structure representing a single key-value pair within the `ByteBox` hash map.
///
/// Each `Entry` holds a key-value pair, where both the key and value are stored as byte vectors (`Vec<u8>`).
/// The `Entry` struct also supports collision resolution by chaining through a linked list,
/// with the `next` field pointing to the subsequent `Entry` in the same bucket (if any).
///
/// This struct is not intended to be accessed directly by users of the `ByteBox`,
/// as it serves the internal mechanics of the hash map.
///
/// # Fields
///
/// - `key`: The key of the entry, stored as a byte vector (`Vec<u8>`).
///   This allows flexibility for a variety of data types to be hashed.
/// - `val`: The value associated with the key, also stored as a byte vector (`Vec<u8>`).
/// - `next`: An optional pointer to the next `Entry` in the linked list, used for collision resolution.
///   If there are no further collisions in the bucket, this will be `None`.
///
/// # Collision Handling
///
/// In cases where multiple keys hash to the same index (bucket), the entries are
/// chained together using the `next` field, forming a linked list.
#[derive(Debug)]
struct Entry {
    key: Vec<u8>,             // The key of the entry, a vector of bytes.
    val: Vec<u8>,             // The value associated with the key, a vector of bytes.
    next: Option<Box<Entry>>, // An optional boxed `Entry` pointing to the next entry in the linked list for this cell.
}

/// A hash map implementation optimized for byte slices as keys and values.
///
/// The `ByteBox` structure is designed to efficiently store and retrieve key-value pairs where both
/// keys and values are byte slices (`&[u8]`). It uses a custom hash function for hashing byte slices
/// and handles collisions via linked lists.
///
/// ## Fields
///
/// - `cells`: A vector of optional boxed `Entry` objects that represent the individual "buckets" of the hash table.
///   Each cell can contain a linked list of entries in case of hash collisions.
/// - `alloc`: The current allocation size (number of cells) in the hash table, representing the underlying storage allocation.
/// - `len`: The number of key-value pairs currently stored in the `ByteBox`.
///
/// ## Methods
///
/// - `new()`: Creates a new `ByteBox` with a default allocation.
/// - `prealloc(size)`: Pre-allocates space for the specified number of cells in the `ByteBox`, optimizing performance
///   for scenarios where the expected number of key-value pairs is known in advance.
#[derive(Debug)]
pub struct ByteBox {
    cells: Vec<Option<Box<Entry>>>, // A vector of optional boxed `Entry` objects representing the hash table's cells.
    alloc: usize,                   // The current allocation (number of cells) of the hash table.
    len: usize,                     // The number of key-value pairs stored in the `ByteBox`.
}

/// Implements the `Display` trait for the `ByteBox` struct, enabling a human-readable string representation.
///
/// This implementation formats the contents of a `ByteBox` instance, allowing the key-value pairs to be displayed
/// in a manner similar to a `HashMap` or JSON object. The `fmt` function is useful for printing and debugging
/// purposes, providing a clear view of the data stored in the `ByteBox`.
///
/// # Formatting Behavior
///
/// The output format follows a JSON-like structure:
///
/// ```plaintext
/// {
///     "key1": "value1",
///     "key2": "value2",
///     ...
/// }
/// ```
///
/// - Each key-value pair is printed on the same line, with keys and values separated by a colon (`:`).
/// - Pairs are separated by commas (`,`), and the entire collection is enclosed in curly braces (`{}`).
/// - Keys and values are displayed as byte slices converted to strings, using `String::from_utf8_lossy` to handle potential non-UTF8 byte sequences gracefully.
///
/// # Example Usage
///
/// Here’s an example demonstrating how to use this display functionality:
///
/// ```rust
/// use bytesbox::ByteBox;
///
/// let mut byte_box = ByteBox::new();
/// byte_box.insert(b"key1", b"value1");
/// byte_box.insert(b"key2", b"value2");
///
/// println!("{}", byte_box);
/// ```
///
/// The above example will print:
///
/// ```plaintext
/// {"key1": "value1", "key2": "value2"}
/// ```
///
/// This makes it easy to visualize the contents of a `ByteBox` when printing or logging.
impl Display for ByteBox {
    /// Formats the `ByteBox` for display purposes.
    ///
    /// The output format will look like:
    ///
    /// ```plaintext
    /// {
    ///     "key1": "value1",
    ///     "key2": "value2",
    ///     ...
    /// }
    /// ```
    ///
    /// # Details
    ///
    /// - The keys and values are displayed using UTF-8 conversion with `String::from_utf8_lossy`, ensuring that
    ///   any invalid UTF-8 sequences are safely handled.
    /// - Entries are separated by commas (`,`), and the entire map is enclosed in curly braces (`{}`).
    /// - The first entry does not have a preceding comma to match typical map formatting.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?; // Start of the map

        let mut first = true;
        for (key, value) in self.iter() {
            if !first {
                write!(f, ", ")?; // Add comma separator between entries
            }
            write!(
                f,
                "{:?}: {:?}",
                String::from_utf8_lossy(key), // Convert key to readable string
                String::from_utf8_lossy(value)  // Convert value to readable string
            )?;
            first = false;
        }

        write!(f, "}}") // End of the map
    }
}

/// Implementation of the `ByteBox` structure, providing methods to interact with the custom hash map.
///
/// The `ByteBox` is optimized for storing and retrieving key-value pairs where both keys and values
/// are byte slices (`&[u8]`). It supports dynamic resizing, custom hash functions, and collision handling
/// using linked lists. This implementation allows users to create, manipulate, and inspect the contents
/// of the `ByteBox`.
///
/// # Key Features
///
/// - **Custom Hash Function (`hpulse`)**: A fast and efficient hashing algorithm for byte slices, ensuring good distribution.
/// - **Dynamic Resizing**: The hash table resizes itself automatically when the load factor exceeds a certain threshold.
/// - **Collision Handling**: Uses chaining via linked lists to manage hash collisions, ensuring all entries are accessible.
/// - **Flexible Allocation Management**: Allows users to create a `ByteBox` with a default or custom allocation and view the current allocation.
/// - **Iterator Support**: Provides iteration methods (`iter`, `iter_mut`) to traverse the key-value pairs stored in the `ByteBox`.
///
/// # Usage
///
/// The `ByteBox` structure is useful in scenarios where the keys and values are raw byte data and performance is critical.
/// It avoids unnecessary allocations and copies, making it ideal for systems programming, network protocols, and other
/// low-level operations.
///
/// The provided methods allow users to perform common operations such as insertion, lookup, deletion, and resizing.
///
/// ## Example
///
/// ```rust
/// use bytesbox::ByteBox;
///
/// let mut byte_box = ByteBox::new();
/// byte_box.insert(b"key1", b"value1");
/// byte_box.insert(b"key2", b"value2");
///
/// if let Some(value) = byte_box.get(b"key1") {
///     println!("Found value: {:?}", value);
/// }
///
/// byte_box.remove(b"key2");
/// ```
///
/// This example demonstrates how to create a `ByteBox`, insert key-value pairs, retrieve a value, and remove an entry.
impl ByteBox {
    /// Creates a new `ByteBox` with a default allocation size.
    ///
    /// This method initializes a `ByteBox` with a default allocation of 16 cells.
    /// It is useful for scenarios where the expected number of key-value pairs is not known
    /// in advance, providing a balanced initial allocation suitable for general use cases.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytesbox::ByteBox;
    ///
    /// let byte_box = ByteBox::new();
    /// ```
    ///
    /// This will create a new, empty `ByteBox` with 16 pre-allocated cells.
    pub fn new() -> Self {
        Self::prealloc(16)
    }

    /// Creates a new `ByteBox` with the specified allocation.
    ///
    /// # Parameters
    ///
    /// - `size`: The initial allocation (number of cells) for the `ByteBox`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytesbox::ByteBox;
    ///
    /// let mut byte_box = ByteBox::prealloc(64);
    /// ```
    pub fn prealloc(size: usize) -> Self {
        ByteBox {
            cells: (0..size).map(|_| None).collect(),
            alloc: size,
            len: 0,
        }
    }

    /// Returns the number of key-value pairs currently stored in the `ByteBox`.
    ///
    /// This method provides an efficient way to determine how many unique key-value pairs
    /// are stored in the hash map. The value returned by `len()` increases as more pairs
    /// are inserted and decreases when pairs are removed.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use bytesbox::ByteBox;
    ///
    /// let mut byte_box = ByteBox::new();
    /// byte_box.insert(b"key1", b"value1");
    /// byte_box.insert(b"key2", b"value2");
    ///
    /// // After inserting two key-value pairs, len should be 2.
    /// assert_eq!(byte_box.len(), 2);
    ///
    /// byte_box.remove(b"key1");
    ///
    /// // After removing one pair, len should be 1.
    /// assert_eq!(byte_box.len(), 1);
    /// ```
    ///
    /// # Performance
    ///
    /// This method runs in constant time, as the `ByteBox` maintains an internal counter
    /// (`len`) that tracks the number of stored pairs.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns the current allocation of the `ByteBox`, i.e., the total number of available cells.
    ///
    /// The allocation indicates how many key-value pairs the `ByteBox` can store before a resize occurs.
    /// The allocation is not the same as the number of key-value pairs currently stored (`len`), but
    /// rather the number of potential storage slots. When the number of key-value pairs exceeds a
    /// certain threshold (based on the load factor), the `ByteBox` automatically increases its allocation
    /// by resizing.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use bytesbox::ByteBox;
    ///
    /// // Create a ByteBox with an initial allocation of 64.
    /// let byte_box = ByteBox::prealloc(64);
    ///
    /// // Check that the allocation matches the preallocated size.
    /// assert_eq!(byte_box.allocation(), 64);
    /// ```
    ///
    /// # Notes
    ///
    /// The allocation changes dynamically as the `ByteBox` resizes to accommodate more key-value pairs.
    /// It is useful for understanding when resizing might occur and to avoid potential performance costs
    /// associated with frequent resizing.
    pub fn allocation(&self) -> usize {
        self.alloc
    }

    /// Inserts a key-value pair into the `ByteBox`.
    ///
    /// If the key already exists in the `ByteBox`, its value is updated with the new value, and
    /// the method returns `false`. If the key is new, the key-value pair is added to the `ByteBox`,
    /// and the method returns `true`.
    ///
    /// The method automatically resizes the internal storage if the load factor exceeds 0.75.
    ///
    /// # Parameters
    ///
    /// - `key`: A byte slice (`&[u8]`) representing the key.
    /// - `val`: A byte slice (`&[u8]`) representing the value.
    ///
    /// # Returns
    ///
    /// - `true` if a new key-value pair was added.
    /// - `false` if the key already existed, and its value was updated.
    ///
    /// # Panics
    ///
    /// This method does not panic.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytesbox::ByteBox;
    ///
    /// let key = b"foo";
    /// let val = b"bar";
    ///
    /// let mut byte_box = ByteBox::new();
    /// assert!(byte_box.insert(key, val));  // A new key-value pair is added, returns true.
    ///
    /// let updated_val = b"baz";
    /// assert!(!byte_box.insert(key, updated_val));  // The key already exists, value is updated, returns false.
    /// ```
    pub fn insert(&mut self, key: &[u8], val: &[u8]) -> bool {
        let key = key.to_vec();
        let val = val.to_vec();
        if self.load_factor() > 0.75 {
            self.resize();
        }

        let idx = Self::hpulse(&key, self.alloc);
        let cell = &mut self.cells[idx];

        let mut curr = cell.as_mut();
        while let Some(entry) = curr {
            if entry.key == key {
                entry.val = val.to_vec();
                return false;
            }
            curr = entry.next.as_mut();
        }

        let new_entry = Entry {
            key,
            val,
            next: cell.take(),
        };
        *cell = Some(Box::new(new_entry));
        self.len += 1;

        true
    }

    /// Retrieves a value associated with the given key from the `ByteBox`.
    ///
    /// This method searches for the specified key in the `ByteBox` and returns the value associated
    /// with it, if the key is found. If the key is not present, it returns `None`. The search is
    /// performed in constant average time, though collisions may affect performance.
    ///
    /// # Parameters
    ///
    /// - `key`: A reference to a byte slice (`&[u8]`) representing the key to search for.
    ///
    /// # Returns
    ///
    /// An `Option<&[u8]>`:
    /// - `Some(&[u8])` if the key is found, containing a reference to the value.
    /// - `None` if the key is not present in the `ByteBox`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```rust
    /// use bytesbox::ByteBox;
    ///
    /// let key = b"hello";
    /// let val = b"world";
    ///
    /// let mut byte_box = ByteBox::new();
    /// byte_box.insert(key, val);
    ///
    /// // Retrieve the value associated with "hello".
    /// if let Some(value) = byte_box.get(key) {
    ///     println!("Found value: {:?}", value);
    /// }
    /// let key = b"missing";
    /// assert_eq!(byte_box.get(key), None);
    /// ```
    ///
    /// # Performance
    ///
    /// The performance of the `get` method is O(1) on average, assuming a uniform distribution of keys.
    /// However, in cases of hash collisions, multiple keys may map to the same cell, resulting in O(n)
    /// performance for that particular cell.
    pub fn get(&self, key: &[u8]) -> Option<&[u8]> {
        let idx = Self::hpulse(key, self.alloc);
        let mut curr = self.cells[idx].as_ref();

        while let Some(entry) = curr {
            if entry.key == key {
                return Some(&entry.val);
            }
            curr = entry.next.as_ref();
        }

        None
    }

    /// Removes a key-value pair from the `ByteBox` using the provided key.
    ///
    /// This method removes the key-value pair from the `ByteBox` that matches the given key.
    /// If the key exists, the associated value is returned. If the key does not exist, `None` is returned.
    ///
    /// # Parameters
    ///
    /// - `key`: A reference to the byte slice (`&[u8]`) representing the key to remove.
    ///
    /// # Returns
    ///
    /// Returns `Some(Vec<u8>)` containing the removed value if the key was found, otherwise returns `None`.
    ///
    /// # Safety
    ///
    /// This method uses `unsafe` code to manipulate raw pointers for efficient removal of entries from the linked list.
    /// The `unsafe` block is required to update the `next` pointer of the previous entry without violating Rust's borrowing rules.
    /// Care has been taken to ensure that this is safe in this context, but improper use of unsafe code in other contexts could lead to undefined behavior.
    ///
    /// # Panics
    ///
    /// This method does not panic.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytesbox::ByteBox;
    ///
    /// let key = b"temp";
    /// let val = b"data";
    ///
    /// let mut byte_box = ByteBox::new();
    /// byte_box.insert(key, val);
    ///
    /// let removed = byte_box.remove(key);
    /// assert_eq!(removed, Some(val.to_vec())); // The key-value pair was removed successfully.
    /// assert_eq!(byte_box.get(key), None); // The key no longer exists in the ByteBox.
    /// ```
    pub fn remove(&mut self, key: &[u8]) -> Option<Vec<u8>> {
        let idx = Self::hpulse(key, self.alloc);
        let cell = &mut self.cells[idx];

        let mut prev = cell as *mut Option<Box<Entry>>;
        let mut curr = cell.as_mut();

        while let Some(entry) = curr {
            if entry.key == key {
                let removed_val = entry.val.clone();
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

    /// Computes and returns the load factor of the `ByteBox`.
    ///
    /// The load factor is a metric that indicates how full the hash map is.
    /// It is defined as the ratio of the number of entries (`len`) to the number of available cells (`alloc`).
    ///
    /// When the load factor exceeds a predefined threshold (e.g., 0.75), the `ByteBox` will automatically resize
    /// to ensure performance does not degrade due to excessive collisions.
    ///
    /// # Returns
    ///
    /// A `f64` representing the current load factor.
    ///
    fn load_factor(&self) -> f64 {
        self.len as f64 / self.alloc as f64
    }

    /// Resizes the internal storage of the `ByteBox` when the load factor exceeds 0.75.
    ///
    /// This method doubles the allocation of the hash map, rehashes all entries,
    /// and assigns them to new cells based on their recalculated hash.
    ///
    /// # Performance
    ///
    /// The resizing process is O(n), where `n` is the number of key-value pairs in the map.
    /// However, it ensures that subsequent insertions and lookups remain efficient.
    ///
    /// # Example
    ///
    /// ```rust
    /// use bytesbox::ByteBox;
    ///
    /// let mut byte_box = ByteBox::new();
    ///
    /// for i in 0..20 {
    ///     byte_box.insert(format!("key{}", i).as_bytes(), b"value");
    /// }
    ///
    /// assert!(byte_box.allocation() > 16); // After many inserts, the allocation should have increased.
    /// ```
    fn resize(&mut self) {
        let new_allocation = self.alloc * 2;
        let mut new_cells: Vec<Option<Box<Entry>>> = (0..new_allocation).map(|_| None).collect();

        for cell in self.cells.iter_mut() {
            while let Some(mut entry) = cell.take() {
                let idx = Self::hpulse(&entry.key, new_allocation);
                let new_cell = &mut new_cells[idx];

                entry.next = new_cell.take(); // Move the current entry to the new position
                *new_cell = Some(entry); // Place the entry in the new cell
            }
        }

        self.cells = new_cells; // Replace the old cells with the new, larger one
        self.alloc = new_allocation; // Update the allocation size
    }

    /// A custom hash function (`hpulse`) optimized for hashing byte slices (`&[u8]`).
    ///
    /// This hash function is based on the **FNV-1a** (Fowler-Noll-Vo) hashing algorithm,
    /// which is designed for fast, simple, and low-collision hash generation. `hpulse` takes
    /// a byte slice as input, computes a 64-bit hash value, and reduces it to a size suitable
    /// for the allocation of the hash table.
    ///
    /// The **FNV-1a** algorithm works by XORing each byte with an offset value (`OFFSET`),
    /// and then multiplying the result by a large prime number (`PRIME`). This provides good
    /// hash distribution and avoids clustering of similar byte sequences. The function then
    /// uses modulo arithmetic to fit the resulting hash into the current allocation of the `ByteBox`.
    ///
    /// # Parameters
    ///
    /// - `key`: A reference to the byte slice (`&[u8]`) that needs to be hashed.
    /// - `alloc`: The current allocation of the hash table, which is used to calculate the final index.
    ///
    /// # Returns
    ///
    /// A `usize` representing the index into the hash table, calculated by taking the hash
    /// value modulo the table's allocation.
    ///
    /// # Hashing Algorithm (FNV-1a)
    ///
    /// 1. **FNV Offset Basis**: The hash is initialized to a large prime number (`OFFSET`),
    ///    which provides a good starting value.
    /// 2. **Byte-by-byte XOR and Multiply**: For each byte in the input, the current hash value
    ///    is XORed with the byte, and the result is multiplied by a prime (`PRIME`) to generate
    ///    the new hash value.
    /// 3. **Modulo Operation**: After processing all the bytes, the final hash value is reduced
    ///    modulo the table's allocation (`alloc`) to ensure it fits within the available number of
    ///    cells in the hash table.
    ///
    /// # Performance
    ///
    /// FNV-1a is a non-cryptographic hash function known for its simplicity and speed, making it
    /// suitable for hash tables. While not as collision-resistant as cryptographic hash functions
    /// like SHA-256, FNV-1a provides good distribution for typical use cases where data does not
    /// exhibit extreme clustering or adversarial input.
    ///
    ///
    /// # Example of Hash Function:
    ///
    /// The FNV-1a hashing process can be visualized as follows:
    ///
    /// ```plaintext
    /// hash = OFFSET
    /// for each byte in key:
    ///     hash = hash XOR byte
    ///     hash = hash * PRIME
    /// ```
    ///
    /// After processing the entire `key`, the result is taken modulo the current `alloc` to
    /// obtain the final index for insertion or retrieval in the hash table.
    ///
    fn hpulse(key: &[u8], alloc: usize) -> usize {
        const OFFSET: u64 = 14695981039346656037; // FNV-1a offset basis
        const PRIME: u64 = 1099511628211; // FNV-1a prime

        let mut hash = OFFSET; // Initialize the hash to the offset basis
        for &byte in key {
            hash ^= byte as u64; // XOR the byte with the current hash
            hash = hash.wrapping_mul(PRIME); // Multiply by the FNV prime, using wrapping to avoid overflow
        }
        (hash % alloc as u64) as usize // Reduce the hash value modulo the allocation
    }

    /// Returns an iterator over the entries in the `ByteBox`. Each item yielded
    /// by the iterator is a tuple containing a reference to a key and a reference
    /// to a value, both as byte slices (`&[u8]`).
    ///
    /// The iterator traverses through all the buckets (cells) in the `ByteBox`,
    /// yielding each key-value pair in the order they are stored. If collisions occurred
    /// (multiple keys hashed to the same bucket), the iterator will traverse the linked
    /// list of entries in that bucket.
    ///
    /// # Performance
    ///
    /// This method returns an iterator that iterates in O(n) time, where `n` is the total
    /// number of entries stored in the `ByteBox`. The iterator ensures efficient traversal
    /// without copying or reallocating the underlying data.
    ///
    /// # Example
    ///
    /// ```rust
    /// use bytesbox::ByteBox;
    ///
    /// let mut byte_box = ByteBox::new();
    /// byte_box.insert(b"key1", b"value1");
    /// byte_box.insert(b"key2", b"value2");
    ///
    /// for (key, value) in byte_box.iter() {
    ///     println!("{:?} -> {:?}", key, value);
    /// }
    /// ```
    ///
    /// In this example, we create a new `ByteBox`, insert two key-value pairs, and then
    /// iterate over the box to print each pair. The iteration yields each pair in the order
    /// they are stored in the hash map.
    pub fn iter(&self) -> ByteBoxIterator {
        ByteBoxIterator {
            byte_box: self,
            index: 0,
        }
    }

    /// Displays a detailed visual representation of the hash table for debugging purposes.
    ///
    /// This method prints the internal structure of the `ByteBox` in a human-readable format.
    /// It iterates over each cell in the hash table, printing the cell index and the key-value pairs
    /// contained within that cell. If a cell contains multiple entries due to hash collisions, the entries
    /// are printed sequentially in the order they are stored in the linked list.
    ///
    /// This output is useful for debugging purposes to observe how keys and values are stored internally
    /// and to examine the structure of the hash table, including how entries are distributed across buckets.
    ///
    /// # Output Format
    ///
    /// - Each cell is printed with its index number.
    /// - If the cell contains entries, each key-value pair is displayed. If there are multiple entries (due to collisions),
    ///   they are shown in a chain, starting with the first entry and continuing down the linked list.
    /// - Empty cells are explicitly marked as "Empty" in the output.
    ///
    /// The following symbols are used to represent the structure:
    /// - **k**: The key associated with an entry.
    /// - **v**: The value associated with the key.
    /// - `->`: Represents a pointer from one entry to the next in case of collisions.
    /// - Boxes (`+-+`) surround the key and value to indicate the contents of the entry.
    ///
    /// # Example Usage
    ///
    /// ```rust
    /// use bytesbox::ByteBox;
    ///
    /// let mut byte_box = ByteBox::prealloc(4);
    /// byte_box.insert(b"key1", b"value1");
    /// byte_box.insert(b"key2", b"value2");
    /// byte_box.view_table();
    /// ```
    ///
    /// In this example, after inserting two key-value pairs into the `ByteBox`, the `view_table` method
    /// will print the contents of the hash table, showing how the entries are distributed across the available cells.
    ///
    /// # Note
    ///
    /// The output of this method is intended for debugging and visual inspection of the internal state of the hash table.
    /// It may not be suitable for large hash tables or production environments where performance is critical.
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
            // Cell Header
            let cell_header = format!("  Cell {}:", index).magenta();
            // Print separator before each cell
            println!(
                "{}",
                "────────────────────────────────────────────────".red()
            );
            println!("{}", cell_header);

            if let Some(entry) = cell {
                let mut max_key_len = 0;
                let mut max_val_len = 0;

                let k_len = entry.key.len();
                let v_len = entry.val.len();

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
                        format!("{}", String::from_utf8_lossy(&entry.val)).yellow(),
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
                        format!("{}", String::from_utf8_lossy(&entry.val)).yellow()
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
            } else {
                // Indicate that the cell is empty in red
                println!("    {}", b"Empty".red());
            }
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
}

/// An iterator over the key-value pairs in a `ByteBox`.
///
/// The `ByteBoxIterator` is responsible for iterating over the cells (buckets) in the `ByteBox`
/// hash table. It yields key-value pairs, where both the keys and values are byte slices (`&[u8]`).
///
/// The iterator sequentially moves through the cells of the hash table, returning the first entry
/// in each non-empty cell. If a cell contains multiple entries due to hash collisions, only the
/// first entry is returned, and the iterator does not currently traverse linked entries.
///
/// # Fields
///
/// - `byte_box`: A reference to the `ByteBox` being iterated over. This ensures that the iterator
///   has access to the internal cells of the `ByteBox`.
/// - `index`: The current index in the `cells` vector, representing which cell is currently being
///   processed.
///
/// # Usage
///
/// This struct is typically used internally within the `iter()` method of the `ByteBox`.
/// You will rarely need to create it manually, as the `iter()` method handles that.
///
/// # Example
///
/// ```rust
/// use bytesbox::ByteBox;
///
/// let mut byte_box = ByteBox::new();
/// byte_box.insert(b"key1", b"value1");
/// byte_box.insert(b"key2", b"value2");
///
/// let iter = byte_box.iter(); // Creates a `ByteBoxIterator`
/// ```
pub struct ByteBoxIterator<'a> {
    byte_box: &'a ByteBox, // Reference to the ByteBox being iterated
    index: usize,          // The current index in the `cells` vector
}

/// Implements the `Iterator` trait for `ByteBoxIterator`.
///
/// This implementation allows `ByteBoxIterator` to iterate over the key-value pairs in a `ByteBox`.
/// Each iteration returns a tuple `(&'a [u8], &'a [u8])`, where the first element is a reference to
/// the key and the second is a reference to the value.
///
/// The iterator advances through the internal `cells` of the `ByteBox`, yielding key-value pairs
/// for each non-empty cell. If multiple entries are stored in the same cell due to hash collisions,
/// only the first entry is returned, and the rest are not currently traversed.
///
/// # Return Type
///
/// - `Some((&[u8], &[u8]))`: If there is a key-value pair in the current cell, a tuple containing
///   references to the key and value is returned.
/// - `None`: If there are no more entries to iterate over, `None` is returned, ending the iteration.
///
/// # Example
///
/// ```rust
/// use bytesbox::ByteBox;
///
/// let mut byte_box = ByteBox::new();
/// byte_box.insert(b"key1", b"value1");
/// byte_box.insert(b"key2", b"value2");
///
/// let mut iter = byte_box.iter();  // Creates an iterator
///
/// while let Some((key, value)) = iter.next() {
///     println!("Key: {:?}, Value: {:?}", key, value);
/// }
/// ```
///
/// In this example, the iterator traverses the `ByteBox`, yielding each key-value pair in turn.
impl<'a> Iterator for ByteBoxIterator<'a> {
    type Item = (&'a [u8], &'a [u8]);

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.byte_box.cells.len() {
            if let Some(ref entry) = self.byte_box.cells[self.index] {
                self.index += 1;
                return Some((&entry.key[..], &entry.val[..]));
            }
            self.index += 1;
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hpulse() {
        let key = b"example_key";
        let cap = 64;
        let hash_index = ByteBox::hpulse(key, cap);
        println!("Hash index for 'example_key': {}", hash_index);
    }
}
