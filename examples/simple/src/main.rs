use std::{fs::File, io::Write};

use metaballs_rs::{Action, BUFFER_HEIGHT, BUFFER_WIDTH, Metaballs};

const FRAMES_NUM: usize = 240;
const FPS: usize = 60;

fn main() {
    let mut app = Metaballs::setup();
    let delta_t = 1. / FPS as f32;

    app.update(&[], delta_t);

    let mut x_push = 300;

    for i in 0..FRAMES_NUM {
        app.update(
            &[Action::MovePlayerBy {
                x: {
                    if !(-200..=900).contains(&app.player_pos().0) {
                        x_push *= -1;
                    }
                    x_push
                },
                y: 0,
            }],
            delta_t,
        );

        let f_name = format!("out/output-{:02}.ppm", i);
        let mut f = File::create(f_name).unwrap();
        write!(
            &mut f,
            "P6\n{} {}\n{}\n",
            BUFFER_WIDTH,
            BUFFER_HEIGHT,
            u8::MAX
        )
        .unwrap();

        let buf = app.buffer();
        let mut img_buf = vec![0x0; buf.len() * 3];

        for y in 0..BUFFER_HEIGHT {
            for x in 0..BUFFER_WIDTH {
                let p = buf[y * BUFFER_WIDTH + x];

                let idx = (y * BUFFER_WIDTH + x) * 3;

                img_buf[idx] = p.r;
                img_buf[idx + 1] = p.g;
                img_buf[idx + 2] = p.b;
            }
        }

        f.write_all(&img_buf).unwrap();
        f.flush().unwrap();
    }
}
