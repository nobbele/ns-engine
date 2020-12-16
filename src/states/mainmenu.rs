use std::{cell::RefCell, io::Read, path::PathBuf, rc::Rc};

use crate::{
    containers::{
        button::Button,
        config_window::{ConfigWindow, VolumeControl},
        mainmenuscreen::MainMenuScreen,
        mainmenuscreen::{MenuButtonId, Window},
        slider::Slider,
        sprite::Sprite,
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
    game::{Config, GameState, Resources},
    State, StateEventHandler,
};

pub struct MainMenuState {
    pub resources: &'static Resources,
    pub screen: MainMenuScreen,
    pub clicked_event: Option<MenuButtonId>,
    pub music: ggez::audio::Source,
    pub ui_sfx: Rc<RefCell<Option<ggez::audio::Source>>>,
    pub config: &'static Config,
}

impl MainMenuState {
    pub fn new(ctx: &mut Context, resources: &'static Resources, config: &'static Config) -> Self {
        let music = ggez::audio::Source::new(ctx, "/audio/bgm.mp3").unwrap();
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
                        a: 0.9,
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
            music,
            ui_sfx: Rc::new(RefCell::new(None)),
            config,
        };
        for (n, d) in [
            ("Start", MenuButtonId::Start),
            ("Options", MenuButtonId::Options),
            ("Quit", MenuButtonId::Quit),
        ]
        .iter()
        .enumerate()
        {
            state.screen.menu.children.push((
                Button::new(
                    &resources.button,
                    state.screen.menu.get_rect_for(n as f32),
                    d.0.into(),
                    state.ui_sfx.clone(),
                    &state.config,
                )
                .unwrap(),
                d.1,
            ))
        }
        state.music.play(ctx).unwrap();
        state
    }
}

impl StateEventHandler for MainMenuState {
    fn change_state(&mut self, ctx: &mut Context) -> Option<State> {
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

            Some(State::Game(GameState::new(
                ctx,
                novel,
                self.resources,
                self.config,
            )))
        } else {
            None
        }
    }

    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        if let Some(e) = self.clicked_event {
            match e {
                MenuButtonId::Start => {} // Handled in change_state
                MenuButtonId::Options => {
                    let mut config_window = ConfigWindow {
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
                                a: 0.9,
                            },
                        )
                        .unwrap(),
                        exit_button: Button::new(
                            &self.resources.button,
                            points_to_rect(
                                Position::TopRight.add_in(ctx, (55.0, 5.0)),
                                Position::TopRight.add_in(ctx, (5.0, 55.0)),
                            ),
                            "X".into(),
                            self.ui_sfx.clone(),
                            &self.config,
                        )
                        .unwrap(),
                        volume_controls: StackContainer {
                            children: Vec::new(),
                            position: Position::Center.add_in(ctx, (-120.0, (-46.0 * 3.0) / 2.0)),
                            spacing: 5.0,
                            cell_size: (240.0, 46.0),
                            direction: Direction::Vertical,
                        },
                    };
                    for (n, &(d, v, s)) in [
                        ("Master", self.config.user.borrow().master_volume, "master"),
                        (
                            "SFX",
                            self.config.user.borrow().channel_volumes.0["sfx"],
                            "sfx",
                        ),
                        (
                            "BGM",
                            self.config.user.borrow().channel_volumes.0["music"],
                            "music",
                        ),
                    ]
                    .iter()
                    .enumerate()
                    {
                        let rect = config_window.volume_controls.get_rect_for(n as f32);
                        config_window.volume_controls.children.push((
                            VolumeControl(
                                Sprite {
                                    content: Text::new(d),
                                    param: DrawParam::new().dest(rect.point()),
                                },
                                Slider::new(
                                    ctx,
                                    Rect {
                                        x: rect.x,
                                        y: rect.y + 16.0,
                                        w: rect.w,
                                        h: rect.h - 16.0,
                                    },
                                    v,
                                ),
                            ),
                            s,
                        ))
                    }
                    self.screen.window = Window::Options(config_window);
                }
                MenuButtonId::Quit => {
                    event::quit(ctx);
                }
            }
            // this is handled elsewhere, kinda gross but what you gonna do
            if self.clicked_event != Some(MenuButtonId::Start) {
                self.clicked_event = None;
            }
        }
        self.music.set_volume(
            self.config.user.borrow().master_volume
                * self.config.user.borrow().channel_volumes.0["music"],
        );
        if let Some(audio) = &mut *self.ui_sfx.borrow_mut() {
            audio.set_volume(
                self.config.user.borrow().master_volume
                    * self.config.user.borrow().channel_volumes.0["sfx"],
            )
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context, param: DrawParam) -> ggez::GameResult {
        self.screen.draw(ctx, param)?;
        Ok(())
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, dx: f32, dy: f32) {
        if let Window::None = self.screen.window {
            for (button, _) in &mut self.screen.menu.children {
                button.mouse_motion_event(ctx, x, y);
            }
        } else if let Window::Options(window) = &mut self.screen.window {
            window.exit_button.mouse_motion_event(ctx, x, y);
            for (slider, d) in &mut window.volume_controls.children {
                if let Some(n) = slider.1.mouse_motion_event(ctx, x, y, dx, dy) {
                    let mut config = self.config.user.borrow_mut();
                    match *d {
                        "master" => config.master_volume = n,
                        s => {
                            *config.channel_volumes.0.get_mut(s).unwrap() = n;
                        }
                    }
                }
            }
        }
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if let Window::Options(window) = &mut self.screen.window {
            for (slider, _) in &mut window.volume_controls.children {
                slider.1.mouse_button_down_event(ctx, button, x, y);
            }
        }
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if let Window::None = self.screen.window {
            if let Some(e) = self.screen.menu.children.iter().find_map(|(button, n)| {
                if button.click_event(ctx, x, y) {
                    Some(n)
                } else {
                    None
                }
            }) {
                self.clicked_event = Some(*e);
            }
        } else if let Window::Options(window) = &mut self.screen.window {
            for (slider, _) in &mut window.volume_controls.children {
                slider.1.mouse_button_up_event(ctx, button, x, y);
            }
            if window.exit_button.click_event(ctx, x, y) {
                self.config.user.borrow().update_data(ctx);
                self.screen.window = Window::None;
            }
        }
    }
}
