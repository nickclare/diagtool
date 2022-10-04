//! Internal representation of a diagram/graph being parsed/rendered. Note, this doesn't include the AST for the DSL,
//! which as a first stage after the front-end will be converted into this model.
//!
//! It does however contain data structures for the various stages of the rendering/compilation/solving/etc. An initial version
//! is created by the frontend, and then augmented as the process proceeds.
//!
//!  (Note: how this actually will get represented as a rust data structure isn't something I have figured out :) )

use std::{any::TypeId, collections::HashMap};

pub struct Diagram {
    pub(crate) nodes: Vec<DiagramNode>,
}

pub struct DiagramNode {
    data: HashMap<TypeId, Box<dyn Data>>,
}

pub trait Data: std::fmt::Debug {}

impl DiagramNode {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
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
}

impl Default for DiagramNode {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Copy, Default)]
pub struct ConstraintRange {
    min: Option<i64>,
    max: Option<i64>,
}

/// Data component for storing box constraints
#[derive(Clone, Debug, Default)]
pub(crate) struct ConstraintData {
    pub x: ConstraintRange,
    pub y: ConstraintRange,
    pub w: ConstraintRange,
    pub h: ConstraintRange,
}

impl Data for ConstraintData {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_insert_and_get_data() {
        let mut node = DiagramNode::new();
        let mut constraints = ConstraintData::default();
        constraints.x.min = Some(32);
        constraints.x.max = Some(44);
        node.insert(constraints);

        let constraints = node.get::<ConstraintData>().unwrap();
        assert_eq!(constraints.x.min, Some(32));
        assert_eq!(constraints.x.max, Some(44));
        assert_eq!(constraints.y.max, None);

        let mut new_constraints = constraints.clone();
        new_constraints.w.min = Some(66);
        let old_constraints = node
            .insert(new_constraints)
            .expect("should be an existing value for `ConstraintData`");
        assert_eq!(old_constraints.w.min, None);
        let constraints = node.get::<ConstraintData>().unwrap();
        assert_eq!(constraints.w.min, Some(66));
    }
}
