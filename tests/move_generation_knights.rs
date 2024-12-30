use chesslib::move_generation::MoveGeneration;
use chesslib::move_utils::*;
use chesslib::square_utils::square_from_string;

mod utils;

#[test]
fn knight_moves_startpos() {
    let position =
        fen::BoardState::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
            .unwrap();
    let legal_moves = position.legal_moves();
    let knight_square = square_from_string("b1");
    let expected_moves: Vec<Move> = vec![
        Move::standard(knight_square, square_from_string("a3")),
        Move::standard(knight_square, square_from_string("c3")),
    ];
    assert!(utils::moves_starting_from(
        knight_square,
        &expected_moves,
        &legal_moves
    ));
}

#[test]
fn knight_blocked_by_own_pawns() {
    let position =
        fen::BoardState::from_fen("k7/8/2p1p3/1p3p2/3n4/1p3p2/2p1p3/K7 b - - 0 1").unwrap();
    let legal_moves = position.legal_moves();
    let knight_square = square_from_string("d4");
    assert!(utils::no_move_starting_at(knight_square, &legal_moves));
}

#[test]
fn octopus_knight() {
    let position = fen::BoardState::from_fen("k7/8/8/8/4N3/8/8/K7 w - - 0 1").unwrap();
    let legal_moves = position.legal_moves();
    let knight_square = square_from_string("e4");
    let expected_moves: Vec<Move> = vec![
        Move::standard(knight_square, square_from_string("f2")),
        Move::standard(knight_square, square_from_string("g3")),
        Move::standard(knight_square, square_from_string("d2")),
        Move::standard(knight_square, square_from_string("c3")),
        Move::standard(knight_square, square_from_string("c5")),
        Move::standard(knight_square, square_from_string("d6")),
        Move::standard(knight_square, square_from_string("f6")),
        Move::standard(knight_square, square_from_string("g5")),
    ];
    assert!(utils::moves_starting_from(
        knight_square,
        &expected_moves,
        &legal_moves
    ));
}
