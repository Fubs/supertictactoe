use crate::board::*;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct GameMove {
    pub idx: [u8; 2],
    pub player: Player,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct BigGameMove {
    pub idx: [u8; 2],
    pub sub_idx: [u8; 2],
    pub player: Player,
}

impl GameMove {
    pub fn new(idx: [u8; 2], player: Player) -> GameMove {
        GameMove { idx, player }
    }
}

impl BigGameMove {
    pub fn new(idx: [u8; 2], sub_idx: [u8; 2], player: Player) -> BigGameMove {
        BigGameMove {
            idx,
            sub_idx,
            player,
        }
    }
}

pub fn bgm_x(idx: [u8; 2], sub_idx: [u8; 2]) -> BigGameMove {
    BigGameMove::new(idx, sub_idx, Player::X)
}

pub fn bgm_o(idx: [u8; 2], sub_idx: [u8; 2]) -> BigGameMove {
    BigGameMove::new(idx, sub_idx, Player::O)
}

pub fn gm_x(idx: [u8; 2]) -> GameMove {
    GameMove::new(idx, Player::X)
}

pub fn gm_o(idx: [u8; 2]) -> GameMove {
    GameMove::new(idx, Player::O)
}

pub fn bgm2str(bgm: &BigGameMove) -> String {
    let mut s = String::new();
    match bgm.idx {
        [0, 0] => {
            s.push('a');
        }
        [1, 0] => {
            s.push('b');
        }
        [2, 0] => {
            s.push('c');
        }
        [0, 1] => {
            s.push('d');
        }
        [1, 1] => {
            s.push('e');
        }
        [2, 1] => {
            s.push('f');
        }
        [0, 2] => {
            s.push('g');
        }
        [1, 2] => {
            s.push('h');
        }
        [2, 2] => {
            s.push('i');
        }
        _ => {
            panic!("error: bgm2str: idx not in [0,0] to [2,2]")
        }
    };
    match bgm.sub_idx {
        [0, 0] => {
            s.push('1');
        }
        [1, 0] => {
            s.push('2');
        }
        [2, 0] => {
            s.push('3');
        }
        [0, 1] => {
            s.push('4');
        }
        [1, 1] => {
            s.push('5');
        }
        [2, 1] => {
            s.push('6');
        }
        [0, 2] => {
            s.push('7');
        }
        [1, 2] => {
            s.push('8');
        }
        [2, 2] => {
            s.push('9');
        }
        _ => {
            panic!("error: bgm2str: sub_idx not in [0,0] to [2,2]")
        }
    };
    s
}

pub fn str2bgm(s: &str, p: Player) -> BigGameMove {
    let mut bgm = BigGameMove {
        idx: [0, 0],
        sub_idx: [0, 0],
        player: p,
    };
    let first_letter = s.chars().nth(0).unwrap();
    let second_letter = s.chars().nth(1).unwrap();
    match first_letter {
        'a' => bgm.idx = [0, 0],
        'b' => bgm.idx = [0, 1],
        'c' => bgm.idx = [0, 2],
        'd' => bgm.idx = [1, 0],
        'e' => bgm.idx = [1, 1],
        'f' => bgm.idx = [1, 2],
        'g' => bgm.idx = [2, 0],
        'h' => bgm.idx = [2, 1],
        'i' => bgm.idx = [2, 2],
        _ => {
            panic!("error: str2bgm: first letter not a-i")
        }
    }
    match second_letter {
        '1' => bgm.sub_idx = [0, 0],
        '2' => bgm.sub_idx = [0, 1],
        '3' => bgm.sub_idx = [0, 2],
        '4' => bgm.sub_idx = [1, 0],
        '5' => bgm.sub_idx = [1, 1],
        '6' => bgm.sub_idx = [1, 2],
        '7' => bgm.sub_idx = [2, 0],
        '8' => bgm.sub_idx = [2, 1],
        '9' => bgm.sub_idx = [2, 2],
        _ => {
            panic!("error: str2bgm: second letter not 1-9")
        }
    }
    bgm
}

pub fn str2bgm_checked(s: &str, p: Player) -> Option<BigGameMove> {
    let mut bgm = BigGameMove {
        idx: [0, 0],
        sub_idx: [0, 0],
        player: p,
    };
    let first_letter = s.chars().nth(0).unwrap();
    let second_letter = s.chars().nth(1).unwrap();
    match first_letter {
        'a' => bgm.idx = [0, 0],
        'b' => bgm.idx = [0, 1],
        'c' => bgm.idx = [0, 2],
        'd' => bgm.idx = [1, 0],
        'e' => bgm.idx = [1, 1],
        'f' => bgm.idx = [1, 2],
        'g' => bgm.idx = [2, 0],
        'h' => bgm.idx = [2, 1],
        'i' => bgm.idx = [2, 2],
        _ => return None,
    }
    match second_letter {
        '1' => bgm.sub_idx = [0, 0],
        '2' => bgm.sub_idx = [0, 1],
        '3' => bgm.sub_idx = [0, 2],
        '4' => bgm.sub_idx = [1, 0],
        '5' => bgm.sub_idx = [1, 1],
        '6' => bgm.sub_idx = [1, 2],
        '7' => bgm.sub_idx = [2, 0],
        '8' => bgm.sub_idx = [2, 1],
        '9' => bgm.sub_idx = [2, 2],
        _ => return None,
    }
    Some(bgm)
}
