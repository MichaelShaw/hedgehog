//use std::rc::Rc;
use std::ops::Deref;

#[derive(Clone, Debug)]
pub struct Tree<T> {
    pub value: T,
    pub children: Vec<Tree<T>>,
}

use std::fmt::Debug;

impl<A> Tree<A> where A : Debug {
    pub fn len(&self) -> usize {
        1 + self.children.iter().fold(0, |accu, ele| accu + ele.len())
    }

    pub fn flat_map<F, B>(&self, f: &F) -> Tree<B> where F : Fn(&A) -> Tree<B> {
        let mut new_tree = f(&self.value);

        let mut new_children : Vec<Tree<B>> = self.children.iter().map(|c| {
            c.flat_map(f)
        }).collect();

        new_children.append(&mut new_tree.children);

        Tree {
            value: new_tree.value,
            children: new_children,
        }
    }

    #[recursion_limit="100"]
    pub fn map<F, B>(&self, f: &F) -> Tree<B> where F : Fn(&A) -> B {
        println!("my value -> {:?} children count -> {:?}", self.value, self.children.len());
        let nv = f(&self.value);

        let mapped_children : Vec<Tree<B>> = self.children.iter().map(|child| {
            child.map(f)
        } ).collect();

        Tree {
            value: nv,
            children: mapped_children, // mapped_children,
        }
    }

    pub fn ret(t:A) -> Tree<A> {
        Tree {
            value: t,
            children: vec![]
        }
    }
}

