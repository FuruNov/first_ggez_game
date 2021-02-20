use oorandom::Rand32;
use std::str::FromStr;

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
        collision_timeout: f32,
    ) -> Self {
        Actor {
            tag: tag,
            x_y: x_y,
            w_h: w_h,
            facing: facing,
            vel: vel,
            ang_vel: ang_vel,
            life: life,
            collision_timeout: collision_timeout,
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
}

const MAX_PHYSICS_VEL: f32 = 150.0;

pub fn update_actor_position(actor: &mut Actor, dt: f32) {
    // Clamp the velocity to the max efficiently
    let vel_norm = actor.vel.norm();
    if vel_norm > MAX_PHYSICS_VEL {
        actor.vel = actor.vel / vel_norm * MAX_PHYSICS_VEL;
    }
    if actor.get_tag() != ActorType::Player {
        rotate_actor_position(actor)
    }
    let dv = actor.vel * dt;
    actor.x_y += dv;
    actor.facing += actor.ang_vel;
}

fn rotate_actor_position(actor: &mut Actor) {
    let vel_norm = actor.vel.norm();
    let actor_unit_vel = actor.vel / vel_norm;
    let vel_after = actor_unit_vel + vec_from_angle(actor.facing);
    actor.vel = vel_after * vel_norm;
}

pub fn wrap_actor_position(actor: &mut Actor, screen_w_h: Vector2) {
    // Wrap screen
    let sx = screen_w_h.0;
    let sy = screen_w_h.1;
    let screen_x_bounds = sx / 2.0;
    let screen_y_bounds = sy / 2.0;
    if actor.x_y.0 > screen_x_bounds {
        actor.x_y -= Vector2(sx, 0.0);
    } else if actor.x_y.0 < -screen_x_bounds {
        actor.x_y += Vector2(sx, 0.0);
    };
    if actor.x_y.1 > screen_y_bounds {
        actor.x_y -= Vector2(0.0, sy);
    } else if actor.x_y.1 < -screen_y_bounds {
        actor.x_y += Vector2(0.0, sy);
    }
}

pub fn inside_window(actor: &Actor, screen_w_h: Vector2) -> bool {
    let sx = screen_w_h.0;
    let sy = screen_w_h.1;
    let screen_x_bounds = sx / 2.0;
    let screen_y_bounds = sy / 2.0;
    actor.x_y.0.abs() < screen_x_bounds && actor.x_y.1.abs() < screen_y_bounds
}

// Next ////////////////

// Create Player //////////////////
const PLAYER_LIFE: i32 = 10;
const PLAYER_WIDTH: f32 = 12.0;
const PLAYER_HEIGHT: f32 = 12.0;

pub fn create_player() -> Actor {
    Actor {
        tag: ActorType::Player,
        x_y: Vector2(0.0, -300.0),
        w_h: Vector2(PLAYER_WIDTH, PLAYER_HEIGHT),
        facing: 0.0,
        vel: Vector2(0.0, 0.0),
        ang_vel: 0.0,
        life: PLAYER_LIFE,
        collision_timeout: 0.0,
    }
}

const MAX_BULLET_VEL: f32 = 50.0;

pub fn create_rand_bullets(rng: &mut Rand32, x_y: Vector2, num: i32) -> Vec<Actor> {
    let new_bullet = |_| {
        let r_angle = rng.rand_float() * 2.0 * std::f32::consts::PI;
        let r_distance = rng.rand_float();
        let bullet = create_bullet(
            x_y + vec_from_angle(r_angle) * r_distance,
            Vector2(5.0, 5.0),
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
            Vector2(10.0, 15.0),
            r_angle,
            vec_from_angle(r_angle) * vel_norm,
            ang_vel,
        );
        bullet
    };

    (0..num).map(new_bullet).collect()
}

pub fn create_bullet(x_y: Vector2, w_h: Vector2, facing: f32, vel: Vector2, ang_vel: f32) -> Actor {
    const BULLET_LIFE: i32 = 1000;
    Actor {
        tag: ActorType::Bullet,
        x_y: x_y,
        w_h: w_h,
        facing: facing,
        vel: vel,
        ang_vel: ang_vel,
        life: BULLET_LIFE,
        collision_timeout: 0.0,
    }
}

pub fn handle_actor_collision(actor: &mut Actor, bullet: Actor) {
    const COLLISION_TIMEOUT: f32 = 0.5;
    let player_size = actor.w_h.norm() / 2.0;
    let pdistance = (bullet.x_y - actor.x_y).norm();
    let bullet_size = bullet.w_h.norm();
    if pdistance < player_size + bullet_size && actor.get_collision_timeout() < 0.0 {
        actor.set_collision_timeout(COLLISION_TIMEOUT);
        actor.dec_life(1);
    }
}
