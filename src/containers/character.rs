use ggez::mint;
use ggez::{
    graphics::{self, drawable_size},
    Context,
};

use crate::{states::game::Character, helpers::Position, tween::TweenBox};

use super::Draw;

pub struct CharacterContainer {
    pub current: Vec<TweenBox<Character>>,
}

impl Draw for CharacterContainer {
    fn draw(&self, ctx: &mut Context) -> ggez::GameResult {
        for (n, character) in self.current.iter().enumerate() {
            let character = character.get_current();
            let x_position = (drawable_size(ctx).0 as f32 / (self.current.len() as f32 + 1.0))
                * (n as f32 + 1.0);
            let height = drawable_size(ctx).1 * (4.0 / 5.0);
            let target_size: mint::Point2<f32> = [
                height * (character.image.width() as f32 / character.image.height() as f32),
                height,
            ]
            .into();
            graphics::draw(
                ctx,
                &character.image,
                graphics::DrawParam {
                    dest: Position::BottomLeft.add_in(ctx, (x_position, target_size.y)),
                    scale: [
                        target_size.x / character.image.width() as f32,
                        target_size.y / character.image.height() as f32,
                    ]
                    .into(),
                    color: graphics::Color {
                        a: character.alpha,
                        ..graphics::WHITE
                    },
                    ..Default::default()
                },
            )?;
        }
        Ok(())
    }
}
