pub fn game_loop(game_state: &mut GameState) {
    loop {
        game_state.print_board();

        let (x, y) = get_coords(game_state.get_current_player(), game_state.board.len(), None);

        match game_state.place(x, y) {
            Ok(_) => continue,
            Err(e) => {
                println!("{}", e);
                print!("Press enter to continue...");
                let _: String = read!("{}\n");
            },
        }
    }
}

fn print_prompt(current_player: &Player) -> String {
    print!("Where To place {}: ", current_player);
    read!("{}\n")
}

fn get_coords(current_player: &Player, max_x: usize, optional_max_y: Option<usize>) -> (usize, usize) {
    let max_y = optional_max_y.unwrap_or(max_x);
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
                        if x > max_x {
                            println!("Too large x position");
                            continue;
                        }
                        if x == 0 {
                            println!("X cannot be zero");
                            continue;
                        };
                        x
                    },
                    Err(_) => {
                        println!("Invalid x position");
                        continue;
                    },
                };
                let y: usize = match possible_coords[1].parse() {
                    Ok(y) => {
                        if y > max_y {
                            println!("Too large y position");
                            continue;
                        }
                        if y == 0 {
                            println!("Y cannot be zero");
                            continue;
                        };
                        y
                    },
                    Err(_) => {
                        println!("Invalid y position");
                        continue;
                    },
                };
                return (x-1, y-1);
            },
            _ => { print_help() }
        }
    }
}

fn print_help_prompt() {
    println!("Type '?' for help.");
}

fn print_help() {
    println!("Origin is in the top left.");
    println!("Your position should be x first then y position starting from the top left with a space separating them");
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
    // Origin: top left
    fn place(&mut self, x: usize, y: usize) -> Result<Player, &'static str> {
        if y > self.board.len() || x > self.board[0].len() {
            return Err("Out of Bounds");
        }

        let player = *self.get_current_player();

        match self.board[y][x] {
            None => {

                self.board[y][x] = Some(player);
                self.turn_number+=1;

                // TODO: Check winner
                Ok(player)
            },
            Some(_) => {
                Err("There is already a piece there!")
            }
        }
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
            print!("{}", " ".repeat(left_offset));
            for position in row.iter().take(1) {
                match position {
                    Some(player) => print!("{}", player),
                    None => print!(" ")
                };
            }
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
            print!("{}", " ".repeat(left_offset));
            for position in row.iter().take(1) {
                match position {
                    Some(player) => print!("{}", player),
                    None => print!(" ")
                };
            }
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
