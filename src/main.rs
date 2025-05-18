use std::{
    error::Error,
    fs,
    path::PathBuf,
};
use arboard::{
    Clipboard,
    ImageData,
};
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Fullscreen},
};
use chrono::Local;
use dirs::picture_dir;
use screenshots::Screen;

fn main() -> Result<(), winit::error::EventLoopError> {
    let event_loop = EventLoop::new()
        .expect("Failed to create event loop"); // Create event loop
    let window = WindowBuilder::new()   // Create window
        .with_title("Simple Screenshot Tool")
        .with_transparent(true)
        .with_fullscreen(Some(Fullscreen::Borderless(None)))
        .build(&event_loop)
        .expect("Failed to create window.");

    let mut is_dragging = false;
    let mut drag_start: Option<PhysicalPosition<f64>> = None;
    let mut drag_end: Option<PhysicalPosition<f64>> = None;

    event_loop.run(move |event, event_loop_wt| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::MouseInput { state, button, .. } => {
                    if button == MouseButton::Left {
                        match state {
                            ElementState::Pressed => {  // Handle left mouse button pressed
                                is_dragging = true;
                            }
                            ElementState::Released => { // Handle left mouse button released
                                is_dragging = false;
                                if let (Some(start), Some(end)) = (drag_start, drag_end) {
                                    window.set_visible(false); //Should probably make this actually click through or not drawn, etc.

                                    let x = start.x.min(end.x);
                                    let y = start.y.min(end.y);
                                    let path = capture_region(
                                        x as i32,
                                        y as i32,
                                        (start.x.max(end.x) - x) as u32,
                                        (start.y.max(end.y) - y) as u32
                                    );
                                }
                                drag_start = None;
                                drag_end = None;

                                event_loop_wt.exit();
                            }
                        }
                    }
                }
                WindowEvent::CursorMoved { position, .. } => {
                    if is_dragging {    // Handle cursor movement when clicking and dragging
                        if drag_start.is_none() {
                            drag_start = Some(position);
                        }
                        drag_end = Some(position);
                    }
                }
                WindowEvent::CloseRequested => {
                    event_loop_wt.exit();   // Handle closing window manually
                }
                _ => {}
            },
            _ => {}
        }
        event_loop_wt.set_control_flow(ControlFlow::Wait);
    })
}

pub fn capture_region(x: i32, y: i32, width: u32, height: u32) -> Result<PathBuf, Box<dyn Error>> {
    // Grab primary screen
    let screen = Screen::all()
        .expect("Failed to get screen")
        .into_iter()
        .next()
        .ok_or("Failed to get screen")
        .expect("Failed to get screen");

    // Capture rectangle
    let img = screen.capture_area(x, y, width, height).expect("Failed to capture image.");
    let img_save = img.clone();
    let img_raw = img.into_raw();

    // Prepare out dir
    let mut out = picture_dir().unwrap_or_else(|| PathBuf::from("."));
    out.push("SimpleScreenshotTool");
    fs::create_dir_all(&out).expect("Failed to create output directory.");

    // Save with timestamp
    let filename = format!("screenshot_{}.png", Local::now().format("%Y%m%d_%H%M%S"));
    let path = out.join(filename);
    img_save.save(&path).expect("Failed to save image.");

    // Copy raw to clipboard
    let mut clipboard = Clipboard::new().expect("Failed to get clipboard");
    clipboard.set_image(ImageData {
        width: width as usize,
        height: height as usize,
        bytes: std::borrow::Cow::Owned(img_raw),
    }).expect("Failed to copy image to clipboard");

    Ok(path)
}