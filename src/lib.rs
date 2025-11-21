use std::{
    collections::{
        HashMap, HashSet,
        hash_map::{self, IntoKeys, IntoValues},
        hash_set,
    },
    fmt::{self, Debug, Formatter},
    ops::{Deref, DerefMut},
};

use uuid::Uuid;

pub use self::hasher::{UuidBuildHasher, UuidHasher};

mod hasher;
#[cfg(test)]
mod tests;

/// A wrapper around an [`HashMap`] where the keys are UUIDv4s or UUIDv7s and don't
/// require hashing.
///
/// This uses [`UuidHasher`] as the hasher, so that the random bits of UUIDv4s and
/// UUIDv7s are used instead of hashing them.
///
/// ## Panics
///
/// This will panic if trying to use other UUID versions.
#[derive(Clone, PartialEq, Eq)]
pub struct UuidMap<V>(HashMap<Uuid, V, UuidBuildHasher>);

/// A wrapper around an [`HashSet`] where the keys are UUIDv4s or UUIDv7s and don't
/// require hashing.
///
/// This uses [`UuidHasher`] as the hasher, so that the random bits of UUIDv4s and
/// UUIDv7s are used instead of hashing them.
///
/// ## Panics
///
/// This will panic if trying to use other UUID versions.
#[derive(Default, Clone, PartialEq, Eq)]
pub struct UuidSet(HashSet<Uuid, UuidBuildHasher>);

impl<V> UuidMap<V> {
    /// Creates an empty [`UuidMap`].
    ///
    /// See [`HashMap::new()`].
    #[inline]
    pub fn new() -> Self {
        Self(HashMap::with_hasher(UuidBuildHasher))
    }

    /// Creates an empty [`UuidMap`] with at least the specified capacity.
    ///
    /// See [`HashMap::with_capacity()`].
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self(HashMap::with_capacity_and_hasher(capacity, UuidBuildHasher))
    }

    /// Creates a consuming iterator visiting all UUIDs in arbitrary order.
    ///
    /// See [`HashMap::into_keys()`].
    #[inline]
    pub fn into_keys(self) -> IntoKeys<Uuid, V> {
        self.0.into_keys()
    }

    /// Creates a consuming iterator visiting all values in arbitrary order.
    ///
    /// See [`HashMap::into_values()`].
    #[inline]
    pub fn into_values(self) -> IntoValues<Uuid, V> {
        self.0.into_values()
    }
}

impl UuidSet {
    /// Creates an empty [`UuidSet`].
    ///
    /// See [`HashSet::new()`].
    #[inline]
    pub fn new() -> Self {
        Self(HashSet::with_hasher(UuidBuildHasher))
    }

    /// Creates an empty [`UuidSet`] with at least the specified capacity.
    ///
    /// See [`HashSet::with_capacity()`].
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self(HashSet::with_capacity_and_hasher(capacity, UuidBuildHasher))
    }
}

impl<V> Default for UuidMap<V> {
    fn default() -> Self {
        Self(HashMap::default())
    }
}

impl<V> Deref for UuidMap<V> {
    type Target = HashMap<Uuid, V, UuidBuildHasher>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<V> DerefMut for UuidMap<V> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for UuidSet {
    type Target = HashSet<Uuid, UuidBuildHasher>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for UuidSet {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<V: Debug> Debug for UuidMap<V> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl Debug for UuidSet {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl<V> Extend<(Uuid, V)> for UuidMap<V> {
    #[inline]
    fn extend<T: IntoIterator<Item = (Uuid, V)>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

impl Extend<Uuid> for UuidSet {
    #[inline]
    fn extend<T: IntoIterator<Item = Uuid>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

impl<V> FromIterator<(Uuid, V)> for UuidMap<V> {
    #[inline]
    fn from_iter<T: IntoIterator<Item = (Uuid, V)>>(iter: T) -> Self {
        Self(HashMap::from_iter(iter))
    }
}

impl FromIterator<Uuid> for UuidSet {
    #[inline]
    fn from_iter<T: IntoIterator<Item = Uuid>>(iter: T) -> Self {
        Self(HashSet::from_iter(iter))
    }
}

impl<V> IntoIterator for UuidMap<V> {
    type Item = (Uuid, V);
    type IntoIter = hash_map::IntoIter<Uuid, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl IntoIterator for UuidSet {
    type Item = Uuid;
    type IntoIter = hash_set::IntoIter<Uuid>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
