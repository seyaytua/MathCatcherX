use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use std::f64::consts::PI;
use rand::Rng;
use crate::math_problem::MathProblem;

pub enum NumberLabel {
    Number,    // 数
    Divisor,   // 約数
    Multiple,  // 倍数
}

pub struct NumberObject {
    pub x: f64,
    pub y: f64,
    pub value: u32,
    pub radius: f64,
    pub is_correct: bool,
    pub collected: bool,
    pub label: NumberLabel,
}

impl NumberObject {
    pub fn new(x: f64, y: f64, value: u32, is_correct: bool, label: NumberLabel) -> Self {
        NumberObject {
            x,
            y,
            value,
            radius: 35.0,
            is_correct,
            collected: false,
            label,
        }
    }

    pub fn draw(&self, context: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        if self.collected {
            return Ok(()); // Don't draw collected numbers
        }

        // Draw label above the number
        let label_text = match self.label {
            NumberLabel::Number => "数",
            NumberLabel::Divisor => "約数",
            NumberLabel::Multiple => "倍数",
        };
        
        context.set_fill_style(&JsValue::from_str("#ffffff"));
        context.set_font("bold 14px Arial");
        context.set_text_align("center");
        context.set_text_baseline("bottom");
        context.fill_text(label_text, self.x, self.y - self.radius - 5.0)?;

        // Draw number circle (same color for all)
        context.set_fill_style(&JsValue::from_str("#667eea")); // Purple color for all
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

    pub fn generate_in_grid(problem: &MathProblem, width: f64, height: f64, target_number: u32) -> Vec<NumberObject> {
        let mut numbers = Vec::new();
        let mut rng = rand::thread_rng();
        
        // Calculate grid layout
        let cols = 5;
        let rows = 4;
        let cell_width = width / cols as f64;
        let cell_height = (height - 120.0) / rows as f64; // Leave space for UI at top
        let start_y = 120.0;
        
        // Collect all numbers to display (correct + incorrect)
        let mut all_values = Vec::new();
        
        // Add correct answers (divisors or multiples)
        for &answer in &problem.correct_answers {
            all_values.push((answer, true));
        }
        
        // Add incorrect numbers
        let mut attempts = 0;
        while all_values.len() < (cols * rows) && attempts < 100 {
            let value = rng.gen_range(1..50);
            
            // Make sure it's not already in the list
            if !all_values.iter().any(|(v, _)| *v == value) {
                all_values.push((value, false));
            }
            attempts += 1;
        }
        
        // Shuffle the numbers
        use rand::seq::SliceRandom;
        all_values.shuffle(&mut rng);
        
        // Place numbers in grid with labels
        for (i, (value, is_correct)) in all_values.iter().enumerate().take(cols * rows) {
            let col = i % cols;
            let row = i / cols;
            
            let x = (col as f64 + 0.5) * cell_width;
            let y = start_y + (row as f64 + 0.5) * cell_height;
            
            // Determine label type
            let label = if *value == target_number {
                NumberLabel::Number
            } else if target_number % value == 0 {
                NumberLabel::Divisor
            } else if value % target_number == 0 {
                NumberLabel::Multiple
            } else {
                NumberLabel::Number
            };
            
            numbers.push(NumberObject::new(x, y, *value, *is_correct, label));
        }
        
        numbers
    }

    pub fn is_clicked(&self, click_x: f64, click_y: f64) -> bool {
        let dx = click_x - self.x;
        let dy = click_y - self.y;
        let distance = (dx * dx + dy * dy).sqrt();
        distance <= self.radius
    }
}
