use crate::move_utils::Move;
pub use fen::*;
use std::collections::HashMap;

pub mod end_state;
pub mod move_generation;
pub mod move_utils;
pub mod square_utils;

pub struct Game {
    pub board: BoardState,
    pub previous_positions: HashMap<String, usize>,
    pub draw_by_repetition: bool,
}

impl Game {
    // Generate a new game from the standard starting position
    pub fn new() -> Self {
        Self::start_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }

    // Generate a new game starting from the given fen state
    pub fn start_from_fen(fen: &str) -> Self {
        Self {
            board: BoardState::from_fen(fen).expect("Game can't be constructed from invalid fen"),
            previous_positions: HashMap::from([(
                fen.to_string()
                    .split_whitespace()
                    .take(2)
                    .collect::<Vec<_>>()
                    .join(" "),
                1,
            )]),
            draw_by_repetition: false,
        }
    }

    pub fn execute_move(&mut self, mov: &Move) {
        mov.execute(&mut self.board);
        let position = self
            .board
            .to_fen()
            .split_whitespace()
            .take(2)
            .collect::<Vec<_>>()
            .join(" ");
        let mut new_val: usize = 1;
        if let Some(&val) = self.previous_positions.get(&position) {
            new_val = val + 1;
            if new_val >= 3 {
                self.draw_by_repetition = true;
            }
        }
        self.previous_positions.insert(position, new_val);
    }
}

impl Clone for Game {
    fn clone(&self) -> Self {
        Self {
            board: BoardState::from_fen(&self.board.to_fen()).unwrap(),
            previous_positions: self.previous_positions.clone(),
            draw_by_repetition: self.draw_by_repetition,
        }
    }
}
