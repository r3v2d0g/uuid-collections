use std::{
    fmt::{self, Debug, Formatter},
    ops::Deref,
};

use rkyv::{
    Archive, Deserialize, Place, Portable, Serialize,
    collections::swiss_table::{
        ArchivedHashMap, ArchivedHashSet, HashMapResolver, HashSetResolver,
        index_map::{ArchivedIndexMap, IndexMapResolver},
    },
    munge::munge,
    rancor::{Fallible, Source},
    ser::{Allocator, Writer},
};
use uuid::Uuid;
use uuid_like::UuidLike;

use crate::{
    UuidHasher, UuidIndexMap, UuidLikeIndexMap, UuidLikeMap, UuidLikeSet, UuidMap, UuidSet,
};

/// An archived [`UuidMap`].
///
/// See [`ArchivedHashMap`].
#[derive(PartialEq, Eq, Portable)]
#[repr(transparent)]
// TODO(MLB): bytecheck?
pub struct ArchivedUuidMap<V>(ArchivedHashMap<Uuid, V, UuidHasher>);

/// An archived [`UuidIndexMap`].
///
/// See [`ArchivedIndexMap`].
#[derive(PartialEq, Eq, Portable)]
#[repr(transparent)]
// TODO(MLB): bytecheck?
pub struct ArchivedUuidIndexMap<V>(ArchivedIndexMap<Uuid, V, UuidHasher>);

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

/// An archived [`UuidLikeIndexMap`].
///
/// See [`ArchivedIndexMap`].
#[derive(PartialEq, Eq, Portable)]
#[repr(transparent)]
// TODO(MLB): bytecheck?
pub struct ArchivedUuidLikeIndexMap<K: UuidLike, V>(ArchivedIndexMap<K, V, UuidHasher>);

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

/// The resolver for [`ArchivedUuidIndexMap`].
///
/// See [`IndexMapResolver`].
pub struct UuidIndexMapResolver(IndexMapResolver);

/// The resolver for [`ArchivedUuidSet`].
///
/// See [`HashSetResolver`].
pub struct UuidSetResolver(HashSetResolver);

/// The resolver for [`ArchivedUuidLikeMap`].
///
/// See [`HashMapResolver`].
pub struct UuidLikeMapResolver(HashMapResolver);

/// The resolver for [`ArchivedUuidLikeIndexMap`].
///
/// See [`IndexMapResolver`].
pub struct UuidLikeIndexMapResolver(IndexMapResolver);

/// The resolver for [`ArchivedUuidLikeSet`].
///
/// See [`HashSetResolver`].
pub struct UuidLikeSetResolver(HashSetResolver);

macro_rules! key {
    ($K:ident) => {
        $K
    };

    () => {
        Uuid
    };
}

macro_rules! impl_map {
    ($name:ident $(<$K:ident>)? => $archived:ident ($iarchived:ident), $resolver:ident ($iresolver:ident)) => {
        impl<$($K: UuidLike,)? V: Archive> Archive for $name<$($K,)? V> {
            type Archived = $archived<$($K,)? V::Archived>;
            type Resolver = $resolver;

            fn resolve(&self, resolver: Self::Resolver, out: Place<Self::Archived>) {
                munge!(let $archived(out) = out);

                $iarchived::<_, V::Archived, UuidHasher>::resolve_from_len(
                    self.len(),
                    (7, 8),
                    resolver.0,
                    out,
                );
            }
        }

        impl<$($K,)? V, S> Serialize<S> for $name<$($K,)? V>
        where
            $($K: UuidLike + Serialize<S> + Archive<Archived = $K>,)?
            V: Serialize<S>,
            S: Fallible + Writer + Allocator + ?Sized,
            S::Error: Source,
        {
            fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
                let resolver = $iarchived::<_, V::Archived, UuidHasher>::serialize_from_iter::<
                    _,
                    _,
                    _,
                    key!($($K)?),
                    V,
                    _,
                >(self.iter(), (7, 8), serializer)?;

                Ok($resolver(resolver))
            }
        }

        impl<$($K,)? V, D> Deserialize<$name<$($K,)? V>, D> for $archived<$($K,)? V::Archived>
        where
            $($K: UuidLike + Deserialize<$K, D> + Archive<Archived = $K>,)?
            V: Archive,
            V::Archived: Deserialize<V, D>,
            D: Fallible + ?Sized,
        {
            fn deserialize(&self, deserializer: &mut D) -> Result<$name<$($K,)? V>, <D as Fallible>::Error> {
                let mut result = $name::with_capacity(self.len());

                for (id, value) in self.iter() {
                    let value = value.deserialize(deserializer)?;
                    result.insert(*id, value);
                }

                Ok(result)
            }
        }

        impl<$($K,)? V, AV> PartialEq<$name<$($K,)? V>> for $archived<$($K,)? AV>
        where
            $($K: UuidLike,)?
            AV: PartialEq<V>,
        {
            fn eq(&self, other: &$name<$($K,)? V>) -> bool {
                if self.len() != other.len() {
                    return false;
                }

                self.iter()
                    .all(|(id, value)| other.get(id).is_some_and(|other| value.eq(other)))
            }
        }

        impl<$($K,)? V, AV> PartialEq<$archived<$($K,)? AV>> for $name<$($K,)? V>
        where
            $($K: UuidLike,)?
            AV: PartialEq<V>,
        {
            fn eq(&self, other: &$archived<$($K,)? AV>) -> bool {
                other.eq(self)
            }
        }

        impl<$($K: UuidLike,)? V> Deref for $archived<$($K,)? V> {
            type Target = $iarchived<key!($($K)?), V, UuidHasher>;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<$($K: UuidLike,)? AV: Debug> Debug for $archived<$($K,)? AV> {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                Debug::fmt(&self.0, f)
            }
        }
    };
}

