extern crate my_first_ggez;

use ggez;
use ggez::conf;
use ggez::event;

use std::env;
use std::path;

use my_first_ggez::game_state::MainState;

pub fn main() -> ggez::GameResult {
    let cb = ggez::ContextBuilder::new("my_first_ggez", "Nov")
        .window_setup(conf::WindowSetup::default().title("super_simple with imgui"))
        .window_mode(
            conf::WindowMode::default()
                .resizable(true)
                .fullscreen_type(conf::FullscreenType::True), /*.dimensions(750.0, 500.0)*/
        )
        .add_resource_path(path::PathBuf::from("./resources"));
    let (ctx, event_loop) = &mut cb.build()?;
    let hidpi_factor = event_loop.get_primary_monitor().get_hidpi_factor() as f32;
    let state = &mut MainState::new(ctx, hidpi_factor)?;
    event::run(ctx, event_loop, state)
}
