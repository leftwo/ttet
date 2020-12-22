// use cgmath;
use ggez;
use ggez::event::{quit, run, EventHandler, KeyCode, KeyMods};
use ggez::graphics;
use ggez::graphics::{Color, DrawParam};
use ggez::nalgebra::Point2;
use ggez::timer;
use ggez::input;
use ggez::{Context, GameResult};
use rand::{ distributions::{Distribution, Standard}, Rng, };

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

// From stackoverflow:
// https://stackoverflow.com/questions/48490049
// Pick a random Tetrominoe from our enum
impl Distribution<Tetrominoes> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Tetrominoes {
        match rng.gen_range(0, 7) {
            0 => Tetrominoes::I,
            1 => Tetrominoes::O,
            2 => Tetrominoes::T,
            3 => Tetrominoes::J,
            4 => Tetrominoes::L,
            5 => Tetrominoes::S,
            _ => Tetrominoes::Z,
        }
    }
}

// The visible board size is 10x20. We have additional space in the
// array for reasons that involve how we plot the tetrominoes and
// translate their x,y location when plotting on the board.
const BOARD_HEIGHT: usize = 26;
const BOARD_WIDTH: usize = 14;

#[derive(Debug, PartialEq, Clone, Copy)]
enum TileType {
    Border,
    Tet,
    Base,
    Blank,
    Filled,
}

#[derive(Debug, Clone, Copy)]
struct Piece {
    tet_type: Tetrominoes,
    rotation: u8,
    x: usize,
    y: usize,
}

fn check_points(board: [[TileType; BOARD_HEIGHT]; BOARD_WIDTH], ptc: Vec<(usize, usize)>) -> bool {
    for pt in ptc.iter() {
        if board[pt.0][pt.1] == TileType::Base
            || board[pt.0][pt.1] == TileType::Border
            || board[pt.0][pt.1] == TileType::Filled {
            return false;
        }
    }
    true
}

