use game::GameState;
use ggez::event::EventHandler;
use ggez::graphics::DrawParam;
use ggez::graphics::Drawable;
use ggez::input::gamepad::GamepadId;
use ggez::input::keyboard::KeyCode;
use ggez::input::keyboard::KeyMods;
use ggez::input::mouse::MouseButton;
use ggez::Context;
use ggez::{event::quit, event::Axis, graphics, mint::Vector2, GameResult};
use ggez::{event::Button, GameError};

use crate::{
    containers::sprite::Sprite,
    tween::{TransitionTweener, TweenBox},
};

use log::error;

use self::{error::ErrorState, mainmenu::MainMenuState, splash::SplashState};

pub mod error;
pub mod game;
pub mod mainmenu;
pub mod splash;

#[enum_dispatch::enum_dispatch]
trait StateEventHandler {
    fn update(&mut self, _ctx: &mut Context) -> GameResult;

    fn draw(&mut self, _ctx: &mut Context, param: DrawParam) -> GameResult;

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, _dx: f32, _dy: f32) {}

    fn mouse_enter_or_leave(&mut self, _ctx: &mut Context, _entered: bool) {}

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32) {}

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        if keycode == KeyCode::Escape {
            quit(ctx);
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, _keycode: KeyCode, _keymods: KeyMods) {}

    fn text_input_event(&mut self, _ctx: &mut Context, _character: char) {}

    fn gamepad_button_down_event(&mut self, _ctx: &mut Context, _btn: Button, _id: GamepadId) {}

    fn gamepad_button_up_event(&mut self, _ctx: &mut Context, _btn: Button, _id: GamepadId) {}

    fn gamepad_axis_event(&mut self, _ctx: &mut Context, _axis: Axis, _value: f32, _id: GamepadId) {
    }

    fn focus_event(&mut self, _ctx: &mut Context, _gained: bool) {}

    fn quit_event(&mut self, _ctx: &mut Context) -> bool {
        false
    }

    fn resize_event(&mut self, _ctx: &mut Context, _width: f32, _height: f32) {}

    fn change_state(&mut self, _ctx: &mut Context) -> Option<State> {
        None
    }
}

#[allow(clippy::large_enum_variant)]
#[enum_dispatch::enum_dispatch(StateEventHandler)]
pub enum State {
    Game(GameState),
    MainMenu(MainMenuState),
    Splash(SplashState),
    Error(ErrorState),
}

pub struct StateManager {
    pub state: TweenBox<(Option<Sprite<graphics::Image>>, Sprite<State>)>,
    pub error: Option<GameError>,
}

fn switch_scene_tween(
    ctx: &mut Context,
    has_current: bool,
    state: State,
) -> TweenBox<(Option<Sprite<graphics::Image>>, Sprite<State>)> {
    let img = if has_current {
        let img = graphics::screenshot(ctx).unwrap();
        let data = img.to_rgba8(ctx).unwrap();
        let img = graphics::Image::from_rgba8(ctx, img.width(), img.height(), &data).unwrap();
        Some(img)
    } else {
        None
    };
    Box::new(TransitionTweener::new(
        true,
        0.25,
        (
            match img {
                Some(img) => Some(Sprite {
                    content: img,
                    param: DrawParam::new().scale(Vector2 { x: 1.0, y: 1.0 }),
                }),
                None => None,
            },
            Sprite::new(state),
        ),
        |from, to, progress| {
            if let Some(Sprite { param, .. }) = from {
                param.color.a = 1.0 - progress;
            }

            to.param.color.a = progress;
        },
    ))
}

impl StateManager {
    pub fn new(ctx: &mut Context, state: State) -> Self {
        Self {
            state: switch_scene_tween(ctx, false, state),
            error: None,
        }
    }
}

