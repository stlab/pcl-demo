use crate::document::*;
use svg::node::element::Path;
use svg::node::element::path::Data;
use svg::node::element::Circle;

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


    let circle = Circle::new()
    .set("cx", 35)
    .set("cy", 35)
    .set("r", 30)
    .set("fill", "none")
    .set("stroke", "black")
    .set("stroke-width", 3);

    let r = Document::new()
        .set("viewBox", (0, 0, 70, 70))
        .add(path)
        .add(circle);
//    svg::save("image.svg", &r).unwrap();
    r
}
