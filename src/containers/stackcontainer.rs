use ggez::{
    graphics::{DrawParam, Rect},
    mint,
};

use super::{Draw, Update};

pub enum Direction {
    Horizontal,
    Vertical,
}

pub struct StackContainer<T> {
    pub children: Vec<T>,
    pub position: mint::Point2<f32>,
    pub spacing: f32,
    pub cell_size: (f32, f32),
    pub direction: Direction,
}

impl<T> StackContainer<T> {
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

impl<T: Draw> Draw for StackContainer<T> {
    fn draw(&self, ctx: &mut ggez::Context, param: DrawParam) -> ggez::GameResult {
        for child in &self.children {
            child.draw(ctx, param)?;
        }
        Ok(())
    }
}

impl<T: Update> Update for StackContainer<T> {
    fn update(&mut self, dt: f32) {
        for child in &mut self.children {
            child.update(dt);
        }
    }
}
