use ggez::{
    graphics::{self, Drawable},
    mint, Context,
};
use graphics::DrawParam;

use super::{
    button::Button, config_window::ConfigWindow, rich_text::RichText,
    stackcontainer::StackContainer, Update,
};

#[derive(Copy, Clone, PartialEq)]
pub enum MenuButtonId {
    Start,
    Options,
    Quit,
}

#[allow(clippy::large_enum_variant)]
pub enum Window {
    None,
    Options(ConfigWindow),
}

pub struct MainMenuScreen {
    pub background: graphics::Image,
    pub panel: graphics::Mesh,
    pub menu: StackContainer<Button, MenuButtonId>,
    pub window: Window,
    pub text: RichText,
}

impl Drawable for MainMenuScreen {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> ggez::GameResult {
        self.background.draw(
            ctx,
            DrawParam::new()
                .scale(mint::Vector2 {
                    x: crate::helpers::target_size().x / self.background.width() as f32,
                    y: crate::helpers::target_size().y / self.background.height() as f32,
                })
                .color(param.color),
        )?;

        self.panel.draw(ctx, DrawParam::new())?;

        for (button, _) in &self.menu.children {
            button.draw(ctx, param)?;
        }

        if let Window::Options(window) = &self.window {
            window.draw(ctx, param).unwrap();
        }

        self.text.draw(ctx, param).unwrap();

        Ok(())
    }
}

impl Update for MainMenuScreen {
    fn update(&mut self, _dt: f32) {}
}
