use pixels::Pixels;

#[derive(Clone, Copy)]
pub struct Circle {
    pub x: i32,
    pub y: i32,
    pub radius: i32,
    pub color: [u8; 4],
    pub follow_cursor: bool,
    pub emits_light: bool,
}

impl Circle {
    pub fn new(x: i32, y: i32, radius: i32, color: [u8; 4]) -> Self {
        Self {
            x,
            y,
            radius,
            color,
            follow_cursor: false,
            emits_light: false,
        }
    }

    pub fn with_cursor_follow(mut self, follow: bool) -> Self {
        self.follow_cursor = follow;
        self
    }

    pub fn with_light_emission(mut self, emits: bool) -> Self {
        self.emits_light = emits;
        self
    }

    pub fn get_center(&self, cursor_pos: &(f64, f64), window: &(u32, u32)) -> (i32, i32) {
        if self.follow_cursor {
            let x = cursor_pos.0.clamp(0.0, window.0 as f64 - 1.0) as i32;
            let y = cursor_pos.1.clamp(0.0, window.1 as f64 - 1.0) as i32;
            (x, y)
        } else {
            (self.x, self.y)
        }
    }
}

pub fn draw(pixels: &mut Pixels<'static>, cursor_pos: (f64, f64), width: u32, height: u32) {
    let frame = pixels.frame_mut();
    frame.fill(0);

    let window = (width, height);

    // Define objects
    let objects = vec![
        Circle::new(
            (width / 2) as i32,
            (height / 2) as i32,
            100,
            [255, 255, 255, 255],
        ),
        Circle::new(0, 0, 40, [255, 255, 200, 255])
            .with_cursor_follow(true)
            .with_light_emission(true),
    ];

    // Draw rays from light sources first (so they appear behind objects)
    for light in objects.iter() {
        if light.emits_light {
            let (cx, cy) = light.get_center(&cursor_pos, &window);
            draw_rays_with_shadows(cx, cy, &objects, frame, &window, &cursor_pos);
        }
    }

    // Draw all circles on top of rays
    for circle in objects.iter() {
        let (cx, cy) = circle.get_center(&cursor_pos, &window);
        draw_circle(cx, cy, circle.radius, circle.color, frame, &window);
    }
}

/// Checks if a point is inside a circle
fn point_in_circle(px: i32, py: i32, cx: i32, cy: i32, radius: i32) -> bool {
    let dx = px - cx;
    let dy = py - cy;
    (dx * dx + dy * dy) <= (radius * radius)
}

/// Finds the intersection point of a ray with any non-emitting circle
/// Returns Some((x, y)) if intersection found, None otherwise
fn find_ray_intersection(
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    objects: &[Circle],
    cursor_pos: &(f64, f64),
    window: &(u32, u32),
) -> Option<(i32, i32)> {
    // Sample points along the ray
    let dx = x1 - x0;
    let dy = y1 - y0;
    let distance = ((dx * dx + dy * dy) as f32).sqrt();
    let steps = distance.ceil() as i32;

    if steps == 0 {
        return None;
    }

    for step in 1..steps {
        let t = step as f32 / steps as f32;
        let px = (x0 as f32 + dx as f32 * t).round() as i32;
        let py = (y0 as f32 + dy as f32 * t).round() as i32;

        // Check collision with all non-emitting objects
        for obj in objects.iter() {
            if !obj.emits_light {
                let (cx, cy) = obj.get_center(cursor_pos, window);
                if point_in_circle(px, py, cx, cy, obj.radius) {
                    return Some((px, py));
                }
            }
        }
    }

    None
}

/// Draws rays emanating from a light source with shadow casting
fn draw_rays_with_shadows(
    cx: i32,
    cy: i32,
    objects: &[Circle],
    frame: &mut [u8],
    window: &(u32, u32),
    cursor_pos: &(f64, f64),
) {
    let ray_count = 360;
    let ray_color = [255, 255, 0, 100]; // Semi-transparent yellow

    for i in 0..ray_count {
        let theta = (i as f32) * std::f32::consts::PI / 180.0;
        let ray_length = (window.0.max(window.1)) as f32;
        let end_x = cx as f32 + ray_length * theta.cos();
        let end_y = cy as f32 + ray_length * theta.sin();

        let ex = end_x.round() as i32;
        let ey = end_y.round() as i32;

        // Check if ray hits a blocking object
        if let Some((hit_x, hit_y)) =
            find_ray_intersection(cx, cy, ex, ey, objects, cursor_pos, window)
        {
            // Draw ray only up to the intersection point
            draw_line(cx, cy, hit_x, hit_y, ray_color, frame, window);
        } else {
            // Draw full ray if no intersection
            draw_line(cx, cy, ex, ey, ray_color, frame, window);
        }
    }
}

/// Draws a line from (x0, y0) to (x1, y1) using Bresenham's algorithm
fn draw_line(
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    color: [u8; 4],
    frame: &mut [u8],
    window: &(u32, u32),
) {
    let mut x0 = x0;
    let mut y0 = y0;
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        if x0 >= 0 && x0 < window.0 as i32 && y0 >= 0 && y0 < window.1 as i32 {
            draw_on_point(x0 as u32, y0 as u32, color, frame, window.0);
        }
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}

/// Draws a filled circle using the Bresenham midpoint circle algorithm
fn draw_circle(
    cx: i32,
    cy: i32,
    radius: i32,
    color: [u8; 4],
    frame: &mut [u8],
    window: &(u32, u32),
) {
    let mut x = radius;
    let mut y = 0;
    let mut err = 0;

    while x >= y {
        for dy in [-y, y].iter() {
            draw_horizontal_line(cx - x, cx + x, cy + dy, color, frame, window);
        }
        for dy in [-x, x].iter() {
            draw_horizontal_line(cx - y, cx + y, cy + dy, color, frame, window);
        }

        y += 1;
        err += 1 + 2 * y;
        if 2 * (err - x) + 1 > 0 {
            x -= 1;
            err += 1 - 2 * x;
        }
    }
}

/// Draws a horizontal line from (x_start, y) to (x_end, y)
fn draw_horizontal_line(
    x_start: i32,
    x_end: i32,
    y: i32,
    color: [u8; 4],
    frame: &mut [u8],
    window: &(u32, u32),
) {
    if y < 0 || y >= window.1 as i32 {
        return;
    }
    let y = y as u32;

    for x in x_start..=x_end {
        if x >= 0 && x < window.0 as i32 {
            draw_on_point(x as u32, y, color, frame, window.0);
        }
    }
}

/// Draws a single pixel at (x, y) with alpha blending
fn draw_on_point(x: u32, y: u32, color: [u8; 4], frame: &mut [u8], width: u32) {
    let i = ((y * width + x) * 4) as usize;

    if i + 3 < frame.len() {
        let alpha = color[3] as f32 / 255.0;
        let inv_alpha = 1.0 - alpha;

        frame[i] = (color[0] as f32 * alpha + frame[i] as f32 * inv_alpha) as u8;
        frame[i + 1] = (color[1] as f32 * alpha + frame[i + 1] as f32 * inv_alpha) as u8;
        frame[i + 2] = (color[2] as f32 * alpha + frame[i + 2] as f32 * inv_alpha) as u8;
        frame[i + 3] = 255;
    }
}
