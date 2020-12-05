use ggez::graphics::DrawParam;

use super::{button::Button, stackcontainer::StackContainer, Draw};

#[derive(Copy, Clone)]
pub enum MenuButtonId {
    Save,
    Load,
    Skip,
    Auto,
}

pub struct UI {
    pub menu: StackContainer<Button<MenuButtonId>>,
}

impl Draw for UI {
    fn draw(&self, ctx: &mut ggez::Context, param: DrawParam) -> ggez::GameResult {
        self.menu.draw(ctx, param)?;
        Ok(())
    }
}
