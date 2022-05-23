use crate::connect4::board_io::BoardOutput;

pub struct GameBoardConfiguration<'a> {
    pub width: usize,
    pub height: usize,
    pub amount_of_players: u8,
    pub winning_streak: usize,
    pub output: Option<&'a dyn BoardOutput<'a>>,
}

impl<'a> GameBoardConfiguration<'a> {
    pub fn new() -> GameBoardConfiguration<'a> {
        GameBoardConfiguration {
            width: 20,
            height: 10,
            amount_of_players: 2,
            winning_streak: 4,
            output: None,
        }
    }
}
