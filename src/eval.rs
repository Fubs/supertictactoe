use crate::board::*;

pub trait Score {
    fn score(&self) -> i32;
}

impl Score for Board {
    fn score(&self) -> i32 {
        if self.winner().is_some() {
            //returns the same score for either player's win, because when .score() is called on a Game,
            //it will be negated if it is Player::O's turn. (the side-to-move is not stored in a Board)
            40
        } else {
            let win_lines: [[[usize; 2]; 3]; 8] = [
                [[0, 0], [0, 1], [0, 2]],
                [[1, 0], [1, 1], [1, 2]],
                [[2, 0], [2, 1], [2, 2]],
                [[0, 0], [1, 0], [2, 0]],
                [[0, 1], [1, 1], [2, 1]],
                [[0, 2], [1, 2], [2, 2]],
                [[0, 0], [1, 1], [2, 2]],
                [[0, 2], [1, 1], [2, 0]],
            ];
            let mut x_score: [i32; 8] = [0; 8]; //Player::X's score for each of the 8 win_lines
            let mut o_score: [i32; 8] = [0; 8]; //Player::O's score for each of the 8 win_lines
            let mut c = 0;
            for wl in win_lines.iter() {
                for i in wl.iter() {
                    match self.squares[i[0]][i[1]] {
                        //two squares on the same win_line is better than two non-connectable squares,
                        //so score should have some kind of nonlinear increase for each additional connectable square
                        Square::Occupied(Player::X) => x_score[c] += x_score[c] + 1,
                        Square::Occupied(Player::O) => o_score[c] += o_score[c] + 1,
                        Square::Empty => {}
                    }
                    //if both players have a square in this win_line, then neither can win it
                    //so zero points for both players
                    if x_score[c] > 0 && o_score[c] > 0 {
                        x_score[c] = 0;
                        o_score[c] = 0;
                    }
                }
                c += 1;
            }

            //the search functions use negamax layout, so static eval functions like this need to flip
            //such that it returns a score where positive is good for the side-to-move. Board does not store turn info,
            //so remember to negate this when calling from a Games's .score() method if it is Player::O's turn
            x_score.iter().sum::<i32>() - o_score.iter().sum::<i32>()
        }
    }
}

impl Score for Game {
    fn score(&self) -> i32 {
        if self.winner().is_some() {
            match self.winner().unwrap() {
                Player::X => return std::i32::MAX,
                Player::O => return std::i32::MIN,
            }
        }

        let mut score = 0;
        let mut board_scores: [[i32; 3]; 3] = [[0i32; 3]; 3];

        //weights is how many win_lines each subboard is on.
        //used as a multiplier for the subboard's score
        let weights = [[3, 2, 3], [2, 4, 2], [3, 2, 3]];

        let win_lines: [[[usize; 2]; 3]; 8] = [
            [[0, 0], [0, 1], [0, 2]],
            [[1, 0], [1, 1], [1, 2]],
            [[2, 0], [2, 1], [2, 2]],
            [[0, 0], [1, 0], [2, 0]],
            [[0, 1], [1, 1], [2, 1]],
            [[0, 2], [1, 2], [2, 2]],
            [[0, 0], [1, 1], [2, 2]],
            [[0, 2], [1, 1], [2, 0]],
        ];

        for r in 0..3 {
            for c in 0..3 {
                let m = weights[r][c];
                board_scores[r][c] = self.bigboard.subboards[r][c].score() * {
                    if self.turn == Player::X {
                        1
                    } else {
                        -1
                    }
                };
                score += board_scores[r][c] * m;
            }
        }

        //a winning (not necessarily won yet) subboard is even better if the other boards on its
        //win_lines are also winning, so this should be taken into account
        let mut line_scores = [0; 8];
        let mut c = 0;
        for wl in win_lines.iter() {
            let sum = wl.iter().map(|a| board_scores[a[0]][a[1]]).sum::<i32>();
            line_scores[c] = sum;
            c += 1;
        }

        // negamax layout, so static eval functions like this need to be negated if it is Player::O's turn
        if self.turn == Player::X {
            score
        } else {
            -score
        }
    }
}
