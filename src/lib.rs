use std::error::Error;

pub const BUFFER_WIDTH: usize = 1280;
pub const BUFFER_HEIGHT: usize = 720;

pub const BG_COLOR: Pixel = Pixel::from_hex(0x11111B00);
pub const RED_COLOR: Pixel = Pixel::from_hex(0xD20F3900);
pub const BLUE_COLOR: Pixel = Pixel::from_hex(0x7287FD00);

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

    #[inline]
    pub const fn blend(&self, o: Pixel, p: f32) -> Pixel {
        Pixel {
            r: (self.r as f32 * p + o.r as f32 * (1. - p)) as u8,
            g: (self.g as f32 * p + o.g as f32 * (1. - p)) as u8,
            b: (self.b as f32 * p + o.b as f32 * (1. - p)) as u8,
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
    MoveBallTo { id: usize, x: u32, y: u32 },
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
            color: RED_COLOR,
        };

        let b = Ball {
            x: (0.75 * BUFFER_WIDTH as f32) as u32,
            y: half_h,
            r: 15, // pixels
            color: BLUE_COLOR,
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
            match *action {
                Action::MoveBall { id, x, y } => {
                    let mut b = self.balls[id];

                    b.x = (b.x as i32 + x).unsigned_abs();
                    b.y = (b.y as i32 + y).unsigned_abs();

                    self.balls[id] = b;
                }
                Action::MoveBallTo { id, x, y } => {
                    let mut b = self.balls[id];

                    b.x = x;
                    b.y = y;

                    self.balls[id] = b;
                }
            }
        }

        self.draw();

        Ok(())
    }

    fn draw(&'a mut self) {
        let ths: f32 = 0.2; // ???

        for y in 0..BUFFER_HEIGHT {
            for x in 0..BUFFER_WIDTH {
                let b1 = self.balls[0];
                let b2 = self.balls[1];

                let s1 = b1.r as f32 / b1.dst_from(x as u32, y as u32);
                let s2 = b2.r as f32 / b2.dst_from(x as u32, y as u32);
                let metaballs = s1 + s2;

                if metaballs >= ths {
                    self.buf[y * BUFFER_WIDTH + x] = b1.color.blend(b2.color, s1 / metaballs);
                } else {
                    self.buf[y * BUFFER_WIDTH + x] = BG_COLOR;
                }
            }
        }
    }
}
