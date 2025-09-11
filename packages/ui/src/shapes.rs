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

#[derive(PartialEq, Clone)]
pub struct Style {
    pub fill: Color,
}

impl Style {
    pub fn new(fill: Color) -> Style {
        Style { fill }
    }
}

#[derive(PartialEq, Clone)]
pub enum Geometry {
    Rectangle {
        left: f64,
        top: f64,
        width: f64,
        height: f64,
    },
    Circle {
        cx: f64,
        cy: f64,
        radius: f64,
    },
}

impl Geometry {
    pub fn rectangle(left: f64, top: f64, width: f64, height: f64) -> Self {
        Geometry::Rectangle {
            left,
            top,
            width,
            height,
        }
    }

    pub fn circle(cx: f64, cy: f64, radius: f64) -> Self {
        Geometry::Circle { cx, cy, radius }
    }

    pub fn offset_by(&self, dx: f64, dy: f64) -> Geometry {
        match self {
            Geometry::Rectangle {
                left,
                top,
                width,
                height,
            } => Geometry::Rectangle {
                left: left + dx,
                top: top + dy,
                width: *width,
                height: *height,
            },
            Geometry::Circle { cx, cy, radius } => Geometry::Circle {
                cx: cx + dx,
                cy: cy + dy,
                radius: *radius,
            },
        }
    }
}
