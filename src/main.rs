use ggez::event::{quit, run, EventHandler, KeyCode, KeyMods};
use ggez::graphics;
use ggez::graphics::{Color, DrawParam};
use ggez::input;
use ggez::nalgebra::Point2;
use ggez::timer;
use ggez::{Context, GameResult};
use queues::*;
use rand::{
    distributions::{Distribution, Standard},
    seq::SliceRandom,
    Rng,
};

// The seven different game pieces we use.  Their official name in Tetris
// is "Tetrominoes".
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

// Pick a random Tetrominoe from our enum
// Modified from https://stackoverflow.com/questions/48490049
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

// We use a 2d array (basically) to hold the board state and to know
// where to draw the pieces, the base (pieces that hit the floor) and
// the borders of the playing field.
// The visible board size is 10x20. We have additional space in the
// array for reasons that involve how we plot the tetrominoes and
// translate their x,y location when plotting on the board.
const BOARD_HEIGHT: usize = 26;
const BOARD_WIDTH: usize = 14;

// Each board square can be one of these choices.
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

// Board state is used to indicate the different states that the
// board can be in during game play.  Specific states will change how
// things are displayed and what movement is allowed.
//
// Moving: The normal state of things, the player can rotate
// pieces, and pieces drop at every interval.
//
// Clearing: We freeze the board for one interval during which a
// completed row is blanked out.  At the end of Clearing state, the
// pieces above a cleared row fall to fill the empty rows.
//
// Paused: The player has requested a pause, no falling
// or rotation is allowed.  The piece holds at its current location
// and no bounds checking is performed.
//
// Over: The attempt to place a new piece at the top of the board
// has failed, meaning at least one square is occupied already.
#[derive(Debug, PartialEq)]
enum BoardState {
    Moving,
    Clearing,
    Paused,
    Over,
}

