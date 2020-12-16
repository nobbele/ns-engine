use ggez::graphics::{DrawParam, Drawable};

use crate::tween::TweenBox;

pub struct Sprite<T> {
    pub content: T,
    pub param: DrawParam,
}

impl<T: Drawable> Drawable for Sprite<T> {
    fn draw(&self, ctx: &mut ggez::Context, _param: DrawParam) -> ggez::GameResult {
        // TODO param mixing
        self.content.draw(ctx, self.param)
    }
}

impl<T: Drawable> Drawable for Sprite<TweenBox<T>> {
    fn draw(&self, ctx: &mut ggez::Context, _param: DrawParam) -> ggez::GameResult {
        // TODO param mixing
        let current = self.content.get_current();
        current.draw(ctx, self.param)
    }
}
