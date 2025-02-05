use std::net::{ IpAddr, SocketAddr };
use clap::{ command, Command, Arg };

mod tictactoe;
mod client;
mod server;
mod message;

#[macro_use] extern crate text_io;

fn main() {
    let matches = cmdline().get_matches();

    match matches.subcommand() {
        Some(("host", sub_matches)) => {
            let port_string = sub_matches.get_one::<u16>("port").expect("Error when trying to get port number").to_string();

            server::start(port_string);
        },
        Some(("join", sub_matches)) => {
            let port_string = sub_matches.get_one::<u16>("port").expect("Error when trying to get port number");
            let ip_string = sub_matches.get_one::<IpAddr>("address").expect("Error when trying to get ip address");
            let remote = SocketAddr::new(*ip_string, *port_string);
            client::start(remote);
        }
        _ => {
            let mut game_state = tictactoe::build_game_state(3, Some(3), None);
            tictactoe::game_loop(&mut game_state);
        }
    }
}

fn cmdline() -> clap::Command {
    let default_port = "6113";

    command!()
        .about("A little tictactoe game to demonstrate docker")
        .subcommand(
            Command::new("host")
                .about("Host the tictactoe server")
                .arg(
                    Arg::new("port")
                        .help("Port number to listen to connections.")
                        .value_parser(clap::value_parser!(u16).range(1024..))
                        .default_value(default_port)
                )
        )
        .subcommand(
            Command::new("join")
                .about("Joins a live swarm as a drone.")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("address")
                        .help("Ip address of the tictactoe server")
                        .required(true)
                        .value_parser(clap::value_parser!(IpAddr))
                )
                .arg(
                    Arg::new("port")
                        .help("Port number to connect to")
                        .value_parser(clap::value_parser!(u16).range(1024..))
                        .default_value(default_port)
                )
        )
    // btw, if there is no ; at the end of a line in rust it implicitly returns
}
