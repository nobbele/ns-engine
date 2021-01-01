use ggez::{graphics, Context};
use ggez::{
    graphics::{DrawParam, Drawable},
    mint,
};

use crate::{helpers::Position, states::game::Character, tween::TweenBox};

use derive_new::new;

#[derive(new)]
pub struct CharacterContainer {
    #[new(default)]
    pub current: Vec<TweenBox<Character>>,
}

impl Drawable for CharacterContainer {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> ggez::GameResult {
        for (n, character) in self.current.iter().enumerate() {
            let character = character.get_current();

            let height = crate::helpers::target_size().y * (4.0 / 5.0);
            let target_size: mint::Point2<f32> = mint::Point2 {
                x: height * (character.image.width() as f32 / character.image.height() as f32),
                y: height,
            };

            let x_position = (crate::helpers::target_size().x as f32
                / (self.current.len() as f32 + 1.0))
                * (n as f32 + 1.0)
                - (target_size.x / 2.0);

            graphics::draw(
                ctx,
                &character.image,
                graphics::DrawParam::new()
                    .dest(
                        Position::BottomLeft
                            .add_in(ctx, glam::Vec2::new(x_position, target_size.y)),
                    )
                    .scale(mint::Vector2 {
                        x: target_size.x / character.image.width() as f32,
                        y: target_size.y / character.image.height() as f32,
                    })
                    .color(graphics::Color {
                        a: character.alpha * param.color.a,
                        ..graphics::WHITE
                    }),
            )?;
        }
        Ok(())
    }
}
