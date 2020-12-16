use ggez::graphics::{Drawable, Mesh};

use super::{button::Button, slider::Slider, text_sprite::TextSprite};

#[derive(Debug, Clone, Copy)]
pub enum ButtonActionId {
    Exit,
}

pub struct ConfigWindow {
    pub panel: Mesh,
    pub exit_button: Button<ButtonActionId>,
    pub master_volume_label: TextSprite,
    pub master_volume: Slider,
}

impl Drawable for ConfigWindow {
    fn draw(&self, ctx: &mut ggez::Context, param: ggez::graphics::DrawParam) -> ggez::GameResult {
        self.panel.draw(ctx, param).unwrap();
        self.exit_button.draw(ctx, param).unwrap();
        self.master_volume.draw(ctx, param).unwrap();
        self.master_volume_label.draw(ctx, param).unwrap();
        Ok(())
    }
}
