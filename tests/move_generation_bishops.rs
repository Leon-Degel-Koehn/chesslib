use chesslib::move_generation::MoveGeneration;
use chesslib::move_utils::*;
use chesslib::square_utils::square_from_string;

mod utils;

#[test]
fn bishop_in_corner() {
    let position = fen::BoardState::from_fen("8/8/8/8/8/8/8/B7 w - - 0 1").unwrap();
    let legal_moves = position.legal_moves();
    let expected_moves: Vec<Move> = vec![
        Move::standard(square_from_string("a1"), square_from_string("b2")),
        Move::standard(square_from_string("a1"), square_from_string("c3")),
        Move::standard(square_from_string("a1"), square_from_string("d4")),
        Move::standard(square_from_string("a1"), square_from_string("e5")),
        Move::standard(square_from_string("a1"), square_from_string("f6")),
        Move::standard(square_from_string("a1"), square_from_string("g7")),
        Move::standard(square_from_string("a1"), square_from_string("h8")),
    ];
    assert!(utils::contains_moves(&legal_moves, &expected_moves));
}

#[test]
fn blocked_bishop() {
    let position = fen::BoardState::from_fen("k7/8/8/2P1P3/3B4/2P1P3/8/K7 w - - 0 1").unwrap();
    let legal_moves = position.legal_moves();
    assert!(
        utils::no_move_starting_at(square_from_string("d4"), &legal_moves),
        "{:?}",
        legal_moves
    );
}
