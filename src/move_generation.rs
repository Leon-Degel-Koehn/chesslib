pub use crate::move_utils::*;
use crate::square_utils::*;
use fen::{BoardState, Color, Piece, PieceKind};

pub trait PieceLocalization {
    fn find_piece(&self, piece: Piece) -> Option<usize>;
}

impl PieceLocalization for BoardState {
    fn find_piece(&self, piece: Piece) -> Option<usize> {
        self.pieces.iter().position(|x| *x == Some(piece.clone()))
    }
}

pub trait MoveGeneration {
    // supposed to be public
    fn legal_moves(&self) -> Vec<Move>;
    // supposed to be private
    fn generate_moves(&self, castling: bool) -> Vec<Move>;
    fn generate_pawn_moves(&self, square: usize) -> Vec<Move>;
    fn generate_bishop_moves(&self, square: usize) -> Vec<Move>;
    fn generate_rook_moves(&self, square: usize) -> Vec<Move>;
    fn generate_queen_moves(&self, square: usize) -> Vec<Move>;
    fn generate_knight_moves(&self, square: usize) -> Vec<Move>;
    fn generate_king_moves(&self, square: usize, castling: bool) -> Vec<Move>;
    fn generate_kingside_castles(&self, square: usize) -> Vec<Move>;
    fn generate_queenside_castles(&self, square: usize) -> Vec<Move>;
    fn generate_directed_castle(&self, square: usize, direction: i32) -> Vec<Move>;
    fn generate_castling_moves(&self, square: usize) -> Vec<Move>;
    fn generate_pawn_captures(
        &self,
        square: usize,
        move_dir: i32,
        promotion_rank: usize,
    ) -> Vec<Move>;
    fn puts_self_in_check(&self, mov: &Move) -> bool;
    fn print_legal_moves(&self);
    fn player_in_check(&self) -> bool;
    fn straight_line_moves(
        &self,
        square: usize,
        move_dirs: Vec<(i32, i32)>,
        can_capture: bool,
        max_distance: Option<usize>,
    ) -> Vec<Move>;
}

pub trait Duplication {
    fn duplicate(&self) -> Self;
}

impl Duplication for BoardState {
    fn duplicate(&self) -> Self {
        BoardState::from_fen(&self.to_fen()).unwrap()
    }
}

impl MoveGeneration for BoardState {
    fn print_legal_moves(&self) {
        for mov in self.legal_moves() {
            print!(" {} ", mov);
        }
    }
    // checks if the current player can capture opponents king
    fn player_in_check(&self) -> bool {
        let ref_piece = Piece {
            kind: PieceKind::King,
            color: if self.side_to_play == Color::White {
                Color::Black
            } else {
                Color::White
            },
        };
        let king_square = self.find_piece(ref_piece);
        if !king_square.is_some() {
            // in real positions this will not happen, it would mean the player has no king
            // but for testing and calculation it can be convenient to consider positions where
            // we can ignore the kings
            return false;
        }
        let king_square = king_square.unwrap();
        let opponent_moves = self.generate_moves(false);
        return opponent_moves
            .iter()
            .any(|mov| mov.end_square as usize == king_square);
    }
    fn puts_self_in_check(&self, mov: &Move) -> bool {
        let mut simulation_board = self.duplicate();
        mov.execute(&mut simulation_board);
        return simulation_board.player_in_check();
    }
    fn legal_moves(&self) -> Vec<Move> {
        let mut moves = self.generate_moves(true);
        moves.retain(|mv| !self.puts_self_in_check(mv));
        return moves;
    }

