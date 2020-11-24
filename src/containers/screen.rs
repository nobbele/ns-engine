use ggez::Context;

use super::{background::BackgroundContainer, character::CharacterContainer};

pub struct Screen {
    pub current_background: Option<BackgroundContainer>,
    pub current_characters: CharacterContainer,
}

impl Screen {
    pub fn draw(&self, ctx: &mut Context) -> ggez::GameResult {
        if let Some(background) = &self.current_background {
            background.draw(ctx)?;
        }

        self.current_characters.draw(ctx)?;
        Ok(())
    }
}