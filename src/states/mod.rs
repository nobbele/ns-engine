use game::GameState;
use ggez::event::Axis;
use ggez::event::Button;
use ggez::event::EventHandler;
use ggez::input::gamepad::GamepadId;
use ggez::input::keyboard::KeyCode;
use ggez::input::keyboard::KeyMods;
use ggez::input::mouse::MouseButton;
use ggez::Context;

use self::mainmenu::MainMenuState;

pub mod game;
pub mod mainmenu;

pub enum State {
    Game(GameState),
    MainMenu(MainMenuState),
}

macro_rules! impl_eventhandler_for_state {
    ($($p:path),+) => {
        impl EventHandler for State {
            fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
                match self {
                    $(
                        $p(state) => state.update(ctx)
                    ),*
                }
            }

            fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
                match self {
                    $(
                        $p(state) => state.draw(ctx)
                    ),*
                }
            }

            fn mouse_button_down_event(
                &mut self,
                ctx: &mut Context,
                button: MouseButton,
                x: f32,
                y: f32,
            ) {
                match self {
                    $(
                        $p(state) => state.mouse_button_down_event(ctx, button, x, y)
                    ),*
                }
            }

            fn mouse_button_up_event(
                &mut self,
                ctx: &mut Context,
                button: MouseButton,
                x: f32,
                y: f32,
            ) {
                match self {
                    $(
                        $p(state) => state.mouse_button_up_event(ctx, button, x, y)
                    ),*
                }
            }

            fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, dx: f32, dy: f32) {
                match self {
                    $(
                        $p(state) => state.mouse_motion_event(ctx, x, y, dx, dy)
                    ),*
                }
            }

            fn mouse_enter_or_leave(&mut self, ctx: &mut Context, entered: bool) {
                match self {
                    $(
                        $p(state) => state.mouse_enter_or_leave(ctx, entered)
                    ),*
                }
            }

            fn mouse_wheel_event(&mut self, ctx: &mut Context, x: f32, y: f32) {
                match self {
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
                match self {
                    $(
                        $p(state) => state.key_down_event(ctx, keycode, keymods, repeat)
                    ),*
                }
            }

            fn key_up_event(&mut self, ctx: &mut Context, keycode: KeyCode, keymods: KeyMods) {
                match self {
                    $(
                        $p(state) => state.key_up_event(ctx, keycode, keymods)
                    ),*
                }
            }

            fn text_input_event(&mut self, ctx: &mut Context, character: char) {
                match self {
                    $(
                        $p(state) => state.text_input_event(ctx, character)
                    ),*
                }
            }

            fn gamepad_button_down_event(&mut self, ctx: &mut Context, btn: Button, id: GamepadId) {
                match self {
                    $(
                        $p(state) => state.gamepad_button_down_event(ctx, btn, id)
                    ),*
                }
            }

            fn gamepad_button_up_event(&mut self, ctx: &mut Context, btn: Button, id: GamepadId) {
                match self {
                    $(
                        $p(state) => state.gamepad_button_up_event(ctx, btn, id)
                    ),*
                }
            }

            fn gamepad_axis_event(&mut self, ctx: &mut Context, axis: Axis, value: f32, id: GamepadId) {
                match self {
                    $(
                        $p(state) => state.gamepad_axis_event(ctx, axis, value, id)
                    ),*
                }
            }

            fn focus_event(&mut self, ctx: &mut Context, gained: bool) {
                match self {
                    $(
                        $p(state) => state.focus_event(ctx, gained)
                    ),*
                }
            }

            fn quit_event(&mut self, ctx: &mut Context) -> bool {
                match self {
                    $(
                        $p(state) => state.quit_event(ctx)
                    ),*
                }
            }

            fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
                match self {
                    $(
                        $p(state) => state.resize_event(ctx, width, height)
                    ),*
                }
            }
        }
    };
}

impl_eventhandler_for_state!(State::Game, State::MainMenu);
