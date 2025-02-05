use serde::{Deserialize, Serialize};

use crate::tictactoe;

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub game_state: tictactoe::GameState,
    pub your_turn: bool
}

pub fn build_message(game_state: &tictactoe::GameState, your_turn: bool) -> Message {
    Message{
        game_state: game_state.clone(),
        your_turn
    }
}
