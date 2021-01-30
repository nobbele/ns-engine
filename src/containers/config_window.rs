use ggez::graphics::{Drawable, Mesh, Text};

use super::{button::Button, slider::Slider, sprite::Sprite, stackcontainer::StackContainer};

pub struct VolumeControl(pub Sprite<Text>, pub Slider);

impl Drawable for VolumeControl {
    fn draw(&self, ctx: &mut ggez::Context, param: ggez::graphics::DrawParam) -> ggez::GameResult {
        self.0.draw(ctx, param)?;
        self.1.draw(ctx, param)?;
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
        self.panel.draw(ctx, param)?;
        self.exit_button.draw(ctx, param)?;
        self.volume_controls.draw(ctx, param)?;
        Ok(())
    }
}
