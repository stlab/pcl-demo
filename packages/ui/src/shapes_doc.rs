use crate::shapes::{Color, Geometry, Shape, Style};
use std::collections::HashMap;
use std::vec::Vec;

pub type ShapeId = usize;

#[derive(PartialEq, Clone)]
pub struct Document {
    shapes: HashMap<ShapeId, Shape>,
    sequence: Vec<ShapeId>,
    next_shape_id: usize,
}

impl Document {
    pub fn new(elements: Vec<(ShapeId, Shape)>) -> Self {
        let mut sequence = Vec::new();
        let mut shapes = HashMap::new();
        let mut next_shape_id: usize = 0;
        for (shape_id, shape) in elements {
            sequence.push(shape_id);
            shapes.insert(shape_id, shape);
            if next_shape_id <= shape_id {
                next_shape_id = shape_id + 1;
            }
        }
        Self {
            sequence,
            shapes,
            next_shape_id,
        }
    }

    pub fn new_from_shapes(shapes: &Vec<Shape>) -> Self {
        let shapes_with_ids = shapes
            .iter()
            .map(|shape| shape.clone())
            .enumerate()
            .collect();
        Self::new(shapes_with_ids)
    }

    pub fn default() -> Self {
        Self::new_from_shapes(&vec![
            Shape::new(
                Geometry::circle(100.0, 150.0, 80.0),
                Style::new(Color::Blue),
            ),
            Shape::new(
                Geometry::circle(120.0, 120.0, 100.0),
                Style::new(Color::Red),
            ),
            Shape::new(
                Geometry::circle(200.0, 90.0, 70.0),
                Style::new(Color::Indigo),
            ),
            Shape::new(
                Geometry::rectangle(250.0, 80.0, 200.0, 20.0),
                Style::new(Color::Violet),
            ),
            Shape::new(
                Geometry::rectangle(0.0, 0.0, 40.0, 40.0),
                Style::new(Color::Black),
            ),
            Shape::new(
                Geometry::rectangle(40.0, 80.0, 40.0, 40.0),
                Style::new(Color::Green),
            ),
        ])
    }

    /* FIXME: It would be better to access this through an iterator on the shapes but
    the type system is fighting me on that. */

    pub fn get_sequence(&self) -> &Vec<ShapeId> {
        &self.sequence
    }

    pub fn get_shape_by_id(&self, shape_id: ShapeId) -> Option<&Shape> {
        self.shapes.get(&shape_id)
    }

    pub fn generate_shape_id(&mut self) -> ShapeId {
        let id = self.next_shape_id;
        self.next_shape_id = id + 1;
        id
    }

    pub fn upsert_shape_with_id(&mut self, shape_id: ShapeId, shape: Shape) {
        if !self.sequence.contains(&shape_id) {
            self.sequence.push(shape_id)
        }
        if self.next_shape_id <= shape_id {
            self.next_shape_id = shape_id + 1
        }
        self.shapes.insert(shape_id, shape);
    }

    pub fn delete_shape_with_id(&mut self, shape_id: ShapeId) {
        if let Some(idx) = self.sequence.iter().position(|&seq_id| seq_id == shape_id) {
            self.sequence.remove(idx);
        }
        self.shapes.remove(&shape_id);
    }

    pub fn update_geometry_for_shape_id(&mut self, shape_id: &ShapeId, new_geometry: Geometry) {
        self.shapes
            .entry(*shape_id)
            .and_modify(|shape| shape.geometry = new_geometry);
    }

    pub fn move_shape_with_id_to_front(&mut self, shape_id: ShapeId) {
        if let Some(idx) = self.sequence.iter().position(|seq_id| *seq_id == shape_id) {
            if idx != self.sequence.len() - 1 {
                self.sequence.remove(idx);
                self.sequence.push(shape_id);
            }
        }
    }
}
