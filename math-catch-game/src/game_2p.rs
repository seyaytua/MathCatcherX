use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, window};
use crate::number::NumberObject;
use crate::math_problem::{MathProblem, ProblemType};
use rand::Rng;

pub struct Player {
    pub score: u32,
    pub hp: f64,
    pub max_hp: f64,
    pub time_remaining: f64,
    pub problem: MathProblem,
    pub numbers: Vec<NumberObject>,
    pub game_over: bool,
}

impl Player {
    pub fn new(problem: MathProblem, width: f64, height: f64) -> Self {
        let numbers = NumberObject::generate_in_grid(&problem, width, height);
        Player {
            score: 0,
            hp: 100.0,
            max_hp: 100.0,
            time_remaining: 60.0,
            problem,
            numbers,
            game_over: false,
        }
    }

    pub fn update(&mut self, delta_time: f64, width: f64, height: f64) {
        if self.game_over {
            return;
        }

        self.time_remaining -= delta_time;

        if self.hp <= 0.0 || self.time_remaining <= 0.0 {
            self.game_over = true;
            return;
        }

        // Check if all correct numbers are collected
        let all_correct_collected = self.numbers.iter()
            .filter(|n| n.is_correct)
            .all(|n| n.collected);

        if all_correct_collected {
            self.problem = Self::random_problem();
            self.numbers = NumberObject::generate_in_grid(&self.problem, width, height);
            self.score += 200;
            self.hp = (self.hp + 30.0).min(self.max_hp);
        }
    }

    pub fn handle_click(&mut self, x: f64, y: f64) {
        if self.game_over {
            return;
        }

        for number in &mut self.numbers {
            if number.is_clicked(x, y) && !number.collected {
                number.collected = true;
                
                if self.problem.is_correct_answer(number.value) {
                    self.score += 100;
                } else {
                    self.hp = (self.hp - 20.0).max(0.0);
                }
                
                break;
            }
        }
    }

    pub fn skip_problem(&mut self, width: f64, height: f64) {
        if self.game_over {
            return;
        }

        self.problem = Self::random_problem();
        self.numbers = NumberObject::generate_in_grid(&self.problem, width, height);
        self.hp = (self.hp - 10.0).max(0.0);
    }

    fn random_problem() -> MathProblem {
        let mut rng = rand::thread_rng();
        let problem_types = [
            ProblemType::Divisors,
            ProblemType::Multiples,
            ProblemType::Primes,
            ProblemType::Gcd,
            ProblemType::Lcm,
            ProblemType::Modular,
        ];
        let problem_type = problem_types[rng.gen_range(0..problem_types.len())];
        MathProblem::new(problem_type)
    }
}

pub struct Game2P {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    player1: Player,
    player2: Player,
    last_time: f64,
    winner: Option<u8>, // None, Some(1), or Some(2)
}

