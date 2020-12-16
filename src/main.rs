// use cgmath;
use ggez;
use ggez::event::{quit, run, EventHandler, KeyCode, KeyMods};
use ggez::graphics;
use ggez::graphics::{Color, DrawParam};
use ggez::nalgebra::Point2;
// use ggez::timer;
use ggez::input;
use ggez::{Context, GameResult};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tetrominoes {
    I,
    O,
    T,
    J,
    L,
    S,
    Z,
}

const BOARD_HEIGHT: usize = 22;
const BOARD_WIDTH: usize = 14;

#[derive(Debug, PartialEq, Clone, Copy)]
enum TileType {
    Border,
    Tet,
    Base,
    Blank,
}

#[derive(Debug, Clone, Copy)]
struct Tet {
    tet_type: Tetrominoes,
    rotation: u8,
    x: usize,
    y: usize,
}

fn plot_tet(
    board: &mut [[TileType; BOARD_HEIGHT]; BOARD_WIDTH],
    tet_type: Tetrominoes,
    rotation: u8,
    tet_x: usize,
    tet_y: usize,
    clear: bool,
) {
    // if clear is true, then set the TileType to blank for the tet at the
    // specified rotation.

    let new_type: TileType;
    if clear {
        new_type = TileType::Blank;
    } else {
        new_type = TileType::Tet;
    }
    match tet_type {
        Tetrominoes::I => {
            match rotation {
                0 => {
                    board[tet_x][tet_y + 1] = new_type;
                    board[tet_x + 1][tet_y + 1] = new_type;
                    board[tet_x + 2][tet_y + 1] = new_type;
                    board[tet_x + 3][tet_y + 1] = new_type;
                }
                1 => {
                    board[tet_x + 2][tet_y] = new_type;
                    board[tet_x + 2][tet_y + 1] = new_type;
                    board[tet_x + 2][tet_y + 2] = new_type;
                    board[tet_x + 2][tet_y + 3] = new_type;
                }
                2 => {
                    board[tet_x][tet_y + 2] = new_type;
                    board[tet_x + 1][tet_y + 2] = new_type;
                    board[tet_x + 2][tet_y + 2] = new_type;
                    board[tet_x + 3][tet_y + 2] = new_type;
                }
                _ => {
                    // 3,   make this an enum
                    board[tet_x + 1][tet_y] = new_type;
                    board[tet_x + 1][tet_y + 1] = new_type;
                    board[tet_x + 1][tet_y + 2] = new_type;
                    board[tet_x + 1][tet_y + 3] = new_type;
                }
            }
        }
        _ => (),
    }
}

fn move_tet_left(
    board: &mut [[TileType; BOARD_HEIGHT]; BOARD_WIDTH],
    tet_type: Tetrominoes,
    rotation: u8,
    tet_x: &mut usize,
    tet_y: usize,
) {
    // Check if move left is possible.
    // If yes, then first clear current position.
    //
    match tet_type {
        Tetrominoes::I => {
            match rotation {
                0 => {
                    if board[*tet_x - 1][tet_y + 1] == TileType::Blank {
                        plot_tet(board, tet_type, rotation, *tet_x, tet_y, true);
                        *tet_x -= 1;
                    }
                }
                1 => {
                    if board[*tet_x + 1][tet_y] == TileType::Blank
                        && board[*tet_x + 1][tet_y + 1] == TileType::Blank
                        && board[*tet_x + 1][tet_y + 2] == TileType::Blank
                        && board[*tet_x + 1][tet_y + 3] == TileType::Blank
                    {
                        plot_tet(board, tet_type, rotation, *tet_x, tet_y, true);
                        *tet_x -= 1;
                    }
                }
                2 => {
                    if board[*tet_x - 1][tet_y + 2] == TileType::Blank {
                        plot_tet(board, tet_type, rotation, *tet_x, tet_y, true);
                        *tet_x -= 1;
                    }
                }
                _ => {
                    // 3,   make this an enum
                    if board[*tet_x][tet_y] == TileType::Blank
                        && board[*tet_x][tet_y + 1] == TileType::Blank
                        && board[*tet_x][tet_y + 2] == TileType::Blank
                        && board[*tet_x][tet_y + 3] == TileType::Blank
                    {
                        plot_tet(board, tet_type, rotation, *tet_x, tet_y, true);
                        *tet_x -= 1;
                    }
                }
            }
        }
        _ => (),
    }
}
fn move_tet_right(
    board: &mut [[TileType; BOARD_HEIGHT]; BOARD_WIDTH],
    tet_type: Tetrominoes,
    rotation: u8,
    tet_x: &mut usize,
    tet_y: usize,
) {
    // Check if move left is possible.
    // If yes, then first clear current position.
    //
    match tet_type {
        Tetrominoes::I => {
            match rotation {
                0 => {
                    if board[*tet_x + 4][tet_y + 1] == TileType::Blank {
                        plot_tet(board, tet_type, rotation, *tet_x, tet_y, true);
                        *tet_x += 1;
                    }
                }
                1 => {
                    if board[*tet_x + 3][tet_y] == TileType::Blank
                        && board[*tet_x + 3][tet_y + 1] == TileType::Blank
                        && board[*tet_x + 3][tet_y + 2] == TileType::Blank
                        && board[*tet_x + 3][tet_y + 3] == TileType::Blank
                    {
                        plot_tet(board, tet_type, rotation, *tet_x, tet_y, true);
                        *tet_x += 1;
                    }
                }
                2 => {
                    if board[*tet_x + 4][tet_y + 2] == TileType::Blank {
                        plot_tet(board, tet_type, rotation, *tet_x, tet_y, true);
                        *tet_x += 1;
                    }
                }
                _ => {
                    // 3,   make this an enum
                    if board[*tet_x + 2][tet_y] == TileType::Blank
                        && board[*tet_x + 2][tet_y + 1] == TileType::Blank
                        && board[*tet_x + 2][tet_y + 2] == TileType::Blank
                        && board[*tet_x + 2][tet_y + 3] == TileType::Blank
                    {
                        plot_tet(board, tet_type, rotation, *tet_x, tet_y, true);
                        *tet_x += 1;
                    }
                }
            }
        }
        _ => (),
    }
}

