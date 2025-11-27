use metaballs_rs::Action;
use metaballs_rs::BUFFER_HEIGHT;
use metaballs_rs::BUFFER_WIDTH;
use metaballs_rs::Metaballs;
use wasm_bindgen::Clamped;
use wasm_bindgen::prelude::*;
use web_sys::js_sys;
use web_sys::{CanvasRenderingContext2d, ImageData};

#[wasm_bindgen]
pub struct MetaballsWeb {
    metaballs: Metaballs,
}

#[wasm_bindgen]
impl MetaballsWeb {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            metaballs: Metaballs::setup(),
        }
    }

    pub fn update(&mut self, actions: js_sys::Array, dt: f32) -> Result<(), JsValue> {
        let mut rust_actions = Vec::new();

        for i in 0..actions.length() {
            let action = actions.get(i);

            let action_type = js_sys::Reflect::get(&action, &JsValue::from_str("type"))?
                .as_string()
                .ok_or_else(|| JsValue::from_str("Missing type"))?;

            match action_type.as_str() {
                "MovePlayerBy" => {
                    let x = js_sys::Reflect::get(&action, &JsValue::from_str("x"))?
                        .as_f64()
                        .ok_or_else(|| JsValue::from_str("Missing x"))?
                        as i32;
                    let y = js_sys::Reflect::get(&action, &JsValue::from_str("y"))?
                        .as_f64()
                        .ok_or_else(|| JsValue::from_str("Missing y"))?
                        as i32;
                    rust_actions.push(Action::MovePlayerBy { x, y });
                }
                "MovePlayerTo" => {
                    let x = js_sys::Reflect::get(&action, &JsValue::from_str("x"))?
                        .as_f64()
                        .ok_or_else(|| JsValue::from_str("Missing x"))?
                        as i32;
                    let y = js_sys::Reflect::get(&action, &JsValue::from_str("y"))?
                        .as_f64()
                        .ok_or_else(|| JsValue::from_str("Missing y"))?
                        as i32;
                    rust_actions.push(Action::MovePlayerTo { x, y });
                }
                _ => {}
            }
        }

        self.metaballs.update(&rust_actions, dt);
        Ok(())
    }

    pub fn render(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        let buf = self.metaballs.buffer();
        let mut data = vec![0u8; BUFFER_WIDTH * BUFFER_HEIGHT * 4];

        for i in 0..(BUFFER_WIDTH * BUFFER_HEIGHT) {
            let p = buf[i];
            data[i * 4] = p.r;
            data[i * 4 + 1] = p.g;
            data[i * 4 + 2] = p.b;
            data[i * 4 + 3] = 255;
        }

        let image_data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&data),
            BUFFER_WIDTH as u32,
            BUFFER_HEIGHT as u32,
        )?;

        ctx.put_image_data(&image_data, 0.0, 0.0)?;
        Ok(())
    }
}
