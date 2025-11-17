use std::hash::{BuildHasher, Hasher};

/// A [`BuildHasher`] that builds [`UuidHasher`]s, which use the random bits of
/// UUIDv4s and UUIDv7s instead of hashing them.
#[derive(Clone, Copy, Default)]
pub struct UuidBuildHasher;

/// A [`Hasher`] which uses the random bits of UUIDv4s and UUIDv7s instead of
/// hashing them.
///
/// ## Panics
///
/// This will panic if used with something other than a UUIDv4 or a UUIDv7.
#[derive(Default)]
pub struct UuidHasher {
    hash: u64,
}

impl BuildHasher for UuidBuildHasher {
    type Hasher = UuidHasher;

    fn build_hasher(&self) -> UuidHasher {
        UuidHasher::default()
    }
}

macro_rules! not_supported {
    ($($method:ident : $ty:ty),+ $(,)?) => {$(
        fn $method(&mut self, _: $ty) {
            panic!(
                "`{}` is not suppported, only `write` is",
                stringify!($method)
            );
        }
    )+};
}

impl Hasher for UuidHasher {
    fn finish(&self) -> u64 {
        self.hash
    }

    fn write(&mut self, bytes: &[u8]) {
        assert_eq!(bytes.len(), 16);

        let version = (bytes[6] & 0b11110000) >> 4;
        assert!(version == 4 || version == 7);

        let variant = (bytes[8] & 0b11000000) >> 6;
        assert_eq!(variant, 2);

        // UUIDv4s have the following bit pattern:
        // ```
        //  0                   1                   2                   3
        //  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
        // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
        // |                           random_a                            |
        // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
        // |          random_a             |  ver  |       random_b        |
        // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
        // |var|                       random_c                            |
        // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
        // |                           random_c                            |
        // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
        // ```
        //
        // UUIDv7s have the following bit pattern:
        // ```
        //  0                   1                   2                   3
        //  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
        // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
        // |                           unix_ts_ms                          |
        // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
        // |          unix_ts_ms           |  ver  |       rand_a          |
        // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
        // |var|                        rand_b                             |
        // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
        // |                            rand_b                             |
        // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
        // ```
        //
        // We thus use the 8th byte (part of `random_b` and `rand_a` respectively) and the
        // 7 last bytes (part of `random_c` and `rand_b` respectively) as the hash.

        let mut out = [0; 8];
        out[0] = bytes[7];
        out[1] = bytes[9];
        out[2] = bytes[10];
        out[3] = bytes[11];
        out[4] = bytes[12];
        out[5] = bytes[13];
        out[6] = bytes[14];
        out[7] = bytes[15];

        let hash = u64::from_be_bytes(out);
        self.hash = hash;
    }

    not_supported!(
        write_i8: i8,
        write_i16: i16,
        write_i32: i32,
        write_i64: i64,
        write_i128: i128,
        write_isize: isize,
        write_u8: u8,
        write_u16: u16,
        write_u32: u32,
        write_u64: u64,
        write_u128: u128,
        write_usize: usize,
    );
}
