use ggez::{graphics, graphics::Drawable, mint, Context, GameResult};
use graphics::{drawable_size, DrawParam};

use crate::{
    config::Config,
    tween::{TargetTweener, TweenBox},
};

use super::{game::Resources, mainmenu::MainMenuState, State, StateEventHandler};

#[derive(PartialEq, Debug)]
pub enum SplashAnimState {
    Enter,
    Exit,
}

pub struct SplashState {
    anim_state: SplashAnimState,
    splash_img: graphics::Image,
    splash: TweenBox<DrawParam>,
    resources: &'static Resources,
    config: &'static Config,
}

impl SplashState {
    pub fn new(ctx: &mut Context, resources: &'static Resources, config: &'static Config) -> Self {
        Self {
            anim_state: SplashAnimState::Enter,
            resources,
            splash_img: graphics::Image::new(ctx, "/Splash.png").unwrap(),
            splash: Box::new(TargetTweener::new(
                3.0,
                DrawParam::new(),
                |param, progress| {
                    let scale = 0.8 + 0.2 * (2.5 * std::f32::consts::PI * progress.sqrt()).sin();
                    param.scale = mint::Vector2 { x: scale, y: scale };
                },
            )),
            config,
        }
    }
}

impl StateEventHandler for SplashState {
    fn change_state(&mut self, ctx: &mut Context) -> Option<State> {
        if self.anim_state == SplashAnimState::Exit && self.splash.is_done() {
            Some(State::MainMenu(MainMenuState::new(
                ctx,
                self.resources,
                self.config,
            )))
        } else {
            None
        }
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ggez::timer::delta(ctx).as_secs_f32();
        self.splash.update(dt);
        if self.anim_state == SplashAnimState::Enter && self.splash.is_done() {
            self.anim_state = SplashAnimState::Exit;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, parent_param: DrawParam) -> GameResult {
        let mut param = *self.splash.get_current();
        param.scale = mint::Vector2 {
            x: (drawable_size(ctx).1 / self.splash_img.height() as f32) * param.scale.x,
            y: (drawable_size(ctx).1 / self.splash_img.height() as f32) * param.scale.y,
        };
        param.offset = mint::Point2 { x: 0.5, y: 0.5 };
        param.dest = mint::Point2 {
            x: drawable_size(ctx).0 / 2.0,
            y: drawable_size(ctx).1 / 2.0,
        };
        param.color.a = parent_param.color.a;
        self.splash_img.draw(ctx, param)?;
        //graphics::present(ctx)?;
        Ok(())
    }
}