impl Game2P {
    pub fn new(canvas: HtmlCanvasElement, context: CanvasRenderingContext2d) -> Result<Self, JsValue> {
        let width = canvas.width() as f64;
        let height = (canvas.height() / 2) as f64;
        
        let problem1 = Player::random_problem();
        let problem2 = Player::random_problem();
        
        let player1 = Player::new(problem1, width, height);
        let player2 = Player::new(problem2, width, height);
        
        Ok(Game2P {
            canvas,
            context,
            player1,
            player2,
            last_time: 0.0,
            winner: None,
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

        if self.winner.is_none() {
            let width = self.canvas.width() as f64;
            let height = (self.canvas.height() / 2) as f64;
            
            self.player1.update(delta_time, width, height);
            self.player2.update(delta_time, width, height);
            
            // Check for winner
            if self.player1.game_over && self.player2.game_over {
                self.winner = if self.player1.score > self.player2.score {
                    Some(1)
                } else if self.player2.score > self.player1.score {
                    Some(2)
                } else {
                    Some(0) // Draw
                };
            }
            
            self.render()?;
        }
        
        Ok(())
    }

    fn render(&self) -> Result<(), JsValue> {
        let width = self.canvas.width() as f64;
        let height = self.canvas.height() as f64;
        let half_height = height / 2.0;

        // Clear canvas
        self.context.set_fill_style(&JsValue::from_str("#f5f7fa"));
        self.context.fill_rect(0.0, 0.0, width, height);

        // Draw dividing line
        self.context.set_stroke_style(&JsValue::from_str("#1a1a1a"));
        self.context.set_line_width(3.0);
        self.context.begin_path();
        self.context.move_to(0.0, half_height);
        self.context.line_to(width, half_height);
        self.context.stroke();

        // Render Player 1 (top half)
        self.context.save();
        self.render_player(&self.player1, 0.0, width, half_height, "Player 1")?;
        self.context.restore();

        // Render Player 2 (bottom half) - upside down
        self.context.save();
        self.context.translate(width, height);
        self.context.rotate(std::f64::consts::PI);
        self.render_player(&self.player2, 0.0, width, half_height, "Player 2")?;
        self.context.restore();

        // Draw winner overlay if game is over
        if let Some(winner) = self.winner {
            self.context.set_fill_style(&JsValue::from_str("rgba(255, 255, 255, 0.95)"));
            self.context.fill_rect(0.0, 0.0, width, height);

            self.context.set_fill_style(&JsValue::from_str("#0969da"));
            self.context.set_font("bold 48px Arial");
            self.context.set_text_align("center");
            self.context.set_text_baseline("middle");
            
            let text = match winner {
                1 => "Player 1 Wins!",
                2 => "Player 2 Wins!",
                _ => "Draw!",
            };
            self.context.fill_text(text, width / 2.0, height / 2.0)?;
        }

        Ok(())
    }

    fn render_player(&self, player: &Player, y_offset: f64, width: f64, height: f64, label: &str) -> Result<(), JsValue> {
        // Draw grid
        self.context.set_stroke_style(&JsValue::from_str("#d0d7de"));
        self.context.set_line_width(1.0);
        
        let grid_size = 50.0;
        self.context.begin_path();
        let mut x = 0.0;
        while x <= width {
            self.context.move_to(x, y_offset);
            self.context.line_to(x, y_offset + height);
            x += grid_size;
        }
        
        let mut y = y_offset;
        while y <= y_offset + height {
            self.context.move_to(0.0, y);
            self.context.line_to(width, y);
            y += grid_size;
        }
        self.context.stroke();

        // Draw numbers
        for number in &player.numbers {
            let adjusted_x = number.x;
            let adjusted_y = number.y + y_offset;
            
            if !number.collected {
                // Draw circle
                self.context.set_fill_style(&JsValue::from_str("#0969da"));
                self.context.begin_path();
                self.context.arc(adjusted_x, adjusted_y, number.radius, 0.0, 2.0 * std::f64::consts::PI)?;
                self.context.fill();
                
                // Draw border
                self.context.set_stroke_style(&JsValue::from_str("#1a1a1a"));
                self.context.set_line_width(3.0);
                self.context.stroke();
                
                // Draw text
                self.context.set_fill_style(&JsValue::from_str("#ffffff"));
                self.context.set_font("bold 20px Arial");
                self.context.set_text_align("center");
                self.context.set_text_baseline("middle");
                let text = format!("{}", number.value);
                self.context.fill_text(&text, adjusted_x, adjusted_y)?;
            }
        }

        // Draw player info
        self.context.set_fill_style(&JsValue::from_str("#1a1a1a"));
        self.context.set_font("bold 16px Arial");
        self.context.set_text_align("left");
        self.context.fill_text(label, 10.0, y_offset + 20.0)?;
        
        let score_text = format!("Score: {}", player.score);
        self.context.fill_text(&score_text, 10.0, y_offset + 40.0)?;
        
        let hp_text = format!("HP: {:.0}", player.hp);
        self.context.fill_text(&hp_text, 10.0, y_offset + 60.0)?;

        Ok(())
    }

    pub fn handle_click_player1(&mut self, x: f64, y: f64) {
        let height = (self.canvas.height() / 2) as f64;
        if y <= height {
            self.player1.handle_click(x, y);
        }
    }

    pub fn handle_click_player2(&mut self, x: f64, y: f64) {
        let height = (self.canvas.height() / 2) as f64;
        let canvas_height = self.canvas.height() as f64;
        if y > height {
            // Transform coordinates for upside-down player 2
            let width = self.canvas.width() as f64;
            let transformed_x = width - x;
            let transformed_y = canvas_height - y;
            self.player2.handle_click(transformed_x, transformed_y);
        }
    }

    pub fn skip_problem_player1(&mut self) {
        let width = self.canvas.width() as f64;
        let height = (self.canvas.height() / 2) as f64;
        self.player1.skip_problem(width, height);
    }

    pub fn skip_problem_player2(&mut self) {
        let width = self.canvas.width() as f64;
        let height = (self.canvas.height() / 2) as f64;
        self.player2.skip_problem(width, height);
    }

    pub fn get_player1_score(&self) -> u32 {
        self.player1.score
    }

    pub fn get_player2_score(&self) -> u32 {
        self.player2.score
    }

    pub fn get_player1_hp(&self) -> u32 {
        (self.player1.hp / self.player1.max_hp * 100.0) as u32
    }

    pub fn get_player2_hp(&self) -> u32 {
        (self.player2.hp / self.player2.max_hp * 100.0) as u32
    }

    pub fn get_player1_time(&self) -> u32 {
        self.player1.time_remaining as u32
    }

    pub fn get_player2_time(&self) -> u32 {
        self.player2.time_remaining as u32
    }

    pub fn is_game_over(&self) -> bool {
        self.winner.is_some()
    }
}
