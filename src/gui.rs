use crate::*;
use sfml::graphics::{Color, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable};
use sfml::system::{Vector2f, Vector2i};
use sfml::window::{ContextSettings, Event, Key, VideoMode};

fn new_rect(x: f32, y: f32, w: f32, h: f32) -> RectangleShape<'static> {
    let mut grid_line = RectangleShape::new();
    grid_line.set_size(Vector2f::new(w, h));
    grid_line.set_fill_color(Color::rgb(0, 0, 0));
    grid_line.set_position(Vector2f::new(x, y));
    return grid_line;
}

fn xy_to_subsquare(x: i32, y: i32) -> (i8, i8) {
    return (
        ((x as f32) / 100.).floor() as i8 % 3,
        ((y as f32) / 100.).floor() as i8 % 3,
    );
}

fn xy_to_board(x: i32, y: i32) -> (i8, i8) {
    return (
        ((x as f32) / 300.).floor() as i8 % 3,
        ((y as f32) / 300.).floor() as i8 % 3,
    );
}

pub fn play_gui() {
    const HT: f32 = 900.;
    const WD: f32 = 900.;
    const MONITOR_SIZE: (i32, i32) = (2560, 1440);

    let mut window = RenderWindow::new(
        VideoMode::new(WD as u32, HT as u32, 32),
        "tic-tac-toe",
        window::Style::CLOSE,
        &ContextSettings::default(),
    );
    window.set_position(Vector2i::new(
        (MONITOR_SIZE.0 - WD as i32) / 2,
        (MONITOR_SIZE.1 - HT as i32) / 2,
    ));
    let mut big_grid: Vec<RectangleShape<'static>> = vec![];
    let mut sub_grids: Vec<RectangleShape<'static>> = vec![];
    let mut squares: Vec<RectangleShape<'static>> = vec![];
    for i in 1..=2 {
        big_grid.push(new_rect(0., (i as f32) * 300. - 8., 1200., 16.));
    }
    for i in 1..=2 {
        big_grid.push(new_rect((i as f32) * 300. - 8., 0., 16., 900.));
    }
    for i in 0..=2 {
        for j in 0..=2 {
            for k in 1..=2 {
                sub_grids.push(new_rect(
                    (i as f32) * 300.,
                    (j as f32) * 300. + (k as f32) * 100.,
                    300.,
                    1.,
                ));
                sub_grids.push(new_rect(
                    (i as f32) * 300. + (k as f32) * 100.,
                    (j as f32) * 300.,
                    1.,
                    300.,
                ));
            }
        }
    }
    for i in 0..9 {
        for j in 0..9 {
            squares.push(new_rect((i as f32) * 100., (j as f32) * 100., 100., 100.));
        }
    }
    for i in 0..squares.len() {
        squares[i].set_fill_color(Color::rgb(255, 255, 255));
    }

    let search_depth = 8;

    let mut root = Game::new();
    let mut pv: Vec<BigGameMove> = vec![];
    let mut human_choice: Option<BigGameMove>;
    let _score = -alphabeta(
        &mut root,
        search_depth,
        -std::i32::MAX,
        std::i32::MAX,
        &mut pv,
    );
    root.do_move(pv[0]);
    let eng_choice = pv[0];
    let (brd_r, brd_c) = (eng_choice.idx[0], eng_choice.idx[1]);
    let (sub_r, sub_c) = (eng_choice.sub_idx[0], eng_choice.sub_idx[1]);
    squares[((brd_r * 3 + sub_r) * 9 + brd_c * 3 + sub_c) as usize]
        .set_fill_color(Color::rgb(0, 0, 255));
    pv.clear();

    loop {
        let event = window.poll_event();
        if let Some(event) = event {
            match event {
                Event::Closed => break,
                Event::KeyPressed { code, .. } => match code {
                    Key::Escape => break,
                    Key::Left => {}
                    _ => {}
                },
                Event::MouseButtonPressed { x, y, .. } => {
                    let (sub_r, sub_c) = xy_to_subsquare(x, y);
                    let (brd_r, brd_c) = xy_to_board(x, y);
                    //println!("click x,y: {},{}", x,y);
                    //println!("sub_r,sub_c: {},{}", sub_r,sub_c);
                    //println!("brd_r,brd_c: {},{}", brd_r,brd_c);
                    let mapped_bgm = bgm_o(
                        [brd_r.try_into().unwrap(), brd_c.try_into().unwrap()],
                        [sub_r.try_into().unwrap(), sub_c.try_into().unwrap()],
                    );
                    if root.get_moves().contains(&mapped_bgm) {
                        human_choice = Some(mapped_bgm);
                        println!("human_choice: {}", bgm2str(&human_choice.unwrap()));
                        root.do_move(mapped_bgm);
                        squares[((brd_r * 3 + sub_r) * 9 + brd_c * 3 + sub_c) as usize]
                            .set_fill_color(Color::rgb(255, 0, 0));
                        pv.clear();
                        let score = alphabeta(
                            &mut root,
                            search_depth,
                            -std::i32::MAX,
                            std::i32::MAX,
                            &mut pv,
                        );
                        if root.winner().is_some() {
                            println!("game over! winner: {:?}", root.winner().unwrap());
                            break;
                        }
                        root.do_move(pv[0]);
                        let eng_choice = pv[0];
                        println!("engine choice: {}", bgm2str(&eng_choice));
                        let (brd_r, brd_c) = (eng_choice.idx[0], eng_choice.idx[1]);
                        let (sub_r, sub_c) = (eng_choice.sub_idx[0], eng_choice.sub_idx[1]);
                        squares[((brd_r * 3 + sub_r) * 9 + brd_c * 3 + sub_c) as usize]
                            .set_fill_color(Color::rgb(0, 0, 255));
                        pv.clear();
                        println!("engine eval: {}", score);
                    }
                }
                _ => {}
            }
        }
        window.clear(Color::rgb(0, 0, 0));
        let legal_moves = root.get_moves();
        for i in 0..squares.len() {
            if squares[i].fill_color() == Color::rgb(100, 100, 100) {
                squares[i].set_fill_color(Color::rgb(255, 255, 255));
            }
        }
        for m in legal_moves {
            let (brd_r, brd_c) = (m.idx[0], m.idx[1]);
            let (sub_r, sub_c) = (m.sub_idx[0], m.sub_idx[1]);
            squares[((brd_r * 3 + sub_r) * 9 + brd_c * 3 + sub_c) as usize]
                .set_fill_color(Color::rgb(100, 100, 100));
        }

        for sq in squares.iter() {
            window.draw(sq);
        }
        for line in big_grid.iter() {
            window.draw(line);
        }
        for line in sub_grids.iter() {
            window.draw(line);
        }

        window.display();
    }
}
