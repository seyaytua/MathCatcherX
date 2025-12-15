use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, window};
use std::cell::RefCell;
use std::rc::Rc;

mod game;
mod game_2p;
mod number;
mod math_problem;

use game::Game;
use game_2p::Game2P;

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

    pub fn handle_click(&mut self, x: f64, y: f64) {
        self.game.borrow_mut().handle_click(x, y);
    }

    pub fn skip_problem(&mut self) {
        self.game.borrow_mut().skip_problem();
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

#[wasm_bindgen]
pub struct Game2PWrapper {
    game: Rc<RefCell<Game2P>>,
}

#[wasm_bindgen]
impl Game2PWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str) -> Result<Game2PWrapper, JsValue> {
        let document = window().unwrap().document().unwrap();
        let canvas = document
            .get_element_by_id(canvas_id)
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()?;
        
        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;
        
        let game = Game2P::new(canvas, context)?;
        
        Ok(Game2PWrapper {
            game: Rc::new(RefCell::new(game)),
        })
    }

    pub fn start(&mut self) -> Result<(), JsValue> {
        self.game.borrow_mut().start()
    }

    pub fn update_and_render(&mut self) -> Result<(), JsValue> {
        self.game.borrow_mut().update_and_render()
    }

    pub fn handle_click_player1(&mut self, x: f64, y: f64) {
        self.game.borrow_mut().handle_click_player1(x, y);
    }

    pub fn handle_click_player2(&mut self, x: f64, y: f64) {
        self.game.borrow_mut().handle_click_player2(x, y);
    }

    pub fn skip_problem_player1(&mut self) {
        self.game.borrow_mut().skip_problem_player1();
    }

    pub fn skip_problem_player2(&mut self) {
        self.game.borrow_mut().skip_problem_player2();
    }

    pub fn get_player1_score(&self) -> u32 {
        self.game.borrow().get_player1_score()
    }

    pub fn get_player2_score(&self) -> u32 {
        self.game.borrow().get_player2_score()
    }

    pub fn get_player1_hp(&self) -> u32 {
        self.game.borrow().get_player1_hp()
    }

    pub fn get_player2_hp(&self) -> u32 {
        self.game.borrow().get_player2_hp()
    }

    pub fn get_player1_time(&self) -> u32 {
        self.game.borrow().get_player1_time()
    }

    pub fn get_player2_time(&self) -> u32 {
        self.game.borrow().get_player2_time()
    }

    pub fn is_game_over(&self) -> bool {
        self.game.borrow().is_game_over()
    }
}
