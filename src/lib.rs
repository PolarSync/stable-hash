pub mod crypto;
mod impls;
mod macros;
pub mod prelude;
mod sequence_number;
mod stable_hash;
pub mod utils;

pub use crate::sequence_number::{SequenceNumber, SequenceNumberInt};
pub use crate::stable_hash::{StableHash, StableHasher, UnorderedAggregator};

#[derive(Debug)]
pub struct CallDepth(u32);

thread_local! {
  static DEPTH: core::cell::Cell<u32> = core::cell::Cell::new(0);
}

impl Drop for CallDepth {
    fn drop(&mut self) {
        DEPTH.with(|d| {
            d.set(self.0);
        })
    }
}

impl Default for CallDepth {
    fn default() -> Self {
        Self::new()
    }
}

impl CallDepth {
    pub fn new() -> Self {
        Self(DEPTH.with(|d| {
            let depth = d.get();
            d.set(depth + 1);
            depth
        }))
    }
}

impl core::fmt::Display for CallDepth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for _ in 0..self.0 {
            f.write_str("   ")?;
        }
        Ok(())
    }
}
