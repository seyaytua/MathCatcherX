use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use std::f64::consts::PI;
use crate::number::NumberObject;

pub struct Player {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub net_angle: f64,
    pub net_rotation_speed: f64,
    pub net_length: f64,
    pub net_width: f64,
    pub is_extending: bool,
    pub net_extension: f64,
    pub target_angle: f64,
}

impl Player {
    pub fn new(x: f64, y: f64) -> Self {
        Player {
            x,
            y,
            radius: 25.0,
            net_angle: 0.0,
            net_rotation_speed: 30.0 * PI / 180.0, // 30 degrees in radians
            net_length: 150.0,
            net_width: 80.0,
            is_extending: false,
            net_extension: 0.0,
            target_angle: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        // Rotate net continuously
        self.net_angle += self.net_rotation_speed * delta_time;
        if self.net_angle > 2.0 * PI {
            self.net_angle -= 2.0 * PI;
        }

        // Handle net extension animation
        if self.is_extending {
            self.net_extension += delta_time * 5.0;
            if self.net_extension >= 1.0 {
                self.net_extension = 1.0;
                self.is_extending = false;
            }
        } else if self.net_extension > 0.0 {
            self.net_extension -= delta_time * 3.0;
            if self.net_extension < 0.0 {
                self.net_extension = 0.0;
            }
        }
    }

    pub fn draw(&self, context: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        // Draw player circle
        context.set_fill_style(&JsValue::from_str("#4a90e2"));
        context.begin_path();
        context.arc(self.x, self.y, self.radius, 0.0, 2.0 * PI)?;
        context.fill();

        // Draw player border
        context.set_stroke_style(&JsValue::from_str("#ffffff"));
        context.set_line_width(3.0);
        context.stroke();

        // Draw net
        self.draw_net(context)?;

        Ok(())
    }

    fn draw_net(&self, context: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        let extension = self.net_extension;
        let length = self.net_length * extension;

        if extension > 0.0 {
            let end_x = self.x + length * self.net_angle.cos();
            let end_y = self.y + length * self.net_angle.sin();

            // Draw net line
            context.set_stroke_style(&JsValue::from_str("#ffdd00"));
            context.set_line_width(4.0);
            context.begin_path();
            context.move_to(self.x, self.y);
            context.line_to(end_x, end_y);
            context.stroke();

            // Draw net head (circle)
            context.set_fill_style(&JsValue::from_str("#ffdd00"));
            context.begin_path();
            context.arc(end_x, end_y, self.net_width / 2.0, 0.0, 2.0 * PI)?;
            context.fill();

            // Draw net mesh pattern
            context.set_stroke_style(&JsValue::from_str("#cc9900"));
            context.set_line_width(2.0);
            for i in 0..3 {
                let r = (self.net_width / 2.0) * (i as f64 / 3.0);
                context.begin_path();
                context.arc(end_x, end_y, r, 0.0, 2.0 * PI)?;
                context.stroke();
            }
        }

        // Draw rotation indicator
        let indicator_length = self.radius + 15.0;
        let indicator_x = self.x + indicator_length * self.net_angle.cos();
        let indicator_y = self.y + indicator_length * self.net_angle.sin();

        context.set_fill_style(&JsValue::from_str("#ff6b6b"));
        context.begin_path();
        context.arc(indicator_x, indicator_y, 5.0, 0.0, 2.0 * PI)?;
        context.fill();

        Ok(())
    }

    pub fn handle_click(&mut self, _x: f64, _y: f64) {
        // Extend net when clicked
        self.is_extending = true;
        self.net_extension = 0.0;
    }

    pub fn is_net_extended(&self) -> bool {
        self.net_extension > 0.5
    }

    pub fn check_net_collision(&self, number: &NumberObject) -> bool {
        if self.net_extension < 0.5 {
            return false;
        }

        let length = self.net_length * self.net_extension;
        let net_end_x = self.x + length * self.net_angle.cos();
        let net_end_y = self.y + length * self.net_angle.sin();

        // Check distance from net end to number
        let dx = net_end_x - number.x;
        let dy = net_end_y - number.y;
        let distance = (dx * dx + dy * dy).sqrt();

        distance < (self.net_width / 2.0 + number.radius)
    }
}
