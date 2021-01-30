use ggez::{
    event::MouseButton,
    graphics::{DrawParam, Drawable, Rect},
    mint, Context,
};

use derive_new::new;

use super::rich_text::{Format, RichText};

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

        self.content.draw(ctx, new_param)?;

        Ok(())
    }
}

impl Sprite<RichText> {
    pub fn mouse_button_up_event(
        &mut self,
        ctx: &mut Context,
        _button: MouseButton,
        x: f32,
        y: f32,
    ) {
        for format in &self.content.formatting {
            let positions = &self.content.text.positions(ctx)[format.start..format.end];
            let bounds = Rect {
                x: positions[0].x + self.param.dest.x,
                y: positions[0].y + self.param.dest.y - ggez::graphics::DEFAULT_FONT_SCALE,
                w: (positions[positions.len() - 1].x + ggez::graphics::DEFAULT_FONT_SCALE) - positions[0].x,
                h: positions[positions.len() - 1].y
                    - (positions[0].y - ggez::graphics::DEFAULT_FONT_SCALE),
            };
            if bounds.contains(mint::Point2 { x, y }) {
                match &format.format {
                    Format::Link(url) => {
                        webbrowser::open(url).unwrap();
                    }
                }
                break;
            }
        }
    }
}
