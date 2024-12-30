use chesslib::move_generation::MoveGeneration;
use chesslib::move_utils::*;
use chesslib::square_utils::square_from_string;

mod utils;

#[test]
fn white_can_castle_queen_and_kingside() {
    let position = fen::BoardState::from_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1").unwrap();
    let legal_moves = position.legal_moves();
    let expected_moves: Vec<Move> = vec![
        Move {
            start_square: square_from_string("e1"),
            end_square: square_from_string("c1"),
            promotion: None,
            is_en_passant: false,
            castles: true,
        },
        Move {
            start_square: square_from_string("e1"),
            end_square: square_from_string("g1"),
            promotion: None,
            is_en_passant: false,
            castles: true,
        },
    ];
    assert!(
        utils::contains_moves(&legal_moves, &expected_moves),
        "{:?}",
        legal_moves
    );
}

#[test]
fn black_can_castle_queen_and_kingside() {
    let position = fen::BoardState::from_fen("r3k2r/8/8/8/8/8/8/R3K2R b KQkq - 0 1").unwrap();
    let legal_moves = position.legal_moves();
    let expected_moves: Vec<Move> = vec![
        Move {
            start_square: square_from_string("e8"),
            end_square: square_from_string("c8"),
            promotion: None,
            is_en_passant: false,
            castles: true,
        },
        Move {
            start_square: square_from_string("e8"),
            end_square: square_from_string("g8"),
            promotion: None,
            is_en_passant: false,
            castles: true,
        },
    ];
    assert!(
        utils::contains_moves(&legal_moves, &expected_moves),
        "{:?}",
        legal_moves
    );
}

// white can only castle short, the long castle would go through a check
#[test]
fn white_cant_castle_through_check() {
    let position = fen::BoardState::from_fen("r3k2r/8/8/8/2r5/8/8/R3K2R w KQkq - 0 1").unwrap();
    let legal_moves = position.legal_moves();
    let unwanted_move: Vec<Move> = vec![Move {
        start_square: square_from_string("e1"),
        end_square: square_from_string("c1"),
        promotion: None,
        is_en_passant: false,
        castles: true,
    }];
    let wanted_move: Vec<Move> = vec![Move {
        start_square: square_from_string("e1"),
        end_square: square_from_string("g1"),
        promotion: None,
        is_en_passant: false,
        castles: true,
    }];
    assert!(
        !utils::contains_moves(&legal_moves, &unwanted_move)
            && utils::contains_moves(&legal_moves, &wanted_move),
        "{:?}",
        legal_moves
    );
}

// white cant castle at all because both sides would be through a check
#[test]
fn white_cant_castle_through_check_2() {
    let position = fen::BoardState::from_fen("r3k2r/8/8/8/2qr4/8/8/R3K2R w KQkq - 0 1").unwrap();
    let legal_moves = position.legal_moves();
    let unwanted_moves: Vec<Move> = vec![
        Move {
            start_square: square_from_string("e1"),
            end_square: square_from_string("c1"),
            promotion: None,
            is_en_passant: false,
            castles: true,
        },
        Move {
            start_square: square_from_string("e1"),
            end_square: square_from_string("g1"),
            promotion: None,
            is_en_passant: false,
            castles: true,
        },
    ];
    assert!(
        !utils::contains_moves(&legal_moves, &unwanted_moves),
        "{:?}",
        legal_moves
    );
}

// black can only castle short, the long castle would go through a check
