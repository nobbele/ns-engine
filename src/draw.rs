use crate::{
    containers::{
        gamescreen::{Action, GameScreen},
        sprite::Sprite,
        textbox::TextBox,
    },
    helpers::{points_to_rect, Position},
    states::game::Config,
    tween::Tweener,
    Resources,
};
use ggez::{
    graphics::{self, DrawParam},
    Context,
};

pub fn load_text(
    ctx: &mut Context,
    screen: &mut GameScreen,
    resources: &'static Resources,
    speaker: &Option<String>,
    content: &str,
    config: &'static Config,
) -> ggez::GameResult {
    let layer_bounds = points_to_rect(
        Position::BottomLeft.add_in(ctx, (0.0, 240.0)),
        Position::BottomRight.add_in(ctx, (0.0, 40.0)),
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
        let speaker_text_params = DrawParam::new()
            .dest(Position::TopLeft.add_in_from(&layer_bounds, (15.0, 20.0)))
            .color(
                config
                    .characters
                    .get(speaker)
                    .copied()
                    .unwrap_or_default()
                    .color,
            );
        Some(Sprite {
            content: speaker_text,
            param: speaker_text_params,
        })
    } else {
        None
    };

    let mut text = graphics::Text::default();
    for c in content.chars() {
        text.add(graphics::TextFragment::new(c).color(graphics::Color {
            a: 0.0,
            ..graphics::WHITE
        }));
    }
    text.set_bounds(
        [layer_bounds.w - 10.0, layer_bounds.h - 10.0],
        graphics::Align::Left,
    );

    let text_tween = Tweener::new(text, |text, time, _| {
        let cps = 75.0f32;

        let frag_count = text.fragments().len();

        let lim = (time * cps) as usize;
        let lim = if lim > frag_count { frag_count } else { lim };

        for i in 0..lim {
            if let Some(color) = &mut text.fragments_mut()[i].color {
                color.a = 1.0;
            }
        }

        lim == frag_count
    });

    let text_params = (Position::TopLeft.add_in_from(&layer_bounds, (15.0, 55.0)),).into();

    screen.action = Action::Text(Box::new(TextBox {
        layer: Sprite {
            content: layer_image,
            param: layer_params,
        },
        speaker: speaker_text,
        content: Sprite {
            content: Box::new(text_tween),
            param: text_params,
        },
    }));

    Ok(())
}
