use cgmath;
use ggez;
use ggez::event::{EventHandler, run, KeyMods, KeyCode, quit};
use ggez::graphics;
use ggez::graphics::{Color, DrawMode, DrawParam};
use ggez::nalgebra::Point2;
use ggez::timer;
use ggez::{Context, GameResult};
use ggez::input;

enum Tetrominoes {
    I,
    O,
    T,
    J,
    L,
    S,
    Z,
}

const BOARD_HEIGHT: usize = 20;
const BOARD_WIDTH: usize = 10;

struct MainState {
    meshes: Vec<graphics::Mesh>,
    rotation: u8,
    piece_type: Tetrominoes,
    piece_x: usize,
    piece_y: usize,
    board: [[bool; BOARD_HEIGHT]; BOARD_WIDTH],
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let meshes = vec![build_mesh(ctx)?];
        let s = MainState {
            meshes,
            rotation: 0,
            piece_type: Tetrominoes::I,
            piece_x: 2,
            piece_y: 4,
            board: [[false; BOARD_HEIGHT]; BOARD_WIDTH],
        };
        Ok(s)
    }
}

fn build_mesh(ctx: &mut Context) -> GameResult<graphics::Mesh> {
    let mb = &mut graphics::MeshBuilder::new();

    // This is the game border
    mb.line(
        &[
            Point2::new(400.0, 0.0),
            Point2::new(400.0, 410.0),
            Point2::new(180.0, 410.0),
            Point2::new(180.0, 0.0),
        ],
        20.0,
        Color::new(0.5, 0.5, 1.0, 4.0),
    )?;

    mb.build(ctx)
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 1;

        if input::keyboard::is_key_pressed(ctx, KeyCode::Q) {
            println!("quit");
        }

		/*
        while timer::check_update_time(ctx, DESIRED_FPS) {
	    self.piece_y = self.piece_y + 20.0;
	    if self.piece_y > 380.0 {
	    	self.piece_y = 0.0;
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
        println!(
            "Key pressed: {:?}, modifier {:?}, repeat: {}",
            keycode, keymod, repeat
        );
	if repeat == false {
	    match keycode {
		input::keyboard::KeyCode::Q => quit(ctx),
		input::keyboard::KeyCode::W => {
                    // Check for rotation being legit
		    self.rotation += 1;
		    if self.rotation > 3 {
			self.rotation = 0;
		    }
		}
		input::keyboard::KeyCode::A => {
                    if self.piece_x > 0 {
                        self.piece_x -= 1;
		    }
		}
		input::keyboard::KeyCode::D => {
		    if self.piece_x < BOARD_WIDTH - 1 {
			self.piece_x += 1;
		    }
		}
		input::keyboard::KeyCode::S => {
		    if self.piece_y < BOARD_HEIGHT - 1 {
			self.piece_y += 1;
		    }
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

        // Draw some pre-made meshes
        for m in &self.meshes {
            graphics::draw(ctx, m, DrawParam::new())?;
        }

        // Text
        let text = graphics::Text::new(format!("Rotation:{}", self.rotation));
        graphics::draw(ctx, &text,
                       (Point2::new(20.0, 20.0), graphics::WHITE),)?;
        let text = graphics::Text::new(format!("piece_x:{}", self.piece_x));
        graphics::draw(ctx, &text,
                       (Point2::new(20.0, 40.0), graphics::WHITE),)?;
        let text = graphics::Text::new(format!("piece_y:{}", self.piece_y));
        graphics::draw(ctx, &text,
                       (Point2::new(20.0, 60.0), graphics::WHITE),)?;

        // This is the game grid
        // The x and y index are manually computed each time
        // we walk through the loop.  This is not a good way to do
        // this, we should come up with something better.  Maybe just
        // do all the work compute array ahead of time, then only
        // display

        let mb = &mut graphics::MeshBuilder::new();
        let mut py = 0;
        for y in (0..400).step_by(20) {
            let mut px = 0;
            for x in (200..400).step_by(20) {
                let x = x as f32;
                let y = y as f32;
                mb.line(
                    &[
                        Point2::new(x, y),
                        Point2::new(x, y + 20.0),
                    ],
                    20.0,
                    Color::new(0.9, 0.9, 0.9, 4.0),
                )?;

                if self.board[px][py] == false {
                    let fill = Color::new(0.0, 0.0, 0.0, 4.0);
                    mb.line(
                        &[
                            Point2::new(x, y + 1.0),
                            Point2::new(x, y + 19.0),
                        ],
                        18.0,
                        fill,
                    )?;
                }
                px += 1;
            }
            py += 1;
        }

        // Figure out where our piece is
        /*
        match self.piece_type {
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

    println!("{}", graphics::renderer_info(ctx)?);
    let state = &mut MainState::new(ctx).unwrap();
    run(ctx, events_loop, state)
}
