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
    pub target_x: Option<f64>,
    pub target_y: Option<f64>,
    pub move_speed: f64,
    pub hp: f64,
    pub max_hp: f64,
}

impl Player {
    pub fn new(x: f64, y: f64) -> Self {
        Player {
            x,
            y,
            radius: 25.0,
            net_angle: 0.0,
            net_rotation_speed: 30.0 * PI / 180.0, // 30 degrees per second in radians
            net_length: 100.0,
            net_width: 60.0,
            is_extending: true, // Net is always extended
            net_extension: 1.0,
            target_x: None,
            target_y: None,
            move_speed: 250.0, // pixels per second
            hp: 100.0,
            max_hp: 100.0,
        }
    }

    pub fn update(&mut self, delta_time: f64, canvas_width: f64, canvas_height: f64) {
        // Rotate net continuously (clockwise)
        self.net_angle += self.net_rotation_speed * delta_time;
        if self.net_angle > 2.0 * PI {
            self.net_angle -= 2.0 * PI;
        }

        // Move player towards target position
        if let (Some(target_x), Some(target_y)) = (self.target_x, self.target_y) {
            let dx = target_x - self.x;
            let dy = target_y - self.y;
            let distance = (dx * dx + dy * dy).sqrt();

            if distance > 5.0 {
                let move_distance = self.move_speed * delta_time;
                let ratio = move_distance / distance;
                
                if ratio >= 1.0 {
                    // Reached target
                    self.x = target_x;
                    self.y = target_y;
                } else {
                    // Move towards target
                    self.x += dx * ratio;
                    self.y += dy * ratio;
                }
            }

            // Keep player within bounds
            self.x = self.x.clamp(self.radius, canvas_width - self.radius);
            self.y = self.y.clamp(self.radius, canvas_height - self.radius);
        }
    }

    pub fn set_target(&mut self, x: f64, y: f64) {
        self.target_x = Some(x);
        self.target_y = Some(y);
    }

    pub fn take_damage(&mut self, damage: f64) {
        self.hp = (self.hp - damage).max(0.0);
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0.0
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
        let length = self.net_length;
        let end_x = self.x + length * self.net_angle.cos();
        let end_y = self.y + length * self.net_angle.sin();

        // Draw net line
        context.set_stroke_style(&JsValue::from_str("#ffdd00"));
        context.set_line_width(5.0);
        context.begin_path();
        context.move_to(self.x, self.y);
        context.line_to(end_x, end_y);
        context.stroke();

        // Draw net head (circle)
        context.set_fill_style(&JsValue::from_str("rgba(255, 221, 0, 0.7)"));
        context.begin_path();
        context.arc(end_x, end_y, self.net_width / 2.0, 0.0, 2.0 * PI)?;
        context.fill();

        // Draw net border
        context.set_stroke_style(&JsValue::from_str("#ffdd00"));
        context.set_line_width(3.0);
        context.stroke();

        // Draw net mesh pattern
        context.set_stroke_style(&JsValue::from_str("#cc9900"));
        context.set_line_width(2.0);
        for i in 1..4 {
            let r = (self.net_width / 2.0) * (i as f64 / 4.0);
            context.begin_path();
            context.arc(end_x, end_y, r, 0.0, 2.0 * PI)?;
            context.stroke();
        }

        // Draw rotation direction indicator
        let indicator_length = self.radius + 15.0;
        let indicator_x = self.x + indicator_length * self.net_angle.cos();
        let indicator_y = self.y + indicator_length * self.net_angle.sin();

        context.set_fill_style(&JsValue::from_str("#ff6b6b"));
        context.begin_path();
        context.arc(indicator_x, indicator_y, 5.0, 0.0, 2.0 * PI)?;
        context.fill();

        Ok(())
    }

    pub fn check_net_collision(&self, number: &NumberObject) -> bool {
        let length = self.net_length;
        let net_end_x = self.x + length * self.net_angle.cos();
        let net_end_y = self.y + length * self.net_angle.sin();

        // Check distance from net end to number
        let dx = net_end_x - number.x;
        let dy = net_end_y - number.y;
        let distance = (dx * dx + dy * dy).sqrt();

        distance < (self.net_width / 2.0 + number.radius)
    }

    pub fn get_hp_percentage(&self) -> f64 {
        (self.hp / self.max_hp * 100.0).round()
    }
}
