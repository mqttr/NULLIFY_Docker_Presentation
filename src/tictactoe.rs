use serde::{Deserialize, Serialize};

pub fn game_loop(game_state: &mut GameState) -> Option<Player> {
    loop {
        game_state.print_board();

        let (x, y) = get_coords(game_state.get_current_player(), game_state.board.len(), None);

        match game_state.place(x, y) {
            Ok(_) => {
                match game_state.check_winner() {
                    None => continue,
                    Some(option_winner) => {
                        match option_winner {
                            None => {
                                println!();
                                game_state.print_board();
                                println!("Noone one! This game is a Cat!");
                                return None;
                            },
                            Some(winner) => {
                                println!();
                                game_state.print_board();
                                println!("Game over! Player {} won!", winner);
                                return Some(winner);
                            }
                        }
                    }
                }
            },
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

pub fn get_coords(current_player: &Player, max_x: usize, optional_max_y: Option<usize>) -> (usize, usize) {
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
                            println!("Too large first position");
                            continue;
                        }
                        if x == 0 {
                            println!("First position cannot be zero");
                            continue;
                        };
                        x
                    },
                    Err(_) => {
                        println!("Invalid first position");
                        continue;
                    },
                };
                let y: usize = match possible_coords[1].parse() {
                    Ok(y) => {
                        if y > max_y {
                            println!("Too large second position");
                            continue;
                        }
                        if y == 0 {
                            println!("Second position cannot be zero");
                            continue;
                        };
                        y
                    },
                    Err(_) => {
                        println!("Invalid second position");
                        continue;
                    },
                };
                return (x-1, (max_y)-y);
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
    println!("Over from the left; Up from the bottom");
    println!("Example: '1 1' (Bottom Left)");
    println!("Example: '1 3' (top Left)");
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

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize, Clone)]
pub struct GameState {
    pub turn_number: usize,
    pub count_to_win: usize,
    pub starting_player: Player,
    pub other_player: Player,
    pub board: Vec<Vec<Option<Player>>>,
}

impl GameState {
    // Origin: top left
    pub fn place(&mut self, x: usize, y: usize) -> Result<Player, &'static str> {
        if y > self.board.len() || x > self.board[0].len() {
            return Err("Out of Bounds");
        }

        let player = *self.get_current_player();

        match self.board[y][x] {
            None => {

                self.board[y][x] = Some(player);
                self.turn_number+=1;

                Ok(player)
            },
            Some(_) => {
                Err("There is already a piece there!")
            }
        }
    }

    pub fn check_winner(&self) -> Option<Option<Player>> {
        // Trust me the return makes sense
        let mut current_player: Option<Player>;
        let mut running_count: usize;

        // Check for horizontal win
        for i in 0..self.board.len() {
            running_count = 0;
            current_player = None;
            for j in 0..self.board[0].len() {
                match self.board[i][j] {
                    Some(player) => {
                        match current_player {
                            None => {
                                running_count+=1;
                                current_player = Some(player);
                            },
                            Some(curr) => {
                                if player == curr {
                                    running_count += 1;
                                } else {
                                    running_count = 0;
                                    current_player = Some(player);
                                }
                            }
                        }

                    },
                    None => {
                        running_count = 0;
                        current_player = None;
                    }
                }
                if running_count == self.count_to_win {
                    return Some(current_player);
                }
            }
        }
        // Check for vertical win
        for j in 0..self.board[0].len() {
            running_count = 0;
            current_player = None;
            for i in 0..self.board.len() {
                match self.board[i][j] {
                    Some(player) => {
                        match current_player {
                            None => {
                                running_count+=1;
                                current_player = Some(player);
                            },
                            Some(curr) => {
                                if player == curr {
                                    running_count += 1;
                                } else {
                                    running_count = 0;
                                    current_player = Some(player);
                                }
                            }
                        }

                    },
                    None => {
                        running_count = 0;
                        current_player = None;
                    }
                }
                if running_count == self.count_to_win {
                    return Some(current_player);
                }
            }
        }

        // TODO: Make this more generic cuz I won't
        if
            self.board[0][0].is_some() && self.board[1][1].is_some() && self.board[2][2].is_some() &&
            (self.board[0][0].unwrap() == self.board[1][1].unwrap() && self.board[1][1].unwrap() == self.board[2][2].unwrap() ) {
            return Some(self.board[0][0]);
        }
        if
            self.board[0][2].is_some() && self.board[1][1].is_some() && self.board[2][0].is_some() &&
            (self.board[0][2].unwrap() == self.board[1][1].unwrap() && self.board[1][1].unwrap() == self.board[2][0].unwrap() ) {
            return Some(self.board[0][2]);
        }


        // Check for empty spaces
        for row in self.board.iter() {
            for position in row {
                if *position == None {
                    // There's still more to be played
                    return None;
                }
            }
        }

        // Game is a Cat
        return Some(None);
    }

    pub fn get_current_player(&self) -> &Player {
        match self.turn_number % 2 {
            0 => &self.starting_player,
            1 => &self.other_player,
            _ => unreachable!("There can only be 2 players in tictactoe... unless?")
        }
    }
    pub fn print_board(&self) {
        print!("{}", self.board_string());
    }
    pub fn board_string(&self) -> String {
        let mut s = String::with_capacity(100);
        s.push_str(&format!("{}\n", "-".repeat(25)));
        s.push_str(&format!("Turn: {}\n", self.turn_number));
        s.push_str(&format!("Current Player: {}\n", self.get_current_player()));
        s.push_str(&format!("Board:\n"));
        let left_offset = 20;

        for row in self.board.iter().take(1) {
            s.push_str(&format!("{}", " ".repeat(left_offset)));
            for position in row.iter().take(1) {
                match position {
                    Some(player) => s.push_str(&format!("{}", player)),
                    None => s.push_str(&format!(" "))
                };
            }
            for position in row.iter().skip(1) {
                s.push_str("│");
                match position {
                    Some(player) => s.push_str(&format!("{}", player)),
                    None => s.push_str(&format!(" "))
                };
            }
        }
        for row in self.board.iter().skip(1) {
            s.push_str(&format!("\n"));
            s.push_str(&format!("{}{}{}\n", " ".repeat(left_offset), "─┼".repeat( self.board.len()-1), "─"));
            s.push_str(&format!("{}", " ".repeat(left_offset)));
            for position in row.iter().take(1) {
                match position {
                    Some(player) => s.push_str(&format!("{}", player)),
                    None => s.push_str(&format!(" "))
                };
            }
            for position in row.iter().skip(1) {

                s.push_str(&format!("│"));
                match position {
                    Some(player) => s.push_str(&format!("{}", player)),
                    None => s.push_str(&format!(" "))
                };
            }
        }
        s.push_str(&format!("\n"));
        return s;
    }
}
