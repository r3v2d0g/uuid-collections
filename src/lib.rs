use std::{
    collections::{HashMap, HashSet, hash_map, hash_set},
    fmt::{self, Debug, Formatter},
    ops::{Deref, DerefMut},
};

use indexmap::{IndexMap, IndexSet, map, set};
use uuid::Uuid;

pub use self::{
    hasher::{UuidBuildHasher, UuidHasher},
    like::{UuidLikeIndexMap, UuidLikeIndexSet, UuidLikeMap, UuidLikeSet},
};

mod ext;
mod hasher;
mod like;
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

/// A wrapper around an [`IndexMap`] where the keys are UUIDv4s or UUIDv7s and don't
/// require hashing.
///
/// This uses [`UuidHasher`] as the hasher, so that the random bits of UUIDv4s and
/// UUIDv7s are used instead of hashing them.
///
/// ## Panics
///
/// This will panic if trying to use other UUID versions.
#[derive(Clone, PartialEq, Eq)]
pub struct UuidIndexMap<V>(IndexMap<Uuid, V, UuidBuildHasher>);

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

/// A wrapper around an [`IndexSet`] where the keys are UUIDv4s or UUIDv7s and don't
/// require hashing.
///
/// This uses [`UuidHasher`] as the hasher, so that the random bits of UUIDv4s and
/// UUIDv7s are used instead of hashing them.
///
/// ## Panics
///
/// This will panic if trying to use other UUID versions.
#[derive(Default, Clone, PartialEq, Eq)]
pub struct UuidIndexSet(IndexSet<Uuid, UuidBuildHasher>);

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
    pub fn into_keys(self) -> hash_map::IntoKeys<Uuid, V> {
        self.0.into_keys()
    }

    /// Creates a consuming iterator visiting all values in arbitrary order.
    ///
    /// See [`HashMap::into_values()`].
    #[inline]
    pub fn into_values(self) -> hash_map::IntoValues<Uuid, V> {
        self.0.into_values()
    }
}

impl<V> UuidIndexMap<V> {
    /// Creates an empty [`UuidIndexMap`].
    ///
    /// See [`IndexMap::new()`].
    #[inline]
    pub fn new() -> Self {
        Self(IndexMap::with_hasher(UuidBuildHasher))
    }

    /// Creates an empty [`UuidIndexMap`] with at least the specified capacity.
    ///
    /// See [`IndexMap::with_capacity()`].
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self(IndexMap::with_capacity_and_hasher(
            capacity,
            UuidBuildHasher,
        ))
    }

    /// Creates a consuming iterator visiting all UUIDs in arbitrary order.
    ///
    /// See [`IndexMap::into_keys()`].
    #[inline]
    pub fn into_keys(self) -> map::IntoKeys<Uuid, V> {
        self.0.into_keys()
    }

    /// Creates a consuming iterator visiting all values in arbitrary order.
    ///
    /// See [`IndexMap::into_values()`].
    #[inline]
    pub fn into_values(self) -> map::IntoValues<Uuid, V> {
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

impl UuidIndexSet {
    /// Creates an empty [`UuidIndexSet`].
    ///
    /// See [`IndexSet::new()`].
    #[inline]
    pub fn new() -> Self {
        Self(IndexSet::with_hasher(UuidBuildHasher))
    }

    /// Creates an empty [`UuidIndexSet`] with at least the specified capacity.
    ///
    /// See [`IndexSet::with_capacity()`].
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self(IndexSet::with_capacity_and_hasher(
            capacity,
            UuidBuildHasher,
        ))
    }
}

impl<V> Default for UuidMap<V> {
    #[inline]
    fn default() -> Self {
        Self(HashMap::default())
    }
}

impl<V> Default for UuidIndexMap<V> {
    #[inline]
    fn default() -> Self {
        Self(IndexMap::default())
    }
}

impl<V> From<HashMap<Uuid, V, UuidBuildHasher>> for UuidMap<V> {
    #[inline]
    fn from(map: HashMap<Uuid, V, UuidBuildHasher>) -> Self {
        Self(map)
    }
}

impl<V> From<IndexMap<Uuid, V, UuidBuildHasher>> for UuidIndexMap<V> {
    #[inline]
    fn from(map: IndexMap<Uuid, V, UuidBuildHasher>) -> Self {
        Self(map)
    }
}

impl From<HashSet<Uuid, UuidBuildHasher>> for UuidSet {
    #[inline]
    fn from(set: HashSet<Uuid, UuidBuildHasher>) -> Self {
        Self(set)
    }
}

impl From<IndexSet<Uuid, UuidBuildHasher>> for UuidIndexSet {
    #[inline]
    fn from(set: IndexSet<Uuid, UuidBuildHasher>) -> Self {
        Self(set)
    }
}

impl<V> From<UuidMap<V>> for HashMap<Uuid, V, UuidBuildHasher> {
    #[inline]
    fn from(map: UuidMap<V>) -> Self {
        map.0
    }
}

impl<V> From<UuidIndexMap<V>> for IndexMap<Uuid, V, UuidBuildHasher> {
    #[inline]
    fn from(map: UuidIndexMap<V>) -> Self {
        map.0
    }
}

impl From<UuidSet> for HashSet<Uuid, UuidBuildHasher> {
    #[inline]
    fn from(set: UuidSet) -> Self {
        set.0
    }
}

impl From<UuidIndexSet> for IndexSet<Uuid, UuidBuildHasher> {
    #[inline]
    fn from(set: UuidIndexSet) -> Self {
        set.0
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

impl<V> Deref for UuidIndexMap<V> {
    type Target = IndexMap<Uuid, V, UuidBuildHasher>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<V> DerefMut for UuidIndexMap<V> {
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

impl Deref for UuidIndexSet {
    type Target = IndexSet<Uuid, UuidBuildHasher>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for UuidIndexSet {
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

impl<V: Debug> Debug for UuidIndexMap<V> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl Debug for UuidSet {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl Debug for UuidIndexSet {
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

impl<V> Extend<(Uuid, V)> for UuidIndexMap<V> {
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

impl Extend<Uuid> for UuidIndexSet {
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

impl<V> FromIterator<(Uuid, V)> for UuidIndexMap<V> {
    #[inline]
    fn from_iter<T: IntoIterator<Item = (Uuid, V)>>(iter: T) -> Self {
        Self(IndexMap::from_iter(iter))
    }
}

impl FromIterator<Uuid> for UuidSet {
    #[inline]
    fn from_iter<T: IntoIterator<Item = Uuid>>(iter: T) -> Self {
        Self(HashSet::from_iter(iter))
    }
}

impl FromIterator<Uuid> for UuidIndexSet {
    #[inline]
    fn from_iter<T: IntoIterator<Item = Uuid>>(iter: T) -> Self {
        Self(IndexSet::from_iter(iter))
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

impl<V> IntoIterator for UuidIndexMap<V> {
    type Item = (Uuid, V);
    type IntoIter = map::IntoIter<Uuid, V>;

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

impl IntoIterator for UuidIndexSet {
    type Item = Uuid;
    type IntoIter = set::IntoIter<Uuid>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
