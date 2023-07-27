pub enum Player {
    X,
    O,
}

impl Player {
    pub fn to_char(&self) -> char {
        match self {
            Player::X => 'X',
            Player::O => 'O',
        }
    }
}