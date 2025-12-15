use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use std::f64::consts::PI;
use rand::Rng;
use crate::math_problem::MathProblem;

pub struct NumberObject {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub value: u32,
    pub radius: f64,
    pub color: String,
    pub is_correct: bool,
}

impl NumberObject {
    pub fn new(x: f64, y: f64, value: u32, is_correct: bool) -> Self {
        let mut rng = rand::thread_rng();
        
        let speed = 50.0 + rng.gen::<f64>() * 100.0;
        let angle = rng.gen::<f64>() * 2.0 * PI;
        
        let color = if is_correct {
            "#4ecdc4".to_string() // Cyan for correct answers
        } else {
            "#95a5a6".to_string() // Gray for other numbers
        };

        NumberObject {
            x,
            y,
            vx: speed * angle.cos(),
            vy: speed * angle.sin(),
            value,
            radius: 30.0,
            color,
            is_correct,
        }
    }

    pub fn update(&mut self, delta_time: f64, width: f64, height: f64) {
        self.x += self.vx * delta_time;
        self.y += self.vy * delta_time;

        // Bounce off walls
        if self.x - self.radius < 0.0 || self.x + self.radius > width {
            self.vx = -self.vx;
            self.x = self.x.clamp(self.radius, width - self.radius);
        }

        if self.y - self.radius < 0.0 || self.y + self.radius > height {
            self.vy = -self.vy;
            self.y = self.y.clamp(self.radius, height - self.radius);
        }
    }

    pub fn draw(&self, context: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        // Draw number circle
        context.set_fill_style(&JsValue::from_str(&self.color));
        context.begin_path();
        context.arc(self.x, self.y, self.radius, 0.0, 2.0 * PI)?;
        context.fill();

        // Draw border
        context.set_stroke_style(&JsValue::from_str("#ffffff"));
        context.set_line_width(2.0);
        context.stroke();

        // Draw number text
        context.set_fill_style(&JsValue::from_str("#ffffff"));
        context.set_font("bold 20px Arial");
        context.set_text_align("center");
        context.set_text_baseline("middle");
        let text = format!("{}", self.value);
        context.fill_text(&text, self.x, self.y)?;

        Ok(())
    }

    pub fn generate_for_problem(problem: &MathProblem, width: f64, height: f64) -> Vec<NumberObject> {
        let mut numbers = Vec::new();
        let mut rng = rand::thread_rng();

        // Add correct answers
        for &answer in &problem.correct_answers {
            let x = rng.gen::<f64>() * (width - 100.0) + 50.0;
            let y = rng.gen::<f64>() * (height - 100.0) + 50.0;
            numbers.push(NumberObject::new(x, y, answer, true));
        }

        // Add some incorrect numbers
        let incorrect_count = 8;
        for _ in 0..incorrect_count {
            let value = rng.gen_range(1..50);
            
            // Make sure it's not a correct answer
            if !problem.correct_answers.contains(&value) {
                let x = rng.gen::<f64>() * (width - 100.0) + 50.0;
                let y = rng.gen::<f64>() * (height - 100.0) + 50.0;
                numbers.push(NumberObject::new(x, y, value, false));
            }
        }

        numbers
    }
}
