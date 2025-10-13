// We want a better color model eventually, but an enumeration of fixed colors
// will do for now.

#[derive(PartialEq, Clone)]
pub enum Color {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Indigo,
    Violet,
    White,
    Black,
}

impl Color {
    // Cycle through the colors finding the next in the enumeration.
    // Since the order in the enumeration is arbitrary -- with respect
    // to black and white at least -- the cycling behavior is
    // arbitrary.
    pub fn advance(&mut self) {
        *self = match self {
            Color::Red => Color::Orange,
            Color::Orange => Color::Yellow,
            Color::Yellow => Color::Green,
            Color::Green => Color::Blue,
            Color::Blue => Color::Indigo,
            Color::Indigo => Color::Violet,
            Color::Violet => Color::Black,
            Color::Black => Color::White,
            Color::White => Color::Red,
        };
    }
}

// A shape has geometric information and style information.

#[derive(PartialEq, Clone)]
pub struct Shape {
    pub geometry: Geometry,
    pub style: Style,
}

impl Shape {
    pub fn new(geometry: Geometry, style: Style) -> Shape {
        Shape { geometry, style }
    }
}

// Styles contain a fill color.

#[derive(PartialEq, Clone)]
pub struct Style {
    pub fill: Color,
}

impl Style {
    pub fn new(fill: Color) -> Style {
        Style { fill }
    }
}

// We use xy pairs for much of our geometry.

#[derive(PartialEq, Clone)]
pub struct XYPoint {
    pub x: f64,
    pub y: f64,
}

impl XYPoint {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn add(&self, other: &XYPoint) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }

    pub fn subtract(&self, other: &XYPoint) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

// Geometry can take multiple forms. For now, it just contains rectangles
// and circles.

#[derive(PartialEq, Clone)]
pub enum Geometry {
    Rectangle { top_left: XYPoint, size: XYPoint },
    Circle { center: XYPoint, radius: f64 },
}

impl Geometry {
    pub fn rectangle(left: f64, top: f64, width: f64, height: f64) -> Self {
        Geometry::Rectangle {
            top_left: XYPoint::new(left, top),
            size: XYPoint::new(width, height),
        }
    }

    pub fn circle(cx: f64, cy: f64, radius: f64) -> Self {
        Geometry::Circle {
            center: XYPoint::new(cx, cy),
            radius,
        }
    }

    pub fn offset_by(&self, offset: &XYPoint) -> Geometry {
        match self {
            Geometry::Rectangle { top_left, size } => Geometry::Rectangle {
                top_left: top_left.add(offset),
                size: size.clone(),
            },
            Geometry::Circle { center, radius } => Geometry::Circle {
                center: center.add(offset),
                radius: *radius,
            },
        }
    }
}
