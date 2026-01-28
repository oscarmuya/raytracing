use pixels::Pixels;

const WIDTH: u32 = 600;
const HEIGHT: u32 = 600;

pub fn draw(pixels: &mut Pixels<'static>) {
    let frame = pixels.frame_mut();
    frame.fill(0);

    let center_x = (WIDTH / 2) as i32;
    let center_y = (HEIGHT / 2) as i32;

    draw_circle(center_x, center_y, 50, [255, 255, 255, 255], frame);
}

/// Draws a filled circle into an RGBA frame buffer using the
/// Bresenham midpoint circle algorithm.
///
/// The frame buffer is assumed to be laid out in row-major order,
/// with 4 bytes per pixel in **RGBA** format.
///
/// # Arguments
/// * `center_x` - X coordinate of the circle center (0-based)
/// * `center_y` - Y coordinate of the circle center (0-based)
/// * `radius` - Radius of the circle in pixels
/// * `color` - RGBA color `[r, g, b, a]` to fill the circle
/// * `frame` - Mutable RGBA frame buffer
///
/// # Panics
/// Does not panic if points are out of bounds; points outside the frame
/// are ignored automatically.
fn draw_circle(center_x: i32, center_y: i32, radius: i32, color: [u8; 4], frame: &mut [u8]) {
    let mut x = radius;
    let mut y = 0;
    let mut err = 0;

    while x >= y {
        // Draw horizontal lines between symmetric points to fill
        for dy in [-y, y].iter() {
            draw_horizontal_line(center_x - x, center_x + x, center_y + dy, color, frame);
        }
        for dy in [-x, x].iter() {
            draw_horizontal_line(center_x - y, center_x + y, center_y + dy, color, frame);
        }

        y += 1;
        err += 1 + 2 * y;
        if 2 * (err - x) + 1 > 0 {
            x -= 1;
            err += 1 - 2 * x;
        }
    }
}

/// Draws a horizontal line from `(x_start, y)` to `(x_end, y)`
/// in the RGBA frame buffer.
///
/// Out-of-bounds points are ignored.
///
/// # Arguments
/// * `x_start` - starting x coordinate
/// * `x_end` - ending x coordinate
/// * `y` - y coordinate
/// * `color` - RGBA color `[r, g, b, a]`
/// * `frame` - mutable RGBA frame buffer
fn draw_horizontal_line(x_start: i32, x_end: i32, y: i32, color: [u8; 4], frame: &mut [u8]) {
    if y < 0 || y >= HEIGHT as i32 {
        return;
    }
    let y = y as u32;

    for x in x_start..=x_end {
        if x >= 0 && x < WIDTH as i32 {
            draw_on_point(x as u32, y, color, frame);
        }
    }
}

/// Draws a single pixel at `(x, y)` into an RGBA frame buffer.
///
/// The frame buffer is assumed to be laid out in row-major order,
/// with 4 bytes per pixel in **RGBA** format.
///
/// If the calculated index is out of bounds, the function does nothing.
///
/// # Arguments
/// * `x` - X coordinate of the pixel (0-based)
/// * `y` - Y coordinate of the pixel (0-based)
/// * `color` - RGBA color `[r, g, b, a]`
/// * `frame` - Mutable RGBA frame buffer
fn draw_on_point(x: u32, y: u32, color: [u8; 4], frame: &mut [u8]) {
    let i = ((y * WIDTH + x) * 4) as usize;

    if i + 3 < frame.len() {
        frame[i] = color[0];
        frame[i + 1] = color[1];
        frame[i + 2] = color[2];
        frame[i + 3] = color[3];
    }
}
