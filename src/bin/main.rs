extern crate my_first_ggez;

use ggez;
use ggez::conf;
use ggez::event;

use my_first_ggez::scene_mods::main_scene::MainScene;
use std::path;

pub fn main() -> ggez::GameResult {
    let cb = ggez::ContextBuilder::new("my_first_ggez", "Nov")
        .window_setup(conf::WindowSetup::default().title("first_ggez_game"))
        .window_mode(
            conf::WindowMode::default()
                .resizable(true)
                .fullscreen_type(conf::FullscreenType::True),
        )
        .add_resource_path(path::PathBuf::from("./resources"));
    let (ctx, event_loop) = &mut cb.build()?;
    let hidpi_factor = event_loop.get_primary_monitor().get_hidpi_factor() as f32;
    let state = &mut MainScene::new(ctx, hidpi_factor)?;
    state.load_data()?;
    event::run(ctx, event_loop, state)
}
