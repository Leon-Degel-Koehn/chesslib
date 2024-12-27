use chesslib::move_generation::MoveGeneration;
use chesslib::move_utils::*;
use chesslib::square_utils::square_from_string;

mod utils;

#[test]
// in the standard start position the pawns of the white player can move one or two squares
fn standard_pos_pawn_moves() {
    let position =
        fen::BoardState::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
            .unwrap();
    let legal_moves = position.legal_moves();
    let expected_moves: Vec<Move> = vec![
        Move::standard(square_from_string("a2"), square_from_string("a3")),
        Move::standard(square_from_string("a2"), square_from_string("a4")),
        Move::standard(square_from_string("b2"), square_from_string("b3")),
        Move::standard(square_from_string("b2"), square_from_string("b4")),
        Move::standard(square_from_string("c2"), square_from_string("c3")),
        Move::standard(square_from_string("c2"), square_from_string("c4")),
        Move::standard(square_from_string("d2"), square_from_string("d3")),
        Move::standard(square_from_string("d2"), square_from_string("d4")),
        Move::standard(square_from_string("e2"), square_from_string("e3")),
        Move::standard(square_from_string("e2"), square_from_string("e4")),
        Move::standard(square_from_string("f2"), square_from_string("f3")),
        Move::standard(square_from_string("f2"), square_from_string("f4")),
        Move::standard(square_from_string("g2"), square_from_string("g3")),
        Move::standard(square_from_string("g2"), square_from_string("g4")),
        Move::standard(square_from_string("h2"), square_from_string("h3")),
        Move::standard(square_from_string("h2"), square_from_string("h4")),
    ];
    assert!(utils::contains_moves(&legal_moves, &expected_moves));
}

#[test]
// pawns cannot capture the way they normally move
fn blockade() {
    let position = fen::BoardState::from_fen("8/8/4p3/4P3/8/8/8/8 w - - 0 1").unwrap();
    let legal_moves = position.legal_moves();
    let expected_moves: Vec<Move> = Vec::new();
    assert!(utils::equal_moves(&legal_moves, &expected_moves));
}

#[test]
fn captures_in_center_of_board() {
    // in the standard starting position pawns can move one or two squares
    let position = fen::BoardState::from_fen("8/8/8/8/4p3/3P4/8/8 w - - 0 1").unwrap();
    let legal_moves = position.legal_moves();
    let expected_moves: Vec<Move> = vec![
        Move::standard(square_from_string("d3"), square_from_string("e4")),
        Move::standard(square_from_string("d3"), square_from_string("d4")),
    ];
    assert!(utils::contains_moves(&legal_moves, &expected_moves));
}

#[test]
fn captures_on_edge_of_board() {
    // in the standard starting position pawns can move one or two squares
    let position = fen::BoardState::from_fen("8/8/2p2p2/1P4P1/p6p/1P4P1/8/8 w - - 0 1").unwrap();
    let legal_moves = position.legal_moves();
    let expected_moves: Vec<Move> = vec![
        Move::standard(square_from_string("b3"), square_from_string("b4")),
        Move::standard(square_from_string("b3"), square_from_string("a4")),
        Move::standard(square_from_string("b5"), square_from_string("c6")),
        Move::standard(square_from_string("b5"), square_from_string("b6")),
        Move::standard(square_from_string("g5"), square_from_string("f6")),
        Move::standard(square_from_string("g5"), square_from_string("g6")),
        Move::standard(square_from_string("g3"), square_from_string("g4")),
        Move::standard(square_from_string("g3"), square_from_string("h4")),
    ];
    assert!(
        utils::equal_moves(&legal_moves, &expected_moves),
        "{:?}",
        legal_moves
    );
}

#[test]
fn promotion() {
    // in the standard starting position pawns can move one or two squares
    let position = fen::BoardState::from_fen("8/6P1/8/8/8/8/8/8 w - - 0 1").unwrap();
    let legal_moves = position.legal_moves();
    let expected_moves: Vec<Move> = vec![
        Move {
            start_square: square_from_string("g7"),
            end_square: square_from_string("g8"),
            promotion: Some(fen::PieceKind::Rook),
            is_en_passant: false,
        },
        Move {
            start_square: square_from_string("g7"),
            end_square: square_from_string("g8"),
            promotion: Some(fen::PieceKind::Bishop),
            is_en_passant: false,
        },
        Move {
            start_square: square_from_string("g7"),
            end_square: square_from_string("g8"),
            promotion: Some(fen::PieceKind::Knight),
            is_en_passant: false,
        },
        Move {
            start_square: square_from_string("g7"),
            end_square: square_from_string("g8"),
            promotion: Some(fen::PieceKind::Queen),
            is_en_passant: false,
        },
    ];
    assert!(utils::equal_moves(&legal_moves, &expected_moves));
}

#[test]
// in this position the pawn could move one square forwards
// this would however leave the white king in check and as such
// is an illegal move
fn would_leave_in_check() {
    let position = fen::BoardState::from_fen("8/8/1k6/1b6/8/3P4/4K3/8 w - - 0 1").unwrap();
    let legal_moves = position.legal_moves();
    let illegal_move: Vec<Move> = vec![Move::standard(
        square_from_string("d3"),
        square_from_string("d4"),
    )];
    assert!(
        !utils::contains_moves(&legal_moves, &illegal_move),
        "{:?}",
        legal_moves
    );
}

#[test]
fn en_passant() {
    let position = fen::BoardState::from_fen("1k6/8/8/4pP2/8/8/8/1K6 w - e6 0 1").unwrap();
    let legal_moves = position.legal_moves();
    let expected_moves: Vec<Move> = vec![Move {
        start_square: square_from_string("f5"),
        end_square: square_from_string("e6"),
        promotion: None,
        is_en_passant: true,
    }];
    assert!(
        utils::contains_moves(&legal_moves, &expected_moves),
        "{:?}",
        legal_moves
    );
}
