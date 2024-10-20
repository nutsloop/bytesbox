/// A trait to convert primitive types into byte representations (`Vec<u8>`).
///
/// This trait is implemented for common primitive types such as `u8`, `u16`, `i32`, `f32`, etc.
/// It allows these types to be converted into a byte format for storage in `ByteBox`.
///
/// # Examples
///
/// ```rust
/// use bytesbox::primitives::BytesPrimitives;
///
/// let num: u32 = 42;
/// let bytes = num.to_bytes();
/// assert_eq!(bytes, b"42");
/// ```
pub trait BytesPrimitives {
    /// Converts the primitive into a `Vec<u8>` representing its byte form.
    ///
    /// The default implementation converts the primitive to its string representation and then to bytes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytesbox::primitives::BytesPrimitives;
    ///
    /// let num: u16 = 256;
    /// let bytes = num.to_bytes();
    /// assert_eq!(bytes, b"256");
    /// ```
    fn to_bytes(&self) -> Vec<u8>;
}

// Implement the `BytesPrimitives` trait for various primitive types.

impl BytesPrimitives for u8 {
    /// Converts a `u8` into its byte representation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytesbox::primitives::BytesPrimitives;
    ///
    /// let num: u8 = 255;
    /// let bytes = num.to_bytes();
    /// assert_eq!(bytes, b"255");
    /// ```
    fn to_bytes(&self) -> Vec<u8> {
        format!("{}", &self).into_bytes()
    }
}

impl BytesPrimitives for u16 {
    /// Converts a `u16` into its byte representation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytesbox::primitives::BytesPrimitives;
    ///
    /// let num: u16 = 65535;
    /// let bytes = num.to_bytes();
    /// assert_eq!(bytes, b"65535");
    /// ```
    fn to_bytes(&self) -> Vec<u8> {
        format!("{}", &self).into_bytes()
    }
}

impl BytesPrimitives for u32 {
    /// Converts a `u32` into its byte representation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytesbox::primitives::BytesPrimitives;
    ///
    /// let num: u32 = 4294967295;
    /// let bytes = num.to_bytes();
    /// assert_eq!(bytes, b"4294967295");
    /// ```
    fn to_bytes(&self) -> Vec<u8> {
        format!("{}", &self).into_bytes()
    }
}

impl BytesPrimitives for u64 {
    /// Converts a `u64` into its byte representation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytesbox::primitives::BytesPrimitives;
    ///
    /// let num: u64 = 18446744073709551615;
    /// let bytes = num.to_bytes();
    /// assert_eq!(bytes, b"18446744073709551615");
    /// ```
    fn to_bytes(&self) -> Vec<u8> {
        format!("{}", &self).into_bytes()
    }
}

impl BytesPrimitives for i8 {
    /// Converts an `i8` into its byte representation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytesbox::primitives::BytesPrimitives;
    ///
    /// let num: i8 = -128;
    /// let bytes = num.to_bytes();
    /// assert_eq!(bytes, b"-128");
    /// ```
    fn to_bytes(&self) -> Vec<u8> {
        format!("{}", &self).into_bytes()
    }
}

impl BytesPrimitives for i16 {
    /// Converts an `i16` into its byte representation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytesbox::primitives::BytesPrimitives;
    ///
    /// let num: i16 = -32768;
    /// let bytes = num.to_bytes();
    /// assert_eq!(bytes, b"-32768");
    /// ```
    fn to_bytes(&self) -> Vec<u8> {
        format!("{}", &self).into_bytes()
    }
}

impl BytesPrimitives for i32 {
    /// Converts an `i32` into its byte representation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytesbox::primitives::BytesPrimitives;
    ///
    /// let num: i32 = -2147483648;
    /// let bytes = num.to_bytes();
    /// assert_eq!(bytes, b"-2147483648");
    /// ```
    fn to_bytes(&self) -> Vec<u8> {
        format!("{}", &self).into_bytes()
    }
}

impl BytesPrimitives for i64 {
    /// Converts an `i64` into its byte representation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytesbox::primitives::BytesPrimitives;
    ///
    /// let num: i64 = -9223372036854775808;
    /// let bytes = num.to_bytes();
    /// assert_eq!(bytes, b"-9223372036854775808");
    /// ```
    fn to_bytes(&self) -> Vec<u8> {
        format!("{}", &self).into_bytes()
    }
}

impl BytesPrimitives for f32 {
    /// Converts an `f32` into its byte representation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytesbox::primitives::BytesPrimitives;
    ///
    /// let num: f32 = 3.14;
    /// let bytes = num.to_bytes();
    /// assert_eq!(bytes, b"3.14");
    /// ```
    fn to_bytes(&self) -> Vec<u8> {
        format!("{}", &self).into_bytes()
    }
}

impl BytesPrimitives for f64 {
    /// Converts an `f64` into its byte representation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytesbox::primitives::BytesPrimitives;
    ///
    /// let num: f64 = 2.718281828459045;
    /// let bytes = num.to_bytes();
    /// assert_eq!(bytes, b"2.718281828459045");
    /// ```
    fn to_bytes(&self) -> Vec<u8> {
        format!("{}", &self).into_bytes()
    }
}

impl BytesPrimitives for usize {
    /// Converts a `usize` into its byte representation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytesbox::primitives::BytesPrimitives;
    ///
    /// let num: usize = 1024;
    /// let bytes = num.to_bytes();
    /// assert_eq!(bytes, b"1024");
    /// ```
    fn to_bytes(&self) -> Vec<u8> {
        format!("{}", &self).into_bytes()
    }
}

impl BytesPrimitives for isize {
    /// Converts an `isize` into its byte representation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytesbox::primitives::BytesPrimitives;
    ///
    /// let num: isize = -1024;
    /// let bytes = num.to_bytes();
    /// assert_eq!(bytes, b"-1024");
    /// ```
    fn to_bytes(&self) -> Vec<u8> {
        format!("{}", &self).into_bytes()
    }
}
