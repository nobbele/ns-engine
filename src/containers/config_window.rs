use ggez::graphics::{DrawParam, Drawable, Mesh, Text};

use super::button::Button;

#[derive(Debug, Clone, Copy)]
pub enum ButtonActionId {
    Exit,
}

pub struct ConfigWindow {
    pub panel: Mesh,
    pub exit_button: Button<ButtonActionId>,
    pub text: (Text, DrawParam),
}

impl Drawable for ConfigWindow {
    fn draw(&self, ctx: &mut ggez::Context, param: ggez::graphics::DrawParam) -> ggez::GameResult {
        self.panel.draw(ctx, param).unwrap();
        self.exit_button.draw(ctx, param).unwrap();
        self.text.0.draw(ctx, self.text.1).unwrap();
        Ok(())
    }
}
