use chesslib::move_generation::MoveGeneration;
use chesslib::move_utils::*;
use chesslib::square_utils::square_from_string;

mod utils;

#[test]
fn regular_moves() {
    let position = fen::BoardState::from_fen("8/8/8/8/3R4/8/8/8 w - - 0 1").unwrap();
    let legal_moves = position.legal_moves();
    let expected_moves: Vec<Move> = vec![
        Move::standard(square_from_string("d4"), square_from_string("d5")),
        Move::standard(square_from_string("d4"), square_from_string("d6")),
        Move::standard(square_from_string("d4"), square_from_string("d7")),
        Move::standard(square_from_string("d4"), square_from_string("d8")),
        Move::standard(square_from_string("d4"), square_from_string("d3")),
        Move::standard(square_from_string("d4"), square_from_string("d2")),
        Move::standard(square_from_string("d4"), square_from_string("d1")),
        Move::standard(square_from_string("d4"), square_from_string("e4")),
        Move::standard(square_from_string("d4"), square_from_string("f4")),
        Move::standard(square_from_string("d4"), square_from_string("g4")),
        Move::standard(square_from_string("d4"), square_from_string("h4")),
        Move::standard(square_from_string("d4"), square_from_string("a4")),
        Move::standard(square_from_string("d4"), square_from_string("b4")),
        Move::standard(square_from_string("d4"), square_from_string("c4")),
    ];
    assert!(utils::contains_moves(&legal_moves, &expected_moves));
}

#[test]
fn cannot_move_out_of_board() {
    let position = fen::BoardState::from_fen("r7/8/8/8/8/8/8/8 b - - 0 1").unwrap();
    let legal_moves = position.legal_moves();
    let expected_moves: Vec<Move> = vec![
        Move::standard(square_from_string("a8"), square_from_string("b8")),
        Move::standard(square_from_string("a8"), square_from_string("c8")),
        Move::standard(square_from_string("a8"), square_from_string("d8")),
        Move::standard(square_from_string("a8"), square_from_string("e8")),
        Move::standard(square_from_string("a8"), square_from_string("f8")),
        Move::standard(square_from_string("a8"), square_from_string("g8")),
        Move::standard(square_from_string("a8"), square_from_string("h8")),
        Move::standard(square_from_string("a8"), square_from_string("a1")),
        Move::standard(square_from_string("a8"), square_from_string("a2")),
        Move::standard(square_from_string("a8"), square_from_string("a3")),
        Move::standard(square_from_string("a8"), square_from_string("a4")),
        Move::standard(square_from_string("a8"), square_from_string("a5")),
        Move::standard(square_from_string("a8"), square_from_string("a6")),
        Move::standard(square_from_string("a8"), square_from_string("a7")),
    ];
    assert!(
        utils::equal_moves(&legal_moves, &expected_moves),
        "{:?}",
        legal_moves
    );
}
