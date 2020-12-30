use derive_new::new;
use ggez::graphics::Drawable;

pub trait Tween<T> {
    fn get_current(&self) -> &T;
    fn get_current_mut(&mut self) -> &mut T;
    fn take_final(self) -> T;
    fn take_final_box(self: Box<Self>) -> T;
    fn update(&mut self, dt: f32);
    fn is_done(&self) -> bool;
    fn finish(&mut self) {}
}

impl<T: Drawable> Drawable for Box<dyn Tween<T>> {
    fn draw(&self, ctx: &mut ggez::Context, param: ggez::graphics::DrawParam) -> ggez::GameResult {
        let current = self.get_current();
        current.draw(ctx, param)
    }
}

#[derive(new)]
pub struct Tweener<T, F: Fn(&mut T, f32, f32) -> bool> {
    #[new(value = "0.0")]
    pub time: f32,
    pub current: T,
    pub update: F,
    #[new(value = "false")]
    pub is_done: bool,
}
impl<T, F: Fn(&mut T, f32, f32) -> bool> Tween<T> for Tweener<T, F> {
    fn update(&mut self, dt: f32) {
        self.time += dt;
        self.is_done = (self.update)(&mut self.current, self.time, dt);
    }

    fn get_current(&self) -> &T {
        &self.current
    }

    fn get_current_mut(&mut self) -> &mut T {
        &mut self.current
    }

    fn take_final(self) -> T {
        self.current
    }

    fn take_final_box(self: Box<Self>) -> T {
        self.current
    }

    fn is_done(&self) -> bool {
        self.is_done
    }

    fn finish(&mut self) {
        self.time = f32::MAX;
    }
}

#[derive(new)]
pub struct TargetTweener<T, F: Fn(&mut T, f32)> {
    #[new(value = "0.0")]
    pub time: f32,
    pub target: f32,
    pub current: T,
    pub update: F,
}
impl<T, F: Fn(&mut T, f32)> Tween<T> for TargetTweener<T, F> {
    fn update(&mut self, dt: f32) {
        self.time += dt;
        let progress = if self.time < self.target {
            self.time / self.target
        } else {
            1.0
        };
        (self.update)(&mut self.current, progress);
    }

    fn get_current(&self) -> &T {
        &self.current
    }

    fn get_current_mut(&mut self) -> &mut T {
        &mut self.current
    }

    fn take_final(self) -> T {
        self.current
    }

    fn take_final_box(self: Box<Self>) -> T {
        self.current
    }

    fn is_done(&self) -> bool {
        self.time >= self.target
    }

    fn finish(&mut self) {
        self.time = self.target;
    }
}

#[derive(new)]
pub struct TransitionTweener<T1, T2, F: Fn(&mut Option<T1>, &mut T2, f32)> {
    pub set_instantly_if_no_prev: bool,
    #[new(value = "0.0")]
    pub time: f32,
    pub target: f32,
    pub current: (Option<T1>, T2),
    pub update: F,
}

impl<T1, T2, F: Fn(&mut Option<T1>, &mut T2, f32)> Tween<(Option<T1>, T2)>
    for TransitionTweener<T1, T2, F>
{
    fn update(&mut self, dt: f32) {
        self.time += dt;
        let progress = if self.time < self.target
            && !(self.current.0.is_none() && self.set_instantly_if_no_prev)
        {
            self.time / self.target
        } else {
            self.current.0 = None;
            1.0
        };
        (self.update)(&mut self.current.0, &mut self.current.1, progress);
    }

    fn get_current(&self) -> &(Option<T1>, T2) {
        &self.current
    }

    fn get_current_mut(&mut self) -> &mut (Option<T1>, T2) {
        &mut self.current
    }

    fn take_final(self) -> (Option<T1>, T2) {
        self.current
    }

    fn take_final_box(self: Box<Self>) -> (Option<T1>, T2) {
        self.current
    }

    fn is_done(&self) -> bool {
        self.time >= self.target
    }

    fn finish(&mut self) {
        self.time = self.target;
    }
}

#[derive(new)]
pub struct NonTweener<T> {
    pub current: T,
}
impl<T> Tween<T> for NonTweener<T> {
    fn update(&mut self, _dt: f32) {

    }

    fn get_current(&self) -> &T {
        &self.current
    }

    fn get_current_mut(&mut self) -> &mut T {
        &mut self.current
    }

    fn take_final(self) -> T {
        self.current
    }

    fn take_final_box(self: Box<Self>) -> T {
        self.current
    }

    fn is_done(&self) -> bool {
        true
    }

    fn finish(&mut self) {
        
    }
}

pub type TweenBox<T> = Box<dyn Tween<T>>;
pub type TransitionTweenBox<T> = Box<dyn Tween<(Option<T>, T)>>;