fn rotate_check(board: [[TileType; BOARD_HEIGHT]; BOARD_WIDTH], piece: Piece) -> bool {
    // Check to see if, given the current board and our current
    // rotation, the squares for the next rotation are available to
    // "move" to.
    let x = piece.x;
    let y = piece.y;
    let mut ptc: Vec<(usize, usize)> = Vec::with_capacity(4);
    match piece.tet_type {
        Tetrominoes::I => match piece.rotation {
            0 => {
                ptc.push((x + 2, y));
                ptc.push((x + 2, y + 1));
                ptc.push((x + 2, y + 2));
                ptc.push((x + 2, y + 3));
            }
            1 => {
                ptc.push((x, y + 2));
                ptc.push((x + 1, y + 2));
                ptc.push((x + 2, y + 2));
                ptc.push((x + 3, y + 2));
            }
            2 => {
                ptc.push((x + 1, y));
                ptc.push((x + 1, y + 1));
                ptc.push((x + 1, y + 2));
                ptc.push((x + 1, y + 3));
            }
            _ => {
                ptc.push((x, y + 1));
                ptc.push((x + 1, y + 1));
                ptc.push((x + 2, y + 1));
                ptc.push((x + 3, y + 1));
            }
        },
        Tetrominoes::O => {
            return true;
        }
        Tetrominoes::T => match piece.rotation {
            0 => {
                ptc.push((x + 1, y));
                ptc.push((x + 1, y + 1));
                ptc.push((x + 1, y + 2));
                ptc.push((x + 2, y + 1));
            }
            1 => {
                ptc.push((x, y + 1));
                ptc.push((x + 1, y + 1));
                ptc.push((x + 2, y + 1));
                ptc.push((x + 1, y + 2));
            }
            2 => {
                ptc.push((x, y + 1));
                ptc.push((x + 1, y));
                ptc.push((x + 1, y + 1));
                ptc.push((x + 1, y + 2));
            }
            _ => {
                ptc.push((x + 1, y));
                ptc.push((x, y + 1));
                ptc.push((x + 1, y + 1));
                ptc.push((x + 2, y + 1));
            }
        },
        Tetrominoes::J => match piece.rotation {
            0 => {
                ptc.push((x, y));
                ptc.push((x, y + 1));
                ptc.push((x + 1, y + 1));
                ptc.push((x + 2, y + 1));
            }
            1 => {
                ptc.push((x + 1, y));
                ptc.push((x + 1, y + 1));
                ptc.push((x + 1, y + 2));
                ptc.push((x + 2, y));
            }
            2 => {
                ptc.push((x, y + 1));
                ptc.push((x + 1, y + 1));
                ptc.push((x + 2, y + 1));
                ptc.push((x + 2, y + 2));
            }
            _ => {
                ptc.push((x, y + 2));
                ptc.push((x + 1, y));
                ptc.push((x + 1, y + 1));
                ptc.push((x + 1, y + 2));
            }
        },
        Tetrominoes::L => match piece.rotation {
            0 => {
                ptc.push((x + 1, y));
                ptc.push((x + 1, y + 1));
                ptc.push((x + 1, y + 2));
                ptc.push((x + 2, y + 2));
            }
            1 => {
                ptc.push((x, y + 1));
                ptc.push((x + 1, y + 1));
                ptc.push((x + 2, y + 1));
                ptc.push((x, y + 2));
            }
            2 => {
                ptc.push((x, y));
                ptc.push((x + 1, y));
                ptc.push((x + 1, y + 1));
                ptc.push((x + 1, y + 2));
            }
            _ => {
                ptc.push((x + 2, y));
                ptc.push((x, y + 1));
                ptc.push((x + 1, y + 1));
                ptc.push((x + 2, y + 1));
            }
        },
        Tetrominoes::S => match piece.rotation {
            0 => {
                ptc.push((x + 1, y));
                ptc.push((x + 1, y + 1));
                ptc.push((x + 2, y + 1));
                ptc.push((x + 2, y + 2));
            }
            1 => {
                ptc.push((x + 1, y + 1));
                ptc.push((x + 2, y + 1));
                ptc.push((x, y + 2));
                ptc.push((x + 1, y + 2));
            }
            2 => {
                ptc.push((x, y));
                ptc.push((x, y + 1));
                ptc.push((x + 1, y + 1));
                ptc.push((x + 1, y + 2));
            }
            _ => {
                ptc.push((x + 1, y));
                ptc.push((x + 2, y));
                ptc.push((x, y + 1));
                ptc.push((x + 1, y + 1));
            }
        },
        Tetrominoes::Z => match piece.rotation {
            0 => {
                ptc.push((x + 2, y));
                ptc.push((x + 1, y + 1));
                ptc.push((x + 2, y + 1));
                ptc.push((x + 1, y + 2));
            }
            1 => {
                ptc.push((x, y + 1));
                ptc.push((x + 1, y + 1));
                ptc.push((x + 1, y + 2));
                ptc.push((x + 2, y + 2));
            }
            2 => {
                ptc.push((x + 1, y));
                ptc.push((x, y + 1));
                ptc.push((x + 1, y + 1));
                ptc.push((x, y + 2));
            }
            _ => {
                ptc.push((x, y));
                ptc.push((x + 1, y));
                ptc.push((x + 1, y + 1));
                ptc.push((x + 2, y + 1));
            }
        },
    }
    if check_points(board, ptc) {
        return true;
    }
    false
}

fn plot_tet(board: &mut [[TileType; BOARD_HEIGHT]; BOARD_WIDTH],
            piece: Piece,
            new_type: TileType) {
    // Set the location on the board to the given type.
    // We use the piece location and rotation to determine which
    // squares we need to update.
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

fn tet_to_base(board: &mut [[TileType; BOARD_HEIGHT]; BOARD_WIDTH], piece: &mut Piece) {
    plot_tet(board, *piece, TileType::Base);
    (*piece).tet_type = rand::random();
    if (*piece).tet_type == Tetrominoes::I {
        (*piece).rotation = 1;
        (*piece).x = 0;
    }
    else {
        (*piece).rotation = 0;
        (*piece).x = 2;
    }
    (*piece).y = 0;
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
        plot_tet(board, *piece, TileType::Blank);
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
        plot_tet(board, *piece, TileType::Blank);
        (*piece).x += 1;
    }
}

fn move_tet_down(board: &mut [[TileType; BOARD_HEIGHT]; BOARD_WIDTH], piece: &mut Piece) -> bool {
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
        plot_tet(board, *piece, TileType::Blank);
        (*piece).y += 1;
        return true;
    }
    false
}

