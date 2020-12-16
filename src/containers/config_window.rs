use ggez::graphics::{Drawable, Mesh};

use super::{
    button::Button, slider::Slider, stackcontainer::StackContainer, text_sprite::TextSprite,
};

pub struct VolumeControl(pub TextSprite, pub Slider);

impl Drawable for VolumeControl {
    fn draw(&self, ctx: &mut ggez::Context, param: ggez::graphics::DrawParam) -> ggez::GameResult {
        self.0.draw(ctx, param).unwrap();
        self.1.draw(ctx, param).unwrap();
        Ok(())
    }
}

pub struct ConfigWindow {
    pub panel: Mesh,
    pub exit_button: Button,
    pub volume_controls: StackContainer<VolumeControl, &'static str>,
}

impl Drawable for ConfigWindow {
    fn draw(&self, ctx: &mut ggez::Context, param: ggez::graphics::DrawParam) -> ggez::GameResult {
        self.panel.draw(ctx, param).unwrap();
        self.exit_button.draw(ctx, param).unwrap();
        self.volume_controls.draw(ctx, param).unwrap();
        Ok(())
    }
}
