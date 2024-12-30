use chesslib::move_generation::*;

#[test]
fn black_is_checkmate() {
    let position = fen::BoardState::from_fen("8/8/8/8/8/5KQk/8/8 b - - 0 1").unwrap();
    assert!(
        position.current_player_is_checkmate()
            && position.winner().is_some_and(|c| c == fen::Color::White)
    );
}

#[test]
fn white_is_stalemate() {
    let position = fen::BoardState::from_fen("8/2k5/8/8/8/8/2q5/K7 w - - 0 1").unwrap();
    assert!(position.current_player_is_stalemate());
}

#[test]
fn end_by_insufficient_material() {
    let position = fen::BoardState::from_fen("8/8/3k4/8/8/8/2K5/8 b - - 0 1").unwrap();
    assert!(position.is_draw() && position.insufficient_material());
}

#[test]
fn game_hasnt_ended() {
    let position =
        fen::BoardState::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
            .unwrap();
    assert!(!position.current_player_is_stalemate() && !position.current_player_is_checkmate());
}
