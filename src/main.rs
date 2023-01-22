mod board;
mod eval;
mod gamemove;
mod gui;
mod play;
mod search;
use board::*;
use eval::*;
use gamemove::*;
use gui::*;
use play::*;
use search::*;
use sfml::*;

fn main() {
    //play_gui();
    play_cli();
}
