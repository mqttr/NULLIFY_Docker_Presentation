pub fn game_loop(game_state: &mut GameState) {
    game_state.print_board();

}

pub fn build_game_state(
    board_size: usize,
    count_to_win: Option<usize>,
    starting_player: Option<Player>,
) -> GameState {
    let resolv_starting_player = starting_player.unwrap_or(Player::X);

    GameState{
        turn_number: 0,
        starting_player: resolv_starting_player.clone(),
        other_player: match &resolv_starting_player {
            Player::X => Player::O,
            Player::O => Player::X
        },
        board: vec![vec![None; board_size]; board_size],
        count_to_win: count_to_win.unwrap_or(board_size),
    }
}

#[derive(Clone, Copy)]
pub enum Player {
    X,
    O,
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O")
        }
    }
}

pub struct GameState {
    turn_number: usize,
    count_to_win: usize,
    starting_player: Player,
    other_player: Player,
    board: Vec<Vec<Option<Player>>>,
}

impl GameState {
    // Origin: Bottom left
    fn place(&mut self, x: u32, y: u32) -> Option<Player> {
        todo!();
    }

    fn get_current_player(&self) -> &Player {
        match self.turn_number % 2 {
            0 => &self.starting_player,
            1 => &self.other_player,
            _ => unreachable!("There can only be 2 players in tictactoe... unless?")
        }
    }
    fn print_board(&self) {
        println!("{}", "-".repeat(25));
        println!("Turn: {}", self.turn_number);
        println!("Current Player: {}", self.get_current_player());
        println!("Board:");
        let left_offset = 20;

        for row in self.board.iter().take(1) {
            for position in row.iter().take(1) {
                match position {
                    Some(player) => print!("{}", player),
                    None => print!(" ")
                };
            }
            print!("{}", " ".repeat(left_offset));
            for position in row.iter().skip(1) {
                print!("│");
                match position {
                    Some(player) => print!("{}", player),
                    None => print!(" ")
                };
            }
        }
        for row in self.board.iter().skip(1) {
            println!("");
            println!("{}{}{}", " ".repeat(left_offset), "─┼".repeat( self.board.len()-1), "─");
            for position in row.iter().take(1) {
                match position {
                    Some(player) => print!("{}", player),
                    None => print!(" ")
                };
            }
            print!("{}", " ".repeat(left_offset));
            for position in row.iter().skip(1) {

                print!("│");
                match position {
                    Some(player) => print!("{}", player),
                    None => print!(" ")
                };
            }
        }
        println!("");
    }
}
