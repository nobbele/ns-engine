use ggez::{
    graphics::{self, Color},
    mint as na, Context,
};

use crate::{
    helpers::{get_item_y, points_to_rect, Position},
    Character, MainState, Placement, Resources,
};

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
        /*let layer = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            bounds,
            Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 0.25,
            },
        )?;
        graphics::draw(ctx, &layer, (na::Point2::new(0.0, 0.0),))?;*/
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
    choices: &[String],
    hovered_choice: u32,
) -> ggez::GameResult {
    for (n, choice) in choices.iter().enumerate() {
        let layer_bounds = {
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
            graphics::draw(ctx, &layer, ([0.0, 0.0],))?;
            bounds
        };
        let mut text = graphics::Text::new(choice.as_str());
        text.set_bounds(
            [layer_bounds.w - 10.0, layer_bounds.h - 10.0],
            graphics::Align::Center,
        );

        let text_pos = Position::TopLeft.add_in_from(
            &layer_bounds,
            (0.0, layer_bounds.h / 2.0 - text.height(ctx) as f32 / 2.0),
        );
        graphics::draw(ctx, &text, (text_pos,))?;
    }
    Ok(())
}

pub fn draw_node(state: &mut MainState, ctx: &mut Context) -> ggez::GameResult {
    if let Some(node) = &state.current_node {
        if let novelscript::SceneNodeData::Text { speaker, content } = node {
            draw_text(ctx, &state.resources, speaker, content)?;
        } else if let novelscript::SceneNodeData::Choice(choices) = node {
            draw_choices(ctx, choices, state.hovered_choice)?;
        } else if let novelscript::SceneNodeData::LoadCharacter {
            character,
            expression,
            placement,
        } = node
        {
            let entry = Character {
                _name: character.clone(),
                image: graphics::Image::new(
                    ctx,
                    format!("/char/{}/{}.png", character, expression),
                )?,
                position: match &placement[..] {
                    "Left" => Some(Placement::Left),
                    "Right" => Some(Placement::Right),
                    _ => None,
                },
            };
            state.current_characters.insert(
                match entry.position {
                    Some(Placement::Left) => 0,
                    Some(Placement::Right) => state.current_characters.len(),
                    None => 0,
                },
                entry,
            );
            state.continue_text();
        } else if let novelscript::SceneNodeData::LoadBackground { name } = node {
            state.current_background =
                Some(graphics::Image::new(ctx, format!("/bg/{}.png", name))?);
            state.continue_text();
            draw_node(state, ctx)?;
        }
    }
    Ok(())
}
