use crate::move_generation::*;
use crate::move_utils::ToggleColor;
use crate::Game;
use fen::{Color, PieceKind};

pub trait StateCheck {
    fn current_player_in_check(&self) -> bool;
    fn current_player_is_checkmate(&self) -> bool;
    fn current_player_is_stalemate(&self) -> bool;
    fn winner(&self) -> Option<Color>;
    fn is_draw(&self) -> bool;
    fn insufficient_material(&self) -> bool;
}

impl StateCheck for Game {
    fn current_player_in_check(&self) -> bool {
        let mut simulation_board = self.board.duplicate();
        simulation_board.side_to_play = Color::inverse_color(&simulation_board.side_to_play);
        return simulation_board.player_in_check();
    }
    fn current_player_is_checkmate(&self) -> bool {
        return self.board.legal_moves().len() == 0 && self.current_player_in_check();
    }
    fn current_player_is_stalemate(&self) -> bool {
        return self.board.legal_moves().len() == 0 && !self.current_player_in_check();
    }
    fn insufficient_material(&self) -> bool {
        for square in &self.board.pieces {
            if square.clone().is_some_and(|p| p.kind != PieceKind::King) {
                return false;
            }
        }
        return true;
    }
    fn winner(&self) -> Option<Color> {
        if !self.current_player_is_checkmate() {
            return None;
        }
        return Some(Color::inverse_color(&self.board.side_to_play));
    }
    fn is_draw(&self) -> bool {
        return self.board.fullmove_number >= 100
            || self.insufficient_material()
            || self.current_player_is_stalemate()
            || self.draw_by_repetition;
    }
}
