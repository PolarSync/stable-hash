use crate::prelude::*;

macro_rules! impl_tuple {
    ($($T:ident),*) => {
        impl<$($T : StableHash,)*> StableHash for ($($T,)*) {
            #[allow(non_snake_case)]
            fn stable_hash<H: StableHasher>(&self, mut sequence_number: H::Seq, state: &mut H) {
                profile_method!(stable_hash);

                let ($($T,)*) = self;
                // let d = crate::CallDepth::new();
                // println!("{d}start stable_hash tuple {} {sequence_number:?}", std::any::type_name::<($($T,)*)>());
                $(
                    {
                        // let d = crate::CallDepth::new();
                        // println!("{d}start stable_hash tuple index {} {sequence_number:?}", std::any::type_name::<$T>());
                        $T.stable_hash(sequence_number.next_child(), state);
                        // println!("{d}end stable_hash tuple index {} {sequence_number:?}", std::any::type_name::<$T>());
                    }
                )*
                // println!("{d}end stable_hash tuple {} {sequence_number:?}", std::any::type_name::<($($T,)*)>());
            }
        }
    }
}

macro_rules! impl_tuples {
    ($T:ident) => { };
    ($Head:ident, $($Tail:ident),+) => {
        impl_tuple!($Head, $($Tail),+);
        impl_tuples!($($Tail),+);
    }
}

impl_tuples!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
