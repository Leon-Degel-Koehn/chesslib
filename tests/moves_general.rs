use chesslib::move_generation::MoveGeneration;
use chesslib::move_utils::*;
use chesslib::square_utils::square_from_string;

mod utils;

#[test]
fn leaves_self_in_check() {
    let position = fen::BoardState::from_fen("8/8/1k6/1b6/8/3P4/4K3/8 w - - 0 1").unwrap();
    let mov = Move::standard(square_from_string("d3"), square_from_string("d4"));
    assert!(position.puts_self_in_check(&mov));
}

#[test]
fn move_execution() {
    let mut position = fen::BoardState::from_fen("8/8/1k6/1b6/8/3P4/4K3/8 w - - 0 1").unwrap();
    let mov = Move::standard(square_from_string("d3"), square_from_string("d4"));
    mov.execute(&mut position);
    let end_pos_fen = position.to_fen();
    assert!(
        end_pos_fen == "8/8/1k6/1b6/3P4/8/4K3/8 b - - 0 1",
        "{}",
        end_pos_fen
    );
}

#[test]
fn kingside_castle() {
    let mut position = fen::BoardState::from_fen("2k5/8/8/8/8/8/8/4K2R w K - 0 1").unwrap();
    let mov = Move {
        start_square: square_from_string("e1"),
        end_square: square_from_string("g1"),
        castles: true,
        promotion: None,
        is_en_passant: false,
    };
    mov.execute(&mut position);
    let end_pos_fen = position.to_fen();
    assert!(
        end_pos_fen == "2k5/8/8/8/8/8/8/5RK1 b - - 0 1",
        "{}",
        end_pos_fen
    );
}

#[test]
fn queenside_castle() {
    let mut position = fen::BoardState::from_fen("r3k3/8/8/8/8/8/8/5K2 b q - 0 1").unwrap();
    let mov = Move {
        start_square: square_from_string("e8"),
        end_square: square_from_string("c8"),
        castles: true,
        promotion: None,
        is_en_passant: false,
    };
    mov.execute(&mut position);
    let end_pos_fen = position.to_fen();
    assert!(
        end_pos_fen == "2kr4/8/8/8/8/8/8/5K2 w - - 0 1",
        "{}",
        end_pos_fen
    );
}

#[test]
fn king_move_ends_castling() {
    let mut position = fen::BoardState::from_fen(
        "rnbqk2r/pppp1ppp/5n2/4p3/1P1PP3/5N1P/PP3PP1/RNBQKB1R b KQkq - 0 6",
    )
    .unwrap();
    let mov = Move::from_long_algebraic("e8e7".to_string(), &position);
    mov.execute(&mut position);
    let mov = Move::from_long_algebraic("f1e2".to_string(), &position);
    mov.execute(&mut position);
    let mov = Move::from_long_algebraic("e7e8".to_string(), &position);
    mov.execute(&mut position);
    assert!(
        !position.black_can_oo && !position.black_can_ooo,
        "{}",
        position.to_fen()
    );
}
