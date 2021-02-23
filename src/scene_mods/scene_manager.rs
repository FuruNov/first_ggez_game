use crate::scene_mods::main_scene::MainScene;

pub struct SceneManager {
    _scenes: Vec<MainScene>,
} // TODO #4

impl SceneManager {
    pub fn new() -> Self {
        SceneManager {
            _scenes: Vec::new(),
        }
    }
}
