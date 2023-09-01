use crate::prelude::*;

impl<T: StableHash> StableHash for Vec<T> {
    fn stable_hash<H: StableHasher>(&self, sequence_number: H::Seq, state: &mut H) {
        profile_method!(stable_hash);

        (&self[..]).stable_hash(sequence_number, state)
    }
}

impl<'a, T: StableHash> StableHash for &'a [T] {
    fn stable_hash<H: StableHasher>(&self, mut sequence_number: H::Seq, state: &mut H) {
        profile_method!(stable_hash);
        // let d = crate::CallDepth::new();
        // println!(
        //     "{d}start stable_hash slice {} {} {sequence_number:?}",
        //     std::any::type_name::<T>(),
        //     self.len()
        // );
        for item in self.iter() {
            item.stable_hash(sequence_number.next_child(), state);
        }
        self.len().stable_hash(sequence_number, state);
        // println!(
        //     "{d}end stable_hash slice {} {}",
        //     std::any::type_name::<T>(),
        //     self.len()
        // );
    }
}
