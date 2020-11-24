use ggez::{graphics::Text, Context};

use super::{
    background::BackgroundContainer, character::CharacterContainer, panel::Panel, textbox::TextBox,
};

pub enum Action {
    Choice(Vec<Panel<Text>>),
    Text(Box<TextBox>),
    None,
}

pub struct Screen {
    pub current_background: Option<BackgroundContainer>,
    pub current_characters: CharacterContainer,
    pub action: Action,
}

impl Screen {
    pub fn draw(&self, ctx: &mut Context) -> ggez::GameResult {
        if let Some(background) = &self.current_background {
            background.draw(ctx)?;
        }

        self.current_characters.draw(ctx)?;

        if let Action::Choice(choices) = &self.action {
            for choice in choices {
                choice.draw(ctx)?;
            }
        }

        if let Action::Text(text) = &self.action {
            text.draw(ctx)?;
        }

        Ok(())
    }
}
