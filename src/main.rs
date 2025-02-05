use clap::command;

mod tictactoe;

#[macro_use] extern crate text_io;

fn main() {
    let matches = cmdline().get_matches();

    match matches.subcommand() {
        _ => {
            let mut game_state = tictactoe::build_game_state(3, Some(3), None);
            tictactoe::game_loop(&mut game_state);
        }
    }
}

fn cmdline() -> clap::Command {
    command!()
        .about("A little tictactoe game to demonstrate docker")
    // btw, if there is no ; at the end of a line in rust it implicitly returns
}
