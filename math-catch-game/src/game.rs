use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, window};
use crate::number::NumberObject;
use crate::math_problem::{MathProblem, ProblemType};
use rand::Rng;

pub struct Game {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    numbers: Vec<NumberObject>,
    problem: MathProblem,
    score: u32,
    time_remaining: f64,
    last_time: f64,
    game_over: bool,
    hp: f64,
    max_hp: f64,
}

impl Game {
    pub fn new(canvas: HtmlCanvasElement, context: CanvasRenderingContext2d) -> Result<Self, JsValue> {
        let width = canvas.width() as f64;
        let height = canvas.height() as f64;
        
        // Random problem type
        let problem = Self::random_problem();
        let numbers = NumberObject::generate_in_grid(&problem, width, height);
        
        Ok(Game {
            canvas,
            context,
            numbers,
            problem,
            score: 0,
            time_remaining: 60.0,
            last_time: 0.0,
            game_over: false,
            hp: 100.0,
            max_hp: 100.0,
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

        self.time_remaining -= delta_time;

        // Check if HP is depleted or time is up
        if self.hp <= 0.0 || self.time_remaining <= 0.0 {
            self.game_over = true;
            return;
        }

        // Check if all correct numbers are collected
        let all_correct_collected = self.numbers.iter()
            .filter(|n| n.is_correct)
            .all(|n| n.collected);

        if all_correct_collected {
            // Generate new problem
            let width = self.canvas.width() as f64;
            let height = self.canvas.height() as f64;
            self.problem = Self::random_problem();
            self.numbers = NumberObject::generate_in_grid(&self.problem, width, height);
            self.score += 200; // Bonus for completing problem
            
            // Heal player
            self.hp = (self.hp + 30.0).min(self.max_hp);
        }
    }

    pub fn render(&self) -> Result<(), JsValue> {
        let width = self.canvas.width() as f64;
        let height = self.canvas.height() as f64;

        // Clear canvas with light background
        self.context.set_fill_style(&JsValue::from_str("#f5f7fa"));
        self.context.fill_rect(0.0, 0.0, width, height);

        // Draw grid
        self.draw_grid(width, height)?;

        // Draw numbers
        for number in &self.numbers {
            number.draw(&self.context)?;
        }

        // Draw UI
        self.draw_ui(width, height)?;

        Ok(())
    }

    fn draw_grid(&self, width: f64, height: f64) -> Result<(), JsValue> {
        self.context.set_stroke_style(&JsValue::from_str("#d0d7de"));
        self.context.set_line_width(1.5);

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
        // Draw large target number in center top (if applicable)
        self.context.set_fill_style(&JsValue::from_str("#1a1a1a"));
        self.context.set_text_align("center");
        self.context.set_text_baseline("top");
        
        // For primes, show nothing; for modular, show "mod X ≡ Y"
        let (target_text, font_size) = match self.problem.problem_type {
            ProblemType::Primes => (String::new(), 72),
            ProblemType::Modular => {
                if self.problem.numbers.len() >= 2 {
                    (format!("mod {} ≡ {}", self.problem.numbers[0], self.problem.numbers[1]), 48)
                } else {
                    (String::new(), 72)
                }
            },
            ProblemType::Gcd | ProblemType::Lcm => {
                if self.problem.numbers.len() >= 2 {
                    (format!("{} & {}", self.problem.numbers[0], self.problem.numbers[1]), 56)
                } else {
                    (format!("{}", self.problem.target_number), 72)
                }
            },
            _ => (format!("{}", self.problem.target_number), 72),
        };
        
        if !target_text.is_empty() {
            let font = format!("bold {}px Arial", font_size);
            self.context.set_font(&font);
            self.context.fill_text(&target_text, width / 2.0, 20.0)?;
        }

        // Draw problem description below the number
        self.context.set_fill_style(&JsValue::from_str("#1a1a1a"));
        self.context.set_font("bold 20px Arial");
        self.context.set_text_align("center");
        let problem_text = self.problem.get_description();
        self.context.fill_text(&problem_text, width / 2.0, 105.0)?;

        // Draw score (left side)
        self.context.set_fill_style(&JsValue::from_str("#1a1a1a"));
        self.context.set_text_align("left");
        self.context.set_font("bold 20px Arial");
        let score_text = format!("Score: {}", self.score);
        self.context.fill_text(&score_text, 10.0, 20.0)?;

        // Draw time (left side)
        let time_text = format!("Time: {:.1}s", self.time_remaining);
        self.context.fill_text(&time_text, 10.0, 50.0)?;

        // Draw HP bar (right side)
        let hp_bar_width = 150.0;
        let hp_bar_height = 20.0;
        let hp_bar_x = width - hp_bar_width - 10.0;
        let hp_bar_y = 20.0;
        
        // HP bar background
        self.context.set_fill_style(&JsValue::from_str("#e1e4e8"));
        self.context.fill_rect(hp_bar_x, hp_bar_y, hp_bar_width, hp_bar_height);
        
        // HP bar fill with high contrast colors
        let hp_percentage = self.hp / self.max_hp;
        let hp_color = if hp_percentage > 0.6 {
            "#22863a"
        } else if hp_percentage > 0.3 {
            "#d29922"
        } else {
            "#d73a49"
        };
        self.context.set_fill_style(&JsValue::from_str(hp_color));
        self.context.fill_rect(hp_bar_x, hp_bar_y, hp_bar_width * hp_percentage, hp_bar_height);
        
        // HP bar border
        self.context.set_stroke_style(&JsValue::from_str("#1a1a1a"));
        self.context.set_line_width(2.0);
        self.context.stroke_rect(hp_bar_x, hp_bar_y, hp_bar_width, hp_bar_height);
        
        // HP text
        self.context.set_fill_style(&JsValue::from_str("#ffffff"));
        self.context.set_font("bold 14px Arial");
        self.context.set_text_align("center");
        let hp_text = format!("HP: {:.0}%", (self.hp / self.max_hp * 100.0));
        self.context.fill_text(&hp_text, hp_bar_x + hp_bar_width / 2.0, hp_bar_y + 15.0)?;

        // Draw game over
        if self.game_over {
            self.context.set_fill_style(&JsValue::from_str("rgba(255, 255, 255, 0.95)"));
            self.context.fill_rect(0.0, 0.0, width, height);

            self.context.set_fill_style(&JsValue::from_str("#d73a49"));
            self.context.set_font("bold 48px Arial");
            self.context.fill_text("GAME OVER", width / 2.0 - 150.0, height / 2.0)?;

            self.context.set_fill_style(&JsValue::from_str("#1a1a1a"));
            self.context.set_font("bold 32px Arial");
            let final_score = format!("Final Score: {}", self.score);
            self.context.fill_text(&final_score, width / 2.0 - 120.0, height / 2.0 + 50.0)?;
            
            let reason = if self.hp <= 0.0 {
                "HP reached 0!"
            } else {
                "Time's up!"
            };
            self.context.set_font("bold 24px Arial");
            self.context.fill_text(reason, width / 2.0 - 80.0, height / 2.0 + 90.0)?;
        }

        Ok(())
    }

    pub fn handle_click(&mut self, x: f64, y: f64) {
        if self.game_over {
            return;
        }

        // Check if any number was clicked
        for number in &mut self.numbers {
            if number.is_clicked(x, y) && !number.collected {
                number.collected = true;
                
                if self.problem.is_correct_answer(number.value) {
                    // Correct answer
                    self.score += 100;
                } else {
                    // Wrong answer - take damage
                    self.hp = (self.hp - 20.0).max(0.0);
                }
                
                break; // Only process one click at a time
            }
        }
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn get_time_remaining(&self) -> u32 {
        self.time_remaining as u32
    }

    pub fn get_hp(&self) -> u32 {
        (self.hp / self.max_hp * 100.0) as u32
    }

    pub fn is_game_over(&self) -> bool {
        self.game_over
    }

    pub fn skip_problem(&mut self) {
        if self.game_over {
            return;
        }

        // Generate new problem without bonus
        let width = self.canvas.width() as f64;
        let height = self.canvas.height() as f64;
        self.problem = Self::random_problem();
        self.numbers = NumberObject::generate_in_grid(&self.problem, width, height);
        
        // Small HP penalty for skipping
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
