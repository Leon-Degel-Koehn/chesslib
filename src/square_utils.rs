// increase the rank of the square by the given amount
pub fn add_rank(square: usize, increment: i32) -> i32 {
    return square as i32 + increment * 8;
}

pub fn add_file(square: usize, increment: i32) -> i32 {
    return square as i32 + increment;
}

pub fn rank(square: usize) -> usize {
    return square / 8;
}

pub fn file(square: usize) -> usize {
    return square % 8;
}

pub fn rank_str(square: usize) -> char {
    return (b'1' as u8 + rank(square) as u8) as char;
}

pub fn file_str(square: usize) -> char {
    return (b'a' as u8 + file(square) as u8) as char;
}

pub fn is_on_board(square: i32) -> bool {
    return square >= 0 && square < 64;
}

pub fn square_from_string(square_str: &str) -> usize {
    let mut square: u8 = 0;
    square += (square_str.chars().collect::<Vec<char>>()[0] as u8) - b'a';
    square += ((square_str.chars().collect::<Vec<char>>()[1] as u8) - b'1') * 8;
    return square as usize;
}

pub fn square_to_string(square: usize) -> String {
    let mut str = String::new();
    str.push(file_str(square));
    str.push(rank_str(square));
    return str;
}
