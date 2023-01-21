use crate::board::*;
use crate::gamemove::*;

pub trait Score {
    fn score(&self) -> i64;
}

impl Score for Board {
    fn score(&self) -> i64 {
        if let Some(winner) = self.winner() {
            match winner {
                Player::X => 1000,
                Player::O => -1000,
            }
        } else {
            let win_lines:[[[usize;2];3];8] = [
                [[0,0],[0,1],[0,2]],
                [[1,0],[1,1],[1,2]],
                [[2,0],[2,1],[2,2]],
                [[0,0],[1,0],[2,0]],
                [[0,1],[1,1],[2,1]],
                [[0,2],[1,2],[2,2]],
                [[0,0],[1,1],[2,2]],
                [[0,2],[1,1],[2,0]]
            ];
            let mut x_score: [i64; 8] = [0; 8];
            let mut wc = 0;
            for wl in win_lines.iter() {
                for i in wl.iter() {
                    if self.squares[i[0]][i[1]] == Square::Occupied(Player::O) {
                        x_score[wc] = 0;
                        break;
                    }
                    if self.squares[i[0]][i[1]] == Square::Occupied(Player::X) {
                        x_score[wc] += 1;
                    }
                }
                wc += 1;
            }

            let mut o_score: [i64; 8] = [0; 8];
            wc = 0;
            for wl in win_lines.iter() {
                for i in wl.iter() {
                    if self.squares[i[0]][i[1]] == Square::Occupied(Player::X) {
                        o_score[wc] = 0;
                        break;
                    }
                    if self.squares[i[0]][i[1]] == Square::Occupied(Player::O) {
                        o_score[wc] += 1;
                    }
                }
                wc += 1;
            }

            let mut x_total = 0;
            let mut o_total = 0;
            let mut x_one_move_win = false;
            let mut o_one_move_win = false;

            for i in x_score.iter() {
                if *i == 2 {
                    x_one_move_win = true;
                    break;
                }
            }

            for i in o_score.iter() {
                if *i == 2 {
                    o_one_move_win = true;
                    break;
                }
            }

            for i in x_score.iter() {
                x_total += *i;
            }

            for i in o_score.iter() {
                o_total += *i;
            }

            if x_one_move_win {
                x_total += 15;
            }

            if o_one_move_win {
                o_total += 15;
            }

            x_total - o_total
    }
        }
}

impl Score for Game {
    fn score(&self) -> i64 {
        let mut score = 0;
        let board_idxes = [[0,0],[0,1],[0,2],[1,0],[1,1],[1,2],[2,0],[2,1],[2,2]];
        let mults = [3,2,3,2,4,2,3,2,3];
        for i in 0..9 {
            let idx = board_idxes[i];
            let mult = mults[i];
            score += self.bigboard.subboards[idx[0]][idx[1]].score() * mult;
        }

        score
    }
}

