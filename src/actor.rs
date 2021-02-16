use ggez::nalgebra as na;
use oorandom::Rand32;

use crate::game_state::InputState;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ActorType {
    Player,
    Bullet,
    Other,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Actor {
    tag: ActorType,
    x_y: na::Vector2<f32>,
    w_h: (f32, f32),
    facing: f32,
    velocity: na::Vector2<f32>,
    ang_vel: f32,

    // I am going to lazily overload "life" with a
    // double meaning:
    // for shots, it is the time left to live,
    // for players and rocks, it is the actual hit points.
    life: i32,
}

impl Actor {
    pub fn get_tag(&self) -> ActorType {
        self.tag
    }

    pub fn get_x_y(&self) -> na::Vector2<f32> {
        self.x_y
    }

    pub fn get_w_h(&self) -> (f32, f32) {
        self.w_h
    }

    pub fn get_facing(&self) -> f32 {
        self.facing
    }

    pub fn get_life(&self) -> i32 {
        self.life
    }

    pub fn dec_life(&mut self, amount: i32) {
        // 当たり判定の確認
        if true
        /* 実際のプレイでは、ライフは負に成り得ないので self.life > 0 */
        {
            self.life -= amount;
        }
    }
}

// For Actor Position ////////////////
const MAX_PHYSICS_VEL: f32 = 150.0;

pub fn update_actor_position(actor: &mut Actor, dt: f32) {
    // Clamp the velocity to the max efficiently
    let vel_norm = actor.velocity.norm();
    if vel_norm > MAX_PHYSICS_VEL {
        actor.velocity = actor.velocity / vel_norm * MAX_PHYSICS_VEL;
    }
    if actor.get_tag() == ActorType::Bullet {
        rotate_actor_position(actor)
    }
    let dv = actor.velocity * dt;
    actor.x_y += dv;
    actor.facing += actor.ang_vel;
}

fn rotate_actor_position(actor: &mut Actor) {
    let vel_norm = actor.velocity.norm();
    let actor_unit_vel = actor.velocity / vel_norm;
    actor.velocity = vel_norm * (actor_unit_vel + vec_from_angle(actor.facing));
}

pub fn wrap_actor_position(actor: &mut Actor, screen_w_h: (f32, f32)) {
    // Wrap screen
    let sx = screen_w_h.0;
    let sy = screen_w_h.1;
    let screen_x_bounds = sx / 2.0;
    let screen_y_bounds = sy / 2.0;
    if actor.x_y.x > screen_x_bounds {
        actor.x_y -= na::Vector2::new(sx, 0.0);
    } else if actor.x_y.x < -screen_x_bounds {
        actor.x_y += na::Vector2::new(sx, 0.0);
    };
    if actor.x_y.y > screen_y_bounds {
        actor.x_y -= na::Vector2::new(0.0, sy);
    } else if actor.x_y.y < -screen_y_bounds {
        actor.x_y += na::Vector2::new(0.0, sy);
    }
}

pub fn inside_window(actor: &Actor, screen_w_h: (f32, f32)) -> bool {
    let sx = screen_w_h.0;
    let sy = screen_w_h.1;
    let screen_x_bounds = sx / 2.0;
    let screen_y_bounds = sy / 2.0;
    actor.x_y.x.abs() < screen_x_bounds && actor.x_y.y.abs() < screen_y_bounds
}

// Next ////////////////

// Create Player //////////////////
const PLAYER_LIFE: i32 = 10;
const PLAYER_WIDTH: f32 = 12.0;
const PLAYER_HEIGHT: f32 = 12.0;

pub fn create_player() -> Actor {
    Actor {
        tag: ActorType::Player,
        x_y: na::Vector2::new(0.0, -300.0),
        w_h: (PLAYER_WIDTH, PLAYER_HEIGHT),
        facing: 0.0,
        velocity: na::Vector2::new(0.0, 0.0),
        ang_vel: 0.0,
        life: PLAYER_LIFE,
    }
}

// Next ///////////////////////////

// Handle Player //////////////////
const PLAYER_VEL: f32 = 4.0;

pub fn player_handle_input(actor: &mut Actor, input: &InputState, dt: f32) {
    actor.velocity =
        dt * 10.0_f32.powf(PLAYER_VEL) * na::Vector2::new(input.get_xaxis(), input.get_yaxis());
}

// Next ///////////////////

// Create Bullets /////////////////////
const MAX_BULLET_VEL: f32 = 50.0;

pub fn create_rand_bullets(rng: &mut Rand32, x_y: na::Vector2<f32>, num: i32) -> Vec<Actor> {
    let new_bullet = |_| {
        let r_angle = rng.rand_float() * 2.0 * std::f32::consts::PI;
        let r_distance = rng.rand_float();
        let bullet = create_bullet(
            x_y + vec_from_angle(r_angle) * r_distance,
            (5.0, 5.0),
            r_angle,
            random_vec(rng, MAX_BULLET_VEL),
            0.0,
        );
        bullet
    };
    (0..num).map(new_bullet).collect()
}

pub fn create_circle_bullets(
    x_y: na::Vector2<f32>,
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
            (10.0, 15.0),
            r_angle,
            vel_norm * vec_from_angle(r_angle),
            ang_vel,
        );
        bullet
    };

    (0..num).map(new_bullet).collect()
}

const BULLET_LIFE: i32 = 1000;

pub fn create_bullet(
    x_y: na::Vector2<f32>,
    w_h: (f32, f32),
    facing: f32,
    velocity: na::Vector2<f32>,
    ang_vel: f32,
) -> Actor {
    Actor {
        tag: ActorType::Bullet,
        x_y: x_y,
        w_h: w_h,
        facing: facing,
        velocity: velocity,
        ang_vel: ang_vel,
        life: BULLET_LIFE,
    }
}

fn random_vec(rng: &mut Rand32, max_magnitude: f32) -> na::Vector2<f32> {
    let angle = rng.rand_float() * 2.0 * std::f32::consts::PI;
    let mag = rng.rand_float() * max_magnitude;
    vec_from_angle(angle) * (mag)
}

fn vec_from_angle(angle: f32) -> na::Vector2<f32> {
    let vx = angle.sin();
    let vy = angle.cos();
    na::Vector2::<f32>::new(vx, vy)
}

// Next /////////////////////
