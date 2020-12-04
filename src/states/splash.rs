use ggez::{event::EventHandler, graphics, graphics::Drawable, mint, Context, GameResult};
use graphics::{drawable_size, DrawParam};

use crate::tween::{TargetTweener, TweenBox};

use super::{game::Resources, mainmenu::MainMenuState, State};

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
}

impl SplashState {
    pub fn new(ctx: &mut Context, resources: &'static Resources) -> Self {
        Self {
            anim_state: SplashAnimState::Enter,
            resources,
            splash_img: graphics::Image::new(ctx, "/Splash.png").unwrap(),
            splash: Box::new(TargetTweener {
                time: 0.0,
                target: 3.0,
                current: DrawParam::new(),
                update: |param, progress| {
                    let scale = 0.8 + 0.2 * (2.5 * std::f32::consts::PI * progress.sqrt()).sin();
                    param.scale = mint::Vector2 { x: scale, y: scale };
                },
            }),
        }
    }

    pub fn change_state(&mut self, ctx: &mut Context) -> Option<State> {
        if self.anim_state == SplashAnimState::Exit && self.splash.is_done() {
            Some(State::MainMenu(MainMenuState::new(
                ctx,
                &mut self.resources,
            )))
        } else {
            None
        }
    }
}

impl EventHandler for SplashState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ggez::timer::delta(ctx).as_secs_f32();
        self.splash.update(dt);
        if self.anim_state == SplashAnimState::Enter && self.splash.is_done() {
            self.anim_state = SplashAnimState::Exit;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [1.0, 1.0, 1.0, 1.0].into());

        let mut param = *self.splash.get_current();
        param.scale = mint::Vector2 {
            x: (drawable_size(ctx).0 / self.splash_img.width() as f32) * param.scale.x,
            y: (drawable_size(ctx).1 / self.splash_img.height() as f32) * param.scale.y,
        };
        param.offset = mint::Point2 { x: 0.5, y: 0.5 };
        param.dest = mint::Point2 {
            x: drawable_size(ctx).0 / 2.0,
            y: drawable_size(ctx).1 / 2.0,
        };
        self.splash_img.draw(ctx, param)?;

        graphics::present(ctx)?;
        Ok(())
    }
}
