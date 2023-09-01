use crate::prelude::*;
use std::collections::HashMap;

impl<K: StableHash, V: StableHash, S> StableHash for HashMap<K, V, S> {
    fn stable_hash<H: StableHasher>(&self, sequence_number: H::Seq, state: &mut H) {
        profile_method!(stable_hash);
        // let d = crate::CallDepth::new();
        // println!(
        //     "{d}start stable_hash HashMap: {} {sequence_number:?} ({}, {})",
        //     self.len(),
        //     std::any::type_name::<K>(),
        //     std::any::type_name::<V>()
        // );
        super::unordered_unique_stable_hash(self.iter(), sequence_number, state);
        // println!(
        //     "{d}end stable_hash HashMap: {} ({}, {})",
        //     self.len(),
        //     std::any::type_name::<K>(),
        //     std::any::type_name::<V>()
        // );
    }
}
