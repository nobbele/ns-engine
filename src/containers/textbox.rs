use ggez::{
    graphics::{self, DrawParam, Drawable},
    Context,
};

pub struct TextBox {
    pub layer: (&'static graphics::Image, DrawParam),
    pub speaker: Option<(graphics::Text, DrawParam)>,
    pub content: (graphics::Text, DrawParam),
}

impl TextBox {
    pub fn draw(&self, ctx: &mut Context) -> ggez::GameResult {
        self.layer.0.draw(ctx, self.layer.1)?;
        if let Some(speaker) = &self.speaker {
            speaker.0.draw(ctx, speaker.1)?;
        }
        self.content.0.draw(ctx, self.content.1)?;
        Ok(())
    }
}
