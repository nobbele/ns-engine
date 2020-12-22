use ggez::graphics::{DrawParam, Drawable, Rect};

use super::Update;

use derive_new::new;

pub enum Direction {
    Horizontal,
    Vertical,
}

#[derive(new)]
pub struct StackContainer<T, D> {
    #[new(default)]
    pub children: Vec<(T, D)>,
    pub position: glam::Vec2,
    pub spacing: f32,
    pub cell_size: (f32, f32),
    pub direction: Direction,
}

impl<T, D> StackContainer<T, D> {
    pub fn get_rect_for(&self, n: f32) -> Rect {
        let pos = match self.direction {
            Direction::Vertical => {
                let x = self.position.x;
                let y = self.position.y + n * (self.cell_size.1 + self.spacing);
                (x, y)
            }
            Direction::Horizontal => {
                let x = self.position.x + n * (self.cell_size.0 + self.spacing);
                let y = self.position.y;
                (x, y)
            }
        };
        Rect {
            x: pos.0,
            y: pos.1,
            w: self.cell_size.0,
            h: self.cell_size.1,
        }
    }
}

impl<T: Drawable, D> Drawable for StackContainer<T, D> {
    fn draw(&self, ctx: &mut ggez::Context, param: DrawParam) -> ggez::GameResult {
        for (child, _) in &self.children {
            child.draw(ctx, param)?;
        }
        Ok(())
    }
}

impl<T: Update, D> Update for StackContainer<T, D> {
    fn update(&mut self, dt: f32) {
        for (child, _) in &mut self.children {
            child.update(dt);
        }
    }
}
