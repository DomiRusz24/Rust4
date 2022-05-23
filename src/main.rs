mod connect4;
pub mod util;
mod console;

use connect4::game_board::GameBoard;
use connect4::game_board_config::GameBoardConfiguration;
use console::console::{ConsoleInput};
use connect4::board_io::BoardOutput;

fn main() {

    let console: ConsoleInput = ConsoleInput{};

    let mut config: GameBoardConfiguration = console.get_configuration();

    config.output = Some(&console);

    let mut game_board: GameBoard = GameBoard::new(config);

    game_board.start();

}
