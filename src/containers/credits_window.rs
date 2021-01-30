use ggez::graphics::{Drawable, Mesh};

use super::{button::Button, rich_text::RichText, sprite::Sprite};

pub struct CreditsWindow {
    pub panel: Mesh,
    pub text: Sprite<RichText>,
    pub exit_button: Button,
}

impl Drawable for CreditsWindow {
    fn draw(&self, ctx: &mut ggez::Context, param: ggez::graphics::DrawParam) -> ggez::GameResult {
        self.panel.draw(ctx, param)?;
        self.text.draw(ctx, param)?;
        self.exit_button.draw(ctx, param)?;
        Ok(())
    }
}
