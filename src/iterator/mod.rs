use super::*;
/// An iterator over the key-value pairs of a `ByteBox`.
///
/// This struct is created by the [`ByteBox::iter`] method.
pub struct ByteBoxIterator<'a> {
    pub(crate) byte_box: &'a ByteBox,
    pub(crate) index: usize,
    pub(crate) entry: Option<&'a Entry>,
}

impl<'a> Iterator for ByteBoxIterator<'a> {
    type Item = (&'a [u8], &'a [u8]);

    /// Advances the iterator and returns the next key-value pair.
    ///
    /// # Returns
    ///
    /// * `Some((&[u8], &[u8]))` containing references to the key and value.
    /// * `None` if the iterator has reached the end.
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
    /// let mut iter = bytebox.iter();
    /// assert_eq!(iter.next(), Some((&b"key1"[..], &b"value1"[..])));
    /// assert_eq!(iter.next(), Some((&b"key2"[..], &b"value2"[..])));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(entry) = self.entry {
            self.entry = entry.next.as_deref();
            return Some((&entry.key[..], &entry.value[..]));
        }

        while self.index < self.byte_box.cells.len() {
            if let Some(ref entry) = self.byte_box.cells[self.index] {
                self.entry = entry.next.as_deref();
                self.index += 1;
                return Some((&entry.key[..], &entry.value[..]));
            }
            self.index += 1;
        }

        None
    }
}
