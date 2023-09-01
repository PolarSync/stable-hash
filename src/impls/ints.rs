use crate::prelude::*;

macro_rules! impl_int {
    ($P:ty, $N:ty) => {
        impl StableHash for $P {
            fn stable_hash<H: StableHasher>(&self, sequence_number: H::Seq, state: &mut H) {
                profile_method!(stable_hash);
                // let d = crate::CallDepth::new();
                // println!("{d}stable_hash unsigned int -> {self} {sequence_number:?}");
                AsInt {
                    is_negative: false,
                    little_endian: &self.to_le_bytes(),
                }
                .stable_hash(sequence_number, state);
                // println!("{d}end stable_hash unsigned int -> {self}");
            }
        }
        impl StableHash for $N {
            fn stable_hash<H: StableHasher>(&self, sequence_number: H::Seq, state: &mut H) {
                profile_method!(stable_hash);
                // let d = crate::CallDepth::new();
                // println!("{d}stable_hash signed int -> {self} {sequence_number:?}");
                AsInt {
                    is_negative: self.is_negative(),
                    little_endian: &self.wrapping_abs().to_le_bytes(),
                }
                .stable_hash(sequence_number, state);
                // println!("{d}end stable_hash signed int -> {self}");
            }
        }
    };
}

impl_int!(u128, i128);
impl_int!(u64, i64);
impl_int!(u32, i32);
impl_int!(u16, i16);
impl_int!(u8, i8);
impl_int!(usize, isize);
