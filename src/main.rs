use std::f32;

use ggez::{mint::Point2, timer, Context, ContextBuilder, GameResult};
use ggez::event::{EventHandler, KeyCode, self};
use ggez::graphics::{Align, BLACK, DrawMode, DrawParam, FilterMode, Mesh, Rect, self, Text, WHITE};
use ggez::input::{keyboard::{KeyMods, self}, mouse};

use rand::prelude::*;

const ROTATION_SPEED: f32 = 4f32; // rotate 10 degrees per second
const MIN_DISTANCE: f32 = 16f32; // minimum following distance
const MOVE_SPEED: f32 = 64f32; // how quickly objects can move

#[derive(Clone, Copy)]
enum CoinType {
    Coin, // (YELLOW) up the number of coins that spawn
    Damage, // (RED) hurt player picking up coin
    // others
}

/// State information for the coin grab game
struct State {
    // shapes that will be drawn
    rectangle_mesh: Mesh,
    circle_mesh: Mesh,

    // helpful state information
    mouse: Point2<f32>,
    player: Vec<DrawParam>,
    coins: Vec<(CoinType, DrawParam)>,
    spawn_rate: usize,
    collected: usize,
    message: Text,
}

impl State {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut rng = rand::thread_rng();

        let rectangle_mesh = Mesh::new_rectangle(ctx, DrawMode::stroke(2f32), Rect::new(0f32, 0f32, 64f32, 64f32), WHITE)?;
        let circle_mesh = Mesh::new_circle(ctx, DrawMode::fill(), [32f32, 32f32], 32f32, 0.25f32, WHITE)?;

        let player = vec![
            DrawParam::default().dest([64f32, 64f32]).offset([32f32, 32f32]).rotation(rng.gen_range(0f32, 360f32)),
            DrawParam::default().dest([64f32, 64f32]).offset([32f32, 32f32]).rotation(rng.gen_range(0f32, 360f32)).scale([0.9f32, 0.9f32]),
            DrawParam::default().dest([64f32, 64f32]).offset([32f32, 32f32]).rotation(rng.gen_range(0f32, 360f32)).scale([0.81f32, 0.81f32]),
        ];

        Ok(State {
            rectangle_mesh,
            circle_mesh,
            mouse: mouse::position(ctx),
            player,
            coins: vec![(CoinType::Coin, DrawParam::default().dest([300f32, 64f32]).color([1f32, 1f32, 0f32, 1f32].into()))],
            spawn_rate: 1usize,
            collected: 0usize,
            message: Text::new("Try collecting that coin over there"),
        })
    }

    fn collect_coin(&mut self, coin_type: CoinType) {
        self.collected += 1usize;

        match coin_type {
            CoinType::Coin => { self.spawn_rate += 1usize; self.message = Text::new("Increased number of coins that spawn!"); },
            CoinType::Damage => { self.player.pop(); self.message = Text::new("Ouch that coin hurt!"); },
        }

        self.coins.clear();
        let mut rng = rand::thread_rng();
        for _ in 0..self.spawn_rate {
            let x = rng.gen_range(64f32, 736f32);
            let y = rng.gen_range(64f32, 536f32);
            match rng.gen_range(0usize, 2usize) {
                0usize => self.coins.push((CoinType::Coin, DrawParam::default().dest([x, y]).color([1f32, 1f32, 0f32, 1f32].into()))),
                1usize => self.coins.push((CoinType::Damage, DrawParam::default().dest([x, y]).color([1f32, 0f32, 0f32, 1f32].into()))),
                _ => println!("Oops did not spawn coin..."),
            }
        }
    }
}

impl EventHandler for State {
    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.mouse.x = x;
        self.mouse.y = y;
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if !self.player.is_empty() {
            let delta = timer::delta(ctx);
            let delta_f32 = timer::duration_to_f64(delta) as f32;

            // test for collisions (should use iterator)
            let mut val = (false, CoinType::Coin);
            for coin in self.coins.iter() {
                if (coin.1.dest.x - self.player[0].dest.x).abs() < 32f32 && (coin.1.dest.y - self.player[0].dest.y).abs() < 32f32 {
                    println!("Collected Coin at position {:?}", coin.1.dest);
                    val = (true, coin.0);
                    break;
                }
            }
            if val.0 {
                self.collect_coin(val.1);
            }

            // update player location
            let mut goto = self.mouse;
            goto.x -= 32f32;
            goto.y -= 32f32;
            for player in self.player.iter_mut() {
                // only move if minimum distance away
                if  (goto.x - player.dest.x).abs() > MIN_DISTANCE || (goto.y - player.dest.y).abs() > MIN_DISTANCE {
                    // move horizontally
                    if goto.x > player.dest.x {
                        player.dest.x += delta_f32 * MOVE_SPEED;
                    } else {
                        player.dest.x -= delta_f32 * MOVE_SPEED;
                    }
                    // move virtically
                    if goto.y > player.dest.y {
                        player.dest.y += delta_f32 * MOVE_SPEED;
                    } else {
                        player.dest.y -= delta_f32 * MOVE_SPEED;
                    }
                }
                // rotate
                player.rotation += delta_f32 * ROTATION_SPEED;
                // update following point
                goto = player.dest;
            }
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

        if !self.player.is_empty() {
            // draw player
            for player in self.player.iter().rev() {
                graphics::draw(ctx, &self.rectangle_mesh, *player)?;
                graphics::draw(ctx, &self.circle_mesh, *player)?;
            }

            // draw coins
            for coin in self.coins.iter() {
                graphics::draw(ctx, &self.circle_mesh, coin.1)?;
            }
        }

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
