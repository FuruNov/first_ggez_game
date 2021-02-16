use ggez;

use ggez::event::{self, EventHandler, KeyCode, KeyMods, MouseButton};
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::timer;
use ggez::{Context, GameResult};

use oorandom::Rand32;

use crate::actor::*;
use crate::imgui_wrapper::ImGuiWrapper;
use crate::view::{draw_actor, draw_text};

// TODO #1 アセットの追加

pub struct MainState {
    player_state: PlayerState,
    bullets: Vec<Actor>,
    enemy_shot_timeout: f32,
    imgui_wrapper: ImGuiWrapper,
    screen_w_h: (f32, f32),
    hidpi_factor: f32,
    input: InputState,
    rng: Rand32,
}

const PLAYER_SHOT_TIME: f32 = 0.5;
const ENEMY_SHOT_TIME: f32 = 0.5;
const SHOT_SPEED: f32 = 50.0;

impl MainState {
    pub fn new(ctx: &mut Context, hidpi_factor: f32) -> GameResult<MainState> {
        let seed: [u8; 8] = [0; 8];
        let mut rng = Rand32::new(u64::from_ne_bytes(seed));
        let state = MainState {
            player_state: PlayerState::new(),
            bullets: Vec::new(),
            screen_w_h: graphics::drawable_size(ctx),
            input: InputState::default(),
            enemy_shot_timeout: 0.0,
            rng: rng,
            imgui_wrapper: ImGuiWrapper::new(ctx),
            hidpi_factor: hidpi_factor,
        };
        Ok(state)
    }

    fn clear_dead_stuff(&mut self, screen_w_h: (f32, f32)) {
        self.player_state
            .shots
            .retain(|b| inside_window(b, screen_w_h) && b.get_life() > 0);
        self.bullets
            .retain(|b| inside_window(b, screen_w_h) && b.get_life() > 0);
    }

    // 当たり判定が bullet の向き（facing）に対してずれていると思われる
    fn handle_collisions(&mut self, _ctx: &Context) {
        let player = &mut self.player_state.actor;
        let player_size = (player.get_w_h().0.powi(2) + player.get_w_h().1.powi(2)).sqrt() / 2.0;
        for bullet in &mut self.bullets {
            let pdistance = (bullet.get_x_y() - player.get_x_y()).norm();
            let bullet_size =
                (bullet.get_w_h().0.powi(2) + bullet.get_w_h().1.powi(2)).sqrt() / 2.0;
            if pdistance < player_size + bullet_size {
                player.dec_life(1);
            }
        }
    }

