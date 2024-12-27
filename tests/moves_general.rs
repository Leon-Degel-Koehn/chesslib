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
