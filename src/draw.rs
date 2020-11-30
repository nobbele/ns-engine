use crate::{
    containers::{
        screen::{Action, Screen},
        textbox::TextBox,
    },
    helpers::{points_to_rect, Position},
    tween::Tweener,
    Resources,
};
use ggez::{
    graphics,
    Context,
};

pub fn load_text(
    ctx: &mut Context,
    screen: &mut Screen,
    resources: &'static Resources,
    speaker: &Option<String>,
    content: &str,
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
        },
    };

    let text_params = (Position::TopLeft.add_in_from(&layer_bounds, (15.0, 55.0)),).into();

    screen.action = Action::Text(Box::new(TextBox {
        layer: (layer_image, layer_params),
        speaker: speaker_text,
        content: (Box::new(text_tween), text_params),
    }));

    Ok(())
}