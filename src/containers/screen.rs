use ggez::{Context, graphics::Text};

use super::{background::BackgroundContainer, character::CharacterContainer, panel::Panel};

pub struct Screen {
    pub current_background: Option<BackgroundContainer>,
    pub current_characters: CharacterContainer,
    pub choices: Option<Vec<Panel<Text>>>,
}

impl Screen {
    pub fn draw(&self, ctx: &mut Context) -> ggez::GameResult {
        if let Some(background) = &self.current_background {
            background.draw(ctx)?;
        }

        self.current_characters.draw(ctx)?;

        if let Some(choices) = &self.choices {
            for choice in choices {
                choice.draw(ctx)?;
            }
        }

        Ok(())
    }
}