use std::io::BufReader;

use ggez::event;
use ggez::filesystem;
use ggez::mint as na;
use ggez::{
    self,
    event::{KeyCode, KeyMods, MouseButton},
    Context,
};
use ggez::{
    conf::{WindowMode, WindowSetup},
    graphics::{self, drawable_size},
};
use helpers::{get_item_index, get_item_y, Position};
use tween::Tween;

mod draw;
mod helpers;
mod node;
mod tween;

pub enum Placement {
    Left,
    Right,
}

pub struct Character {
    _name: String,
    image: graphics::Image,
    position: Option<Placement>,
    alpha: f32,
}

pub struct MainState {
    novel: novelscript::Novel,
    state: novelscript::NovelState,
    current_node: Option<novelscript::SceneNodeData>,
    hovered_choice: u32,
    resources: Resources,
    current_background: Option<graphics::Image>,
    current_characters: Vec<Box<dyn Tween<Character>>>,
}

impl MainState {
    fn new(novel: novelscript::Novel, resources: Resources) -> MainState {
        let mut state = MainState {
            state: novel.new_state("start"),
            novel,
            current_node: None,
            hovered_choice: 0,
            resources,
            current_background: None,
            current_characters: Vec::new(),
        };
        state.continue_text();
        state
    }
}

pub struct Resources {
    text_box: graphics::Image,
}

impl MainState {
    fn continue_text(&mut self) {
        self.current_node = self.novel.next(&mut self.state);
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> ggez::GameResult {
        for character in &mut self.current_characters {
            character.update(ggez::timer::delta(ctx).as_secs_f32());
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        if let Some(background) = &self.current_background {
            graphics::draw(
                ctx,
                background,
                graphics::DrawParam {
                    scale: [
                        drawable_size(ctx).0 / background.width() as f32,
                        drawable_size(ctx).1 / background.height() as f32,
                    ]
                    .into(),
                    ..Default::default()
                },
            )?;
        }

        for (n, character) in self.current_characters.iter().enumerate() {
            let character = character.get_current();
            let x_position = (drawable_size(ctx).0 as f32
                / (self.current_characters.len() as f32 + 1.0))
                * (n as f32 + 1.0);
            let height = drawable_size(ctx).1 * (4.0 / 5.0);
            let target_size: na::Point2<f32> = [
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

        node::draw_node(self, ctx)?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, key: KeyCode, _mods: KeyMods, _: bool) {
        match key {
            KeyCode::Space | KeyCode::Escape => {
                if !matches!(
                    self.current_node,
                    Some(novelscript::SceneNodeData::Choice(..))
                ) {
                    self.continue_text();
                }
            }
            _ => (),
        }
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, _x: f32, y: f32, _dx: f32, _dy: f32) {
        if let Some(novelscript::SceneNodeData::Choice(choices)) = &self.current_node {
            if y > get_item_y(ctx, 0.0, choices.len() as f32)
                && y < get_item_y(ctx, choices.len() as f32, choices.len() as f32)
            {
                self.hovered_choice = get_item_index(ctx, y, choices.len() as f32);
            }
        }
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        y: f32,
    ) {
        if let Some(novelscript::SceneNodeData::Choice(choices)) = &self.current_node {
            if y > get_item_y(ctx, 0.0, choices.len() as f32)
                && y < get_item_y(ctx, choices.len() as f32, choices.len() as f32)
            {
                let idx = get_item_index(ctx, y, choices.len() as f32);
                self.state.set_choice(idx as i32 + 1);
                self.hovered_choice = 0;
                self.current_node = self.novel.next(&mut self.state);
            }
        }
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, width, height))
            .unwrap();
    }
}

pub fn main() -> ggez::GameResult {
    let cb = ggez::ContextBuilder::new("ns-engine", "nobbele")
        .window_setup(WindowSetup::default().title("NS Engine"))
        .window_mode(
            WindowMode::default()
                .dimensions(1280.0, 720.0)
                .resizable(true),
        )
        .add_zipfile_bytes(include_bytes!("../resources.zip").to_vec());
    let (mut ctx, event_loop) = cb.build()?;

    let mut novel = novelscript::Novel::new();
    novel
        .add_scene(
            "start".into(),
            BufReader::new(filesystem::open(&mut ctx, "/test.ns").unwrap()),
        )
        .unwrap();

    let resources = Resources {
        text_box: graphics::Image::new(&mut ctx, "/TextBox.png")?,
    };

    let state = MainState::new(novel, resources);
    event::run(ctx, event_loop, state)
}
