use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;
use std::fs;
use std::io::{Read, Write};

use serde::{Deserialize, Serialize};

use local_ip_address::local_ip;

use crate::tictactoe::*;

#[derive(Serialize, Deserialize)]
pub struct DatabaseMessage {
    uuid: u32,
    game_state: GameState,
    game_over: bool
}

pub fn db_message_builder(uuid: u32, game_state: GameState, game_over: Option<bool>) -> DatabaseMessage {
    DatabaseMessage {
        uuid,
        game_state,
        game_over: game_over.unwrap_or(false)
    }
}

pub fn start(port: String) {
    let address = "127.0.0.1:".to_owned();
    let listener = TcpListener::bind(address+&port).expect(&("Error when TCP binding to port ".to_owned()+&port));
    println!("Started TicTacToe Server...\nListening on {}:{}", local_ip().unwrap(), port);

    let (client_database_send, client_database_recv) = mpsc::channel();

    thread::spawn(move ||{
        handle_io(client_database_recv);
    });

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let new_client_database_send = client_database_send.clone();

                thread::spawn(move || {
                    let _ =  handle_connection(stream, new_client_database_send);
                });

            },
            Err(e) => println!("Stream error {:?}", e),
        }
    }
}

fn handle_io(recv: mpsc::Receiver<DatabaseMessage>) {
    fs::create_dir_all("./data").expect("Could not create dir");

    loop {
        match recv.recv() {
            Ok(msg) => {
                let name = "./data/".to_owned()+&msg.uuid.to_string()+&".log";

                let mut file = match fs::exists(name.clone()) {
                    Err(e) => { eprint!("Coulnd't exist file {}", e); continue;},
                    Ok(b) => {
                        match b {
                            true => {
                                fs::OpenOptions::new()
                                    .write(true)
                                    .append(true)
                                    .open(name)
                                    .unwrap()
                            },
                            false => {
                                fs::File::create(name.clone()).unwrap()
                            }
                        }
                    }
                };

                if let Err(e) = writeln!(file, "{}", msg.game_state.board_string()) {
                    eprint!("Couldn't open file {}", e);
                }
            },
            Err(..) => continue,
        }
    }

}
fn handle_connection(mut stream: TcpStream, send: mpsc::Sender<DatabaseMessage>) {
    let mut buf = [0; 10_000];
    loop {
        match stream.read(&mut buf) {
            Ok(size) if size >= 1 => {
                let out: DatabaseMessage = match serde_json::from_slice(&buf[..size]) {
                    Ok(o) => o,
                    Err(..) => continue,
                };

                let _ = send.send(out);
            },
            _ => continue,
        }
    }
}
