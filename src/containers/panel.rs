use ggez::{
    graphics::{self, DrawParam, Drawable},
    Context,
};

use super::Draw;

pub struct Panel<T: graphics::Drawable> {
    pub layer: graphics::Mesh,
    pub content: (T, DrawParam),
}

impl<T: graphics::Drawable> Draw for Panel<T> {
    fn draw(&self, ctx: &mut Context) -> ggez::GameResult {
        self.layer.draw(ctx, graphics::DrawParam::new())?;
        self.content.0.draw(ctx, self.content.1)?;
        Ok(())
    }
}
