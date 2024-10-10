use wasm_bindgen::prelude::*;
use std::collections::HashMap;

// Define a struct for the player's position
#[wasm_bindgen]
#[derive(Clone, Debug, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// Define a struct for the game state
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct GameState {
    pub player_position: Position,
    collected_items: HashMap<String, bool>,
}

#[wasm_bindgen]
impl GameState {
    #[wasm_bindgen(constructor)]
    pub fn new() -> GameState {
        GameState {
            player_position: Position { x: 0.0, y: 0.0, z: 0.0 },
            collected_items: HashMap::new(),
        }
    }

    pub fn move_player(&mut self, x: f32, y: f32, z: f32) {
        self.player_position = Position { x, y, z };
        console_log(&format!("Player moved to: {:?}", self.player_position));
    }

    pub fn collect_item(&mut self, item_name: String) {
        self.collected_items.insert(item_name.clone(), true);
        console_log(&format!("Collected item: {}", item_name));
    }

    // Add a method to get the player's position
    pub fn get_position(&self) -> Position {
        self.player_position
    }

    // Method to get collected items as a Vec<String>
    pub fn get_collected_items(&self) -> Vec<String> {
        self.collected_items.keys().cloned().collect()
    }
}

// Logging function
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn console_log(msg: &str) {
    log(msg);
}
