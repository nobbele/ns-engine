use ggez::{
    graphics::{self, DrawParam, Drawable},
    Context,
};

use crate::tween::TweenBox;

use super::Update;

pub struct TextBox {
    pub layer: (&'static graphics::Image, DrawParam),
    pub speaker: Option<(graphics::Text, DrawParam)>,
    pub content: (TweenBox<graphics::Text>, DrawParam),
}

impl Drawable for TextBox {
    fn draw(&self, ctx: &mut Context, parent_param: DrawParam) -> ggez::GameResult {
        self.layer.0.draw(ctx, self.layer.1)?;
        if let Some(speaker) = &self.speaker {
            let mut param = speaker.1;
            param.color.a = parent_param.color.a;
            speaker.0.draw(ctx, param)?;
        }
        let mut param = self.content.1;
        param.color.a = parent_param.color.a;
        self.content.0.get_current().draw(ctx, param)?;
        Ok(())
    }
}

impl Update for TextBox {
    fn update(&mut self, dt: f32) {
        self.content.0.update(dt);
    }
}
