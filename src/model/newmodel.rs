use std::{
    cell::{Ref, RefCell, RefMut},
    collections::{btree_map::Values, BTreeMap},
    rc::Rc,
};

use super::{DiagramNode, NodeIdx};

#[derive(Clone)]
pub struct Diagram {
    inner: Rc<RefCell<Inner>>,
}

impl Diagram {
    pub fn new() -> Self {
        let inner = Rc::new(RefCell::new(Inner {
            next_idx: 0,
            nodes: BTreeMap::new(),
        }));

        Self { inner }
    }

    pub fn get(&self, idx: NodeIdx) -> Option<Ref<DiagramNode>> {
        let inner = self.inner.borrow();
        Ref::filter_map(inner, |n| n.nodes.get(&idx)).ok()
    }

    pub fn get_mut(&mut self, idx: NodeIdx) -> Option<RefMut<DiagramNode>> {
        let inner = self.inner.borrow_mut();
        RefMut::filter_map(inner, |n| n.nodes.get_mut(&idx)).ok()
    }
}

struct DiagramIter<'a> {
    iterator: Values<'a, NodeIdx, DiagramNode>,
}

impl<'a> Iterator for DiagramIter<'a> {
    type Item = &'a DiagramNode;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
    }
}

struct Inner {
    next_idx: usize,
    nodes: BTreeMap<NodeIdx, DiagramNode>,
}
