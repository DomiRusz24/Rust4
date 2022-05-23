mod connect4;
mod console;
pub mod util;

use connect4::board_io::BoardOutput;
use connect4::game_board::GameBoard;
use connect4::game_board_config::GameBoardConfiguration;
use console::console::ConsoleInput;

fn main() {
    let input: ConsoleInput = ConsoleInput {};

    let mut config: GameBoardConfiguration = input.get_configuration();

    config.output = Some(&input);

    let mut game_board: GameBoard = GameBoard::new(config);

    game_board.start();
}
