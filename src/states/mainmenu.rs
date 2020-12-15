use std::{cell::RefCell, io::Read, path::PathBuf, rc::Rc};

use crate::{
    containers::{
        button::Button,
        config_window::{ButtonActionId, ConfigWindow},
        mainmenuscreen::MainMenuScreen,
        mainmenuscreen::{MenuButtonId, Window},
        stackcontainer::Direction,
        stackcontainer::StackContainer,
    },
    helpers::{points_to_rect, Position},
};
use ggez::{
    audio::SoundSource,
    event::{self, MouseButton},
    filesystem,
    graphics::DrawParam,
    graphics::{self, drawable_size, DrawMode, Drawable},
    Context,
};
use graphics::{FillOptions, Rect, Text};

use super::{
    game::{GameState, Resources},
    State, StateEventHandler,
};

pub struct MainMenuState {
    pub resources: &'static Resources,
    pub screen: MainMenuScreen,
    pub clicked_event: Option<MenuButtonId>,
    pub music: ggez::audio::Source,
    pub ui_sfx: Rc<RefCell<Option<ggez::audio::Source>>>,
}

impl MainMenuState {
    pub fn new(ctx: &mut Context, resources: &'static Resources) -> Self {
        let mut state = Self {
            resources,
            clicked_event: None,
            screen: MainMenuScreen {
                background: graphics::Image::new(ctx, "/MainMenuBackground.png").unwrap(),
                panel: graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::Fill(graphics::FillOptions::DEFAULT),
                    graphics::Rect {
                        x: 0.0,
                        y: 0.0,
                        w: drawable_size(ctx).0 / 2.5,
                        h: drawable_size(ctx).1,
                    },
                    graphics::Color {
                        r: 0.1,
                        g: 0.1,
                        b: 0.1,
                        a: 0.8,
                    },
                )
                .unwrap(),
                menu: StackContainer {
                    children: Vec::new(),
                    position: Position::TopLeft.add_in(
                        ctx,
                        (
                            drawable_size(ctx).0 / (2.5 * 2.0) - (300.0 / 2.0),
                            drawable_size(ctx).1 / 2.0 - (60.0 * 2.0 / 2.0),
                        ),
                    ),
                    cell_size: (300.0, 60.0),
                    spacing: 5.0,
                    direction: Direction::Vertical,
                },
                window: Window::None,
            },
            music: ggez::audio::Source::new(ctx, "/audio/bgm.mp3").unwrap(),
            ui_sfx: Rc::new(RefCell::new(None)),
        };
        for (n, d) in [
            ("Start", MenuButtonId::Start),
            ("Options", MenuButtonId::Options),
            ("Quit", MenuButtonId::Quit),
        ]
        .iter()
        .enumerate()
        {
            state.screen.menu.children.push(
                Button::new(
                    &resources,
                    &resources.button,
                    state.screen.menu.get_rect_for(n as f32),
                    d.0.into(),
                    d.1,
                    state.ui_sfx.clone(),
                )
                .unwrap(),
            )
        }
        state.music.play(ctx).unwrap();
        state
    }

    pub fn change_state(&mut self, ctx: &mut Context) -> Option<State> {
        if let Some(MenuButtonId::Start) = self.clicked_event {
            let mut novel = novelscript::Novel::new();

            for file in filesystem::read_dir(ctx, "scripts").unwrap().skip(1) {
                let name = file.file_stem().unwrap().to_string_lossy().into_owned();
                let mut data = String::new();
                filesystem::open(ctx, PathBuf::from("/").join(file))
                    .unwrap()
                    .read_to_string(&mut data)
                    .unwrap();
                novel.add_scene(name, &data);
            }

            Some(State::Game(GameState::new(ctx, novel, self.resources)))
        } else {
            None
        }
    }
}

impl StateEventHandler for MainMenuState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        if let Some(e) = self.clicked_event {
            match e {
                MenuButtonId::Start => {} // Handled in change_state
                MenuButtonId::Options => {
                    self.screen.window = Window::Options(ConfigWindow {
                        panel: graphics::Mesh::new_rectangle(
                            ctx,
                            DrawMode::Fill(FillOptions::DEFAULT),
                            Rect {
                                x: 0.0,
                                y: 0.0,
                                w: drawable_size(ctx).0,
                                h: drawable_size(ctx).1,
                            },
                            graphics::Color {
                                r: 0.0,
                                g: 0.0,
                                b: 0.0,
                                a: 0.5,
                            },
                        )
                        .unwrap(),
                        exit_button: Button::new(
                            self.resources,
                            &self.resources.button,
                            points_to_rect(
                                Position::TopRight.add_in(ctx, (55.0, 5.0)),
                                Position::TopRight.add_in(ctx, (5.0, 55.0)),
                            ),
                            "X".into(),
                            ButtonActionId::Exit,
                            self.ui_sfx.clone(),
                        )
                        .unwrap(),
                        text: (
                            Text::new("WIP"),
                            DrawParam::new().dest(Position::Center.add_in(ctx, (0.0, 0.0))),
                        ),
                    })
                }
                MenuButtonId::Quit => {
                    event::quit(ctx);
                }
            }
            self.clicked_event = None;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context, param: DrawParam) -> ggez::GameResult {
        self.screen.draw(ctx, param)?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        if let Window::None = self.screen.window {
            for button in &mut self.screen.menu.children {
                button.mouse_motion_event(ctx, x, y);
            }
        } else if let Window::Options(window) = &mut self.screen.window {
            window.exit_button.mouse_motion_event(ctx, x, y);
        }
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, _button: MouseButton, x: f32, y: f32) {
        if let Window::None = self.screen.window {
            if let Some(e) = self
                .screen
                .menu
                .children
                .iter()
                .find_map(|button| button.click_event(ctx, x, y))
            {
                self.clicked_event = Some(e);
            }
        } else if let Window::Options(window) = &mut self.screen.window {
            if let Some(e) = window.exit_button.click_event(ctx, x, y) {
                match e {
                    crate::containers::config_window::ButtonActionId::Exit => {
                        self.screen.window = Window::None;
                    }
                }
            }
        }
    }
}
