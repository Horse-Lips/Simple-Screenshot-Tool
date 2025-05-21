use std::path::PathBuf;
use std::error::Error;
use screenshots::Screen;
use dirs::picture_dir;
use std::fs;
use time::OffsetDateTime;
use arboard::{Clipboard, ImageData};

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
    let now = OffsetDateTime::now_local().unwrap();

    let filename = format!(
        "screenshot_{:04}{:02}{:02}_{:02}{:02}{:02}.png",
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute(),
        now.second()
    );
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