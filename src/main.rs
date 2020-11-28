use std::io::BufReader;

use containers::{
    background::BackgroundContainer, character::CharacterContainer, screen::Action, screen::Screen,
};
use draw::update_draw_choices;
use ggez::event;
use ggez::filesystem;
use ggez::{
    self,
    event::{KeyCode, KeyMods, MouseButton},
    Context,
};
use ggez::{
    conf::{WindowMode, WindowSetup},
    graphics,
};
use helpers::{get_item_index, get_item_y};
use node::{load_background_tween, load_character_tween};

mod containers;
mod draw;
mod helpers;
mod node;
mod tween;

pub enum Placement {
    Left,
    Right,
}

pub struct Character {
    name: String,
    expression: String,
    image: graphics::Image,
    position: Option<Placement>,
    alpha: f32,
}

#[derive(Debug, )]
pub struct Background {
    name: String,
    fade: f32,
    image: graphics::Image,
}

impl Background {
    pub fn new(image: graphics::Image, name: String) -> Self {
        Self {
            image,
            name,
            fade: 0.0,
        }
    }
}

pub struct MainState {
    novel: novelscript::Novel,
    state: novelscript::NovelState,
    hovered_choice: u32,
    resources: &'static Resources,
    screen: Screen,
}

impl MainState {
    fn new(
        ctx: &mut Context,
        novel: novelscript::Novel,
        resources: &'static Resources,
    ) -> MainState {
        let mut state = MainState {
            state: novel.new_state("start"),
            novel,
            hovered_choice: 0,
            resources,
            screen: Screen {
                current_background: None,
                current_characters: CharacterContainer {
                    current: Vec::new(),
                },
                action: Action::None,
            },
        };
        state.continue_text(ctx).unwrap();
        state
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SaveData {
    state: novelscript::NovelState,
    current_background: Option<String>,
    current_characters: Vec<(String, String)>,
}

pub struct Resources {
    text_box: graphics::Image,
}

impl MainState {
    fn continue_text(&mut self, ctx: &mut Context) -> ggez::GameResult {
        match self.novel.next(&mut self.state) {
            Some(novelscript::SceneNodeUser::Data(node)) => {
                node::load_data_node(
                    ctx,
                    &mut self.screen,
                    node,
                    &self.resources,
                    self.hovered_choice,
                )?;
            }
            Some(novelscript::SceneNodeUser::Load(node)) => {
                node::load_load_node(ctx, &mut self.screen, node.clone())?;
                self.continue_text(ctx).unwrap();
            }
            None => {}
        };
        Ok(())
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> ggez::GameResult {
        self.screen.update(ggez::timer::delta(ctx).as_secs_f32());
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        self.screen.draw(ctx)?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, _mods: KeyMods, _: bool) {
        match key {
            KeyCode::Space | KeyCode::Escape => {
                if let Action::Text(..) = &mut self.screen.action {
                    self.continue_text(ctx).unwrap();
                }
            }
            KeyCode::S => {
                println!("Saving game");
                serde_json::to_writer(
                    ggez::filesystem::create(ctx, "/save.json").unwrap(),
                    &SaveData {
                        state: self.state.clone(), // Must clone to be able to be serialized
                        current_characters: self
                            .screen
                            .current_characters
                            .current
                            .iter()
                            .map(|n| {
                                let cur = n.get_current();
                                (cur.name.clone(), cur.expression.clone()) // Must clone to be able to be serialized
                            }) 
                            .collect(),
                        current_background: self
                            .screen
                            .current_background
                            .as_ref()
                            .map(|n| n.current.get_current().1.name.clone()), // Must clone to be able to be serialized
                    },
                )
                .unwrap();
                println!("Saved game!");
            }
            KeyCode::L => {
                if ggez::filesystem::exists(ctx, "/save.json") {
                    let file = ggez::filesystem::open(ctx, "/save.json").unwrap();
                    let savedata: SaveData = serde_json::from_reader(file).unwrap();
                    self.state = savedata.state;
                    self.screen.current_characters.current = Vec::new();
                    for (name, expression) in savedata.current_characters {
                        let character = load_character_tween(ctx, name, expression, "").unwrap();
                        self.screen
                            .current_characters
                            .current
                            .push(Box::new(character));
                    }
                    if let Some(name) = savedata.current_background {
                        let background = load_background_tween(ctx, None, name).unwrap();
                        self.screen.current_background = Some(BackgroundContainer {
                            current: Box::new(background),
                        });
                    } else {
                        self.screen.current_background = None;
                    }

                    self.continue_text(ctx).unwrap();
                }
            }
            _ => (),
        }
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, _x: f32, y: f32, _dx: f32, _dy: f32) {
        if let Action::Choice(choices) = &mut self.screen.action {
            if y > get_item_y(ctx, 0.0, choices.len() as f32)
                && y < get_item_y(ctx, choices.len() as f32, choices.len() as f32)
            {
                let idx = get_item_index(ctx, y, choices.len() as f32);
                self.hovered_choice = idx;
                update_draw_choices(ctx, choices, self.hovered_choice).unwrap();
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
        if let Action::Choice(choices) = &self.screen.action {
            if y > get_item_y(ctx, 0.0, choices.len() as f32)
                && y < get_item_y(ctx, choices.len() as f32, choices.len() as f32)
            {
                let idx = get_item_index(ctx, y, choices.len() as f32);
                self.state.set_choice(idx as i32 + 1);
                self.hovered_choice = 0;
                self.continue_text(ctx).unwrap();
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

    // This will live forever anyway
    let resources = Box::leak(Box::new(Resources {
        text_box: graphics::Image::new(&mut ctx, "/TextBox.png")?,
    }));

    let state = MainState::new(&mut ctx, novel, resources);
    event::run(ctx, event_loop, state)
}