struct MainState {
    board: [[TileType; BOARD_HEIGHT]; BOARD_WIDTH],
    piece: Piece,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let piece = Piece {
            tet_type: rand::random(),
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
        while timer::check_update_time(ctx, DESIRED_FPS) {
            if !move_tet_down(&mut self.board, &mut self.piece) {
                println!("Can't move down");
                tet_to_base(&mut self.board, &mut self.piece);
            }
        }
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

                    if rotate_check(self.board, self.piece) {
                        plot_tet(&mut self.board, self.piece, TileType::Blank);
                        self.piece.rotation += 1;
                        if self.piece.rotation > 3 {
                            self.piece.rotation = 0;
                        }
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
                    if !move_tet_down(&mut self.board, &mut self.piece) {
                        tet_to_base(&mut self.board, &mut self.piece);
                    }
                }
                // Begin debug commands
                input::keyboard::KeyCode::Z => {
                    println!("{:#?}", self.board);
                }
                input::keyboard::KeyCode::C => {
                    tet_to_base(&mut self.board, &mut self.piece);
                }
                input::keyboard::KeyCode::X => {
                    plot_tet(&mut self.board, self.piece, TileType::Blank);
                    self.piece.y -= 1;
                }
                input::keyboard::KeyCode::I => {
                    plot_tet(&mut self.board, self.piece, TileType::Blank);
                    self.piece.tet_type = match self.piece.tet_type {
                        Tetrominoes::I => Tetrominoes::O,
                        Tetrominoes::O => Tetrominoes::T,
                        Tetrominoes::T => Tetrominoes::J,
                        Tetrominoes::J => Tetrominoes::L,
                        Tetrominoes::L => Tetrominoes::S,
                        Tetrominoes::S => Tetrominoes::Z,
                        _ => Tetrominoes::I,
                    };
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
        //
        // Horizional lines
        let mb = &mut graphics::MeshBuilder::new();
        for y in (0..=520).step_by(20) {
            let y = y as f32;
            mb.line(
                &[Point2::new(200.0, y), Point2::new(480.0, y)],
                2.0,
                Color::new(0.9, 0.9, 0.9, 4.0),
            )?;
        }
        // Draw the vertical lines
        for x in (200..=480).step_by(20) {
            let x = x as f32;
            mb.line(
                &[Point2::new(x, 0.0), Point2::new(x, 520.0)],
                2.0,
                Color::new(0.9, 0.9, 0.9, 4.0),
            )?;
        }

        // Draw the border
        let fill = Color::new(1.0, 0.0, 0.0, 1.0);
        for y in 0..BOARD_HEIGHT - 1 {
            self.board[1][y] = TileType::Border;
            self.board[12][y] = TileType::Border;
        }
        for x in 1..BOARD_WIDTH - 2 {
            self.board[x][BOARD_HEIGHT - 2] = TileType::Border;
        }

        plot_tet(&mut self.board, self.piece, TileType::Tet);

        let mut full_lines: Vec<usize> = Vec::with_capacity(4);
        // Draw lines for board
        for (py, y) in (0..520).step_by(20).enumerate() {
                let mut row_count = 0;
            for (px, x) in (200..480).step_by(20).enumerate() {
                let x = x as f32;
                let y = y as f32;

                // XXX We should draw the border once at the start
                // of the program and not have to re-draw it each time.
                // Assuming I can figure out how to leave drawings behind
                // instead of wiping the screen each time.
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
                    TileType::Base => {
                        row_count += 1;
                        mb.line(
                            &[
                                Point2::new(x + 10.0, y + 2.0),
                                Point2::new(x + 10.0, y + 18.0),
                            ],
                            16.0,
                            Color::new(0.0, 1.0, 0.5, 1.0),
                        )?;
                    }
                    _ => (),
                }
            }
            if row_count >= 10 {
                full_lines.push(py);
                // redraw this as a blank, but only the actual
                // squares that pieces can operate on.
                for (px, x) in (200..460).step_by(20).enumerate() {
                    let x = x as f32;
                    let y = y as f32;
                    if self.board[px][py] == TileType::Base {
                        mb.line(
                            &[
                                Point2::new(x + 10.0, y + 2.0),
                                Point2::new(x + 10.0, y + 18.0),
                            ],
                            16.0,
                            Color::new(0.1, 0.2, 0.3, 1.0),
                        )?;
                        self.board[px][py] = TileType::Blank
                    }
                }
            }
        }

        let m = mb.build(ctx)?;
        graphics::draw(ctx, &m, DrawParam::new())?;

        // Finished drawing, show it all on the screen!
        graphics::present(ctx)?;

        // If we cleared a row, then move down what was above it.
        for fl in full_lines.iter() {
            for md_y in (0..*fl).rev() {
                println!("Move down line: {} ", md_y);
                for md_x in 2..12 {
                    self.board[md_x][md_y + 1] = self.board[md_x][md_y];
                }
                // Zero out the first line
                for md_x in 2..12 {
                    self.board[md_x][0] = TileType::Blank;
                }
            }
        }

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
