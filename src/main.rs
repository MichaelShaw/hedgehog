extern crate hedgehog;

use hedgehog::messy::{Gen, self};
use hedgehog::Seed;

pub fn main() {
    let six = messy::double_it(3);
    println!("n -> {:?}", six);

    let (seed, next_seed) = Seed::new_unseeded().split();
    let n = seed.u32();

    println!("original seed -> {:?}", seed);
    println!("n -> {:?}, new seed -> {:?}", n, next_seed);

    let bullshit : Box<Fn(u32) -> u32> = Box::new(|n| n);

    let gen_u32 : Gen<u32> = messy::create_gen(|seed| seed.u32());

    let gen_half: Gen<u32> = messy::map(&gen_u32, |n| n / 2);

    let crap = gen_u32(next_seed);
    println!("crap -> {:?}", crap);
    let half_crap = gen_half(next_seed);
    println!("halved crap -> {:?}", half_crap);
}