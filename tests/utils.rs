use chesslib::move_generation::Move;

// returns true iff the superset contains all moves provided in moves
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
pub fn equal_moves(expected: &Vec<Move>, moves: &Vec<Move>) -> bool {
    return contains_moves(expected, moves) && contains_moves(moves, expected);
}
