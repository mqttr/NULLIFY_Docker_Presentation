use clap::command;

fn main() {
    let matches = cmdline().get_matches();

    match matches.subcommand() {
        _ => {
            println!("Yay! You're playing tictactoe right now!!! In your mind!!!!!")
        }
    }
}

fn cmdline() -> clap::Command {
    command!()
        .about("A little tictactoe game to demonstrate docker")
    // btw, if there is no ; at the end of a line in rust it implicitly returns
}
