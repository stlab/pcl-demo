use crate::shapes::{Color, Geometry, Shape, Style};
use std::collections::HashMap;
use std::vec::Vec;

// ShapeId provides a reference to shapes across changes in the document.
// Most references at the document level should be to ShapeId rather than
// the shape itself since we will be updating the shape.

// FIXME? These could conceivably be UUIDs -- better for collaboration
// but more difficult to generate (and, if one is squeamish about
// randomization, more difficult to guarantee unique).

pub type ShapeId = usize;

// Our document consists of a sequence of shape ids listing the shapes
// to render from bottom to top, a hash map of shapes keyed by shape ids,
// and the next shape id to generate which should be greater than all
// of the shape ids ever used or generated for this document.

#[derive(PartialEq, Clone)]
pub struct Document {
    shapes: HashMap<ShapeId, Shape>,
    sequence: Vec<ShapeId>,
    next_shape_id: usize,
}

pub enum DocError {
    DuplicateShapeId(ShapeId),
}

// Note: We need the lifetime for the shapes iterator

impl<'a> Document {
    // Create a new empty document is easy (and unlike other functions that
    // perform validation, does not fail).
    pub fn new_empty() -> Self {
        Self {
            sequence: Vec::new(),
            shapes: HashMap::new(),
            next_shape_id: 1,
        }
    }

    // The standard way to create a new document (if the empty document
    // is insufficient) is to take a vector of ShapeId/Shape pairs containing
    // the shapes to display from bottom to top. Duplicate shape id's will result
    // in an error.
    pub fn new_from_pairs(pairs: Vec<(ShapeId, Shape)>) -> Result<Self, DocError> {
        let mut doc = Self::new_empty();
        for (shape_id, shape) in pairs {
            // Prevent multiple uses of the same shape_id
            if doc.shapes.contains_key(&shape_id) {
                return Err(DocError::DuplicateShapeId(shape_id));
            }
            // Add the shape to the sequence
            doc.sequence.push(shape_id);
            // Add the shape to the dictionary
            doc.shapes.insert(shape_id, shape);
            // Make sure that next_shape_id is larger than any of these shapes
            if doc.next_shape_id <= shape_id {
                doc.next_shape_id = shape_id + 1;
            }
        }
        Ok(doc)
    }

    // Create a new document from a vector of shapes -- e.g., from
    // reading a file in some external format. The shapes will
    // be displayed in vector order from bottom to top.
    pub fn new_from_shapes(shapes: &Vec<Shape>) -> Self {
        let mut doc = Self::new_empty();
        for shape in shapes {
            let shape_id = doc.generate_shape_id();
            doc.upsert_shape_with_id(shape_id, shape.clone());
        }
        doc
    }

    // Generate a default document with some shapes for demo
    // purposes.
    pub fn new_demo() -> Self {
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

    // Get an iterator for the sequence of shape ids from bottom to top.

    pub fn shape_ids_iter(&self) -> std::slice::Iter<'_, ShapeId> {
        self.sequence.iter()
    }

    // Get an iterator for the sequence of ShapeId, Shape pairs from
    // bottom to top.

    pub fn shape_id_shapes_iter(
        &'a self,
    ) -> std::iter::FilterMap<
        std::slice::Iter<'a, ShapeId>,
        impl Fn(&'a ShapeId) -> Option<(ShapeId, &'a Shape)>,
    > {
        let to_opt_shape_id_shape = |shape_id: &ShapeId| {
            self.get_shape_by_id(*shape_id)
                .map(|shape| (*shape_id, shape))
        };
        self.shape_ids_iter().filter_map(to_opt_shape_id_shape)
    }

    // Get a shape if any with a particular id

    pub fn get_shape_by_id(&self, shape_id: ShapeId) -> Option<&Shape> {
        self.shapes.get(&shape_id)
    }

    // Generate the next unused (for this document) shape id

    pub fn generate_shape_id(&mut self) -> ShapeId {
        let id = self.next_shape_id;
        self.next_shape_id = id + 1;
        id
    }

    // Upsert a shape with an id into the document.
    // If this is an insert, the shape is added at the top.

    pub fn upsert_shape_with_id(&mut self, shape_id: ShapeId, shape: Shape) {
        // If the shape id is not listed in the sequence, we add it at the top.
        if !self.sequence.contains(&shape_id) {
            self.sequence.push(shape_id)
        }
        // Upsert into the shapes hash map.
        self.shapes.insert(shape_id, shape);
        // Make sure that next_shape_id is greater than all other
        // shape id's seen within the document.
        if self.next_shape_id <= shape_id {
            self.next_shape_id = shape_id + 1
        }
    }

    // Remove the shape with the given id from both the shapes sequence
    // and the shape definitions. If there is no shape with this id, then
    // the operation is a no-op.
    pub fn delete_shape_with_id(&mut self, shape_id: ShapeId) {
        if let Some(idx) = self.sequence.iter().position(|&seq_id| seq_id == shape_id) {
            self.sequence.remove(idx);
        }
        self.shapes.remove(&shape_id);
    }

    // If a shape with the given id exists, update its geometry with new geometry.
    // If there is no shape with this id, the operation is a no-op.
    pub fn update_geometry_for_shape_id(&mut self, shape_id: &ShapeId, new_geometry: Geometry) {
        self.shapes
            .entry(*shape_id)
            .and_modify(|shape| shape.geometry = new_geometry);
    }

    // If there is a shape with the given id, pull it to the top of the shapes
    // display sequence -- i.e., to the last position in the sequence.
    pub fn move_shape_with_id_to_top(&mut self, shape_id: ShapeId) {
        if let Some(idx) = self.sequence.iter().position(|seq_id| *seq_id == shape_id) {
            if idx != self.sequence.len() - 1 {
                self.sequence.remove(idx);
                self.sequence.push(shape_id);
            }
        }
    }
}
