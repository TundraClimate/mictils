#![warn(missing_docs)]
//! mictils is util crate that contains various functions.

pub mod prelude;

/// Proveide instant hasher.
///
/// Types implementing [Hash](std::hash::Hash) are able to be instant-hash.
///
/// # Examples
/// Hash is implemented by default:
/// ```rust
/// use mictils::HashCode;
///
/// let str1 = String::from("foo");
/// let str2 = String::from("foo");
///
/// assert_eq!(str1.hashcode(), str2.hashcode());
/// ```
pub trait HashCode {
    /// Java-like hasher function.
    ///
    /// Basic uses:
    /// ```rust
    /// # use mictils::HashCode;
    /// let text = String::from("HashCode");
    ///
    /// assert_eq!(17255704455115175831, text.hashcode());
    /// ```
    ///
    /// Advanced uses:
    /// ```rust
    /// use mictils::HashCode;
    /// use std::hash::Hash;
    ///
    /// fn hash_eq<H: Hash + HashCode>(a: H, b: H) -> bool {
    ///     a.hashcode() == b.hashcode()
    /// }
    ///
    /// assert!(hash_eq(12, 12));
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

/// Kotlin-like trait, but name changed.
/// not using `let`, it use `bind`.
///
/// Takes ownership and returns the closure return value.
///
/// It similar the [map](std::iter::Map). but not only iterator.
pub trait Bind {
    /// By "consuming" ownership, you can avoid the risk of unnecessary variables being used.
    ///
    /// ```rust
    /// # use mictils::Bind;
    /// let text = String::from("hi, world!");
    /// let bind = text.bind(|s| s.to_ascii_uppercase()); // text is moved
    ///
    /// assert_eq!("HI, WORLD!", bind);
    ///
    /// // COMPILE ERROR!
    /// // assert_eq!("hi, world!", text);
    /// ```
    ///
    /// Bond ownership to a closure:
    /// ```rust
    /// # use mictils::Bind;
    /// let rwlock = std::sync::RwLock::new(String::from("HhEellOo"));
    ///
    /// /*
    /// {
    ///     let mut w = rwlock.write().unwrap();
    ///     w.retain(|c| !c.is_uppercase());
    ///     w.push_str(", world");
    /// }
    /// */
    ///
    /// rwlock.write().unwrap().bind(|mut w| {
    ///     w.retain(|c| !c.is_uppercase());
    ///     w.push_str(", world");
    /// });
    ///
    /// assert_eq!(String::from("hello, world"), *rwlock.read().unwrap());
    /// ```
    fn bind<R, F: FnOnce(Self) -> R>(self, f: F) -> R
    where
        Self: Sized,
    {
        f(self)
    }
}

impl<T> Bind for T {}

/// Kotlin-like trait, but name changed.
/// not using `also`, it use `hold`.
///
/// Hold ownership and use it refs.
///
/// **Caution**: closure does **not** return a value.
pub trait Hold {
    /// It similar the iterator [inspect](std::iter::Inspect).
    ///
    /// ```rust
    /// # use mictils::Hold;
    /// let value = String::from("UTF-8 encoded").hold_ref(|s| println!("{s}"));
    /// ```
    fn hold_ref<F: FnOnce(&Self)>(self, f: F) -> Self
    where
        Self: Sized,
    {
        f(&self);
        self
    }

    /// This is useful when initializing self.
    ///
    /// ```rust
    /// # use mictils::Hold;
    /// let text = String::from("Hello").hold(|s| s.push_str(", World"));
    ///
    /// assert_eq!(String::from("Hello, World"), text);
    /// ```
    fn hold<F: FnOnce(&mut Self)>(mut self, f: F) -> Self
    where
        Self: Sized,
    {
        f(&mut self);
        self
    }
}

impl<T> Hold for T {}

/// Impl [exists_symlink](crate::SymlinkExists::exists_symlink) for [PathBuf](std::path::PathBuf), [Path](std::path::Path) and etc.
///
/// [exists](std::path::Path::exists) follows the symlink and returns the value,  
/// but [exists_symlink](crate::SymlinkExists::exists_symlink) does not follow it and returns the exists of the file itself.
pub trait SymlinkExists {
    /// Get the exists of the file without following symlinks.
    fn exists_symlink(&self) -> bool;
}

impl<T> SymlinkExists for T
where
    T: std::ops::Deref<Target = std::path::Path>,
{
    fn exists_symlink(&self) -> bool {
        self.symlink_metadata()
            .is_ok_and(|m| m.is_symlink() || self.exists())
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

    #[test]
    fn bind_value() {
        let val = String::from("Hello");

        let bind = val.bind(|val| format!("{val}, World"));
        assert_eq!(String::from("Hello, World"), bind);
    }

    #[test]
    fn bind_vec_value() {
        let val = vec![2, 12, 1002];

        let bind = val.bind(|val| val.into_iter().sum::<u32>());
        assert_eq!(1016, bind);
    }

    #[test]
    fn bind_str() {
        let val = "Hi";

        let bind = val.bind(|s| s);
        assert_eq!("Hi", bind);
    }

    #[test]
    fn hold_ref_value() {
        let mut buf = String::from("Hello, ");
        let hold = String::from("World").hold_ref(|val| buf.push_str(val));

        assert_eq!(String::from("World"), hold);
        assert_eq!(String::from("Hello, World"), buf);
    }

    #[test]
    fn hold_value() {
        let hold = String::from("Hello").hold(|val| val.push_str(", World"));

        assert_eq!(String::from("Hello, World"), hold);
    }

    #[test]
    fn bind_lock_write() {
        let rwlock = std::sync::RwLock::new(String::from("HhEellOo"));

        /*
        {
            let mut w = rwlock.write().unwrap();
            w.retain(|c| !c.is_uppercase());
            w.push_str(", world");
        }
        */

        rwlock.write().unwrap().bind(|mut w| {
            w.retain(|c| !c.is_uppercase());
            w.push_str(", world");
        });

        assert_eq!(String::from("hello, world"), *rwlock.read().unwrap());
    }
}
