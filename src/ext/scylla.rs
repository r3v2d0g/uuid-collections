use std::collections::{HashMap, HashSet};

use scylla::{
    cluster::metadata::ColumnType,
    deserialize::{DeserializationError, FrameSlice, value::DeserializeValue},
    errors::TypeCheckError,
    serialize::{
        SerializationError,
        value::SerializeValue,
        writers::{CellWriter, WrittenCellProof},
    },
};
use uuid::Uuid;
use uuid_like::UuidLike;

use crate::{UuidLikeMap, UuidLikeSet, UuidMap, UuidSet};

macro_rules! key {
    ($K:ident) => {
        $K
    };

    () => {
        Uuid
    };
}

macro_rules! impl_map {
    ($name:ident $(<$K:ident>)? ($inner:ident)) => {
        impl<$($K,)? V: SerializeValue> SerializeValue for $name<$($K,)? V>
        $(where $K: UuidLike + SerializeValue,)?
        {
            #[inline]
            fn serialize<'b>(
                &self,
                typ: &ColumnType,
                writer: CellWriter<'b>,
            ) -> Result<WrittenCellProof<'b>, SerializationError> {
                (&**self).serialize(typ, writer)
            }
        }

        impl<'frame, 'metadata, $($K,)? V> DeserializeValue<'frame, 'metadata> for $name<$($K,)? V>
        where
            $($K: UuidLike + DeserializeValue<'frame, 'metadata>,)?
            V: DeserializeValue<'frame, 'metadata>,
        {
            #[inline]
            fn type_check(typ: &ColumnType<'_>) -> Result<(), TypeCheckError> {
                $inner::<key!($($K)?), V>::type_check(typ)
            }

            #[inline]
            fn deserialize(
                typ: &'metadata ColumnType<'metadata>,
                v: Option<FrameSlice<'frame>>,
            ) -> Result<Self, DeserializationError> {
                let inner = $inner::<key!($($K)?), V, _>::deserialize(typ, v)?;
                Ok(Self::from(inner))
            }
        }
    };
}

macro_rules! impl_set {
    ($name:ident $(<$K:ident>)? ($inner:ident)) => {
        impl $(<$K>)? SerializeValue for $name $(<$K>)?
        $(where $K: UuidLike + SerializeValue,)?
        {
            #[inline]
            fn serialize<'b>(
                &self,
                typ: &ColumnType,
                writer: CellWriter<'b>,
            ) -> Result<WrittenCellProof<'b>, SerializationError> {
                (&**self).serialize(typ, writer)
            }
        }

        impl<'frame, 'metadata $(, $K)?> DeserializeValue<'frame, 'metadata> for $name $(<$K>)?
        $(where $K: UuidLike + DeserializeValue<'frame, 'metadata>)?
        {
            #[inline]
            fn type_check(typ: &ColumnType<'_>) -> Result<(), TypeCheckError> {
                $inner::<key!($($K)?)>::type_check(typ)
            }

            #[inline]
            fn deserialize(
                typ: &'metadata ColumnType<'metadata>,
                v: Option<FrameSlice<'frame>>,
            ) -> Result<Self, DeserializationError> {
                let inner = $inner::<key!($($K)?), _>::deserialize(typ, v)?;
                Ok(Self::from(inner))
            }
        }
    };
}

impl_map!(UuidMap(HashMap));
impl_map!(UuidLikeMap<K> (HashMap));
// TODO(MLB): `UuidIndexMap` and `UuidLikeIndexMap`

impl_set!(UuidSet(HashSet));
impl_set!(UuidLikeSet<K> (HashSet));
// TODO(MLB): `UuidIndexSet` and `UuidLikeIndexSet`
