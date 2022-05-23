use std::fmt::{Display};
use crate::connect4::board_io::{BoardInput, BoardOutput};
use crate::util::{get_input, get_input_with_message, split};
use std::num::ParseIntError;
use std::str::FromStr;
use crate::GameBoardConfiguration;

pub struct ConsoleInput{}

impl<'a> BoardOutput<'a> for ConsoleInput {
    fn display_board(&self, board: &[Vec<Option<u8>>], width: usize, height: usize) {

        split();

        let mut x: usize;
        let mut y: usize = 0;

        while y < height {
            x = 0;
            while x < width {

                if board[x][y] == None {
                    print!("[ ]");
                } else {
                    print!("[{}]", board[x][y].unwrap());
                }

                x+= 1;
            }
            println!();

            y+= 1;
        }

    }

    fn waiting_for_input(&self, id: u8, input: &mut dyn BoardInput) {
        println!("Player {} turn: ", id);

        let option: String = get_input();

        let x1: Result<i16, ParseIntError> = option.parse::<i16>();

        if let Ok(..) = x1 {
            input.input(x1.unwrap());
        } else {
            println!("Incorrect format!");
            self.waiting_for_input(id, input);
        }
    }


    fn cant_place_tile(&self, id: u8, _x: i16) {
        println!("Player {}, can't place tile here!", id);
    }

    fn player_win(&self, id: u8) {
        println!("Player {} won! Press enter to quit.", id);

        let _ = get_input();
    }

    fn get_configuration(&self) -> GameBoardConfiguration<'a> {
        let mut config: GameBoardConfiguration = GameBoardConfiguration::new();

        config.width = get_config_value("width", config.width);
        config.height = get_config_value("height", config.height);
        config.amount_of_players = get_config_value("player amount", config.amount_of_players);
        config.winning_streak = get_config_value("winning streak", config.winning_streak);

        config
    }
}

fn get_config_value<T: Display + FromStr>(name: &str, default: T) -> T {
    get_input_with_message(format!("Enter {} (Press enter for {}):", name, default).as_str()).parse().unwrap_or(default)
}