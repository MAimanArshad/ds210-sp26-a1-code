#[derive(PartialEq, Clone, Copy)]
pub enum Player {
    X,
    O,
}
impl Player {
    pub fn flip(self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
    pub fn to_string(self) -> String {
        match self {
            Player::X => String::from("X"),
            Player::O => String::from("O"),
        }
    }
}