macro_rules! impl_eventhandler_for_statemanager {
    ($($p:path),+) => {
        impl EventHandler for StateManager {
            fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
                if let Some(e) = &self.error {
                    error!("Draw Error: {}", e);
                    let state = State::Error(ErrorState::new(ctx, e.clone()));
                    self.state = switch_scene_tween(ctx, true, state);
                    self.error = None;
                    return Ok(());
                }
                self.state.update(ggez::timer::delta(ctx).as_secs_f32());
                let current = &mut self.state.get_current_mut().1.content;
                if let Err(e) = current.update(ctx) {
                    error!("Update Error: {}", e);
                    let state = State::Error(ErrorState::new(ctx, e));
                    self.state = switch_scene_tween(ctx, true, state);
                    return Ok(());
                }
                if let Some(new_state) = current.change_state(ctx) {
                    self.state = switch_scene_tween(ctx, true, new_state);
                }
                Ok(())
            }

            fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
                graphics::clear(ctx, graphics::WHITE);

                let current = self.state.get_current_mut();
                let state_param = current.1.param;//.dest(pos);
                if let Err(e) = current.1.content.draw(ctx, state_param) {
                    self.error = Some(e);
                }
                if let Some(Sprite { content, param }) = &current.0 {
                    let new_param = *param;//.dest(pos);
                    content.draw(ctx, new_param).unwrap();
                }
                graphics::present(ctx)?;
                Ok(())
            }

            fn mouse_button_down_event(
                &mut self,
                ctx: &mut Context,
                button: MouseButton,
                x: f32,
                y: f32,
            ) {
                self.state
                    .get_current_mut()
                    .1
                    .content
                    .mouse_button_down_event(ctx, button, x, y);
                self.mouse_motion_event(ctx, x, y, 0.0, 0.0);
            }

            fn mouse_button_up_event(
                &mut self,
                ctx: &mut Context,
                button: MouseButton,
                x: f32,
                y: f32,
            ) {
                self.state
                    .get_current_mut()
                    .1
                    .content
                    .mouse_button_up_event(ctx, button, x, y);
            }

            fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, dx: f32, dy: f32) {
                self.state
                    .get_current_mut()
                    .1
                    .content
                    .mouse_motion_event(ctx, x, y, dx, dy);
            }

            fn mouse_enter_or_leave(&mut self, ctx: &mut Context, entered: bool) {
                self.state
                    .get_current_mut()
                    .1
                    .content
                    .mouse_enter_or_leave(ctx, entered);
            }

            fn mouse_wheel_event(&mut self, ctx: &mut Context, x: f32, y: f32) {
                self.state
                    .get_current_mut()
                    .1
                    .content
                    .mouse_wheel_event(ctx, x, y);
            }

            fn key_down_event(
                &mut self,
                ctx: &mut Context,
                keycode: KeyCode,
                keymods: KeyMods,
                repeat: bool,
            ) {
                self.state
                    .get_current_mut()
                    .1
                    .content
                    .key_down_event(ctx, keycode, keymods, repeat);
            }

            fn key_up_event(&mut self, ctx: &mut Context, keycode: KeyCode, keymods: KeyMods) {
                self.state
                    .get_current_mut()
                    .1
                    .content
                    .key_up_event(ctx, keycode, keymods);
            }

            fn text_input_event(&mut self, ctx: &mut Context, character: char) {
                self.state
                    .get_current_mut()
                    .1
                    .content
                    .text_input_event(ctx, character);
            }

            fn gamepad_button_down_event(&mut self, ctx: &mut Context, btn: Button, id: GamepadId) {
                self.state
                    .get_current_mut()
                    .1
                    .content
                    .gamepad_button_down_event(ctx, btn, id);
            }

            fn gamepad_button_up_event(&mut self, ctx: &mut Context, btn: Button, id: GamepadId) {
                self.state
                    .get_current_mut()
                    .1
                    .content
                    .gamepad_button_up_event(ctx, btn, id);
            }

            fn gamepad_axis_event(
                &mut self,
                ctx: &mut Context,
                axis: Axis,
                value: f32,
                id: GamepadId,
            ) {
                self.state
                    .get_current_mut()
                    .1
                    .content
                    .gamepad_axis_event(ctx, axis, value, id);
            }

            fn focus_event(&mut self, ctx: &mut Context, gained: bool) {
                self.state
                    .get_current_mut()
                    .1
                    .content
                    .focus_event(ctx, gained);
            }

            fn quit_event(&mut self, ctx: &mut Context) -> bool {
                self.state.get_current_mut().1.content.quit_event(ctx)
            }

            fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
                let ratio = glam::Vec2::new(
                    width / crate::helpers::target_size().x,
                    height / crate::helpers::target_size().y,
                );

                let coords = if ratio.x > ratio.y {
                    ggez::graphics::Rect::new(
                        0.0,
                        0.0,
                        width / ratio.y,
                        crate::helpers::target_size().y,
                    )
                } else if ratio.y > ratio.x {
                    ggez::graphics::Rect::new(
                        0.0,
                        0.0,
                        crate::helpers::target_size().x,
                        height / ratio.x,
                    )
                } else {
                    ggez::graphics::Rect::new(
                        0.0,
                        0.0,
                        width, 
                        height,
                    )
                };
                ggez::graphics::set_screen_coordinates(
                    ctx,
                    coords,
                )
                .unwrap();

                self.state
                    .get_current_mut()
                    .1
                    .content
                    .resize_event(ctx, width, height);
            }
        }
    };
}

impl_eventhandler_for_statemanager!(State::Game, State::MainMenu, State::Splash, State::Error);
