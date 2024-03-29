use stable_hash::prelude::*;
use stable_hash::utils::AsBytes;
mod common;

struct One<T0> {
    one: T0,
}

impl<T0: StableHash> StableHash for One<T0> {
    fn stable_hash<H: StableHasher>(&self, field_address: H::Addr, state: &mut H) {
        self.one.stable_hash(field_address.child(0), state);
    }
}

struct Two<T0, T1> {
    one: T0,
    two: T1,
}

impl<T0: StableHash, T1: StableHash> StableHash for Two<T0, T1> {
    fn stable_hash<H: StableHasher>(&self, field_address: H::Addr, state: &mut H) {
        self.one.stable_hash(field_address.child(0), state);
        self.two.stable_hash(field_address.child(1), state);
    }
}

#[test]
fn add_optional_field() {
    let one = One { one: 5u32 };
    let two = Two {
        one: 5u32,
        two: Option::<u32>::None,
    };
    equal!(102568403942768160221811810082933398928, "3428a4134bfdac56aa04614504705b0ffd1d48f27777b109a793e5a641324212"; one, two);
}

#[test]
fn add_default_field() {
    let one = One { one: "one" };
    let two = Two {
        one: "one",
        two: "",
    };
    equal!(237994494046445339248193596542695086083, "65bf96c193b5d365191b86da83097939ccd67ac226d9f3a3c991719e338de7ed"; one, two);
}

#[test]
fn add_non_default_field() {
    let one = One { one: "one" };
    let two = Two {
        one: "one",
        two: "two",
    };
    not_equal!(one, two);
}

#[test]
fn some_default_ne() {
    not_equal!(Some(0u32), Option::<u32>::None);
}

#[test]
fn empty_vec_is_default() {
    let one = One { one: true };
    let two = Two {
        one: true,
        two: Vec::<u32>::new(),
    };
    equal!(338065623630761276859032195206974584513, "db4657c873e33a60e581eb5458aba6c76f510e023872c76a3134608619342c59"; one, two);
}

#[test]
fn two_is_used() {
    let one = One { one: true };
    let two = Two {
        one: true,
        two: true,
    };
    not_equal!(one, two);
}

#[test]
fn omitted_defaults_dont_collide() {
    not_equal!(vec![1u32, 0u32, 2u32], vec![0u32, 1u32, 2u32]);
}

// See also 33a9b3bf-0d43-4fd0-a3ed-a77807505255
#[test]
fn last_default_does_not_collide() {
    not_equal!(vec![1u32, 2u32, 0u32], vec![1u32, 2u32]);
}

#[test]
fn as_bytes() {
    let v = vec![0u8];
    not_equal!(&v[..], AsBytes(&v[..]));

    let v = vec![1u8, 2u8];
    not_equal!(&v[..], AsBytes(&v[..]));
}

#[test]
fn numbers_through_vec() {
    equal!(
        99946908715907655724842348751554312075, "25dfaa9f92a3f2b05a1bdfbc66ec594c545dc39ebdb0e9ae769350ea1726e2b7";
        vec![1u32, 2u32],
        vec![1u16, 2u16]
    );
}
