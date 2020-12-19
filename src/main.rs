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
struct Piece {
    tet_type: Tetrominoes,
    rotation: u8,
    x: usize,
    y: usize,
}

fn check_points(board: [[TileType; BOARD_HEIGHT]; BOARD_WIDTH], ptc: Vec<(usize, usize)>) -> bool{
    println!("checking points");
    for pt in ptc.iter() {
        println!("checking point: {:#?}", pt);
        if board[pt.0][pt.1] == TileType::Base || board[pt.0][pt.1] == TileType::Border {
            return false;
        }
    }
    true
}

fn plot_tet(board: &mut [[TileType; BOARD_HEIGHT]; BOARD_WIDTH], piece: Piece, clear: bool) {
    // if clear is true, then set the TileType to blank for the tet at the
    // specified rotation.

    let new_type: TileType;
    if clear {
        new_type = TileType::Blank;
    } else {
        new_type = TileType::Tet;
    }
    match piece.tet_type {
        Tetrominoes::I => match piece.rotation {
            0 => {
                board[piece.x][piece.y + 1] = new_type;
                board[piece.x + 1][piece.y + 1] = new_type;
                board[piece.x + 2][piece.y + 1] = new_type;
                board[piece.x + 3][piece.y + 1] = new_type;
            }
            1 => {
                board[piece.x + 2][piece.y] = new_type;
                board[piece.x + 2][piece.y + 1] = new_type;
                board[piece.x + 2][piece.y + 2] = new_type;
                board[piece.x + 2][piece.y + 3] = new_type;
            }
            2 => {
                board[piece.x][piece.y + 2] = new_type;
                board[piece.x + 1][piece.y + 2] = new_type;
                board[piece.x + 2][piece.y + 2] = new_type;
                board[piece.x + 3][piece.y + 2] = new_type;
            }
            _ => {
                board[piece.x + 1][piece.y] = new_type;
                board[piece.x + 1][piece.y + 1] = new_type;
                board[piece.x + 1][piece.y + 2] = new_type;
                board[piece.x + 1][piece.y + 3] = new_type;
            }
        },
        Tetrominoes::O => {
            board[piece.x][piece.y] = new_type;
            board[piece.x + 1][piece.y] = new_type;
            board[piece.x][piece.y + 1] = new_type;
            board[piece.x + 1][piece.y + 1] = new_type;
        }
        Tetrominoes::T => match piece.rotation {
            0 => {
                board[piece.x + 1][piece.y] = new_type;
                board[piece.x][piece.y + 1] = new_type;
                board[piece.x + 1][piece.y + 1] = new_type;
                board[piece.x + 2][piece.y + 1] = new_type;
            }
            1 => {
                board[piece.x + 1][piece.y] = new_type;
                board[piece.x + 1][piece.y + 1] = new_type;
                board[piece.x + 1][piece.y + 2] = new_type;
                board[piece.x + 2][piece.y + 1] = new_type;
            }
            2 => {
                board[piece.x][piece.y + 1] = new_type;
                board[piece.x + 1][piece.y + 1] = new_type;
                board[piece.x + 2][piece.y + 1] = new_type;
                board[piece.x + 1][piece.y + 2] = new_type;
            }
            _ => {
                board[piece.x][piece.y + 1] = new_type;
                board[piece.x + 1][piece.y] = new_type;
                board[piece.x + 1][piece.y + 1] = new_type;
                board[piece.x + 1][piece.y + 2] = new_type;
            }
        },
        Tetrominoes::J => match piece.rotation {
            0 => {
                board[piece.x][piece.y] = new_type;
                board[piece.x][piece.y + 1] = new_type;
                board[piece.x + 1][piece.y + 1] = new_type;
                board[piece.x + 2][piece.y + 1] = new_type;
            }
            1 => {
                board[piece.x + 1][piece.y] = new_type;
                board[piece.x + 1][piece.y + 1] = new_type;
                board[piece.x + 1][piece.y + 2] = new_type;
                board[piece.x + 2][piece.y] = new_type;
            }
            2 => {
                board[piece.x][piece.y + 1] = new_type;
                board[piece.x + 1][piece.y + 1] = new_type;
                board[piece.x + 2][piece.y + 1] = new_type;
                board[piece.x + 2][piece.y + 2] = new_type;
            }
            _ => {
                board[piece.x][piece.y + 2] = new_type;
                board[piece.x + 1][piece.y] = new_type;
                board[piece.x + 1][piece.y + 1] = new_type;
                board[piece.x + 1][piece.y + 2] = new_type;
            }
        },
        Tetrominoes::L => match piece.rotation {
            0 => {
                board[piece.x + 2][piece.y] = new_type;
                board[piece.x][piece.y + 1] = new_type;
                board[piece.x + 1][piece.y + 1] = new_type;
                board[piece.x + 2][piece.y + 1] = new_type;
            }
            1 => {
                board[piece.x + 1][piece.y] = new_type;
                board[piece.x + 1][piece.y + 1] = new_type;
                board[piece.x + 1][piece.y + 2] = new_type;
                board[piece.x + 2][piece.y + 2] = new_type;
            }
            2 => {
                board[piece.x][piece.y + 1] = new_type;
                board[piece.x + 1][piece.y + 1] = new_type;
                board[piece.x + 2][piece.y + 1] = new_type;
                board[piece.x][piece.y + 2] = new_type;
            }
            _ => {
                board[piece.x][piece.y] = new_type;
                board[piece.x + 1][piece.y] = new_type;
                board[piece.x + 1][piece.y + 1] = new_type;
                board[piece.x + 1][piece.y + 2] = new_type;
            }
        },
        Tetrominoes::S => match piece.rotation {
            0 => {
                board[piece.x + 1][piece.y] = new_type;
                board[piece.x + 2][piece.y] = new_type;
                board[piece.x][piece.y + 1] = new_type;
                board[piece.x + 1][piece.y + 1] = new_type;
            }
            1 => {
                board[piece.x + 1][piece.y] = new_type;
                board[piece.x + 1][piece.y + 1] = new_type;
                board[piece.x + 2][piece.y + 1] = new_type;
                board[piece.x + 2][piece.y + 2] = new_type;
            }
            2 => {
                board[piece.x + 1][piece.y + 1] = new_type;
                board[piece.x + 2][piece.y + 1] = new_type;
                board[piece.x][piece.y + 2] = new_type;
                board[piece.x + 1][piece.y + 2] = new_type;
            }
            _ => {
                board[piece.x][piece.y] = new_type;
                board[piece.x][piece.y + 1] = new_type;
                board[piece.x + 1][piece.y + 1] = new_type;
                board[piece.x + 1][piece.y + 2] = new_type;
            }
        },
        Tetrominoes::Z => match piece.rotation {
            0 => {
                board[piece.x][piece.y] = new_type;
                board[piece.x + 1][piece.y] = new_type;
                board[piece.x + 1][piece.y + 1] = new_type;
                board[piece.x + 2][piece.y + 1] = new_type;
            }
            1 => {
                board[piece.x + 2][piece.y] = new_type;
                board[piece.x + 1][piece.y + 1] = new_type;
                board[piece.x + 2][piece.y + 1] = new_type;
                board[piece.x + 1][piece.y + 2] = new_type;
            }
            2 => {
                board[piece.x][piece.y + 1] = new_type;
                board[piece.x + 1][piece.y + 1] = new_type;
                board[piece.x + 1][piece.y + 2] = new_type;
                board[piece.x + 2][piece.y + 2] = new_type;
            }
            _ => {
                board[piece.x + 1][piece.y] = new_type;
                board[piece.x][piece.y + 1] = new_type;
                board[piece.x + 1][piece.y + 1] = new_type;
                board[piece.x][piece.y + 2] = new_type;
            }
        },
    }
}

