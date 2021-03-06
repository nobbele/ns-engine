use ggez::{
    graphics::{self, Drawable},
    Context, GameError, GameResult,
};
use graphics::{DrawParam, Text};

use crate::helpers::Position;

use super::StateEventHandler;

pub struct ErrorState {
    pub bg: graphics::Image,
    pub text: Text,
}

impl ErrorState {
    pub fn new(ctx: &mut Context, e: GameError) -> Self {
        Self {
            bg: graphics::Image::new(ctx, "/Error.jpg").unwrap(),
            text: Text::new(e.to_string()),
        }
    }
}

impl StateEventHandler for ErrorState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, param: DrawParam) -> GameResult {
        param.scale(ggez::mint::Vector2 {
            x: crate::helpers::target_size().x / self.bg.width() as f32,
            y: crate::helpers::target_size().x / self.bg.height() as f32,
        });
        self.bg.draw(ctx, param).unwrap();
        self.text
            .draw(
                ctx,
                param
                    .dest(Position::Center.add_in(
                        ctx,
                        glam::Vec2::new(self.text.width(ctx) as f32 / -2.0, 0.0),
                    ))
                    .color(graphics::RED),
            )
            .unwrap();
        Ok(())
    }
}