// Debug function to print the board
fn print_board(board: [[TileType; BOARD_HEIGHT]; BOARD_WIDTH]) {
    for y in 0..BOARD_HEIGHT {
        print!("[{:02}] ", y);
        for x in 0..BOARD_WIDTH {
            if board[x][y] == TileType::Base {
                print!("B");
            } else if board[x][y] == TileType::Border {
                print!("#");
            } else if board[x][y] == TileType::Tet {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

// Check a vector of points to see if a piece can be placed on
// those places on the board.  Return true if it is possible,
// false if it is not possible.
fn check_points(board: [[TileType; BOARD_HEIGHT]; BOARD_WIDTH], ptc: Vec<(usize, usize)>) -> bool {
    for pt in ptc.iter() {
        if board[pt.0][pt.1] == TileType::Base || board[pt.0][pt.1] == TileType::Border {
            return false;
        }
    }
    true
}

// Check to see if the piece in the requested location and rotation
// will fit on the given board.
fn validate_move(board: [[TileType; BOARD_HEIGHT]; BOARD_WIDTH], piece: Piece) -> bool {
    let x = piece.x;
    let y = piece.y;

    // We put the points of the desired position into a vector
    // then check at the end if all points are valid.
    let mut ptc: Vec<(usize, usize)> = Vec::with_capacity(4);
    match piece.tet_type {
        Tetrominoes::I => match piece.rotation {
            0 => {
                ptc.push((x, y + 1));
                ptc.push((x + 1, y + 1));
                ptc.push((x + 2, y + 1));
                ptc.push((x + 3, y + 1));
            }
            1 => {
                ptc.push((x + 2, y));
                ptc.push((x + 2, y + 1));
                ptc.push((x + 2, y + 2));
                ptc.push((x + 2, y + 3));
            }
            2 => {
                ptc.push((x, y + 2));
                ptc.push((x + 1, y + 2));
                ptc.push((x + 2, y + 2));
                ptc.push((x + 3, y + 2));
            }
            _ => {
                ptc.push((x + 1, y));
                ptc.push((x + 1, y + 1));
                ptc.push((x + 1, y + 2));
                ptc.push((x + 1, y + 3));
            }
        },
        Tetrominoes::O => {
            ptc.push((x, y));
            ptc.push((x + 1, y));
            ptc.push((x, y + 1));
            ptc.push((x + 1, y + 1));
        }
        Tetrominoes::T => match piece.rotation {
            0 => {
                ptc.push((x + 1, y));
                ptc.push((x, y + 1));
                ptc.push((x + 1, y + 1));
                ptc.push((x + 2, y + 1));
            }
            1 => {
                ptc.push((x + 1, y));
                ptc.push((x + 1, y + 1));
                ptc.push((x + 1, y + 2));
                ptc.push((x + 2, y + 1));
            }
            2 => {
                ptc.push((x, y + 1));
                ptc.push((x + 1, y + 1));
                ptc.push((x + 2, y + 1));
                ptc.push((x + 1, y + 2));
            }
            _ => {
                ptc.push((x, y + 1));
                ptc.push((x + 1, y));
                ptc.push((x + 1, y + 1));
                ptc.push((x + 1, y + 2));
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
                ptc.push((x + 2, y));
                ptc.push((x, y + 1));
                ptc.push((x + 1, y + 1));
                ptc.push((x + 2, y + 1));
            }
            1 => {
                ptc.push((x + 1, y));
                ptc.push((x + 1, y + 1));
                ptc.push((x + 1, y + 2));
                ptc.push((x + 2, y + 2));
            }
            2 => {
                ptc.push((x, y + 1));
                ptc.push((x + 1, y + 1));
                ptc.push((x + 2, y + 1));
                ptc.push((x, y + 2));
            }
            _ => {
                ptc.push((x, y));
                ptc.push((x + 1, y));
                ptc.push((x + 1, y + 1));
                ptc.push((x + 1, y + 2));
            }
        },
        Tetrominoes::S => match piece.rotation {
            0 => {
                ptc.push((x + 1, y));
                ptc.push((x + 2, y));
                ptc.push((x, y + 1));
                ptc.push((x + 1, y + 1));
            }
            1 => {
                ptc.push((x + 1, y));
                ptc.push((x + 1, y + 1));
                ptc.push((x + 2, y + 1));
                ptc.push((x + 2, y + 2));
            }
            2 => {
                ptc.push((x + 1, y + 1));
                ptc.push((x + 2, y + 1));
                ptc.push((x, y + 2));
                ptc.push((x + 1, y + 2));
            }
            _ => {
                ptc.push((x, y));
                ptc.push((x, y + 1));
                ptc.push((x + 1, y + 1));
                ptc.push((x + 1, y + 2));
            }
        },
        Tetrominoes::Z => match piece.rotation {
            0 => {
                ptc.push((x, y));
                ptc.push((x + 1, y));
                ptc.push((x + 1, y + 1));
                ptc.push((x + 2, y + 1));
            }
            1 => {
                ptc.push((x + 2, y));
                ptc.push((x + 1, y + 1));
                ptc.push((x + 2, y + 1));
                ptc.push((x + 1, y + 2));
            }
            2 => {
                ptc.push((x, y + 1));
                ptc.push((x + 1, y + 1));
                ptc.push((x + 1, y + 2));
                ptc.push((x + 2, y + 2));
            }
            _ => {
                ptc.push((x + 1, y));
                ptc.push((x, y + 1));
                ptc.push((x + 1, y + 1));
                ptc.push((x, y + 2));
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
fn plot_tet(board: &mut [[TileType; BOARD_HEIGHT]; BOARD_WIDTH], piece: Piece, new_type: TileType) {
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
// down no further.  We convert the squares the piece contained to the base
// type, then we check to see if any rows have been filled.
//
// A filled row,       return BoardState::Clearing
// No filled rows,     return BoardState::Moving (but see below)
//
// If we don't clear rows, then we generate the next piece at the top of
// the board.  If we are clearing, then don't generate the next piece.
//
// If we are not clearing rows, then it's possible that the placement of
// the new piece will fail, in that case the tail call will end up
// returning BoardState::Over
//
fn convert_and_check(
    board: &mut [[TileType; BOARD_HEIGHT]; BOARD_WIDTH],
    piece: &mut Piece,
    piece_queue: &mut TetQueue,
    next_board: &mut [[TileType; BOARD_HEIGHT]; BOARD_WIDTH],
    next_piece: &mut Piece,
) -> BoardState {
    // Redraw the piece as a "base" type
    plot_tet(board, *piece, TileType::Base);

    // See if there are any "full" rows.
    for y in (*piece).y..BOARD_HEIGHT - 2 {
        let mut row_count = 0;
        for x in 2..BOARD_WIDTH - 2 {
            if board[x][y] == TileType::Base {
                row_count += 1;
            }
        }
        if row_count == 10 {
            // We have at least one full row, go ahead and tell the caller the
            // new state.  We don't care how many rows are full, one is
            // enough to know the board state is changing to clearing and
            // we can return now.
            return BoardState::Clearing;
        }
    }

    // We can go ahead with placing a new piece now, return the
    // result of this call
    place_new_piece(board, piece, piece_queue, next_board, next_piece)
}

// This starts a new piece moving down from the top of the
// board.  We also check for game over if the new piece has
// no empty squares to be placed in.
fn place_new_piece(
    board: &mut [[TileType; BOARD_HEIGHT]; BOARD_WIDTH],
    piece: &mut Piece,
    piece_queue: &mut TetQueue,
    next_board: &mut [[TileType; BOARD_HEIGHT]; BOARD_WIDTH],
    next_piece: &mut Piece,
) -> BoardState {
    let mut res = BoardState::Moving;

    (*piece).tet_type = piece_queue.next();
    if (*piece).tet_type == Tetrominoes::I {
        (*piece).rotation = 1;
        (*piece).y = 0;
    } else {
        (*piece).rotation = 0;
        (*piece).y = 2;
    }
    (*piece).x = 6;

    // Clear the old and and fill the next_board array with the next piece
    plot_tet(next_board, *next_piece, TileType::Blank);
    *next_piece = set_next_piece(piece_queue.peek());
    plot_tet(next_board, *next_piece, TileType::Tet);

    if !validate_move(*board, *piece) {
        println!("Game Over");
        res = BoardState::Over;
    }
    res
}

// Try to move tet down.  If it works, then return true.
// If the move is not valid, then return false.
fn move_tet_down(board: &mut [[TileType; BOARD_HEIGHT]; BOARD_WIDTH], piece: &mut Piece) -> bool {
    (*piece).y += 1;
    if validate_move(*board, *piece) {
        (*piece).y -= 1;
        plot_tet(board, *piece, TileType::Blank);
        (*piece).y += 1;
        return true;
    } // else
    (*piece).y -= 1;
    false
}

// Scoring formula, Original NES, based on lines
// cleared and current level:
fn get_score(cleared: u32, level: u32) -> u32 {
    assert!(cleared > 0 && cleared < 5);
    match cleared {
        1 => 40 * (level + 1),
        2 => 100 * (level + 1),
        3 => 300 * (level + 1),
        _ => 1200 * (level + 1),
    }
}

// This struct keeps track of the coming tetrominoes.
// We randomly shuffle the seven valid tets, then put them
// on a queue whenever the queue gets low.
// This also enables us to print the next tetrominoe.
// #[derive(Debug, Clone, Copy)]
#[derive(Debug)]
struct TetQueue {
    q: Queue<Tetrominoes>,
    x: [Tetrominoes; 7],
    rng: rand::rngs::ThreadRng,
}

// We use the random bag method for selecting the next tet.
impl TetQueue {
    fn new() -> TetQueue {
        let mut q: Queue<Tetrominoes> = queue![];
        let mut x = [
            Tetrominoes::I,
            Tetrominoes::O,
            Tetrominoes::T,
            Tetrominoes::J,
            Tetrominoes::L,
            Tetrominoes::S,
            Tetrominoes::Z,
        ];

        let mut rng = rand::thread_rng();
        x.shuffle(&mut rng);
        for tet in &x {
            q.add(*tet).unwrap();
        }
        TetQueue { q, x, rng }
    }
    fn next(&mut self) -> Tetrominoes {
        if self.q.size() <= 1 {
            self.x.shuffle(&mut self.rng);
            for tet in &self.x {
                self.q.add(*tet).unwrap();
            }
        }
        self.q.remove().unwrap()
    }
    fn peek(&mut self) -> Tetrominoes {
        self.q.peek().unwrap()
    }
}

struct MainState {
    // The main game board
    board: [[TileType; BOARD_HEIGHT]; BOARD_WIDTH],
    // The place where we render the next piece
    next_board: [[TileType; BOARD_HEIGHT]; BOARD_WIDTH],
    piece: Piece,
    next_piece: Piece,
    piece_queue: TetQueue,
    board_state: BoardState,
    score: u32,
    level: u32,
    lines: u32,
}

fn set_next_piece(next_tet: Tetrominoes) -> Piece {
    let mut x = 1;
    let mut y = 1;
    let mut rotation = 0;

    if next_tet == Tetrominoes::I {
        x = 0;
        y = 0;
        rotation = 1;
    };
    let next_piece = Piece {
        tet_type: next_tet,
        rotation,
        x,
        y,
    };
    next_piece
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let mut q = TetQueue::new();

        let piece = Piece {
            tet_type: q.next(),
            rotation: 0,
            x: 6,
            y: 0,
        };
        let next_piece = set_next_piece(q.peek());

        let s = MainState {
            board: [[TileType::Blank; BOARD_HEIGHT]; BOARD_WIDTH],
            next_board: [[TileType::Blank; BOARD_HEIGHT]; BOARD_WIDTH],
            piece,
            next_piece,
            piece_queue: q,
            board_state: BoardState::Moving,
            score: 0,
            level: 0,
            lines: 0,
        };
        Ok(s)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if input::keyboard::is_key_pressed(ctx, KeyCode::Q) {
            println!("quit");
        }
        while timer::check_update_time(ctx, self.level + 1) {
            match self.board_state {
                BoardState::Clearing => {
                    // Once we have done one cycle clear, we then resume
                    // part movement and drop down pieces above our cleared
                    // row(s)

                    // Start from the bottom and work our way up.
                    // The destination Y always starts at the highest valid
                    // valid Y we can have pieces at.
                    // We walk the array from the bottom up.
                    let mut y_dest = BOARD_HEIGHT - 3;
                    let mut cleared = 0;
                    for y in (0..BOARD_HEIGHT - 2).rev() {
                        let mut row_count = 0;
                        for x in 2..BOARD_WIDTH - 2 {
                            if self.board[x][y] == TileType::Base {
                                row_count += 1;
                            }
                        }

                        if row_count == 10 {
                            cleared += 1;
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
                    }

                    self.lines += cleared;
                    if self.lines >= 10 {
                        self.lines = 0;
                        self.level += 1;
                    }
                    self.score += get_score(cleared, self.level);

                    // Now make the new piece.
                    self.board_state = place_new_piece(
                        &mut self.board,
                        &mut self.piece,
                        &mut self.piece_queue,
                        &mut self.next_board,
                        &mut self.next_piece,
                    );
                }
                BoardState::Paused => (),
                BoardState::Over => (),
                BoardState::Moving => {
                    if !move_tet_down(&mut self.board, &mut self.piece) {
                        self.board_state = convert_and_check(
                            &mut self.board,
                            &mut self.piece,
                            &mut self.piece_queue,
                            &mut self.next_board,
                            &mut self.next_piece,
                        );
                        // Should we always set new piece here?? ZZZ
                    }
                }
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
                // Pause the game
                input::keyboard::KeyCode::P => {
                    if self.board_state == BoardState::Paused {
                        self.board_state = BoardState::Moving;
                    } else {
                        // XXX Don't pause if game over...
                        self.board_state = BoardState::Paused;
                    }
                }
                // Rotate
                input::keyboard::KeyCode::W => {
                    // Check for rotation being legit
                    // Erase current location

                    let original_rotation = self.piece.rotation;
                    self.piece.rotation += 1;
                    if self.piece.rotation > 3 {
                        self.piece.rotation = 0;
                    }
                    if validate_move(self.board, self.piece) {
                        let new_rotation = self.piece.rotation;
                        self.piece.rotation = original_rotation;
                        plot_tet(&mut self.board, self.piece, TileType::Blank);
                        self.piece.rotation = new_rotation;
                    } else {
                        self.piece.rotation = original_rotation;
                    }
                }
                // Move left
                input::keyboard::KeyCode::A => {
                    // The I piece has a rotation that could have an x value
                    // of zero, so we have to prevent it underflowing
                    if self.piece.x > 0 {
                        self.piece.x -= 1;
                        if validate_move(self.board, self.piece) {
                            self.piece.x += 1;
                            plot_tet(&mut self.board, self.piece, TileType::Blank);
                            self.piece.x -= 1;
                        } else {
                            self.piece.x += 1;
                        }
                    }
                }
                // Move right
                input::keyboard::KeyCode::D => {
                    self.piece.x += 1;
                    if validate_move(self.board, self.piece) {
                        self.piece.x -= 1;
                        plot_tet(&mut self.board, self.piece, TileType::Blank);
                        self.piece.x += 1;
                    } else {
                        self.piece.x -= 1;
                    }
                }
                // Single down
                input::keyboard::KeyCode::S => {
                    if !move_tet_down(&mut self.board, &mut self.piece) {
                        self.board_state = convert_and_check(
                            &mut self.board,
                            &mut self.piece,
                            &mut self.piece_queue,
                            &mut self.next_board,
                            &mut self.next_piece,
                        );
                    }
                }
                // All the way down
                input::keyboard::KeyCode::Space => {
                    while move_tet_down(&mut self.board, &mut self.piece) {
                        println!("Down");
                        // XXX This needs to not do the convert and check yet,
                        // let the timer run out (so we can slide to the side)
                        // before making final the piece.
                    }
                    self.board_state = convert_and_check(
                        &mut self.board,
                        &mut self.piece,
                        &mut self.piece_queue,
                        &mut self.next_board,
                        &mut self.next_piece,
                    );
                }
                // Begin debug commands
                input::keyboard::KeyCode::Z => {
                    println!("{:#?}", self.board);
                }
                input::keyboard::KeyCode::C => {
                    self.board_state = convert_and_check(
                        &mut self.board,
                        &mut self.piece,
                        &mut self.piece_queue,
                        &mut self.next_board,
                        &mut self.next_piece,
                    );
                }
                input::keyboard::KeyCode::Y => {
                    if self.level > 1 {
                        self.level -= 1;
                    }
                }
                input::keyboard::KeyCode::U => {
                    if self.level < 255 {
                        self.level += 1;
                    }
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
        let text = graphics::Text::new(format!("Board state:{:#?}", self.board_state));
        graphics::draw(ctx, &text, (Point2::new(10.0, 60.0), graphics::WHITE))?;
        let text = graphics::Text::new(format!("Score:{}", self.score));
        graphics::draw(ctx, &text, (Point2::new(10.0, 80.0), graphics::WHITE))?;
        let text = graphics::Text::new(format!("Lines:{}", self.lines));
        graphics::draw(ctx, &text, (Point2::new(10.0, 100.0), graphics::WHITE))?;
        let text = graphics::Text::new(format!("Level:{}", self.level));
        graphics::draw(ctx, &text, (Point2::new(10.0, 120.0), graphics::WHITE))?;
        let next = self.piece_queue.peek();
        let text = graphics::Text::new(format!("Next: {:?}", next));
        graphics::draw(ctx, &text, (Point2::new(10.0, 140.0), graphics::WHITE))?;

        // The board grid
        // Horizional lines for the playfield
        let mb = &mut graphics::MeshBuilder::new();

        for y in (0..=520).step_by(20) {
            let y = y as f32;
            mb.line(
                &[Point2::new(200.0, y), Point2::new(480.0, y)],
                2.0,
                Color::new(0.9, 0.9, 0.9, 4.0),
            )?;
        }
        // Draw the vertical lines for the playfield
        for x in (200..=480).step_by(20) {
            let x = x as f32;
            mb.line(
                &[Point2::new(x, 0.0), Point2::new(x, 520.0)],
                2.0,
                Color::new(0.9, 0.9, 0.9, 4.0),
            )?;
        }

        // Draw the border squares
        let fill = Color::new(1.0, 0.0, 0.0, 1.0);
        for y in 0..BOARD_HEIGHT - 1 {
            self.board[1][y] = TileType::Border;
            self.board[12][y] = TileType::Border;
        }
        for x in 1..BOARD_WIDTH - 2 {
            self.board[x][BOARD_HEIGHT - 2] = TileType::Border;
        }

        // When clearing, the piece location has become base, so
        // don't draw anything.
        if self.board_state != BoardState::Clearing {
            plot_tet(&mut self.board, self.piece, TileType::Tet);
        }

        // Draw content on the board
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

        // Draw the horizional lines for the next box
        for y in (20..=100).step_by(20) {
            let y = y as f32;
            mb.line(
                &[Point2::new(500.0, y), Point2::new(580.0, y)],
                2.0,
                Color::new(0.9, 0.9, 0.9, 4.0),
            )?;
        }
        // Draw the vertical lines for the next box
        for x in (500..=580).step_by(20) {
            let x = x as f32;
            mb.line(
                &[Point2::new(x, 20.0), Point2::new(x, 100.0)],
                2.0,
                Color::new(0.9, 0.9, 0.9, 4.0),
            )?;
        }

        // When we add a game start button, we can do this less
        // frequently.  The only time it needs to change is on start
        // and when a piece has reached the bottom.  XXX
        plot_tet(&mut self.next_board, self.next_piece, TileType::Tet);
        // Draw content in the next box
        for (py, y) in (20..100).step_by(20).enumerate() {
            for (px, x) in (500..580).step_by(20).enumerate() {
                let x = x as f32;
                let y = y as f32;
                if self.next_board[px][py] == TileType::Tet {
                    mb.line(
                        &[
                            Point2::new(x + 10.0, y + 2.0),
                            Point2::new(x + 10.0, y + 18.0),
                        ],
                        16.0,
                        Color::new(0.0, 1.0, 1.0, 1.0),
                    )?;
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
