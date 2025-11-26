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

use crate::{UuidHasher, UuidMap, UuidSet};

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

/// The resolver for [`ArchivedUuidMap`].
///
/// See [`HashMapResolver`].
pub struct UuidMapResolver(HashMapResolver);

/// The resolver for [`ArchivedUuidSet`].
///
/// See [`HashSetResolver`].
pub struct UuidSetResolver(HashSetResolver);

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
