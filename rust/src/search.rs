use crate::board::*;
use crate::gamemove::*;
use crate::eval::*;

pub fn alphabeta(game: &mut Game, depth: usize, mut alpha: i64, mut beta: i64, line: &mut Vec<BigGameMove>) -> i64 {
    
    if depth == 0 {
        return game.score();
    }

    let mut next_line:Vec<BigGameMove> = Vec::new();
    let moves = game.get_moves();

    for m in moves.iter() {
        let mut game_copy = game.clone();
        game_copy.do_move(*m);
        let score = -alphabeta(&mut game_copy, depth - 1, -beta, -alpha, &mut next_line);
        if score >= beta {
            return beta;
        }
        if score > alpha {
            alpha = score;
            line.clear();
            line.push(m.clone());
            line.append(&mut next_line);
        }
    }

    return alpha;
}
