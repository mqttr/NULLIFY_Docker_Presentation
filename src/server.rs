use std::sync::mpsc;
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::thread;
use std::time::Duration;
use rand::Rng;
use std::io::{Read, Write};

use local_ip_address::local_ip;

use crate::tictactoe;
use crate::message::*;
use crate::database;

pub fn start(port: String, db_addr: Option<SocketAddr>) {


    // Communication between the db handler and all other threads including the main thread (main thread)
    let (client_database_send, client_database_recv) = mpsc::channel();
    match db_addr {
        Some(socket_addr) => {
            thread::spawn(move ||{
                handle_database(socket_addr, client_database_recv);
            });
        },
        None => {println!("Starting without DB")},
    }


    let mut last_client: Option<TcpStream> = None;
    loop {
        let address = "127.0.0.1:".to_owned();
        let listener = TcpListener::bind(address+&port).expect(&("Error when TCP binding to port ".to_owned()+&port));

        println!("Started TicTacToe Server...\nListening on {}:{}", local_ip().unwrap(), port);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let new_client_database_send = client_database_send.clone();

                    match last_client {
                        Some(s1) => {
                            thread::spawn(move || {
                                let _ =  handle_client(s1, stream, new_client_database_send);
                            });
                            last_client = None;
                        },
                        None => {
                            last_client = Some(stream);
                        }
                    }
                },
                Err(e) => println!("Stream error {:?}", e),
            }
        }
    };
}

fn handle_client(s1: std::net::TcpStream, s2: TcpStream, database_sender: mpsc::Sender<database::DatabaseMessage>) -> Result<tictactoe::Player, tictactoe::GameState> {
    let mut rng = rand::rng();
    let random = Rng::random_range(&mut rng, 0.0..1.0);
    let uuid: u32 = Rng::random(&mut rng);
    let (mut player1, mut player2) = if random <= 0.5 {
            (s1, s2)
        } else {
            (s2, s1)
        };

    let mut game_state = tictactoe::build_game_state(3, None, None);

    let mut out;
    let mut recv = [0; 10_000];

    out = match serde_json::to_string(&build_message(&game_state, true)) {
        Ok(o) => o,
        Err(..) => return Err(game_state)
    };
    match player1.write_all(&mut out.as_bytes()) {
        Ok(..) => {},
        Err(..) => return Err(game_state),
    }
    match player1.flush() {
        Ok(..) => {},
        Err(..) => return Err(game_state),
    }
    out = match serde_json::to_string(&build_message(&game_state, false)) {
        Ok(o) => o,
        Err(..) => return Err(game_state)
    };
    match player2.write_all(&mut out.as_bytes()) {
        Ok(..) => {},
        Err(..) => return Err(game_state),
    }
    match player2.flush() {
        Ok(..) => {},
        Err(..) => return Err(game_state),
    }
    let result = loop {
        // Player 1 Turn
        let size = match player1.read(&mut recv) {
            Ok(s) => s,
            Err(_) => continue,
        };
        let mut message: Message = match serde_json::from_slice(&recv[..size]) {
            Ok(m) => m,
            Err(..) => break game_state,
        };
        game_state = message.game_state;
        game_state.print_board();

        out = match serde_json::to_string(&build_message(&game_state, true)) {
            Ok(o) => o,
            Err(..) => break game_state
        };
        match player2.write_all(&mut out.as_bytes()) {
            Ok(..) => {},
            Err(..) => break game_state,
        }
        match player2.flush() {
            Ok(..) => {},
            Err(..) => break game_state,
        }
        // Player 2 Turn
        let size = match player2.read(&mut recv) {
            Ok(s) => s,
            Err(_) => continue,
        };
        message = match serde_json::from_slice(&recv[..size]) {
            Ok(m) => m,
            Err(..) => break game_state,
        };
        game_state = message.game_state;
        game_state.print_board();

        out = match serde_json::to_string(&build_message(&game_state, true)) {
            Ok(o) => o,
            Err(..) => break game_state,
        };
        match player1.write_all(&mut out.as_bytes()) {
            Ok(..) => {},
            Err(..) => break game_state,
        }
        match player1.flush() {
            Ok(..) => {},
            Err(..) => break game_state,
        }
    };

    let _ = database_sender.send(database::db_message_builder(uuid, result.clone(), Some(true)));
    return Err(result);
}

fn handle_database(db_addr: SocketAddr, recv: mpsc::Receiver<database::DatabaseMessage>) {
    println!("Connecting to {}", db_addr);
    let mut db_stream = loop {
        match TcpStream::connect(db_addr) {
            Ok(x) => break x,
            Err(e) => {
                println!("Unable to connect to remote {}\nERROR:{}", db_addr, e);
            }
        }
        thread::sleep(Duration::from_secs(2));
    };

    loop {
        let message = match recv.recv() {
            Err(..) => continue,
            Ok(m) => m,
        };
        let out = match serde_json::to_string(&message) {
            Ok(o) => o,
            Err(..) => continue,
        };
        match db_stream.write_all(&mut out.as_bytes()) {
            Ok(..) => {},
            Err(..) => continue,
        }
        match db_stream.flush() {
            Ok(..) => {},
            Err(..) => continue,
        }
    }
}
