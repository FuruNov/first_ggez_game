use ggez;
use ggez::audio;
use ggez::graphics;
use ggez::{Context, GameResult};

use csv::Reader;
use maplit::hashmap;
use std::collections::HashMap;
use std::error::Error;

use crate::actor_mods::actor::{Actor, ActorType};

#[derive(Debug)]
pub struct Assets {
    images: HashMap<&'static str, graphics::Image>,
    sounds: HashMap<&'static str, audio::Source>,
    fonts: HashMap<&'static str, graphics::Font>,
    // shot_sound: audio::Source,
    // hit_sound: audio::Source,
}

impl Assets {
    pub fn new(_ctx: &mut Context) -> GameResult<Assets> {
        Ok(Assets {
            images: hashmap!(),
            sounds: hashmap!(),
            fonts: hashmap!(),
        })
    }

    pub fn load(&mut self, ctx: &mut Context) -> Result<Assets, Box<dyn Error>> {
        let player_image = graphics::Image::new(ctx, "/player.png")?;
        let enemy_image = graphics::Image::new(ctx, "/enemy.png")?;
        let shot_image = graphics::Image::new(ctx, "/shot.png")?;

        let normal_font = graphics::Font::new(ctx, "/LiberationMono-Regular.ttf")?;

        // let shot_sound = audio::Source::new(ctx, "/pew.ogg")?;
        // let hit_sound = audio::Source::new(ctx, "/boom.ogg")?;

        let images = hashmap!(
            "player" => player_image,
            "enemy" => enemy_image,
            "shot" => shot_image,
        );

        // csv から名前と画像を読み出して格納する
        // ＊record がダングリング(?)を引き起こしている模様

        // let mut rdr = Reader::from_path("./data/enemy_state.csv")?;
        // for result in rdr.records() {
        //     let record = result?;
        //     // デシリアライズの方法が不明なため、要素を逐一代入
        //     let path = &record[1];
        //     let image = graphics::Image::new(ctx, &record[0])?;
        //     // println!("{:#?}", enemy_state);
        //     self.images.insert(path, image);
        // }

        // let sounds = ... ;

        let fonts = hashmap!(
            "normal" => normal_font,
        );

        Ok(Assets {
            images: images,
            sounds: hashmap!(), // ← sounds
            fonts: fonts,
        })
    }

    pub fn get_font(&self) -> graphics::Font {
        self.fonts["normal"]
    }

    pub fn actor_image(&self, actor: Actor) -> &graphics::Image {
        match actor.get_tag() {
            ActorType::Player => &self.images["player"],
            ActorType::Enemy => &self.images["enemy"],
            ActorType::Bullet => &self.images["shot"],
            ActorType::Other => &self.images["player"],
        }
    }
}