    fn draw_debug_status(&self, ctx: &mut Context, font: graphics::Font) -> GameResult {
        let text_pos = na::Vector2::new(10.0, 10.0);
        let font_size = 24.0;
        draw_text(
            ctx,
            format!(
                "
FPS: {}\n
time: {}\n
bullet_num: {}\n
Player:\n
{:#?}
                ",
                timer::fps(ctx) as f32,
                timer::time_since_start(ctx).as_secs_f32(),
                self.bullets.len() + self.player_state.shots.len(),
                self.player_state.actor,
            ),
            text_pos,
            font_size,
            font,
        )
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;
        while timer::check_update_time(ctx, DESIRED_FPS) {
            let seconds = 1.0 / (DESIRED_FPS as f32);
            let _time_since_start: f32 = timer::time_since_start(ctx).as_secs_f32();

            {
                let player = &mut self.player_state.actor;
                player_handle_input(player, &self.input, seconds);
                self.player_state.shot_timeout -= seconds;
                update_actor_position(player, seconds);
                wrap_actor_position(player, self.screen_w_h);
            }

            if self.input.fire && self.player_state.shot_timeout < 0.0 {
                self.player_state.fire_player_shot(ctx);
            }

            // fire enemy shot
            self.enemy_shot_timeout -= seconds;
            if self.enemy_shot_timeout < 0.0 {
                self.enemy_shot_timeout = ENEMY_SHOT_TIME;

                // 渦巻弾の発射
                self.bullets.extend(create_circle_bullets(
                    na::Vector2::new(0.0, 0.0),
                    9,
                    (0.0, 1.0),
                    25.0,
                    0.01,
                ));
            }

            for shot in &mut self.player_state.shots {
                update_actor_position(shot, seconds);
                // wrap_actor_position(shot, self.screen_w_h);
                shot.dec_life(1);
            }

            for bullet in &mut self.bullets {
                update_actor_position(bullet, seconds);
                // wrap_actor_position(bullet, self.screen_w_h);
                bullet.dec_life(1);
            }

            self.handle_collisions(ctx);
            self.clear_dead_stuff(self.screen_w_h);

            if self.player_state.actor.get_life() == 0 {
                // println!("Game over!");
                // let _ = event::quit(ctx);
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        let font = graphics::Font::new(ctx, "/LiberationMono-Regular.ttf")?;
        self.draw_debug_status(ctx, font)?;

        let coords = (self.screen_w_h.0, self.screen_w_h.1);

        let player = &self.player_state.actor;
        draw_actor(ctx, player, graphics::WHITE, coords)?;

        for bullet in &self.bullets {
            draw_actor(
                ctx,
                bullet,
                graphics::Color::new(1.0, 1.0, 0.0, 1.0),
                coords,
            )?;
        }

        for shot in &self.player_state.shots {
            draw_actor(ctx, shot, graphics::Color::new(0.0, 1.0, 1.0, 1.0), coords)?;
        }

        // Render game ui
        {
            self.imgui_wrapper.render(ctx, self.hidpi_factor);
        }

        // Then we flip the screen...
        graphics::present(ctx)?;

        // And yield the timeslice
        // This tells the OS that we're done using the CPU but it should
        // get back to this program as soon as it can.
        // This ideally prevents the game from using 100% CPU all the time
        // even if vsync is off.
        // The actual behavior can be a little platform-specific.
        timer::yield_now();
        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.imgui_wrapper.update_mouse_pos(x, y);
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.imgui_wrapper.update_mouse_down(button);
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) {
        self.imgui_wrapper.update_mouse_up(button);
    }

    // Handle key events.  These just map keyboard events
    // and alter our input state appropriately.
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Up => {
                self.input.yaxis = 1.0;
            }
            KeyCode::Down => {
                self.input.yaxis = -1.0;
            }
            KeyCode::Left => {
                self.input.xaxis = -1.0;
            }
            KeyCode::Right => {
                self.input.xaxis = 1.0;
            }
            KeyCode::Z => {
                self.input.fire = true;
            }
            KeyCode::Escape => event::quit(ctx),
            _ => (), // Do nothing
        }
        self.imgui_wrapper.update_key_down(keycode, keymods);
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, keymods: KeyMods) {
        match keycode {
            KeyCode::Up | KeyCode::Down => {
                self.input.yaxis = 0.0;
            }
            KeyCode::Left | KeyCode::Right => {
                self.input.xaxis = 0.0;
            }
            KeyCode::Z => {
                self.input.fire = false;
            }
            _ => (), // Do nothing
        }
        self.imgui_wrapper.update_key_up(keycode, keymods);
    }

    fn text_input_event(&mut self, _ctx: &mut Context, val: char) {
        self.imgui_wrapper.update_text(val);
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, width, height))
            .unwrap();
        // println!("{:?}", graphics::screen_coordinates(ctx));
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, x: f32, y: f32) {
        self.imgui_wrapper.update_scroll(x, y);
    }
}

pub struct InputState {
    xaxis: f32,
    yaxis: f32,
    fire: bool,
}

impl InputState {
    pub fn get_xaxis(&self) -> f32 {
        self.xaxis
    }
    pub fn get_yaxis(&self) -> f32 {
        self.yaxis
    }
    pub fn get_fire(&self) -> bool {
        self.fire
    }
}

impl Default for InputState {
    fn default() -> InputState {
        InputState {
            xaxis: 0.0,
            yaxis: 0.0,
            fire: false,
        }
    }
}

// TODO #2 無敵時間の実装
#[derive(Debug)]
struct PlayerState {
    actor: Actor,
    shots: Vec<Actor>,
    shot_timeout: f32,
}

impl PlayerState {
    fn new() -> PlayerState {
        PlayerState {
            actor: create_player(),
            shots: Vec::new(),
            shot_timeout: 0.0,
        }
    }

    fn fire_player_shot(&mut self, _ctx: &Context) {
        self.shot_timeout = PLAYER_SHOT_TIME;
        let player = &self.actor;
        let shot = create_circle_bullets(
            player.get_x_y() + na::Vector2::new(player.get_w_h().0 / 2.0, player.get_w_h().1 / 2.0),
            5,
            (-2.0 / 5.0, 0.0 / 5.0),
            SHOT_SPEED,
            0.0,
        );

        self.shots.extend(shot);
        // ctx は音声の再生に用いる
        // let _ = self.assets.shot_sound.play(ctx);
    }
}
