use ggez::{
    graphics::{self, Color, DrawParam, Drawable, Rect},
    mint, Context,
};

use super::Draw;

pub struct Button<T: Copy> {
    pub layer: graphics::Mesh,
    pub text: graphics::Text,
    pub data_on_click: T,
}

fn create_layer(
    ctx: &mut Context,
    rect: Rect,
    is_hovered: bool,
) -> ggez::GameResult<graphics::Mesh> {
    let color = if is_hovered {
        Color {
            r: 0.6,
            g: 0.6,
            b: 0.6,
            a: 1.0,
        }
    } else {
        Color {
            r: 0.2,
            g: 0.2,
            b: 0.2,
            a: 1.0,
        }
    };
    graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, color)
}

impl<T: Copy> Button<T> {
    pub fn new(
        ctx: &mut Context,
        rect: Rect,
        text: String,
        data_on_click: T,
    ) -> ggez::GameResult<Self> {
        let layer = create_layer(ctx, rect, false)?;
        let mut text = graphics::Text::new(text);
        text.set_bounds(
            mint::Point2 {
                x: rect.w,
                y: rect.h / 2.0,
            },
            graphics::Align::Center,
        );
        Ok(Self {
            layer,
            text,
            data_on_click,
        })
    }

    pub fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32) {
        let rect = self.layer.dimensions(ctx).unwrap();
        self.layer = create_layer(
            ctx,
            rect,
            x > rect.x && x < rect.x + rect.w && y > rect.y && y < rect.y + rect.h,
        )
        .unwrap();
    }

    pub fn mouse_button_down_event(&mut self, ctx: &mut Context, x: f32, y: f32) -> Option<T> {
        let rect = self.layer.dimensions(ctx).unwrap();
        if x > rect.x && x < rect.x + rect.w && y > rect.y && y < rect.y + rect.h {
            Some(self.data_on_click)
        } else {
            None
        }
    }
}

impl<T: Copy> Draw for Button<T> {
    fn draw(&self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let layer_bounds = self.layer.dimensions(ctx).unwrap();

        self.layer.draw(ctx, DrawParam::new())?;

        let mut text_pos = layer_bounds.point();
        text_pos.y -= self.text.height(ctx) as f32 / 2.0;
        text_pos.y += layer_bounds.h / 2.0;

        self.text.draw(ctx, DrawParam::new().dest(text_pos))?;

        Ok(())
    }
}