fn move_tet_left(board: &mut [[TileType; BOARD_HEIGHT]; BOARD_WIDTH], piece: &mut Piece) {
    // Check if move left is possible.
    // If yes, then first clear current position.
    //
    let x = (*piece).x;
    let y = (*piece).y;
    let mut ptc: Vec<(usize, usize)> = Vec::with_capacity(4);
    match piece.tet_type {
        Tetrominoes::I => {
            match (*piece).rotation {
                0 => {
                    ptc.push((x - 1, y + 1));
                }
                1 => {
                    ptc.push((x + 1,y));
                    ptc.push((x + 1, y + 1));
                    ptc.push((x + 1, y + 2));
                    ptc.push((x + 1, y + 3));
                }
                2 => {
                    ptc.push((x - 1, y + 2));
                }
                _ => {
                    ptc.push((x ,y));
                    ptc.push((x, y + 1));
                    ptc.push((x, y + 2));
                    ptc.push((x, y + 3));
                }
            }
        }
        Tetrominoes::O => {
            ptc.push((x - 1, y));
            ptc.push((x - 1, y + 1));
        }
        Tetrominoes::T => match (*piece).rotation {
            0 => {
                ptc.push((x, y));
                ptc.push((x - 1, y + 1));
            }
            1 => {
                ptc.push((x, y));
                ptc.push((x, y + 1));
                ptc.push((x, y + 2));
            }
            2 => {
                ptc.push((x - 1, y + 1));
                ptc.push((x, y + 2));
            }
            _ => {
                ptc.push((x, y));
                ptc.push((x - 1, y + 1));
                ptc.push((x, y + 1));
            }
        },
        Tetrominoes::J => match (*piece).rotation {
            0 => {
                ptc.push((x - 1, y));
                ptc.push((x - 1, y + 1));
            }
            1 => {
                ptc.push((x, y));
                ptc.push((x, y + 1));
                ptc.push((x, y + 2));
            }
            2 => {
                ptc.push((x - 1, y + 1));
                ptc.push((x + 1, y + 2));
            }
            _ => {
                ptc.push((x, y));
                ptc.push((x, y + 1));
                ptc.push((x - 1, y + 2));
            }
        },
        Tetrominoes::L => match (*piece).rotation {
            0 => {
                ptc.push((x + 1, y));
                ptc.push((x - 1, y + 1));
            }
            1 => {
                ptc.push((x, y));
                ptc.push((x, y + 1));
                ptc.push((x, y + 2));
            }
            2 => {
                ptc.push((x - 1, y + 1));
                ptc.push((x - 1, y + 2));
            }
            _ => {
                ptc.push((x - 1, y));
                ptc.push((x, y + 1));
                ptc.push((x, y + 2));
            }
        },
        Tetrominoes::S => match (*piece).rotation {
            0 => {
                ptc.push((x, y));
                ptc.push((x - 1, y + 1));
            }
            1 => {
                ptc.push((x, y));
                ptc.push((x, y + 1));
                ptc.push((x + 1, y + 2));
            }
            2 => {
                ptc.push((x, y + 1));
                ptc.push((x - 1, y + 2));
            }
            _ => {
                ptc.push((x - 1, y));
                ptc.push((x - 1, y + 1));
                ptc.push((x, y + 2));
            }
        },
        Tetrominoes::Z => match (*piece).rotation {
            0 => {
                ptc.push((x - 1, y));
                ptc.push((x, y + 1));
            }
            1 => {
                ptc.push((x + 1, y));
                ptc.push((x, y + 1));
                ptc.push((x, y + 2));
            }
            2 => {
                ptc.push((x - 1, y + 1));
                ptc.push((x, y + 2));
            }
            _ => {
                ptc.push((x, y));
                ptc.push((x - 1, y + 1));
                ptc.push((x - 1, y + 2));
            }
        },
    }
    if check_points(*board, ptc) {
        plot_tet(board, *piece, true);
        (*piece).x -= 1;
    }
}