fn move_tet_down(
    board: &mut [[TileType; BOARD_HEIGHT]; BOARD_WIDTH],
    tet_type: Tetrominoes,
    rotation: u8,
    tet_x: usize,
    tet_y: &mut usize,
) {
    match tet_type {
        Tetrominoes::I => {
            match rotation {
                0 => {
                    if board[tet_x][*tet_y + 2] == TileType::Blank
                        && board[tet_x + 1][*tet_y + 2] == TileType::Blank
                        && board[tet_x + 2][*tet_y + 2] == TileType::Blank
                        && board[tet_x + 3][*tet_y + 2] == TileType::Blank
                    {
                        plot_tet(board, tet_type, rotation, tet_x, *tet_y, true);
                        *tet_y += 1;
                    }
                }
                1 => {
                    if board[tet_x + 2][*tet_y + 4] == TileType::Blank {
                        plot_tet(board, tet_type, rotation, tet_x, *tet_y, true);
                        *tet_y += 1;
                    }
                }
                2 => {
                    if board[tet_x][*tet_y + 3] == TileType::Blank
                        && board[tet_x + 1][*tet_y + 3] == TileType::Blank
                        && board[tet_x + 2][*tet_y + 3] == TileType::Blank
                        && board[tet_x + 3][*tet_y + 3] == TileType::Blank
                    {
                        plot_tet(board, tet_type, rotation, tet_x, *tet_y, true);
                        *tet_y += 1;
                    }
                }
                _ => {
                    // 3,   make this an enum
                    if board[tet_x + 1][*tet_y + 4] == TileType::Blank {
                        plot_tet(board, tet_type, rotation, tet_x, *tet_y, true);
                        *tet_y += 1;
                    }
                }
            }
        }
        _ => (),
    }
}

struct MainState {
    rotation: u8,
    tet_type: Tetrominoes,
    tet_x: usize,
    tet_y: usize,
    board: [[TileType; BOARD_HEIGHT]; BOARD_WIDTH],
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState {
            rotation: 0,
            tet_type: Tetrominoes::I,
            tet_x: 6,
            tet_y: 4,
            board: [[TileType::Blank; BOARD_HEIGHT]; BOARD_WIDTH],
        };
        Ok(s)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 1;

