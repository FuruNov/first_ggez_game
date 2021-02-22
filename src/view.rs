use ggez;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

use crate::actor_mods::actor::Actor;
use crate::assets::Assets;
use crate::vector2::Vector2;

pub fn draw_text(
    ctx: &mut Context,
    text: String,
    x_y: Vector2,
    size: f32,
    font: graphics::Font,
    world_coords: (f32, f32),
) -> GameResult<()> {
    let x_y = world_to_screen_coords(world_coords, x_y);
    let x_y = na::Point2::new(x_y.0, x_y.1);
    let text = graphics::Text::new((text, font, size));
    let drawparams = graphics::DrawParam::new()
        .dest(x_y)
        .offset(na::Point2::new(0.5, 0.5));

    graphics::draw(ctx, &text, drawparams)?;
    Ok(())
}

pub fn draw_actor(
    ctx: &mut Context,
    actor: &Actor,
    assets: &Assets,
    world_coords: (f32, f32),
) -> GameResult<()> {
    let x_y = world_to_screen_coords(world_coords, actor.get_x_y());
    let x_y = na::Point2::new(x_y.0, x_y.1);
    let image = assets.actor_image(actor);
    let drawparams = graphics::DrawParam::new()
        .dest(x_y)
        .rotation(actor.get_facing() as f32)
        .offset(na::Point2::new(0.5, 0.5));

    graphics::draw(ctx, image, drawparams)?;
    Ok(())
}

pub fn draw_collision(
    ctx: &mut Context,
    actor: &Actor,
    color: graphics::Color,
    world_coords: (f32, f32),
) -> GameResult<()> {
    let w_h = actor.get_w_h();
    let x_y = world_to_screen_coords(world_coords, actor.get_x_y());
    let x_y = na::Point2::new(x_y.0, x_y.1);
    let rect = graphics::Mesh::new_circle(
        ctx,
        graphics::DrawMode::stroke(2.0),
        na::Point2::new(0.0, 0.0),
        w_h.norm(),
        1.0,
        color,
    )?;
    let drawparams = graphics::DrawParam::new()
        .dest(x_y)
        .rotation(actor.get_facing() as f32)
        .offset(na::Point2::new(0.5, 0.5));

    graphics::draw(ctx, &rect, drawparams)?;
    Ok(())
}

fn world_to_screen_coords(screen_w_h: (f32, f32), point: Vector2) -> Vector2 {
    let x = point.0 + screen_w_h.0 / 2.0;
    let y = screen_w_h.1 - (point.1 + screen_w_h.1 / 2.0);
    Vector2(x, y)
}
