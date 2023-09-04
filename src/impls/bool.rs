use crate::prelude::*;

impl StableHash for bool {
    #[inline]
    fn stable_hash<H: StableHasher>(&self, field_address: H::Addr, state: &mut H) {
        profile_method!(stable_hash);

        if *self {
            hash_debug!("bool");
            state.write(field_address, &[]);
        }
    }
}
