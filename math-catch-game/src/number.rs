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
    pub is_correct: bool,
    pub collected: bool,
}

impl NumberObject {
    pub fn new_with_radius(x: f64, y: f64, value: u32, is_correct: bool, radius: f64) -> Self {
        NumberObject {
            x,
            y,
            value,
            radius,
            is_correct,
            collected: false,
        }
    }
}

impl NumberObject {
    pub fn new(x: f64, y: f64, value: u32, is_correct: bool) -> Self {
        NumberObject {
            x,
            y,
            value,
            radius: 35.0,
            is_correct,
            collected: false,
        }
    }

    pub fn draw(&self, context: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        if self.collected {
            return Ok(()); // Don't draw collected numbers
        }

        // Draw number circle (high contrast blue for all)
        context.set_fill_style(&JsValue::from_str("#0969da")); // High contrast blue
        context.begin_path();
        context.arc(self.x, self.y, self.radius, 0.0, 2.0 * PI)?;
        context.fill();

        // Draw border with high contrast
        context.set_stroke_style(&JsValue::from_str("#1a1a1a"));
        context.set_line_width(3.0);
        context.stroke();

        // Draw number text with dynamic font size (very conservative sizing)
        context.set_fill_style(&JsValue::from_str("#ffffff"));
        let font_size = (self.radius * 0.50).max(8.0); // Further reduced: 0.50 ratio, min 8px
        let font = format!("bold {}px Arial", font_size as i32);
        context.set_font(&font);
        context.set_text_align("center");
        context.set_text_baseline("middle");
        let text = format!("{}", self.value);
        context.fill_text(&text, self.x, self.y)?;

        Ok(())
    }

    pub fn generate_in_grid(problem: &MathProblem, width: f64, height: f64) -> Vec<NumberObject> {
        let mut numbers = Vec::new();
        let mut rng = rand::thread_rng();
        
        // Calculate grid layout
        let cols = 5;
        let rows = 4;
        let cell_width = width / cols as f64;
        let cell_height = height / rows as f64;
        let start_y = 0.0;
        
        // Calculate appropriate radius based on cell size - VERY conservative sizing
        // Use divisor of 6.0 for very small circles that definitely won't overlap
        // Target: circle diameter should be less than 33% of cell size
        let base_radius = cell_width.min(cell_height) / 6.0;
        // Cap at 20px for guaranteed fit on all screens
        let max_radius = base_radius.min(20.0);
        
        // Debug: Log the calculated values
        web_sys::console::log_1(&format!("Grid: width={}, height={}, cell={}x{}, base_radius={:.1}, max_radius={:.1}", 
            width, height, cell_width, cell_height, base_radius, max_radius).into());
        
        // Collect all numbers to display (correct + incorrect)
        let mut all_values = Vec::new();
        
        // Add correct answers
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
        
        // Count correct answers
        let correct_count = all_values.iter().filter(|(_, is_correct)| *is_correct).count();
        web_sys::console::log_1(&format!("📋 Generated {} numbers ({} correct, {} incorrect)", 
            all_values.len(), correct_count, all_values.len() - correct_count).into());
        
        // Place numbers in grid with calculated radius
        for (i, (value, is_correct)) in all_values.iter().enumerate().take(cols * rows) {
            let col = i % cols;
            let row = i / cols;
            
            let x = (col as f64 + 0.5) * cell_width;
            let y = start_y + (row as f64 + 0.5) * cell_height;
            
            numbers.push(NumberObject::new_with_radius(x, y, *value, *is_correct, max_radius));
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
