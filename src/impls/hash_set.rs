use crate::prelude::*;
use std::collections::HashSet;

impl<T: StableHash, S> StableHash for HashSet<T, S> {
    fn stable_hash<H: StableHasher>(&self, sequence_number: H::Seq, state: &mut H) {
        profile_method!(stable_hash);
        // let d = crate::CallDepth::new();
        // println!(
        //     "{d}start stable_hash HashSet: {} {sequence_number:?} {}",
        //     self.len(),
        //     std::any::type_name::<T>()
        // );
        super::unordered_unique_stable_hash(self.iter(), sequence_number, state);
        // println!(
        //     "{d}end stable_hash HashSet: {} {}",
        //     self.len(),
        //     std::any::type_name::<T>()
        // );
    }
}