    fn generate_pawn_captures(
        &self,
        square: usize,
        move_dir: i32,
        promotion_rank: usize,
    ) -> Vec<Move> {
        let mut moves = Vec::new();
        let promotion_pieces = vec![
            PieceKind::Queen,
            PieceKind::Knight,
            PieceKind::Bishop,
            PieceKind::Rook,
        ];
        for capture_dir in vec![1, -1] {
            let end_file = file(square) as i32 + capture_dir;
            let end_rank = rank(square) as i32 + move_dir;
            if !is_on_board(end_rank, end_file) {
                continue;
            }
            let end_square = add_rank(add_file(square, capture_dir) as usize, move_dir) as usize;
            let mut is_en_passant = false;
            if self.pieces[end_square].is_none() {
                if self.en_passant_square.is_none()
                    || self.en_passant_square.unwrap() != end_square as u8
                {
                    continue;
                }
                is_en_passant = true;
            }
            if rank(end_square) == promotion_rank {
                for promotion_target in &promotion_pieces {
                    moves.push(Move {
                        start_square: square,
                        end_square,
                        promotion: Some(promotion_target.clone()),
                        is_en_passant,
                        castles: false,
                    });
                }
            } else {
                moves.push(Move {
                    start_square: square,
                    end_square,
                    promotion: None,
                    is_en_passant,
                    castles: false,
                });
            }
        }
        return moves;
    }

