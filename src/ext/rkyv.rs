use std::{
    fmt::{self, Debug, Formatter},
    ops::Deref,
};

use rkyv::{
    Archive, Deserialize, Place, Portable, Serialize,
    collections::swiss_table::{
        ArchivedHashMap, ArchivedHashSet, HashMapResolver, HashSetResolver,
    },
    munge::munge,
    rancor::{Fallible, Source},
    ser::{Allocator, Writer},
};
use uuid::Uuid;
use uuid_like::UuidLike;

use crate::{UuidHasher, UuidLikeMap, UuidLikeSet, UuidMap, UuidSet};

/// An archived [`UuidMap`].
///
/// See [`ArchivedHashMap`].
#[derive(PartialEq, Eq, Portable)]
#[repr(transparent)]
// TODO(MLB): bytecheck?
pub struct ArchivedUuidMap<V>(ArchivedHashMap<Uuid, V, UuidHasher>);

/// An archived [`UuidSet`].
///
/// See [`ArchivedHashSet`].
#[derive(PartialEq, Eq, Portable)]
#[repr(transparent)]
// TODO(MLB): bytecheck?
pub struct ArchivedUuidSet(ArchivedHashSet<Uuid, UuidHasher>);

/// An archived [`UuidLikeMap`].
///
/// See [`ArchivedHashMap`].
#[derive(PartialEq, Eq, Portable)]
#[repr(transparent)]
// TODO(MLB): bytecheck?
pub struct ArchivedUuidLikeMap<K: UuidLike, V>(ArchivedHashMap<K, V, UuidHasher>);

/// An archived [`UuidSet`].
///
/// See [`ArchivedHashSet`].
#[derive(PartialEq, Eq, Portable)]
#[repr(transparent)]
// TODO(MLB): bytecheck?
pub struct ArchivedUuidLikeSet<K: UuidLike>(ArchivedHashSet<K, UuidHasher>);

/// The resolver for [`ArchivedUuidMap`].
///
/// See [`HashMapResolver`].
pub struct UuidMapResolver(HashMapResolver);

/// The resolver for [`ArchivedUuidSet`].
///
/// See [`HashSetResolver`].
pub struct UuidSetResolver(HashSetResolver);

/// The resolver for [`ArchivedUuidLikeMap`].
///
/// See [`HashMapResolver`].
pub struct UuidLikeMapResolver(HashMapResolver);

/// The resolver for [`ArchivedUuidLikeSet`].
///
/// See [`HashSetResolver`].
pub struct UuidLikeSetResolver(HashSetResolver);

impl<V: Archive> Archive for UuidMap<V> {
    type Archived = ArchivedUuidMap<V::Archived>;
    type Resolver = UuidMapResolver;

    fn resolve(&self, resolver: Self::Resolver, out: Place<Self::Archived>) {
        munge!(let ArchivedUuidMap(out) = out);

        ArchivedHashMap::<Uuid, V::Archived, UuidHasher>::resolve_from_len(
            self.len(),
            (7, 8),
            resolver.0,
            out,
        );
    }
}

impl Archive for UuidSet {
    type Archived = ArchivedUuidSet;
    type Resolver = UuidSetResolver;

    fn resolve(&self, resolver: Self::Resolver, out: Place<Self::Archived>) {
        munge!(let ArchivedUuidSet(out) = out);

        ArchivedHashSet::<Uuid, UuidHasher>::resolve_from_len(self.len(), (7, 8), resolver.0, out);
    }
}

impl<K: UuidLike, V: Archive> Archive for UuidLikeMap<K, V> {
    type Archived = ArchivedUuidLikeMap<K, V::Archived>;
    type Resolver = UuidLikeMapResolver;

    fn resolve(&self, resolver: Self::Resolver, out: Place<Self::Archived>) {
        munge!(let ArchivedUuidLikeMap(out) = out);

        ArchivedHashMap::<K, V::Archived, UuidHasher>::resolve_from_len(
            self.len(),
            (7, 8),
            resolver.0,
            out,
        );
    }
}

impl<K: UuidLike> Archive for UuidLikeSet<K> {
    type Archived = ArchivedUuidLikeSet<K>;
    type Resolver = UuidLikeSetResolver;

    fn resolve(&self, resolver: Self::Resolver, out: Place<Self::Archived>) {
        munge!(let ArchivedUuidLikeSet(out) = out);

        ArchivedHashSet::<K, UuidHasher>::resolve_from_len(self.len(), (7, 8), resolver.0, out);
    }
}

impl<V, S> Serialize<S> for UuidMap<V>
where
    V: Serialize<S>,
    S: Fallible + Writer + Allocator + ?Sized,
    S::Error: Source,
{
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        let resolver = ArchivedHashMap::<Uuid, V::Archived, UuidHasher>::serialize_from_iter::<
            _,
            _,
            _,
            Uuid,
            V,
            _,
        >(self.iter(), (7, 8), serializer)?;

        Ok(UuidMapResolver(resolver))
    }
}

impl<S> Serialize<S> for UuidSet
where
    S: Fallible + Writer + Allocator + ?Sized,
    S::Error: Source,
{
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        let resolver = ArchivedHashSet::<Uuid, UuidHasher>::serialize_from_iter::<_, Uuid, _>(
            self.iter(),
            (7, 8),
            serializer,
        )?;

        Ok(UuidSetResolver(resolver))
    }
}

impl<K, V, S> Serialize<S> for UuidLikeMap<K, V>
where
    K: UuidLike + Serialize<S> + Archive<Archived = K>,
    V: Serialize<S>,
    S: Fallible + Writer + Allocator + ?Sized,
    S::Error: Source,
{
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        let resolver =
            ArchivedHashMap::<K, V::Archived, UuidHasher>::serialize_from_iter::<_, _, _, K, V, _>(
                self.iter(),
                (7, 8),
                serializer,
            )?;

        Ok(UuidLikeMapResolver(resolver))
    }
}

