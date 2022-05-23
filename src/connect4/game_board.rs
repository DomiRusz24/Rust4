use crate::util::{max, min};

use std::vec::Vec;

use crate::connect4::board_io::*;
use crate::connect4::game_board_config::GameBoardConfiguration;

pub struct GameBoard<'a> {
    width: usize,
    height: usize,
    winning_streak: usize,
    amount_of_players: u8,

    current_player: u8,

    output: &'a dyn BoardOutput<'a>,
    board: Vec<Vec<Option<u8>>>,

    waiting_for_input: bool,
}

impl<'a> BoardInput for GameBoard<'a> {
    fn input(&mut self, x1: i16) {
        if x1 <= 0 {
            self.output.cant_place_tile(self.current_player, x1);
            self.wait_for_input();
            return;
        }

        let x: usize = usize::from(x1 as u16) - 1;

        if x >= self.width {
            self.output.cant_place_tile(self.current_player, x1);
            self.wait_for_input();
            return;
        }

        if !self.waiting_for_input {
            return;
        }
        let mut y = self.height;

        while y > 0 {
            y -= 1;
            if self.board[x][y] == None {
                self.board[x][y] = Some(self.current_player);
                self.waiting_for_input = false;

                if self.check_for_win(x, y) {
                    self.display_board();
                    self.output.player_win(self.current_player);
                } else {
                    self.current_player = (self.current_player + 1) % self.amount_of_players;
                    self.new_round();
                }

                return;
            }
        }

        self.output.cant_place_tile(self.current_player, x1);
        self.wait_for_input();
    }
}

impl<'a> GameBoard<'a> {
    pub fn new(config: GameBoardConfiguration) -> GameBoard {
        if config.output.is_none() {
            panic!("Board input not implemented!");
        }

        GameBoard {
            width: config.width,
            height: config.height,
            amount_of_players: config.amount_of_players,
            winning_streak: config.winning_streak,
            current_player: 0,
            output: config.output.unwrap(),
            board: vec![vec![None; config.height as usize]; config.width as usize],
            waiting_for_input: false,
        }
    }

    pub fn start(&mut self) {
        self.new_round();
    }

    fn display_board(&mut self) {
        self.output
            .display_board(&self.board, self.width, self.height);
    }

    fn new_round(&mut self) {
        self.display_board();
        self.wait_for_input();
    }

    fn wait_for_input(&mut self) {
        self.waiting_for_input = true;
        self.output.waiting_for_input(self.current_player, self);
    }

    fn check_for_win(&self, x: usize, y: usize) -> bool {
        let mut from_x: usize = max(x as isize - self.winning_streak as isize - 1, 0);
        let mut from_y: usize = max(y as isize - self.winning_streak as isize - 1, 0);

        let to_x: usize = min(
            x as isize + self.winning_streak as isize - 1,
            self.width as isize - 1,
        );
        let to_y: usize = min(
            y as isize + self.winning_streak as isize - 1,
            self.height as isize - 1,
        );

        let mut combo: usize = 0;

        while from_x <= to_x {
            combo = self.is_combo(combo, from_x, y);

            if combo == self.winning_streak {
                return true;
            }

            from_x += 1;
        }

        combo = 0;

        while from_y <= to_y {
            combo = self.is_combo(combo, x, from_y);

            if combo == self.winning_streak {
                return true;
            }

            from_y += 1;
        }

        false
    }

    fn is_combo(&self, combo: usize, x: usize, y: usize) -> usize {
        match self.board[x][y] {
            None => 0,
            Some(id) => {
                if id == self.current_player {
                    combo + 1
                } else {
                    0
                }
            }
        }
    }
}
