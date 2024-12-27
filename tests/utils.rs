use chesslib::move_generation::Move;

// returns true iff the superset contains all moves provided in moves
#[allow(dead_code)]
pub fn contains_moves(superset: &Vec<Move>, moves: &Vec<Move>) -> bool {
    let mut success = true;
    for mov in moves {
        if !superset.contains(&mov) {
            success = false;
        }
    }
    return success;
}

// returns true iff the superset contains all moves provided in moves
#[allow(dead_code)]
pub fn equal_moves(expected: &Vec<Move>, moves: &Vec<Move>) -> bool {
    return contains_moves(expected, moves) && contains_moves(moves, expected);
}

// return true iff no move in the given list of moves starts at the provided square
#[allow(dead_code)]
pub fn no_move_starting_at(square: usize, moves: &Vec<Move>) -> bool {
    return !moves.iter().any(|m| m.start_square == square);
}

// return true iff exactly the expected moves starting from square are present in move
#[allow(dead_code)]
pub fn moves_starting_from(square: usize, expected: &Vec<Move>, moves: &Vec<Move>) -> bool {
    return moves
        .iter()
        .all(|m| m.start_square != square || expected.contains(m));
}
