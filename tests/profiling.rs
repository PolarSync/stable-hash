mod common;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use common::*;
use firestorm::profile_fn;
use stable_hash::utils::AsBytes;
use stable_hash::*;

#[test]
#[ignore = "benchmark"]
fn compare() {
    let mut data = HashMap::new();

    data.insert("abc", 100u64);
    data.insert("abcdef", 100u64);
    data.insert("abcdefged", 0u64);
    data.insert("abcdefgedaekllw", 50000u64);
    data.insert("acfaek", 50000u64);
    data.insert("aek", 511110000u64);

    fn profile(value: &impl StableHash) {
        profile_fn!(profile);
        fast_stable_hash(value);
        crypto_stable_hash(value);
    }

    if firestorm::enabled() {
        firestorm::bench("./firestorm", || profile(&data)).unwrap();
    }
}

use rand::{thread_rng, Rng, RngCore};
trait R {
    fn rand() -> Self;
}

impl R for i32 {
    fn rand() -> Self {
        thread_rng().gen()
    }
}

impl R for usize {
    fn rand() -> Self {
        let num: u32 = thread_rng().gen();
        (num % 45) as usize
    }
}

impl<T> R for Vec<T>
where
    T: R,
{
    fn rand() -> Self {
        let count = R::rand();
        let mut v = Vec::with_capacity(count);
        for _ in 0..count {
            v.push(R::rand());
        }
        v
    }
}

impl<K, V> R for HashMap<K, V>
where
    K: R + Hash + Eq,
    V: R,
{
    fn rand() -> Self {
        let count = R::rand();
        let mut h = HashMap::with_capacity(count);
        for _ in 0..count {
            let k = R::rand();
            let v = R::rand();
            h.insert(k, v);
        }
        h
    }
}

impl<T> R for HashSet<T>
where
    T: R + Hash + Eq,
{
    fn rand() -> Self {
        let count = R::rand();
        let mut h = HashSet::with_capacity(count);
        for _ in 0..count {
            let t = R::rand();
            h.insert(t);
        }
        h
    }
}

impl R for u8 {
    fn rand() -> Self {
        thread_rng().gen()
    }
}

impl R for String {
    fn rand() -> Self {
        loop {
            let bytes = R::rand();
            if let Ok(s) = String::from_utf8(bytes) {
                return s;
            }
        }
    }
}

impl R for bool {
    fn rand() -> Self {
        thread_rng().gen()
    }
}

impl R for [u8; 32] {
    fn rand() -> Self {
        let mut value = Self::default();
        thread_rng().fill_bytes(&mut value);
        value
    }
}

impl R for Value {
    fn rand() -> Self {
        let d: u32 = thread_rng().gen();
        match d % 5 {
            0 => Value::Null,
            1 => Value::Number(R::rand()),
            2 => Value::String(R::rand()),
            3 => Value::Bool(R::rand()),
            4 => Value::Array(R::rand()),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Value {
    Null,
    Number(i32),
    String(String),
    Bool(bool),
    Array([u8; 32]),
}

// See also d3ba3adc-6e9b-4586-a7e7-6b542df39462
impl StableHash for Value {
    fn stable_hash<H: StableHasher>(&self, field_address: H::Addr, state: &mut H) {
        let variant = match self {
            Self::Null => return,
            Self::Number(n) => {
                n.stable_hash(field_address.child(0), state);
                1
            }
            Self::String(n) => {
                n.stable_hash(field_address.child(0), state);
                2
            }
            Self::Bool(n) => {
                n.stable_hash(field_address.child(0), state);
                3
            }
            Self::Array(n) => {
                AsBytes(n).stable_hash(field_address.child(0), state);
                4
            }
        };
        // Alternatively, variant.stable_hash(field_address, state).
        // But, this is slightly faster at the expense of interacting
        // with a more low-level API that is easier to screw up.
        state.write(field_address, &[variant]);
    }
}

#[derive(Debug)]
struct C {
    s: HashMap<String, Value>,
    n: i32,
}

impl_stable_hash!(C { s, n });

impl R for C {
    fn rand() -> Self {
        Self {
            s: R::rand(),
            n: R::rand(),
        }
    }
}

#[derive(Debug)]
struct A {
    v1: Vec<B>,
    v2: Vec<B>,
    v3: Vec<B>,
}

impl_stable_hash!(A { v1, v2, v3 });

impl R for A {
    fn rand() -> Self {
        Self {
            v1: R::rand(),
            v2: R::rand(),
            v3: R::rand(),
        }
    }
}

#[derive(Debug)]
struct B {
    a: u8,
    c: HashMap<String, C>,
}

impl_stable_hash!(B { a, c });

impl R for B {
    fn rand() -> Self {
        Self {
            a: R::rand(),
            c: R::rand(),
        }
    }
}
