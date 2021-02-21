use ggez;
use ggez::audio;
use ggez::graphics;
use ggez::{Context, GameResult};

use crate::actor::{Actor, ActorType};

#[derive(Debug)]
pub struct Assets {
    player_image: graphics::Image,
    enemy_image: graphics::Image,
    shot_image: graphics::Image,
    font: graphics::Font,
    // shot_sound: audio::Source,
    // hit_sound: audio::Source,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let player_image = graphics::Image::new(ctx, "/player.png")?;
        let enemy_image = graphics::Image::new(ctx, "/enemy.png")?;
        let shot_image = graphics::Image::new(ctx, "/shot.png")?;

        let font = graphics::Font::new(ctx, "/LiberationMono-Regular.ttf")?;
        // let shot_sound = audio::Source::new(ctx, "/pew.ogg")?;
        // let hit_sound = audio::Source::new(ctx, "/boom.ogg")?;

        Ok(Assets {
            player_image: player_image,
            enemy_image: enemy_image,
            shot_image: shot_image,
            font: font,
        })
    }

    pub fn get_font(&self) -> graphics::Font {
        self.font
    }

    pub fn actor_image(&mut self, actor: &Actor) -> &mut graphics::Image {
        match actor.get_tag() {
            ActorType::Player => &mut self.player_image,
            ActorType::Bullet => &mut self.shot_image,
            ActorType::Enemy => &mut self.enemy_image,
            ActorType::Other => &mut self.shot_image,
        }
    }
}
