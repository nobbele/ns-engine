use game::GameState;
use ggez::graphics::DrawParam;
use ggez::graphics::Drawable;
use ggez::input::gamepad::GamepadId;
use ggez::input::keyboard::KeyCode;
use ggez::input::keyboard::KeyMods;
use ggez::input::mouse::MouseButton;
use ggez::Context;
use ggez::{event::quit, event::Axis, graphics, mint::Vector2, GameResult};
use ggez::{event::Button, GameError};
use ggez::{event::EventHandler, mint::Point2};
use graphics::drawable_size;

use crate::tween::{TransitionTweener, TweenBox};

use log::{error, info};

use self::{error::ErrorState, mainmenu::MainMenuState, splash::SplashState};

pub mod error;
pub mod game;
pub mod mainmenu;
pub mod splash;

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

pub enum State {
    Game(GameState),
    MainMenu(MainMenuState),
    Splash(SplashState),
    Error(ErrorState),
}

pub struct StateManager {
    pub state: TweenBox<(Option<(graphics::Image, DrawParam)>, (State, DrawParam))>,
    pub error: Option<GameError>,
}

fn switch_scene_tween(
    ctx: &mut Context,
    has_current: bool,
    state: State,
) -> TweenBox<(Option<(graphics::Image, DrawParam)>, (State, DrawParam))> {
    let img = if has_current {
        let img = graphics::screenshot(ctx).unwrap();
        let data = img.to_rgba8(ctx).unwrap();
        let img = graphics::Image::from_rgba8(ctx, img.width(), img.height(), &data).unwrap();
        Some(img)
    } else {
        None
    };
    Box::new(TransitionTweener {
        time: 0.0,
        target: 0.25,
        set_instantly_if_no_prev: true,
        current: (
            match img {
                Some(img) => Some((img, DrawParam::new().scale(Vector2 { x: 1.0, y: 1.0 }))),
                None => None,
            },
            (state, DrawParam::new()),
        ),
        update: |from, to, progress| {
            if let Some((_, from_param)) = from {
                from_param.color.a = 1.0 - progress;
            }

            to.1.color.a = progress;
        },
    })
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
                match &mut self.state.get_current_mut().1.0 {
                    $(
                        $p(state) => {
                            if let Err(e) = state.update(ctx) {
                                error!("Update Error: {}", e);
                                let state = State::Error(ErrorState::new(ctx, e));
                                self.state = switch_scene_tween(ctx, true, state);
                                return Ok(());
                            }
                            if let Some(new_state) = state.change_state(ctx) {
                                self.state = switch_scene_tween(ctx, true, new_state);
                            }
                        }
                    ),*
                }
                Ok(())
            }

            fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
                graphics::clear(ctx, graphics::WHITE);
                let current = self.state.get_current_mut();
                match &mut current.1.0 {
                    $(
                        $p(state) => {
                            if let Err(e) = state.draw(ctx, current.1.1) {
                                self.error = Some(e);
                            }
                        }
                    ),*
                }
                if let Some((img, drawparam)) = &current.0 {
                    img.draw(ctx, *drawparam).unwrap();
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
                match &mut self.state.get_current_mut().1.0 {
                    $(
                        $p(state) => state.mouse_button_down_event(ctx, button, x, y)
                    ),*
                }
                self.mouse_motion_event(ctx, x, y, 0.0, 0.0);
            }

            fn mouse_button_up_event(
                &mut self,
                ctx: &mut Context,
                button: MouseButton,
                x: f32,
                y: f32,
            ) {
                match &mut self.state.get_current_mut().1.0 {
                    $(
                        $p(state) => state.mouse_button_up_event(ctx, button, x, y)
                    ),*
                }
            }

            fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, dx: f32, dy: f32) {
                match &mut self.state.get_current_mut().1.0 {
                    $(
                        $p(state) => state.mouse_motion_event(ctx, x, y, dx, dy)
                    ),*
                }
            }

            fn mouse_enter_or_leave(&mut self, ctx: &mut Context, entered: bool) {
                match &mut self.state.get_current_mut().1.0 {
                    $(
                        $p(state) => state.mouse_enter_or_leave(ctx, entered)
                    ),*
                }
            }

            fn mouse_wheel_event(&mut self, ctx: &mut Context, x: f32, y: f32) {
                match &mut self.state.get_current_mut().1.0 {
                    $(
                        $p(state) => state.mouse_wheel_event(ctx, x, y)
                    ),*
                }
            }

            fn key_down_event(
                &mut self,
                ctx: &mut Context,
                keycode: KeyCode,
                keymods: KeyMods,
                repeat: bool,
            ) {
                match &mut self.state.get_current_mut().1.0 {
                    $(
                        $p(state) => state.key_down_event(ctx, keycode, keymods, repeat)
                    ),*
                }
            }

            fn key_up_event(&mut self, ctx: &mut Context, keycode: KeyCode, keymods: KeyMods) {
                match &mut self.state.get_current_mut().1.0 {
                    $(
                        $p(state) => state.key_up_event(ctx, keycode, keymods)
                    ),*
                }
            }

            fn text_input_event(&mut self, ctx: &mut Context, character: char) {
                match &mut self.state.get_current_mut().1.0 {
                    $(
                        $p(state) => state.text_input_event(ctx, character)
                    ),*
                }
            }

            fn gamepad_button_down_event(&mut self, ctx: &mut Context, btn: Button, id: GamepadId) {
                match &mut self.state.get_current_mut().1.0 {
                    $(
                        $p(state) => state.gamepad_button_down_event(ctx, btn, id)
                    ),*
                }
            }

            fn gamepad_button_up_event(&mut self, ctx: &mut Context, btn: Button, id: GamepadId) {
                match &mut self.state.get_current_mut().1.0 {
                    $(
                        $p(state) => state.gamepad_button_up_event(ctx, btn, id)
                    ),*
                }
            }

            fn gamepad_axis_event(&mut self, ctx: &mut Context, axis: Axis, value: f32, id: GamepadId) {
                match &mut self.state.get_current_mut().1.0 {
                    $(
                        $p(state) => state.gamepad_axis_event(ctx, axis, value, id)
                    ),*
                }
            }

            fn focus_event(&mut self, ctx: &mut Context, gained: bool) {
                match &mut self.state.get_current_mut().1.0 {
                    $(
                        $p(state) => state.focus_event(ctx, gained)
                    ),*
                }
            }

            fn quit_event(&mut self, ctx: &mut Context) -> bool {
                match &mut self.state.get_current_mut().1.0 {
                    $(
                        $p(state) => state.quit_event(ctx)
                    ),*
                }
            }

            fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
                ggez::graphics::set_screen_coordinates(ctx, ggez::graphics::Rect::new(0.0, 0.0, width, height))
                    .unwrap();

                match &mut self.state.get_current_mut().1.0 {
                    $(
                        $p(state) => state.resize_event(ctx, width, height)
                    ),*
                }
            }
        }
    };
}

impl_eventhandler_for_statemanager!(State::Game, State::MainMenu, State::Splash, State::Error);
