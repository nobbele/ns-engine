use ggez::{graphics, graphics::Drawable, mint, Context, GameResult};
use graphics::DrawParam;

use crate::{
    resource_manager::ResourceManager,
    tween::{TargetTweener, TweenBox},
};

use super::{mainmenu::MainMenuState, State, StateEventHandler};

#[derive(PartialEq, Debug)]
pub enum SplashAnimState {
    Enter,
    Exit,
}

pub struct SplashState {
    anim_state: SplashAnimState,
    splash_img: graphics::Image,
    splash: TweenBox<DrawParam>,
    resources: &'static ResourceManager,
}

impl SplashState {
    pub fn new(ctx: &mut Context, resources: &'static ResourceManager) -> Self {
        Self {
            anim_state: SplashAnimState::Enter,
            resources,
            splash_img: resources.get_image(ctx, "/Splash.png"),
            splash: Box::new(TargetTweener::new(
                3.0,
                DrawParam::new(),
                |param, progress| {
                    let scale = 0.8 + 0.2 * (2.5 * std::f32::consts::PI * progress.sqrt()).sin();
                    param.scale = mint::Vector2 { x: scale, y: scale };
                },
            )),
        }
    }
}

impl StateEventHandler for SplashState {
    fn change_state(&mut self, ctx: &mut Context) -> Option<State> {
        if self.anim_state == SplashAnimState::Exit && self.splash.is_done() {
            Some(State::MainMenu(MainMenuState::new(ctx, self.resources)))
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
            x: (crate::helpers::target_size().y / self.splash_img.height() as f32) * param.scale.x,
            y: (crate::helpers::target_size().y / self.splash_img.height() as f32) * param.scale.y,
        };
        param.offset = mint::Point2 { x: 0.5, y: 0.5 };
        param.dest = mint::Point2 {
            x: crate::helpers::target_size().x / 2.0,
            y: crate::helpers::target_size().y / 2.0,
        };
        param.color.a = parent_param.color.a;
        self.splash_img.draw(ctx, param)?;
        //graphics::present(ctx)?;
        Ok(())
    }
}
