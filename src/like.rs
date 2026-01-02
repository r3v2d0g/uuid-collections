use std::{
    collections::{HashMap, HashSet, hash_map, hash_set},
    fmt::{self, Debug, Formatter},
    ops::{Deref, DerefMut},
};

use indexmap::{IndexMap, IndexSet, map, set};
use uuid_like::UuidLike;

use crate::UuidBuildHasher;

/// A [`UuidMap`][1] for UUID-like keys which implement [`UuidLike`].
///
/// See [`UuidMap`][1] and [`UuidLike`] for more details.
///
/// [1]: crate::UuidMap
#[derive(Clone, PartialEq, Eq)]
pub struct UuidLikeMap<K: UuidLike, V>(HashMap<K, V, UuidBuildHasher>);

/// A [`UuidIndexMap`][1] for UUID-like keys which implement [`UuidLike`].
///
/// See [`UuidIndexMap`][1] and [`UuidLike`] for more details.
///
/// [1]: crate::UuidIndexMap
#[derive(Clone, PartialEq, Eq)]
pub struct UuidLikeIndexMap<K: UuidLike, V>(IndexMap<K, V, UuidBuildHasher>);

/// A [`UuidSet`][1] for UUID-like keys which implement [`UuidLike`].
///
/// See [`UuidSet`][1] and [`UuidLike`] for more details.
///
/// [1]: crate::UuidSet
#[derive(Clone, PartialEq, Eq)]
pub struct UuidLikeSet<K: UuidLike>(HashSet<K, UuidBuildHasher>);

/// A [`UuidIndexSet`][1] for UUID-like keys which implement [`UuidLike`].
///
/// See [`UuidIndexSet`][1] and [`UuidLike`] for more details.
///
/// [1]: crate::UuidSet
#[derive(Clone, PartialEq, Eq)]
pub struct UuidLikeIndexSet<K: UuidLike>(IndexSet<K, UuidBuildHasher>);

impl<K: UuidLike, V> UuidLikeMap<K, V> {
    /// Creates an empty [`UuidLikeMap`].
    ///
    /// See [`HashMap::new()`].
    #[inline]
    pub fn new() -> Self {
        Self(HashMap::with_hasher(UuidBuildHasher))
    }

    /// Creates an empty [`UuidLikeMap`] with at least the specified capacity.
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
    pub fn into_keys(self) -> hash_map::IntoKeys<K, V> {
        self.0.into_keys()
    }

    /// Creates a consuming iterator visiting all values in arbitrary order.
    ///
    /// See [`HashMap::into_values()`].
    #[inline]
    pub fn into_values(self) -> hash_map::IntoValues<K, V> {
        self.0.into_values()
    }
}

impl<K: UuidLike, V> UuidLikeIndexMap<K, V> {
    /// Creates an empty [`UuidLikeIndexMap`].
    ///
    /// See [`IndexMap::new()`].
    #[inline]
    pub fn new() -> Self {
        Self(IndexMap::with_hasher(UuidBuildHasher))
    }

    /// Creates an empty [`UuidLikeIndexMap`] with at least the specified capacity.
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
    pub fn into_keys(self) -> map::IntoKeys<K, V> {
        self.0.into_keys()
    }

    /// Creates a consuming iterator visiting all values in arbitrary order.
    ///
    /// See [`IndexMap::into_values()`].
    #[inline]
    pub fn into_values(self) -> map::IntoValues<K, V> {
        self.0.into_values()
    }
}

impl<K: UuidLike> UuidLikeSet<K> {
    /// Creates an empty [`UuidLikeSet`].
    ///
    /// See [`HashSet::new()`].
    #[inline]
    pub fn new() -> Self {
        Self(HashSet::with_hasher(UuidBuildHasher))
    }

    /// Creates an empty [`UuidLikeSet`] with at least the specified capacity.
    ///
    /// See [`HashSet::with_capacity()`].
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self(HashSet::with_capacity_and_hasher(capacity, UuidBuildHasher))
    }
}

impl<K: UuidLike> UuidLikeIndexSet<K> {
    /// Creates an empty [`UuidLikeIndexSet`].
    ///
    /// See [`IndexSet::new()`].
    #[inline]
    pub fn new() -> Self {
        Self(IndexSet::with_hasher(UuidBuildHasher))
    }

