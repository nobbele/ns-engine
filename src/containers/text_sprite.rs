use ggez::graphics::{DrawParam, Drawable, Text};

pub struct TextSprite {
    pub content: Text,
    pub params: DrawParam,
}

impl Drawable for TextSprite {
    fn draw(&self, ctx: &mut ggez::Context, _param: DrawParam) -> ggez::GameResult {
        let mut param = self.params;
        param.dest.x -= self.content.dimensions(ctx).0 as f32 / 2.0;
        self.content.draw(ctx, param)
    }
}