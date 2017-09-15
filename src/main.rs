extern crate hedgehog;

use hedgehog::messy::*;
use hedgehog::Seed;

pub fn main() {

    let seed = Seed::new_unseeded();

    let gen_u32: Gen<u32> = gen(|seed| seed.u32());

    let gen_half: Gen<u32> = map(&gen_u32, |n| n / 2);

    let gen_zero = ret(0);

    let gen_pair = flat_map(&gen_u32, |a| {
        flat_map(&gen_u32, move|b| {
            ret((a, b))
        })
    });

    println!("zero -> {:?}", gen_zero(seed));
    println!("n -> {:?}", gen_u32(seed));
    println!("half  -> {:?}", gen_half(seed));
    println!("pair -> {:?}", gen_pair(seed));

    let gen_to_point : Gen<_> = gen(|seed| {
       |(a, b): (u32,u32)| Point { x: a, y: b }
    });

    let gen_point = apb(&gen_to_point, &gen_pair);

    println!("point -> {:?}", gen_point(seed));
}

#[derive(Copy, Clone, Debug)]
pub struct Point { x: u32, y: u32 }





