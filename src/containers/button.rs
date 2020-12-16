use std::{cell::RefCell, rc::Rc};

use ggez::{
    audio::SoundSource,
    graphics::{self, Color, DrawParam, Drawable, Rect},
    mint, Context,
};

use crate::states::game::Config;

#[derive(Debug)]
pub struct Button<T: Copy> {
    pub layer: (&'static graphics::Image, DrawParam),
    pub text: graphics::Text,
    pub data_on_click: T,
    pub ui_sfx: Rc<RefCell<Option<ggez::audio::Source>>>,
    pub last_state: bool,
    pub color: &'static Color,
    pub on_hover_color: &'static Color,
    pub on_click_color: &'static Color,
    pub config: &'static Config,
}

impl<T: Copy> Button<T> {
    pub fn new(
        layer: &'static graphics::Image,
        rect: Rect,
        text: String,
        data_on_click: T,
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
            layer: (
                layer,
                DrawParam::new()
                    .dest(rect.point())
                    .scale(mint::Vector2 {
                        x: rect.w / layer.dimensions().w,
                        y: rect.h / layer.dimensions().h,
                    })
                    .color(config.ui.button_color),
            ),
            text,
            data_on_click,
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
            x: self.layer.1.dest.x,
            y: self.layer.1.dest.y,
            w: self.layer.0.dimensions().w * self.layer.1.scale.x,
            h: self.layer.0.dimensions().h * self.layer.1.scale.y,
        }
    }

    pub fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32) {
        let rect = self.layer_dimensions();
        let is_hovered = x > rect.x && x < rect.x + rect.w && y > rect.y && y < rect.y + rect.h;
        self.layer.1.color = if is_hovered {
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

    pub fn click_event(&self, ctx: &mut Context, x: f32, y: f32) -> Option<T> {
        let rect = self.layer_dimensions();
        if x > rect.x && x < rect.x + rect.w && y > rect.y && y < rect.y + rect.h {
            let mut audio = ggez::audio::Source::new(ctx, "/audio/ui_confirm.wav").unwrap();
            audio.play(ctx).unwrap();
            self.ui_sfx.replace(Some(audio));
            Some(self.data_on_click)
        } else {
            None
        }
    }
}

impl<T: Copy> Drawable for Button<T> {
    fn draw(&self, ctx: &mut ggez::Context, parent_param: DrawParam) -> ggez::GameResult {
        let mut layer_param = self.layer.1;
        layer_param.color.a = parent_param.color.a;
        self.layer.0.draw(ctx, layer_param)?;

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
