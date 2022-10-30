//! Render output to SVG

use crate::model::visual::SolvedDimensions;
use crate::model::{self, *};
use crate::{Error, Result};

pub fn render(model: &Diagram) -> Result<svg::Document> {
    let mut document = svg::Document::new();

    // This is very temporary
    let box_nodes = model.iter().filter(|node| node.ty == NodeType::Box);
    for node in box_nodes {
        if let Some(dims) = node.get::<SolvedDimensions>() {
            let rect = svg::node::element::Rectangle::new()
                .set("x", dims.x)
                .set("y", dims.y)
                .set("width", dims.w)
                .set("height", dims.h);
            // TODO: color
            document = document.add(rect);
        } else {
            println!("[WARN] Box without solved dimensions: {}", node.id.0);
        }
    }

    Ok(document)
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::*;

    fn build_sample_diagram() -> Rc<RefCell<Diagram>> {
        let diagram = Diagram::new();
        {
            let b1 = DiagramNode::create(Rc::clone(&diagram), NodeType::Box);
            let mut d = diagram.borrow_mut();
            let b1 = d.get_mut(b1).unwrap();
            b1.insert(SolvedDimensions {
                x: 5,
                y: 10,
                w: 100,
                h: 50,
            });
        }

        diagram
    }

    #[test]
    fn test_simple_box() {
        let diagram = build_sample_diagram();
        let mut output = String::new();
        let result = render(&diagram.borrow()).unwrap();
        let output = result.to_string();
        println!("{}", output);
    }
}
