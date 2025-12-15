use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, window};
use std::cell::RefCell;
use std::rc::Rc;
use rand::Rng;

mod game;
mod player;
mod number;
mod math_problem;

use game::Game;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    Ok(())
}

#[wasm_bindgen]
pub struct GameWrapper {
    game: Rc<RefCell<Game>>,
}

#[wasm_bindgen]
impl GameWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str) -> Result<GameWrapper, JsValue> {
        let document = window().unwrap().document().unwrap();
        let canvas = document
            .get_element_by_id(canvas_id)
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()?;
        
        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;
        
        let game = Game::new(canvas, context)?;
        
        Ok(GameWrapper {
            game: Rc::new(RefCell::new(game)),
        })
    }

    pub fn start(&mut self) -> Result<(), JsValue> {
        self.game.borrow_mut().start()
    }

    pub fn update_and_render(&mut self) -> Result<(), JsValue> {
        self.game.borrow_mut().update_and_render()
    }

    pub fn handle_touch(&mut self, x: f64, y: f64) {
        self.game.borrow_mut().handle_touch(x, y);
    }

    pub fn get_score(&self) -> u32 {
        self.game.borrow().get_score()
    }

    pub fn get_time_remaining(&self) -> u32 {
        self.game.borrow().get_time_remaining()
    }

    pub fn get_hp(&self) -> u32 {
        self.game.borrow().get_hp()
    }

    pub fn is_game_over(&self) -> bool {
        self.game.borrow().is_game_over()
    }
}
