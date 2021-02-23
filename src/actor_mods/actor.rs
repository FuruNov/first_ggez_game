use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

use oorandom::Rand32;
use std::str::FromStr;

use crate::assets::Assets;
use crate::vector2::{random_vec, vec_from_angle, Vector2};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ActorType {
    Player,
    Bullet,
    Enemy,
    Other,
}

impl Default for ActorType {
    fn default() -> Self {
        Self::Other
    }
}

impl FromStr for ActorType {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Player" => Ok(ActorType::Player),
            "Bullet" => Ok(ActorType::Bullet),
            "Enemy" => Ok(ActorType::Enemy),
            "Other" => Ok(ActorType::Other),
            _ => Err("Error"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Actor {
    tag: ActorType,
    x_y: Vector2,
    w_h: Vector2,
    facing: f32,
    pub vel: Vector2,
    ang_vel: f32,

    // I am going to lazily overload "life" with a
    // double meaning:
    // for shots, it is the time left to live,
    // for players and rocks, it is the actual hit points.
    life: i32,
    collision_timeout: f32,
    max_collision_timeout: f32,
}

impl Actor {
    pub fn new(
        tag: ActorType,
        x_y: Vector2,
        w_h: Vector2,
        facing: f32,
        vel: Vector2,
        ang_vel: f32,
        life: i32,
        max_collision_timeout: f32,
    ) -> Self {
        Actor {
            tag: tag,
            x_y: x_y,
            w_h: w_h,
            facing: facing,
            vel: vel,
            ang_vel: ang_vel,
            life: life,
            collision_timeout: 0.0,
            max_collision_timeout: max_collision_timeout,
        }
    }

    pub fn get_tag(&self) -> ActorType {
        self.tag
    }

    pub fn get_x_y(&self) -> Vector2 {
        self.x_y
    }

    pub fn get_w_h(&self) -> Vector2 {
        self.w_h
    }

    pub fn get_facing(&self) -> f32 {
        self.facing
    }

    pub fn get_life(&self) -> i32 {
        self.life
    }

    pub fn get_collision_timeout(&self) -> f32 {
        self.collision_timeout
    }

    pub fn dec_life(&mut self, amount: i32) {
        // 当たり判定の確認
        if true
        /* 実際のプレイでは、ライフは負に成り得ないので self.life > 0 */
        {
            self.life -= amount;
        }
    }

    pub fn dec_collision_timeout(&mut self, amount: f32) {
        if amount.is_sign_positive() && self.collision_timeout.is_sign_positive() {
            self.collision_timeout -= amount;
        }
    }

    pub fn set_collision_timeout(&mut self, amount: f32) {
        if amount.is_sign_positive() {
            self.collision_timeout = amount;
        }
    }

    pub fn draw(self, ctx: &mut Context, assets: &Assets, world_coords: (f32, f32)) -> GameResult {
        let x_y = self.get_x_y().world_to_screen_coords(world_coords);
        let x_y = na::Point2::new(x_y.0, x_y.1);
        let image = assets.actor_image(self);
        let drawparams = graphics::DrawParam::new()
            .dest(x_y)
            .rotation(self.get_facing() as f32)
            .offset(na::Point2::new(0.5, 0.5));

        graphics::draw(ctx, image, drawparams)?;
        Ok(())
    }

    pub fn draw_collision(
        self,
        ctx: &mut Context,
        color: graphics::Color,
        world_coords: (f32, f32),
    ) -> GameResult {
        let w_h = self.get_w_h();
        let x_y = self.get_x_y().world_to_screen_coords(world_coords);
        let x_y = na::Point2::new(x_y.0, x_y.1);
        let rect = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::stroke(2.0),
            na::Point2::new(0.0, 0.0),
            w_h.norm(),
            1.0,
            color,
        )?;
        let drawparams = graphics::DrawParam::new()
            .dest(x_y)
            .rotation(self.get_facing() as f32)
            .offset(na::Point2::new(0.5, 0.5));

        graphics::draw(ctx, &rect, drawparams)?;
        Ok(())
    }

    pub fn update_actor_position(&mut self, dt: f32) {
        const MAX_PHYSICS_VEL: f32 = 150.0;
        // Clamp the velocity to the max efficiently
        let vel_norm = self.vel.norm();
        if vel_norm > MAX_PHYSICS_VEL {
            self.vel = self.vel / vel_norm * MAX_PHYSICS_VEL;
        }
        if self.get_tag() != ActorType::Player {
            self.rotate_actor_position();
        }
        let dv = self.vel * dt;
        self.x_y += dv;
        self.facing += self.ang_vel;
    }

    fn rotate_actor_position(&mut self) {
        let vel_norm = self.vel.norm();
        let unit_vel = self.vel / vel_norm;
        let vel_after = unit_vel + vec_from_angle(self.facing);
        self.vel = vel_after * vel_norm;
    }

    pub fn wrap_actor_position(&mut self, screen_w_h: Vector2) {
        // Wrap screen
        let (sx, sy) = (screen_w_h.0, screen_w_h.1);
        let (screen_x_bounds, screen_y_bounds) = (sx / 2.0, sy / 2.0);

        if self.x_y.0 > screen_x_bounds {
            self.x_y -= Vector2(sx, 0.0);
        } else if self.x_y.0 < -screen_x_bounds {
            self.x_y += Vector2(sx, 0.0);
        };
        if self.x_y.1 > screen_y_bounds {
            self.x_y -= Vector2(0.0, sy);
        } else if self.x_y.1 < -screen_y_bounds {
            self.x_y += Vector2(0.0, sy);
        }
    }

    pub fn inside_window(&self, screen_w_h: Vector2) -> bool {
        let (sx, sy) = (screen_w_h.0, screen_w_h.1);
        let (screen_x_bounds, screen_y_bounds) = (sx / 2.0, sy / 2.0);
        self.x_y.0.abs() < screen_x_bounds && self.x_y.1.abs() < screen_y_bounds
    }

    pub fn handle_actor_collision(&mut self, bullet: Actor) {
        let player_size = self.w_h.norm() / 2.0;
        let pdistance = (bullet.x_y - self.x_y).norm();
        let bullet_size = bullet.w_h.norm();
        if pdistance < player_size + bullet_size && self.get_collision_timeout() < 0.0 {
            self.set_collision_timeout(self.max_collision_timeout);
            self.dec_life(1);
        }
    }
}

pub fn create_player() -> Actor {
    const PLAYER_LIFE: i32 = 10;
    const PLAYER_WIDTH: f32 = 8.0;
    const PLAYER_HEIGHT: f32 = 8.0;
    Actor::new(
        ActorType::Player,
        Vector2(0.0, -300.0),
        Vector2(PLAYER_WIDTH, PLAYER_HEIGHT),
        0.0,
        Vector2(0.0, 0.0),
        0.0,
        PLAYER_LIFE,
        0.5,
    )
}

pub fn create_rand_bullets(rng: &mut Rand32, x_y: Vector2, num: i32) -> Vec<Actor> {
    const MAX_BULLET_VEL: f32 = 50.0;
    let new_bullet = |_| {
        let r_angle = rng.rand_float() * 2.0 * std::f32::consts::PI;
        let r_distance = rng.rand_float();
        let bullet = create_bullet(
            x_y + vec_from_angle(r_angle) * r_distance,
            Vector2(3.0, 3.0),
            r_angle,
            random_vec(rng, MAX_BULLET_VEL),
            0.0,
        );
        bullet
    };
    (0..num).map(new_bullet).collect()
}

pub fn create_circle_bullets(
    x_y: Vector2,
    num: i32,
    range: (f32, f32),
    vel_norm: f32,
    ang_vel: f32,
) -> Vec<Actor> {
    let new_bullet = |i| {
        let r_angle =
            (i as f32 / num as f32 + range.0) * (range.1 - range.0) * (2.0 * std::f32::consts::PI);
        let bullet = create_bullet(
            x_y + vec_from_angle(r_angle),
            Vector2(4.0, 4.0),
            r_angle,
            vec_from_angle(r_angle) * vel_norm,
            ang_vel,
        );
        bullet
    };

    (0..num).map(new_bullet).collect()
}

pub fn create_bullet(x_y: Vector2, w_h: Vector2, facing: f32, vel: Vector2, ang_vel: f32) -> Actor {
    const BULLET_LIFE: i32 = i32::MAX;
    Actor::new(
        ActorType::Bullet,
        x_y,
        w_h,
        facing,
        vel,
        ang_vel,
        BULLET_LIFE,
        f32::MAX,
    )
}
