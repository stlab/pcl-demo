use crate::shapes::{Color, Geometry, Shape, Style, XYPoint};
use crate::shapes_doc::{Document, ShapeId};
use dioxus::prelude::*;
use std::rc::Rc;

const SHAPES_UI_CSS: Asset = asset!("/assets/styling/shapes_ui.css");

// APP_STATE contains properties that are global across the document display.

static APP_STATE: GlobalSignal<AppState> = Global::new(|| AppState::default());

// For drawing new shapes, our AppState contains the fill color to use.

struct AppState {
    fill_color: Color,
}

impl AppState {
    fn default() -> Self {
        Self {
            fill_color: Color::Red,
        }
    }

    fn fill_color(&self) -> Color {
        self.fill_color.clone()
    }

    // Because we have not yet bothered with UI for setting the fill
    // color, we provide a way to get the current fill color and then
    // advance it to the next color.

    fn get_fill_color_and_advance(&mut self) -> Color {
        let old_color = self.fill_color.clone();
        self.fill_color.advance();
        old_color
    }

    // We can also get the next fill color and advance while also
    // skipping white in case we don't want to draw white shapes.

    fn get_fill_color_and_advance_skipping_white(&mut self) -> Color {
        let result_color = self.get_fill_color_and_advance();
        if result_color == Color::White {
            self.get_fill_color_and_advance_skipping_white()
        } else {
            result_color
        }
    }
}

// Mouse move trackers can respond by asking to continue or to announcing
// that they are done.

enum TrackerNext {
    Continue,
    Done,
}

// A tracker handles the mouse events while tracker.

trait Tracker {
    fn track_mouse_move(&self, evt: &MouseEvent) -> TrackerNext;
    fn track_mouse_up(&self, evt: &MouseEvent);
}

// CANVAS_TRACKER contains the current tracker if any. SvgCanvasDiv will route
// mouse moved and mouse up messages to this tracker.

static CANVAS_TRACKER: GlobalSignal<Option<Rc<dyn Tracker>>> = Global::new(|| None);

// The primary job of the SvgCanvasDiv element is to handle mouse move
// and mouse up phases of tracking based on the current values of CANVAS_TRACKER.

#[component]
pub fn SvgCanvasDiv() -> Element {
    let opt_mouse_move_tracker = CANVAS_TRACKER();
    let opt_mouse_up_tracker = opt_mouse_move_tracker.clone();

    let mouse_move_handler = move |evt: MouseEvent| {
        if let Some(tracker) = &opt_mouse_move_tracker {
            evt.stop_propagation();
            evt.prevent_default();
            match tracker.track_mouse_move(&evt) {
                TrackerNext::Continue => {}
                TrackerNext::Done => {
                    *CANVAS_TRACKER.write() = None;
                }
            };
        }
    };

    let mouse_up_handler = move |evt: MouseEvent| {
        if let Some(tracker) = &opt_mouse_up_tracker {
            evt.stop_propagation();
            evt.prevent_default();
            tracker.track_mouse_up(&evt);
            *CANVAS_TRACKER.write() = None;
        }
    };

    rsx! {
        document::Link { rel: "stylesheet", href: SHAPES_UI_CSS }
        div {
            id: "svg_canvas_div",
            onmousemove: mouse_move_handler,
            onmouseup: mouse_up_handler,
            SvgCanvas{}
        }
    }
}

// The SvgCancas element actually displays the background and the shapes.

#[component]
fn SvgCanvas() -> Element {
    rsx! {
        svg {
            width: "100%",
            height: "1500px",
            preserve_aspect_ratio: "none",
            // view_box: "0 0 500 500",
            Background{},
            RenderedShapes{}
        }
    }
}

// DOC provides the current state of the document

static DOC: GlobalSignal<Document> = Global::new(|| Document::new_demo());

// Given a pair of coordinates, find the mimimum coordinate and the non-negative span
// to the other coordinate.

fn to_min_span(x1: f64, x2: f64) -> (f64, f64) {
    if x1 < x2 {
        (x1, x2 - x1)
    } else {
        (x2, x1 - x2)
    }
}

// We will do our tacking in terms of page coordinates.

fn xy_point_from_page_coordinates(mouse_event: &MouseEvent) -> XYPoint {
    XYPoint::new(
        mouse_event.data.page_coordinates().x,
        mouse_event.data.page_coordinates().y,
    )
}

// Track a new rectangle with a given shape id and style

struct NewRectTracker {
    mouse_down: XYPoint,
    shape_id: ShapeId,
    style: Style,
}

impl NewRectTracker {
    fn new(mouse_down: &MouseEvent, shape_id: ShapeId, style: Style) -> Self {
        Self {
            mouse_down: xy_point_from_page_coordinates(mouse_down),
            shape_id,
            style,
        }
    }

