use crate::prelude::*;

impl StableHash for bool {
    fn stable_hash<H: StableHasher>(&self, sequence_number: H::Seq, state: &mut H) {
        profile_method!(stable_hash);
        // let d = crate::CallDepth::new();
        if *self {
            // println!("{d}stable_hash: bool {sequence_number:?}");
            state.write(sequence_number, &[]);
        } else {
            // println!("{d}stable_hash: bool skipped {sequence_number:?}");
        }
    }
}
