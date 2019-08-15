use ggez::{nalgebra as na, timer, Context, ContextBuilder, GameResult};
use ggez::event::{EventHandler, KeyCode, self};
use ggez::graphics::{BLACK, DrawMode, DrawParam, FilterMode, Mesh, Rect, self, Text, WHITE};
use ggez::input::{keyboard::{KeyMods, self}, mouse};

/// State information for the coin grab game
struct State {
    message: Text,
}

impl State {
    fn new(_ctx: &mut Context) -> Self {
        State {
            message: Text::new("Hello World")
        }
    }
}

impl EventHandler for State {
    fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, _dx: f32, _dy: f32) {
        // track mouse
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, BLACK);

        // draw message at center of screen
        let width_height = (self.message.width(ctx) as f32 / 2f32, self.message.height(ctx) as f32 / 2f32);
        graphics::queue_text(ctx, &self.message, [400f32 - width_height.0, 300f32 - width_height.1], Some(WHITE));
        graphics::draw_queued_text(ctx, DrawParam::default(), None, FilterMode::Linear)?;

        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }
}

fn main() {
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("Coin Grab", "William Olsen").build().unwrap();

    let state = &mut State::new(ctx);

    match event::run(ctx, event_loop, state) {
        Ok(_) => println!("Goodbye, thanks for playing!"),
        Err(e) => eprintln!("Critical Error: {}", e),
    }
}
