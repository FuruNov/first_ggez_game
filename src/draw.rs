use ggez;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

use crate::vector2::Vector2;

pub fn draw_text(
    ctx: &mut Context,
    text: String,
    x_y: Vector2,
    size: f32,
    font: graphics::Font,
    world_coords: (f32, f32),
) -> GameResult {
    let x_y = x_y.world_to_screen_coords(world_coords);
    let x_y = na::Point2::new(x_y.0, x_y.1);
    let text = graphics::Text::new((text, font, size));
    let drawparams = graphics::DrawParam::new()
        .dest(x_y)
        .offset(na::Point2::new(0.5, 0.5));

    graphics::draw(ctx, &text, drawparams)?;
    Ok(())
}
