use ggez;
use ggez::graphics;
use ggez::{Context, GameResult};

use crate::actor_mods::actor::*;
use crate::assets::Assets;
use crate::draw::draw_text;
use crate::input::InputState;
use crate::vector2::Vector2;

const SHOT_SPEED: f32 = 100.0;

// TODO #2 無敵時間の実装
#[derive(Debug)]
pub struct ActorState {
    actor: Actor,
    shots: Vec<Actor>,
    shot_timeout: f32,
}

impl ActorState {
    pub fn new(actor: Actor) -> ActorState {
        ActorState {
            actor: actor,
            shots: Vec::new(),
            shot_timeout: 0.0,
        }
    }

    pub fn get_actor(&self) -> Actor {
        self.actor
    }
    pub fn get_mut_actor(&mut self) -> &mut Actor {
        &mut self.actor
    }
    pub fn get_shots(&self) -> &Vec<Actor> {
        &self.shots
    }
    pub fn get_mut_shots(&mut self) -> &mut Vec<Actor> {
        &mut self.shots
    }
    pub fn get_shot_timeout(&self) -> f32 {
        self.shot_timeout
    }

    fn _load() -> GameResult<ActorState> {
        unimplemented!();
    }

    pub fn fire_shot(&mut self, _ctx: &Context) {
        match self.actor.get_tag() {
            ActorType::Player => {
                const PLAYER_SHOT_TIME: f32 = 0.5;
                self.shot_timeout = PLAYER_SHOT_TIME;
                let player = &self.actor;
                let shot = create_circle_bullets(
                    player.get_x_y(),
                    5,
                    (-2.0 / 5.0, 0.0 / 5.0),
                    SHOT_SPEED,
                    0.0,
                );

                self.shots.extend(shot);
                // ctx は音声の再生に用いる
                // let _ = self.assets.shot_sound.play(ctx);
            }
            ActorType::Enemy => {
                const ENEMY_SHOT_TIME: f32 = 0.5;
                self.shot_timeout = ENEMY_SHOT_TIME;
                let enemy = &self.actor;
                let shot = create_circle_bullets(
                    enemy.get_x_y() + enemy.get_w_h() / 2.0,
                    7,
                    (0.0, 1.0),
                    SHOT_SPEED / 8.0,
                    0.01,
                );

                self.shots.extend(shot);
                // ctx は音声の再生に用いる
                // let _ = self.assets.shot_sound.play(ctx);
            }
            _ => (),
        }
    }

    pub fn handle_input(&mut self, input: &InputState, dt: f32) {
        const PLAYER_VEL: f32 = 4.0;
        self.actor.vel =
            Vector2(input.get_xaxis(), input.get_yaxis()) * dt * 10.0_f32.powf(PLAYER_VEL);
    }

    pub fn dec_shot_timeout(&mut self, amount: f32) {
        if amount.is_sign_positive() && self.shot_timeout.is_sign_positive() {
            self.shot_timeout -= amount;
        }
    }

    pub fn clear_dead_stuff(&mut self, screen_w_h: Vector2) {
        self.shots
            .retain(|s| s.inside_window(screen_w_h) && s.get_life() > 0);
    }

    pub fn update(&mut self, seconds: f32, screen_w_h: Vector2) {
        for shot in self.get_mut_shots() {
            shot.update_actor_position(seconds);
            // wrap_actor_position(shot, self.screen_w_h);
            shot.dec_life(1);
        }
        self.dec_shot_timeout(seconds);

        let actor = self.get_mut_actor();
        actor.dec_collision_timeout(seconds);
        actor.update_actor_position(seconds);
        actor.wrap_actor_position(screen_w_h);
    }

    pub fn draw(&self, ctx: &mut Context, assets: &Assets, coords: (f32, f32)) -> GameResult {
        let actor = self.get_actor();
        actor.draw(ctx, assets, coords)?;
        actor.draw_collision(ctx, graphics::WHITE, coords)?;
        // 残りライフの表示
        draw_text(
            ctx,
            format!("{:#?}", actor.get_life()),
            actor.get_x_y() + actor.get_w_h() * 2.0,
            24.0,
            assets.get_font(),
            coords,
        )?;

        let color = graphics::Color::new(0.0, 1.0, 1.0, 1.0);
        for shot in self.get_shots() {
            shot.draw(ctx, assets, coords)?;
            shot.draw_collision(ctx, color, coords)?;
        }
        Ok(())
    }
}
