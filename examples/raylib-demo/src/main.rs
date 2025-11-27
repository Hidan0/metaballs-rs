use crate::KeyboardKey::{KEY_DOWN, KEY_LEFT, KEY_RIGHT, KEY_UP};
use metaballs_rs::{Action, BG_COLOR};
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

    let vel = 250.;

    rl.set_target_fps(60);
    let mut actions = vec![];

    while !rl.window_should_close() {
        actions.clear();

        if rl.is_key_down(KEY_RIGHT) {
            actions.push(Action::MovePlayerBy {
                x: vel as i32,
                y: 0,
            });
        }
        if rl.is_key_down(KEY_LEFT) {
            actions.push(Action::MovePlayerBy {
                x: -vel as i32,
                y: 0,
            });
        }
        if rl.is_key_down(KEY_UP) {
            actions.push(Action::MovePlayerBy {
                x: 0,
                y: -vel as i32,
            });
        }
        if rl.is_key_down(KEY_DOWN) {
            actions.push(Action::MovePlayerBy {
                x: 0,
                y: vel as i32,
            });
        }

        app.update(&actions, rl.get_frame_time());
        let buf = app.buffer();

        let mut image_data = vec![0_u8; BUFFER_WIDTH * BUFFER_HEIGHT * 4];

        for i in 0..(BUFFER_WIDTH * BUFFER_HEIGHT) {
            let p = buf[i];

            image_data[i * 4] = p.r;
            image_data[i * 4 + 1] = p.g;
            image_data[i * 4 + 2] = p.b;
            image_data[i * 4 + 3] = 255;
        }

        texture.update_texture(&image_data).unwrap();

        rl.draw(&thread, |mut d| {
            d.clear_background(Color::new(BG_COLOR.r, BG_COLOR.g, BG_COLOR.b, 255));
            d.draw_texture(&texture, 0, 0, Color::WHITE);
        });
    }
}
