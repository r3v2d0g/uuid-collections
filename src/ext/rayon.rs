use std::collections::{HashMap, HashSet};

use indexmap::{IndexMap, IndexSet};
use rayon::iter::{FromParallelIterator, IntoParallelIterator, ParallelExtend};
use uuid::Uuid;
use uuid_like::UuidLike;

use crate::{
    UuidIndexMap, UuidIndexSet, UuidLikeIndexMap, UuidLikeIndexSet, UuidLikeMap, UuidLikeSet,
    UuidMap, UuidSet,
};

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
        impl<$($K: UuidLike,)? V: Send> FromParallelIterator<(key!($($K)?), V)> for $name<$($K,)? V> {
            fn from_par_iter<I>(par_iter: I) -> Self
            where
                I: IntoParallelIterator<Item = (key!($($K)?), V)>,
            {
                let inner = $inner::from_par_iter(par_iter);
                Self::from(inner)
            }
        }

        impl<$($K: UuidLike,)? V: Send> IntoParallelIterator for $name<$($K,)? V> {
            type Item = (key!($($K)?), V);
            type Iter = <$inner::<key!($($K)?), V> as IntoParallelIterator>::Iter;

            fn into_par_iter(self) -> Self::Iter {
                $inner::from(self).into_par_iter()
            }
        }

        impl<$($K: UuidLike,)? V: Send> ParallelExtend<(key!($($K)?), V)> for $name<$($K,)? V> {
            fn par_extend<I>(&mut self, iter: I)
            where
                I: IntoParallelIterator<Item = (key!($($K)?), V)>,
            {
                (**self).par_extend(iter)
            }
        }
    };
}

macro_rules! impl_set {
    ($name:ident $(<$K:ident>)? ($inner:ident)) => {
        impl$(<$K: UuidLike>)? FromParallelIterator<key!($($K)?)> for $name $(<$K>)? {
            fn from_par_iter<I>(par_iter: I) -> Self
            where
                I: IntoParallelIterator<Item = key!($($K)?)>,
            {
                let inner = $inner::from_par_iter(par_iter);
                Self::from(inner)
            }
        }

        impl $(<$K: UuidLike>)? IntoParallelIterator for $name $(<$K>)? {
            type Item = key!($($K)?);
            type Iter = <$inner::<key!($($K)?)> as IntoParallelIterator>::Iter;

            fn into_par_iter(self) -> Self::Iter {
                $inner::from(self).into_par_iter()
            }
        }

        impl $(<$K: UuidLike>)? ParallelExtend<key!($($K)?)> for $name $(<$K>)? {
            fn par_extend<I>(&mut self, par_iter: I)
            where
                I: IntoParallelIterator<Item = key!($($K)?)>,
            {
                (**self).par_extend(par_iter)
            }
        }
    };
}

impl_map!(UuidMap(HashMap));
impl_map!(UuidLikeMap<K>(HashMap));
impl_map!(UuidIndexMap(IndexMap));
impl_map!(UuidLikeIndexMap<K>(IndexMap));

impl_set!(UuidSet(HashSet));
impl_set!(UuidLikeSet<K>(HashSet));
impl_set!(UuidIndexSet(IndexSet));
impl_set!(UuidLikeIndexSet<K>(IndexSet));
