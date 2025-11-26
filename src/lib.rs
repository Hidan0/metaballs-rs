use std::f32::consts::PI;

pub const BUFFER_WIDTH: usize = 1280;
pub const BUFFER_HEIGHT: usize = 720;

pub const BG_COLOR: Pixel = Pixel::from_hex(0x11111B);
pub const RED_COLOR: Pixel = Pixel::from_hex(0xD20F39);
pub const BLUE_COLOR: Pixel = Pixel::from_hex(0x1E66F5);
pub const YELLOW_COLOR: Pixel = Pixel::from_hex(0xDF8E1D);

#[derive(Clone, Copy)]
pub struct Pixel {
    pub r: u8,
    pub b: u8,
    pub g: u8,
}

impl Pixel {
    /// 0xRRGGBBxx
    #[inline]
    pub const fn from_hex(literal: u32) -> Self {
        Pixel {
            r: ((literal & 0xFF0000) >> 16) as u8,
            g: ((literal & 0xFF00) >> 8) as u8,
            b: (literal & 0xFF) as u8,
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
    x: i32,
    y: i32,
    r: i32,
    color: Pixel,
}

fn dst_from(x1: i32, y1: i32, x2: i32, y2: i32) -> f32 {
    let dx = (x2 - x1) as f32;
    let dy = (y2 - y1) as f32;

    (dx * dx + dy * dy).sqrt()
}

impl Ball {
    fn dst_from(&self, x: i32, y: i32) -> f32 {
        dst_from(self.x, self.y, x, y)
    }
}

pub enum Action {
    MovePlayerBy { x: i32, y: i32 },
    MovePlayerTo { x: i32, y: i32 },
}

const PLAYER_ID: usize = 0;
const SUN_ID: usize = 1;
const PLANET_ID: usize = 2;

pub struct Metaballs {
    buf: Vec<Pixel>,
    balls: [Ball; 3],
    planet_angle: f32,
    orbit_r: f32,
}

impl<'a> Metaballs {
    pub fn setup() -> Self {
        let half_h = (0.5 * BUFFER_HEIGHT as f32) as i32;
        let half_w = (0.5 * BUFFER_WIDTH as f32) as i32;

        let player = Ball {
            x: half_w - 400,
            y: half_h,
            r: 5,
            color: RED_COLOR,
        };

        let sun = Ball {
            x: half_w,
            y: half_h,
            r: 15,
            color: YELLOW_COLOR,
        };

        let r = 225;
        let planet = Ball {
            x: half_w + r,
            y: half_h,
            r: 5,
            color: BLUE_COLOR,
        };

        let dx = (planet.x - sun.x) as f32;
        let dy = (planet.y - sun.y) as f32;

        let angle = dy.atan2(dx);

        Self {
            buf: vec![BG_COLOR; BUFFER_HEIGHT * BUFFER_WIDTH],
            balls: [player, sun, planet],
            planet_angle: angle,
            orbit_r: r as f32,
        }
    }

    pub fn buffer(&'a self) -> &'a [Pixel] {
        &self.buf
    }

    pub fn update(&'a mut self, actions: &[Action], dt: f32) {
        for action in actions {
            match *action {
                Action::MovePlayerBy { x, y } => {
                    let mut b = self.balls[PLAYER_ID];

                    b.x = (b.x + (x as f32 * dt) as i32).clamp(0, BUFFER_WIDTH as i32);
                    b.y = (b.y + (y as f32 * dt) as i32).clamp(0, BUFFER_HEIGHT as i32);

                    self.balls[PLAYER_ID] = b;
                }
                Action::MovePlayerTo { x, y } => {
                    let mut b = self.balls[PLAYER_ID];

                    b.x = x.clamp(0, BUFFER_WIDTH as i32);
                    b.y = y.clamp(0, BUFFER_HEIGHT as i32);

                    self.balls[PLAYER_ID] = b;
                }
            }
        }

        let sun = self.balls[SUN_ID];
        let mut planet = self.balls[PLANET_ID];

        self.planet_angle += PI / 90.;

        planet.x = (sun.x as f32 + self.planet_angle.cos() * self.orbit_r) as i32;
        planet.y = (sun.y as f32 + self.planet_angle.sin() * self.orbit_r) as i32;

        self.balls[PLANET_ID] = planet;

        self.draw();
    }

    fn draw(&'a mut self) {
        let ths: f32 = 0.2; // ???

        for y in 0..BUFFER_HEIGHT {
            for x in 0..BUFFER_WIDTH {
                let mut f = [0.; 3];
                let mut dx = [0.; 3];

                let mut tot_dx = 0.;
                let mut metaballs = 0.;

                (0..3).for_each(|i| {
                    let b = self.balls[i];
                    let dst = b.dst_from(x as i32, y as i32);

                    dx[i] = dst;
                    f[i] = b.r as f32 / dst;

                    tot_dx += dst;
                    metaballs += f[i];
                });

                if metaballs >= ths {
                    let b1 = self.balls[0].color;
                    let p1 = (tot_dx - dx[0]) / tot_dx;
                    let b2 = self.balls[1].color;
                    let p2 = (tot_dx - dx[1]) / tot_dx;
                    let b3 = self.balls[2].color;
                    let p3 = (tot_dx - dx[2]) / tot_dx;

                    self.buf[y * BUFFER_WIDTH + x] = Pixel {
                        r: (p1 * b1.r as f32 + p2 * b2.r as f32 + p3 * b3.r as f32) as u8,
                        g: (p1 * b1.g as f32 + p2 * b2.g as f32 + p3 * b3.g as f32) as u8,
                        b: (p1 * b1.b as f32 + p2 * b2.b as f32 + p3 * b3.b as f32) as u8,
                    };
                } else {
                    self.buf[y * BUFFER_WIDTH + x] = BG_COLOR;
                }
            }
        }
    }

    pub fn player_pos(&self) -> (i32, i32) {
        let p = self.balls[PLAYER_ID];

        (p.x, p.y)
    }
}
