use ggez::{
    graphics::{self, drawable_size, Drawable},
    mint, Context,
};
use graphics::DrawParam;

use super::{button::Button, stackcontainer::StackContainer, Update};

#[derive(Copy, Clone)]
pub enum MenuButtonId {
    Start,
    Quit,
}

pub struct MainMenuScreen {
    pub background: graphics::Image,
    pub panel: graphics::Mesh,
    pub menu: StackContainer<Button<MenuButtonId>>,
}

impl Drawable for MainMenuScreen {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> ggez::GameResult {
        self.background.draw(
            ctx,
            DrawParam::new()
                .scale(mint::Vector2 {
                    x: drawable_size(ctx).0 / self.background.width() as f32,
                    y: drawable_size(ctx).1 / self.background.height() as f32,
                })
                .color(param.color),
        )?;

        self.panel.draw(ctx, DrawParam::new())?;

        for button in &self.menu.children {
            button.draw(ctx, param)?;
        }

        Ok(())
    }
}

impl Update for MainMenuScreen {
    fn update(&mut self, _dt: f32) {}
}
