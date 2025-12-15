use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, window};
use std::f64::consts::PI;
use crate::player::Player;
use crate::number::NumberObject;
use crate::math_problem::{MathProblem, ProblemType};

pub struct Game {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    player: Player,
    numbers: Vec<NumberObject>,
    problem: MathProblem,
    score: u32,
    time_remaining: f64,
    game_time: f64,
    last_time: f64,
    game_over: bool,
    animation_id: Option<i32>,
}

impl Game {
    pub fn new(canvas: HtmlCanvasElement, context: CanvasRenderingContext2d) -> Result<Self, JsValue> {
        let width = canvas.width() as f64;
        let height = canvas.height() as f64;
        
        let player = Player::new(width / 2.0, height / 2.0);
        let problem = MathProblem::new(ProblemType::Gcd);
        let numbers = NumberObject::generate_for_problem(&problem, width, height);
        
        Ok(Game {
            canvas,
            context,
            player,
            numbers,
            problem,
            score: 0,
            time_remaining: 60.0,
            game_time: 0.0,
            last_time: 0.0,
            game_over: false,
            animation_id: None,
        })
    }

    pub fn start(&mut self) -> Result<(), JsValue> {
        let window = window().unwrap();
        let performance = window.performance().unwrap();
        self.last_time = performance.now();
        Ok(())
    }

    pub fn update_and_render(&mut self) -> Result<(), JsValue> {
        let window = window().unwrap();
        let performance = window.performance().unwrap();
        let current_time = performance.now();
        let delta_time = (current_time - self.last_time) / 1000.0;
        self.last_time = current_time;

        if !self.game_over {
            self.update(delta_time);
            self.render()?;
        }
        
        Ok(())
    }

    fn update(&mut self, delta_time: f64) {
        if self.game_over {
            return;
        }

        self.game_time += delta_time;
        self.time_remaining -= delta_time;

        if self.time_remaining <= 0.0 {
            self.game_over = true;
            return;
        }

        // Update player
        self.player.update(delta_time);

        // Update numbers
        let width = self.canvas.width() as f64;
        let height = self.canvas.height() as f64;
        
        for number in &mut self.numbers {
            number.update(delta_time, width, height);
        }

        // Check collisions with net
        if self.player.is_net_extended() {
            let mut captured_indices = Vec::new();
            
            for (i, number) in self.numbers.iter().enumerate() {
                if self.player.check_net_collision(&number) {
                    captured_indices.push(i);
                }
            }

            // Process captured numbers
            for &i in captured_indices.iter().rev() {
                let number = &self.numbers[i];
                if self.problem.is_correct_answer(number.value) {
                    self.score += 100;
                    self.numbers.remove(i);
                } else {
                    // Wrong answer - penalty
                    if self.score >= 50 {
                        self.score -= 50;
                    }
                }
            }

            // Check if problem is solved
            if self.numbers.is_empty() || 
               self.numbers.iter().all(|n| !self.problem.is_correct_answer(n.value)) {
                // Generate new problem
                self.problem = MathProblem::new(ProblemType::Gcd);
                self.numbers = NumberObject::generate_for_problem(&self.problem, width, height);
                self.score += 200; // Bonus for completing problem
            }
        }
    }

    pub fn render(&self) -> Result<(), JsValue> {
        let width = self.canvas.width() as f64;
        let height = self.canvas.height() as f64;

        // Clear canvas
        self.context.set_fill_style(&JsValue::from_str("#1a1a2e"));
        self.context.fill_rect(0.0, 0.0, width, height);

        // Draw grid
        self.draw_grid(width, height)?;

        // Draw numbers
        for number in &self.numbers {
            number.draw(&self.context)?;
        }

        // Draw player
        self.player.draw(&self.context)?;

        // Draw UI
        self.draw_ui(width, height)?;

        Ok(())
    }

    fn draw_grid(&self, width: f64, height: f64) -> Result<(), JsValue> {
        self.context.set_stroke_style(&JsValue::from_str("#2a2a3e"));
        self.context.set_line_width(1.0);

        let grid_size = 50.0;
        
        self.context.begin_path();
        let mut x = 0.0;
        while x <= width {
            self.context.move_to(x, 0.0);
            self.context.line_to(x, height);
            x += grid_size;
        }
        
        let mut y = 0.0;
        while y <= height {
            self.context.move_to(0.0, y);
            self.context.line_to(width, y);
            y += grid_size;
        }
        self.context.stroke();

        Ok(())
    }

    fn draw_ui(&self, width: f64, height: f64) -> Result<(), JsValue> {
        // Draw problem description
        self.context.set_fill_style(&JsValue::from_str("#ffffff"));
        self.context.set_font("bold 20px Arial");
        let problem_text = self.problem.get_description();
        self.context.fill_text(&problem_text, 10.0, 30.0)?;

        // Draw score
        self.context.set_font("bold 24px Arial");
        let score_text = format!("Score: {}", self.score);
        self.context.fill_text(&score_text, 10.0, 60.0)?;

        // Draw time
        let time_text = format!("Time: {:.1}s", self.time_remaining);
        self.context.fill_text(&time_text, 10.0, 90.0)?;

        // Draw game over
        if self.game_over {
            self.context.set_fill_style(&JsValue::from_str("rgba(0, 0, 0, 0.7)"));
            self.context.fill_rect(0.0, 0.0, width, height);

            self.context.set_fill_style(&JsValue::from_str("#ffdd00"));
            self.context.set_font("bold 48px Arial");
            self.context.fill_text("GAME OVER", width / 2.0 - 150.0, height / 2.0)?;

            self.context.set_font("bold 32px Arial");
            let final_score = format!("Final Score: {}", self.score);
            self.context.fill_text(&final_score, width / 2.0 - 120.0, height / 2.0 + 50.0)?;
        }

        Ok(())
    }

    pub fn handle_click(&mut self, x: f64, y: f64) {
        if self.game_over {
            return;
        }
        self.player.handle_click(x, y);
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn get_time_remaining(&self) -> u32 {
        self.time_remaining as u32
    }

    pub fn is_game_over(&self) -> bool {
        self.game_over
    }
}
