#![windows_subsystem = "windows"]   // Don't show terminal

use minifb::{Key, MouseButton, MouseMode, Scale, ScaleMode, Window, WindowOptions};
use winit::{
    event_loop::{EventLoop},
    window::{Fullscreen, WindowBuilder},
};

mod capture;

fn main() {
    let event_loop = EventLoop::new()
        .expect("Failed to create event loop"); // Create event loop
    let window = WindowBuilder::new()   // Create window
        .with_fullscreen(Some(Fullscreen::Borderless(None)))
        .build(&event_loop)
        .expect("Failed to create window.");

    let width = window.outer_size().width as usize; // Still need winit to get width/height
    let height = window.outer_size().height as usize;

    event_loop.exit();

    let mut window = Window::new(
        "Simple Screenshot Tool",
        width,
        height,
        WindowOptions {
            borderless: true,
            title: false,
            resize: true,
            scale: Scale::X1,
            scale_mode: ScaleMode::Stretch,
            topmost: true,
            transparency: true,
            none: false,
        },
    ).unwrap_or_else(|e| {
        panic!("Failed to create window: {}", e);
    });

    let mut buffer_dim = vec![0x88000000; width * height];
    let buffer_clear = vec![0x00000000; width * height];

    let mut is_dragging = false;
    let mut drag_start: Option<(f32, f32)> = None;
    let mut drag_end: Option<(f32, f32)> = None;

    while window.is_open() {
        window.update_with_buffer(&buffer_dim, width, height).unwrap();

        if let Some((x, y)) = window.get_mouse_pos(MouseMode::Clamp) {  // Clamp mouse coords within window
            let left_pressed = window.get_mouse_down(MouseButton::Left);

            if left_pressed && !is_dragging {   // Left mouse pressed
                is_dragging = true;
                drag_start = Some((x, y));
            }

            if left_pressed && is_dragging {    // Mouse moved while left mouse pressed
                drag_end = Some((x, y));

                buffer_dim.fill(0x88000000);    // Required to make transparent selection smaller

                if let (Some((start_x, start_y)), Some((end_x, end_y))) = (drag_start, drag_end) {
                    let x = start_x.min(end_x) as usize;
                    let y = start_y.min(end_y) as usize;
                    let w = start_x.max(end_x) as usize - x;
                    let h = start_y.max(end_y) as usize - y;

                    for i in y..(y + h) {
                        for j in x..(x + w) {
                            let index = i * width + j;
                            if index < buffer_dim.len() {
                                buffer_dim[index] = 0x00000000;  // Make selection transparent
                            }
                        }
                    }
                }
            }

            if !left_pressed && is_dragging {   // Left mouse released
                is_dragging = false;

                if let (Some((start_x, start_y)), Some((end_x, end_y))) = (drag_start, drag_end) {
                    let x = start_x.min(end_x);
                    let y = start_y.min(end_y);
                    let w = (start_x.max(end_x) - x) as u32;
                    let h = (start_y.max(end_y) - y) as u32;

                    window.update_with_buffer(&buffer_clear, width, height).unwrap();   // Set whole window transparent for screenshot

                    let _path = capture::capture_region(x as i32, y as i32, w, h);
                    break;
                }
            }
        }

        if window.is_key_down(Key::Escape) {
            break;
        }
    }
}