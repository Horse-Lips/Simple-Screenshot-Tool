use std::error::Error;
use std::fs;
use std::path::PathBuf;
use chrono::Local;
use dirs::picture_dir;
use screenshots::Screen;
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Fullscreen},
};

fn main() -> Result<(), winit::error::EventLoopError> {
    let event_loop = EventLoop::new()?; // Create event loop
    let window = WindowBuilder::new()   // Create window
        .with_title("Simple Screenshot Tool")
        .with_decorations(false)
        .with_transparent(true)
        .with_fullscreen(Some(Fullscreen::Borderless(None)))
        .build(&event_loop)?;

    let mut is_dragging = false;
    let mut drag_start: Option<PhysicalPosition<f64>> = None;
    let mut drag_end: Option<PhysicalPosition<f64>> = None;

    event_loop.run(move |event, event_loop_wt| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::MouseInput { state, button, .. } => {  // Handle mouse left click and release
                    if button == MouseButton::Left {
                        match state {
                            ElementState::Pressed => {  // Handle left mouse button pressed
                                is_dragging = true;
                                println!("Dragging started");
                            }
                            ElementState::Released => { // Handle left mouse button released
                                is_dragging = false;
                                println!("Dragging finished");
                                if let (Some(start), Some(end)) = (drag_start, drag_end) {
                                    println!(
                                        "Region: ({:.0}, {:.0}) ({:.0}, {:.0})",
                                         start.x, start.y, end.x, end.y
                                    );

                                    window.set_visible(false); //Should probably make this actually click through or not drawn, etc.

                                    let x = start.x.min(end.x);
                                    let y = start.y.min(end.y);
                                    let path = capture_region(
                                        x as i32,
                                        y as i32,
                                        (start.x.max(end.x) - x) as u32,
                                        (start.y.max(end.y) - y) as u32
                                    );

                                    println!("Saved screenshot to {:?}", path);
                                }
                                drag_start = None;
                                drag_end = None;

                                event_loop_wt.exit();
                            }
                        }
                    }
                }
                WindowEvent::CursorMoved { position, .. } => {  // Handle cursor movement when clicking and dragging
                    if is_dragging {
                        if drag_start.is_none() {
                            drag_start = Some(position);
                        }
                        drag_end = Some(position);
                    }
                }
                WindowEvent::CloseRequested => {    // Handle closing window manually
                    event_loop_wt.exit();
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
    let screen = Screen::all()?
        .into_iter()
        .next()
        .ok_or("No screen found")?;

    // Capture rectangle
    let img = screen.capture_area(x, y, width, height);

    // Prepare out dir
    let mut out = picture_dir().unwrap_or_else(|| PathBuf::from("."));
    out.push("SimpleScreenshotTool");
    fs::create_dir_all(&out)?;

    // Save with timestamp
    let filename = format!("screenshot_{}.png", Local::now().format("%Y%m%d_%H%M%S"));
    let path = out.join(filename);
    img?.save(&path)?;

    Ok(path)
}