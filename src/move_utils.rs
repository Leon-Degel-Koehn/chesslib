use crate::square_utils::*;
use fen::{BoardState, Color, PieceKind};

#[derive(PartialEq, Clone)]
pub struct Move {
    pub start_square: usize,
    pub end_square: usize,
    pub promotion: Option<PieceKind>,
    pub is_en_passant: bool,

    // indicates if the move is a castling move of the king
    // in such a case the start_square and end_square are those
    // of the king as to differentiate short and long castles
    pub castles: bool,
}

impl std::fmt::Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            square_to_string(self.start_square),
            square_to_string(self.end_square)
        )
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            square_to_string(self.start_square),
            square_to_string(self.end_square)
        )
    }
}

pub trait ToggleColor {
    fn inverse_color(&self) -> Self;
}

impl ToggleColor for Color {
    fn inverse_color(&self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

impl Move {
    // Generate a standard move from a starting square to an end square
    // without captures, checks etc.
    pub fn standard(start_square: usize, end_square: usize) -> Self {
        return Self {
            start_square,
            end_square,
            promotion: None,
            is_en_passant: false,
            castles: false,
        };
    }

    pub fn from_long_algebraic(move_str: String, board: &BoardState) -> Move {
        let start_square = square_from_string(&move_str[0..2]);
        let end_square = square_from_string(&move_str[2..4]);
        let is_en_passant = board
            .en_passant_square
            .is_some_and(|s| s as usize == end_square);
        let castles = board.pieces[start_square]
            .clone()
            .is_some_and(|p| p.kind == PieceKind::King)
            && match board.side_to_play {
                Color::White => start_square == 4 && start_square.abs_diff(end_square) == 2,
                Color::Black => start_square == 60 && start_square.abs_diff(end_square) == 2,
            };
        let promotion = if move_str.len() > 4 {
            match move_str.chars().collect::<Vec<char>>()[4] {
                'q' => Some(PieceKind::Queen),
                'r' => Some(PieceKind::Rook),
                'n' => Some(PieceKind::Knight),
                'b' => Some(PieceKind::Bishop),
                _ => panic!("Chesslib was asked to promote a pawn to an invalid piece"),
            }
        } else {
            None
        };
        return Move {
            start_square,
            end_square,
            promotion,
            is_en_passant,
            castles,
        };
    }

    pub fn to_long_algebraic(&self) -> String {
        let mut promotion = "";
        if let Some(promotion_piece) = &self.promotion {
            promotion = match promotion_piece {
                PieceKind::Rook => "r",
                PieceKind::Knight => "n",
                PieceKind::Bishop => "b",
                PieceKind::Queen => "q",
                _ => "",
            }
        }
        return format!(
            "{}{}{}",
            square_to_string(self.start_square),
            square_to_string(self.end_square),
            promotion,
        );
    }

    pub fn execute(&self, board: &mut BoardState) {
        match &board.pieces[self.start_square] {
            Some(piece) => {
                let mut end_piece = piece.clone();
                if let Some(promotion_piece) = &self.promotion {
                    end_piece.kind = promotion_piece.clone();
                }
                if piece.kind == PieceKind::King {
                    match board.side_to_play {
                        Color::White => {
                            board.white_can_oo = false;
                            board.white_can_ooo = false;
                        }
                        Color::Black => {
                            board.black_can_oo = false;
                            board.black_can_ooo = false;
                        }
                    }
                }
                if piece.kind == PieceKind::Rook {
                    match board.side_to_play {
                        Color::White => {
                            if self.start_square == 0 {
                                board.white_can_ooo = false;
                            }
                            if self.start_square == 7 {
                                board.white_can_oo = false;
                            }
                        }
                        Color::Black => {
                            if self.start_square == 56 {
                                board.white_can_ooo = false;
                            }
                            if self.start_square == 63 {
                                board.white_can_oo = false;
                            }
                        }
                    }
                }
                board.pieces[self.start_square] = None;
                board.pieces[self.end_square] = Some(end_piece);
                if self.is_en_passant {
                    board.pieces[board.en_passant_square.unwrap() as usize] = None;
                }
                if self.castles {
                    let dir = (self.end_square as i32) - (self.start_square as i32);
                    if dir.is_positive() {
                        match board.side_to_play {
                            Color::White => board.white_can_oo = false,
                            Color::Black => board.black_can_oo = false,
                        }
                        let rook_square = self.start_square + 3;
                        board.pieces[self.end_square - 1] = board.pieces[rook_square].clone();
                        board.pieces[rook_square] = None;
                    } else {
                        match board.side_to_play {
                            Color::White => board.white_can_ooo = false,
                            Color::Black => board.black_can_ooo = false,
                        }
                        let rook_square = self.start_square - 4;
                        board.pieces[self.end_square + 1] = board.pieces[rook_square].clone();
                        board.pieces[rook_square] = None;
                    }
                }
                board.side_to_play = board.side_to_play.inverse_color();
            }
            None => return,
        }
    }
}
