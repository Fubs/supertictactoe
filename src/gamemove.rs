use crate::board::*;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct GameMove {
    pub idx: [u8;2],
    pub player: Player,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct BigGameMove {
    pub idx: [u8;2],
    pub sub_idx: [u8;2],
    pub player: Player,
}

impl GameMove {
    pub fn new(idx: [u8;2], player: Player) -> GameMove {
        GameMove {
            idx,
            player,
        }
    }
}

impl BigGameMove {
    pub fn new(idx: [u8;2], sub_idx: [u8;2], player: Player) -> BigGameMove {
        BigGameMove {
            idx,
            sub_idx,
            player,
        }
    }
}

pub fn bgm_x(idx: [u8;2], sub_idx: [u8;2]) -> BigGameMove {
    BigGameMove::new(idx, sub_idx, Player::X)
}

pub fn bgm_o(idx: [u8;2], sub_idx: [u8;2]) -> BigGameMove {
    BigGameMove::new(idx, sub_idx, Player::O)
}

pub fn gm_x(idx: [u8;2]) -> GameMove {
    GameMove::new(idx, Player::X)
}

pub fn gm_o(idx: [u8;2]) -> GameMove {
    GameMove::new(idx, Player::O)
}

pub fn bgm2str(bgm: &BigGameMove) -> String {
    match bgm.idx {
        [0,0] => match bgm.sub_idx {
            [0,0] => "a1".to_string(),
            [0,1] => "a2".to_string(),
            [0,2] => "a3".to_string(),
            [1,0] => "a4".to_string(),
            [1,1] => "a5".to_string(),
            [1,2] => "a6".to_string(),
            [2,0] => "a7".to_string(),
            [2,1] => "a8".to_string(),
            [2,2] => "a9".to_string(),
            _ => "error".to_string(),
        },
        [0,1] => match bgm.sub_idx {
            [0,0] => "b1".to_string(),
            [0,1] => "b2".to_string(),
            [0,2] => "b3".to_string(),
            [1,0] => "b4".to_string(),
            [1,1] => "b5".to_string(),
            [1,2] => "b6".to_string(),
            [2,0] => "b7".to_string(),
            [2,1] => "b8".to_string(),
            [2,2] => "b9".to_string(),
            _ => "error".to_string(),
        },
        [0,2] => match bgm.sub_idx {
            [0,0] => "c1".to_string(),
            [0,1] => "c2".to_string(),
            [0,2] => "c3".to_string(),
            [1,0] => "c4".to_string(),
            [1,1] => "c5".to_string(),
            [1,2] => "c6".to_string(),
            [2,0] => "c7".to_string(),
            [2,1] => "c8".to_string(),
            [2,2] => "c9".to_string(),
            _ => "error".to_string(),
        },
        [1,0] => match bgm.sub_idx {
            [0,0] => "d1".to_string(),
            [0,1] => "d2".to_string(),
            [0,2] => "d3".to_string(),
            [1,0] => "d4".to_string(),
            [1,1] => "d5".to_string(),
            [1,2] => "d6".to_string(),
            [2,0] => "d7".to_string(),
            [2,1] => "d8".to_string(),
            [2,2] => "d9".to_string(),
            _ => "error".to_string(),
        },
        [1,1] => match bgm.sub_idx {
            [0,0] => "e1".to_string(),
            [0,1] => "e2".to_string(),
            [0,2] => "e3".to_string(),
            [1,0] => "e4".to_string(),
            [1,1] => "e5".to_string(),
            [1,2] => "e6".to_string(),
            [2,0] => "e7".to_string(),
            [2,1] => "e8".to_string(),
            [2,2] => "e9".to_string(),
            _ => "error".to_string(),
        },
        [1,2] => match bgm.sub_idx {
            [0,0] => "f1".to_string(),
            [0,1] => "f2".to_string(),
            [0,2] => "f3".to_string(),
            [1,0] => "f4".to_string(),
            [1,1] => "f5".to_string(),
            [1,2] => "f6".to_string(),
            [2,0] => "f7".to_string(),
            [2,1] => "f8".to_string(),
            [2,2] => "f9".to_string(),
            _ => "error".to_string(),
        },
        [2,0] => match bgm.sub_idx {
            [0,0] => "g1".to_string(),
            [0,1] => "g2".to_string(),
            [0,2] => "g3".to_string(),
            [1,0] => "g4".to_string(),
            [1,1] => "g5".to_string(),
            [1,2] => "g6".to_string(),
            [2,0] => "g7".to_string(),
            [2,1] => "g8".to_string(),
            [2,2] => "g9".to_string(),
            _ => "error".to_string(),
        },
        [2,1] => match bgm.sub_idx {
            [0,0] => "h1".to_string(),
            [0,1] => "h2".to_string(),
            [0,2] => "h3".to_string(),
            [1,0] => "h4".to_string(),
            [1,1] => "h5".to_string(),
            [1,2] => "h6".to_string(),
            [2,0] => "h7".to_string(),
            [2,1] => "h8".to_string(),
            [2,2] => "h9".to_string(),
            _ => "error".to_string(),
        },
        [2,2] => match bgm.sub_idx {
            [0,0] => "i1".to_string(),
            [0,1] => "i2".to_string(),
            [0,2] => "i3".to_string(),
            [1,0] => "i4".to_string(),
            [1,1] => "i5".to_string(),
            [1,2] => "i6".to_string(),
            [2,0] => "i7".to_string(),
            [2,1] => "i8".to_string(),
            [2,2] => "i9".to_string(),
            _ => "error".to_string(),
        },
        _ => "error".to_string(),
    }
}

pub fn str2bgm(s: &str, p: Player) -> BigGameMove {
    let mut bgm = BigGameMove{idx: [0,0], sub_idx: [0,0], player: p};
    let first_letter = s.chars().nth(0).unwrap();
    let second_letter = s.chars().nth(1).unwrap();
    match first_letter {
        'a' => bgm.idx = [0,0],
        'b' => bgm.idx = [0,1],
        'c' => bgm.idx = [0,2],
        'd' => bgm.idx = [1,0],
        'e' => bgm.idx = [1,1],
        'f' => bgm.idx = [1,2],
        'g' => bgm.idx = [2,0],
        'h' => bgm.idx = [2,1],
        'i' => bgm.idx = [2,2],
        _ => {panic!("error: str2bgm: first letter not a-i")},
    }
    match second_letter {
        '1' => bgm.sub_idx = [0,0],
        '2' => bgm.sub_idx = [0,1],
        '3' => bgm.sub_idx = [0,2],
        '4' => bgm.sub_idx = [1,0],
        '5' => bgm.sub_idx = [1,1],
        '6' => bgm.sub_idx = [1,2],
        '7' => bgm.sub_idx = [2,0],
        '8' => bgm.sub_idx = [2,1],
        '9' => bgm.sub_idx = [2,2],
        _ => {panic!("error: str2bgm: second letter not 1-9")},
    }
    bgm
}

pub fn str2bgm_checked(s: &str, p: Player) -> Option<BigGameMove> {
    let mut bgm = BigGameMove{idx: [0,0], sub_idx: [0,0], player: p};
    let first_letter = s.chars().nth(0).unwrap();
    let second_letter = s.chars().nth(1).unwrap();
    match first_letter {
        'a' => bgm.idx = [0,0],
        'b' => bgm.idx = [0,1],
        'c' => bgm.idx = [0,2],
        'd' => bgm.idx = [1,0],
        'e' => bgm.idx = [1,1],
        'f' => bgm.idx = [1,2],
        'g' => bgm.idx = [2,0],
        'h' => bgm.idx = [2,1],
        'i' => bgm.idx = [2,2],
        _ => {return None},
    }
    match second_letter {
        '1' => bgm.sub_idx = [0,0],
        '2' => bgm.sub_idx = [0,1],
        '3' => bgm.sub_idx = [0,2],
        '4' => bgm.sub_idx = [1,0],
        '5' => bgm.sub_idx = [1,1],
        '6' => bgm.sub_idx = [1,2],
        '7' => bgm.sub_idx = [2,0],
        '8' => bgm.sub_idx = [2,1],
        '9' => bgm.sub_idx = [2,2],
        _ => {return None},
    }
    Some(bgm)
}
