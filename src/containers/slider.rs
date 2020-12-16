use ggez::{Context, event::MouseButton, graphics::{Color, DrawMode, Drawable, FillOptions, Mesh, Rect}};

pub struct Slider {
    layer: Mesh,
    progress_layer: Mesh,
    is_clicking: bool,
}

pub fn update_progress_layer(ctx: &mut Context, dim: Rect, progress: f32) -> Mesh {
    let mut dim = dim;
    dim.w *= progress;
    Mesh::new_rectangle(
        ctx,
        DrawMode::Fill(FillOptions::DEFAULT),
        dim,
        Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 0.7,
        },
    )
    .unwrap()
}

impl Slider {
    pub fn new(ctx: &mut Context, dim: Rect, starting_progress: f32) -> Self {
        let layer = Mesh::new_rectangle(
            ctx,
            DrawMode::Fill(FillOptions::DEFAULT),
            dim,
            Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.7,
            },
        )
        .unwrap();
        let progress_layer = update_progress_layer(ctx, dim, starting_progress);
        Self {
            layer,
            progress_layer,
            is_clicking: false,
        }
    }

    pub fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) -> Option<f32> {
        if self.is_clicking {
            let bounds = self.layer.dimensions(ctx).unwrap();
            let x = x.max(bounds.x).min(bounds.x + bounds.w);
            let progress = (x - bounds.x) / bounds.w;
            self.progress_layer = update_progress_layer(ctx, bounds, progress);
            Some(progress)
        } else {
            None
        }
    }

    pub fn mouse_button_down_event(&mut self, ctx: &mut Context, _button: MouseButton, x: f32, y: f32) {
        let bounds = self.layer.dimensions(ctx).unwrap();
        if x > bounds.x && x < bounds.x + bounds.w && y > bounds.y && y < bounds.y + bounds.h {
            self.is_clicking = true;
        }
    }

    pub fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, x: f32, y: f32) {
        self.is_clicking = false;
    }
}

impl Drawable for Slider {
    fn draw(&self, ctx: &mut Context, param: ggez::graphics::DrawParam) -> ggez::GameResult {
        self.layer.draw(ctx, param).unwrap();
        self.progress_layer.draw(ctx, param).unwrap();
        Ok(())
    }
}
