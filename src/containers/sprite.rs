use ggez::graphics::{DrawParam, Drawable};

use derive_new::new;

#[derive(Debug, new)]
pub struct Sprite<T> {
    pub content: T,
    #[new(default)]
    pub param: DrawParam,
}

impl<T: Drawable> Drawable for Sprite<T> {
    fn draw(&self, ctx: &mut ggez::Context, param: DrawParam) -> ggez::GameResult {
        let mut new_param = self.param;
        new_param.color.r *= param.color.r;
        new_param.color.g *= param.color.g;
        new_param.color.b *= param.color.b;
        new_param.color.a *= param.color.a;

        new_param.scale.x *= param.scale.x;
        new_param.scale.y *= param.scale.y;

        self.content.draw(ctx, new_param)
    }
}
