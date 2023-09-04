mod bool;
mod floats;
mod hash_map;
mod hash_set;
mod ints;
mod option;
mod string;
mod tuple;
mod vec;

use crate::prelude::*;

pub(self) fn unordered_unique_stable_hash<H: StableHasher>(
    items: impl Iterator<Item = impl StableHash>,
    field_address: H::Addr,
    state: &mut H,
) {
    profile_fn!(unordered_unique_stable_hash);

    let d = CallDepth::new();
    let mut count = 0;
    for member in items {
        // Must create an independent hasher to "break" relationship between
        // independent field addresses.
        // See also a817fb02-7c77-41d6-98e4-dee123884287
        let mut new_hasher = H::new();
        let (a, b) = field_address.unordered();
        member.stable_hash(a, &mut new_hasher);
        state.write(b, new_hasher.to_bytes().as_ref());
        count += 1;
        hash_debug!("member {count}: {}", hex::encode(state.to_bytes()));
    }
}

impl<'a, T: StableHash> StableHash for &'a T {
    #[inline]
    fn stable_hash<H: StableHasher>(&self, field_address: H::Addr, state: &mut H) {
        profile_method!(stable_hash);

        (*self).stable_hash(field_address, state)
    }
}