impl<K, S> Serialize<S> for UuidLikeSet<K>
where
    K: UuidLike + Serialize<S> + Archive<Archived = K>,
    S: Fallible + Writer + Allocator + ?Sized,
    S::Error: Source,
{
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        let resolver = ArchivedHashSet::<K, UuidHasher>::serialize_from_iter::<_, K, _>(
            self.iter(),
            (7, 8),
            serializer,
        )?;

        Ok(UuidLikeSetResolver(resolver))
    }
}

impl<V, D> Deserialize<UuidMap<V>, D> for ArchivedUuidMap<V::Archived>
where
    V: Archive,
    V::Archived: Deserialize<V, D>,
    D: Fallible + ?Sized,
{
    fn deserialize(&self, deserializer: &mut D) -> Result<UuidMap<V>, <D as Fallible>::Error> {
        let mut result = UuidMap::with_capacity(self.len());

        for (id, value) in self.iter() {
            let value = value.deserialize(deserializer)?;
            result.insert(*id, value);
        }

        Ok(result)
    }
}

impl<D> Deserialize<UuidSet, D> for ArchivedUuidSet
where
    D: Fallible + ?Sized,
{
    fn deserialize(&self, _: &mut D) -> Result<UuidSet, <D as Fallible>::Error> {
        let mut result = UuidSet::with_capacity(self.len());

        for id in self.iter() {
            result.insert(*id);
        }

        Ok(result)
    }
}

impl<K, V, D> Deserialize<UuidLikeMap<K, V>, D> for ArchivedUuidLikeMap<K, V::Archived>
where
    K: UuidLike + Deserialize<K, D> + Archive<Archived = K>,
    V: Archive,
    V::Archived: Deserialize<V, D>,
    D: Fallible + ?Sized,
{
    fn deserialize(
        &self,
        deserializer: &mut D,
    ) -> Result<UuidLikeMap<K, V>, <D as Fallible>::Error> {
        let mut result = UuidLikeMap::with_capacity(self.len());

        for (id, value) in self.iter() {
            let value = value.deserialize(deserializer)?;
            result.insert(*id, value);
        }

        Ok(result)
    }
}

impl<K, D> Deserialize<UuidLikeSet<K>, D> for ArchivedUuidLikeSet<K>
where
    K: UuidLike + Deserialize<K, D> + Archive<Archived = K>,
    D: Fallible + ?Sized,
{
    fn deserialize(&self, _: &mut D) -> Result<UuidLikeSet<K>, <D as Fallible>::Error> {
        let mut result = UuidLikeSet::with_capacity(self.len());

        for id in self.iter() {
            result.insert(*id);
        }

        Ok(result)
    }
}

impl<V, AV> PartialEq<UuidMap<V>> for ArchivedUuidMap<AV>
where
    AV: PartialEq<V>,
{
    fn eq(&self, other: &UuidMap<V>) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter()
            .all(|(id, value)| other.get(id).is_some_and(|other| value.eq(other)))
    }
}

impl PartialEq<UuidSet> for ArchivedUuidSet {
    fn eq(&self, other: &UuidSet) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter().all(|id| other.contains(id))
    }
}

impl<K, V, AV> PartialEq<UuidLikeMap<K, V>> for ArchivedUuidLikeMap<K, AV>
where
    K: UuidLike,
    AV: PartialEq<V>,
{
    fn eq(&self, other: &UuidLikeMap<K, V>) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter()
            .all(|(id, value)| other.get(id).is_some_and(|other| value.eq(other)))
    }
}

impl<K: UuidLike> PartialEq<UuidLikeSet<K>> for ArchivedUuidLikeSet<K> {
    fn eq(&self, other: &UuidLikeSet<K>) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter().all(|id| other.contains(id))
    }
}

impl<V, AV> PartialEq<ArchivedUuidMap<AV>> for UuidMap<V>
where
    AV: PartialEq<V>,
{
    fn eq(&self, other: &ArchivedUuidMap<AV>) -> bool {
        other.eq(self)
    }
}

impl PartialEq<ArchivedUuidSet> for UuidSet {
    fn eq(&self, other: &ArchivedUuidSet) -> bool {
        other.eq(self)
    }
}

impl<K, V, AV> PartialEq<ArchivedUuidLikeMap<K, AV>> for UuidLikeMap<K, V>
where
    K: UuidLike,
    AV: PartialEq<V>,
{
    fn eq(&self, other: &ArchivedUuidLikeMap<K, AV>) -> bool {
        other.eq(self)
    }
}

impl<K: UuidLike> PartialEq<ArchivedUuidLikeSet<K>> for UuidLikeSet<K> {
    fn eq(&self, other: &ArchivedUuidLikeSet<K>) -> bool {
        other.eq(self)
    }
}

impl<V> Deref for ArchivedUuidMap<V> {
    type Target = ArchivedHashMap<Uuid, V, UuidHasher>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for ArchivedUuidSet {
    type Target = ArchivedHashSet<Uuid, UuidHasher>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K: UuidLike, V> Deref for ArchivedUuidLikeMap<K, V> {
    type Target = ArchivedHashMap<K, V, UuidHasher>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K: UuidLike> Deref for ArchivedUuidLikeSet<K> {
    type Target = ArchivedHashSet<K, UuidHasher>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<AV: Debug> Debug for ArchivedUuidMap<AV> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl Debug for ArchivedUuidSet {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl<K: UuidLike, AV: Debug> Debug for ArchivedUuidLikeMap<K, AV> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl<K: UuidLike> Debug for ArchivedUuidLikeSet<K> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.0, f)
    }
}
