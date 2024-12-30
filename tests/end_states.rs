use chesslib::end_state::*;
use chesslib::move_utils::Move;
use chesslib::square_utils::*;
use chesslib::Game;

#[test]
fn black_is_checkmate() {
    let position = Game::start_from_fen("8/8/8/8/8/5KQk/8/8 b - - 0 1");
    assert!(
        position.current_player_is_checkmate()
            && position.winner().is_some_and(|c| c == fen::Color::White)
    );
}

#[test]
fn white_is_stalemate() {
    let position = Game::start_from_fen("8/2k5/8/8/8/8/2q5/K7 w - - 0 1");
    assert!(position.current_player_is_stalemate());
}

#[test]
fn draw_by_insufficient_material() {
    let position = Game::start_from_fen("8/8/3k4/8/8/8/2K5/8 b - - 0 1");
    assert!(position.is_draw() && position.insufficient_material());
}

#[test]
fn draw_by_repetition() {
    let mut game = Game::start_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    game.execute_move(&Move::standard(
        square_from_string("e2"),
        square_from_string("e4"),
    ));
    game.execute_move(&Move::standard(
        square_from_string("e7"),
        square_from_string("e5"),
    ));
    game.execute_move(&Move::standard(
        square_from_string("e1"),
        square_from_string("e2"),
    ));
    game.execute_move(&Move::standard(
        square_from_string("e8"),
        square_from_string("e7"),
    ));
    game.execute_move(&Move::standard(
        square_from_string("e2"),
        square_from_string("e1"),
    ));
    game.execute_move(&Move::standard(
        square_from_string("e7"),
        square_from_string("e8"),
    ));
    game.execute_move(&Move::standard(
        square_from_string("e1"),
        square_from_string("e2"),
    ));
    game.execute_move(&Move::standard(
        square_from_string("e8"),
        square_from_string("e7"),
    ));
    game.execute_move(&Move::standard(
        square_from_string("e2"),
        square_from_string("e1"),
    ));
    game.execute_move(&Move::standard(
        square_from_string("e7"),
        square_from_string("e8"),
    ));
    assert!(game.draw_by_repetition);
    assert!(game.is_draw());
}

#[test]
fn game_hasnt_ended() {
    let position = Game::start_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    assert!(!position.current_player_is_stalemate() && !position.current_player_is_checkmate());
}