        if input::keyboard::is_key_pressed(ctx, KeyCode::Q) {
            println!("quit");
        }
        /*
                while timer::check_update_time(ctx, DESIRED_FPS) {
                self.tet_y = self.tet_y + 20.0;
                if self.tet_y > 380.0 {
                    self.tet_y = 0.0;
                }
                }
        */
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        keymod: KeyMods,
        repeat: bool,
    ) {
        if repeat == false {
            println!(
                "Key pressed: {:?}, modifier {:?}, repeat: {}",
                keycode, keymod, repeat
            );
            match keycode {
                input::keyboard::KeyCode::Q => quit(ctx),
                input::keyboard::KeyCode::W => {
                    // Check for rotation being legit
                    // Erase current location
                    plot_tet(
                        &mut self.board,
                        self.tet_type,
                        self.rotation,
                        self.tet_x,
                        self.tet_y,
                        true,
                    );
                    self.rotation += 1;
                    if self.rotation > 3 {
                        self.rotation = 0;
                    }
                    // plot of new location will happen when we draw the board
                }
                input::keyboard::KeyCode::A => {
                    move_tet_left(
                        &mut self.board,
                        self.tet_type,
                        self.rotation,
                        &mut self.tet_x,
                        self.tet_y,
                    );
                }
                input::keyboard::KeyCode::D => {
                    move_tet_right(
                        &mut self.board,
                        self.tet_type,
                        self.rotation,
                        &mut self.tet_x,
                        self.tet_y,
                    );
                }
                input::keyboard::KeyCode::S => {
                    move_tet_down(
                        &mut self.board,
                        self.tet_type,
                        self.rotation,
                        self.tet_x,
                        &mut self.tet_y,
                    );
                }
                input::keyboard::KeyCode::Z => {
                    println!("{:#?}", self.board);
                }
                _ => (),
            }
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        // Text
        let text = graphics::Text::new(format!("Rotation:{}", self.rotation));
        graphics::draw(ctx, &text, (Point2::new(20.0, 20.0), graphics::WHITE))?;
        let text = graphics::Text::new(format!("tet_x:{}", self.tet_x));
        graphics::draw(ctx, &text, (Point2::new(20.0, 40.0), graphics::WHITE))?;
        let text = graphics::Text::new(format!("tet_y:{}", self.tet_y));
        graphics::draw(ctx, &text, (Point2::new(20.0, 60.0), graphics::WHITE))?;

        // The board grid
        let mb = &mut graphics::MeshBuilder::new();
        for y in (0..=440).step_by(20) {
            let y = y as f32;
            mb.line(
                &[Point2::new(200.0, y), Point2::new(480.0, y)],
                2.0,
                Color::new(0.9, 0.9, 0.9, 4.0),
            )?;
        }
        for x in (200..=480).step_by(20) {
            let x = x as f32;
            mb.line(
                &[Point2::new(x, 0.0), Point2::new(x, 440.0)],
                2.0,
                Color::new(0.9, 0.9, 0.9, 4.0),
            )?;
        }

        let fill = Color::new(1.0, 0.0, 0.0, 1.0);
        for y in 0..21 {
            self.board[1][y] = TileType::Border;
            self.board[12][y] = TileType::Border;
        }
        for x in 1..12 {
            self.board[x][20] = TileType::Border;
        }

        plot_tet(
            &mut self.board,
            self.tet_type,
            self.rotation,
            self.tet_x,
            self.tet_y,
            false,
        );

        let mut py = 0;
        for y in (0..440).step_by(20) {
            let mut px = 0;
            for x in (200..480).step_by(20) {
                let x = x as f32;
                let y = y as f32;

                match self.board[px][py] {
                    TileType::Border => {
                        mb.line(
                            &[
                                Point2::new(x + 10.0, y + 2.0),
                                Point2::new(x + 10.0, y + 18.0),
                            ],
                            16.0,
                            fill,
                        )?;
                    }
                    TileType::Tet => {
                        mb.line(
                            &[
                                Point2::new(x + 10.0, y + 2.0),
                                Point2::new(x + 10.0, y + 18.0),
                            ],
                            16.0,
                            Color::new(0.0, 1.0, 1.0, 1.0),
                        )?;
                    }
                    _ => (),
                }
                px += 1;
            }
            py += 1;
        }

        // Figure out where our piece is
        /*
        match self.tet_type {
            Tetrominoes::I => {
                if self.rotation == 0 { // ----
                }
                else if self.rotation == 1 { //  |
                    self.board[self.p_x + 2][self.p_y] = true;
                    self.board[self.p_x + 2][self.p_y + 1] = true;
                    self.board[self.p_x + 2][self.p_y + 2] = true;
                    self.board[self.p_x + 2][self.p_y + 3] = true;
                }
                else if self.rotation == 2 { // ____
                    self.board[self.p_x][self.p_y + 2] = true;
                    self.board[self.p_x + 1][self.p_y + 2] = true;
                    self.board[self.p_x + 2][self.p_y + 2] = true;
                    self.board[self.p_x + 3][self.p_y + 2] = true;
                }
                else  /* self.rotation == 3 */ { // |
                    self.board[self.p_x + 1][self.p_y] = true;
                    self.board[self.p_x + 1][self.p_y + 1] = true;
                    self.board[self.p_x + 1][self.p_y + 2] = true;
                    self.board[self.p_x + 1][self.p_y + 3] = true;
                }
            },
            _ => (),
        }
        */
        let m = mb.build(ctx)?;
        graphics::draw(ctx, &m, DrawParam::new())?;

        // Finished drawing, show it all on the screen!
        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("drawing", "ggez");

    let (ctx, events_loop) = &mut cb.build()?;
    graphics::set_window_title(ctx, "Work In Progress");

    println!("{}", graphics::renderer_info(ctx)?);
    let state = &mut MainState::new(ctx).unwrap();
    run(ctx, events_loop, state)
}
