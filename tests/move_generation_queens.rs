use chesslib::move_generation::MoveGeneration;
use chesslib::move_utils::*;
use chesslib::square_utils::square_from_string;

mod utils;

#[test]
fn pinned_queen() {
    let position = fen::BoardState::from_fen("8/8/8/2k5/3q4/8/5B2/4K3 b - - 0 1").unwrap();
    let legal_moves = position.legal_moves();
    let expected_moves: Vec<Move> = vec![
        Move::standard(square_from_string("d4"), square_from_string("e3")),
        Move::standard(square_from_string("d4"), square_from_string("f2")),
    ];
    assert!(utils::moves_starting_from(
        square_from_string("d4"),
        &expected_moves,
        &legal_moves
    ));
}
