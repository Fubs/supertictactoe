#![allow(unused_imports, dead_code)]
mod board;
mod gamemove;
mod eval;
mod search;
mod play;
mod gui;
use sfml::*;
use board::*;
use gamemove::*;
use eval::*;
use search::*;
use play::*;
use gui::*;

fn main() {
    start_gui();
    //play();

    /*
    let mut search_depth = 2;
    let mut node_count:u64 = 0;
    let mut pv:Vec<BigGameMove> = Vec::new();

    while search_depth < 20 {
        let score = alphabeta(&mut game, search_depth, -1000000, 1000000, &mut pv);
        //node_count = nc;

        println!("Score: {}", score);
        //println!("Node count: {}", node_count);
        println!("PV: {:?}", pv);
        let mut pv_board = Game::new();
        for mv in pv.iter() {
            pv_board.do_move(*mv);
        }
        pv_board.print();

        search_depth += 1;

    }
    */




    
}
