use ggez::{Context, mint};

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
    fn get_position(&self, n: f32) -> (f32, f32) {
        match self.direction {
            Direction::Vertical => {
                let x = self.position.x;
                let y = self.position.y + n * (self.cell_size.1 + self.spacing);
                (x, y)
            },
            Direction::Horizontal => {
                let x = self.position.x + n * (self.cell_size.0 + self.spacing);
                let y = self.position.y;
                (x, y)
            },
        }
    }

    pub fn init<D>(&mut self, ctx: &mut Context, data: Vec<D>, load: impl Fn(&mut Context, D, (f32, f32)) -> T) {
        for (n, d) in data.into_iter().enumerate() {
            self.children.push(load(ctx, d, self.get_position(n as f32)))
        }
    }
}

impl<T: Draw> Draw for StackContainer<T> {
    fn draw(&self, ctx: &mut ggez::Context) -> ggez::GameResult {
        for child in &self.children {
            child.draw(ctx)?;
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
