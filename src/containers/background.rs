use ggez::{
    graphics::{self, drawable_size, DrawParam, Drawable},
    mint, Context,
};

use crate::{states::game::Background, tween::TransitionTweenBox};

pub struct BackgroundContainer {
    pub current: TransitionTweenBox<Background>,
}

impl Drawable for BackgroundContainer {
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
                graphics::DrawParam::new()
                    .scale(mint::Vector2 {
                        x: drawable_size(ctx).0 / image.width() as f32,
                        y: drawable_size(ctx).1 / image.height() as f32,
                    })
                    .color(graphics::Color {
                        a: *fade * param.color.a,
                        ..graphics::WHITE
                    }),
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
            graphics::DrawParam::new()
                .scale(mint::Vector2 {
                    x: drawable_size(ctx).0 / image.width() as f32,
                    y: drawable_size(ctx).1 / image.height() as f32,
                })
                .color(graphics::Color {
                    a: *fade * param.color.a,
                    ..graphics::WHITE
                }),
        )?;
        Ok(())
    }
}
