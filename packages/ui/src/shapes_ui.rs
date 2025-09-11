use crate::shapes::{Color, Geometry, Shape, Style};
use crate::shapes_doc::{Document, ShapeId};
use dioxus::prelude::*;
use std::rc::Rc;

const USE_MESSAGE_BOX: bool = false;

const SHAPES_UI_CSS: Asset = asset!("/assets/styling/shapes_ui.css");

static DOC: GlobalSignal<Document> = Global::new(|| Document::default());

enum TrackerNext {
    Continue,
    Done,
}

trait Tracker {
    fn track_mouse_move(&self, evt: &MouseEvent) -> TrackerNext;
    fn track_mouse_up(&self, evt: &MouseEvent);
}

static CANVAS_TRACKER: GlobalSignal<Option<Rc<dyn Tracker>>> = Global::new(|| None);

static MESSAGE_BOX: GlobalSignal<String> = Global::new(|| "".to_string());

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

    fn get_fill_color_and_advance(&mut self) -> Color {
        let old_color = self.fill_color.clone();
        self.fill_color.advance();
        old_color
    }
}

static APP_STATE: GlobalSignal<AppState> = Global::new(|| AppState::default());

#[component]
pub fn SvgCanvasDiv() -> Element {
    let opt_tracker = &*CANVAS_TRACKER.read();
    match opt_tracker {
        Some(tracker) => {
            let mouse_move_tracker = tracker.clone();
            let mouse_up_tracker = tracker.clone();
            rsx! {
                div {
                    id: "svg_canvas_div",
                    onmousemove: move |evt| {
                        evt.stop_propagation();
                        evt.prevent_default();
                        match mouse_move_tracker.track_mouse_move(&evt) {
                            TrackerNext::Continue => {},
                            TrackerNext::Done => { *CANVAS_TRACKER.write() = None; }
                        };
                    },
                    onmouseup: move |evt| {
                        evt.stop_propagation();
                        evt.prevent_default();
                        mouse_up_tracker.track_mouse_up(&evt);
                        *CANVAS_TRACKER.write() = None;
                    },
                    SvgCanvas{}
                }
            }
        }
        None => rsx! {
            div {
                id: "svg_canvas_div",
                onmousemove: move |evt| {
                    let coords = evt.page_coordinates();
                    let x = coords.x;
                    let y = coords.y;
                    *MESSAGE_BOX.write() = format!("Mouse: {}, {}", x, y );
                },
                document::Link { rel: "stylesheet", href: SHAPES_UI_CSS }
                SvgCanvas{}
                if USE_MESSAGE_BOX {
                    MessageBox{}
                }
            }
        },
    }
}

#[component]
fn MessageBox() -> Element {
    if USE_MESSAGE_BOX {
        let text: String = MESSAGE_BOX.read().clone();
        rsx! {
        div {
            position: "absolute",
            top: "0px",
            left: "0px",
            height: "20px",
            right: "0px",
            background_color: "blue",
            { text }
        } }
    } else {
        rsx! {
        div {
            position: "absolute",
            top: "0px",
            left: "0px",
            height: "20px",
            right: "0px",
            background_color: "blue"
        } }
    }
}

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

fn to_min_span(x1: f64, x2: f64) -> (f64, f64) {
    if x1 < x2 {
        (x1, x2 - x1)
    } else {
        (x2, x1 - x2)
    }
}

struct NewRectTracker {
    mouse_down_x: f64,
    mouse_down_y: f64,
    shape_id: ShapeId,
    style: Style,
}

impl NewRectTracker {
    fn new(mouse_down: MouseEvent, shape_id: ShapeId, style: Style) -> Self {
        let mouse_down_x = mouse_down.data.page_coordinates().x;
        let mouse_down_y = mouse_down.data.page_coordinates().y;
        Self {
            mouse_down_x,
            mouse_down_y,
            shape_id,
            style,
        }
    }

