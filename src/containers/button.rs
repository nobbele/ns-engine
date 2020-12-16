use std::{cell::RefCell, rc::Rc};

use ggez::{
    audio::SoundSource,
    graphics::{self, Color, DrawParam, Drawable, Rect},
    mint, Context,
};

use crate::states::game::Config;

use super::sprite::Sprite;

#[derive(Debug)]
pub struct Button {
    pub layer: Sprite<&'static graphics::Image>,
    pub text: graphics::Text,
    pub ui_sfx: Rc<RefCell<Option<ggez::audio::Source>>>,
    pub last_state: bool,
    pub color: &'static Color,
    pub on_hover_color: &'static Color,
    pub on_click_color: &'static Color,
    pub config: &'static Config,
}

impl Button {
    pub fn new(
        layer: &'static graphics::Image,
        rect: Rect,
        text: String,
        ui_sfx: Rc<RefCell<Option<ggez::audio::Source>>>,
        config: &'static Config,
    ) -> ggez::GameResult<Self> {
        let mut text = graphics::Text::new(text);
        text.set_bounds(
            mint::Point2 {
                x: rect.w,
                y: rect.h / 2.0,
            },
            graphics::Align::Center,
        );
        Ok(Self {
            layer: Sprite {
                content: layer,
                param: DrawParam::new()
                    .dest(rect.point())
                    .scale(mint::Vector2 {
                        x: rect.w / layer.dimensions().w,
                        y: rect.h / layer.dimensions().h,
                    })
                    .color(config.ui.button_color),
            },
            text,
            ui_sfx,
            last_state: false,
            color: &config.ui.button_color,
            on_hover_color: &config.ui.button_highlight_color,
            on_click_color: &config.ui.button_pressed_color,
            config,
        })
    }

    fn layer_dimensions(&self) -> Rect {
        Rect {
            x: self.layer.param.dest.x,
            y: self.layer.param.dest.y,
            w: self.layer.content.dimensions().w * self.layer.param.scale.x,
            h: self.layer.content.dimensions().h * self.layer.param.scale.y,
        }
    }

    pub fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32) {
        let rect = self.layer_dimensions();
        let is_hovered = rect.contains(mint::Point2 { x, y });
        self.layer.param.color = if is_hovered {
            *self.on_hover_color
        } else {
            *self.color
        };
        if self.last_state != is_hovered {
            if is_hovered {
                let mut audio = ggez::audio::Source::new(ctx, "/audio/ui_select.wav").unwrap();
                audio.play(ctx).unwrap();
                self.ui_sfx.replace(Some(audio));
            }

            self.last_state = is_hovered;
        }
    }

    pub fn click_event(&self, ctx: &mut Context, x: f32, y: f32) -> bool {
        let rect = self.layer_dimensions();
        if rect.contains(mint::Point2 { x, y }) {
            let mut audio = ggez::audio::Source::new(ctx, "/audio/ui_confirm.wav").unwrap();
            audio.play(ctx).unwrap();
            self.ui_sfx.replace(Some(audio));
            true
        } else {
            false
        }
    }
}

impl Drawable for Button {
    fn draw(&self, ctx: &mut ggez::Context, parent_param: DrawParam) -> ggez::GameResult {
        self.layer.draw(ctx, parent_param)?;

        let layer_bounds = self.layer_dimensions();

        let mut text_pos = layer_bounds.point();
        text_pos.y -= self.text.height(ctx) as f32 / 2.0;
        text_pos.y += layer_bounds.h / 2.0;

        let mut text_param = DrawParam::new().dest(text_pos);
        text_param.color.a = parent_param.color.a;

        self.text.draw(ctx, text_param)?;

        Ok(())
    }
}
