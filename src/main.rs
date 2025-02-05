use std::net::{ IpAddr, SocketAddr };
use clap::{ command, Command, Arg };

mod tictactoe;
mod client;
mod server;
mod message;
mod database;

#[macro_use] extern crate text_io;

fn main() {
    let matches = cmdline().get_matches();

    match matches.subcommand() {
        Some(("host", sub_matches)) => {
            let port_string = sub_matches.get_one::<u16>("port").expect("Error when trying to get port number").to_string();

            let db_port_string = sub_matches.get_one::<u16>("dbport").expect("Error when trying to get port number");

            // TODO: Accept Hostnames
            match sub_matches.get_one::<IpAddr>("db") {
                Some(ip) => {
                    let db = SocketAddr::new(*ip, *db_port_string);
                    server::start(port_string, Some(db));
                },
                None => {
                    server::start(port_string, None);
                }
            }

        },
        Some(("join", sub_matches)) => {
            let port_string = sub_matches.get_one::<u16>("port").expect("Error when trying to get port number");
            let ip_string = sub_matches.get_one::<IpAddr>("address").expect("Error when trying to get ip address");
            let remote = SocketAddr::new(*ip_string, *port_string);
            client::start(remote);
        }
        Some(("db", sub_matches)) => {
            let port_string = sub_matches.get_one::<u16>("port").expect("Error when trying to get port number").to_string();
            database::start(port_string);
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
                        .short('p')
                        .long("port")
                        .value_parser(clap::value_parser!(u16).range(1024..))
                        .default_value(default_port)
                ).arg(
                    Arg::new("db")
                        .short('d')
                        .long("dbaddr")
                        .help("Address for remote db")
                ).arg(
                    Arg::new("dbport")
                        .short('r')
                        .long("dbport")
                        .help("Port for remote db")
                        .value_parser(clap::value_parser!(u16).range(1024..))
                        .default_value("5000")
                )
        )
        .subcommand(
            Command::new("join")
                .about("Join tictactoe server")
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
        .subcommand(
            Command::new("db")
                .about("Start tictactoe db server")
                .arg(
                    Arg::new("port")
                        .help("Port of db server")
                        .value_parser(clap::value_parser!(u16).range(1024..))
                        .default_value("5000")
                )
        )
    // btw, if there is no ; at the end of the last line in a block in rust it implicitly returns
}
