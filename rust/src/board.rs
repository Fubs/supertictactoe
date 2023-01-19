use crate::gamemove::*;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Player {
    #[default] X,
    O,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Square {
    #[default] Empty,
    Occupied(Player),
}

impl From<Square> for Player {
    fn from(square: Square) -> Self {
        debug_assert!(square != Square::Empty);
        match square {
            Square::Occupied(player) => player,
            _ => panic!("Square is empty"),
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub struct Board {
    pub squares: [[Square; 3]; 3],
}

impl Board {
    pub fn new() -> Board {
        Board {
            squares: [[Square::default(); 3]; 3],
        }
    }

    pub fn do_move(&mut self, gamemove: GameMove) {
        self.squares[gamemove.idx[0] as usize][gamemove.idx[1] as usize] = Square::Occupied(gamemove.player);
    }

    pub fn print(&self) {
        for row in self.squares.iter() {
            for square in row.iter() {
                match square {
                    Square::Empty => print!("."),
                    Square::Occupied(player) => match player {
                        Player::X => print!("X"),
                        Player::O => print!("O"),
                    },
                }
            }
            println!();
        }
    }

}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct BigBoard{
    pub subboards: [[Board; 3]; 3], 
}

impl BigBoard {
    pub fn new() -> BigBoard {
        BigBoard {
            subboards: [[Board::new(); 3]; 3],
        }
    }

    pub fn do_move(&mut self, biggamemove: BigGameMove) {
        self.subboards[biggamemove.idx[0] as usize][biggamemove.idx[1] as usize].do_move(GameMove::new(biggamemove.sub_idx, biggamemove.player));
    }
    
    pub fn print(&self) {
        println!("-------------------------- coords:");
        for big_row in 0..9 {
            if big_row % 3 == 0 && big_row != 0 {
                println!("-------------------------- ");
            }
            print!("| ");
            for big_col in 0..9 {
                if big_col % 3 == 0 && big_col != 0 {
                    print!("| ");
                }
                let sub_row = big_row / 3;
                let sub_col = big_col / 3;
                let row = big_row % 3;
                let col = big_col % 3;
                let square = self.subboards[sub_row][sub_col].squares[row][col];
                match square {
                    Square::Empty => print!("· "),
                    Square::Occupied(player) => match player {
                        Player::X => print!("X "),
                        Player::O => print!("O "),
                    },
                }
            }
            print!(" |");
            match big_row {
                0 => print!(" a1 a2 a3 | b1 b2 b3 | c1 c2 c3"),
                1 => print!(" a4 a5 a6 | b4 b5 b6 | c4 c5 c6"),
                2 => print!(" a7 a8 a9 | b7 b8 b9 | c7 c8 c9"),
                3 => print!(" d1 d2 d3 | e1 e2 e3 | f1 f2 f3"),
                4 => print!(" d4 d5 d6 | e4 e5 e6 | f4 f5 f6"),
                5 => print!(" d7 d8 d9 | e7 e8 e9 | f7 f8 f9"),
                6 => print!(" g1 g2 g3 | h1 h2 h3 | i1 i2 i3"),
                7 => print!(" g4 g5 g6 | h4 h5 h6 | i4 i5 i6"),
                8 => print!(" g7 g8 g9 | h7 h8 h9 | i7 i8 i9"),
                _ => unreachable!(),
            }
            println!();
        }
        println!("--------------------------");

    }

}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct Game {
    pub bigboard: BigBoard,
    pub subboard_wins: [[Option<Player>; 3]; 3],
    pub turn: Player,
    pub winner: Option<Player>,
    pub last_move: Option<BigGameMove>,
    pub prev_state: Option<Box<Game>>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            bigboard: BigBoard::new(),
            subboard_wins: [[None; 3]; 3],
            turn: Player::default(),
            winner: None,
            last_move: None,
            prev_state: None,
        }
    }

    pub fn print(&self) {
        println!("=== Game ===");
        self.bigboard.print();
        println!("  Turn: {:?}", self.turn);
        //println!("  Winner: {:?}", self.winner);
        println!("  Last move: {}", bgm2str(&self.last_move.unwrap_or(BigGameMove::new([0, 0], [0, 0], Player::default()))));
        /*println!("  Subboard wins:");
        for row in self.subboard_wins.iter() {
            print!("    ");
            for square in row.iter() {
                match square {
                    None => print!("."),
                    Some(player) => match player {
                        Player::X => print!("X"),
                        Player::O => print!("O"),
                    },
                }
            }
            println!();
        }*/
        println!("  Legal Moves:");
        if self.is_next_move_anywhere(){
            println!("    Anywhere");
        } else {
            print!("    ");
            for m in self.get_moves() {
                print!("{}, ", bgm2str(&m));
            }
            println!();
        }
    }

    pub fn print_wins(&self) {
        for row in self.subboard_wins.iter() {
            for square in row.iter() {
                match square {
                    None => print!("."),
                    Some(player) => match player {
                        Player::X => print!("X"),
                        Player::O => print!("O"),
                    },
                }
            }
            println!();
        }
    }

    pub fn is_next_move_anywhere(&self) -> bool {
        if self.last_move.is_none() {
            return true;
        }
        let last_move = self.last_move.unwrap();
        self.subboard_wins[last_move.sub_idx[0] as usize][last_move.sub_idx[1] as usize].is_some()
    }

    pub fn do_move(&mut self, biggamemove: BigGameMove) {
        debug_assert!(biggamemove.player == self.turn);
        self.prev_state = Some(Box::new(self.clone()));
        self.bigboard.do_move(biggamemove);
        self.last_move = Some(biggamemove);
        self.turn = match self.turn {
            Player::X => Player::O,
            Player::O => Player::X,
        };
        //check if subboard is won
        let subboard = &self.bigboard.subboards[biggamemove.idx[0] as usize][biggamemove.idx[1] as usize];
        let w = subboard.winner();
        if let Some(winner) = w {
            self.subboard_wins[biggamemove.idx[0] as usize][biggamemove.idx[1] as usize] = Some(winner);
        }

        //check if bigboard is won
        let w = self.winner();
        if let Some(winner) = w {
            self.winner = Some(winner);
        }

    }

    pub fn undo_move(&mut self) {
        debug_assert!(self.last_move.is_some());
        debug_assert!(self.prev_state.is_some());
        self.bigboard = self.prev_state.as_ref().unwrap().bigboard;
        self.subboard_wins = self.prev_state.as_ref().unwrap().subboard_wins;
        self.turn = self.prev_state.as_ref().unwrap().turn;
        self.winner = self.prev_state.as_ref().unwrap().winner;
        self.last_move = self.prev_state.as_ref().unwrap().last_move;
        self.prev_state = None;
    }

    pub fn get_moves(&self) -> Vec<BigGameMove> {
        debug_assert!(self.winner.is_none());
        let mut moves = Vec::new();
        if let Some(last_move) = self.last_move {
            //next turn's moves must be in the subboard pointed-to by last_move
            if self.subboard_wins[last_move.sub_idx[0] as usize][last_move.sub_idx[1] as usize].is_some(){
                for big_row in 0..3 {
                    for big_col in 0..3 {
                        let subboard = &self.bigboard.subboards[big_row][big_col];
                        for row in 0..3 {
                            for col in 0..3 {
                                if subboard.squares[row][col] == Square::Empty {
                                    moves.push(BigGameMove::new([big_row as u8, big_col as u8], [row as u8, col as u8], self.turn));
                                }
                            }
                        }
                    }
                }
            }
            else{
                let subboard = &self.bigboard.subboards[last_move.sub_idx[0] as usize][last_move.sub_idx[1] as usize];
                for row in 0..3 {
                    for col in 0..3 {
                        if subboard.squares[row][col] == Square::Empty {
                            moves.push(BigGameMove::new([last_move.sub_idx[0] as u8, last_move.sub_idx[1] as u8], [row as u8, col as u8], self.turn));
                        }
                    }
                }
        
            }
            
        } else {
            //first turn, all moves are valid
            for big_row in 0..3 {
                for big_col in 0..3 {
                    let subboard = &self.bigboard.subboards[big_row][big_col];
                    for row in 0..3 {
                        for col in 0..3 {
                            if subboard.squares[row][col] == Square::Empty {
                                moves.push(BigGameMove::new([big_row as u8, big_col as u8], [row as u8, col as u8], self.turn));
                            }
                        }
                    }
                }
            }
        }
        moves
    }
}


//Winner trait
pub trait Win {
    fn winner(&self) -> Option<Player>;
}

impl Win for Board {
    fn winner(&self) -> Option<Player> {
        let mut winner = None;
        for i in 0..3 {
            if self.squares[i][0] == self.squares[i][1] && self.squares[i][1] == self.squares[i][2] && self.squares[i][0] != Square::Empty {
                winner = Some(self.squares[i][0].into());
            }
            if self.squares[0][i] == self.squares[1][i] && self.squares[1][i] == self.squares[2][i] && self.squares[0][i] != Square::Empty {
                winner = Some(self.squares[0][i].into());
            }
        }
        if self.squares[0][0] == self.squares[1][1] && self.squares[1][1] == self.squares[2][2]  && self.squares[0][0] != Square::Empty {
            winner = Some(self.squares[0][0].into());
        }
        if self.squares[0][2] == self.squares[1][1] && self.squares[1][1] == self.squares[2][0]  && self.squares[0][2] != Square::Empty {
            winner = Some(self.squares[0][2].into());
        }
        winner
    }
}

impl Win for BigBoard {
    fn winner(&self) -> Option<Player> {
        let mut winner = None;
        for i in 0..3 {
            if self.subboards[i][0].winner() == self.subboards[i][1].winner() && self.subboards[i][1].winner() == self.subboards[i][2].winner()  && self.subboards[i][0].winner() != None {
                winner = self.subboards[i][0].winner();
            }
            if self.subboards[0][i].winner() == self.subboards[1][i].winner() && self.subboards[1][i].winner() == self.subboards[2][i].winner()  && self.subboards[0][i].winner() != None{
                winner = self.subboards[0][i].winner();
            }
        }
        if self.subboards[0][0].winner() == self.subboards[1][1].winner() && self.subboards[1][1].winner() == self.subboards[2][2].winner()  && self.subboards[0][0].winner() != None{
            winner = self.subboards[0][0].winner();
        }
        if self.subboards[0][2].winner() == self.subboards[1][1].winner() && self.subboards[1][1].winner() == self.subboards[2][0].winner()  && self.subboards[0][2].winner() != None{
            winner = self.subboards[0][2].winner();
        }
        winner
    }

}

impl Win for Game {
    fn winner(&self) -> Option<Player> {
        self.bigboard.winner()
    }
}


