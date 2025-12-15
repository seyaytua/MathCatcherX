use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use std::f64::consts::PI;
use rand::Rng;
use crate::math_problem::MathProblem;

pub struct NumberObject {
    pub x: f64,
    pub y: f64,
    pub value: u32,
    pub radius: f64,
    pub color: String,
    pub is_correct: bool,
    pub collected: bool,
}

impl NumberObject {
    pub fn new(x: f64, y: f64, value: u32, is_correct: bool) -> Self {
        let color = if is_correct {
            "#4ecdc4".to_string() // Cyan for correct answers
        } else {
            "#95a5a6".to_string() // Gray for other numbers
        };

        NumberObject {
            x,
            y,
            value,
            radius: 35.0,
            color,
            is_correct,
            collected: false,
        }
    }

    pub fn draw(&self, context: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        if self.collected {
            return Ok(()); // Don't draw collected numbers
        }

        // Draw number circle
        context.set_fill_style(&JsValue::from_str(&self.color));
        context.begin_path();
        context.arc(self.x, self.y, self.radius, 0.0, 2.0 * PI)?;
        context.fill();

        // Draw border
        context.set_stroke_style(&JsValue::from_str("#ffffff"));
        context.set_line_width(3.0);
        context.stroke();

        // Draw number text
        context.set_fill_style(&JsValue::from_str("#ffffff"));
        context.set_font("bold 24px Arial");
        context.set_text_align("center");
        context.set_text_baseline("middle");
        let text = format!("{}", self.value);
        context.fill_text(&text, self.x, self.y)?;

        Ok(())
    }

    pub fn generate_for_problem(problem: &MathProblem, width: f64, height: f64) -> Vec<NumberObject> {
        let mut numbers = Vec::new();
        let mut rng = rand::thread_rng();
        let margin = 80.0;
        let min_distance = 90.0;

        // Helper function to check if position is valid
        let is_valid_position = |x: f64, y: f64, existing: &Vec<NumberObject>| -> bool {
            for num in existing {
                let dx = x - num.x;
                let dy = y - num.y;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist < min_distance {
                    return false;
                }
            }
            true
        };

        // Add correct answers
        for &answer in &problem.correct_answers {
            let mut attempts = 0;
            loop {
                let x = rng.gen::<f64>() * (width - 2.0 * margin) + margin;
                let y = rng.gen::<f64>() * (height - 2.0 * margin) + margin;
                
                if is_valid_position(x, y, &numbers) {
                    numbers.push(NumberObject::new(x, y, answer, true));
                    break;
                }
                
                attempts += 1;
                if attempts > 100 {
                    // Fallback: just place it somewhere
                    numbers.push(NumberObject::new(x, y, answer, true));
                    break;
                }
            }
        }

        // Add some incorrect numbers
        let incorrect_count = 10;
        for _ in 0..incorrect_count {
            let value = rng.gen_range(1..50);
            
            // Make sure it's not a correct answer
            if !problem.correct_answers.contains(&value) {
                let mut attempts = 0;
                loop {
                    let x = rng.gen::<f64>() * (width - 2.0 * margin) + margin;
                    let y = rng.gen::<f64>() * (height - 2.0 * margin) + margin;
                    
                    if is_valid_position(x, y, &numbers) {
                        numbers.push(NumberObject::new(x, y, value, false));
                        break;
                    }
                    
                    attempts += 1;
                    if attempts > 100 {
                        break; // Skip this number if can't find valid position
                    }
                }
            }
        }

        numbers
    }
}
