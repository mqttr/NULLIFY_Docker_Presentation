use std::sync::mpsc;
use std::net::{TcpListener, TcpStream};
use std::thread;
use rand::Rng;
use std::io::{Read, Write};

use local_ip_address::local_ip;

use crate::tictactoe;
use crate::message::*;

pub fn start(port: String) {
    // Communication between the db handler and all other threads including the main thread (main thread)
    let (client_database_send, client_database_recv) = mpsc::channel();

    thread::spawn(move ||{
        handle_database(client_database_recv);
    });


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
                                handle_client(s1, stream, new_client_database_send);
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

fn handle_client(s1: std::net::TcpStream, s2: TcpStream, database_sender: mpsc::Sender<String>) -> Result<tictactoe::Player, tictactoe::GameState> {
    let mut rng = rand::rng();
    let random = Rng::random_range(&mut rng, 0.0..1.0);
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
    loop {
        // Player 1 Turn
        let size = match player1.read(&mut recv) {
            Ok(s) => s,
            Err(_) => continue,
        };
        let mut message: Message = match serde_json::from_slice(&recv[..size]) {
            Ok(m) => m,
            Err(..) => return Err(game_state),
        };
        game_state = message.game_state;
        game_state.print_board();

        out = match serde_json::to_string(&build_message(&game_state, true)) {
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
        // Player 2 Turn
        let size = match player2.read(&mut recv) {
            Ok(s) => s,
            Err(_) => continue,
        };
        message = match serde_json::from_slice(&recv[..size]) {
            Ok(m) => m,
            Err(..) => return Err(game_state),
        };
        game_state = message.game_state;
        game_state.print_board();

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
    }
}

fn handle_database(recv: mpsc::Receiver<String>) {

}


fn send_string(stream: &mut TcpStream, message: &str) -> Result<(), std::io::Error> {
    stream.write_all(&message.bytes().collect::<Vec<u8>>())?;
    stream.flush()?;
    return Ok(());
}