    /// Creates an empty [`UuidLikeSet`] with at least the specified capacity.
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

impl<K: UuidLike, V> Default for UuidLikeMap<K, V> {
    #[inline]
    fn default() -> Self {
        Self(HashMap::default())
    }
}

impl<K: UuidLike, V> Default for UuidLikeIndexMap<K, V> {
    #[inline]
    fn default() -> Self {
        Self(IndexMap::default())
    }
}

impl<K: UuidLike> Default for UuidLikeSet<K> {
    fn default() -> Self {
        Self(HashSet::with_hasher(UuidBuildHasher))
    }
}

impl<K: UuidLike> Default for UuidLikeIndexSet<K> {
    fn default() -> Self {
        Self(IndexSet::with_hasher(UuidBuildHasher))
    }
}

impl<K: UuidLike, V> From<HashMap<K, V, UuidBuildHasher>> for UuidLikeMap<K, V> {
    #[inline]
    fn from(map: HashMap<K, V, UuidBuildHasher>) -> Self {
        Self(map)
    }
}

impl<K: UuidLike, V> From<IndexMap<K, V, UuidBuildHasher>> for UuidLikeIndexMap<K, V> {
    #[inline]
    fn from(map: IndexMap<K, V, UuidBuildHasher>) -> Self {
        Self(map)
    }
}

impl<K: UuidLike> From<HashSet<K, UuidBuildHasher>> for UuidLikeSet<K> {
    #[inline]
    fn from(set: HashSet<K, UuidBuildHasher>) -> Self {
        Self(set)
    }
}

impl<K: UuidLike> From<IndexSet<K, UuidBuildHasher>> for UuidLikeIndexSet<K> {
    #[inline]
    fn from(set: IndexSet<K, UuidBuildHasher>) -> Self {
        Self(set)
    }
}

impl<K: UuidLike, V> From<UuidLikeMap<K, V>> for HashMap<K, V, UuidBuildHasher> {
    #[inline]
    fn from(map: UuidLikeMap<K, V>) -> Self {
        map.0
    }
}

impl<K: UuidLike, V> From<UuidLikeIndexMap<K, V>> for IndexMap<K, V, UuidBuildHasher> {
    #[inline]
    fn from(map: UuidLikeIndexMap<K, V>) -> Self {
        map.0
    }
}

impl<K: UuidLike> From<UuidLikeSet<K>> for HashSet<K, UuidBuildHasher> {
    #[inline]
    fn from(set: UuidLikeSet<K>) -> Self {
        set.0
    }
}

impl<K: UuidLike> From<UuidLikeIndexSet<K>> for IndexSet<K, UuidBuildHasher> {
    #[inline]
    fn from(set: UuidLikeIndexSet<K>) -> Self {
        set.0
    }
}

impl<K: UuidLike, V> Deref for UuidLikeMap<K, V> {
    type Target = HashMap<K, V, UuidBuildHasher>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K: UuidLike, V> DerefMut for UuidLikeMap<K, V> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<K: UuidLike, V> Deref for UuidLikeIndexMap<K, V> {
    type Target = IndexMap<K, V, UuidBuildHasher>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K: UuidLike, V> DerefMut for UuidLikeIndexMap<K, V> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<K: UuidLike> Deref for UuidLikeSet<K> {
    type Target = HashSet<K, UuidBuildHasher>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K: UuidLike> DerefMut for UuidLikeSet<K> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<K: UuidLike> Deref for UuidLikeIndexSet<K> {
    type Target = IndexSet<K, UuidBuildHasher>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K: UuidLike> DerefMut for UuidLikeIndexSet<K> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<K: UuidLike, V: Debug> Debug for UuidLikeMap<K, V> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl<K: UuidLike, V: Debug> Debug for UuidLikeIndexMap<K, V> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl<K: UuidLike> Debug for UuidLikeSet<K> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl<K: UuidLike> Debug for UuidLikeIndexSet<K> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl<K: UuidLike, V> Extend<(K, V)> for UuidLikeMap<K, V> {
    #[inline]
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

impl<K: UuidLike, V> Extend<(K, V)> for UuidLikeIndexMap<K, V> {
    #[inline]
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

impl<K: UuidLike> Extend<K> for UuidLikeSet<K> {
    #[inline]
    fn extend<T: IntoIterator<Item = K>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

impl<K: UuidLike> Extend<K> for UuidLikeIndexSet<K> {
    #[inline]
    fn extend<T: IntoIterator<Item = K>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

impl<K: UuidLike, V> FromIterator<(K, V)> for UuidLikeMap<K, V> {
    #[inline]
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Self(HashMap::from_iter(iter))
    }
}

impl<K: UuidLike, V> FromIterator<(K, V)> for UuidLikeIndexMap<K, V> {
    #[inline]
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Self(IndexMap::from_iter(iter))
    }
}

impl<K: UuidLike> FromIterator<K> for UuidLikeSet<K> {
    #[inline]
    fn from_iter<T: IntoIterator<Item = K>>(iter: T) -> Self {
        Self(HashSet::from_iter(iter))
    }
}

impl<K: UuidLike> FromIterator<K> for UuidLikeIndexSet<K> {
    #[inline]
    fn from_iter<T: IntoIterator<Item = K>>(iter: T) -> Self {
        Self(IndexSet::from_iter(iter))
    }
}

impl<K: UuidLike, V> IntoIterator for UuidLikeMap<K, V> {
    type Item = (K, V);
    type IntoIter = hash_map::IntoIter<K, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<K: UuidLike, V> IntoIterator for UuidLikeIndexMap<K, V> {
    type Item = (K, V);
    type IntoIter = map::IntoIter<K, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<K: UuidLike> IntoIterator for UuidLikeSet<K> {
    type Item = K;
    type IntoIter = hash_set::IntoIter<K>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<K: UuidLike> IntoIterator for UuidLikeIndexSet<K> {
    type Item = K;
    type IntoIter = set::IntoIter<K>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
