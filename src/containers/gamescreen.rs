use ggez::Context;

use super::{
    background::BackgroundContainer, button::Button, character::CharacterContainer,
    stackcontainer::StackContainer, textbox::TextBox, ui::UI, Draw, Update,
};

pub enum Action {
    Choice(StackContainer<Button<u32>>),
    Text(Box<TextBox>),
    None,
}

pub struct GameScreen {
    pub current_background: Option<BackgroundContainer>,
    pub current_characters: CharacterContainer,
    pub action: Action,
    pub ui: UI,
}

impl Draw for GameScreen {
    fn draw(&self, ctx: &mut Context) -> ggez::GameResult {
        if let Some(background) = &self.current_background {
            background.draw(ctx)?;
        }

        self.current_characters.draw(ctx)?;

        if let Action::Choice(container) = &self.action {
            for choice in &container.children {
                choice.draw(ctx)?;
            }
        } else if let Action::Text(text) = &self.action {
            text.draw(ctx)?;
        }

        self.ui.draw(ctx)?;

        Ok(())
    }
}

impl Update for GameScreen {
    fn update(&mut self, dt: f32) {
        for character in &mut self.current_characters.current {
            character.update(dt);
        }
        if let Some(current_background) = &mut self.current_background {
            current_background.current.update(dt);
        }

        if let Action::Text(text) = &mut self.action {
            text.update(dt);
        }
    }
}
