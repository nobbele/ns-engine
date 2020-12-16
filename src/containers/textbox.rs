use ggez::{
    graphics::{self, DrawParam, Drawable},
    Context,
};

use crate::tween::TweenBox;

use super::{Update, sprite::Sprite};

pub struct TextBox {
    pub layer: Sprite<&'static graphics::Image>,
    pub speaker: Option<Sprite<graphics::Text>>,
    pub content: Sprite<TweenBox<graphics::Text>>,
}

impl Drawable for TextBox {
    fn draw(&self, ctx: &mut Context, parent_param: DrawParam) -> ggez::GameResult {
        self.layer.draw(ctx, parent_param)?;
        if let Some(speaker) = &self.speaker {
            speaker.draw(ctx, parent_param)?;
        }
        let mut param = self.content.param;
        param.color.a = parent_param.color.a;
        self.content.draw(ctx, param)?;
        Ok(())
    }
}

impl Update for TextBox {
    fn update(&mut self, dt: f32) {
        self.content.content.update(dt);
    }
}
