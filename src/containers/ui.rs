use ggez::graphics::{DrawParam, Drawable};

use super::{button::Button, stackcontainer::StackContainer};

#[derive(Copy, Clone)]
pub enum MenuButtonId {
    Save,
    Load,
    Skip,
    Auto,
}

pub struct UI {
    pub menu: StackContainer<Button<MenuButtonId>, MenuButtonId>,
}

impl Drawable for UI {
    fn draw(&self, ctx: &mut ggez::Context, param: DrawParam) -> ggez::GameResult {
        self.menu.draw(ctx, param)?;
        Ok(())
    }
}
