use std::net::{ SocketAddr, TcpStream };

use std::io::{Read, Write};

use crate::tictactoe::*;
use crate::message::*;

pub fn start(remote: SocketAddr) -> Option<Player> {
    println!("Connecting to {}", remote);
    let mut stream = match TcpStream::connect(remote) {
        Ok(x) => x,
        Err(e) => {
            println!("Unable to connect to remote {}\nERROR:{}", remote, e);
            std::process::exit(0);
        }
    };

    println!("Accepted by Server: {}", stream.peer_addr().unwrap());

    let mut recv = [0; 10_000];
    let mut game_state;
    let mut out;
    loop {
        match stream.peek(&mut recv) {
            Ok(mut size) if size >= 1 => {
                size = stream.read(&mut recv).unwrap();

                let message: Message = serde_json::from_slice(&recv[..size]).unwrap();

                game_state = message.game_state;
                game_state.print_board();

                match game_state.check_winner() {
                    None => {},
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

                match message.your_turn {
                    true => {
                        game_state = loop {
                            let (x, y) = get_coords(game_state.get_current_player(), game_state.board.len(), None);

                            match game_state.place(x, y) {
                                Ok(_) => {break game_state},
                                Err(e) => {
                                    println!("{}", e);
                                    print!("Press enter to continue...");
                                    let _: String = read!("{}\n");
                                },
                            }
                        };
                        out = match serde_json::to_string(&build_message(&game_state, true)) {
                            Ok(o) => o,
                            Err(..) => {
                                println!("Server disconnected...");
                                std::process::exit(1);
                            }
                        };
                        match stream.write_all(&mut out.as_bytes()) {
                            Ok(..) => {},
                            Err(..) => {
                                println!("Server disconnected...");
                                std::process::exit(1);
                            },
                        }
                        match stream.flush() {
                            Ok(..) => {},
                            Err(..) => {
                                println!("Server disconnected...");
                                std::process::exit(1);
                            },
                        }
                        game_state.print_board();
                    },
                    false => {
                        println!("It's not your turn yet!!");
                    }
                }
                match game_state.check_winner() {
                    None => {},
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
            _ => continue,
        }
    }
}
