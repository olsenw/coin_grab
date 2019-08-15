use std::f32;

use ggez::{mint::Point2, timer, Context, ContextBuilder, GameResult};
use ggez::event::{EventHandler, KeyCode, self};
use ggez::graphics::{Align, BLACK, DrawMode, DrawParam, FilterMode, Mesh, Rect, self, Text, WHITE};
use ggez::input::{keyboard::{KeyMods, self}, mouse};

use rand::prelude::*;

/// State information for the coin grab game
struct State {
    // shapes that will be drawn
    rectangle_mesh: Mesh,

    // helpful state information
    player: Vec<DrawParam>,
    mouse: Point2<f32>,
    message: Text,
    collected: usize,
}

impl State {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut rng = rand::thread_rng();

        let rectangle_mesh = Mesh::new_rectangle(ctx, DrawMode::stroke(2f32), Rect::new(0f32, 0f32, 64f32, 64f32), WHITE)?;

        let player = vec![
            DrawParam::default().dest([64f32, 64f32]).offset([32f32, 32f32]).rotation(rng.gen_range(0f32, 360f32)),
            DrawParam::default().dest([64f32, 64f32]).offset([32f32, 32f32]).rotation(rng.gen_range(0f32, 360f32)),
            DrawParam::default().dest([64f32, 64f32]).offset([32f32, 32f32]).rotation(rng.gen_range(0f32, 360f32)),
        ];

        Ok(State {
            rectangle_mesh,
            player,
            mouse: mouse::position(ctx),
            message: Text::new("Hello World"),
            collected: 0usize,
        })
    }
}

impl EventHandler for State {
    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.mouse.x = x;
        self.mouse.y = y;
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if !self.player.is_empty() {
            // update game
        } else {
            // game over
            self.message = Text::new(format!("Game Over\nCollected {} Coins", self.collected));
            self.message.set_bounds([self.message.width(ctx) as f32, self.message.height(ctx) as f32], Align::Center);
        }
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

    let state = &mut State::new(ctx).unwrap();

    match event::run(ctx, event_loop, state) {
        Ok(_) => println!("Goodbye, thanks for playing!"),
        Err(e) => eprintln!("Critical Error: {}", e),
    }
}
