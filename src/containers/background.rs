use ggez::{Context, graphics::{self, drawable_size}};

use crate::{Background, tween::TransitionTweenBox};

pub struct BackgroundContainer {
    pub current: TransitionTweenBox<Background>,
}

impl BackgroundContainer {
    pub fn draw(&self, ctx: &mut Context) -> ggez::GameResult {
        let background = self.current.get_current();
        if let Some(Background {
            name: _,
            fade,
            image,
        }) = &background.0
        {
            graphics::draw(
                ctx,
                image,
                graphics::DrawParam {
                    scale: [
                        drawable_size(ctx).0 / image.width() as f32,
                        drawable_size(ctx).1 / image.height() as f32,
                    ]
                    .into(),
                    color: graphics::Color {
                        a: *fade,
                        ..graphics::WHITE
                    },
                    ..Default::default()
                },
            )?;
        }
        let Background {
            name: _,
            fade,
            image,
        } = &background.1;
        graphics::draw(
            ctx,
            image,
            graphics::DrawParam {
                scale: [
                    drawable_size(ctx).0 / image.width() as f32,
                    drawable_size(ctx).1 / image.height() as f32,
                ]
                .into(),
                color: graphics::Color {
                    a: *fade,
                    ..graphics::WHITE
                },
                ..Default::default()
            },
        )?;
        Ok(())
    }
}