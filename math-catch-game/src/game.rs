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

        // Check if player is still alive
        if !self.player.is_alive() {
            self.game_over = true;
            return;
        }

        if self.time_remaining <= 0.0 {
            self.game_over = true;
            return;
        }

        // Update player
        let width = self.canvas.width() as f64;
        let height = self.canvas.height() as f64;
        self.player.update(delta_time, width, height);

        // Check collisions with net
        for number in &mut self.numbers {
            if !number.collected && self.player.check_net_collision(&number) {
                number.collected = true;
                
                if self.problem.is_correct_answer(number.value) {
                    // Correct answer
                    self.score += 100;
                } else {
                    // Wrong answer - take damage
                    self.player.take_damage(20.0);
                }
            }
        }

        // Check if all correct numbers are collected
        let all_correct_collected = self.numbers.iter()
            .filter(|n| n.is_correct)
            .all(|n| n.collected);

        if all_correct_collected {
            // Generate new problem
            self.problem = MathProblem::new(ProblemType::Gcd);
            self.numbers = NumberObject::generate_for_problem(&self.problem, width, height);
            self.score += 200; // Bonus for completing problem
            
            // Heal player a bit
            self.player.hp = (self.player.hp + 30.0).min(self.player.max_hp);
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
        // Draw HP bar
        let hp_bar_width = 200.0;
        let hp_bar_height = 20.0;
        let hp_bar_x = width - hp_bar_width - 20.0;
        let hp_bar_y = 20.0;
        
        // HP bar background
        self.context.set_fill_style(&JsValue::from_str("#333333"));
        self.context.fill_rect(hp_bar_x, hp_bar_y, hp_bar_width, hp_bar_height);
        
        // HP bar fill
        let hp_percentage = self.player.hp / self.player.max_hp;
        let hp_color = if hp_percentage > 0.6 {
            "#4ecdc4"
        } else if hp_percentage > 0.3 {
            "#ffdd00"
        } else {
            "#ff6b6b"
        };
        self.context.set_fill_style(&JsValue::from_str(hp_color));
        self.context.fill_rect(hp_bar_x, hp_bar_y, hp_bar_width * hp_percentage, hp_bar_height);
        
        // HP bar border
        self.context.set_stroke_style(&JsValue::from_str("#ffffff"));
        self.context.set_line_width(2.0);
        self.context.stroke_rect(hp_bar_x, hp_bar_y, hp_bar_width, hp_bar_height);
        
        // HP text
        self.context.set_fill_style(&JsValue::from_str("#ffffff"));
        self.context.set_font("bold 16px Arial");
        let hp_text = format!("HP: {:.0}%", self.player.get_hp_percentage());
        self.context.fill_text(&hp_text, hp_bar_x + hp_bar_width / 2.0 - 30.0, hp_bar_y + 15.0)?;

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
            
            let reason = if !self.player.is_alive() {
                "HP reached 0!"
            } else {
                "Time's up!"
            };
            self.context.set_font("bold 24px Arial");
            self.context.fill_text(reason, width / 2.0 - 80.0, height / 2.0 + 90.0)?;
        }

        Ok(())
    }

    pub fn handle_key_down(&mut self, key: &str) {
        if self.game_over {
            return;
        }
        
        match key {
            // Left: A, Arrow Left, or left side keys (Q, Z)
            "ArrowLeft" | "a" | "A" => {
                self.player.set_moving("left", true);
            }
            // Right: D, Arrow Right, or right side keys (H and beyond)
            "ArrowRight" | "d" | "D" => {
                self.player.set_moving("right", true);
            }
            // Up: W, Arrow Up, or top row keys
            "ArrowUp" | "w" | "W" => {
                self.player.set_moving("up", true);
            }
            // Down: S, Arrow Down, or bottom row keys
            "ArrowDown" | "s" | "S" => {
                self.player.set_moving("down", true);
            }
            // Additional left keys (Q row and Z row left of G)
            "q" | "Q" | "z" | "Z" | "e" | "E" | "r" | "R" | "f" | "F" | "g" | "G" => {
                if matches!(key, "q" | "Q" | "e" | "E") {
                    self.player.set_moving("up", true);
                }
                if matches!(key, "z" | "Z") {
                    self.player.set_moving("down", true);
                }
                if matches!(key, "q" | "Q" | "z" | "Z" | "f" | "F" | "g" | "G") {
                    self.player.set_moving("left", true);
                }
            }
            // Additional right keys (H and beyond)
            "h" | "H" | "j" | "J" | "k" | "K" | "l" | "L" |
            "u" | "U" | "i" | "I" | "o" | "O" | "p" | "P" |
            "x" | "X" | "c" | "C" | "v" | "V" | "b" | "B" | "n" | "N" | "m" | "M" => {
                if matches!(key, "u" | "U" | "i" | "I" | "o" | "O" | "p" | "P") {
                    self.player.set_moving("up", true);
                }
                if matches!(key, "x" | "X" | "c" | "C" | "v" | "V" | "b" | "B" | "n" | "N" | "m" | "M") {
                    self.player.set_moving("down", true);
                }
                self.player.set_moving("right", true);
            }
            // Rotate net: Space, Enter
            " " | "Enter" => {
                self.player.rotate_net();
            }
            _ => {}
        }
    }

    pub fn handle_key_up(&mut self, key: &str) {
        if self.game_over {
            return;
        }
        
        match key {
            "ArrowLeft" | "a" | "A" => {
                self.player.set_moving("left", false);
            }
            "ArrowRight" | "d" | "D" => {
                self.player.set_moving("right", false);
            }
            "ArrowUp" | "w" | "W" => {
                self.player.set_moving("up", false);
            }
            "ArrowDown" | "s" | "S" => {
                self.player.set_moving("down", false);
            }
            "q" | "Q" | "z" | "Z" | "e" | "E" | "r" | "R" | "f" | "F" | "g" | "G" => {
                if matches!(key, "q" | "Q" | "e" | "E") {
                    self.player.set_moving("up", false);
                }
                if matches!(key, "z" | "Z") {
                    self.player.set_moving("down", false);
                }
                if matches!(key, "q" | "Q" | "z" | "Z" | "f" | "F" | "g" | "G") {
                    self.player.set_moving("left", false);
                }
            }
            "h" | "H" | "j" | "J" | "k" | "K" | "l" | "L" |
            "u" | "U" | "i" | "I" | "o" | "O" | "p" | "P" |
            "x" | "X" | "c" | "C" | "v" | "V" | "b" | "B" | "n" | "N" | "m" | "M" => {
                if matches!(key, "u" | "U" | "i" | "I" | "o" | "O" | "p" | "P") {
                    self.player.set_moving("up", false);
                }
                if matches!(key, "x" | "X" | "c" | "C" | "v" | "V" | "b" | "B" | "n" | "N" | "m" | "M") {
                    self.player.set_moving("down", false);
                }
                self.player.set_moving("right", false);
            }
            _ => {}
        }
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn get_time_remaining(&self) -> u32 {
        self.time_remaining as u32
    }

    pub fn get_hp(&self) -> u32 {
        self.player.get_hp_percentage() as u32
    }

    pub fn is_game_over(&self) -> bool {
        self.game_over
    }
}
