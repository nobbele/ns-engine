use std::{cell::RefCell, io::Read, path::PathBuf, rc::Rc};

use crate::{
    containers::{
        button::Button,
        config_window::{ConfigWindow, VolumeControl},
        credits_window::CreditsWindow,
        mainmenuscreen::MainMenuScreen,
        mainmenuscreen::{MenuButtonId, Window},
        rich_text::RichText,
        slider::Slider,
        sprite::Sprite,
        stackcontainer::Direction,
        stackcontainer::StackContainer,
    },
    helpers::{points_to_rect, Position},
    resource_manager::ResourceManager,
};
use ggez::{
    audio::SoundSource,
    event::{self, MouseButton},
    filesystem,
    graphics::DrawParam,
    graphics::{self, DrawMode, Drawable},
    mint::Point2,
    Context,
};
use glam::vec2;
use graphics::{FillOptions, Rect, Text};

use super::{game::GameState, State, StateEventHandler};

pub struct MainMenuState {
    pub resources: &'static ResourceManager,
    pub screen: MainMenuScreen,
    pub clicked_event: Option<MenuButtonId>,
    pub music: ggez::audio::Source,
    pub ui_sfx: Rc<RefCell<Option<ggez::audio::Source>>>,
}

impl MainMenuState {
    pub fn new(ctx: &mut Context, resources: &'static ResourceManager) -> Self {
        let music = ggez::audio::Source::new(ctx, "/audio/bgm.mp3").unwrap();
        let mut state = Self {
            resources,
            clicked_event: None,
            screen: MainMenuScreen {
                background: resources.get_image(ctx, "/MainMenuBackground"),
                panel: graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::Fill(graphics::FillOptions::DEFAULT),
                    graphics::Rect {
                        x: 0.0,
                        y: 0.0,
                        w: crate::helpers::target_size().x / 2.5,
                        h: crate::helpers::target_size().y,
                    },
                    graphics::Color {
                        r: 0.1,
                        g: 0.1,
                        b: 0.1,
                        a: 0.9,
                    },
                )
                .unwrap(),
                title: Sprite {
                    content: Text::new(resources.get_config().ui.title.clone()),
                    param: DrawParam::new().dest(Position::TopLeft.add_in(
                        ctx,
                        glam::Vec2::new(
                            crate::helpers::target_size().x / (2.5 * 2.0) - (300.0 / 2.0),
                            100.0,
                        ),
                    )),
                },
                menu: StackContainer::new(
                    Position::TopLeft.add_in(
                        ctx,
                        glam::Vec2::new(
                            crate::helpers::target_size().x / (2.5 * 2.0) - (300.0 / 2.0),
                            crate::helpers::target_size().y / 2.0 - (60.0 * 2.0 / 2.0),
                        ),
                    ),
                    5.0,
                    (300.0, 60.0),
                    Direction::Vertical,
                ),
                window: Window::None,
            },
            music,
            ui_sfx: Rc::new(RefCell::new(None)),
        };
        for (n, d) in [
            ("Start", MenuButtonId::Start),
            ("Options", MenuButtonId::Options),
            ("Credits", MenuButtonId::Credits),
            ("Quit", MenuButtonId::Quit),
        ]
        .iter()
        .enumerate()
        {
            state.screen.menu.children.push((
                Button::new(
                    ctx,
                    resources,
                    state.screen.menu.get_rect_for(n as f32),
                    d.0.into(),
                    state.ui_sfx.clone(),
                )
                .unwrap(),
                d.1,
            ))
        }
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

            Some(State::Game(GameState::new(ctx, novel, self.resources)))
        } else {
            None
        }
    }

    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let config = self.resources.get_config();
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
                                w: crate::helpers::target_size().x,
                                h: crate::helpers::target_size().y,
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
                            ctx,
                            self.resources,
                            points_to_rect(
                                Position::TopRight.add_in(ctx, glam::Vec2::new(55.0, 5.0)),
                                Position::TopRight.add_in(ctx, glam::Vec2::new(5.0, 55.0)),
                            ),
                            "X".into(),
                            self.ui_sfx.clone(),
                        )
                        .unwrap(),
                        volume_controls: StackContainer::new(
                            Position::Center
                                .add_in(ctx, glam::Vec2::new(-120.0, (-46.0 * 3.0) / 2.0)),
                            5.0,
                            (240.0, 46.0),
                            Direction::Vertical,
                        ),
                    };
                    for (n, &(d, v, s)) in [
                        ("Master", config.user.borrow().master_volume, "master"),
                        ("SFX", config.user.borrow().channel_volumes.0["sfx"], "sfx"),
                        (
                            "BGM",
                            config.user.borrow().channel_volumes.0["music"],
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
                MenuButtonId::Credits => {
                    let credits_window = CreditsWindow {
                        panel: graphics::Mesh::new_rectangle(
                            ctx,
                            DrawMode::Fill(FillOptions::DEFAULT),
                            Rect {
                                x: 0.0,
                                y: 0.0,
                                w: crate::helpers::target_size().x,
                                h: crate::helpers::target_size().y,
                            },
                            graphics::Color {
                                r: 0.0,
                                g: 0.0,
                                b: 0.0,
                                a: 0.9,
                            },
                        )
                        .unwrap(),
                        text: Sprite {
                            content: RichText::new(&config.credits, {
                                let mut text = Text::default();
                                text.set_bounds(
                                    Point2 {
                                        x: crate::helpers::target_size().x - 50.0,
                                        y: crate::helpers::target_size().y - 50.0,
                                    },
                                    graphics::Align::Left,
                                );
                                text
                            }),
                            param: DrawParam::new().dest(vec2(50.0, 50.0)),
                        },
                        exit_button: Button::new(
                            ctx,
                            self.resources,
                            points_to_rect(
                                Position::TopRight.add_in(ctx, glam::Vec2::new(55.0, 5.0)),
                                Position::TopRight.add_in(ctx, glam::Vec2::new(5.0, 55.0)),
                            ),
                            "X".into(),
                            self.ui_sfx.clone(),
                        )
                        .unwrap(),
                    };
                    self.screen.window = Window::Credits(credits_window);
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
            config.user.borrow().master_volume * config.user.borrow().channel_volumes.0["music"],
        );
        if !self.music.playing() {
            self.music.play(ctx)?;
        }
        if let Some(audio) = self.ui_sfx.borrow_mut().as_mut() {
            audio.set_volume(
                config.user.borrow().master_volume * config.user.borrow().channel_volumes.0["sfx"],
            );
            if !audio.playing() && audio.elapsed().as_millis() < 1 {
                audio.play(ctx)?;
            }
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
                    let config = self.resources.get_config();
                    let mut config = config.user.borrow_mut();
                    match *d {
                        "master" => config.master_volume = n,
                        s => {
                            *config.channel_volumes.0.get_mut(s).unwrap() = n;
                        }
                    }
                }
            }
        } else if let Window::Credits(window) = &mut self.screen.window {
            window.exit_button.mouse_motion_event(ctx, x, y);
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
                self.resources
                    .get_config()
                    .user
                    .borrow()
                    .update_data(ctx, &self.resources.get_config().short_game_name);
                self.screen.window = Window::None;
            }
        } else if let Window::Credits(window) = &mut self.screen.window {
            window.text.mouse_button_up_event(ctx, button, x, y);
            if window.exit_button.click_event(ctx, x, y) {
                self.resources
                    .get_config()
                    .user
                    .borrow()
                    .update_data(ctx, &self.resources.get_config().short_game_name);
                self.screen.window = Window::None;
            }
        }
    }
}
