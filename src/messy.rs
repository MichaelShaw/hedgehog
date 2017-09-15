
use std::rc::Rc;
use Seed;

pub fn double_it(n:u32) -> u32 {
    n * 2
}

//pub type Gen<T> = Option<T>;

pub type Gen<T> = Rc<Box<Fn(Seed) -> T>>;

pub fn create_gen<F, T>(f: F) -> Gen<T> where F: Fn(Seed) -> T + 'static {
    let boxed = Box::new(f);
    Rc::new(boxed)
}

pub fn map<F, A, B>(a: &Gen<A>, f: F) -> Gen<B> where F : Fn(A) -> B + 'static, A: 'static {
    let ac = a.clone();
    create_gen::<_, B>(move |seed| {
        let av = ac(seed);
        f(av)
    })
}

//pub fn flat_map<F, A, B>(a:Gen<A>, f: F) -> Gen<B> where F : Fn(A) -> Gen<B> + 'static {
//    let ac = a.clone();
//    create_gen::<_, B>(|seed| {
//        let (sa, sb) = seed.split();
//        let av = ac(sa);
//
//        let bg = f(av);
//
//    })
//}