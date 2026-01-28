# Ray Tracing in Rust

In this project im implementing how the rays from a light source behave when they hit an object.

<img width="1366" height="768" alt="screenshot-2026-01-28_18-16-05" src="https://github.com/user-attachments/assets/05e7d4cf-54f1-410d-86fe-10d620cd757f" />


## Project Overview

The application is built in Rust and uses the `pixels` crate for rendering and `winit` for windowing and event handling. The core logic is split into two main files: `src/main.rs` for the application entry point and window management, and `src/draw.rs` for the rendering and ray tracing logic.

## `src/main.rs`: The Application Core

This file is responsible for setting up the application window, handling user input, and managing the main event loop.

### `App` Struct

The `App` struct holds the state of the application:

```rust
struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    cursor_pos: (f64, f64),
    width: u32,
    height: u32,
}
```

-   `window`: The `winit` window object.
-   `pixels`: The `pixels` frame buffer where all rendering happens.
-   `cursor_pos`: The current position of the mouse cursor, used for the interactive light source.
-   `width`, `height`: The dimensions of the window.

### Event Loop

The `main` function initializes the `App` struct and starts the `winit` event loop. The event loop handles the following events:

-   **`Resumed`**: This is where the window and the `pixels` buffer are created.
-   **`WindowEvent::CursorMoved`**: Updates the `cursor_pos` in the `App` struct.
-   **`WindowEvent::CloseRequested`**: Closes the application.
-   **`WindowEvent::RedrawRequested`**: This event is triggered when the window needs to be repainted. This is where we call the `draw::draw` function to render a new frame.

## `src/draw.rs`: Rendering and Ray Tracing

This module contains all the logic for drawing objects, casting rays, and calculating shadows.

### `Circle` Struct

The `Circle` struct is the only object type in our scene for now.

```rust
pub struct Circle {
    pub x: i32,
    pub y: i32,
    pub radius: i32,
    pub color: [u8; 4],
    pub follow_cursor: bool,
    pub emits_light: bool,
}
```

-   `x`, `y`, `radius`: Define the circle's geometry.
-   `color`: The RGBA color of the circle.
-   `follow_cursor`: A boolean flag to indicate if the circle should follow the mouse cursor.
-   `emits_light`: A boolean flag to indicate if the circle is a light source.

### The `draw` function

This is the main entry point for the rendering logic. It is called on every frame.

1.  **Clear the frame**: The frame is first filled with a solid color (black) to clear the previous frame's content.
2.  **Define objects**: A `Vec<Circle>` is created to define the objects in the scene.
3.  **Draw rays and shadows**: The function iterates through the objects and finds any that emit light (`emits_light == true`). For each light source, it calls `draw_rays_with_shadows`.
4.  **Draw objects**: After drawing the rays, the function iterates through all the objects again and draws them on top of the rays using the `draw_circle` function. This ensures that the objects are rendered in front of the rays.

### Ray Tracing and Shadows

The core of the ray tracing logic is in the `draw_rays_with_shadows` and `find_ray_intersection` functions.

-   `draw_rays_with_shadows`: For a given light source, this function casts 360 rays in all directions. For each ray, it calls `find_ray_intersection` to determine if the ray hits an object.
    -   If an intersection is found, the ray is drawn only up to the intersection point.
    -   If no intersection is found, the ray is drawn to the edge of the screen.
-   `find_ray_intersection`: This function takes a ray (defined by a start and end point) and a list of objects. It works by "marching" along the ray and checking at each step if the current point is inside any of the non-light-emitting objects. The first intersection found is returned.

### Drawing Primitives

The following functions are used for the actual drawing on the `pixels` buffer:

-   **`draw_circle`**: Implements the midpoint circle algorithm to draw a filled circle. It works by drawing horizontal lines to fill the circle.
-   **`draw_line`**: Implements Bresenham's line algorithm to draw a line between two points.
-   **`draw_on_point`**: This is the lowest-level drawing function. It draws a single pixel at a given `(x, y)` coordinate with a given color. It also handles alpha blending to allow for semi-transparent objects.