fn move_tet_right(board: &mut [[TileType; BOARD_HEIGHT]; BOARD_WIDTH], piece: &mut Piece) {
    // Check if move left is possible.
    // If yes, then first clear current position.
    //
    let x = (*piece).x;
    let y = (*piece).y;
    let mut ptc: Vec<(usize, usize)> = Vec::with_capacity(4);
    match piece.tet_type {
        Tetrominoes::I => {
            match piece.rotation {
                0 => {
                    ptc.push((x + 4, y + 1));
                }
                1 => {
                    ptc.push((x + 3, y));
                    ptc.push((x + 3, y + 1));
                    ptc.push((x + 3, y + 2));
                    ptc.push((x + 3, y + 3));
                }
                2 => {
                    ptc.push((x + 4, y + 2));
                }
                _ => {
                    // 3,   make this an enum
                    ptc.push((x + 2, y));
                    ptc.push((x + 2, y + 1));
                    ptc.push((x + 2, y + 2));
                    ptc.push((x + 2, y + 3));
                }
            }
        }
        Tetrominoes::O => {
            ptc.push((x + 2, y));
            ptc.push((x + 2, y + 1));
        }
        Tetrominoes::T => match (*piece).rotation {
            0 => {
                ptc.push((x + 2, y));
                ptc.push((x + 3, y + 1));
            }
            1 => {
                ptc.push((x + 2, y));
                ptc.push((x + 3, y + 1));
                ptc.push((x + 2, y + 2));
            }
            2 => {
                ptc.push((x + 3, y + 1));
                ptc.push((x + 2, y + 2));
            }
            _ => {
                ptc.push((x + 2, y));
                ptc.push((x + 2, y + 1));
                ptc.push((x + 2, y + 2));
            }
        },
        Tetrominoes::J => match (*piece).rotation {
            0 => {
                ptc.push((x + 1, y));
                ptc.push((x + 3, y + 1));
            }
            1 => {
                ptc.push((x + 3, y));
                ptc.push((x + 2, y + 1));
                ptc.push((x + 2, y + 2));
            }
            2 => {
                ptc.push((x + 3, y + 1));
                ptc.push((x + 3, y + 2));
            }
            _ => {
                ptc.push((x + 2, y));
                ptc.push((x + 2, y + 1));
                ptc.push((x + 2, y + 2));
            }
        },
        Tetrominoes::L => match (*piece).rotation {
            0 => {
                ptc.push((x + 3, y));
                ptc.push((x + 3, y + 1));
            }
            1 => {
                ptc.push((x + 2, y));
                ptc.push((x + 2, y + 1));
                ptc.push((x + 3, y + 2));
            }
            2 => {
                ptc.push((x + 3, y + 1));
                ptc.push((x + 1, y + 2));
            }
            _ => {
                ptc.push((x + 2, y));
                ptc.push((x + 2, y + 1));
                ptc.push((x + 2, y + 2));
            }
        },
        Tetrominoes::S => match (*piece).rotation {
            0 => {
                ptc.push((x + 3, y));
                ptc.push((x + 2, y + 1));
            }
            1 => {
                ptc.push((x + 2, y));
                ptc.push((x + 3, y + 1));
                ptc.push((x + 3, y + 2));
            }
            2 => {
                ptc.push((x + 3, y + 1));
                ptc.push((x + 2, y + 2));
            }
            _ => {
                ptc.push((x + 1, y));
                ptc.push((x + 2, y + 1));
                ptc.push((x + 2, y + 2));
            }
        },
        Tetrominoes::Z => match (*piece).rotation {
            0 => {
                ptc.push((x + 2, y));
                ptc.push((x + 3, y + 1));
            }
            1 => {
                ptc.push((x + 3, y));
                ptc.push((x + 3, y + 1));
                ptc.push((x + 2, y + 2));
            }
            2 => {
                ptc.push((x + 2, y + 1));
                ptc.push((x + 3, y + 2));
            }
            _ => {
                ptc.push((x + 2, y));
                ptc.push((x + 2, y + 1));
                ptc.push((x + 1, y + 2));
            }
        },
    }
    if check_points(*board, ptc) {
        plot_tet(board, *piece, true);
        (*piece).x += 1;
    }
}

