use crate::connect4::game_board_config::GameBoardConfiguration;

pub trait BoardOutput<'a> {
    fn display_board(&self, board: &[Vec<Option<u8>>], width: usize, height: usize);
    fn waiting_for_input(&self, id: u8, input: &mut dyn BoardInput);
    fn cant_place_tile(&self, id: u8, x: i16);
    fn player_win(&self, id: u8);
    fn get_configuration(&self) -> GameBoardConfiguration<'a>;
}

pub trait BoardInput {
    fn input(&mut self, x: i16);
}
