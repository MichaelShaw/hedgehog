use std::rc::Rc;

#[derive(Clone)]
pub struct Tree<T> {
    pub value: T,
    pub shrink: Rc<Box<Fn(&T) -> Vec<T>>>,
}

impl<T> Tree<T> {
    pub fn children(&self) -> Vec<Tree<T>> {
        let cs = (self.shrink)(&self.value);
        cs.into_iter().map(|v|
            Tree {
                value: v,
                shrink: self.shrink.clone(),
            }
        ).collect()
    }
}


pub fn map<F, A, B>(a: &Tree<A>, f: Box<Fn(A) -> B>) -> Tree<B> where A : Clone + 'static, B: 'static {
    let sc = a.shrink.clone();
    let oa = a.value.clone();

    Tree::<B> {
        value: f(a.value.clone()),
        shrink: Rc::new(Box::new(move |t| {
            sc(&oa).into_iter().map(|e| f(e)).collect()
        } ))
    }
}