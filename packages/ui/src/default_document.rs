use crate::document::*;
use svg::node::element::Path;
use svg::node::element::path::Data;

pub fn default_document() -> Document {
    let square = Data::new()
        .move_to((10, 10))
        .line_by((0, 50))
        .line_by((50, 0))
        .line_by((0, -50))
        .close();

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 3)
        .set("d", square);

    Document::new()
        .set("viewBox", (0, 0, 70, 70))
        .add(path)
}
