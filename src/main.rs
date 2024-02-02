mod intro_packs;
mod programing_guessing_game;

use crate::intro_packs::intro_funcs::hello_world;
use crate::programing_guessing_game::guessing_game::guessing_game_play;

fn main() {
    hello_world();
    guessing_game_play();
}
