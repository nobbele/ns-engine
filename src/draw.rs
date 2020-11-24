use ggez::{Context, graphics::{self, Color, Drawable}, mint as na};
use crate::{Resources, containers::{panel::Panel, screen::Screen}, helpers::{get_item_y, points_to_rect, Position}};

pub fn draw_text(
    ctx: &mut Context,
    resources: &Resources,
    speaker: &Option<String>,
    content: &str,
) -> ggez::GameResult {
    let layer_bounds = {
        let bounds = points_to_rect(
            Position::BottomLeft.add_in(ctx, (0.0, 200.0)),
            Position::BottomRight.add_in(ctx, (0.0, 0.0)),
        );
        let image = &resources.text_box;
        graphics::draw(
            ctx,
            image,
            graphics::DrawParam::new()
                .dest([bounds.x, bounds.y])
                .scale([
                    bounds.w / image.width() as f32,
                    bounds.h / image.height() as f32,
                ]),
        )?;
        bounds
    };
    if let Some(speaker) = speaker {
        let mut text = graphics::Text::new(speaker.as_str());
        text.set_bounds([f32::INFINITY, f32::INFINITY], graphics::Align::Left);
        graphics::draw(
            ctx,
            &text,
            (Position::TopLeft.add_in_from(&layer_bounds, (15.0, 20.0)),),
        )?;
    }
    let mut text = graphics::Text::new(content);
    text.set_bounds(
        [layer_bounds.w - 10.0, layer_bounds.h - 10.0],
        graphics::Align::Left,
    );
    graphics::draw(
        ctx,
        &text,
        (Position::TopLeft.add_in_from(&layer_bounds, (15.0, 55.0)),),
    )?;
    Ok(())
}

pub fn draw_choices(
    ctx: &mut Context,
    screen: &mut Screen,
    choices: &[String],
    hovered_choice: u32,
) -> ggez::GameResult {
    let mut v = Vec::new();
    for (n, choice) in choices.iter().enumerate() {
        let textbox = {
            let layer = {
                let size = (210.0, 40.0);
                let pos: na::Point2<f32> = [
                    Position::Center.add_in(ctx, (-size.0 / 2.0, 0.0)).x,
                    get_item_y(ctx, n as f32, choices.len() as f32),
                ]
                .into();
                let bounds = points_to_rect(pos, [pos.x + size.0, pos.y + size.1].into());
                let layer = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    bounds,
                    if n == hovered_choice as usize {
                        Color {
                            r: 1.0,
                            g: 1.0,
                            b: 1.0,
                            a: 0.25,
                        }
                    } else {
                        Color {
                            r: 0.25,
                            g: 0.25,
                            b: 0.25,
                            a: 0.25,
                        }
                    },
                )?;
                layer
            };
            let layer_bounds = layer.dimensions(ctx).unwrap();
            let mut text = graphics::Text::new(choice.as_str());
            text.set_bounds(
                [layer_bounds.w - 10.0, layer_bounds.h - 10.0],
                graphics::Align::Center,
            );

            let text_pos = Position::TopLeft.add_in_from(
                &layer_bounds,
                (0.0, layer_bounds.h / 2.0 - text.height(ctx) as f32 / 2.0),
            );
            Panel {
                layer,
                content: (text, (text_pos,).into()),
            }
        };
        v.push(textbox);
    }
    screen.choices = Some(v);
    Ok(())
}