    fn post_shape_for_event(&self, mouse: &MouseEvent) {
        let event_coords = xy_point_from_page_coordinates(mouse);
        let (min_x, span_x) = to_min_span(self.mouse_down.x, event_coords.x);
        let (min_y, span_y) = to_min_span(self.mouse_down.y, event_coords.y);
        // If the result is non-empty, upsert the shape.
        if 0.0 < span_x && 0.0 < span_y {
            let geometry = Geometry::Rectangle {
                top_left: XYPoint::new(min_x, min_y),
                size: XYPoint::new(span_x, span_y),
            };
            DOC.write().upsert_shape_with_id(
                self.shape_id,
                Shape {
                    geometry,
                    style: self.style.clone(),
                },
            )
        // If empty, delete the shape.
        } else {
            DOC.write().delete_shape_with_id(self.shape_id)
        }
    }
}

impl Tracker for NewRectTracker {
    fn track_mouse_move(&self, evt: &MouseEvent) -> TrackerNext {
        self.post_shape_for_event(evt);
        TrackerNext::Continue
    }
    fn track_mouse_up(&self, evt: &MouseEvent) {
        self.post_shape_for_event(evt)
    }
}

// We have a component to draw the background for the shapes. It's
// most important job is handling clicks in the backgrouns.

#[component]
fn Background() -> Element {
    // Mouse down on the canvas tracks out a rectangle. We use the
    // next color in sequence, skipping white. (See Color::advance.)
    // FIXME: Obviously, it would be better to have a color picker in
    // the App UI but that would be more UI than we need for testing.
    let canvas_mouse_down = move |evt| {
        let shape_id = DOC.write().generate_shape_id();
        let fill_color = APP_STATE
            .write()
            .get_fill_color_and_advance_skipping_white();
        let style = Style::new(fill_color);
        *CANVAS_TRACKER.write() = Some(Rc::new(NewRectTracker::new(&evt, shape_id, style)))
    };

    rsx! {
        rect {
            id: "background",
            x: "0",
            y: "0",
            width: "100%",
            height: "100%",
            fill: "white",
            onmousedown: canvas_mouse_down
        }
    }
}

// We have a component to render the shapes from bottom to top.

#[component]
fn RenderedShapes() -> Element {
    let doc: &Document = &*DOC.read();
    let shape_id_shapes_iter = doc.shape_id_shapes_iter();
    let rendered_shapes_iter =
        shape_id_shapes_iter.map(|(shape_id, shape)| render_shape(shape_id, shape));
    rsx! {
        for rendered_shape in rendered_shapes_iter {
            { rendered_shape }
        }
    }
}

/* Shape dragging */

struct ShapeDragTracker {
    mouse_down: XYPoint,
    shape_id: ShapeId,
    initial_geometry: Geometry,
}

impl ShapeDragTracker {
    fn new(mouse_down: &MouseEvent, shape_id: ShapeId, initial_geometry: &Geometry) -> Self {
        Self {
            mouse_down: xy_point_from_page_coordinates(mouse_down),
            shape_id,
            initial_geometry: initial_geometry.clone(),
        }
    }

    fn update_shape_for_drag(&self, mouse: &MouseEvent) {
        let event_coords = xy_point_from_page_coordinates(mouse);
        let delta = event_coords.subtract(&self.mouse_down);
        let new_geometry = self.initial_geometry.offset_by(&delta);
        DOC.write()
            .update_geometry_for_shape_id(&self.shape_id, new_geometry)
    }
}

impl Tracker for ShapeDragTracker {
    fn track_mouse_move(&self, evt: &MouseEvent) -> TrackerNext {
        self.update_shape_for_drag(evt);
        TrackerNext::Continue
    }
    fn track_mouse_up(&self, evt: &MouseEvent) {
        self.update_shape_for_drag(evt)
    }
}

/* SVG generation  */

fn svg_color(color: &Color) -> String {
    match color {
        Color::Red => "red".to_string(),
        Color::Orange => "orange".to_string(),
        Color::Yellow => "yellow".to_string(),
        Color::Green => "green".to_string(),
        Color::Blue => "blue".to_string(),
        Color::Indigo => "indigo".to_string(),
        Color::Violet => "violet".to_string(),
        Color::White => "white".to_string(),
        Color::Black => "black".to_string(),
    }
}

// Render a shape to SVG and attach a mouse down handler that
// initiates dragging.

fn render_shape(shape_id: ShapeId, shape: &Shape) -> Element {
    let id_string = format!("shape_{}", shape_id);
    let fill_color = svg_color(&shape.style.fill);
    let initial_geometry = shape.geometry.clone();
    let shape_mouse_down = move |evt| {
        *CANVAS_TRACKER.write() = Some(Rc::new(ShapeDragTracker::new(
            &evt,
            shape_id,
            &initial_geometry,
        )))
    };
    /*
    let move_shape_to_top = move |evt| DOC.write().move_shape_with_id_to_top(shape_id);
    */
    match &shape.geometry {
        Geometry::Circle { center, radius } => rsx! {
            circle {
                id: id_string,
                cx: center.x,
                cy: center.y,
                r: *radius,
                fill: fill_color,
                onmousedown: shape_mouse_down,
            }
        },
        Geometry::Rectangle { top_left, size } => rsx! {
            rect {
                id: id_string,
                x: top_left.x,
                y: top_left.y,
                width: size.x,
                height: size.y,
                fill: fill_color,
                onmousedown: shape_mouse_down,
            }
        },
    }
}
