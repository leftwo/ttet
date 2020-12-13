use cgmath;
use ggez;
use ggez::event::{EventHandler, run, KeyMods, KeyCode, quit};
use ggez::graphics;
use ggez::graphics::{Color, DrawMode, DrawParam};
use ggez::nalgebra::Point2;
use ggez::timer;
use ggez::{Context, GameResult};
use ggez::input;

struct MainState {
    meshes: Vec<graphics::Mesh>,
    rotation: f32,
    piece_x: f32,
    piece_y: f32,
    board: [[bool; 10]; 20],
}

enum Tetrominoes {
    I,
    O,
    T,
    J,
    L,
    S,
    Z,
}

impl MainState {
    // I think this should be the static background that never changes
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let meshes = vec![build_mesh(ctx)?];
        let s = MainState {
            meshes,
            rotation: 0.0,
            piece_x: 300.0,
            piece_y: 40.0,
            board: [[true; 10]; 20],
        };
        Ok(s)
    }
}

fn build_mesh(ctx: &mut Context) -> GameResult<graphics::Mesh> {
    let mb = &mut graphics::MeshBuilder::new();

    mb.line(
        &[
            Point2::new(420.0, 20.0),
            Point2::new(420.0, 420.0),
            Point2::new(200.0, 420.0),
            Point2::new(200.0, 20.0),
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
		    self.rotation += 90.0f32;
		    if self.rotation >= 360.0 {
			self.rotation = 0.0;
		    }
		}
		input::keyboard::KeyCode::A => {
                    // Check for -10x 
		    self.piece_x -= 20.0f32;
		    if self.piece_x <= 200.0 {
			self.piece_x = 200.0;
		    }
		}
		input::keyboard::KeyCode::D => {
                    // Check for +10x 
		    self.piece_x += 20.0f32;
		    if self.piece_x >= 400.0 {
			self.piece_x = 400.0;
		    }
		}
		_ => (),
	    }
	}
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        // Text
        let text = graphics::Text::new(format!("Rotation:{}", self.rotation));
        graphics::draw(ctx, &text,
                       (Point2::new(20.0, 20.0),
                       graphics::WHITE),)?;

        // Create and draw a filled rectangle mesh.
        let rect = graphics::Rect::new(0.0, 0.0, 60.0, 20.0);
        let r1 = graphics::Mesh::new_rectangle(ctx,
                                               DrawMode::fill(),
                                               rect,
                                               graphics::WHITE)?;

        // This is where the object will be placed
        let dst2 = cgmath::Point2::new(self.piece_x, self.piece_y);
        // ZZZ scale allows us to scale the image
        let scale = cgmath::Vector2::new(1.0, 1.0);
        graphics::draw(ctx, &r1, graphics::DrawParam::new()
                                    .dest(dst2)
                                    .rotation(self.rotation.to_radians())
                                    .offset(Point2::new(30.0, 0.0))
                                    .scale(scale),
                        )?;


        // Create and draw a stroked rectangle mesh.
        let rect = graphics::Rect::new(450.0, 450.0, 50.0, 50.0);
        let r2 = graphics::Mesh::new_rectangle(
            ctx,
            DrawMode::stroke(2.0),
            rect,
            graphics::Color::new(1.0, 0.0, 0.0, 1.0),
        )?;
        graphics::draw(ctx, &r2, DrawParam::default())?;

        // Draw some pre-made meshes
        for m in &self.meshes {
            graphics::draw(ctx, m, DrawParam::new())?;
        }

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
