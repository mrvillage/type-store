//! # type-store
//!
//! A generic type map for storing arbitrary data by type.

use std::{
    any::{Any, TypeId},
    collections::HashMap,
    hash::{BuildHasherDefault, Hasher},
};

/// Optimized hasher for `TypeId`
/// See https://github.com/chris-morgan/anymap/blob/2e9a570491664eea18ad61d98aa1c557d5e23e67/src/lib.rs#L599
/// and https://github.com/actix/actix-web/blob/97399e8c8ce584d005577604c10bd391e5da7268/actix-http/src/extensions.rs#L8
#[derive(Debug, Default)]
struct TypeIdHasher(u64);

impl Hasher for TypeIdHasher {
    fn write(&mut self, bytes: &[u8]) {
        unimplemented!("This TypeIdHasher can only handle u64s, not {:?}", bytes);
    }

    fn write_u64(&mut self, i: u64) {
        self.0 = i;
    }

    fn finish(&self) -> u64 {
        self.0
    }
}

/// A generic type map for storing arbitrary data by type.
///
/// # Example
/// ```rs
/// use extractors::TypeStore;
///
/// let mut store = TypeStore::new();
/// store.insert(1u32);
/// store.insert("hello");
/// assert_eq!(store.get::<u32>(), Some(&1u32));
/// assert_eq!(store.get::<&str>(), Some(&"hello"));
/// assert_eq!(store.get::<u64>(), None);
/// ```
#[derive(Debug, Default)]
pub struct TypeStore {
    map: HashMap<TypeId, Box<dyn Any>, BuildHasherDefault<TypeIdHasher>>,
}

impl TypeStore {
    /// Creates an empty `Store`.
    ///
    /// # Example
    /// ```rs
    /// use extractors::TypeStore;
    ///
    /// let store = TypeStore::new();
    /// assert!(store.is_empty());
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self {
            map: HashMap::default(),
        }
    }

    /// Insert an item into the map.
    ///
    /// If an item of this type was already stored, it will be replaced.
    ///
    /// # Example
    /// ```rs
    /// use extractors::TypeStore;
    ///
    /// let mut store = TypeStore::new();
    /// store.insert(1u32);
    /// assert_eq!(store.get::<u32>(), Some(&1u32));
    /// store.insert(2u32);
    /// assert_eq!(store.get::<u32>(), Some(&2u32));
    /// ```
    #[inline]
    pub fn insert<T: 'static>(&mut self, val: T) {
        self.map.insert(TypeId::of::<T>(), Box::new(val));
    }

    /// Get a reference to an item in the map.
    /// Returns `None` if the item is not present.
    ///
    /// # Example
    /// ```rs
    /// use extractors::TypeStore;
    ///
    /// let mut store = TypeStore::new();
    /// store.insert(1u32);
    /// assert_eq!(store.get::<u32>(), Some(&1u32));
    /// assert_eq!(store.get::<u64>(), None);
    /// ```
    #[inline]
    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.map
            .get(&TypeId::of::<T>())
            .and_then(|v| v.downcast_ref::<T>())
    }

    /// Get a mutable reference to an item in the map.
    /// Returns `None` if the item is not present.
    ///
    /// # Example
    /// ```rs
    /// use extractors::TypeStore;
    ///
    /// let mut store = TypeStore::new();
    /// store.insert(1u32);
    /// let val = store.get_mut::<u32>().unwrap();
    /// *val = 2;
    /// assert_eq!(store.get::<u32>(), Some(&2u32));
    /// ```
    #[inline]
    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.map
            .get_mut(&TypeId::of::<T>())
            .and_then(|v| v.downcast_mut::<T>())
    }

    /// Remove an item from the map.
    /// Returns `None` if the item is not present, `Some(T)` if it was.
    ///
    /// # Example
    /// ```rs
    /// use extractors::TypeStore;
    ///
    /// let mut store = TypeStore::new();
    /// store.insert(1u32);
    /// assert_eq!(store.remove::<u32>(), Some(1u32));
    /// assert_eq!(store.remove::<u32>(), None);
    /// ```
    #[inline]
    pub fn remove<T: 'static>(&mut self) -> Option<T> {
        self.map
            .remove(&TypeId::of::<T>())
            .and_then(|v| v.downcast::<T>().ok())
            .map(|v| *v)
    }

    /// Check if the map contains an item of type `T`.
    /// Returns `true` if it does, `false` if it doesn't.
    ///
    /// # Example
    /// ```rs
    /// use extractors::TypeStore;
    ///
    /// let mut store = TypeStore::new();
    /// store.insert(1u32);
    /// assert!(store.contains::<u32>());
    /// assert!(!store.contains::<u64>());
    /// ```
    #[inline]
    pub fn contains<T: 'static>(&self) -> bool {
        self.map.contains_key(&TypeId::of::<T>())
    }

    /// Clear the map, removing all items.
    ///
    /// # Example
    /// ```rs
    /// use extractors::TypeStore;
    ///
    /// let mut store = TypeStore::new();
    /// store.insert(1u32);
    /// store.clear();
    /// assert!(store.is_empty());
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        self.map.clear();
    }

    /// Check if the map is empty.
    /// Returns `true` if it is, `false` if it isn't.
    ///
    /// # Example
    /// ```rs
    /// use extractors::TypeStore;
    ///
    /// let mut store = TypeStore::new();
    /// assert!(store.is_empty());
    /// store.insert(1u32);
    /// assert!(!store.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}