macro_rules! impl_set {
    ($name:ident $(<$K:ident>)? => $archived:ident ($iarchived:ident), $resolver:ident ($iresolver:ident)) => {
        impl $(<$K: UuidLike>)? Archive for $name $(<$K>)? {
            type Archived = $archived$(<$K>)?;
            type Resolver = $resolver;

            fn resolve(&self, resolver: Self::Resolver, out: Place<Self::Archived>) {
                munge!(let $archived(out) = out);

                $iarchived::<_, UuidHasher>::resolve_from_len(
                    self.len(),
                    (7, 8),
                    resolver.0,
                    out,
                );
            }
        }

        impl<$($K,)? S> Serialize<S> for $name $(<$K>)?
        where
            $($K: UuidLike + Serialize<S> + Archive<Archived = $K>,)?
            S: Fallible + Writer + Allocator + ?Sized,
            S::Error: Source,
        {
            fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
                let resolver = $iarchived::<_, UuidHasher>::serialize_from_iter::<
                    _,
                    key!($($K)?),
                    _,
                >(self.iter(), (7, 8), serializer)?;

                Ok($resolver(resolver))
            }
        }

        impl<$($K,)? D> Deserialize<$name $(<$K>)?, D> for $archived $(<$K>)?
        where
            $($K: UuidLike + Deserialize<$K, D> + Archive<Archived = $K>,)?
            D: Fallible + ?Sized,
        {
            fn deserialize(&self, _: &mut D) -> Result<$name $(<$K>)?, <D as Fallible>::Error> {
                let mut result = $name::with_capacity(self.len());

                for id in self.iter() {
                    result.insert(*id);
                }

                Ok(result)
            }
        }

        impl $(<$K: UuidLike>)? PartialEq<$name $(<$K>)?> for $archived $(<$K>)? {
            fn eq(&self, other: &$name $(<$K>)?) -> bool {
                if self.len() != other.len() {
                    return false;
                }

                self.iter().all(|id| other.contains(id))
            }
        }

        impl $(<$K: UuidLike>)? PartialEq<$archived $(<$K>)?> for $name $(<$K>)? {
            fn eq(&self, other: &$archived $(<$K>)?) -> bool {
                other.eq(self)
            }
        }

        impl $(<$K: UuidLike>)? Deref for $archived $(<$K>)? {
            type Target = $iarchived<key!($($K)?), UuidHasher>;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl $(<$K: UuidLike>)? Debug for $archived $(<$K>)? {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                Debug::fmt(&self.0, f)
            }
        }
    };
}

impl_map!(UuidMap => ArchivedUuidMap (ArchivedHashMap), UuidMapResolver (HashMapResolver));
impl_map!(UuidIndexMap => ArchivedUuidIndexMap (ArchivedIndexMap), UuidIndexMapResolver (IndexMapResolver));
impl_map!(UuidLikeMap<K> => ArchivedUuidLikeMap (ArchivedHashMap), UuidLikeMapResolver (HashMapResolver));
impl_map!(UuidLikeIndexMap<K> => ArchivedUuidLikeIndexMap (ArchivedIndexMap), UuidLikeIndexMapResolver (IndexMapResolver));

impl_set!(UuidSet => ArchivedUuidSet (ArchivedHashSet), UuidSetResolver (HashSetResolver));
impl_set!(UuidLikeSet<K> => ArchivedUuidLikeSet (ArchivedHashSet), UuidLikeSetResolver (HashSetResolver));
