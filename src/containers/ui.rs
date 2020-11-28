use super::{Draw, button::Button, stackcontainer::StackContainer};

#[derive(Copy, Clone)]
pub enum MenuButtonId {
    Save,
    Load,
}

pub struct UI {
    pub menu: StackContainer<Button<MenuButtonId>>,
}

impl Draw for UI {
    fn draw(&self, ctx: &mut ggez::Context) -> ggez::GameResult {
        self.menu.draw(ctx)?;
        Ok(())
    }
}
