use crate::prelude::*;

impl StableHash for String {
    fn stable_hash<H: StableHasher>(&self, sequence_number: H::Seq, state: &mut H) {
        profile_method!(stable_hash);

        self.as_str().stable_hash(sequence_number, state);
    }
}

impl<'a> StableHash for &'a str {
    fn stable_hash<H: StableHasher>(&self, sequence_number: H::Seq, state: &mut H) {
        profile_method!(stable_hash);
        // let d = crate::CallDepth::new();
        // println!("{d}stable_hash: &str -> {self} {sequence_number:?}");
        AsBytes(self.as_bytes()).stable_hash(sequence_number, state)
    }
}