    fn generate_pawn_moves(&self, square: usize) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        if !self.pieces[square]
            .clone()
            .is_some_and(|pc| pc.color == self.side_to_play)
        {
            return moves;
        }
        let promotion_pieces = vec![
            PieceKind::Queen,
            PieceKind::Knight,
            PieceKind::Bishop,
            PieceKind::Rook,
        ];
        let piece = self.pieces[square].clone().unwrap();
        let (move_dir, promotion_rank, start_rank): (i32, usize, usize) = match piece.color {
            Color::White => (1, 7, 1),
            Color::Black => (-1, 0, 6),
        };
        let num_legal_steps = if rank(square) == start_rank { 2 } else { 1 };
        for move_dist in 1..(num_legal_steps + 1) {
            let end_file = file(square);
            let end_rank = rank(square) as i32 + move_dir * move_dist;
            if !is_on_board(end_rank, end_file as i32) {
                break;
            }
            let target_square = add_rank(square as usize, move_dir * move_dist) as usize;
            if self.pieces[target_square].is_some() {
                break;
            }
            if rank(target_square) == promotion_rank {
                for promotion_target in &promotion_pieces {
                    moves.push(Move {
                        start_square: square,
                        end_square: target_square,
                        promotion: Some(promotion_target.clone()),
                        is_en_passant: false,
                        castles: false,
                    });
                }
            } else {
                moves.push(Move {
                    start_square: square,
                    end_square: target_square,
                    promotion: None,
                    is_en_passant: false,
                    castles: false,
                });
            }
        }
        moves.append(&mut self.generate_pawn_captures(square, move_dir, promotion_rank));
        return moves;
    }

    fn straight_line_moves(
        &self,
        square: usize,
        move_dirs: Vec<(i32, i32)>,
        can_capture: bool,
        max_distance: Option<usize>,
    ) -> Vec<Move> {
        let mut moves = Vec::new();
        for (file_inc, rank_inc) in move_dirs {
            let mut distance = 0;
            loop {
                distance += 1;
                let target_file = file(square) as i32 + file_inc * distance;
                let target_rank = rank(square) as i32 + rank_inc * distance;
                if !is_on_board(target_rank, target_file)
                    || max_distance.is_some_and(|d| distance as usize > d)
                {
                    break;
                }
                let target_square = add_rank(
                    add_file(square, file_inc * distance) as usize,
                    rank_inc * distance,
                ) as usize;
                if self.pieces[target_square].is_some() {
                    if can_capture
                        && self.pieces[target_square].clone().unwrap().color != self.side_to_play
                    {
                        moves.push(Move::standard(square, target_square));
                    }
                    break;
                }
                moves.push(Move::standard(square, target_square));
            }
        }
        return moves;
    }

    fn generate_bishop_moves(&self, square: usize) -> Vec<Move> {
        let move_dirs = vec![(1, 1), (1, -1), (-1, 1), (-1, -1)];
        return self.straight_line_moves(square, move_dirs, true, None);
    }

    fn generate_rook_moves(&self, square: usize) -> Vec<Move> {
        let move_dirs = vec![(1, 0), (0, -1), (-1, 0), (0, 1)];
        return self.straight_line_moves(square, move_dirs, true, None);
    }

    fn generate_queen_moves(&self, square: usize) -> Vec<Move> {
        let mut moves = self.generate_rook_moves(square);
        moves.append(&mut self.generate_bishop_moves(square));
        return moves;
    }

    fn generate_kingside_castles(&self, square: usize) -> Vec<Move> {
        return self.generate_directed_castle(square, 1);
    }

    fn generate_queenside_castles(&self, square: usize) -> Vec<Move> {
        return self.generate_directed_castle(square, -1);
    }

    fn generate_directed_castle(&self, square: usize, direction: i32) -> Vec<Move> {
        let mut moves = Vec::new();
        // player can still castle based on previous moves
        match self.side_to_play {
            Color::White => {
                if !self.white_can_ooo {
                    return moves;
                }
            }
            Color::Black => {
                if !self.black_can_ooo {
                    return moves;
                }
            }
        }
        // cannot castle out of check
        let mov = Move::standard(square, square);
        if self.puts_self_in_check(&mov) {
            return moves;
        }
        // some piece stands inbetween king and rook
        // or one would castle through a check
        let end_square = (square as i32 + direction) as usize;
        let mov = Move::standard(square, end_square);
        if self.pieces[end_square].is_some() || self.puts_self_in_check(&mov) {
            return moves;
        }
        moves.push(Move {
            start_square: square,
            end_square: (square as i32 + 2 * direction) as usize,
            promotion: None,
            castles: true,
            is_en_passant: false,
        });
        return moves;
    }

    fn generate_castling_moves(&self, square: usize) -> Vec<Move> {
        let mut moves = Vec::new();
        moves.append(&mut self.generate_kingside_castles(square));
        moves.append(&mut self.generate_queenside_castles(square));
        return moves;
    }

    fn generate_king_moves(&self, square: usize, castling: bool) -> Vec<Move> {
        let move_dirs = vec![
            (1, 0),
            (0, -1),
            (-1, 0),
            (0, 1),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ];
        let mut moves = self.straight_line_moves(square, move_dirs, true, Some(1));
        if castling {
            moves.append(&mut self.generate_castling_moves(square));
        }
        return moves;
    }

    fn generate_knight_moves(&self, square: usize) -> Vec<Move> {
        let move_dirs = vec![
            (2, 1),
            (2, -1),
            (-2, 1),
            (-2, -1),
            (1, 2),
            (1, -2),
            (-1, 2),
            (-1, -2),
        ];
        let mut moves = Vec::new();
        for (file_inc, rank_inc) in move_dirs {
            let target_file = file(square) as i32 + file_inc;
            let target_rank = rank(square) as i32 + rank_inc;
            if !is_on_board(target_rank, target_file) {
                continue;
            }
            let target_square = add_rank(add_file(square, file_inc) as usize, rank_inc) as usize;
            if self.pieces[target_square]
                .clone()
                .is_some_and(|p| p.color == self.side_to_play)
            {
                continue;
            }
            moves.push(Move::standard(square, target_square));
        }
        return moves;
    }

    fn generate_moves(&self, castling: bool) -> Vec<Move> {
        let mut moves = Vec::new();
        for (square, occupant) in self.pieces.iter().enumerate() {
            if self.pieces[square].is_none()
                || self.pieces[square].clone().unwrap().color != self.side_to_play
            {
                continue;
            }
            match occupant {
                Some(piece) => match piece.kind {
                    PieceKind::Pawn => moves.append(&mut self.generate_pawn_moves(square)),
                    PieceKind::Bishop => moves.append(&mut self.generate_bishop_moves(square)),
                    PieceKind::Rook => moves.append(&mut self.generate_rook_moves(square)),
                    PieceKind::Queen => moves.append(&mut self.generate_queen_moves(square)),
                    PieceKind::King => {
                        moves.append(&mut self.generate_king_moves(square, castling))
                    }
                    PieceKind::Knight => moves.append(&mut self.generate_knight_moves(square)),
                },
                None => continue,
            }
        }
        return moves;
    }
}
