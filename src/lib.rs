use std::error::Error;

pub const BUFFER_WIDTH: usize = 1280;
pub const BUFFER_HEIGHT: usize = 720;

pub const BG_COLOR: Pixel = Pixel::from_hex(0x11111B00);
pub const RED_COLOR: Pixel = Pixel::from_hex(0xF38BA800);
pub const BLUE_COLOR: Pixel = Pixel::from_hex(0x89B4FA00);

#[derive(Clone, Copy)]
pub struct Pixel {
    pub r: u8,
    pub b: u8,
    pub g: u8,
}

impl Pixel {
    /// 0xRRGGBBxx
    #[inline]
    pub const fn from_hex(word: u32) -> Self {
        Pixel {
            r: ((word & 0xFF000000) >> 24) as u8,
            g: ((word & 0x00FF0000) >> 16) as u8,
            b: ((word & 0x0000FF00) >> 8) as u8,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Ball {
    x: u32,
    y: u32,
    r: u32,
    color: Pixel,
}

impl Ball {
    fn dst_from(&self, x: u32, y: u32) -> f32 {
        let dx = x.abs_diff(self.x) as f32;
        let dy = y.abs_diff(self.y) as f32;

        (dx * dx + dy * dy).sqrt()
    }
}

pub enum Action {
    MoveBall { id: usize, x: i32, y: i32 },
}

pub struct Metaballs {
    buf: Vec<Pixel>,
    balls: [Ball; 2],
}

impl Default for Metaballs {
    fn default() -> Self {
        Metaballs::setup()
    }
}

impl<'a> Metaballs {
    pub fn setup() -> Self {
        let half_h = (0.5 * BUFFER_HEIGHT as f32) as u32;
        let a = Ball {
            x: (0.25 * BUFFER_WIDTH as f32) as u32,
            y: half_h,
            r: 10, // pixels
            color: Pixel::from_hex(0xFF000000),
        };

        let b = Ball {
            x: (0.75 * BUFFER_WIDTH as f32) as u32,
            y: half_h,
            r: 15, // pixels
            color: Pixel::from_hex(0x0000FF00),
        };

        Self {
            buf: vec![BG_COLOR; BUFFER_HEIGHT * BUFFER_WIDTH],
            balls: [a, b],
        }
    }

    pub fn buffer(&'a self) -> &'a [Pixel] {
        &self.buf
    }

    pub fn update(&'a mut self, actions: &[Action]) -> Result<(), Box<dyn Error>> {
        for action in actions {
            match action {
                &Action::MoveBall { id, x, y } => {
                    let mut b = self.balls[id];

                    b.x = (b.x as i32 + x).unsigned_abs();
                    b.y = (b.y as i32 + y).unsigned_abs();

                    self.balls[id] = b;
                }
            }
        }

        self.draw();

        Ok(())
    }

    fn draw(&'a mut self) {
        let ths = 0.2; // ???

        for y in 0..BUFFER_HEIGHT {
            for x in 0..BUFFER_WIDTH {
                let metaball: f32 = self
                    .balls
                    .map(|b| b.r as f32 / b.dst_from(x as u32, y as u32))
                    .iter()
                    .sum();

                if metaball >= ths {
                    self.buf[y * BUFFER_WIDTH + x] = RED_COLOR;
                } else {
                    self.buf[y * BUFFER_WIDTH + x] = BG_COLOR;
                }
            }
        }
    }
}
