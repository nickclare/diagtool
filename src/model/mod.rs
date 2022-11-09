//! Internal representation of a diagram/graph being parsed/rendered. Note, this doesn't include the AST for the DSL,
//! which as a first stage after the front-end will be converted into this model.
//!
//! It does however contain data structures for the various stages of the rendering/compilation/solving/etc. An initial version
//! is created by the frontend, and then augmented as the process proceeds.
//!
//!  (Note: how this actually will get represented as a rust data structure isn't something I have figured out :) )

mod newmodel;
pub mod visual;

use std::{
    any::TypeId,
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    rc::Rc,
};

pub type DiagRef = Rc<RefCell<Diagram>>;

pub struct Diagram {
    next_idx: usize,
    pub(crate) nodes: BTreeMap<NodeIdx, DiagramNode>,
}

impl Diagram {
    pub fn new() -> DiagRef {
        Rc::new(RefCell::new(Self {
            next_idx: 0,
            nodes: BTreeMap::new(),
        }))
    }

    pub fn get(&self, idx: NodeIdx) -> Option<&DiagramNode> {
        self.nodes.get(&idx)
    }

    pub fn get_mut(&mut self, idx: NodeIdx) -> Option<&mut DiagramNode> {
        self.nodes.get_mut(&idx)
    }

    pub fn iter(&self) -> impl Iterator<Item = &DiagramNode> {
        self.nodes.values()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct NodeIdx(pub(crate) usize);

impl From<usize> for NodeIdx {
    fn from(v: usize) -> Self {
        NodeIdx(v)
    }
}
impl From<NodeIdx> for usize {
    fn from(v: NodeIdx) -> Self {
        v.0
    }
}

pub type NodeData = HashMap<TypeId, Box<dyn Data>>;

#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
    /// Root of the object graph.
    Root,
    /// Denotes a box that can get rendered, depending on `BoxData`.
    Box,
    Text,
    Connector,
    Line,
}

pub struct DiagramNode {
    diagram: Rc<RefCell<Diagram>>,
    pub id: NodeIdx,
    pub ty: NodeType,
    data: NodeData,
    children: Vec<NodeIdx>,
}

pub trait Data: std::fmt::Debug {}

impl DiagramNode {
    pub fn create(diagram: DiagRef, ty: NodeType) -> NodeIdx {
        let mut d = diagram.as_ref().borrow_mut();
        let idx = d.next_idx.into();
        d.next_idx += 1;
        let node = Self {
            diagram: Rc::clone(&diagram),
            ty,
            id: idx,
            data: HashMap::new(),
            children: Vec::new(),
        };
        d.nodes.insert(idx, node);
        idx
    }

    pub fn insert<T: Data + 'static>(&mut self, data: T) -> Option<&T> {
        let key = TypeId::of::<T>();
        self.data.insert(key, Box::new(data)).map(|b| unsafe {
            // SAFETY: we only ever put a T into the box (with key T), so converting it back to a &T is okay.
            let p = &*b as *const _;
            &*(p as *const T)
        })
    }

    pub fn get<T: Data + 'static>(&self) -> Option<&T> {
        let key = TypeId::of::<T>();
        self.data.get(&key).map(|b| unsafe {
            // SAFETY: we only ever put a T into the box (with key T), so converting it back to a &T is okay.
            let p = &**b as *const _;
            &*(p as *const T)
        })
    }

    /// Make `child` a child node of `self`.
    pub fn link_child(&mut self, child: NodeIdx) -> bool {
        let diagram = self.diagram.as_ref().borrow();
        if diagram.nodes.contains_key(&child) {
            self.children.push(child);
            true
        } else {
            // Couldn't create link, no such child element exists.
            false
        }
    }

    pub fn unlink_child(&mut self, child: NodeIdx) -> bool {
        if let Some(idx) = self.children.iter().position(|e| *e == child) {
            self.children.remove(idx);
            true
        } else {
            false
        }
    }
}
