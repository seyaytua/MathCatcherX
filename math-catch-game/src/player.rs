use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use std::f64::consts::PI;
use crate::number::NumberObject;

pub struct Player {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub net_angle: f64,
    pub net_length: f64,
    pub net_width: f64,
    pub move_speed: f64,
    pub hp: f64,
    pub max_hp: f64,
    pub moving_left: bool,
    pub moving_right: bool,
    pub moving_up: bool,
    pub moving_down: bool,
}

impl Player {
    pub fn new(x: f64, y: f64) -> Self {
        Player {
            x,
            y,
            radius: 30.0,
            net_angle: 0.0,
            net_length: 45.0,  // 1.5 times character radius (30 * 1.5)
            net_width: 15.0,   // Half of character radius (30 / 2)
            move_speed: 300.0, // pixels per second
            hp: 100.0,
            max_hp: 100.0,
            moving_left: false,
            moving_right: false,
            moving_up: false,
            moving_down: false,
        }
    }

    pub fn update(&mut self, delta_time: f64, canvas_width: f64, canvas_height: f64) {
        // Move player based on input
        let mut dx = 0.0;
        let mut dy = 0.0;

        if self.moving_left {
            dx -= 1.0;
        }
        if self.moving_right {
            dx += 1.0;
        }
        if self.moving_up {
            dy -= 1.0;
        }
        if self.moving_down {
            dy += 1.0;
        }

        // Normalize diagonal movement
        if dx != 0.0 || dy != 0.0 {
            let length = ((dx * dx + dy * dy) as f64).sqrt();
            dx /= length;
            dy /= length;

            self.x += dx * self.move_speed * delta_time;
            self.y += dy * self.move_speed * delta_time;

            // Keep player within bounds
            self.x = self.x.clamp(self.radius, canvas_width - self.radius);
            self.y = self.y.clamp(self.radius, canvas_height - self.radius);
        }
    }

    pub fn rotate_net(&mut self) {
        // Rotate net by 30 degrees clockwise
        self.net_angle += 30.0 * PI / 180.0;
        if self.net_angle >= 2.0 * PI {
            self.net_angle -= 2.0 * PI;
        }
    }

    pub fn set_moving(&mut self, direction: &str, moving: bool) {
        match direction {
            "left" => self.moving_left = moving,
            "right" => self.moving_right = moving,
            "up" => self.moving_up = moving,
            "down" => self.moving_down = moving,
            _ => {}
        }
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
        let end_x = self.x + self.net_length * self.net_angle.cos();
        let end_y = self.y + self.net_length * self.net_angle.sin();

        // Draw net line
        context.set_stroke_style(&JsValue::from_str("#ffdd00"));
        context.set_line_width(self.net_width);
        context.set_line_cap("round");
        context.begin_path();
        context.move_to(self.x, self.y);
        context.line_to(end_x, end_y);
        context.stroke();

        // Draw net end circle
        context.set_fill_style(&JsValue::from_str("#ffdd00"));
        context.begin_path();
        context.arc(end_x, end_y, self.net_width / 2.0, 0.0, 2.0 * PI)?;
        context.fill();

        // Draw net border
        context.set_stroke_style(&JsValue::from_str("#cc9900"));
        context.set_line_width(2.0);
        context.stroke();

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
