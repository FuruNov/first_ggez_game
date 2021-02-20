use ggez;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

use crate::actor::Actor;
use crate::vector2::Vector2;

pub fn draw_text(
    ctx: &mut Context,
    text: String,
    x_y: Vector2,
    size: f32,
    font: graphics::Font,
) -> GameResult<()> {
    let text = graphics::Text::new((text, font, size));
    let dest_point = na::Point2::new(x_y.0, x_y.1);
    graphics::draw(ctx, &text, (dest_point,))?;
    Ok(())
}

pub fn draw_actor(
    ctx: &mut Context,
    actor: &Actor,
    color: graphics::Color,
    world_coords: (f32, f32),
) -> GameResult<()> {
    let w_h = actor.get_w_h();
    let x_y = world_to_screen_coords(world_coords, actor.get_x_y());
    let rect = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        graphics::Rect {
            x: 0.0,
            y: 0.0,
            w: w_h.0,
            h: w_h.1,
        },
        color,
    )?;

    let drawparams = graphics::DrawParam::new()
        .dest(na::Point2::new(x_y.0, x_y.1))
        .rotation(actor.get_facing() as f32);

    graphics::draw(ctx, &rect, drawparams)?;
    Ok(())
}

fn world_to_screen_coords(screen_w_h: (f32, f32), point: Vector2) -> Vector2 {
    let x = point.0 + screen_w_h.0 / 2.0;
    let y = screen_w_h.1 - (point.1 + screen_w_h.1 / 2.0);
    Vector2(x, y)
}
