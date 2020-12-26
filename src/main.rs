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

// Board state is used to indicate the different states that the
// board can be in during game play.
// Moving is the normal sate of things, the player can rotate
// pieces, and pieces drop at every interval.
// Clearing is when we hold the board for one interval during
// which a completed row is blanked out.  At the end of Clearing
// state, the pieces above fall to fill the gap created.
// Paused is when the player has requested a pause, no falling
// or rotation is allowed.
#[derive(Debug, PartialEq)]
enum BoardState {
    Moving,
    Clearing,
    Paused,
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

// Check to see if, given the current board and our current
// rotation, the squares for the next rotation are available to
// "move" to.
fn rotate_check(board: [[TileType; BOARD_HEIGHT]; BOARD_WIDTH], piece: Piece) -> bool {
    let x = piece.x;
    let y = piece.y;

    // We put the points of the desired position into a vector
    // then check at the end if all points are valid.
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

// On the given board, draw the shape of the Tetrominoe of
// the given type in the given rotation.
// The new_type can be any supported tile type, including blank
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

// Call this when the active piece has hit something below it and can move
// down no further.  We convert the piece to base type, then we check to
// see if any rows have been filled.
// A filled row,   return BoardState::Clearing
// No filled rows, return BoardState::Moving
fn convert_and_check(board: &mut [[TileType; BOARD_HEIGHT]; BOARD_WIDTH], piece: &mut Piece) -> BoardState {
    // Redraw the piece as a "base" type
    let mut res = BoardState::Moving;

    plot_tet(board, *piece, TileType::Base);

    for y in (*piece).y..BOARD_HEIGHT - 2 {
        let mut row_count = 0;
        for x in 2..BOARD_WIDTH - 2 {
            if board[x][y] == TileType::Base {
                row_count += 1;
            }
        }
        if row_count == 10 {
            // We have at least one full row, go ahead
            // and tell the caller we need to change
            // state to clearing
            println!("Found row {} is full", y);
            res = BoardState::Clearing;
            break;
        }
    }

    // Get a new piece ready and put it up at the top.
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

    res
}

fn move_tet_left(board: &mut [[TileType; BOARD_HEIGHT]; BOARD_WIDTH], piece: &mut Piece) {
    // Check if move left is possible.
    // If yes, then first clear current position.
    let x = (*piece).x;
    let y = (*piece).y;

    // For this and all the movement functions, we build a vector with
    // all the points we need to verify are open before the move can happen.
    // At the end of the match, we check to verify all of the needed points
    // are valid.  Note that it's ok if a point is occupied by ourselves.
    let mut ptc: Vec<(usize, usize)> = Vec::with_capacity(4);

    // For checking, we only need to check points that are on the side we
    // wish to move into.   We don't need to check points that we ourselves
    // occupy.
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
    board_state: BoardState,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let piece = Piece {
            tet_type: rand::random(),
            rotation: 0,
            x: 6,
            y: 0,
        };
        let s = MainState {
            board: [[TileType::Blank; BOARD_HEIGHT]; BOARD_WIDTH],
            piece,
            board_state: BoardState::Moving,
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
            if self.board_state == BoardState::Clearing {
                // Once we have done one cycle clear, we then resume
                // part movement and drop down pieces above our cleared
                // row(s)

                println!("Cleared time elapsed, now drop");

                // Clear out the piece so we don't move it down with
                // the base pieces.  We re-draw it at the start of the
                // draw method.
                plot_tet(&mut self.board, self.piece, TileType::Blank);
                // update score XXX after counting full rows

                // Start from the bottom and work our way up.
                // The destination Y always starts at the highest valid
                // valid Y we can have pieces at.
                // We walk the array from the bottom up.
                let mut y_dest = BOARD_HEIGHT - 3;
                for y in (0..BOARD_HEIGHT - 2).rev() {
                    let mut row_count = 0;
                    for x in 2..BOARD_WIDTH - 2 {
                        if self.board[x][y] == TileType::Base {
                            row_count += 1;
                        }
                    }
                    if row_count == 10 {
                        continue;
                    }

                    // Move the source y to the current y.
                    for x in 2..BOARD_WIDTH - 2 {
                        self.board[x][y_dest] = self.board[x][y];
                    }

                    y_dest -= 1;
                }
                // Clear out any rows left at the top
                for y in 0..=y_dest {
                    for x in 2..BOARD_WIDTH - 2 {
                        self.board[x][y] = TileType::Blank;
                    }
                    println!("assign [x][{}] to blank", y);
                }
                self.board_state = BoardState::Moving;
            }
            else if self.board_state == BoardState::Paused {
                println!("Paused");
            }
            else if !move_tet_down(&mut self.board, &mut self.piece) {
                println!("Piece has reached the bottom");
                self.board_state = convert_and_check(&mut self.board, &mut self.piece);
            }
        }
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        repeat: bool,
    ) {
        if !repeat {
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
                        self.board_state = convert_and_check(&mut self.board,
                                                            &mut self.piece);
                    }
                }
                // Begin debug commands
                input::keyboard::KeyCode::Z => {
                    println!("{:#?}", self.board);
                }
                input::keyboard::KeyCode::C => {
                    self.board_state = convert_and_check(&mut self.board,
                                                        &mut self.piece);
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
        let text = graphics::Text::new(format!("Board state:{:#?}", self.board_state));
        graphics::draw(ctx, &text, (Point2::new(10.0, 80.0), graphics::WHITE))?;

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
                // Redraw this as a blank, but only the actual
                // squares that pieces can operate on.
                for (px, x) in (200..460).step_by(20).enumerate() {
                    let x = x as f32;
                    let y = y as f32;
                    // The following should be asserted
                    if self.board[px][py] == TileType::Base {
                        mb.line(
                            &[
                                Point2::new(x + 10.0, y + 2.0),
                                Point2::new(x + 10.0, y + 18.0),
                            ],
                            16.0,
                            Color::new(0.1, 0.2, 0.3, 1.0),
                        )?;
                    }
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
