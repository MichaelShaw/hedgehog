extern crate hedgehog;

use hedgehog::messy;

pub fn main() {
    let six = messy::double_it(3);
    println!("n -> {:?}", six);

    let seed = hedgehog::Seed::new_unseeded();

    let (n, new_seed) = seed.next_u32();

    println!("original seed -> {:?}", seed);
    println!("n -> {:?}, new seed -> {:?}", n, new_seed);
}