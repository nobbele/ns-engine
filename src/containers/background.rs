use ggez::{Context, graphics::{self, DrawParam, drawable_size}};

use crate::{states::game::Background, tween::TransitionTweenBox};

use super::Draw;

pub struct BackgroundContainer {
    pub current: TransitionTweenBox<Background>,
}

impl Draw for BackgroundContainer {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> ggez::GameResult {
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
                        a: *fade * param.color.a,
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
                    a: *fade * param.color.a,
                    ..graphics::WHITE
                },
                ..Default::default()
            },
        )?;
        Ok(())
    }
}
