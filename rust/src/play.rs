use crate::board::*;
use crate::gamemove::*;
use crate::eval::*;
use crate::search::*;
use std::io;
use std::io::Write;
use std::io::BufRead;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub trait MoveSelect {
    fn choose_move(game: Game) -> BigGameMove;
}

pub struct Engine;

impl Engine {
    fn new() -> Engine {
        Engine{}
    }
}

impl MoveSelect for Engine {

    fn choose_move(game: Game) -> BigGameMove{
        println!("Thinking...");
        let mut game_cpy = game.clone();
        let mut pv = vec![];
        //run engine at depth 8 for a while, if it takes longer than 10 seconds then rerun at depth 6
        let (sender, receiver) = mpsc::channel();
        let t = std::thread::spawn(move || {
            match game.turn {
                Player::X => {
                    let score = alphabeta(&mut game_cpy, 8, -1000000, 1000000, &mut pv);
                    match sender.send(pv) {
                        Ok(()) => {}, // everything good
                        Err(_) => {}, // we have been released, don't panic
                    }
                },
                Player::O => {
                    let score = alphabeta(&mut game_cpy, 8, 1000000, -1000000, &mut pv);
                    match sender.send(pv) {
                        Ok(()) => {}, // everything good
                        Err(_) => {}, // we have been released, don't panic
                    }
                },
            }
        });
        let result = receiver.recv_timeout(Duration::from_millis(10000));
        if result.is_err() {
            println!("Engine took longer than 10 sec, rerunning at reduced depth");
            match game.turn {
                Player::X => {
                    let mut game_cpy2 = game.clone();
                    let mut pv2= vec![];
                    let score = alphabeta(&mut game_cpy2, 6, -1000000, 1000000, &mut pv2);
                    return pv2[0];
                },
                Player::O => {
                    let mut game_cpy2 = game.clone();
                    let mut pv2= vec![];
                    let _score = alphabeta(&mut game_cpy2, 6, 1000000, -1000000, &mut pv2);
                    return pv2[0];
                },
            }
            
        }
        else{ 
            return result.unwrap()[0];
        }
    }
    
}


pub struct Human;

impl Human{
    fn new() -> Human{
        Human{}
    }
}

impl MoveSelect for Human{

    fn choose_move(game: Game) -> BigGameMove{
        let mut game_cpy = game.clone();
        game_cpy.print();

        let mut line = String::new();

        //println!("Enter coords as four numbers a b c d -> board [a,b], square [c,d] >");
        //println!("or enter two numbers a b ");
        println!("Enter coords as a letter and number >");

        let b1 = std::io::stdin().read_line(&mut line).unwrap();
        while line.trim() == "h" || line.trim() == "" {
            if line.trim() == "h" {
                println!("Board coordinates:");
                println!("a1 a2 a3 | b1 b2 b3 | c1 c2 c3");
                println!("a4 a5 a6 | b4 b5 b6 | c4 c5 c6");
                println!("a7 a8 a9 | b7 b8 b9 | c7 c8 c9");
                println!("------------------------------");
                println!("d1 d2 d3 | e1 e2 e3 | f1 f2 f3");
                println!("d4 d5 d6 | e4 e5 e6 | f4 f5 f6");
                println!("d7 d8 d9 | e7 e8 e9 | f7 f8 f9");
                println!("------------------------------");
                println!("g1 g2 g3 | h1 h2 h3 | i1 i2 i3");
                println!("g4 g5 g6 | h4 h5 h6 | i4 i5 i6");
                println!("g7 g8 g9 | h7 h8 h9 | i7 i8 i9");
                line.clear();
                let b1 = std::io::stdin().read_line(&mut line).unwrap();
            }
            else {
                line.clear();
                let b1 = std::io::stdin().read_line(&mut line).unwrap();
            }
        }
        let m = str2bgm_checked(&line, Player::O);
        match m {
            Some(m) => {
                if game_cpy.get_moves().contains(&m) {
                    return m;
                }
                else {
                    println!("Invalid move, try again");
                    return Human::choose_move(game);
                }
            },
            None => {
                println!("Invalid move, try again");
                return Human::choose_move(game);
            },
        }
    }
}

pub fn play() {
    let mut game = Game::new();
    while game.winner().is_none() {
        let emv = Engine::choose_move(game.clone());
        println!("Engine chose {}", bgm2str(&emv));
        game.do_move(emv);
        let hmv = Human::choose_move(game.clone());
        game.do_move(hmv);
    }
    match game.winner() {
        Some(winner) => println!("Winner is {:?}", winner),
        None => println!("No winner, error?"),
    }
}

