use ggez::{Context, graphics::{self, DrawParam, Drawable}};

pub struct Panel<T: graphics::Drawable> {
    pub layer: graphics::Mesh,
    pub content: (T, DrawParam),
}

impl<T: graphics::Drawable> Panel<T> {
    pub fn draw(&self, ctx: &mut Context) -> ggez::GameResult {
        self.layer.draw(ctx, graphics::DrawParam::new())?;
        self.content.0.draw(ctx, self.content.1)?;
        Ok(())
    }
}