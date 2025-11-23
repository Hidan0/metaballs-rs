use std::{fs::File, io::Write};

use metaballs_rs::{Action, BUFFER_HEIGHT, BUFFER_WIDTH, Metaballs};

const FRAMES_NUM: usize = 180;
const FPS: usize = 60;

fn main() {
    let mut app = Metaballs::setup();
    let delta_t = 1. / FPS as f32;

    app.update(&[
        Action::MoveBallTo {
            id: 0,
            x: 200,
            y: BUFFER_HEIGHT as u32 - 200,
        },
        Action::MoveBallTo {
            id: 1,
            x: (BUFFER_WIDTH as f32 * 0.5) as u32,
            y: (BUFFER_HEIGHT as f32 * 0.5) as u32,
        },
    ])
    .unwrap();

    let force = (350., -150.);

    for i in 0..FRAMES_NUM {
        app.update(&[Action::MoveBall {
            id: 0,
            x: (force.0 * delta_t) as i32,
            y: (force.1 * delta_t) as i32,
        }])
        .unwrap();

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
