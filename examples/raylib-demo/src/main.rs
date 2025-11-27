use metaballs_rs::{BUFFER_HEIGHT, BUFFER_WIDTH, Metaballs};
use raylib::prelude::*;

fn main() {
    let mut app = Metaballs::setup();
    let (mut rl, thread) = raylib::init()
        .size(BUFFER_WIDTH as i32, BUFFER_HEIGHT as i32)
        .title("Metaballs demo")
        .resizable()
        .build();

    let image = Image::gen_image_color(BUFFER_WIDTH as i32, BUFFER_HEIGHT as i32, Color::BLACK);
    let mut texture = rl.load_texture_from_image(&thread, &image).unwrap();

    while !rl.window_should_close() {
        app.update(&[], rl.get_frame_time());

        let mut image_data = vec![0_u8; BUFFER_WIDTH * BUFFER_HEIGHT * 4];

        let buf = app.buffer();

        for i in 0..(BUFFER_WIDTH * BUFFER_HEIGHT) {
            let p = buf[i];

            image_data[i * 4] = p.r;
            image_data[i * 4 + 1] = p.g;
            image_data[i * 4 + 2] = p.b;
            image_data[i * 4 + 3] = 255;
        }

        texture.update_texture(&image_data).unwrap();

        rl.draw(&thread, |mut d| {
            d.draw_texture(&texture, 0, 0, Color::WHITE);
        });
    }
}
