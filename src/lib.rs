#![warn(missing_docs)]
//! mictils is util crate that contains various functions.

/// Proveide instant hasher.
///
/// Types implementing `Hash` are able to be instant-hash.
///
/// # Examples
/// Hash is implemented by default:
/// ```rust
/// # fn main() {
/// use mictils::HashCode;
///
/// let str1 = String::from("foo");
/// let str2 = String::from("foo");
///
/// assert_eq!(str1.hashcode(), str2.hashcode());
/// # }
/// ```
pub trait HashCode {
    /// Java-like hasher function.
    ///
    /// Basic uses:
    /// ```rust
    /// # fn main() {
    /// # use mictils::HashCode;
    /// let text = String::from("HashCode");
    ///
    /// assert_eq!(17255704455115175831, text.hashcode());
    /// # }
    /// ```
    ///
    /// Advanced uses:
    /// ```rust
    /// # fn main() {
    /// #   assert!(hash_eq(12, 12));
    /// # }
    /// use mictils::HashCode;
    /// use std::hash::Hash;
    ///
    /// fn hash_eq<H: Hash + HashCode>(a: H, b: H) -> bool {
    ///     a.hashcode() == b.hashcode()
    /// }
    /// ```
    fn hashcode(&self) -> u64;
}

impl<T: std::hash::Hash> HashCode for T {
    fn hashcode(&self) -> u64 {
        use std::hash::Hasher;

        let mut hasher = std::hash::DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashcode_eq_usize() {
        let val1 = 12usize.hashcode();
        let val2 = 12usize.hashcode();

        assert_eq!(val1, val2);
    }

    #[test]
    fn hashcode_eq_isize() {
        let val1 = 12isize.hashcode();
        let val2 = 12isize.hashcode();

        assert_eq!(val1, val2);
    }

    #[test]
    fn hashcode_eq_str() {
        let val1 = String::from("HashCode").hashcode();
        let val2 = String::from("HashCode").hashcode();

        assert_eq!(val1, val2);
    }

    #[test]
    fn hashcode_eq_vec() {
        let val1 = vec![1, 2, 3].hashcode();
        let val2 = vec![1, 2, 3].hashcode();

        assert_eq!(val1, val2);
    }
}
