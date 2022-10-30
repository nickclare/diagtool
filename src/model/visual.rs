//! Data used for the visual representation of objects in the diagram.
//!

use super::Data;

#[derive(Debug, Clone)]
pub struct BoxData {
    pub stroke: Stroke,
    pub fill: Stroke,
}

impl Data for BoxData {}

#[derive(Debug, Clone)]
pub struct Stroke {
    width: u8,
    color: Color,
    // TODO: add support for non-solid lines
}

#[derive(Debug, Clone, Copy)]
pub struct Fill {
    color: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[derive(Debug, Clone)]
pub struct TextData {
    pub text: String,
}

impl Data for TextData {}

#[derive(Clone, Debug, Copy, Default)]
pub struct ConstraintRange {
    min: Option<i64>,
    max: Option<i64>,
}

/// Data component for storing box constraints
#[derive(Clone, Debug, Default)]
pub(crate) struct DimensionConstraintData {
    pub x: ConstraintRange,
    pub y: ConstraintRange,
    pub w: ConstraintRange,
    pub h: ConstraintRange,
}

impl Data for DimensionConstraintData {}

// Once the constraints have been solved, we should end up with fixed dimensions
#[derive(Debug, Clone, Default)]
pub(crate) struct SolvedDimensions {
    pub x: i64,
    pub y: i64,
    pub w: i64,
    pub h: i64,
}

impl Data for SolvedDimensions {}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[test]
    fn can_insert_and_get_data() {
        let diagram = Diagram::new();

        let node = DiagramNode::create(Rc::clone(&diagram), NodeType::Box);
        let mut constraints = DimensionConstraintData::default();
        constraints.x.min = Some(32);
        constraints.x.max = Some(44);
        let mut diagram = diagram.borrow_mut();
        let node = diagram.get_mut(node).unwrap();
        node.insert(constraints);

        let constraints = node.get::<DimensionConstraintData>().unwrap();
        assert_eq!(constraints.x.min, Some(32));
        assert_eq!(constraints.x.max, Some(44));
        assert_eq!(constraints.y.max, None);

        let mut new_constraints = constraints.clone();
        new_constraints.w.min = Some(66);
        let old_constraints = node
            .insert(new_constraints)
            .expect("should be an existing value for `ConstraintData`");
        assert_eq!(old_constraints.w.min, None);
        let constraints = node.get::<DimensionConstraintData>().unwrap();
        assert_eq!(constraints.w.min, Some(66));
    }
}