fn move_tet_down(board: &mut [[TileType; BOARD_HEIGHT]; BOARD_WIDTH], piece: &mut Piece) {
    let x = (*piece).x;
    let y = (*piece).y;
    let mut ptc: Vec<(usize, usize)> = Vec::with_capacity(4);
    match piece.tet_type {
        Tetrominoes::I => {
            match piece.rotation {
                0 => {
                    ptc.push((x, y + 2));
                    ptc.push((x + 1, y + 2));
                    ptc.push((x + 2, y + 2));
                    ptc.push((x + 3, y + 2));
                }
                1 => {
                    ptc.push((x + 2, y + 4));
                }
                2 => {
                    ptc.push((x, y + 3));
                    ptc.push((x + 1, y + 3));
                    ptc.push((x + 2, y + 3));
                    ptc.push((x + 3, y + 3));
                }
                _ => {
                    // 3,   make this an enum
                    ptc.push((x + 1, y + 4));
                }
            }
        }
        Tetrominoes::O => {
            ptc.push((x, y + 2));
            ptc.push((x + 1, y + 2));
        }
        Tetrominoes::T => match piece.rotation {
            0 => {
                ptc.push((x, y + 2));
                ptc.push((x + 1, y + 2));
                ptc.push((x + 2, y + 2));
            }
            1 => {
                ptc.push((x + 1, y + 3));
                ptc.push((x + 2, y + 2));
            }
            2 => {
                ptc.push((x, y + 2));
                ptc.push((x + 1, y + 3));
                ptc.push((x + 2, y + 2));
            }
            _ => {
                ptc.push((x, y + 2));
                ptc.push((x + 1, y + 3));
            }
        },
        Tetrominoes::J => match piece.rotation {
            0 => {
                ptc.push((x, y + 2));
                ptc.push((x + 1, y + 2));
                ptc.push((x + 2, y + 2));
            }
            1 => {
                ptc.push((x + 1, y + 3));
                ptc.push((x + 2, y + 1));
            }
            2 => {
                ptc.push((x, y + 2));
                ptc.push((x + 1, y + 2));
                ptc.push((x + 2, y + 3));
            }
            _ => {
                ptc.push((x, y + 3));
                ptc.push((x + 1, y + 3));
            }
        },
        Tetrominoes::L => match piece.rotation {
            0 => {
                ptc.push((x, y + 2));
                ptc.push((x + 1, y + 2));
                ptc.push((x + 2, y + 2));
            }
            1 => {
                ptc.push((x + 1, y + 3));
                ptc.push((x + 2, y + 3));
            }
            2 => {
                ptc.push((x, y + 3));
                ptc.push((x + 1, y + 2));
                ptc.push((x + 2, y + 2));
            }
            _ => {
                ptc.push((x, y + 1));
                ptc.push((x + 1, y + 3));
            }
        },
        Tetrominoes::S => match piece.rotation {
            0 => {
                ptc.push((x, y + 2));
                ptc.push((x + 1, y + 2));
                ptc.push((x + 2, y + 1));
            }
            1 => {
                ptc.push((x + 1, y + 2));
                ptc.push((x + 2, y + 3));
            }
            2 => {
                ptc.push((x, y + 3));
                ptc.push((x + 1, y + 3));
                ptc.push((x + 2, y + 2));
            }
            _ => {
                ptc.push((x, y + 2));
                ptc.push((x + 1, y + 3));
            }
        },
        Tetrominoes::Z => match piece.rotation {
            0 => {
                ptc.push((x, y + 1));
                ptc.push((x + 1, y + 2));
                ptc.push((x + 2, y + 2));
            }
            1 => {
                ptc.push((x + 1, y + 3));
                ptc.push((x + 2, y + 2));
            }
            2 => {
                ptc.push((x, y + 2));
                ptc.push((x + 1, y + 3));
                ptc.push((x + 2, y + 3));
            }
            _ => {
                ptc.push((x, y + 3));
                ptc.push((x + 1, y + 2));
            }
        },
    }
    if check_points(*board, ptc) {
        plot_tet(board, *piece, true);
        (*piece).y += 1;
    }
}

