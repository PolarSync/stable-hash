use std::convert::TryInto;

use super::fld::FldMix;
use crate::prelude::*;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct FastStableHasher {
    mixer: FldMix,
    count: u64,
}

impl StableHasher for FastStableHasher {
    type Out = u128;
    type Addr = u128;
    type Bytes = [u8; 32];

    fn new() -> Self {
        hash_debug!("New hasher");
        Self {
            mixer: FldMix::new(),
            count: 0,
        }
    }

    fn mixin(&mut self, other: &Self) {
        hash_debug!(
            "mixin start {}, {}",
            hex::encode(self.to_bytes()),
            hex::encode(other.to_bytes())
        );
        self.mixer.mixin(&other.mixer);
        self.count += other.count;
        hash_debug!("mixin end {}", hex::encode(self.to_bytes()));
    }

    fn to_bytes(&self) -> Self::Bytes {
        let mixer = self.mixer.to_bytes();
        let count = self.count.to_le_bytes();

        let mut bytes = [0; 32];
        bytes[0..24].copy_from_slice(&mixer);
        bytes[24..32].copy_from_slice(&count);
        bytes
    }

    fn from_bytes(bytes: Self::Bytes) -> Self {
        Self {
            mixer: FldMix::from_bytes(bytes[0..24].try_into().unwrap()),
            count: u64::from_le_bytes(bytes[24..32].try_into().unwrap()),
        }
    }

    fn write(&mut self, field_address: Self::Addr, bytes: &[u8]) {
        profile_method!(write);

        // xxh3 128 has no weaknesses listed on SMHasher.
        // It also is built for checksumming, meaning all bytes are accounted for.
        // And it is the fastest, making it a clear choice.
        // Also considered: t1ha3, MetroHash, SipHasher24
        // For more information about XXH3, see this:
        // https://fastcompression.blogspot.com/2019/03/presenting-xxh3.html
        let _d = CallDepth::new();
        hash_debug!(
            "start write bytes: {}, hashed #{}, {}",
            hex::encode(bytes),
            self.count,
            hex::encode(self.to_bytes())
        );
        let hash = xxhash_rust::xxh3::xxh3_128_with_seed(bytes, field_address as u64);
        self.mixer.mix(hash, (field_address >> 64) as u64);
        self.count += 1;
        hash_debug!(
            "end write bytes: {}, hashed #{}, {}",
            hex::encode(bytes),
            self.count,
            hex::encode(self.to_bytes())
        );
    }

    fn finish(&self) -> u128 {
        profile_method!(finish);
        xxhash_rust::xxh3::xxh3_128_with_seed(&self.mixer.to_bytes(), self.count)
    }
}
