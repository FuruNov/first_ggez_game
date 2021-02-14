use ggez;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

use crate::actor::Actor;

pub fn draw_text(
    ctx: &mut Context,
    text: String,
    x_y: na::Vector2<f32>,
    size: f32,
    font: graphics::Font,
) -> GameResult<()> {
    let text = graphics::Text::new((text, font, size));
    let dest_point = na::Point2::new(x_y.x, x_y.y);
    graphics::draw(ctx, &text, (dest_point,))?;
    Ok(())
}

pub fn draw_actor(
    ctx: &mut Context,
    actor: &Actor,
    color: graphics::Color,
    world_coords: (f32, f32),
) -> GameResult<()> {
    let x_y = world_to_screen_coords(world_coords, actor.get_x_y());
    let rect = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        graphics::Rect {
            x: 0.0,
            y: 0.0,
            w: actor.get_w_h().0,
            h: actor.get_w_h().1,
        },
        color,
    )?;

    let drawparams = graphics::DrawParam::new()
        .dest(na::Point2::new(
            x_y.x + actor.get_w_h().0 / 2.0,
            x_y.y + actor.get_w_h().1 / 2.0,
        ))
        .rotation(actor.get_facing() as f32);

    graphics::draw(ctx, &rect, drawparams)?;
    Ok(())
}

fn world_to_screen_coords(screen_w_h: (f32, f32), point: na::Vector2<f32>) -> na::Vector2<f32> {
    let x = point.x + screen_w_h.0 / 2.0;
    let y = screen_w_h.1 - (point.y + screen_w_h.1 / 2.0);
    na::Vector2::new(x, y)
}
