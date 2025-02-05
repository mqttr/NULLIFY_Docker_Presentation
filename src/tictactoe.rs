pub fn game_loop(game_state: &mut GameState) {
    game_state.print_board();

    let (x, y) = get_coords(game_state.get_current_player(), game_state.board.len());

}

fn print_prompt(current_player: &Player) -> String {
    print!("Where To place {}: ", current_player);
    read!("{}\n")
}

fn get_coords(current_player: &Player, max: usize) -> (usize, usize) {
    print_help_prompt();
    loop {
        let user: String = print_prompt(current_player);
        let stripped_user = user.trim();

        let possible_coords: Vec<&str> = stripped_user.split_whitespace().collect();

        match possible_coords.len() {
            0 => continue,
            1 => {
                if possible_coords[0] == "?" {
                    print_help();
                } else {
                    println!("Missing other coordinate!");
                    print_help_prompt();
                    continue;
                }
            },
            2 => {
                let x: usize = match possible_coords[0].parse() {
                    Ok(x) => {
                        if x > max {
                            println!("Too large x position");
                            continue;
                        }
                        x
                    },
                    Err(_) => {
                        println!("Invalid x position");
                        continue;
                    },
                };
                let y: usize = match possible_coords[1].parse() {
                    Ok(y) => {
                        if y > max {
                            println!("Too large y position");
                            continue;
                        }
                        y
                    },
                    Err(_) => {
                        println!("Invalid y position");
                        continue;
                    },
                };
                return (x, y);
            },
            _ => { print_help() }
        }
    }
}

fn print_help_prompt() {
    println!("Type '?' for help.");
}

fn print_help() {
    println!("Origin is in the bottom left.");
    println!("Your position should be x first then y position starting from the bottom left with a separating them");
    println!("Examples: 'x y' '3 1' '1 2' '1 3'");
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