struct MainState {
    board: [[TileType; BOARD_HEIGHT]; BOARD_WIDTH],
    piece: Piece,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let piece = Piece {
            tet_type: Tetrominoes::I,
            rotation: 0,
            x: 6,
            y: 4,
        };
        let s = MainState {
            board: [[TileType::Blank; BOARD_HEIGHT]; BOARD_WIDTH],
            piece: piece,
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
        if !repeat {
            println!(
                "Key pressed: {:?}, modifier {:?}, repeat: {}",
                keycode, keymod, repeat
            );
            match keycode {
                input::keyboard::KeyCode::Q => quit(ctx),
                input::keyboard::KeyCode::W => {
                    // Check for rotation being legit
                    // Erase current location
                    plot_tet(&mut self.board, self.piece, true);
                    self.piece.rotation += 1;
                    if self.piece.rotation > 3 {
                        self.piece.rotation = 0;
                    }
                    // plot of new location will happen when we draw the board
                }
                input::keyboard::KeyCode::A => {
                    move_tet_left(&mut self.board, &mut self.piece);
                }
                input::keyboard::KeyCode::D => {
                    move_tet_right(&mut self.board, &mut self.piece);
                }
                input::keyboard::KeyCode::S => {
                    move_tet_down(&mut self.board, &mut self.piece);
                }
                // Begin debug commands
                input::keyboard::KeyCode::Z => {
                    println!("{:#?}", self.board);
                }
                input::keyboard::KeyCode::X => {
                    plot_tet(&mut self.board, self.piece, true);
                    self.piece.y -= 1;
                }
                input::keyboard::KeyCode::I => {
                    plot_tet(&mut self.board, self.piece, true);
                    self.piece.tet_type = Tetrominoes::I;
                }
                input::keyboard::KeyCode::O => {
                    plot_tet(&mut self.board, self.piece, true);
                    self.piece.tet_type = Tetrominoes::O;
                }
                input::keyboard::KeyCode::P => {
                    plot_tet(&mut self.board, self.piece, true);
                    self.piece.tet_type = Tetrominoes::T;
                }
                input::keyboard::KeyCode::J => {
                    plot_tet(&mut self.board, self.piece, true);
                    self.piece.tet_type = Tetrominoes::J;
                }
                input::keyboard::KeyCode::K => {
                    plot_tet(&mut self.board, self.piece, true);
                    self.piece.tet_type = Tetrominoes::L;
                }
                input::keyboard::KeyCode::L => {
                    plot_tet(&mut self.board, self.piece, true);
                    self.piece.tet_type = Tetrominoes::S;
                }
                input::keyboard::KeyCode::U => {
                    plot_tet(&mut self.board, self.piece, true);
                    self.piece.tet_type = Tetrominoes::Z;
                }
                _ => (),
            }
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        // Text
        let text = graphics::Text::new(format!("Rotation:{}", self.piece.rotation));
        graphics::draw(ctx, &text, (Point2::new(20.0, 20.0), graphics::WHITE))?;
        let text = graphics::Text::new(format!("x:{}", self.piece.x));
        graphics::draw(ctx, &text, (Point2::new(20.0, 40.0), graphics::WHITE))?;
        let text = graphics::Text::new(format!("y:{}", self.piece.y));
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

        plot_tet(&mut self.board, self.piece, false);

        for (py, y) in (0..440).step_by(20).enumerate() {
            for (px, x) in (200..480).step_by(20).enumerate() {
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
            }
        }

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
