use ggez;
use ggez::audio;
use ggez::graphics;
use ggez::{Context, GameResult};

use crate::actor::{Actor, ActorType};

struct Assets {
    player_image: graphics::Image,
    shot_image: graphics::Image,
    other_image: graphics::Image,
    font: graphics::Font,
    shot_sound: audio::Source,
    hit_sound: audio::Source,
}

impl Assets {
    fn new(ctx: &mut Context) -> GameResult<Assets> {
        let player_image = graphics::Image::new(ctx, "/player.png")?;
        let shot_image = graphics::Image::new(ctx, "/shot.png")?;
        let other_image = graphics::Image::new(ctx, "/other.png")?;
        let font = graphics::Font::new(ctx, "/LiberationMono-Regular.ttf")?;
        let shot_sound = audio::Source::new(ctx, "/pew.ogg")?;
        let hit_sound = audio::Source::new(ctx, "/boom.ogg")?;

        Ok(Assets {
            player_image,
            shot_image,
            other_image,
            font,
            shot_sound,
            hit_sound,
        })
    }

    fn actor_image(&mut self, actor: &Actor) -> &mut graphics::Image {
        match actor.get_tag() {
            ActorType::Player => &mut self.player_image,
            ActorType::Bullet => &mut self.shot_image,
            ActorType::Other => &mut self.other_image,
        }
    }
}
