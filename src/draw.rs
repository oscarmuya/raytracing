use pixels::{Pixels};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;


pub fn draw(pixels: &mut Pixels<'static>) {
    let frame = pixels.frame_mut();
    frame.fill(0);

    let center_x = WIDTH / 2;
    let center_y = HEIGHT / 2;
    let i = ((center_y * WIDTH + center_x) * 4) as usize;

    println!("{}, {}", i, frame.len());

    if i + 3 < frame.len() {
        frame[i] = 255;
        frame[i + 1] = 255;
        frame[i + 2] = 255;
        frame[i + 3] = 255;
    }
}
