use winit::dpi::PhysicalPosition;

pub fn draw_rect(frame: &mut [u8], width: u32, start: PhysicalPosition<f64>, end: PhysicalPosition<f64>) {
    for chunk in frame.chunks_exact_mut(4) {
        chunk.copy_from_slice(&[0, 0, 0, 0]);   //Clear frame
    }

    let (x_min, x_max) = (start.x.min(end.x) as u32, start.x.max(end.x) as u32);
    let (y_min, y_max) = (start.y.min(end.y) as u32, start.y.max(end.y) as u32);

    for x in x_min..=x_max {
        draw_line(frame, width, x, y_min);
        draw_line(frame, width, x, y_max);
    }

    for y in y_min..=y_max {
        draw_line(frame, width, x_min, y);
        draw_line(frame, width, x_max, y);
    }
}

fn draw_line(frame: &mut [u8], width: u32, x: u32, y: u32) {
    let colour = [255, 0, 0, 255];
    let idx = ((y * width + x) * 4) as usize;
    if idx <= frame.len() {
        frame[idx..idx + 4].copy_from_slice(&colour);
    }
}