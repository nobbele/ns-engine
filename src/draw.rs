use crate::{Resources, tween::Tweener, containers::{
        panel::Panel,
        screen::{Action, Screen},
        textbox::TextBox,
    }, helpers::{get_item_y, points_to_rect, Position}};
use ggez::{
    graphics::{self, Color, Drawable, Text},
    mint as na, Context,
};

pub fn load_text(
    ctx: &mut Context,
    screen: &mut Screen,
    resources: &'static Resources,
    speaker: &Option<String>,
    content: &str,
) -> ggez::GameResult {
    let layer_bounds = points_to_rect(
        Position::BottomLeft.add_in(ctx, (0.0, 200.0)),
        Position::BottomRight.add_in(ctx, (0.0, 0.0)),
    );
    let layer_image = &resources.text_box;
    let layer_params = graphics::DrawParam::new()
        .dest([layer_bounds.x, layer_bounds.y])
        .scale([
            layer_bounds.w / layer_image.width() as f32,
            layer_bounds.h / layer_image.height() as f32,
        ]);

    let speaker_text = if let Some(speaker) = speaker {
        let mut speaker_text = graphics::Text::new(speaker.as_str());
        speaker_text.set_bounds([f32::INFINITY, f32::INFINITY], graphics::Align::Left);
        let speaker_text_params = (Position::TopLeft.add_in_from(&layer_bounds, (15.0, 20.0)),);
        Some((speaker_text, speaker_text_params.into()))
    } else {
        None
    };

    let mut text = graphics::Text::default();
    for c in content.chars() {
        text.add(graphics::TextFragment::new(c).color(graphics::Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 0.0,
        }));
    }
    text.set_bounds(
        [layer_bounds.w - 10.0, layer_bounds.h - 10.0],
        graphics::Align::Left,
    );

    let text_tween = Tweener {
        current: text,
        time: 0.0,
        update: |text, time, _| {
            let cps = 75.0f32;

            let lim = (time * cps) as usize;
            let lim = if lim > text.fragments().len() {
                text.fragments().len()
            } else {
                lim
            };

            for i in 0..lim {
                if let Some(color) = &mut text.fragments_mut()[i].color {
                    color.a = 1.0;
                }
            }
        }
    };

    let text_params = (Position::TopLeft.add_in_from(&layer_bounds, (15.0, 55.0)),).into();

    screen.action = Action::Text(Box::new(TextBox {
        layer: (layer_image, layer_params),
        speaker: speaker_text,
        content: (Box::new(text_tween), text_params),
    }));

    Ok(())
}

pub fn load_choices_layer(
    ctx: &mut Context,
    n: usize,
    max: usize,
    hovered_choice: u32,
) -> Result<graphics::Mesh, ggez::GameError> {
    let size = (210.0, 40.0);
    let pos: na::Point2<f32> = [
        Position::Center.add_in(ctx, (-size.0 / 2.0, 0.0)).x,
        get_item_y(ctx, n as f32, max as f32),
    ]
    .into();
    let bounds = points_to_rect(pos, [pos.x + size.0, pos.y + size.1].into());
    graphics::Mesh::new_rectangle(
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
    )
}

pub fn load_choices(
    ctx: &mut Context,
    screen: &mut Screen,
    choices: &[String],
    hovered_choice: u32,
) -> ggez::GameResult {
    let mut v = Vec::new();
    for (n, choice) in choices.iter().enumerate() {
        let textbox = {
            let layer = load_choices_layer(ctx, n, choices.len(), hovered_choice)?;
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
    screen.action = Action::Choice(v);
    Ok(())
}

pub fn update_draw_choices(
    ctx: &mut Context,
    choices: &mut Vec<Panel<Text>>,
    hovered_choice: u32,
) -> ggez::GameResult {
    let length = choices.len();
    for (n, choice) in choices.iter_mut().enumerate() {
        choice.layer = load_choices_layer(ctx, n as usize, length, hovered_choice)?;
    }
    Ok(())
}
