use ggez::graphics::{DrawParam, Drawable};

use derive_new::new;

#[derive(Debug, new)]
pub struct Sprite<T> {
    pub content: T,
    #[new(default)]
    pub param: DrawParam,
}

impl<T: Drawable> Drawable for Sprite<T> {
    fn draw(&self, ctx: &mut ggez::Context, _param: DrawParam) -> ggez::GameResult {
        // TODO param mixing
        self.content.draw(ctx, self.param)
    }
}