    fn post_shape_for_event(&self, mouse: &MouseEvent) {
        let event_x = mouse.data.page_coordinates().x;
        let event_y = mouse.data.page_coordinates().y;
        let (left, width) = to_min_span(self.mouse_down_x, event_x);
        let (top, height) = to_min_span(self.mouse_down_y, event_y);
        if USE_MESSAGE_BOX {
            *MESSAGE_BOX.write() = format!("Rectangle: {}, {}, {}, {}", left, top, width, height);
        }
        if 0.0 < width && 0.0 < height {
            let geometry = Geometry::Rectangle {
                top,
                left,
                height,
                width,
            };
            DOC.write().upsert_shape_with_id(
                self.shape_id,
                Shape {
                    geometry,
                    style: self.style.clone(),
                },
            )
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

#[component]
fn Background() -> Element {
    let canvas_mouse_down = move |evt| {
        let shape_id = DOC.write().generate_shape_id();
        let mut fill_color = APP_STATE.write().get_fill_color_and_advance();
        if fill_color == Color::White {
            fill_color = APP_STATE.write().get_fill_color_and_advance();
        }
        let style = Style::new(fill_color);
        *CANVAS_TRACKER.write() = Some(Rc::new(NewRectTracker::new(evt, shape_id, style)))
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

#[component]
fn RenderedShapes() -> Element {
    let doc: &Document = &*DOC.read();
    // FIXME: It would be better if this iterator lived in the document
    let rendered_shapes_iter =
        doc.get_sequence()
            .iter()
            .filter_map(|shape_id| match &doc.get_shape_by_id(*shape_id) {
                Some(shape) => Some(render_shape(*shape_id, shape)),
                None => None,
            });
    rsx! {
        for shape in rendered_shapes_iter {
            { shape }
        }
    }
}

/* Shape dragging */

struct ShapeDragTracker {
    mouse_down_x: f64,
    mouse_down_y: f64,
    shape_id: ShapeId,
    initial_geometry: Geometry,
}

impl ShapeDragTracker {
    fn new(mouse_down: MouseEvent, shape_id: ShapeId, initial_geometry: &Geometry) -> Self {
        let mouse_down_x = mouse_down.data.page_coordinates().x;
        let mouse_down_y = mouse_down.data.page_coordinates().y;
        Self {
            mouse_down_x,
            mouse_down_y,
            shape_id,
            initial_geometry: initial_geometry.clone(),
        }
    }

    fn update_shape_for_drag(&self, mouse: &MouseEvent) {
        let event_x = mouse.data.page_coordinates().x;
        let event_y = mouse.data.page_coordinates().y;
        let new_geometry = self
            .initial_geometry
            .offset_by(event_x - self.mouse_down_x, event_y - self.mouse_down_y);
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

fn render_shape(shape_id: ShapeId, shape: &Shape) -> Element {
    let id_string = format!("shape_{}", shape_id);
    let fill_color = svg_color(&shape.style.fill);
    let initial_geometry = shape.geometry.clone();
    let shape_mouse_down = move |evt| {
        *CANVAS_TRACKER.write() = Some(Rc::new(ShapeDragTracker::new(
            evt,
            shape_id,
            &initial_geometry,
        )))
    };
    /*
    let move_shape_to_front = move |evt| DOC.write().move_shape_with_id_to_front(shape_id);
    */
    match &shape.geometry {
        Geometry::Circle { cx, cy, radius } => rsx! {
            circle {
                id: id_string,
                cx: *cx,
                cy: *cy,
                r: *radius,
                fill: fill_color,
                onmousedown: shape_mouse_down,
            }
        },
        Geometry::Rectangle {
            left,
            top,
            width,
            height,
        } => rsx! {
            rect {
                id: id_string,
                x: *left,
                y: *top,
                width: *width,
                height: *height,
                fill: fill_color,
                onmousedown: shape_mouse_down,
            }
        },
    }
}
