use ggez;

use ggez::event::{self, EventHandler, KeyCode, KeyMods, MouseButton};
use ggez::graphics;
use ggez::timer;
use ggez::{Context, GameResult};

use csv::Reader;
use oorandom::Rand32;
use std::error::Error;
use std::str::FromStr;

use crate::actor_mods::actor::*;
use crate::actor_mods::actor_state::*;
use crate::assets::Assets;
use crate::draw::draw_text;
use crate::imgui_wrapper::ImGuiWrapper;
use crate::input::*;
use crate::vector2::Vector2;

// TODO #4
pub struct MainScene {
    player_state: (ActorState, Vec<ActorState>), // (親機, 子機) TODO #5
    enemies_state: Vec<ActorState>,
    imgui_wrapper: ImGuiWrapper,
    assets: Assets,
    screen_w_h: Vector2,
    hidpi_factor: f32,
    input: InputState,
    _rng: Rand32,
}

impl MainScene {
    pub fn new(ctx: &mut Context, hidpi_factor: f32) -> GameResult<MainScene> {
        let seed: [u8; 8] = [0; 8];
        let mut _rng = Rand32::new(u64::from_ne_bytes(seed));

        let state = MainScene {
            player_state: (ActorState::new(create_player()), Vec::new()),
            enemies_state: Vec::new(),
            assets: Assets::new(ctx)?.load(ctx).unwrap(),
            screen_w_h: Vector2(
                graphics::drawable_size(ctx).0,
                graphics::drawable_size(ctx).1,
            ),
            input: InputState::default(),
            _rng: _rng,
            imgui_wrapper: ImGuiWrapper::new(ctx),
            hidpi_factor: hidpi_factor,
        };

        Ok(state)
    }

    pub fn load_data(&mut self) -> GameResult {
        if let Err(err) = self.load_enemy_data() {
            println!("{}", err);
        // process::exit(1);
        } else {
            println!("success loading!!");
        }
        Ok(())
    }

    fn load_enemy_data(&mut self) -> Result<(), Box<dyn Error>> {
        // ステージ制を導入する際は、CSVファイルの読み分けが必要
        let mut rdr = Reader::from_path("./data/enemy_state.csv")?;
        for result in rdr.records() {
            let record = result?;
            // デシリアライズの方法が不明なため、要素を逐一代入
            let enemy = Actor::new(
                ActorType::from_str(&record[0])?,
                Vector2(record[1].parse()?, record[2].parse()?),
                Vector2(record[3].parse()?, record[4].parse()?),
                record[5].parse()?,
                Vector2(record[6].parse()?, record[7].parse()?),
                record[8].parse()?,
                record[9].parse()?,
                record[10].parse()?,
            );
            let enemy_state = ActorState::new(enemy);
            // println!("{:#?}", enemy_state);
            self.enemies_state.push(enemy_state);
        }
        Ok(())
    }

    fn clear_dead_stuff(&mut self) {
        self.player_state.0.clear_dead_stuff(self.screen_w_h);
        for enemy_state in &mut self.enemies_state {
            enemy_state.clear_dead_stuff(self.screen_w_h);
        }
        self.enemies_state
            .retain(|es| es.get_actor().get_life() > 0)
    }

    fn handle_collisions(&mut self, _ctx: &Context) {
        for enemy_state in &mut self.enemies_state {
            for shot in enemy_state.get_shots() {
                self.player_state
                    .0
                    .get_mut_actor()
                    .handle_actor_collision(*shot)
            }
            for shot in self.player_state.0.get_shots() {
                enemy_state.get_mut_actor().handle_actor_collision(*shot)
            }
        }
    }

    fn draw_debug_status(&self, ctx: &mut Context) -> GameResult {
        let text_pos = Vector2(10.0, 10.0);
        let font_size = 24.0;
        let all_shot_num = &mut self.player_state.0.get_shots().len();
        for enemy_state in &self.enemies_state {
            *all_shot_num += enemy_state.get_shots().len();
        }
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
                all_shot_num,
                self.player_state.0.get_actor(),
            ),
            text_pos,
            font_size,
            self.assets.get_font(),
            (0.0, 0.0),
        )
    }
}

impl EventHandler for MainScene {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;
        while timer::check_update_time(ctx, DESIRED_FPS) {
            let seconds = 1.0 / (DESIRED_FPS as f32);
            let _time_since_start: f32 = timer::time_since_start(ctx).as_secs_f32();

            {
                let player_state = &mut self.player_state.0;
                player_state.handle_input(&self.input, seconds);
                if self.input.get_fire() && player_state.get_shot_timeout() < 0.0 {
                    player_state.fire_shot(ctx);
                }
                player_state.update(seconds, self.screen_w_h);

                if self.player_state.0.get_actor().get_life() <= 0 {
                    println!("Game over!!");
                    event::quit(ctx);
                }
            }

            for enemy_state in &mut self.enemies_state {
                if enemy_state.get_shot_timeout() < 0.0 {
                    enemy_state.fire_shot(ctx);
                }
                enemy_state.update(seconds, self.screen_w_h);
            }

            self.handle_collisions(ctx);
            self.clear_dead_stuff();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        self.draw_debug_status(ctx)?;

        let assets = &self.assets;
        let coords = (self.screen_w_h.0, self.screen_w_h.1);

        &self.player_state.0.draw(ctx, assets, coords)?;

        for enemy_state in &self.enemies_state {
            enemy_state.draw(ctx, assets, coords)?;
        }

        // Render game ui
        self.imgui_wrapper.render(ctx, self.hidpi_factor);

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
            KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => {
                self.input.move_actor(keycode);
            }
            KeyCode::Z => {
                self.input.fire_shot(keycode);
            }
            /* // TODO #6 低速移動の実装
            KeyCode::LShift | KeyCode::RShift => {
                self.input.xaxis /= 2.0;
                self.input.yaxis /= 2.0;
            }
            */
            KeyCode::Escape => event::quit(ctx),
            _ => (), // Do nothing
        }

        self.imgui_wrapper.update_key_down(keycode, keymods);
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, keymods: KeyMods) {
        match keycode {
            KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => {
                self.input.stop_actor(keycode);
            }
            KeyCode::Z => {
                self.input.stop_shot(keycode);
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
