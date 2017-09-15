
use std::rc::Rc;
use Seed;

pub fn double_it(n:u32) -> u32 {
    n * 2
}

//pub type BoxedF<A, B> = Box<Fn(A) -> B>;

pub type Gen<'a, T> = Rc<Box<Fn(Seed) -> T + 'a>>;

pub fn gen<'a, F, T>(f: F) -> Gen<'a, T> where F: Fn(Seed) -> T + 'a {
    let boxed = Box::new(f);
    Rc::new(boxed)
}

pub fn ret<'a, T>(t:T) -> Gen<'a, T> where T : 'a + Clone {
    gen(move|_| t.clone() )
}

pub fn map<'a, F, A, B>(a: &Gen<'a, A>, f: F) -> Gen<'a, B> where F : Fn(A) -> B + 'a, A: 'a {
    let ac = a.clone();
    gen::<_, B>(move |seed| {
        let av = ac(seed);
        f(av)
    })
}

pub fn flat_map<'a, F, A, B>(a:&Gen<'a, A>, f: F) -> Gen<'a, B> where F : Fn(A) -> Gen<'a, B> + 'a, A: 'a {
    let ac = a.clone();
    gen::<_, B>(move |seed| {
        let (seed_a, seed_b) = seed.split();
        let out_a = ac(seed_a);
        let gen_b = f(out_a);
        gen_b(seed_b)
    })
}

pub fn ap<'a, A, B>(f: &'a Gen<Box<Fn(A) -> B>>, a: &Gen<'a, A>) -> Gen<'a, B> where A: 'a, B: Clone {
    let ac = a.clone();
    flat_map(f, move |fnc| {
        map(&ac, move |av| {
            let v = fnc(av);
            v.clone()
        })
    })
}

pub fn apb<'a, A, B, F>(f: &'a Gen<'a, F>, a: &Gen<'a, A>) -> Gen<'a, B> where F: Fn(A) -> B, A: 'a, B: Clone {
    let ac = a.clone();
    flat_map(f, move |fnc| {
        map(&ac, move |av| {
            let v = fnc(av);
            v.clone()
        })
    })
}
